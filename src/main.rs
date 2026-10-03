use axum::{
    extract::State,
    http::StatusCode,
    routing::get,
    Router,
};
use sqlx::{postgres::PgPoolOptions, PgPool};
use std::{env, time::Duration};

async fn home() -> &'static str {
    "Rust backend is running!"
}

async fn db_health(
    State(pool): State<PgPool>,
) -> Result<&'static str, (StatusCode, &'static str)> {
    sqlx::query("SELECT 1")
        .execute(&pool)
        .await
        .map_err(|error| {
            eprintln!("Database check failed: {error}");
            (
                StatusCode::SERVICE_UNAVAILABLE,
                "Database unavailable",
            )
        })?;

    Ok("PostgreSQL is connected!")
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load .env before starting the server.
    dotenvy::dotenv().ok();

    let database_url = env::var("DATABASE_URL")?;
    let port: u16 = env::var("PORT")
        .unwrap_or_else(|_| "3001".to_string())
        .parse()?;

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .acquire_timeout(Duration::from_secs(10))
        .connect(&database_url)
        .await?;

    println!("Connected to PostgreSQL");

    let app = Router::new()
        .route("/", get(home))
        .route("/db-health", get(db_health))
        .with_state(pool);

    let listener =
        tokio::net::TcpListener::bind(("127.0.0.1", port)).await?;

    println!("Backend running at http://localhost:{port}");

    axum::serve(listener, app).await?;

    Ok(())
}