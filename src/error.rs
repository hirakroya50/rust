use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};

pub(crate) struct ApiError(pub(crate) StatusCode, pub(crate) String);

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        (
            self.0,
            Json(serde_json::json!({
                "error": self.1
            })),
        )
            .into_response()
    }
}

pub(crate) fn bad(message: &str) -> ApiError {
    ApiError(StatusCode::BAD_REQUEST, message.to_string())
}

pub(crate) fn not_found(resource: &str) -> ApiError {
    ApiError(StatusCode::NOT_FOUND, format!("{resource} not found"))
}

pub(crate) fn db_error(error: sqlx::Error) -> ApiError {
    if let sqlx::Error::Database(ref database_error) = error {
        match database_error.code().as_deref() {
            Some("23505") => {
                return ApiError(
                    StatusCode::CONFLICT,
                    "A record with this unique value already exists".to_string(),
                );
            }
            Some("23514") | Some("22003") => {
                return bad("Data violates database constraints");
            }
            _ => {}
        }
    }

    eprintln!("Database error: {error}");

    ApiError(
        StatusCode::INTERNAL_SERVER_ERROR,
        "Database operation failed".to_string(),
    )
}
