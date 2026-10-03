mod dto;
mod handlers;
mod models;
mod repository;
mod service;

use axum::{Router, routing::get};
use sqlx::PgPool;

pub(crate) fn routes() -> Router<PgPool> {
    Router::new()
        .route(
            "/products",
            get(handlers::list_products).post(handlers::create_product),
        )
        .route(
            "/products/{id}",
            get(handlers::get_product)
                .put(handlers::update_product)
                .delete(handlers::delete_product),
        )
}
