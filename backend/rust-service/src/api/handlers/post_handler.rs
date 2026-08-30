use axum::{
    Json,
    extract::{Multipart, Path, Query, State},
};
use hyper::StatusCode;
use multipart_derive::Multipart;

use crate::{
    api::{
        APIError, RequestAuth, dtos::post_dtos::{ LikeDTO, PostDTO}, version,
    }, application::{
        repository::{
            media::{self as media_repo, row::{MediaStatus, MediaType, ProcessingState}}, post::{self as post_repo, find::{FetchMode, PostQOpts}, row::PostRow},
        }, service::{
            errors::{AuthServiceError, PostServiceError},
            media::{
                self,
                extractor::{ExtractorFileOptions, ValidationOptions},
                model::{FileContainer, container::ContainerConfig},
                processor::types::{
                    ImageProcessorType, MediaProcessorOptions, ResizeStyle, VideoPostProcessorType,
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
#[derive(serde::Deserialize, sqlx::Type, Debug, serde::Serialize)]
#[sqlx(type_name = "post_visibility", rename_all = "lowercase")]
pub enum PostVisibility {
    Everyone,
    Friend,
    Private,
}

#[derive(
    serde::Deserialize,
    serde::Serialize,
    sqlx::Type,
    Debug,
)]
#[sqlx(type_name = "tag_attachment_types", rename_all = "lowercase")]
#[serde(rename_all = "lowercase")]
pub enum TagTarget {
    User,
    Post,
    Guild,
}

impl ToString for TagTarget {
   fn to_string(&self) -> String {
       match self {
           TagTarget::User => "user".to_string(),
           TagTarget::Post => "post".to_string(),
           TagTarget::Guild => "guild".to_string(),
       }
   }
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
pub enum MediaTypeAttachment {
    Post,
    Comment,
    Message,
    Community,
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
    pub media_src: Option<FileContainer>,
    pub repost_from: Option<i64>,
    pub visibility: PostVisibility,
    pub media_tags: Option<Vec<i64>>,
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
        None => None,
    };
    
    let mut tx = state.db_pool.begin().await?;

    let all_post = post_repo::find::get_feed_for_user(&mut tx, user_id.unwrap_or_default(), query.limit.unwrap_or(8) as i32).await?;

    Ok(Json(all_post))
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

    let post = post_repo::find::get_post_by_id(&mut tx, post_id, user_id).await?;
    
    Ok(Json(post))
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

    let ext_opts = ExtractorFileOptions {
        max_size: Some(512_000_000),
        max_files: Some(5),
        validation: Some(
            ValidationOptions::new_whitelist()
                .add_type(media::inspector::FileType::Category(
                    MediaType::Image,
                ))
                .add_type(media::inspector::FileType::Category(
                    MediaType::Video,
                )),
        ),
        field_options: None,
    };

    let mut extracted = state
        .multi_extractor
        .extract::<CreatePostRequest>(request, Some(ext_opts))
        .await?;

    let new_post_id = &state.snowflake_generator.generate_id()?;

    let mut tx = state.db_pool.begin().await?;

    if extracted.content.len() > 2500 {
        return Err(PostServiceError::PostTextContentTooLarge.into());
    }

    let content = extracted.content;

    if let Some(container) = &mut extracted.media_src {
        container
            .set_uploader_id(user_id)
            .set_target_path(format!("posts/{}", new_post_id))
            .set_config(
                ContainerConfig::new()
                    .set_generate_thumbhash(true)
                    .set_processing_options(
                        MediaProcessorOptions::new().set_fflags_video_gpu_accel(true)
                        .set_fflags_video_thumbnail(true)
                            .set_image_processors(vec![ImageProcessorType::Resize {
                                style: ResizeStyle::Absolute {
                                    width: 1024,
                                    height: 1024,
                                },
                                upscale: false,
                            }])
                            .set_post_video_processors(vec![VideoPostProcessorType::HLS {
                                segment_time: 10,
                            }]),
                    ),
            );

        state.media_service.save_media(&state, container).await?;

        for media in container.resolve_files() {
            // set to complete the media processing
            media_repo::update::processing_state(
                &mut tx,
                &media.id,
                &ProcessingState::Completed,
            )
            .await?;
            post_repo::post::add_has_attachment(
                &mut tx,
                *new_post_id,
                media.id,
                MediaTypeAttachment::Post.as_str().to_string(),
            )
            .await?;
        }
    }

    let post_tags = extracted.media_tags.unwrap_or_default();

    let _ = post_repo::post::create_post(
        &mut tx,
        new_post_id,
        user_id,
        &content,
        extracted.repost_from,
        !extracted.media_src.is_none(),
        extracted.repost_from.is_some(),
        extracted.visibility,
    )
    .await?;

    if !post_tags.is_empty() {
        tracing::trace!("Entering add tags stage");
        for tag in post_tags {
            post_repo::post::add_tags_target(&mut tx, *new_post_id, TagTarget::Post, tag)
                .await?;
        }
    }
    tx.commit().await?;

    tracing::trace!("Committing transaction and fetching post by ID");

    let mut tx = state.db_pool.begin().await?;
    let post = post_repo::find::get_post_by_id(&mut tx, *new_post_id, Some(user_id))
        .await?;

    // post.current_user_id = Some(user_id.to_string());
    Ok(Json(post))
}

// todo: impl to cache later if everything stable
// * test create null update to has tags
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

    let extracted = state
        .multi_extractor
        .extract::<CreatePostRequest>(media_src, None)
        .await?;

    if extracted.content.len() > 2500 {
        return Err(PostServiceError::PostTextContentTooLarge.into());
    }

    tracing::debug!("Extracted payload: {:#?}", extracted);

    let mut tx = state.db_pool.begin().await?;

    let old_post = post_repo::find::get_post_by_id(&mut tx, post_id, Some(user_id)).await?;
    let old_tags = post_repo::post::get_tag_attachments(&mut tx, post_id).await?;

    if let Some(new_tags) = extracted.media_tags {
        let old_tag_ids: std::collections::HashSet<i64> =
            old_tags.iter().map(|tag| tag.tag_id).collect();

        let new_tag_ids: std::collections::HashSet<i64> = new_tags.iter().copied().collect();

        // * Delete old tags that are no longer present.
        for old_tag in &old_tags {
            if !new_tag_ids.contains(&old_tag.tag_id) {
                post_repo::post::delete_tag_attachment(&mut tx, post_id, old_tag.tag_id).await?;
            }
        }

        // * Add new tags that weren't already attached.
        for tag_id in new_tags {
            if !old_tag_ids.contains(&tag_id) {
                post_repo::post::add_tags_target(&mut tx, post_id, TagTarget::Post, tag_id)
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

    let _ = post_repo::post::update_post(&mut tx, post_id, user_id, content, extracted.visibility)
        .await?;

    tx.commit().await?;

    Ok(Json(()))
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

    post_repo::find::get_post_by_id(&mut tx, post_id, Some(user_id)).await?;
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

pub async fn post_liked_handler(
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
    let post_like =
        post_repo::post::like_post_repo(&mut tx, user_id, target_id, req.is_like, &"post".to_string()).await?;

    tx.commit().await?;

    Ok(Json(LikeDTO {
        id: post_like.id.to_string(),
        total_liked: post_like.total_likes,
    }))
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
        post_repo::post::toggle_bookmark(&mut tx, target_id, user_id, payload.is_bookmark).await;

    Ok(())
}
