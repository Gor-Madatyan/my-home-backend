use serde::{Deserialize, Serialize};
use crate::serialize_into_request;
use crate::error::AppError;
use axum::response::{IntoResponse, Response};

#[derive(Serialize)]
pub struct PostPreview {
    pub post_id: i64,
    pub title: String,
    summary: String,
    upload_date: String,
    revision_date: String,
    likes: i64,
}

#[derive(Serialize)]
struct Post {
    post_id: i64,
    title: String,
    summary: String,
    upload_date: String,
    revision_date: String,
    body: String,
    tags: sqlx::types::Json<Vec<String>>,
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

#[derive(Serialize)]
struct PostResponse {
    post: Post,
}

serialize_into_request! {PostsPreviewResponse, PostResponse}