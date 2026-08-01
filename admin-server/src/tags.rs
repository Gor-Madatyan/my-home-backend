use axum::Json;
use axum::Router;
use axum::extract::{Path, State};
use axum::routing::{delete, put};
use furniture::error::Result;
use furniture::tags::TagDraf;
use sqlx::{Pool, Sqlite};

pub fn get_router() -> Router<Pool<Sqlite>> {
    Router::new()
        .route("/tags", put(put_tag))
        .route("/tags/{tag_path}", delete(delete_tag))
}

async fn put_tag(State(pool): State<Pool<Sqlite>>, Json(draft): Json<TagDraf>) -> Result<()> {
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

async fn delete_tag(State(pool): State<Pool<Sqlite>>, Path(tag_path): Path<u32>) -> Result<()> {
    sqlx::query!("DELETE FROM tags WHERE tag_id = ?", tag_path)
        .execute(&pool)
        .await?;
    Ok(())
}

pub async fn create_tag(pool: &Pool<Sqlite>, tag_name: &str) -> Result<u32> {
    sqlx::query!(
        "INSERT OR IGNORE INTO tags (tag_name) VALUES (?)",
        tag_name
    )
    .execute(pool)
    .await?;

    let row = sqlx::query!(
        "SELECT tag_id FROM tags WHERE tag_name = ?",
        tag_name
    )
    .fetch_one(pool)
    .await?;

    Ok(row.tag_id as u32)
}
