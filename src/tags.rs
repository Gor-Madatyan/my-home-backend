use crate::error::{AppError, Result};
use crate::serialize_into_request;
use axum::Router;
use axum::extract::{Query, State};
use axum::response::{IntoResponse, Response};
use axum::routing::get;
use serde::{Deserialize, Serialize};
use sqlx::{Pool, Sqlite};

#[derive(Serialize)]
struct Tag {
    tag_name: String,
    tag_id: i64
}

#[derive(Serialize)]
struct TagsResponse {
    tags: Vec<Tag>,
}

#[derive(Deserialize)]
struct GetTagsQuery {
    q: Option<String>,
}

serialize_into_request! {TagsResponse}

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
