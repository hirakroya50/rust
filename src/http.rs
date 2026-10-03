use axum::{Json, extract::rejection::JsonRejection};

use crate::error::ApiError;

pub(crate) fn body<T>(input: Result<Json<T>, JsonRejection>) -> Result<T, ApiError> {
    input
        .map(|Json(value)| value)
        .map_err(|error| ApiError(error.status(), error.body_text()))
}
