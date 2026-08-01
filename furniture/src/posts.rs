use serde::{Deserialize, Serialize};
use crate::serialize_into_request;
use crate::error::AppError;
use axum::response::{IntoResponse, Response};

#[derive(Serialize)]
pub struct PostPreview {
    pub post_id: i64,
    pub title: String,
    pub summary: String,
    pub upload_date: String,
    pub revision_date: String,
    pub likes: i64,
}

#[derive(Serialize)]
pub struct Post {
    pub post_id: i64,
    pub title: String,
    pub summary: String,
    pub upload_date: String,
    pub revision_date: String,
    pub body: String,
    pub tags: sqlx::types::Json<Vec<String>>,
    pub likes: i64,
}

#[derive(Deserialize)]
pub struct PostsPreviewQuery {
    pub page_size: u8,
    pub page: u32,
    pub search: Option<String>,
    #[serde(default)]
    pub tag: Vec<String>,
}

#[derive(Serialize)]
pub struct PostsPreviewResponse {
    pub posts: Vec<PostPreview>,
}

#[derive(Serialize)]
pub struct PostResponse {
    pub post: Post,
}

serialize_into_request! {PostsPreviewResponse, PostResponse}
