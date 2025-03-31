use axum::{Router, routing::{get, post}, http::{HeaderValue, Method}};
use std::sync::Arc;
use parking_lot::Mutex;
use crate::api::{chat_handler, generate_journal};
use crate::services::session::SessionStore;
use dotenv::dotenv;
use std::net::SocketAddr;
use tower_http::cors::{CorsLayer, Any};

mod api;
mod models;
mod services;
mod utils;

#[tokio::main]
async fn main() {
    dotenv().ok(); // Load .env variables

    let session_store = Arc::new(Mutex::new(SessionStore::new()));

    // Set up CORS
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods([Method::GET, Method::POST])
        .allow_headers(Any);

    let app = Router::new()
        .route("/health", get(|| async { "OK" }))
        .route("/chat", post(chat_handler))
        .route("/generate-journal", post(generate_journal))
        .layer(cors)
        .with_state(session_store);

    let addr: SocketAddr = "0.0.0.0:8080".parse().unwrap();
    println!("🚀 Server running on {}", addr);
    
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
