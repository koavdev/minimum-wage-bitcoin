use axum::{Router, routing::get, response::IntoResponse, Json};
use reqwest::StatusCode;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize)]
struct WageResponse {
    countries: HashMap<String, CountryWage>,
    metadata: WageMetadata,
}

#[derive(Serialize)]
struct CountryWage {
    annual: Option<f64>,
    monthly: Option<f64>,
    hourly: Option<f64>,
    currency: String,
}

#[derive(Serialize)]
struct WageMetadata {
    description: String,
    prepared: String,
    name: String,
}

#[derive(Debug, Deserialize)]
struct SdmxResponse {
    meta: SdmxMeta,
    data: SdmxData,
}

#[derive(Debug, Deserialize)]
struct SdmxMeta {
    prepared: String,
}

#[derive(Debug, Deserialize)]
struct SdmxData {
    structures: Vec<SdmxStructure>,
    dataSets: Vec<SdmxDataSet>,
}

#[derive(Debug, Deserialize)]
struct SdmxStructure {
    name: String,
    description: String,
    dimensions: StructureDimensions,
}

#[derive(Debug, Deserialize)]
struct StructureDimensions {
    observation: Vec<DimensionDef>,
}

#[derive(Debug, Deserialize)]
struct DimensionDef {
    id: String,
    values: Vec<DimValue>,
}

#[derive(Debug, Deserialize)]
struct DimValue {
    id: String,
    // order and other fields are ignored for mapping by position
    #[serde(default)]
    order: Option<i32>,
}

#[derive(Debug, Deserialize)]
struct SdmxDataSet {
    observations: HashMap<String, Vec<Option<f64>>>,
}

async fn get_wage_data() -> impl IntoResponse {
    let url = "https://sdmx.oecd.org/public/rest/data/OECD.ELS.SAE,DSD_EARNINGS@MW_CURP,1.0/?startPeriod=2024&detail=DataOnly&dimensionAtObservation=AllDimensions&format=jsondata";
    
    let error_response = WageResponse {
        countries: HashMap::new(),
        metadata: WageMetadata {
            description: String::new(),
            prepared: String::new(),
            name: String::new(),
        },
    };

    match reqwest::get(url).await {
        Ok(resp) => {
            match resp.json::<SdmxResponse>().await {
                Ok(sdmx_data) => {
                    println!("{:?}", sdmx_data);
                    let mut countries = HashMap::new();
                    
                    // build mapping from observation-dimensions
                    let mut index_to_iso: HashMap<String, String> = HashMap::new();
                    let mut index_to_payperiod: HashMap<String, String> = HashMap::new();

                    if let Some(structure) = sdmx_data.data.structures.first() {
                        // find REF_AREA dimension
                        if let Some(ref_area_dim) = structure.dimensions.observation.iter().find(|d| d.id == "REF_AREA") {
                            for (i, v) in ref_area_dim.values.iter().enumerate() {
                                index_to_iso.insert(i.to_string(), v.id.clone());
                            }
                        }

                        // find PAY_PERIOD dimension
                        if let Some(pay_dim) = structure.dimensions.observation.iter().find(|d| d.id == "PAY_PERIOD") {
                            for (i, v) in pay_dim.values.iter().enumerate() {
                                index_to_payperiod.insert(i.to_string(), v.id.clone());
                            }
                        }
                    }

                    if let Some(dataset) = sdmx_data.data.dataSets.first() {
                        // create country entries lazily when we see them
                        for (key, values) in &dataset.observations {
                            if let Some(Some(value)) = values.first() {
                                let coords: Vec<&str> = key.split(':').collect();
                                if coords.len() >= 8 {
                                    let country_idx = coords[0];
                                    let pay_idx = coords[3];

                                    // get ISO from index map
                                    let country_iso = match index_to_iso.get(country_idx) {
                                        Some(id) => id.clone(),
                                        None => continue,
                                    };

                                    // find or insert default CountryWage
                                    let country_wage = countries.entry(country_iso.clone()).or_insert_with(|| CountryWage {
                                        annual: None,
                                        monthly: None,
                                        hourly: None,
                                        currency: country_iso.clone(), // fallback: use iso as currency placeholder
                                    });

                                    // determine pay period id (M/H/A) from pay_idx
                                    if let Some(pay_id) = index_to_payperiod.get(pay_idx) {
                                        match pay_id.as_str() {
                                            "M" => country_wage.monthly = Some(*value),
                                            "H" => country_wage.hourly = Some(*value),
                                            "A" => country_wage.annual = Some(*value),
                                            _ => (),
                                        }
                                    }
                                }
                            }
                        }
                    }

                    // Remove países sem dados
                    countries.retain(|_, wage| {
                        wage.annual.is_some() || wage.monthly.is_some() || wage.hourly.is_some()
                    });

                    let response = WageResponse {
                        countries,
                        metadata: WageMetadata {
                            description: sdmx_data.data.structures[0].description.clone(),
                            prepared: sdmx_data.meta.prepared,
                            name: sdmx_data.data.structures[0].name.clone(),
                        },
                    };

                    (StatusCode::OK, Json(response))
                },
                Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, Json(error_response)),
            }
        }
        Err(_) => (StatusCode::BAD_GATEWAY, Json(error_response)),
    }
}

pub fn wage_routes() -> Router {
    Router::new()
        .route("/wage/data", get(get_wage_data))
}