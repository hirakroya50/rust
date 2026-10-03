use sqlx::PgPool;

use super::{dto::UserInput, models::User};

pub(super) async fn list(pool: &PgPool, limit: i64, offset: i64) -> Result<Vec<User>, sqlx::Error> {
    sqlx::query_as::<_, User>(
        "SELECT id, name, email, phone, created_at, updated_at
         FROM users
         ORDER BY id
         LIMIT $1 OFFSET $2",
    )
    .bind(limit)
    .bind(offset)
    .fetch_all(pool)
    .await
}

pub(super) async fn get(pool: &PgPool, id: i64) -> Result<Option<User>, sqlx::Error> {
    sqlx::query_as::<_, User>(
        "SELECT id, name, email, phone, created_at, updated_at
         FROM users
         WHERE id = $1",
    )
    .bind(id)
    .fetch_optional(pool)
    .await
}

pub(super) async fn create(pool: &PgPool, input: UserInput) -> Result<User, sqlx::Error> {
    sqlx::query_as::<_, User>(
        "INSERT INTO users (name, email, phone)
         VALUES ($1, $2, $3)
         RETURNING id, name, email, phone,
                   created_at, updated_at",
    )
    .bind(input.name)
    .bind(input.email)
    .bind(input.phone)
    .fetch_one(pool)
    .await
}

pub(super) async fn update(
    pool: &PgPool,
    id: i64,
    input: UserInput,
) -> Result<Option<User>, sqlx::Error> {
    sqlx::query_as::<_, User>(
        "UPDATE users
         SET name = $1,
             email = $2,
             phone = $3,
             updated_at = NOW()
         WHERE id = $4
         RETURNING id, name, email, phone,
                   created_at, updated_at",
    )
    .bind(input.name)
    .bind(input.email)
    .bind(input.phone)
    .bind(id)
    .fetch_optional(pool)
    .await
}

pub(super) async fn delete(pool: &PgPool, id: i64) -> Result<bool, sqlx::Error> {
    let result = sqlx::query("DELETE FROM users WHERE id = $1")
        .bind(id)
        .execute(pool)
        .await?;

    Ok(result.rows_affected() > 0)
}
