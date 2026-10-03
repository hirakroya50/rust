use chrono::{DateTime, Utc};
use serde::Serialize;
use sqlx::FromRow;

#[derive(Serialize, FromRow)]
pub(crate) struct User {
    pub(super) id: i64,
    pub(super) name: String,
    pub(super) email: String,
    pub(super) phone: Option<String>,
    pub(super) created_at: DateTime<Utc>,
    pub(super) updated_at: DateTime<Utc>,
}
