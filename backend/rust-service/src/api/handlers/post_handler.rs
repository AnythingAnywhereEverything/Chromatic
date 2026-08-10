use std::ptr::null;

use axum::{
    Json,
    extract::{Multipart, Path, State},
};
use multipart_derive::Multipart;

use crate::{
    api::{APIError, RequestAuth, dtos::post_dtos::PostDTO, handlers::user_handlers::get_current_user_handler, version}, application::{
        repository::{
            media::{self as media_repo, row::MediaStatus},
            post::{self as post_repo},
        }, service::{errors::AuthServiceError, media::{
            multipart_ex::MultipartLimits, service::MediaService, types::{FileSizeGate, MediaOptions, MediaProcessing, MediaType, OnProcessingType, PostProcessingType, ProcessingOptions, ResizeStyle, TempUpload, ValidationOptions, ValidationType},
        }}, state::SharedState,
    },
};
#[derive(serde::Deserialize, sqlx::Type, Debug)]
#[sqlx(type_name = "status", rename_all = "lowercase")]
pub enum PostStatus {
    Pending,
    Active,
    InActive,
}
#[derive(serde::Deserialize, Debug, Multipart)]

pub struct CreatePostRequest {
    pub content: String,
    #[multipart]
    pub multipart: Option<Vec<TempUpload>>,
    pub repost_from: Option<i64>,
    pub status: PostStatus,
    pub visibility: String,
}

pub async fn create_new_post_handler(
    State(state): State<SharedState>,
    Path(version): Path<String>,
    multipart: Multipart,
    req_auth: RequestAuth,
) -> Result<(), APIError> {
    let api_version = version::parse_version(&version)?;
    tracing::trace!("api version: {}", api_version);

    let user_id = match req_auth.user {
        Some(user) => user.user_id,
        None => return Err(AuthServiceError::InvalidCredentials.into()),
    };

    let limits = MultipartLimits {
        max_file_size: 512_000_000,
        max_files: 5,
    };
    let extracted = state
        .multipart_extractor
        .extract::<CreatePostRequest>(multipart, limits)
        .await?;
    tracing::debug!("Extracted payload: {:#?}", extracted);

    let new_post_id = &state.snowflake_generator.generate_id()?;

    let media_options = MediaOptions {
        folder: format!("posts/{}", new_post_id),
        size_gate: Some(vec![
            FileSizeGate{
                media_type: MediaType::Image,
                max_size: 25_000_000, // 25 MB
            },
        ]),
        validation: Some(ValidationOptions {
            validation_type: ValidationType::Whitelisted,
            value: vec![MediaType::Image, MediaType::Video],
        }),
        processing_order: MediaProcessing {
            options: ProcessingOptions {
                // Define processing options here
                ..Default::default()
            },
            on_processing: Some(vec![
                OnProcessingType::ImageResize { 
                    style: ResizeStyle::AbsoluteKeepsRatio { width: 1024, height: 1024 },
                    upscale: false 
                }
            ]),
            post_processing: Some(vec![
                PostProcessingType::VideoHls { segment_duration: 10 },
            ]),
        },
        ..Default::default()
    };


    if let Some(files) = extracted.multipart.as_ref() {
        tracing::debug!("Extracted multipart files: {:#?}", files);

        let all_media = MediaService::save_media_group(&state.media_service, 1234, files.to_vec(), media_options)
            .await?;

        tracing::debug!("Saved media group: {:#?}", all_media);
    }
    let content = extracted.content;
    let repost_from = extracted.repost_from;
    let status = PostStatus::Pending;
    let visibility = &extracted.visibility;
    let is_repost = !repost_from.is_none();

    let mut tx = state.db_pool.begin().await?;
    let new_post = post_repo::post::create_post(&mut tx, new_post_id, user_id, &content, status, repost_from, is_repost ,visibility).await?;
    Ok(())
}
// pub async fn update_post_handler(

// ) -> Result<(), APIError> {
    
// }

pub async fn delete_post_handler(
    State(state): State<SharedState>,
    Path((version, post_id)): Path<(String, i64)>,
    req_auth: RequestAuth,
) -> Result<(), APIError> {
    let api_version = version::parse_version(&version)?;
    tracing::trace!("api version: {}", api_version);
    let mut tx = state.db_pool.begin().await?;
    
    post_repo::post::get_post_by_id(&mut tx, post_id).await?;
    let user_id = match req_auth.user {
        Some(user) => user.user_id,
        None => return Err(AuthServiceError::InvalidCredentials.into()),
    };
    // Delete the post
    post_repo::post::delete_post(&mut tx, post_id, user_id).await?;

    Ok(())
}