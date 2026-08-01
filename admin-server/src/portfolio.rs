use axum::Json;
use axum::Router;
use axum::extract::{Path, State};
use axum::routing::{delete, put};
use furniture::error::Result;
use furniture::portfolio::ProjectDraft;
use sqlx::{Pool, Sqlite};

pub fn get_router() -> Router<Pool<Sqlite>> {
    Router::new()
        .route("/portfolio", put(put_project))
        .route("/portfolio/{project_path}", delete(delete_project))
}

async fn put_project(
    State(pool): State<Pool<Sqlite>>,
    Json(draft): Json<ProjectDraft>,
) -> Result<()> {
    if let Some(id) = draft.project_id {
        sqlx::query!(
            "INSERT OR REPLACE INTO portfolio (project_id, rizz, project_name, note) VALUES (?, ?, ?, ?)",
            id,
            draft.rizz,
            draft.project_name,
            draft.note
        )
        .execute(&pool)
    } else {
        sqlx::query!(
            "INSERT OR REPLACE INTO portfolio (rizz, project_name, note) VALUES (?, ?, ?)",
            draft.rizz,
            draft.project_name,
            draft.note
        )
        .execute(&pool)
    } .await?;
    Ok(())
}

async fn delete_project(
    State(pool): State<Pool<Sqlite>>,
    Path(project_path): Path<u32>,
) -> Result<()> {
    sqlx::query!("DELETE FROM portfolio WHERE project_id = ?", project_path)
        .execute(&pool)
        .await?;
    Ok(())
}
