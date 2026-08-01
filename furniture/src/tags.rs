use serde::{Deserialize, Serialize};
use crate::serialize_into_request;
use crate::error::AppError;
use axum::response::{Response, IntoResponse};

#[derive(Serialize)]
pub struct Tag {
    pub tag_name: String,
    pub tag_id: i64
}

#[derive(Serialize)]
pub struct TagsResponse {
    pub tags: Vec<Tag>,
}

#[derive(Deserialize)]
pub struct GetTagsQuery {
    pub q: Option<String>,
}

serialize_into_request! {TagsResponse}
