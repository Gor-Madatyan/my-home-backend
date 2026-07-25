use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use thiserror::Error;

pub type Result<T> = std::result::Result<T,AppError>;

#[derive(Error, Debug)]
pub enum AppError {
    #[error(transparent)]
    Other(#[from] anyhow::Error),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        match self {
            AppError::Other(_) => {
                (StatusCode::INTERNAL_SERVER_ERROR, "failed".to_string()).into_response()
            }
        }
    }
}
