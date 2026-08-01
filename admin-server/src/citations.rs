use axum::Router;
use axum::extract::State;
use axum::routing::put;
use axum::Json;
use furniture::citations::CitationDraft;
use furniture::error::Result;
use sqlx::{Pool, Sqlite};

pub fn get_router() -> Router<Pool<Sqlite>>{
    Router::new()
        .route("/citations", put(create_citation))
}

async fn create_citation(
    State(pool): State<Pool<Sqlite>>,
    Json(draft): Json<CitationDraft>,
) -> Result<()> {
    sqlx::query(
        "INSERT OR REPLACE INTO citations (author, rizz, source, body) VALUES (?, ?, ?, ?)"
    )
    .bind(&draft.author)
    .bind(draft.rizz)
    .bind(&draft.source)
    .bind(&draft.body)
    .execute(&pool)
    .await?;
    Ok(())
}
