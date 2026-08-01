mod citations;
mod portfolio;
mod tags;

use sqlx::sqlite::SqlitePoolOptions;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let connection = SqlitePoolOptions::new()
        .max_connections(5)
        .connect("sqlite:mydb.db")
        .await?;

    let router = citations::get_router()
        .merge(portfolio::get_router())
        .merge(tags::get_router())
        .with_state(connection);

    let listener = tokio::net::TcpListener::bind("localhost:9090").await?;
    axum::serve(listener, router).await?;
    Ok(())
}
