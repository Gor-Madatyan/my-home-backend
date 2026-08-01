use axum::Json;
use axum::Router;
use axum::extract::State;
use axum::routing::put;
use furniture::citations::CitationDraft;
use furniture::error::Result;
use sqlx::{Pool, Sqlite};

pub fn get_router() -> Router<Pool<Sqlite>> {
    Router::new().route("/citations", put(put_citation))
}

async fn put_citation(
    State(pool): State<Pool<Sqlite>>,
    Json(draft): Json<CitationDraft>,
) -> Result<()> {
    sqlx::query!(
        "INSERT OR REPLACE INTO citations (author, rizz, source, body) VALUES (?, ?, ?, ?)",
        draft.author,
        draft.rizz,
        draft.source,
        draft.body
    )
    .execute(&pool)
    .await?;
    Ok(())
}
