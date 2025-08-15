use axum::{
    Router, 
    routing::get,
    serve,
    http::{HeaderValue, Method, header::CONTENT_TYPE, header::AUTHORIZATION},
};
use std::net::SocketAddr;
use tokio::net::TcpListener;
use tower_http::{
    cors::CorsLayer,
    trace::TraceLayer,
};
mod wage_routes;

async fn health() -> &'static str {
    "OK"
}

fn init_router() -> Router {
    // Configuração do CORS
    let cors = CorsLayer::new()
        .allow_origin("http://localhost:5173".parse::<HeaderValue>().unwrap())
        .allow_methods([Method::GET, Method::POST, Method::PUT, Method::DELETE])
        .allow_headers([CONTENT_TYPE, AUTHORIZATION])
        .allow_credentials(true);

    Router::new()
        .route("/health", get(health))
        .merge(wage_routes::wage_routes())
        .layer(cors)
        .layer(TraceLayer::new_for_http())
}

#[tokio::main]
async fn main() {
    let app = init_router();
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    
    let listener = TcpListener::bind(addr).await.unwrap();
    println!("Servidor rodando em http://{}", addr);
    
    serve(listener, app).await.unwrap();
}