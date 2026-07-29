use crate::error::{AppError, Result};
use crate::serialize_into_request;
use axum::Router;
use axum::extract::State;
use axum::response::{IntoResponse, Response};
use axum::routing::get;
use serde::Serialize;
use sqlx::{Pool, Sqlite};

#[derive(Serialize)]
struct Project {
    project_id: i64,
    project_name: String,
    rizz: i64,
    note: String,
}

#[derive(Serialize)]
struct PortfolioResponse {
    portfolio: Vec<Project>,
}

serialize_into_request! {PortfolioResponse}

pub fn get_router() -> Router<Pool<Sqlite>> {
    Router::new().route("/portfolio", get(get_portfolio))
}

async fn get_portfolio(State(pool): State<Pool<Sqlite>>) -> Result<PortfolioResponse> {
    let portfolio = sqlx::query_as!(
        Project,
        "
        SELECT * FROM portfolio
        ORDER BY rizz
"
    )
    .fetch_all(&pool)
    .await?;

    Ok(PortfolioResponse { portfolio })
}
