mod citations;
mod error;
pub mod macros;
mod posts;
mod portfolio;

use anyhow::Result;
use sqlx::sqlite::SqlitePoolOptions;
use tower_http::cors::{Any, CorsLayer};

#[tokio::main]
async fn main() -> Result<()> {
    let connection = SqlitePoolOptions::new()
        .max_connections(5)
        .connect("sqlite:mydb.db")
        .await?;

    let cors_layer = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    let router = posts::get_router()
        .merge(citations::get_router())
        .merge(portfolio::get_router())
        .with_state(connection)
        .layer(cors_layer);

    let listener = tokio::net::TcpListener::bind("localhost:8080").await?;
    axum::serve(listener, router).await?;
    Ok(())
}
