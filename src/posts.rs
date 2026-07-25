use anyhow::{anyhow};
use axum::Router;
use axum::extract::{Query, State};
use axum::routing::get;
use serde::Deserialize;
use sqlx::{Pool, Sqlite};
use crate::error::{AppError,Result};

#[derive(Deserialize)]
struct PostsQuery {
    page_size: u8,
    page: u32,
    search: Option<String>,
}

pub fn get_router() -> Router<Pool<Sqlite>> {
    Router::new().route(
        "/posts",
        get(posts),
    )
}

async fn posts(State(pool): State<Pool<Sqlite>>,
               Query(PostsQuery {
                         page_size,
                         page,
                         search,
                     }): Query<PostsQuery>) -> Result<()>{
    Err(anyhow!("iya"))?;
    Ok(())
}