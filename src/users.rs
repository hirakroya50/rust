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
            "/users",
            get(handlers::list_users).post(handlers::create_user),
        )
        .route(
            "/users/{id}",
            get(handlers::get_user)
                .put(handlers::update_user)
                .delete(handlers::delete_user),
        )
}
