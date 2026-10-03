use axum::{
    extract::State,
    http::StatusCode,
    routing::get,
    Router,
};
use sqlx::{postgres::PgPoolOptions, PgPool};
use std::{env, time::Duration};
use utoipa_swagger_ui::SwaggerUi;

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
    dotenvy::dotenv().ok();

    let database_url = env::var("DATABASE_URL")?;

    let port: u16 = env::var("PORT")
        .unwrap_or_else(|_| "3002".to_string())
        .parse()?;

    // Bind first so a busy port is detected immediately.
    let listener =
        match tokio::net::TcpListener::bind(("127.0.0.1", port)).await {
            Ok(listener) => listener,
            Err(error) => {
                eprintln!(
                    "Cannot start on port {port}: {error}. \
                     If the port is busy, change PORT in .env \
                     or stop the process using it."
                );
                return Err(error.into());
            }
        };

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .acquire_timeout(Duration::from_secs(10))
        .connect(&database_url)
        .await?;

    println!("Connected to PostgreSQL");

    let spec_path =
        std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("openapi.json");

    let spec_text = std::fs::read_to_string(&spec_path)?;
    let openapi: serde_json::Value =
        serde_json::from_str(&spec_text)?;

    let app = Router::new()
        .route("/", get(home))
        .route("/db-health", get(db_health))
        .merge(
            SwaggerUi::new("/swagger-ui")
                .external_url_unchecked(
                    "/api-docs/openapi.json",
                    openapi,
                ),
        )
        .with_state(pool);

    println!("Backend: http://localhost:{port}");
    println!("Swagger: http://localhost:{port}/swagger-ui/");
    println!("OpenAPI: http://localhost:{port}/api-docs/openapi.json");

    axum::serve(listener, app)
        .with_graceful_shutdown(async {
            if let Err(error) = tokio::signal::ctrl_c().await {
                eprintln!("Shutdown signal error: {error}");
            }
        })
        .await?;

    Ok(())
}