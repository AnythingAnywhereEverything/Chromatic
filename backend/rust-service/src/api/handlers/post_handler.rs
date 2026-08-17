use axum::{
    Json, extract::{Multipart, Path, Query, State},
};
use hyper::StatusCode;
use multipart_derive::Multipart;
use tracing::warn;

use crate::{
    api::{APIError, RequestAuth, dtos::{post_dtos::{LikeDTO, PostDTO, TagDTO}, user_dtos::MediaFullDTO}, version}, application::{
        repository::{
            media::{self as media_repo, row::MediaStatus},
            post::{self as post_repo},
        }, service::{
            errors::PostServiceError, media::{
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
            },
        }, state::SharedState,
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

impl ToString for PostVisibility {
    fn to_string(&self) -> String {
        match self {
            PostVisibility::Everyone => "everyone".to_string(),
            PostVisibility::Friend => "friend".to_string(),
            PostVisibility::Private => "private".to_string(),
        }
    }
}

#[derive(serde::Deserialize, sqlx::Type, Debug)]
#[sqlx(rename_all = "lowercase")]
pub enum MediaTypeAttachment{
    Post,
    Comment,
    Message,
    Community
}

impl MediaTypeAttachment {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Post => "post",
            Self::Comment => "comment",
            Self::Message => "message",
            Self::Community => "community",
        }
    }
}

#[derive(serde::Deserialize, Debug, Multipart)]
pub struct CreatePostRequest {
    pub content: String,
    #[multipart]
    pub media_src: Option<Vec<MultipartFile>>,
    pub repost_from: Option<i64>,
    pub visibility: PostVisibility,
    pub media_tags: Option<Vec<i64>>
}

#[derive(Debug, serde::Deserialize)]
pub struct LikeRequest {
    pub is_like: bool,
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

// ! This might cause slow server
pub async fn get_feed_post_handler(
    State(state): State<SharedState>,
    Path(version): Path<String>,
) -> Result<Json<Vec<PostDTO>>, APIError> {
    let api_version = version::parse_version(&version)?;
    tracing::trace!("api version: {}", api_version);

    let mut tx = state.db_pool.begin().await?;
    let all_post = post_repo::post::get_feed_public(&mut tx, None).await?;
    tracing::warn!("POST AS JSON BEFORE {:#?}", all_post);

    let post_vec: Vec<PostDTO> = all_post.into_iter().map(|post| post.into()).collect();

    Ok(Json(post_vec))
}

pub async fn create_new_post_handler(
    State(state): State<SharedState>,
    Path(version): Path<String>,
    req_auth: RequestAuth,
    request: Multipart,
) -> Result<Json<PostDTO>, APIError> {
    let api_version = version::parse_version(&version)?;
    tracing::trace!("api version: {}", api_version);

    // ! Temporary testing ID
    let user_id = match req_auth.user {
        Some(user) => user.user_id,
        // None => return Err(AuthServiceError::InvalidCredentials.into()),
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
        .extract::<CreatePostRequest>(request, options)
        .await?;
    tracing::debug!("Extracted payload: {:#?}", extracted);

    let new_post_id = &state.snowflake_generator.generate_id()?;

    let new_media_opts = MediaServiceOptions {
        upload_route: format!("posts/{}", new_post_id),
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
    
    if extracted.content.len() > 2500 {
        return Err(PostServiceError::PostTextContentTooLarge.into());
    }
    
    let content = extracted.content;
    let media_service = MediaService::new();

    if let Some(ref files) = extracted.media_src {
        tracing::debug!("Extracted media_src files: {:#?}", files);

        let all_media = media_service
            .save_media_group(&state, files.to_vec(), new_media_opts)
            .await?;
        tracing::debug!("Saved media group: {:#?}", all_media);
        for media in all_media {
            // set to complete the media processing
            media_repo::update::media_status(&mut tx, &media.get_id(), &MediaStatus::Completed).await?;
            tracing::debug!("Media processing completed for media ID: {}", media.get_id());
            post_repo::post::add_has_attachment(&mut tx, *new_post_id, media.get_id(), MediaTypeAttachment::Post.as_str().to_string()).await?;
        }
    }

    let repost_from = extracted.repost_from;
    let visibility = extracted.visibility;
    let post_tags = extracted.media_tags.unwrap_or_default();
    let is_repost = repost_from.is_some();
    let new_post =
        post_repo::post::create_post(&mut tx, new_post_id, user_id, &content, repost_from, !extracted.media_src.is_none(),is_repost, visibility).await?;
        tracing::trace!("post content : {:#?}", new_post);

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
        status: media.status.to_string(),
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

// todo: impl to cache later if everything stable
pub async fn update_post_handler(
    State(state): State<SharedState>,
    Path((version, post_id)): Path<(String, i64)>,
    req_auth: RequestAuth,
    media_src: Multipart,
) -> Result<Json<PostDTO>, APIError> {
    let api_version = version::parse_version(&version)?;
    tracing::trace!("api version: {}", api_version);

    // ! Temporary testing ID
    let user_id = match req_auth.user {
        Some(user) => user.user_id,
        None => 1234,
    };

    let options = MultipartExtractorOptions {
        max_file_size: None,
        max_files: None,
        validation: None,
        filter: None,
    };

    let extracted = state
        .multipart_extractor
        .extract::<CreatePostRequest>(media_src, options)
        .await?;

    tracing::debug!("Extracted payload: {:#?}", extracted);

    let mut tx = state.db_pool.begin().await?;

    let old_post = post_repo::post::get_post_by_id(&mut tx, post_id).await?;

    tracing::trace!("Old post: {:?}", old_post);

    let content = if extracted.content != old_post.content {
        extracted.content
    } else {
        old_post.content
    };

    let updated_post = post_repo::post::update_post(&mut tx, post_id, user_id, content, extracted.visibility).await?;

    let media_with_post = post_repo::post::get_post_attachment(&mut tx, post_id).await?;

    let media_tags = post_repo::post::get_tag_attachments(&mut tx, post_id)
        .await?
        .into_iter()
        .map(|tag| TagDTO {
            target_id: tag.target_id.to_string(),
            tag_name: tag.tag_name,
            tag_id: tag.tag_id.to_string(),
        })
        .collect::<Vec<_>>();

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
            mime_type: media.mime_type,
            width: media.width,
            height: media.height,
            duration: media.duration,
        })
        .collect::<Vec<_>>();

    tx.commit().await?;

    Ok(Json(PostDTO {
        id: updated_post.id.to_string(),
        user_id: updated_post.user_id.to_string(),
        content: updated_post.content,
        total_comments: updated_post.total_comments,
        total_likes: updated_post.total_likes,
        reposted_from: Some(
            updated_post
                .reposted_from
                .map(|id| id.to_string())
                .unwrap_or_default(),
        ),
        is_repost: updated_post.is_repost,
        has_attachment: updated_post.has_attachment,
        created_at: Some(updated_post.created_at.to_rfc3339()),
        updated_at: Some(updated_post.updated_at.to_rfc3339()),
        visibility: updated_post.visibility.as_str().to_string(),
        media,
        tag: media_tags,
    }))
}

pub async fn delete_post_handler(
    State(state): State<SharedState>,
    Path((version, post_id)): Path<(String, i64)>,
    req_auth: RequestAuth,
) -> Result<StatusCode, APIError> {
    let api_version = version::parse_version(&version)?;
    tracing::trace!("api version: {}", api_version);
    let mut tx = state.db_pool.begin().await?;

    post_repo::post::get_post_by_id(&mut tx, post_id).await?;
    let user_id = match req_auth.user {
        Some(user) => user.user_id,
        // None => return Err(AuthServiceError::InvalidCredentials.into()),
        None => 1234,
    };

    let delete = post_repo::post::delete_post(&mut tx, post_id, user_id).await?;
    tx.commit().await?;

    if delete == 0 {
        return Err(PostServiceError::CommentNotFoundOrUnauthorized.into());
    }
    
    Ok(StatusCode::NO_CONTENT)
}

pub async fn get_post_handler(
    State(state): State<SharedState>,
    Path((version, post_id)): Path<(String, i64)>,
) -> Result<Json<Vec<PostDTO>>, APIError> {
    let api_version = version::parse_version(&version)?;
    tracing::trace!("api version: {}", api_version);

    let mut tx = state.db_pool.begin().await?;
    let all_post = post_repo::post::get_feed_public(&mut tx, None).await?;
    tracing::warn!("POST AS JSON BEFORE {:#?}", all_post);

    let mut post_vec: Vec<PostDTO> = Vec::new();
    
    for post in all_post.into_iter() {
        tracing::warn!("POST AS JSON BEFORE {:#?}", post);
        let media_with_post = post_repo::post::get_post_attachment(&mut tx, post.id).await?;
        let media_tags = post_repo::post::get_tag_attachments(&mut tx, post.id)
            .await?
            .into_iter()
            .map(|tag| TagDTO {
                target_id: tag.target_id.to_string(),
                tag_name: tag.tag_name,
                tag_id: tag.tag_id.to_string(),
            })
            .collect::<Vec<_>>();

        let media = media_with_post
            .into_iter()
            .map(|m| MediaFullDTO {
                id: m.id.to_string(),
                path: m.path,
                name: m.name,
                thumbhash: m.thumbhash,
                status: m.status.to_string(),
                created_at: m.created_at,
                file_size: m.file_size,
                mime_type: m.mime_type,
                width: m.width,
                height: m.height,
                duration: m.duration,
            })
            .collect::<Vec<_>>();

        post_vec.push(PostDTO {
            id: post.id.to_string(),
            user_id: post.user_id.to_string(),
            content: post.content,
            total_comments: post.total_comments,
            total_likes: post.total_likes,
            reposted_from: post.reposted_from.map(|id| id.to_string()),
            is_repost: post.is_repost,
            has_attachment: post.has_attachment,
            created_at: Some(post.created_at.to_rfc3339()),
            updated_at: Some(post.updated_at.to_rfc3339()),
            visibility: post.visibility.as_str().to_string(),
            media,
            tag: media_tags,
        });
    }
    Ok(Json(post_vec))
}



pub async fn liked_handler(
    State(state): State<SharedState>,
    Path((version, target_id)): Path<(String, i64)>,
    req_auth: RequestAuth,
    Json(req): Json<LikeRequest>,
) -> Result<Json<LikeDTO>, APIError> {
    let api_version = version::parse_version(&version)?;
    tracing::trace!("api version: {}", api_version);

    let user_id = match req_auth.user {
        Some(user) => user.user_id,
        // None => return Err(AuthServiceError::InvalidCredentials.into()),
        None => 81727418892554240,
    };

    let mut tx = state.db_pool.begin().await?;
    tracing::info!(user_id, target_id, req.is_like, "liking post");
    let total_liked = post_repo::post::like_post_repo(
        &mut tx,
        user_id,
        target_id,
        req.is_like,
    ).await?;

    tx.commit().await?;

    Ok(Json(LikeDTO {
        id: total_liked.id.to_string(),
        total_liked: total_liked.total_likes
    }))
}