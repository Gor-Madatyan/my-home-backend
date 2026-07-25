use crate::error::{AppError, Result};
use axum::Router;
use axum::extract::{State};
use axum_extra::extract::{Query};
use axum::response::{IntoResponse, Response};
use axum::routing::get;
use serde::{Deserialize, Serialize};
use sqlx::{Pool, Sqlite};

#[derive(Serialize)]
struct Post {
    post_id: i64,
    title: String,
    summary: String,
    upload_date: String,
    revision_date: String,
    body: String,
    likes: i64,
}

#[derive(Deserialize)]
struct PostsQuery {
    page_size: u8,
    page: u32,
    search: Option<String>,
    #[serde(default)]
    tag:Vec<String>
}

#[derive(Serialize)]
struct PostsResponse {
    posts: Vec<Post>,
}

impl IntoResponse for PostsResponse {
    fn into_response(self) -> Response {
        serde_json::to_string(&self)
            .map_err(|e| AppError::from(e))
            .into_response()
    }
}

pub fn get_router() -> Router<Pool<Sqlite>> {
    Router::new().route("/posts", get(posts))
}

async fn posts(
    State(pool): State<Pool<Sqlite>>,
    Query(PostsQuery {
        page_size,
        page,
        search,
              tag: tags
    }): Query<PostsQuery>,
) -> Result<PostsResponse> {
    println!("{:?}",tags);
    let posts = if let Some(search) = search {
        let search = format!("\"{}\"", search.replace("\"", "\"\""));
        sqlx::query_as!(
            Post,
            "
    SELECT b.post_id,b.title,b.summary,b.upload_date,b.revision_date,b.body,b.likes
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
        .await?
    } else {
        sqlx::query_as!(
            Post,
            "
    SELECT post_id,title,summary,upload_date,revision_date,body,likes
    FROM blog_posts ORDER BY upload_date LIMIT ? OFFSET ?
",
            page_size,
            page * page_size as u32
        )
        .fetch_all(&pool)
        .await?
    };

    Ok(PostsResponse { posts })
}
