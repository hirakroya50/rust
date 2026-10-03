use axum::{Json, extract::State, http::StatusCode};
use sqlx::PgPool;

use crate::error::ApiError;

pub(crate) async fn health(
    State(pool): State<PgPool>,
) -> Result<Json<serde_json::Value>, ApiError> {
    sqlx::query("SELECT 1")
        .execute(&pool)
        .await
        .map_err(|error| {
            eprintln!("Health check failed: {error}");

            ApiError(
                StatusCode::SERVICE_UNAVAILABLE,
                "Database unavailable".to_string(),
            )
        })?;

    Ok(Json(serde_json::json!({
        "status": "healthy"
    })))
}
