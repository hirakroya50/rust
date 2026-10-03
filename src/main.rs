mod config;
mod db;
mod error;
mod health;
mod http;
mod pagination;
mod products;
mod routes;
mod users;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = config::Config::load()?;

    let listener = tokio::net::TcpListener::bind(("127.0.0.1", config.port))
        .await
        .map_err(|error| {
            eprintln!(
                "Cannot bind port {}: {}. Stop the other server \
                     or change PORT in .env.",
                config.port, error
            );
            error
        })?;

    let pool = db::connect(&config.database_url).await?;
    println!("Connected to PostgreSQL");

    let spec_path = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("openapi.json");

    let spec_text = std::fs::read_to_string(spec_path)?;
    let openapi = serde_json::from_str(&spec_text)?;

    let app = routes::build(pool, openapi);

    println!("API: http://localhost:{}", config.port);
    println!("Swagger: http://localhost:{}/swagger-ui/", config.port);

    axum::serve(listener, app)
        .with_graceful_shutdown(async {
            if let Err(error) = tokio::signal::ctrl_c().await {
                eprintln!("Shutdown signal failed: {error}");
            }
        })
        .await?;

    Ok(())
}
