use axum::{
    Json,
    extract::{Path, Query, State, rejection::JsonRejection},
    http::StatusCode,
};
use sqlx::PgPool;

use super::{dto::ProductInput, models::Product, service};
use crate::{error::ApiError, http::body, pagination::Pagination};

pub(crate) async fn list_products(
    State(pool): State<PgPool>,
    Query(page): Query<Pagination>,
) -> Result<Json<Vec<Product>>, ApiError> {
    Ok(Json(service::list(&pool, page).await?))
}

pub(crate) async fn get_product(
    State(pool): State<PgPool>,
    Path(id): Path<i64>,
) -> Result<Json<Product>, ApiError> {
    Ok(Json(service::get(&pool, id).await?))
}

pub(crate) async fn create_product(
    State(pool): State<PgPool>,
    input: Result<Json<ProductInput>, JsonRejection>,
) -> Result<(StatusCode, Json<Product>), ApiError> {
    let product = service::create(&pool, body(input)?).await?;

    Ok((StatusCode::CREATED, Json(product)))
}

pub(crate) async fn update_product(
    State(pool): State<PgPool>,
    Path(id): Path<i64>,
    input: Result<Json<ProductInput>, JsonRejection>,
) -> Result<Json<Product>, ApiError> {
    let product = service::update(&pool, id, body(input)?).await?;

    Ok(Json(product))
}

pub(crate) async fn delete_product(
    State(pool): State<PgPool>,
    Path(id): Path<i64>,
) -> Result<StatusCode, ApiError> {
    service::delete(&pool, id).await?;

    Ok(StatusCode::NO_CONTENT)
}
