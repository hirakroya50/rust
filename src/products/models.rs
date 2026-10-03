use chrono::{DateTime, Utc};
use serde::Serialize;
use sqlx::FromRow;

#[derive(Serialize, FromRow)]
pub(crate) struct Product {
    pub(super) id: i64,
    pub(super) name: String,
    pub(super) description: Option<String>,
    pub(super) price: String,
    pub(super) stock: i32,
    pub(super) category: Option<String>,
    pub(super) created_at: DateTime<Utc>,
}
