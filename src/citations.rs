use axum::Router;
use axum::extract::{Query, State};
use axum::routing::get;
use serde::{Deserialize, Serialize};
use sqlx::{Pool, Sqlite};
use crate::error::{Result,AppError};
use crate::serialize_into_request;
use axum::response::{IntoResponse, Response};

#[derive(Deserialize)]
struct CitationsQuery {
    author: Option<String>,
    source: Option<String>,
}

#[derive(Serialize)]
struct CitationsResponse {
    citations: Vec<Citation>,
}
#[derive(Serialize)]
struct Citation {
    author: String,
    source: String,
    body: String,
}

serialize_into_request!{CitationsResponse}

pub fn get_router() -> Router<Pool<Sqlite>> {
    Router::new().route("/citations", get(citations))
}

async fn citations(
    State(pool): State<Pool<Sqlite>>,
    Query(CitationsQuery { author, source }): Query<CitationsQuery>,
) -> Result<CitationsResponse> {
    let author = author.unwrap_or_default();
    let source = source.unwrap_or_default();
    let citations = sqlx::query_as!(
        Citation,
        "
    SELECT author,source,body FROM citations
    WHERE author LIKE ? AND source LIKE ?
    ORDER BY author
",
        format!("{}%",author),
        format!("{}%",source)
    ).fetch_all(&pool).await?;

     Ok(CitationsResponse{citations})
}
