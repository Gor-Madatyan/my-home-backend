use crate::error::{AppError, Result};
use anyhow::anyhow;
use axum::Router;
use axum::extract::State;
use axum::response::{IntoResponse, Response};
use axum::routing::get;
use axum_extra::extract::Query;
use serde::{Deserialize, Serialize};
use sqlx::{Pool, Sqlite};

#[derive(Serialize)]
struct PostPreview {
    post_id: i64,
    title: String,
    summary: String,
    upload_date: String,
    revision_date: String,
    likes: i64,
}

#[derive(Deserialize)]
struct PostsPreviewQuery {
    page_size: u8,
    page: u32,
    search: Option<String>,
    #[serde(default)]
    tag: Vec<String>,
}

#[derive(Serialize)]
struct PostsPreviewResponse {
    posts: Vec<PostPreview>,
}

impl IntoResponse for PostsPreviewResponse {
    fn into_response(self) -> Response {
        serde_json::to_string(&self)
            .map_err(|e| AppError::from(e))
            .into_response()
    }
}

pub fn get_router() -> Router<Pool<Sqlite>> {
    Router::new().route("/posts", get(posts)).route("/posts/{postid}",get(||async { todo!("do something") }))
}

async fn posts(
    State(pool): State<Pool<Sqlite>>,
    Query(PostsPreviewQuery {
        page_size,
        page,
        search,
        tag,
    }): Query<PostsPreviewQuery>,
) -> Result<PostsPreviewResponse> {
    let posts = if let Some(search) = search {
        if tag.len() > 0 {
            return Err(anyhow!("you can't do full text search with tags specified (niche use case to implement, I am lazy)").into());
        }
        fulltext_search(pool, page_size, page, search).await?
    } else {
        select_page(pool, page_size, page, tag).await?
    };

    Ok(PostsPreviewResponse { posts })
}

async fn select_page(
    pool: Pool<Sqlite>,
    page_size: u8,
    page: u32,
    tags: Vec<String>,
) -> Result<Vec<PostPreview>> {
    let posts = if tags.len() > 0 {
        sqlx::query_as!(
            PostPreview,
            "
        SELECT DISTINCT post_id,title,summary,upload_date,revision_date,likes
        FROM blog_posts
        INNER JOIN posts_tags USING(post_id)
        INNER JOIN tags USING(tag_id)
        WHERE tag_name IN (SELECT value FROM json_each(?))
        ORDER BY upload_date LIMIT ? OFFSET ?
    ",
            serde_json::to_string(&tags).unwrap(),
            page_size,
            page * page_size as u32
        )
        .fetch_all(&pool)
        .await?
    } else {
        sqlx::query_as!(
            PostPreview,
            "
        SELECT post_id,title,summary,upload_date,revision_date,likes
        FROM blog_posts
        ORDER BY upload_date LIMIT ? OFFSET ?
    ",
            page_size,
            page * page_size as u32
        )
            .fetch_all(&pool)
            .await?
    };

    Ok(posts)
}
async fn fulltext_search(
    pool: Pool<Sqlite>,
    page_size: u8,
    page: u32,
    search: String,
) -> Result<Vec<PostPreview>> {
    let search = format!("\"{}\"", search.replace("\"", "\"\""));
    let posts = sqlx::query_as!(
        PostPreview,
        "
    SELECT b.post_id,b.title,b.summary,b.upload_date,b.revision_date,b.likes
    FROM blog_posts_fts f
        INNER JOIN blog_posts b ON f.rowid = b.post_id
    WHERE blog_posts_fts MATCH ?
    ORDER BY f.rank
    LIMIT ? OFFSET ?
",
        search,
        page_size,
        page * page_size as u32
    )
    .fetch_all(&pool)
    .await?;

    Ok(posts)
}
