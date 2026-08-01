use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use thiserror::Error;

pub type Result<T> = std::result::Result<T,AppError>;

#[derive(Error, Debug)]
pub enum AppError {
    #[error(transparent)]
    SqlError(#[from] sqlx::error::Error),
    #[error(transparent)]
    JsonError(#[from] serde_json::error::Error),
    #[error(transparent)]
    Other(#[from] anyhow::Error),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        match self {
            AppError::SqlError(e) =>{
                (StatusCode::INTERNAL_SERVER_ERROR, format!("{}",e)).into_response()
            }
            AppError::JsonError(e) =>{
                (StatusCode::INTERNAL_SERVER_ERROR, format!("{}",e)).into_response()
            }
            AppError::Other(e) => {
                (StatusCode::INTERNAL_SERVER_ERROR, format!("{}",e)).into_response()
            }
        }
    }
}
