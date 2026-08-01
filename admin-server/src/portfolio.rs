use axum::Router;
use axum::routing::{delete, put};
use sqlx::{Pool, Sqlite};

pub fn get_router() -> Router<Pool<Sqlite>> {
    Router::new()
}