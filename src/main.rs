use axum::{routing::get, Json, Router};
use serde::Serialize;

#[derive(Serialize)]
struct HealthResponse {
    status: String,
}

async fn home() -> &'static str {
    "Hello! ----- Your Rust backend is running."
}

async fn health() -> Json<HealthResponse> {
    Json(HealthResponse {
        status: "healthy".to_string(),
    })
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let app = Router::new()
        .route("/", get(home))
        .route("/health", get(health));

    let listener =
        tokio::net::TcpListener::bind("127.0.0.1:3001").await?;

    println!("Backend running at http://localhost:3001");

    axum::serve(listener, app).await
}