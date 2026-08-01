use axum::Router;
use sqlx::{Pool, Sqlite};

pub fn get_router() -> Router<Pool<Sqlite>> {
    Router::new()
}
