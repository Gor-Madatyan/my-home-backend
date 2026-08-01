use anyhow::anyhow;
use axum::Router;
use axum::extract::{Path, State};
use axum::routing::{get, put};
use axum_extra::extract::Query;
use furniture::error::Result;
use furniture::sanitize;
use sqlx::{Executor, Pool, Sqlite};
use furniture::posts::*;


pub fn get_router() -> Router<Pool<Sqlite>> {
    Router::new()
        .route("/posts", get(posts))
        .route("/posts/{postid}", get(get_post))
        .route("/posts/{postid}/like", put(like_post))
        .route("/posts/{postid}/unlike", put(unlike_post))
}

async fn get_post(
    State(pool): State<Pool<Sqlite>>,
    Path(post_id): Path<u32>,
) -> Result<PostResponse> {
    let post = sqlx::query_as!(
        Post,
    "
    SELECT  b.post_id AS 'post_id:u32', b.likes AS 'likes:u32', b.title, b.summary, b.upload_date, b.revision_date, b.body,
        json_group_array(t.tag_name) FILTER (WHERE t.tag_name IS NOT NULL) AS 'tags!: sqlx::types::Json<Vec<String>>'
    FROM blog_posts b
    LEFT JOIN posts_tags pt USING(post_id)
    LEFT JOIN tags t USING(tag_id)
    WHERE b.post_id = ?
    GROUP BY b.post_id;
",post_id).fetch_one(&pool).await?;
    Ok(PostResponse { post })
}

async fn like_post(State(pool): State<Pool<Sqlite>>, Path(post_id): Path<u32>) -> Result<()> {
    pool.execute(sqlx::query_as!(
        Post,
    "
    UPDATE blog_posts
    SET likes = likes+1
    WHERE post_id = ?
",post_id)).await?;
    Ok(())
}

async fn unlike_post(State(pool): State<Pool<Sqlite>>, Path(post_id): Path<u32>) -> Result<()> {
    pool.execute(sqlx::query_as!(
        Post,
    "
    UPDATE blog_posts
    SET likes = MAX(0,likes-1)
    WHERE post_id = ?
",post_id)).await?;
    Ok(())
}

async fn posts(
    State(pool): State<Pool<Sqlite>>,
    Query(PostsPreviewQuery {
        page_size,
        page,
        search,
        tag,
    }): Query<PostsPreviewQuery>,
) -> Result<PostsPreviewResponse> {
    let posts = if let Some(search) = search {
        if tag.len() > 0 {
            return Err(anyhow!("you can't do full text search with tags specified (niche use case to implement, I am lazy)").into());
        }
        fulltext_search(pool, page_size, page, search).await?
    } else {
        select_page(pool, page_size, page, tag).await?
    };

    Ok(PostsPreviewResponse { posts })
}

async fn select_page(
    pool: Pool<Sqlite>,
    page_size: u8,
    page: u32,
    tags: Vec<String>,
) -> Result<Vec<PostPreview>> {
    let posts = if tags.len() > 0 {
        sqlx::query_as!(
            PostPreview,
            "
        SELECT DISTINCT post_id,title,summary,upload_date,revision_date,likes
        FROM blog_posts
        INNER JOIN posts_tags USING(post_id)
        INNER JOIN tags USING(tag_id)
        WHERE tag_name IN (SELECT value FROM json_each(?))
        ORDER BY upload_date DESC LIMIT ? OFFSET ?
    ",
            serde_json::to_string(&tags).unwrap(),
            page_size,
            page * page_size as u32
        )
        .fetch_all(&pool)
        .await?
    } else {
        sqlx::query_as!(
            PostPreview,
            "
        SELECT post_id,title,summary,upload_date,revision_date,likes
        FROM blog_posts
        ORDER BY revision_date DESC LIMIT ? OFFSET ?
    ",
            page_size,
            page * page_size as u32
        )
        .fetch_all(&pool)
        .await?
    };

    Ok(posts)
}
async fn fulltext_search(
    pool: Pool<Sqlite>,
    page_size: u8,
    page: u32,
    search: String,
) -> Result<Vec<PostPreview>> {
    sanitize! {search}
    let posts = sqlx::query_as!(
        PostPreview,
        "
    SELECT b.post_id,b.title,b.summary,b.upload_date,b.revision_date,b.likes
    FROM blog_posts_fts f
        INNER JOIN blog_posts b ON f.rowid = b.post_id
    WHERE blog_posts_fts MATCH ?
    ORDER BY f.rank
    LIMIT ? OFFSET ?
",
        search,
        page_size,
        page * page_size as u32
    )
    .fetch_all(&pool)
    .await?;

    Ok(posts)
}
