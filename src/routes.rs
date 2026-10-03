use axum::{Router, routing::get};
use sqlx::PgPool;
use utoipa_swagger_ui::SwaggerUi;

pub(crate) fn build(pool: PgPool, openapi: serde_json::Value) -> Router {
    Router::new()
        .route("/", get(|| async { "Rust users and products API" }))
        .route("/health", get(crate::health::health))
        .merge(crate::users::routes())
        .merge(crate::products::routes())
        .merge(
            SwaggerUi::new("/swagger-ui").external_url_unchecked("/api-docs/openapi.json", openapi),
        )
        .with_state(pool)
}
