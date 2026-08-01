use axum::Json;
use axum::Router;
use axum::extract::{Path, State};
use axum::routing::{delete, put};
use furniture::citations::CitationDraft;
use furniture::error::Result;
use sqlx::{Pool, Sqlite};

pub fn get_router() -> Router<Pool<Sqlite>> {
    Router::new()
        .route("/citations", put(put_citation))
        .route("/citations/{citation_path}", delete(delete_citation))
}

async fn put_citation(
    State(pool): State<Pool<Sqlite>>,
    Json(draft): Json<CitationDraft>,
) -> Result<()> {
    if let Some(id) = draft.citation_id {
        sqlx::query!(
            "INSERT OR REPLACE INTO citations (citation_id, author, rizz, source, body) VALUES (?, ?, ?, ?, ?)",
            id,
            draft.author,
            draft.rizz,
            draft.source,
            draft.body
        )
        .execute(&pool)
    } else {
        sqlx::query!(
            "INSERT OR REPLACE INTO citations (author, rizz, source, body) VALUES (?, ?, ?, ?)",
            draft.author,
            draft.rizz,
            draft.source,
            draft.body
        )
        .execute(&pool)
    }.await?;
    Ok(())
}

async fn delete_citation(
    State(pool): State<Pool<Sqlite>>,
    Path(citation_path): Path<u32>,
) -> Result<()> {
    sqlx::query!(
        "DELETE FROM citations WHERE citation_id = ?",
        citation_path
    )
    .execute(&pool)
    .await?;
    Ok(())
}
