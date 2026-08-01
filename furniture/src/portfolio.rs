use serde::{Deserialize, Serialize};
use crate::serialize_into_request;
use crate::error::{AppError};
use axum::response::{Response, IntoResponse};

#[derive(Deserialize)]
pub struct ProjectDraft {
    pub project_id: Option<u32>,
    pub rizz:u32,
    pub project_name: String,
    pub note: String,
}

#[derive(Serialize)]
pub struct Project {
    pub project_id: u32,
    pub rizz:u32,
    pub project_name: String,
    pub note: String,
}

#[derive(Serialize)]
pub struct PortfolioResponse {
    pub portfolio: Vec<Project>,
}

serialize_into_request! {PortfolioResponse}