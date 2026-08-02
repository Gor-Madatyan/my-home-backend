use axum::Json;
use axum::Router;
use axum::extract::{Path, State};
use axum::routing::{delete, put};
use furniture::error::Result;
use furniture::posts::PostDraft;
use sqlx::{Pool, Sqlite, SqliteConnection};
use crate::tags::clean_up_tags;

pub fn get_router() -> Router<Pool<Sqlite>> {
    Router::new()
        .route("/posts", put(put_post))
        .route("/posts/{post_path}", delete(delete_post))
}

async fn put_post(State(pool): State<Pool<Sqlite>>, Json(draft): Json<PostDraft>) -> Result<()> {
    let mut tx = pool.begin().await?;

    let post_id: u32 = if let Some(id) = draft.post_id {
        sqlx::query_scalar!(
            "INSERT OR REPLACE INTO blog_posts (post_id, title, summary, upload_date, revision_date, body, likes)
             VALUES (?, ?, ?, ?, ?, ?, ?)
             RETURNING post_id AS 'post_id:u32'",
            id, draft.title, draft.summary, draft.upload_date, draft.revision_date, draft.body, draft.likes
        )
            .fetch_one(&mut *tx)
            .await?
    } else {
        sqlx::query_scalar!(
            "INSERT OR REPLACE INTO blog_posts (title, summary, upload_date, revision_date, body, likes)
             VALUES (?, ?, ?, ?, ?, ?)
             RETURNING post_id AS 'post_id:u32'",
            draft.title, draft.summary, draft.upload_date, draft.revision_date, draft.body, draft.likes
        )
            .fetch_one(&mut *tx)
            .await?
    };

    for tag in split_tags(&draft.tags) {
        let tag_id = create_tag(&mut *tx, &tag).await?;

        sqlx::query!(
            "INSERT OR IGNORE INTO posts_tags (post_id, tag_id) VALUES (?, ?)",
            post_id,
            tag_id
        )
        .execute(&mut *tx)
        .await?;
    }

    tx.commit().await?;
    Ok(())
}

async fn delete_post(State(pool): State<Pool<Sqlite>>, Path(post_path): Path<u32>) -> Result<()> {
    let mut tx = pool.begin().await?;
    sqlx::query!("DELETE FROM blog_posts WHERE post_id = ?", post_path)
        .execute(&mut *tx)
        .await?;

    sqlx::query!("DELETE FROM posts_tags WHERE post_id = ?", post_path)
        .execute(&mut *tx)
        .await?;

    clean_up_tags(&mut *tx).await?;
    tx.commit().await?;
    Ok(())
}

fn split_tags(tags: &str) -> Vec<String> {
    tags.split(',')
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
        .collect()
}

async fn create_tag(handle: &mut SqliteConnection, tag_name: &str) -> Result<u32> {
    sqlx::query!("INSERT OR IGNORE INTO tags (tag_name) VALUES (?)", tag_name)
        .execute(&mut *handle)
        .await?;

    let id: u32 = sqlx::query_scalar!(
        "SELECT tag_id AS 'tag_id:u32' FROM tags WHERE tag_name = ?",
        tag_name
    )
    .fetch_one(&mut *handle)
    .await?;

    Ok(id)
}
