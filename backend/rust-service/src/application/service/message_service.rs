use std::result::Result;

use crate::application::{
    repository::{
        media::{
            self as media_repo,
            row::{MediaType, ProcessingState},
        },
        messages::{self, row::MessageRow},
    },
    service::{
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
    },
    state::AppState,
};
use axum::extract::multipart::Multipart;
use multipart_derive::Multipart;
use sqlx::types::Json;

#[derive(serde::Deserialize, Debug, Multipart)]
pub struct CreateMessageRequest {
    pub content: Option<String>,
    #[multipart]
    pub files: Option<FileContainer>,
}

pub struct MessageService;

impl MessageService {
    pub async fn get_messages_chat(
        &self,
        state: &AppState,
        user_id: i64,
        target_id: i64,
        limit: i64,
    ) -> Result<Vec<MessageRow>, MessageServiceError> {
        let mut tx = state.db_pool.begin().await?;
        let messages_id: Vec<i64> =
            messages::get::get_message_id(&mut tx, user_id, target_id, limit).await?;

        let mut messages = Vec::new();
        for id in messages_id {
            if let Some(message) = self.get_message(state, &mut tx, id, user_id).await? {
                messages.push(message);
            }
        }
        Ok(messages)
    }

    pub async fn get_message(
        &self,
        state: &AppState,
        tx: &mut sqlx::Transaction<'_, sqlx::Postgres>,
        message_id: i64,
        user_id: i64,
    ) -> Result<Option<MessageRow>, MessageServiceError> {
        let message = messages::get::base_message(tx, message_id, user_id).await?;

        if let Some(message) = message {
            let user_profile =
                ProfileService::get_profile_by_id(state, message.user_id, Some(user_id)).await?;
            Ok(Some(MessageRow {
                id: message.id,
                profile: Json(user_profile),
                target_id: message.target_id,
                content: message.content,
                has_attachment: message.has_attachment,
                created_at: message.created_at,
                updated_at: message.updated_at,
            }))
        } else {
            Ok(None)
        }
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
}