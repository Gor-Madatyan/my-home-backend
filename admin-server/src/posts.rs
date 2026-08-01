use axum::Router;
use axum::extract::{Path, State};
use axum::routing::{delete, put};
use axum::Json;
use furniture::error::Result;
use furniture::posts::PostDraft;
use sqlx::{Pool, Sqlite};

pub fn get_router() -> Router<Pool<Sqlite>> {
    Router::new()
        .route("/posts", put(put_post))
        .route("/posts/{post_path}", delete(delete_post))
}

async fn put_post(
    State(pool): State<Pool<Sqlite>>,
    Json(draft): Json<PostDraft>,
) -> Result<()> {
    let mut tx = pool.begin().await?;

    if let Some(id) = draft.post_id {
        sqlx::query!(
            "INSERT OR REPLACE INTO blog_posts (post_id, title, summary, upload_date, revision_date, body, likes) VALUES (?, ?, ?, ?, ?, ?, ?)",
            id,
            draft.title,
            draft.summary,
            draft.upload_date,
            draft.revision_date,
            draft.body,
            draft.likes
        )
        .execute(&mut *tx)
        .await?;
    } else {
        sqlx::query!(
            "INSERT OR REPLACE INTO blog_posts (title, summary, upload_date, revision_date, body, likes) VALUES (?, ?, ?, ?, ?, ?)",
            draft.title,
            draft.summary,
            draft.upload_date,
            draft.revision_date,
            draft.body,
            draft.likes
        )
        .execute(&mut *tx)
        .await?;
    }

    let post_id: i64 = match draft.post_id {
        Some(id) => id as i64,
        None => {
            let row = sqlx::query!("SELECT last_insert_rowid() AS id")
                .fetch_one(&mut *tx)
                .await?;
            row.id
        }
    };

    for tag in split_tags(&draft.tags) {
        let tag_id = crate::tags::create_tag(&pool, &tag).await?;
        sqlx::query!(
            "INSERT OR IGNORE INTO posts_tags (post_id, tag_id) VALUES (?, ?)",
            post_id as u32,
            tag_id
        )
        .execute(&mut *tx)
        .await?;
    }

    tx.commit().await?;
    Ok(())
}

async fn delete_post(
    State(pool): State<Pool<Sqlite>>,
    Path(post_path): Path<u32>,
) -> Result<()> {
    sqlx::query!(
        "DELETE FROM blog_posts WHERE post_id = ?",
        post_path
    )
    .execute(&pool)
    .await?;
    Ok(())
}

fn split_tags(tags: &str) -> Vec<String> {
    tags.split(',')
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
        .collect()
}
