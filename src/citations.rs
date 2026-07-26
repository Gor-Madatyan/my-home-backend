use axum::Router;
use axum::extract::{Query, State};
use axum::routing::get;
use serde::{Deserialize, Serialize};
use sqlx::{Pool, Sqlite};
use crate::error::{Result,AppError};
use crate::{sanitize, serialize_into_request};
use axum::response::{IntoResponse, Response};

#[derive(Deserialize)]
struct CitationsQuery {
    author: Option<String>,
    source: Option<String>,
    page_size:u8,
    page:u32
}

#[derive(Serialize)]
struct CitationsResponse {
    citations: Vec<Citation>,
}
#[derive(Serialize)]
struct Citation {
    citation_id:i64,
    author: String,
    rizz:i64,
    source: String,
    body: String,
}

serialize_into_request!{CitationsResponse}

pub fn get_router() -> Router<Pool<Sqlite>> {
    Router::new().route("/citations", get(citations))
}

async fn citations(
    State(pool): State<Pool<Sqlite>>,
    Query(CitationsQuery { author, source, page_size, page }): Query<CitationsQuery>,
) -> Result<CitationsResponse> {
    let author = author.unwrap_or_default();
    let source = source.unwrap_or_default();
    sanitize!{author,source}
    let citations = sqlx::query_as!(
        Citation,
        "
    SELECT citation_id,author,source,rizz,body FROM citations
    WHERE author LIKE ? AND source LIKE ?
    ORDER BY rizz DESC
    LIMIT ? OFFSET ?
",
        format!("{}%",author),
        format!("{}%",source),
        page_size,
        page*page_size as u32
    ).fetch_all(&pool).await?;

     Ok(CitationsResponse{citations})
}
