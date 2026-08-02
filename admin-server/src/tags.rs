use axum::Router;
use furniture::error::Result;
use sqlx::{Executor, Pool, Sqlite};

pub fn get_router() -> Router<Pool<Sqlite>> {
    Router::new()
}



pub async fn clean_up_tags<'a,E>(pool:E) -> Result<()> where E:Executor<'a,Database=Sqlite>{
    sqlx::query!(
        "DELETE FROM tags
         WHERE NOT EXISTS (
             SELECT 1
             FROM posts_tags
             WHERE posts_tags.tag_id = tags.tag_id
         )"
    )
        .execute(pool)
        .await?;
    Ok(())
}
