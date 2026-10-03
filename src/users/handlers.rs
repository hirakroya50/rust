use axum::{
    Json,
    extract::{Path, Query, State, rejection::JsonRejection},
    http::StatusCode,
};
use sqlx::PgPool;

use super::{dto::UserInput, models::User, service};
use crate::{error::ApiError, http::body, pagination::Pagination};

pub(crate) async fn list_users(
    State(pool): State<PgPool>,
    Query(page): Query<Pagination>,
) -> Result<Json<Vec<User>>, ApiError> {
    Ok(Json(service::list(&pool, page).await?))
}

pub(crate) async fn get_user(
    State(pool): State<PgPool>,
    Path(id): Path<i64>,
) -> Result<Json<User>, ApiError> {
    Ok(Json(service::get(&pool, id).await?))
}

pub(crate) async fn create_user(
    State(pool): State<PgPool>,
    input: Result<Json<UserInput>, JsonRejection>,
) -> Result<(StatusCode, Json<User>), ApiError> {
    let user = service::create(&pool, body(input)?).await?;

    Ok((StatusCode::CREATED, Json(user)))
}

pub(crate) async fn update_user(
    State(pool): State<PgPool>,
    Path(id): Path<i64>,
    input: Result<Json<UserInput>, JsonRejection>,
) -> Result<Json<User>, ApiError> {
    let user = service::update(&pool, id, body(input)?).await?;

    Ok(Json(user))
}

pub(crate) async fn delete_user(
    State(pool): State<PgPool>,
    Path(id): Path<i64>,
) -> Result<StatusCode, ApiError> {
    service::delete(&pool, id).await?;

    Ok(StatusCode::NO_CONTENT)
}
