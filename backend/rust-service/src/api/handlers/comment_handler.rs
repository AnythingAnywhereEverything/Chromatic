use axum::{
    extract::{Multipart, Path, State},
    http::StatusCode,
    Json,
};
use multipart_derive::Multipart;

use crate::{
    api::{APIError, RequestAuth, dtos::{post_dtos::CommentDTO, user_dtos::MediaFullDTO}, handlers::post_handler::MediaTypeAttachment, version}, application::{
        repository::{
            media::{self as media_repo, row::MediaStatus}, post::{self as post_repo, comment},
        }, service::{ errors::CommentServiceError, media::{
                processor::types::{
                    ImageProcessorType, MediaProcessorFFlags, MediaProcessorOptions,
                    PostProcessingType, ResizeStyle, VideoPostProcessorType,
                },
                service::MediaService,
                service_type::MediaServiceOptions,
                types::{
                    file::MultipartFile,
                    media_options::{
                        FieldTypeFilter, MediaType, MultipartExtractorOptions, ValidationOptions,
                        ValidationType,
                    },
                },
            }}, state::SharedState,
    },
};

#[derive(serde::Deserialize, Debug, Multipart)]
pub struct CreateCommentRequest {
    pub content: String,
    #[multipart]
    pub media_src: Option<Vec<MultipartFile>>,
}

pub async fn get_comment_handler(
    State(state): State<SharedState>,
    Path((version, post_id)): Path<(String, i64)>, 
    req_auth: RequestAuth,
) -> Result<Json<Vec<CommentDTO>>, APIError> {
    let api_version = version::parse_version(&version)?;
    tracing::trace!("api version: {}", api_version);
    
    let _user_id = match req_auth.user {
        Some(user) => user.user_id,
        None => 81727418892554240,
    };

    let mut tx = state.db_pool.begin().await?;
    let all_comment = comment::get_comment(&mut tx, post_id).await?;
    let mut comment_vec: Vec<CommentDTO> = Vec::new();

    for comment in all_comment.into_iter() {
        let media_rows = post_repo::post::get_post_attachment(&mut tx, comment.id).await?;
        let media = media_rows
            .into_iter()
            .map(|media| MediaFullDTO {
                id: media.id.to_string(),
                path: media.path,
                name: media.name,
                thumbhash: media.thumbhash,
                status: media.status,
                created_at: media.created_at,
                file_size: media.file_size,
                mime_type: media.mime_type,
                width: media.width,
                height: media.height,
                duration: media.duration,
            })
            .collect::<Vec<_>>();

        comment_vec.push(CommentDTO {
            id: comment.id.to_string(),
            user_id: comment.user_id.to_string(),
            post_id: comment.post_id.to_string(),
            content: comment.content,
            has_attachment: comment.has_attachment,
            total_likes: comment.total_likes,
            created_at: Some(comment.created_at.to_rfc3339()),
            updated_at: Some(comment.updated_at.to_rfc3339()),
            media,
        });
    }

    Ok(Json(comment_vec))
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
        None => 81727418892554240,
    };

    let options = MultipartExtractorOptions {
        max_file_size: Some(512_000_000),
        max_files: Some(5),
        validation: Some(ValidationOptions {
            validation_type: ValidationType::Whitelisted,
            value: vec![MediaType::Image, MediaType::Video],
        }),
        filter: Some(vec![FieldTypeFilter {
            max_file_size: Some(25_000_000),
            affected_types: Some(vec![MediaType::Image]),
        }]),
    };

    let extracted = state
        .multipart_extractor
        .extract::<CreateCommentRequest>(media_src, options)
        .await?;
    tracing::debug!("Extracted payload: {:#?}", extracted);

    let new_comment_id = state.snowflake_generator.generate_id()?;

    let new_media_opts = MediaServiceOptions {
        upload_route: format!("comments/{}", new_comment_id),
        uploader_id: user_id,
        container: None,
        processor: Some(MediaProcessorOptions {
            fflags: Some(MediaProcessorFFlags {
                video_thumbnail: true,
                video_gpu_accel: true,
                video_transcode: true,
                image_thumbhash: true,
                ..Default::default()
            }),
            image_processors: Some(vec![ImageProcessorType::Resize {
                style: ResizeStyle::Absolute {
                    width: 1024,
                    height: 1024,
                },
                upscale: false,
            }]),
            video_processors: None,
            post_processors: Some(PostProcessingType::Video(vec![
                VideoPostProcessorType::HLS { segment_time: 10 },
            ])),
        }),
    };

    let mut tx = state.db_pool.begin().await?;
    let has_attachment = extracted.media_src.is_some();

    let comment = post_repo::comment::create_comment(
        &mut tx,
        &new_comment_id,
        post_id,
        user_id,
        &extracted.content,
        has_attachment,
    )
    .await?;

    if let Some(ref files) = extracted.media_src {
        tracing::debug!("Extracted media_src files: {:#?}", files);

        let media_service = MediaService::new();
        let all_media = media_service
            .save_media_group(&state, files.to_vec(), new_media_opts)
            .await?;

        tracing::debug!("Saved media group: {:#?}", all_media);

        for media in all_media {
            media_repo::update::media_status(&mut tx, &media.get_id(), &MediaStatus::Completed).await?;
            post_repo::post::add_has_attachment(&mut tx, comment.id, media.get_id(), MediaTypeAttachment::Comment.as_str().to_string()).await?;
        }
    }
    let media_with_post = post_repo::post::get_post_attachment(&mut tx, new_comment_id).await?;
    let media = media_with_post.into_iter()
    .map(|media| MediaFullDTO {
        id: media.id.to_string(),
        path: media.path,
        name: media.name,
        thumbhash: media.thumbhash,
        status: media.status,
        created_at: media.created_at,
        file_size: media.file_size,
        mime_type: media.mime_type,
        width: media.width,
        height: media.height,
        duration: media.duration,
    })
    .collect::<Vec<_>>();
    tracing::warn!("Updated Post after upload: {:#?}", media);

    tx.commit().await?;

    Ok(Json(CommentDTO {
        id: comment.id.to_string(),
        post_id: comment.post_id.to_string(),
        user_id: comment.user_id.to_string(),
        content: comment.content,
        total_likes: comment.total_likes,
        has_attachment: comment.has_attachment,
        created_at: Some(comment.created_at.to_rfc3339()),
        updated_at: Some(comment.updated_at.to_rfc3339()),
        media,
    }))
}

pub async fn update_comment_handler(
    State(state): State<SharedState>,
    Path((version, comment_id)): Path<(String, i64)>,
    req_auth: RequestAuth,
    media_src: Multipart,
) -> Result<Json<CommentDTO>, APIError> {
    let api_version = version::parse_version(&version)?;
    tracing::trace!("api version: {}", api_version);

    let _ = match req_auth.user {
        Some(user) => user.user_id,
        None => 81727418892554240,
    };

    let options = MultipartExtractorOptions {
        max_file_size: Some(512_000_000),
        max_files: Some(5),
        validation: Some(ValidationOptions {
            validation_type: ValidationType::Whitelisted,
            value: vec![MediaType::Image, MediaType::Video],
        }),
        filter: Some(vec![FieldTypeFilter {
            max_file_size: Some(25_000_000),
            affected_types: Some(vec![MediaType::Image]),
        }]),
    };

    let extracted = state
        .multipart_extractor
        .extract::<CreateCommentRequest>(media_src, options)
        .await?;
    tracing::debug!("Extracted payload: {:#?}", extracted);

    let mut tx = state.db_pool.begin().await?;

    let old_comment = post_repo::comment::get_specific_comment(&mut tx, comment_id).await?;
    tracing::trace!("Old comment: {:?}", old_comment);

    let content = if extracted.content != old_comment.content {
        extracted.content
    } else {
        old_comment.content
    };

    let updated_comment = post_repo::comment::update_comment(
        &mut tx,
        comment_id,
        &content,
        old_comment.has_attachment,
    )
    .await?;

    let media_with_post = post_repo::post::get_post_attachment(&mut tx, comment_id).await?;
    let media = media_with_post
        .into_iter()
        .map(|media| MediaFullDTO {
            id: media.id.to_string(),
            path: media.path,
            name: media.name,
            thumbhash: media.thumbhash,
            status: media.status,
            created_at: media.created_at,
            file_size: media.file_size,
            mime_type: media.mime_type,
            width: media.width,
            height: media.height,
            duration: media.duration,
        })
        .collect::<Vec<_>>();

    tx.commit().await?;

    Ok(Json(CommentDTO {
        id: updated_comment.id.to_string(),
        post_id: updated_comment.post_id.to_string(),
        user_id: updated_comment.user_id.to_string(),
        content: updated_comment.content,
        total_likes: updated_comment.total_likes,
        has_attachment: updated_comment.has_attachment,
        created_at: Some(updated_comment.created_at.to_rfc3339()),
        updated_at: Some(updated_comment.updated_at.to_rfc3339()),
        media,
    }))
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
