use axum::{
    Json, extract::{Multipart, Path, Query, State},
};
use multipart_derive::Multipart;

use crate::{
    api::{
        APIError, RequestAuth, dtos::post_dtos::CommentDTO, handlers::post_handler::LikeRequest,
        version,
    },
    application::{
        repository::{
            media::{
                row::{MediaType},
            },
            post::{
                self as post_repo,
                row::{CommentRow},
            },
        },
        service::{
            errors::{AuthServiceError, CommentServiceError},
            media::{
                extractor::{ExtractorFileOptions, ValidationOptions},
                inspector::FileType,
            },
            post_service::PostService,
        },
        state::SharedState,
    },
};

#[derive(serde::Deserialize, Debug, Multipart)]
pub struct CreateCommentRequest {
    pub content: String,
}

#[derive(serde::Deserialize, Debug)]
pub struct CommentQuery {
    pub before: chrono::DateTime<chrono::Utc>,
    pub limit: i64,
}

pub async fn get_comment_handler(
    State(state): State<SharedState>,
    Path((version, post_id)): Path<(String, i64)>,
    req_auth: RequestAuth,
    Query(query): Query<CommentQuery>,
) -> Result<Json<Vec<CommentRow>>, APIError> {
    let api_version = version::parse_version(&version)?;
    tracing::trace!("api version: {}", api_version);

    let user_id = match req_auth.user {
        Some(user) => user.user_id,
        None => return Err(AuthServiceError::InvalidCredentials.into()),
    };

    let comments = PostService
        .get_comments(&state, post_id, user_id, query.before, query.limit)
        .await?;

    Ok(Json(comments))
}

pub async fn create_new_comment_handler(
    State(state): State<SharedState>,
    Path((version, post_id)): Path<(String, i64)>,
    req_auth: RequestAuth,
    files: Multipart,
) -> Result<Json<CommentRow>, APIError> {
    let api_version = version::parse_version(&version)?;
    tracing::trace!("api version: {}", api_version);

    let user_id = match req_auth.user {
        Some(user) => user.user_id,
        None => return Err(AuthServiceError::InvalidCredentials.into()),
    };

    let ext_opts = ExtractorFileOptions {
        max_files: Some(1),
        max_size: Some(25 * 1024 * 1024), // 25 MB
        validation: Some(
            ValidationOptions::new_whitelist().add_type(FileType::Category(MediaType::Image)),
        ),
        ..Default::default()
    };

    let extracted = state
        .multi_extractor
        .extract::<CreateCommentRequest>(files, Some(ext_opts))
        .await?;

    tracing::debug!("Extracted payload: {:#?}", extracted);

    let new_comment_id = state.snowflake_generator.generate_id()?;

    let mut tx = state.db_pool.begin().await?;
    let has_attachment = false;

    post_repo::comment::create_comment(
        &mut tx,
        &new_comment_id,
        post_id,
        user_id,
        &extracted.content,
        has_attachment,
    )
    .await?;

    let mut tx = state.db_pool.begin().await?;
    let get_comment =
        PostService.get_comment(&state, &mut tx, new_comment_id, user_id).await?;
    tx.commit().await?;

    Ok(Json(get_comment))
}

pub async fn update_comment_handler(
    State(_state): State<SharedState>,
    Path((version, _comment_id)): Path<(String, i64)>,
    req_auth: RequestAuth,
    _media_src: Multipart,
) -> Result<Json<CommentDTO>, APIError> {
    let api_version = version::parse_version(&version)?;
    tracing::trace!("api version: {}", api_version);

    let _ = match req_auth.user {
        Some(user) => user.user_id,
        None => return Err(AuthServiceError::InvalidCredentials.into()),
    };

    todo!();
}

pub async fn delete_comment_handler(
    State(state): State<SharedState>,
    Path((version, _post_id, comment_id)): Path<(String, i64, i64)>,
    req_auth: RequestAuth,
) -> Result<(), APIError> {
    let _api_version = version::parse_version(&version)?;

    let user_id = match req_auth.user {
        Some(user) => user.user_id,
        None => return Err(AuthServiceError::InvalidCredentials.into()),
    };
    let mut tx = state.db_pool.begin().await?;

    let deleted = post_repo::comment::delete_comment(&mut tx, comment_id, user_id).await?;
    tx.commit().await?;

    if deleted == 0 {
        return Err(CommentServiceError::CommentNotFoundOrUnauthorized.into());
    }

    Ok(())
}

pub async fn liked_comment_handler(
    State(state): State<SharedState>,
    Path((version, _post_id, comment_id)): Path<(String, i64, i64)>,
    req_auth: RequestAuth,
    Json(payload): Json<LikeRequest>,
) -> Result<(), APIError> {
    let api_version = version::parse_version(&version)?;
    tracing::trace!("api version: {}", api_version);

    let user_id = match req_auth.user {
        Some(user) => user.user_id,
        None => return Err(AuthServiceError::InvalidCredentials.into()),
    };

    tracing::info!(
        user_id,
        comment_id,
        payload.is_like,
        "received like request for comment"
    );
    let mut tx = state.db_pool.begin().await?;
    tracing::info!(user_id, comment_id, payload.is_like, "liking comment");

    post_repo::comment::liked_comment(&mut tx, comment_id, user_id, payload.is_like).await?;

    tx.commit().await?;

    Ok(())
}
