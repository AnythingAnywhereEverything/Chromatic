use axum::extract::multipart::Multipart;
use multipart_derive::Multipart;
use redis::AsyncTypedCommands;
use sqlx::types::Json;

use crate::application::{
    repository::{
        media::{
            self as media_repo,
            row::{MediaType, ProcessingState},
        }, post::{
            self as post_repo, row::{CommentRow, MediaTypeAttachment, PostRow, PostVisibility, TagTarget},
        }, tags:: {
            self as tags_repo
        }, user::follow::is_following,
    }, service::{
        errors::PostServiceError,
        media::{
            extractor::{ExtractorFileOptions, ValidationOptions},
            inspector::FileType,
            model::{FileContainer, container::ContainerConfig},
            processor::types::{
                ImageProcessorType, MediaProcessorOptions, ResizeStyle, VideoPostProcessorType,
            },
        },
        profile_service::ProfileService,
    }, state::AppState,
};

#[derive(serde::Deserialize, Debug, Multipart)]
pub struct CreatePostRequest {
    pub content: Option<String>,
    #[multipart]
    pub files: Option<FileContainer>,
    pub repost_from: Option<i64>,
    pub visibility: PostVisibility,
    pub media_tags: Option<Vec<i64>>,
}

#[derive(serde::Deserialize, Debug, Multipart)]
pub struct UpdatePostRequest {
    pub content: Option<String>,
    pub media_to_delete: Option<Vec<i64>>,
    pub visibility: Option<PostVisibility>,
    pub media_tags: Option<Vec<i64>>,
}

pub struct PostService;

impl PostService {
    pub async fn get_comments(
        &self,
        state: &AppState,
        post_id: i64,
        requester_id: i64,
        before: chrono::DateTime<chrono::Utc>,
        limit: i64,
    ) -> Result<Vec<CommentRow>, PostServiceError> {
        let mut tx = state.db_pool.begin().await?;
        let comment_ids: Vec<i64> = post_repo::get::post_comment_ids(&mut tx, post_id, requester_id, before, limit).await?;

        let comments = {
            let mut comments = Vec::new();
            for comment_id in comment_ids {
                if let Some(comment) = self.get_comment(state, &mut tx, comment_id, requester_id).await.ok() {
                    comments.push(comment);
                }
            }
            comments
        };

        Ok(comments)
    }

    pub async fn get_comment(
        &self,
        state: &AppState,
        tx: &mut sqlx::Transaction<'_, sqlx::Postgres>,
        comment_id: i64,
        requester_id: i64,
    ) -> Result<CommentRow, PostServiceError> {
        let comment = post_repo::get::base_comment(tx, comment_id, requester_id).await?;        

        // add author information to the comment
        let author = ProfileService::get_profile_by_id(state, comment.author_id, Some(requester_id)).await?;
        let comment = CommentRow {
            id: comment.id,
            post_id: comment.post_id,
            author: Json(author),
            total_likes: comment.total_likes,
            is_liked: comment.is_liked,
            content: comment.content,
            has_attachment: comment.has_attachment,
            created_at: comment.created_at,
            updated_at: comment.updated_at,
            attachments: comment.attachments,
        };

        Ok(comment)
    }

    pub async fn update_post(
        &self,
        state: &AppState,
        post_id: i64,
        author_id: i64,
        multipart: Multipart,
    ) -> Result<PostRow, PostServiceError> {
        // Check if everything is none
        let extracted = state
            .multi_extractor
            .extract::<UpdatePostRequest>(multipart, None)
            .await?;

        if extracted.content.is_none()
            && extracted.media_to_delete.is_none()
            && extracted.visibility.is_none()
            && extracted.media_tags.is_none()
        {
            return Err(PostServiceError::NothingToUpdate);
        }

        let mut tx = state.db_pool.begin().await?;

        if let Some(content) = extracted.content {
            post_repo::update::content(&mut tx, post_id, content).await?;
        }

        // fetch the updated post
        let Some(post) = post_repo::get::base_post(&mut tx, post_id, Some(author_id)).await? else {
            return Err(PostServiceError::PostNotFound);
        };

        tx.commit().await?;
        // update the cache with the latest post data
        let mut conn = state.redis.get().await?;
        let cache_key = format!("post:{}", post_id);
        let _: () = conn
            .set_ex(&cache_key, serde_json::to_string(&post)?, 60 * 20)
            .await?;

        let mut tx = state.db_pool.begin().await?;
        let updated_post = self
            .get_post(state, &mut tx, post_id, Some(author_id))
            .await?;
        if let Some(updated_post) = updated_post {
            Ok(updated_post)
        } else {
            Err(PostServiceError::PostNotFound)
        }
    }

    pub async fn delete_post(
        &self,
        state: &AppState,
        post_id: i64,
        author_id: i64,
    ) -> Result<(), PostServiceError> {
        let mut tx = state.db_pool.begin().await?;
        let rows_affected = post_repo::delete::post(&mut tx, post_id, author_id).await?;
        if rows_affected == 0 {
            return Err(PostServiceError::PostNotFound);
        }
        tx.commit().await?;

        // remove from cache
        let mut conn = state.redis.get().await?;
        let cache_key = format!("post:{}", post_id);
        let _ = conn.del(&cache_key).await?;

        ProfileService::update_post_counts(state, author_id, -1).await?;
        Ok(())
    }

    pub async fn get_user_posts(
        &self,
        state: &AppState,
        target_id: i64,
        requester_id: Option<i64>,
        before: chrono::DateTime<chrono::Utc>,
        limit: i32,
    ) -> Result<Vec<PostRow>, PostServiceError> {
        let mut tx = state.db_pool.begin().await?;
        let user_posts =
            post_repo::get::user_posts(&mut tx, target_id, requester_id, before, limit).await?;

        let mut posts = Vec::new();
        for post_id in &user_posts {
            if let Some(post) = self
                .get_post(state, &mut tx, *post_id, requester_id)
                .await?
            {
                posts.push(post);
            }
        }

        Ok(posts)
    }

    pub async fn get_feed(
        &self,
        state: &AppState,
        user_id: i64,
        limit: i32,
    ) -> Result<Vec<PostRow>, PostServiceError> {
        let mut tx = state.db_pool.begin().await?;
        let feed = post_repo::get::feed(&mut tx, user_id, limit).await?;

        let mut posts = Vec::new();
        for post_id in &feed {
            // Fetch each post by its ID
            if let Some(post) = self
                .get_post(state, &mut tx, *post_id, Some(user_id))
                .await?
            {
                posts.push(post);
            }
        }

        Ok(posts)
    }

    pub async fn get_post(
        &self,
        state: &AppState,
        tx: &mut sqlx::Transaction<'_, sqlx::Postgres>,
        post_id: i64,
        user_id: Option<i64>,
    ) -> Result<Option<PostRow>, PostServiceError> {
        let redis = &mut state.redis.get().await?;
        let cache_key: String = format!("post:{}", post_id);
        
        // check cache
        let post = {
            let cached_post: Option<String> = redis.get(&cache_key).await?;
            if let Some(value) = cached_post {
                if let Ok(post) = serde_json::from_str(&value) {
                    return Ok(post);
                }
            }
            let post = post_repo::get::base_post(tx, post_id, user_id).await?;
            // Set cache with expiration of 20 minutes (60 * 20 seconds)
            let _: () = redis
                .set_ex(&cache_key, serde_json::to_string(&post)?, 60 * 20)
                .await?;
            post
        };

        if let Some(post) = post {
            let author = ProfileService::get_profile_by_id(state, post.author_id, user_id).await?;

            let is_followed = is_following(tx, user_id, post.author_id).await?;
            Ok(Some(PostRow {
                author: Json(author),
                post_id: post.post_id,
                content: post.content,
                total_likes: post.total_likes,
                total_comments: post.total_comments,
                is_reposted: post.is_reposted,
                reposted_post: post.reposted_post,
                visibility: post.visibility,
                tags: post.tags,
                is_liked: post.is_liked,
                has_attachment: post.has_attachment,
                attachments: post.attachments,
                created_at: post.created_at,
                updated_at: post.updated_at,
                is_followed: is_followed,
            }))
        } else {
            Ok(None)
        }
    }

    pub async fn create_post(
        &self,
        state: &AppState,
        author_id: i64,
        multipart: Multipart,
    ) -> Result<PostRow, PostServiceError> {
        let ext_opts = ExtractorFileOptions {
            max_size: Some(512_000_000),
            max_files: Some(5),
            validation: Some(
                ValidationOptions::new_whitelist()
                    .add_type(FileType::Category(MediaType::Image))
                    .add_type(FileType::Category(MediaType::Video)),
            ),
            field_options: None,
        };

        let mut extracted = state
            .multi_extractor
            .extract::<CreatePostRequest>(multipart, Some(ext_opts))
            .await?;

        let new_post_id = &state.snowflake_generator.generate_id()?;

        let mut tx = state.db_pool.begin().await?;

        if let Some(content) = &extracted.content {
            if content.len() > 2500 {
                return Err(PostServiceError::PostTextContentTooLarge.into());
            }
        }

        if let Some(container) = &mut extracted.files {
            container
                .set_uploader_id(author_id)
                .set_target_path(format!("posts/{}", new_post_id))
                .set_config(
                    ContainerConfig::new()
                        .set_generate_thumbhash(true)
                        .set_processing_options(
                            MediaProcessorOptions::new()
                                .set_fflags_video_gpu_accel(true)
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
            author_id,
            extracted.content,
            extracted.repost_from,
            !extracted.files.is_none(),
            extracted.repost_from.is_some(),
            extracted.visibility,
        )
        .await?;

        if !post_tags.is_empty() {
            tracing::trace!("Entering add tags stage");
            for tag in post_tags {
                tags_repo::add::add_tags_target(&mut tx, *new_post_id, TagTarget::Post, tag)
                    .await?;
            }
        }
        let post = self
            .get_post(state, &mut tx, *new_post_id, Some(author_id))
            .await?;
        tx.commit().await?;

        ProfileService::update_post_counts(state, author_id, 1).await?;

        let Some(post) = post else {
            return Err(PostServiceError::PostNotFound);
        };
        Ok(post)
    }
}
