use serde::{Deserialize, Serialize};
use crate::serialize_into_request;
use crate::error::AppError;
use axum::response::{Response, IntoResponse};

#[derive(Deserialize)]
pub struct TagDraft {
    pub tag_name: String,
    pub tag_id: Option<u32>
}

#[derive(Serialize)]
pub struct Tag {
    pub tag_name: String,
    pub tag_id: u32
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
