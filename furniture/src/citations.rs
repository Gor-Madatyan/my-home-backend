use crate::error::AppError;
use serde::{Deserialize, Serialize};
use axum::response::{Response, IntoResponse};
use crate::serialize_into_request;

#[derive(Deserialize)]
pub struct CitationsQuery {
    pub author: Option<String>,
    pub source: Option<String>,
    pub page_size:u8,
    pub page:u32
}

#[derive(Serialize)]
pub struct CitationsResponse {
    pub citations: Vec<Citation>,
}
#[derive(Serialize, Debug)]
pub struct Citation {
    pub citation_id:i64,
    pub author: String,
    pub rizz:i64,
    pub source: String,
    pub body: String,
}

serialize_into_request!{CitationsResponse}