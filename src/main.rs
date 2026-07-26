mod posts;
mod error;
pub mod macros;

use anyhow::Result;
use sqlx::sqlite::SqlitePoolOptions;

#[tokio::main]
async fn main() -> Result<()> {
    let connection = SqlitePoolOptions::new()
        .max_connections(5)
        .connect("sqlite:mydb.db")
        .await?;

    let router = posts::get_router().with_state(connection);

    let listener = tokio::net::TcpListener::bind("localhost:8080").await?;
    axum::serve(listener, router).await?;
    Ok(())
}
