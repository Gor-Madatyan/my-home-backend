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
    pub citation_id:u32,
    pub author: String,
    pub rizz:u16,
    pub source: String,
    pub body: String,
}

#[derive(Deserialize, Debug)]
pub struct CitationDraft {
    pub citation_id:Option<u32>,
    pub author: String,
    pub rizz:u16,
    pub source: String,
    pub body: String,
}


#[derive(Serialize)]
pub struct CitationResponse {
    pub citation: Citation,
}

serialize_into_request!{CitationsResponse, CitationResponse}