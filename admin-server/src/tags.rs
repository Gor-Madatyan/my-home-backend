use axum::extract::State;
use axum::Router;
use axum::routing::post;
use furniture::error::Result;
use sqlx::{Pool, Sqlite};

pub fn get_router() -> Router<Pool<Sqlite>> {
    Router::new().route("/tags/cleanup", post(cleanup_tags))
}


async fn cleanup_tags(State(pool): State<Pool<Sqlite>>) -> Result<()> {
    sqlx::query!(
        "DELETE FROM tags
         WHERE NOT EXISTS (
             SELECT 1
             FROM posts_tags
             WHERE posts_tags.tag_id = tags.tag_id
         )"
    )
    .execute(&pool)
    .await?;
    Ok(())
}

