use axum::{
    Json,
    extract::{Multipart, Path, State},
    http::StatusCode,
};
use multipart_derive::Multipart;

use crate::{
    api::{
        APIError, RequestAuth,
        dtos::{post_dtos::CommentDTO, user_dtos::MediaFullDTO},
        handlers::post_handler::MediaTypeAttachment,
        version,
    }, application::{
        repository::{
            media::{self as media_repo, row::MediaStatus},
            post::{self as post_repo, comment},
        }, service::{
            errors::{AuthServiceError, CommentServiceError}, media::{
                extractor::{ExtractorFileOptions, ValidationOptions},
                inspector::{FileType, MediaKind},
                model::{FileContainer, container::ContainerConfig},
                processor::types::{ImageProcessorType, MediaProcessorOptions, ResizeStyle},
            },
        }, state::SharedState,
    },
};

#[derive(serde::Deserialize, Debug, Multipart)]
pub struct CreateCommentRequest {
    pub content: String,
    #[multipart]
    pub media_src: Option<FileContainer>,
}

pub async fn get_comment_handler(
    State(state): State<SharedState>,
    Path((version, post_id)): Path<(String, i64)>,
    req_auth: RequestAuth,
    
) -> Result<Json<Vec<CommentDTO>>, APIError> {
    let api_version = version::parse_version(&version)?;
    tracing::trace!("api version: {}", api_version);

    let user_id = match req_auth.user {
        Some(user) => Some(user.user_id),
        None => None,
    };

    let mut tx = state.db_pool.begin().await?;
    let comments = post_repo::comment::get_comment(&mut tx, post_id, user_id).await?;
    let mut comments_vec: Vec<CommentDTO> = comments.into_iter().map(|post| post.into()).collect(); 
    for comment in & mut comments_vec{
        comment.current_user_id = user_id.map(|id | id.to_string())
    }

    Ok(Json(comments_vec))
}

pub async fn create_new_comment_handler(
    State(state): State<SharedState>,
    Path((version, post_id)): Path<(String, i64)>,
    req_auth: RequestAuth,
    media_src: Multipart,
) -> Result<Json<CommentDTO>, APIError> {
    let api_version = version::parse_version(&version)?;
    tracing::trace!("api version: {}", api_version);

    let user_id = match req_auth.user {
        Some(user) => user.user_id,
        None => return Err(AuthServiceError::InvalidCredentials.into()),
    };

    let ext_opts = ExtractorFileOptions {
        max_files: Some(5),
        max_size: Some(25 * 1024 * 1024), // 25 MB
        validation: Some(
            ValidationOptions::new_whitelist().add_type(FileType::Category(MediaKind::Image)),
        ),
        ..Default::default()
    };

    let mut extracted = state
        .multi_extractor
        .extract::<CreateCommentRequest>(media_src, Some(ext_opts))
        .await?;

    tracing::debug!("Extracted payload: {:#?}", extracted);

    let new_comment_id = state.snowflake_generator.generate_id()?;

    let img_container = &mut extracted.media_src;

    let mut tx = state.db_pool.begin().await?;
    let has_attachment = img_container.is_some();

    let comment = post_repo::comment::create_comment(
        &mut tx,
        &new_comment_id,
        post_id,
        user_id,
        &extracted.content,
        has_attachment,
    )
    .await?;

    if let Some(container) = img_container {
        container
            .prepare_ids(&state.snowflake_generator)?
            .set_uploader_id(user_id)
            .set_target_path(format!("comments/{}", new_comment_id))
            .set_config(
                ContainerConfig::new()
                    .set_generate_thumbhash(true)
                    .set_processing_options(MediaProcessorOptions::new().set_image_processors(
                        vec![ImageProcessorType::Resize {
                            style: ResizeStyle::Absolute {
                                width: 1024,
                                height: 1024,
                            },
                            upscale: false,
                        }],
                    )),
            );

        state
            .media_service
            .save_media(&state, container)
            .await?;

        // save all the id state
        for file in container.files_mut() {
            media_repo::update::media_status(&mut tx, &file.id().unwrap(), &MediaStatus::Completed)
                .await?;
            post_repo::post::add_has_attachment(
                &mut tx,
                comment.id,
                file.id().unwrap(),
                MediaTypeAttachment::Comment.as_str().to_string(),
            )
            .await?;
        }
    }

    let media_with_post = post_repo::post::get_post_attachment(&mut tx, new_comment_id).await?;
    let media = media_with_post
        .into_iter()
        .map(|media| MediaFullDTO {
            id: media.id.to_string(),
            path: media.path,
            name: media.name,
            thumbhash: media.thumbhash,
            status: media.status.to_string(),
            created_at: media.created_at,
            file_size: media.file_size,
            flags: media.flags,
            mime_type: media.mime_type,
            width: media.width,
            height: media.height,
            duration: media.duration,
        })
        .collect::<Vec<_>>();
    tracing::warn!("Updated Post after upload: {:#?}", media);

    tx.commit().await?;
    todo!();
    // Ok(Json(get_comment))
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
    Path((version, comment_id)): Path<(String, i64)>,
    req_auth: RequestAuth,
) -> Result<StatusCode, APIError> {
    let _api_version = version::parse_version(&version)?;

    let user_id = match req_auth.user {
        Some(user) => user.user_id,
        // ! hardcoded fallback id might be a security risk in production
        None => 81727418892554240,
    };
    let mut tx = state.db_pool.begin().await?;

    let deleted = post_repo::comment::delete_comment(&mut tx, comment_id, user_id).await?;
    tx.commit().await?;

    if deleted == 0 {
        return Err(CommentServiceError::CommentNotFoundOrUnauthorized.into());
    }

    Ok(StatusCode::NO_CONTENT)
}
