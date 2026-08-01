use axum::Router;
use axum::extract::{Query, State};
use axum::routing::get;
use furniture::error::Result;
use furniture::tags::*;
use sqlx::{Pool, Sqlite};


pub fn get_router() -> Router<Pool<Sqlite>> {
    Router::new().route("/tags", get(get_tags))
}

async fn get_tags(
    State(pool): State<Pool<Sqlite>>,
    Query(GetTagsQuery { q }): Query<GetTagsQuery>,
) -> Result<TagsResponse> {
    let tags = sqlx::query_as!(
        Tag,
        "
    SELECT tag_id, tag_name FROM tags
    WHERE tag_name LIKE ?;
",
        format!("{}%",q.unwrap_or_default())
    )
    .fetch_all(&pool)
    .await?;

    Ok(TagsResponse { tags })
}
