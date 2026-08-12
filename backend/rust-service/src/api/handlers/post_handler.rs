use axum::{
    extract::{Multipart, Path, State},
    Json,
};
use multipart_derive::Multipart;

use crate::{
    api::{APIError, RequestAuth, dtos::{post_dtos::{PostDTO, TagDTO}, user_dtos::MediaFullDTO}, version}, application::{
        repository::{
            media::{self as media_repo, row::MediaStatus}, post::{self as post_repo},
        }, service::{errors::AuthServiceError, media::{service::MediaService, types::{image_transform::ResizeStyle, media_options::{FileSizeGate, MediaOptions, MediaProcessing, MediaType, MultipartExtractorOptions, MultipartLimits, OnProcessingType, PostProcessingType, ProcessingOptions, TempUpload, ValidationOptions, ValidationType}}}}, state::SharedState,
    },
};
#[derive(serde::Deserialize, sqlx::Type, Debug)]
#[sqlx(rename_all = "lowercase")]
pub enum PostStatus {
    Pending,
    Active,
    InActive,
}
#[derive(serde::Deserialize, sqlx::Type, Debug)]
#[sqlx(type_name = "post_visibility",rename_all = "lowercase")]
pub enum PostVisibility {
    Everyone,
    Friend,
    Private
}

#[derive(serde::Deserialize, sqlx::Type, Debug)]
#[sqlx(rename_all = "lowercase")]
pub enum PostTagsAttachment{
    User,
    Media
}
#[derive(serde::Deserialize, Debug, Multipart)]
pub struct CreatePostRequest {
    pub content: String,
    #[multipart]
    pub media_src: Option<Vec<TempUpload>>,
    pub repost_from: Option<i64>,
    pub visibility: PostVisibility,
    pub media_tags: Vec<i64>
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

// todo; refactor after media service get update
pub async fn create_new_post_handler(
    State(state): State<SharedState>,
    Path(version): Path<String>,
    req_auth: RequestAuth,
    media_src: Multipart,
) -> Result<Json<PostDTO>, APIError> {
    let api_version = version::parse_version(&version)?;
    tracing::trace!("api version: {}", api_version);

    // ! Temporary testing ID
    let user_id = match req_auth.user {
        Some(user) => user.user_id,
        // None => return Err(AuthServiceError::InvalidCredentials.into()),
        None => 1234,
    };

    let options = MultipartExtractorOptions {
        limits: MultipartLimits {
            max_file_size: 512_000_000,
            max_files: 5,
        },
        validation: Some(ValidationOptions {
            validation_type: ValidationType::Whitelisted,
            value: vec![MediaType::Image, MediaType::Video],
        }),
        size_filter_gate: Some(vec![
            FileSizeGate {
                media_type: MediaType::Image,
                max_size: 25_000_000,
            }
        ]),
    };

    let extracted = state
        .multipart_extractor
        .extract::<CreatePostRequest>(media_src, options)
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
                use_thumbhash_generation: true,
                use_gpu_acceleration: true,
                use_video_transcoding: true,
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
    
    let mut tx = state.db_pool.begin().await?;

    if let Some(files) = extracted.media_src.as_ref() {
        tracing::debug!("Extracted media_src files: {:#?}", files);

        let all_media = MediaService::save_media_group(&state.media_service, user_id, files.to_vec(), media_options)
            .await?;

        tracing::debug!("Saved media group: {:#?}", all_media);
        for media in all_media {
            // set to complete the media processing
            media_repo::update::media_status(&mut tx, &media.file_id, &MediaStatus::Completed).await?;
            tracing::debug!("Media processing completed for media ID: {}", media.file_id);
            post_repo::post::add_has_attachment(&mut tx, *new_post_id, media.file_id, "user".to_string()).await?;
        }
    }

    let content = extracted.content;
    let repost_from = extracted.repost_from;
    let visibility = extracted.visibility;
    let post_tags = extracted.media_tags;
    let is_repost = repost_from.is_some();
    let new_post =
        post_repo::post::create_post(&mut tx, new_post_id, user_id, &content, repost_from, is_repost, visibility).await?;
        tracing::trace!("post content : {:?}", new_post);

    if !post_tags.is_empty() {
        tracing::trace!("Entering add tags stage");
        for tag in post_tags {
            post_repo::post::add_tags_target(&mut tx, *new_post_id, "media".to_string(), tag ).await?;
        }
    }

    let media_with_post = post_repo::post::get_post_attachment(&mut tx, new_post.id).await?;
    let media_tags = 
    post_repo::post::get_tag_attachments(&mut tx, new_post.id).await?
    .into_iter()
    .map(|tag| TagDTO {
        target_id: tag.target_id.to_string(),
        tag_name: tag.tag_name,
        // tag.tag_id may be a Vec<i64>; convert to a comma-separated string
        tag_id: tag.tag_id.to_string()
    })
    .collect::<Vec<_>>();

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
    tracing::warn!("Updated Post after avatar upload: {:?}", media);

    
    tx.commit().await?;
    Ok(Json(PostDTO{
        id: new_post.id.to_string(),
        user_id: new_post.user_id.to_string(),
        content: new_post.content,
        total_comments: new_post.total_comments,
        total_likes: new_post.total_likes,
        reposted_from: Some(new_post.reposted_from
            .map(|id| id.to_string())
            .unwrap_or_default()),
        is_repost: new_post.is_repost,
        has_attachment: new_post.has_attachment,
        created_at: Some(new_post.created_at.to_rfc3339()),
        updated_at: Some(new_post.updated_at.to_rfc3339()),
        visibility: new_post.visibility.as_str().to_string(),
        media,
        tag: media_tags
    }))
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
