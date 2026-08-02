use axum::Router;
use axum::extract::{State, Path};
use axum::routing::get;
use axum::Json;
use furniture::error::Result;
use furniture::portfolio::*;
use sqlx::{Pool, Sqlite};


pub fn get_router() -> Router<Pool<Sqlite>> {
    Router::new()
        .route("/portfolio", get(get_portfolio))
        .route("/portfolio/{id}", get(get_project))
}

async fn get_portfolio(State(pool): State<Pool<Sqlite>>) -> Result<PortfolioResponse> {
    let portfolio = sqlx::query_as!(
        Project,
        "
        SELECT project_id AS 'project_id:u32', rizz AS 'rizz:u32', project_name, note FROM portfolio
        ORDER BY rizz
"
    )
    .fetch_all(&pool)
    .await?;

    Ok(PortfolioResponse { portfolio })
}

async fn get_project(State(pool): State<Pool<Sqlite>>, Path(project_id): Path<u32>) -> Result<Json<Project>> {
    let project = sqlx::query_as!(
        Project,
        "
        SELECT project_id AS 'project_id:u32', rizz AS 'rizz:u32', project_name, note FROM portfolio
        WHERE project_id = ?
        ",
        project_id
    )
    .fetch_one(&pool)
    .await?;

    Ok(Json(project))
}
