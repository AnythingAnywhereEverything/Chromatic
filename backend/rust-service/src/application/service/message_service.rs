use std::result::Result;

use crate::application::{
    repository::{
        media::{
            self as media_repo,
            row::{MediaType, ProcessingState},
        }, messages::{self, row::MessageRow}, user::{ self, row::UserProfileRow},
    }, service::{
        errors::MessageServiceError,
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
use axum::extract::multipart::Multipart;
use multipart_derive::Multipart;

#[derive(serde::Deserialize, Debug, Multipart)]
pub struct CreateMessageRequest {
    pub content: Option<String>,
    #[multipart]
    pub files: Option<FileContainer>,
}
#[derive(serde::Deserialize, Debug, Multipart)]
pub struct UpdateMessageRequest {
    pub new_content: Option<String>,
    #[multipart]
    pub media_to_delete: Option<Vec<i64>>,
}

pub struct MessageService;

impl MessageService {
    pub async fn get_messages_chat(
        &self,
        state: &AppState,
        user_id: i64,
        target_id: i64,
        before: chrono::DateTime<chrono::Utc>,
        limit: i32,
    ) -> Result<Vec<MessageRow>, MessageServiceError> {
        let mut tx = state.db_pool.begin().await?;

        let messages_id =
            messages::get::get_messages_id(&mut tx, user_id, target_id, before, limit).await?;
        if messages_id.is_empty() {
            let target_exists = user::find::is_user_exist(&mut tx, target_id).await?;
            if !target_exists {
                return Err(MessageServiceError::UserNotFound);
            }

            return Ok(Vec::new());
        }

        // allocate mem for the messages
        let mut messages = Vec::with_capacity(messages_id.len());

        for id in messages_id {
            if let Some(message) = self.get_message(state, &mut tx, id, user_id).await? {
                messages.push(message);
            }
        }

        Ok(messages)
    }

    // base message retrieval
    pub async fn get_message(
        &self,
        _state: &AppState,
        tx: &mut sqlx::Transaction<'_, sqlx::Postgres>,
        message_id: i64,
        user_id: i64,
    ) -> Result<Option<MessageRow>, MessageServiceError> {
        tracing::info!("Getting base message");
        let message = messages::get::base_message(tx, message_id, user_id).await?;

        // * I should not put profile in response it's cause too much space on response
        // * temp removed attachments to make less change to error
        if let Some(message) = message {
            Ok(Some(MessageRow {
                id: message.id,
                target_id: message.target_id,
                content: message.content,
                has_attachment: message.has_attachment,
                has_reactions: message.has_reactions,
                created_at: message.created_at,
                updated_at: message.updated_at,
            }))
        } else {
            Ok(None)
        }
    }

    pub async fn is_message_exist(
        &self,
        state: &AppState,
        message_id: i64,
        sender_id: i64,
    ) -> Result<bool, MessageServiceError> {
        let mut tx = state.db_pool.begin().await?;
        let exists = messages::get::is_message_exist(&mut tx, sender_id, message_id).await?;
        tx.commit().await?;
        Ok(exists)
    }

    pub async fn create_message(
        &self,
        state: &AppState,
        sender_id: i64,
        target_id: i64,
        request: Multipart,
    ) -> Result<MessageRow, MessageServiceError> {
        let ext_opts = ExtractorFileOptions {
            max_size: Some(512_000_000),
            max_files: Some(5),
            validation: Some(
                ValidationOptions::new_whitelist()
                    .add_type(FileType::Category(MediaType::Image))
                    .add_type(FileType::Category(MediaType::Video))
                    .add_type(FileType::Category(MediaType::Audio))
                    .add_type(FileType::Category(MediaType::Document))
                    .add_type(FileType::Category(MediaType::Other)),
            ),
            field_options: None,
        };
        let mut extracted = state
            .multi_extractor
            .extract::<CreateMessageRequest>(request, Some(ext_opts))
            .await?;

        if extracted.content.is_none() && extracted.files.is_none() {
            return Err(MessageServiceError::EmptyContent);
        }

        let new_message_id = &state.snowflake_generator.generate_id()?;
        let mut tx = state.db_pool.begin().await?;

        if let Some(content) = &extracted.content {
            if content.len() > 500 {
                return Err(MessageServiceError::ContentTooLong);
            }
        }

        if let Some(container) = &mut extracted.files {
            container
                .set_uploader_id(sender_id)
                .set_target_path(format!("messages/{}", new_message_id))
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
                media_repo::update::processing_state(
                    &mut tx,
                    &media.id,
                    &ProcessingState::Completed,
                )
                .await?;
                messages::create::add_has_attachment(
                    &mut tx,
                    target_id,
                    media.id,
                    "message".to_string(),
                )
                .await?;
            }
        }

        let message = self
            .get_message(state, &mut tx, *new_message_id, sender_id)
            .await?;

        let Some(message) = message else {
            return Err(MessageServiceError::MessageNotFound);
        };
        Ok(message)
    }

    // Accept only Text no files for updating a message
    // check there's image if new content is empty
    pub async fn update_message(
        &self,
        state: &AppState,
        sender_id: i64,
        message_id: i64,
        multipart: Multipart,
    ) -> Result<MessageRow, MessageServiceError> {
        let extracted = state
            .multi_extractor
            .extract::<UpdateMessageRequest>(multipart, None)
            .await?;

        if extracted.new_content.is_none() && extracted.media_to_delete.is_none() {
            return Err(MessageServiceError::EmptyContent);
        }
        let mut tx = state.db_pool.begin().await?;

        if let Some(content) = extracted.new_content {
            messages::update::update_message(&mut tx, message_id, content, sender_id).await?;
        }

        let Some(_message) = messages::get::base_message(&mut tx, message_id, sender_id).await?
        else {
            return Err(MessageServiceError::MessageNotFound);
        };

        tx.commit().await?;

        let mut tx = state.db_pool.begin().await?;
        let updated_message = self
            .get_message(state, &mut tx, message_id, sender_id)
            .await?;
        if let Some(updated_message) = updated_message {
            return Ok(updated_message);
        }
        Err(MessageServiceError::MessageNotFound) // Return an error if the message was not found after update
    }

    pub async fn delete_message(
        &self,
        state: &AppState,
        message_id: i64,
        sender_id: i64,
    ) -> Result<(), MessageServiceError> {
        let mut tx = state.db_pool.begin().await?;
        let row_affected = messages::delete::delete_message(&mut tx, message_id, sender_id).await?;
        if row_affected == 0 {
            return Err(MessageServiceError::MessageNotFound);
        };

        tx.commit().await?;
        Ok(())
    }

    pub async fn get_followed_users(
        &self,
        state: &AppState,
        user_id: i64,
    ) -> Result<Vec<UserProfileRow>, MessageServiceError> {
        let mut tx = state.db_pool.begin().await?;

        let followed_user_ids = messages::get::get_id_for_new_messages(&mut tx, user_id).await?;
        let mut followed_users = Vec::new();

        // The response might large
        for followed_user_id in followed_user_ids {
            let profile = ProfileService::get_profile_by_id(state, followed_user_id, None).await?;
            followed_users.push(profile);
        }
        Ok(followed_users)
    }
}
