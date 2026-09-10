use axum::{
    Json,
    extract::{Multipart, Path, Query, State},
};

use crate::{
    api::{APIError, RequestAuth, version},
    application::{
        repository::post::{
            self as post_repo,
            row::{PostRow, PostVisibility},
        },
        service::{
            errors::{AuthServiceError, PostServiceError},
            post_service::PostService,
        },
        state::SharedState,
    },
};
#[derive(serde::Deserialize, sqlx::Type, Debug)]
#[sqlx(rename_all = "lowercase")]
pub enum PostStatus {
    Pending,
    Active,
    InActive,
}

#[derive(serde::Deserialize)]
pub struct FeedQuery {
    pub limit: Option<i32>,
    pub cursor_id: Option<i64>,
}
#[derive(Debug, serde::Deserialize)]
pub struct LikeRequest {
    pub is_like: bool,
}
#[derive(Debug, serde::Deserialize)]
pub struct BookmarkRequest {
    pub is_bookmark: bool,
}
impl PostVisibility {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Everyone => "everyone",
            Self::Friend => "friend",
            Self::Private => "private",
        }
    }
}

#[derive(serde::Deserialize, Debug)]
pub struct UserPostQuery {
    pub limit: Option<i32>,
    pub before: chrono::DateTime<chrono::Utc>,
}

pub async fn get_user_posts_handler(
    State(state): State<SharedState>,
    Path((version, target_id)): Path<(String, i64)>,
    req_auth: RequestAuth,
    query: Query<UserPostQuery>,
) -> Result<Json<Vec<PostRow>>, APIError> {
    let api_version = version::parse_version(&version)?;
    tracing::trace!("api version: {}", api_version);

    let user_id = match req_auth.user {
        Some(user) => Some(user.user_id),
        None => None,
    };

    let posts = PostService
        .get_user_posts(
            &state,
            target_id,
            user_id,
            query.before,
            query.limit.unwrap_or(8) as i32,
        )
        .await?;

    Ok(Json(posts))
}

pub async fn get_feed_post_handler(
    State(state): State<SharedState>,
    Path(version): Path<String>,
    req_auth: RequestAuth,
    query: Query<FeedQuery>,
) -> Result<Json<Vec<PostRow>>, APIError> {
    let api_version = version::parse_version(&version)?;
    tracing::trace!("api version: {}", api_version);

    let user_id = match req_auth.user {
        Some(user) => Some(user.user_id),
        None => return Err(AuthServiceError::InvalidCredentials.into()),
    };

    let feed = PostService
        .get_feed(
            &state,
            user_id.unwrap(),
            query.limit.unwrap_or(8) as i32,
        )
        .await?;

    Ok(Json(feed))
}

pub async fn get_info_post_handler(
    State(state): State<SharedState>,
    Path((version, post_id)): Path<(String, i64)>,
    req_auth: RequestAuth,
) -> Result<Json<PostRow>, APIError> {
    let api_version = version::parse_version(&version)?;
    tracing::trace!("api version: {}", api_version);

    let user_id = match req_auth.user {
        Some(user) => Some(user.user_id),
        None => None,
    };

    let mut tx = state.db_pool.begin().await?;

    let post = PostService.get_post(&state, &mut tx, post_id, user_id).await?;
    tx.commit().await?;
    if let Some(post) = post {
        Ok(Json(post))
    } else {
        Err(PostServiceError::PostNotFound.into())
    }
}

#[axum::debug_handler]
pub async fn create_new_post_handler(
    State(state): State<SharedState>,
    Path(version): Path<String>,
    req_auth: RequestAuth,
    request: Multipart,
) -> Result<Json<PostRow>, APIError> {
    let api_version = version::parse_version(&version)?;
    tracing::trace!("api version: {}", api_version);

    let user_id = match req_auth.user {
        Some(user) => user.user_id,
        None => return Err(AuthServiceError::InvalidCredentials.into()),
    };

    let post = PostService.create_post(&state, user_id, request).await?;
    Ok(Json(post))
}

pub async fn update_post_handler(
    State(state): State<SharedState>,
    Path((version, post_id)): Path<(String, i64)>,
    req_auth: RequestAuth,
    media_src: Multipart,
) -> Result<Json<()>, APIError> {
    let api_version = version::parse_version(&version)?;
    tracing::trace!("api version: {}", api_version);

    let user_id = match req_auth.user {
        Some(user) => user.user_id,
        None => return Err(AuthServiceError::InvalidCredentials.into()),
    };

    PostService.update_post(&state, post_id, user_id, media_src).await?;

    Ok(Json(()))
}

pub async fn delete_post_handler(
    State(state): State<SharedState>,
    Path((version, post_id)): Path<(String, i64)>,
    req_auth: RequestAuth,
) -> Result<(), APIError> {
    let api_version = version::parse_version(&version)?;
    tracing::trace!("api version: {}", api_version);

    let user_id = match req_auth.user {
        Some(user) => user.user_id,
        None => return Err(AuthServiceError::InvalidCredentials.into()),
    };

    PostService.delete_post(&state, post_id, user_id).await?;

    Ok(())
}

pub async fn post_liked_handler(
    State(state): State<SharedState>,
    Path((version, target_id)): Path<(String, i64)>,
    req_auth: RequestAuth,
    Json(req): Json<LikeRequest>,
) -> Result<(), APIError> {
    let api_version = version::parse_version(&version)?;
    tracing::trace!("api version: {}", api_version);

    let user_id = match req_auth.user {
        Some(user) => user.user_id,
        None => return Err(AuthServiceError::InvalidCredentials.into()),
    };

    let mut tx = state.db_pool.begin().await?;
    tracing::info!(user_id, target_id, req.is_like, "liking post");

    post_repo::post::like_post_repo(
        &mut tx,
        user_id,
        target_id,
        req.is_like,
        &"post".to_string(),
    )
    .await?;

    tx.commit().await?;

    Ok(())
}

pub async fn bookmark_handler(
    State(state): State<SharedState>,
    Path((version, target_id)): Path<(String, i64)>,
    req_auth: RequestAuth,
    payload: Json<BookmarkRequest>,
) -> Result<(), APIError> {
    let api_version = version::parse_version(&version)?;
    tracing::trace!("api version: {}", api_version);

    let user_id = match req_auth.user {
        Some(user) => user.user_id,
        None => return Err(AuthServiceError::InvalidCredentials.into()),
    };

    let mut tx = state.db_pool.begin().await?;
    let _bookmark =
        post_repo::post::toggle_bookmark(&mut tx, target_id, user_id, payload.is_bookmark).await?;
    tx.commit().await?;

    Ok(())
}
