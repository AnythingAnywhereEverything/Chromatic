use axum::{
    Json, extract::{Multipart, Path, State},
};
use hyper::StatusCode;
use multipart_derive::Multipart;

use crate::{
    api::{APIError, RequestAuth, dtos::post_dtos::{LikeDTO, PostDTO}, version}, application::{
        repository::{
            media::{self as media_repo, row::MediaStatus},
            post::{self as post_repo},
        }, service::{
            errors::{AuthServiceError, PostServiceError}, media::{
                processor::types::{
                    ImageProcessorType, MediaProcessorFFlags, MediaProcessorOptions,
                    PostProcessingType, ResizeStyle, VideoPostProcessorType,
                }, service::MediaService, service_type::{ContainerConfig, MediaServiceOptions}, types::{
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

pub async fn get_feed_post_handler(
    State(state): State<SharedState>,
    Path(version): Path<String>,
    req_auth: RequestAuth
) -> Result<Json<Vec<PostDTO>>, APIError> {
    let api_version = version::parse_version(&version)?;
    tracing::trace!("api version: {}", api_version);

    let user_id = match req_auth.user {
        Some(user) => Some(user.user_id),
        None => None,
    };

    let mut tx = state.db_pool.begin().await?;
    let all_post = post_repo::post::get_feed_public(&mut tx, None, user_id).await?;
    tracing::warn!("POST AS JSON BEFORE {:#?}", all_post);

    let mut post_vec: Vec<PostDTO> = all_post.into_iter().map(|post| post.into()).collect();
    
    for post in &mut post_vec {
        post.current_user_id = user_id.map(|id| id.to_string());
    }
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
        None => return Err(AuthServiceError::InvalidCredentials.into()),
        // None => 81727418892554240,
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

    let new_post_id = &state.snowflake_generator.generate_id()?;

    let new_media_opts = MediaServiceOptions {
        upload_route: format!("posts/{}", new_post_id),
        uploader_id: user_id,
        container: Some(ContainerConfig {
            generate_thumbhash: true,
            use_animated_image_indicator: true,
            ..Default::default()
        }),
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

        let all_media = media_service
            .save_media_group(&state, files.to_vec(), new_media_opts)
            .await?;
        for media in all_media {
            // set to complete the media processing
            media_repo::update::media_status(&mut tx, &media.get_id(), &MediaStatus::Completed).await?;
            post_repo::post::add_has_attachment(&mut tx, *new_post_id, media.get_id(), MediaTypeAttachment::Post.as_str().to_string()).await?;
        }
    }

    let post_tags = extracted.media_tags.unwrap_or_default();

    let _ =
            post_repo::post::create_post(&mut tx, new_post_id, 
            user_id, &content, 
            extracted.repost_from, !extracted.media_src.is_none(),
            extracted.repost_from.is_some(), extracted.visibility).await?;

    if !post_tags.is_empty() {
        tracing::trace!("Entering add tags stage");
        for tag in post_tags {
            post_repo::post::add_tags_target(&mut tx, *new_post_id, "post".to_string(), tag ).await?;
        }
    }
    let mut post: PostDTO = post_repo::post::get_post_by_id(&mut tx, *new_post_id, user_id).await?.into();
    
    post.current_user_id = Some(user_id.to_string());
    tx.commit().await?;
    Ok(Json(post))
}

// todo: impl to cache later if everything stable
// * test create null update to has tags
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
        None => return Err(AuthServiceError::InvalidCredentials.into()),
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

    if extracted.content.len() > 2500 {
        return Err(PostServiceError::PostTextContentTooLarge.into());
    }

    tracing::debug!("Extracted payload: {:#?}", extracted);

    let mut tx = state.db_pool.begin().await?;

    let old_post = post_repo::post::get_post_by_id(&mut tx, post_id, user_id).await?;
    let old_tags = post_repo::post::get_tag_attachments(&mut tx, post_id).await?;

if let Some(new_tags) = extracted.media_tags {
    let old_tag_ids: std::collections::HashSet<i64> =
        old_tags.iter().map(|tag| tag.tag_id).collect();

    let new_tag_ids: std::collections::HashSet<i64> =
        new_tags.iter().copied().collect();

    // * Delete old tags that are no longer present.
    for old_tag in &old_tags {
        if !new_tag_ids.contains(&old_tag.tag_id) {
            post_repo::post::delete_tag_attachment(
                &mut tx,
                post_id,
                old_tag.tag_id,
            )
            .await?;
        }
    }

    // * Add new tags that weren't already attached.
    for tag_id in new_tags {
        if !old_tag_ids.contains(&tag_id) {
            post_repo::post::add_tags_target(
                &mut tx,
                post_id,
                "post".to_string(),
                tag_id,
            )
            .await?;
        }
    }
}
    tracing::trace!("Old post: {:?}", old_post);

    let content = if extracted.content != old_post.content {
        extracted.content
    } else {
        old_post.content
    };

    let _ = post_repo::post::update_post(
        &mut tx,
        post_id,
        user_id,
        content,
        extracted.visibility,
    )
    .await?;

    let post: PostDTO =
        post_repo::post::get_post_by_id(&mut tx, post_id, user_id)
            .await?
            .into();

    tracing::trace!("Updated post {:#?}", post);

    tx.commit().await?;

    Ok(Json(post))
}

pub async fn delete_post_handler(
    State(state): State<SharedState>,
    Path((version, post_id)): Path<(String, i64)>,
    req_auth: RequestAuth,
) -> Result<StatusCode, APIError> {
    let api_version = version::parse_version(&version)?;
    tracing::trace!("api version: {}", api_version);


    let mut tx = state.db_pool.begin().await?;

    let user_id = match req_auth.user {
        Some(user) => user.user_id,
        None => return Err(AuthServiceError::InvalidCredentials.into()),
        // None => 1234,
    };
    
    post_repo::post::get_post_by_id(&mut tx, post_id, user_id).await?;
    let delete = post_repo::post::delete_post(&mut tx, post_id, user_id).await?;
    tx.commit().await?;

    if delete == 0 {
        return Err(PostServiceError::CommentNotFoundOrUnauthorized.into());
    }
    
    Ok(StatusCode::NO_CONTENT)
}
// pub async fn get_like_handler(
//     State(state): State<SharedState>,
//     Path((version, post_id)): Path<(String, i64)>,
// ) -> Result<> {
    
// }

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
        None => return Err(AuthServiceError::InvalidCredentials.into()),
    };

    let mut tx = state.db_pool.begin().await?;
    tracing::info!(user_id, target_id, req.is_like, "liking post");
    let post_like = post_repo::post::like_post_repo(
        &mut tx,
        user_id,
        target_id,
        req.is_like,
    ).await?;

    tx.commit().await?;

    Ok(Json(LikeDTO {
        id: post_like.id.to_string(),
        total_liked: post_like.total_likes,
    }))
}

pub async fn bookmark_handler(
    State(state) : State<SharedState>,
    Path((version, target_id)): Path<(String, i64)>,
    req_auth: RequestAuth,
    payload: Json<BookmarkRequest>
) -> Result<(), APIError> {
    let api_version = version::parse_version(&version)?;
    tracing::trace!("api version: {}", api_version);

    let user_id = match req_auth.user {
        Some(user) => user.user_id,
        None => return Err(AuthServiceError::InvalidCredentials.into()),
    };

    let mut tx = state.db_pool.begin().await?;
    let _bookmark = post_repo::post::toggle_bookmark(&mut tx, target_id, user_id, payload.is_bookmark).await;

    Ok(())
}