use sqlx::PgPool;

use super::{dto::ProductInput, models::Product};

pub(super) async fn list(
    pool: &PgPool,
    limit: i64,
    offset: i64,
) -> Result<Vec<Product>, sqlx::Error> {
    sqlx::query_as::<_, Product>(
        "SELECT id, name, description, price::text AS price,
                stock, category, created_at
         FROM products
         ORDER BY id
         LIMIT $1 OFFSET $2",
    )
    .bind(limit)
    .bind(offset)
    .fetch_all(pool)
    .await
}

pub(super) async fn get(pool: &PgPool, id: i64) -> Result<Option<Product>, sqlx::Error> {
    sqlx::query_as::<_, Product>(
        "SELECT id, name, description, price::text AS price,
                stock, category, created_at
         FROM products
         WHERE id = $1",
    )
    .bind(id)
    .fetch_optional(pool)
    .await
}

pub(super) async fn create(pool: &PgPool, input: ProductInput) -> Result<Product, sqlx::Error> {
    sqlx::query_as::<_, Product>(
        "INSERT INTO products
             (name, description, price, stock, category)
         VALUES ($1, $2, $3::text::numeric, $4, $5)
         RETURNING id, name, description,
                   price::text AS price,
                   stock, category, created_at",
    )
    .bind(input.name)
    .bind(input.description)
    .bind(input.price)
    .bind(input.stock)
    .bind(input.category)
    .fetch_one(pool)
    .await
}

pub(super) async fn update(
    pool: &PgPool,
    id: i64,
    input: ProductInput,
) -> Result<Option<Product>, sqlx::Error> {
    sqlx::query_as::<_, Product>(
        "UPDATE products
         SET name = $1,
             description = $2,
             price = $3::text::numeric,
             stock = $4,
             category = $5
         WHERE id = $6
         RETURNING id, name, description,
                   price::text AS price,
                   stock, category, created_at",
    )
    .bind(input.name)
    .bind(input.description)
    .bind(input.price)
    .bind(input.stock)
    .bind(input.category)
    .bind(id)
    .fetch_optional(pool)
    .await
}

pub(super) async fn delete(pool: &PgPool, id: i64) -> Result<bool, sqlx::Error> {
    let result = sqlx::query("DELETE FROM products WHERE id = $1")
        .bind(id)
        .execute(pool)
        .await?;

    Ok(result.rows_affected() > 0)
}
