mod citations;
mod portfolio;
mod posts;
mod tags;

use sqlx::sqlite::SqlitePoolOptions;
use std::env;
use dotenvy::dotenv;
use tower_http::validate_request::ValidateRequestHeaderLayer;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv()?;
    let api_key = env::var("API_KEY").expect("API_KEY must be set");
    let connection = SqlitePoolOptions::new()
        .max_connections(5)
        .connect("sqlite:mydb.db")
        .await?;

    let router = citations::get_router()
        .merge(portfolio::get_router())
        .merge(tags::get_router())
        .merge(posts::get_router())
        .with_state(connection)
        .route_layer(ValidateRequestHeaderLayer::has_header_value(
            "X-API-KEY",
            &api_key,
        )?);

    let listener = tokio::net::TcpListener::bind("localhost:9090").await?;
    axum::serve(listener, router).await?;
    Ok(())
}
