use axum::Json;
use axum::Router;
use axum::extract::{Path, State};
use axum::routing::{delete, post, put};
use furniture::error::Result;
use furniture::tags::TagDraft;
use sqlx::{Pool, Sqlite};

pub fn get_router() -> Router<Pool<Sqlite>> {
    Router::new()
        .route("/tags", put(put_tag))
        .route("/tags/{tag_name}", delete(delete_tag))
        .route("/tags/cleanup", post(cleanup_tags))
}

async fn put_tag(State(pool): State<Pool<Sqlite>>, Json(draft): Json<TagDraft>) -> Result<()> {
    if let Some(id) = draft.tag_id {
        sqlx::query!(
            "INSERT OR REPLACE INTO tags (tag_id, tag_name) VALUES (?, ?)",
            id,
            draft.tag_name
        )
        .execute(&pool)
    } else {
        sqlx::query!(
            "INSERT OR REPLACE INTO tags (tag_name) VALUES (?)",
            draft.tag_name
        )
        .execute(&pool)
    }
    .await?;

    Ok(())
}

async fn delete_tag(State(pool): State<Pool<Sqlite>>, Path(tag_name): Path<String>) -> Result<()> {
    sqlx::query!("DELETE FROM tags WHERE tag_name = ?", tag_name)
        .execute(&pool)
        .await?;
    Ok(())
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

