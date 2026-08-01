use axum::Router;
use axum::extract::{Query, State};
use axum::routing::get;
use furniture::citations::*;
use furniture::error::Result;
use sqlx::{Pool, Sqlite};

pub fn get_router() -> Router<Pool<Sqlite>> {
    Router::new().route("/citations", get(citations))
}

async fn citations(
    State(pool): State<Pool<Sqlite>>,
    Query(CitationsQuery {
        author,
        source,
        page_size,
        page,
    }): Query<CitationsQuery>,
) -> Result<CitationsResponse> {
    let author = author.unwrap_or_default();
    let source = source.unwrap_or_default();
    let citations = sqlx::query_as!(
        Citation,
        "
    SELECT citation_id,author,source,rizz,body FROM citations
    WHERE author LIKE ? AND source LIKE ?
    ORDER BY rizz DESC
    LIMIT ? OFFSET ?
",
        format!("{}%", author),
        format!("{}%", source),
        page_size,
        page * page_size as u32
    )
    .fetch_all(&pool)
    .await?;
    Ok(CitationsResponse { citations })
}
