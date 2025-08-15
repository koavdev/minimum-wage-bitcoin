use axum::{Router, routing::get, response::IntoResponse};

async fn get_currency_data() -> impl IntoResponse {
    
}

pub fn currency_routes() -> Router {
    Router::new()
        .route("/currency/data", get(get_currency_data))
}