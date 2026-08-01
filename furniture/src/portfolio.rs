use serde::Serialize;
use crate::serialize_into_request;
use crate::error::{AppError};
use axum::response::{Response, IntoResponse};

#[derive(Serialize)]
pub struct Project {
    pub project_id: i64,
    pub project_name: String,
    pub note: String,
}

#[derive(Serialize)]
pub struct PortfolioResponse {
    pub portfolio: Vec<Project>,
}

serialize_into_request! {PortfolioResponse}