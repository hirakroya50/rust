use sqlx::PgPool;

use super::{dto::ProductInput, models::Product, repository};
use crate::{
    error::{ApiError, db_error, not_found},
    pagination::Pagination,
};

pub(super) async fn list(pool: &PgPool, page: Pagination) -> Result<Vec<Product>, ApiError> {
    let (limit, offset) = page.values()?;

    repository::list(pool, limit, offset)
        .await
        .map_err(db_error)
}

pub(super) async fn get(pool: &PgPool, id: i64) -> Result<Product, ApiError> {
    repository::get(pool, id)
        .await
        .map_err(db_error)?
        .ok_or_else(|| not_found("Product"))
}

pub(super) async fn create(pool: &PgPool, input: ProductInput) -> Result<Product, ApiError> {
    let input = input.clean()?;

    repository::create(pool, input).await.map_err(db_error)
}

pub(super) async fn update(
    pool: &PgPool,
    id: i64,
    input: ProductInput,
) -> Result<Product, ApiError> {
    let input = input.clean()?;

    repository::update(pool, id, input)
        .await
        .map_err(db_error)?
        .ok_or_else(|| not_found("Product"))
}

pub(super) async fn delete(pool: &PgPool, id: i64) -> Result<(), ApiError> {
    let deleted = repository::delete(pool, id).await.map_err(db_error)?;

    if !deleted {
        return Err(not_found("Product"));
    }

    Ok(())
}
