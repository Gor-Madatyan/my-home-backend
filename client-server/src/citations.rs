use axum::Router;
use axum::extract::{Path, Query, State};
use axum::routing::get;
use furniture::citations::*;
use furniture::error::Result;
use sqlx::{Pool, Sqlite};

pub fn get_router() -> Router<Pool<Sqlite>> {
    Router::new()
        .route("/citations", get(citations))
        .route("/citations/{citation_id}", get(citation))
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
    SELECT citation_id AS 'citation_id: u32',author,source,rizz AS 'rizz: u16',body FROM citations
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

async fn citation(
    State(pool): State<Pool<Sqlite>>,
    Path(citation_id): Path<u32>,
) -> Result<CitationResponse> {
    let citation = sqlx::query_as!(
        Citation,
        "
    SELECT citation_id AS 'citation_id: u32',author,source,rizz AS 'rizz: u16',body FROM citations
    WHERE citation_id = ?
",
        citation_id
    )
    .fetch_one(&pool)
    .await?;

    Ok(
        CitationResponse{
            citation
        }
    )
}
