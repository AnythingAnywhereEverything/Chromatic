use crate::{
    api::APIError, application::{
        repository::media::{self, row::MediaType}, service::{
            errors::MediaServiceError, media::{
                extractor::{
                    ExtractorFileOptions,
                    ValidationOptions,
                }, inspector::FileType, model::container::{ContainerConfig, NamingStrategy}, processor::types::{
                    CropStyle, ImageProcessorType, MediaProcessorOptions, ResizeStyle, VideoPostProcessorType,
                }
            },
        }, state::SharedState,
    },
};
use axum::extract::{Multipart, Path, State};
use multipart_derive::Multipart;

use crate::application::service::media::model::FileContainer;

#[derive(serde::Deserialize, Debug, Multipart)]
pub struct DevPayload {
    pub content: String,
    pub test_boolean: bool,
    // upload files
    #[multipart]
    pub uploaded_files: FileContainer,
}

#[axum::debug_handler]
pub async fn files_upload_handler(
    State(state): State<SharedState>,
    Path(_version): Path<String>,
    multipart: Multipart,
) -> Result<(), APIError> {
    let ext_opts = ExtractorFileOptions {
        max_files: Some(3),
        max_size: Some(100 * 1024 * 1024),
        validation: Some(
            ValidationOptions::new_whitelist()
                .add_type(FileType::Category(MediaType::Image))
                .add_type(FileType::Category(MediaType::Video))
        ),
        field_options: None,
    };

    let mut extracted = state.multi_extractor
        .extract::<DevPayload>(multipart, Some(ext_opts))
        .await?;

    let uploaded_files = &mut extracted.uploaded_files;

    uploaded_files
        .prepare_ids(&state.snowflake_generator)?
        .set_uploader_id(84547479869067264)
        .set_target_path(format!("dev_uploads/{}", state.snowflake_generator.generate_id()?))
        .set_config(
        ContainerConfig::new()
            .set_generate_thumbhash(true)
            .set_naming_strategy(NamingStrategy::FinalHash)
            .set_animated_image_indicator(true)
            .set_processing_options(MediaProcessorOptions::new()
                .set_fflags_video_gpu_accel(true)
                .set_fflags_video_thumbnail(true)
                .set_image_processors(vec![
                ImageProcessorType::Resize {
                    style: ResizeStyle::Normalized {
                        width: 0.5,
                        height: 0.5,
                    },
                    upscale: false,
                },
                ImageProcessorType::Crop {
                    style: CropStyle::Ratio {
                        width: 5,
                        height: 2,
                        scale: 1.0,
                    },
                    position: Some((0.5, 0.5)),
                },
            ]).set_post_video_processors( vec![
                VideoPostProcessorType::HLS { 
                    segment_time: 6 
                }
            ])
        ),
    );

    state.media_service
        .save_media(&state, uploaded_files)
        .await
        .map_err(|e| {
            tracing::error!("Failed to save media: {:?}", e);
            MediaServiceError::ProcessingFailed
        })?;
    
    // get media datas
    let mut tx = state.db_pool.begin().await?;
    for file in uploaded_files.resolve_files() {
        let media_full = media::get::media_full_data(&mut tx, &file.id).await?;
        tracing::info!("Media full data: {:#?}", media_full);
    }
    Ok(())
}
