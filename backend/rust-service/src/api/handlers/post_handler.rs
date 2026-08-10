use axum::{
    Json,
    extract::{Multipart, Path, State},
};
use multipart_derive::Multipart;
use serde::Serialize;

use crate::{
    api::{APIError, dtos::post_dtos::PostDTO, version}, application::{
        service::media::{
            multipart_ex::{MultipartExtractorOptions, MultipartLimits}, service::MediaService, types::{MediaOptions, MediaProcessing, MediaType, OnProcessingType, PostProcessingType, ProcessingOptions, ResizeStyle, TempUpload, ValidationOptions, ValidationType},
        }, state::SharedState,
    },
};

#[derive(serde::Deserialize, Debug, Multipart)]

pub struct CreatePostRequest {
    pub content: String,
    #[multipart]
    pub multipart: Option<Vec<TempUpload>>,
    pub repost_from: Option<String>,
}

pub async fn create_new_post_handler(
    State(state): State<SharedState>,
    Path(version): Path<String>,
    multipart: Multipart,
) -> Result<(), APIError> {
    let api_version = version::parse_version(&version)?;
    tracing::trace!("api version: {}", api_version);

    let options = MultipartExtractorOptions {
        limits: MultipartLimits {
            max_file_size: 512_000_000,
            max_files: 5,
        },
        ..Default::default()
    };
    let extracted = state
        .multipart_extractor
        .extract::<CreatePostRequest>(multipart, options)
        .await?;
    tracing::debug!("Extracted payload: {:#?}", extracted);

    let new_post_id = &state.snowflake_generator.generate_id()?;

    let media_options = MediaOptions {
        folder: format!("posts/{}", new_post_id),
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

    Ok(())
}
