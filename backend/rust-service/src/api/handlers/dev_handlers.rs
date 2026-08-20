use crate::{
    api::{APIError, RequestAuth, dtos::post_dtos::PostDTO, version}, application::{
        repository::{media::{self as media_repo, row::MediaStatus}, post::{self as post_repo}}, service::{errors::AuthServiceError, media::{
            processor::types::{
                CropStyle, ImageProcessorType, MediaProcessorFFlags, MediaProcessorOptions, PostProcessingType, ResizeStyle, VideoPostProcessorType,
            }, service::MediaService, service_type::MediaServiceOptions, types::{
                file::MultipartFile,
                media_options::{
                    FieldTypeFilter, MediaType, MultipartExtractorOptions, ValidationOptions,
                    ValidationType,
                },
            },
        }}, state::SharedState,
    },
};
use axum::{Json, extract::{Multipart, Path, State}};
use multipart_derive::Multipart;

#[derive(serde::Deserialize, Debug, Multipart)]
pub struct DevPayload {
    pub content: String,
    pub test_boolean: bool,
    // upload files
    #[multipart]
    pub images: Vec<MultipartFile>,
}

#[axum::debug_handler]
pub async fn files_upload_handler(
    State(state): State<SharedState>,
    Path(version): Path<String>,
    multipart: Multipart,
) -> Result<(), APIError> {
    let api_version = version::parse_version(&version)?;
    tracing::trace!("api version: {}", api_version);

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
        .extract::<DevPayload>(multipart, options)
        .await?;
    tracing::debug!("Extracted payload: {:#?}", extracted);

    let upload_group = &state.snowflake_generator.generate_id()?;

    let new_media_opts = MediaServiceOptions {
        upload_route: format!("dev_uploads/{}", upload_group),
        uploader_id: 1234,
        container: None,
        processor: Some(MediaProcessorOptions {
            fflags: Some(MediaProcessorFFlags {
                video_thumbnail: true,
                video_gpu_accel: true,
                video_transcode: true,
                image_thumbhash: true,
                ..Default::default()
            }),
            image_processors: Some(vec![
                ImageProcessorType::Crop { 
                    style: CropStyle::Ratio { width: 1, height: 1, scale: 1.0 },
                    position: Some((0.5, 0.5)),
                },
                ImageProcessorType::Resize {
                    style: ResizeStyle::Absolute {
                        width: 1024,
                        height: 1024,
                    },
                    upscale: false,
                },
            ]),
            video_processors: None,
            post_processors: Some(PostProcessingType::Video(vec![
                VideoPostProcessorType::HLS { segment_time: 10 },
            ])),
        }),
    };

    tracing::debug!("Media options for upload: {:#?}", new_media_opts);

    let mut tx = state.db_pool.begin().await?;

    tracing::debug!("Extracted multipart files: {:#?}", extracted.images);

    let media_service = MediaService::new();

    let all_media = media_service
        .save_media_group(&state, extracted.images, new_media_opts)
        .await?;

    tracing::debug!("Saved media group: {:#?}", all_media);
    for media in all_media {
        // set to complete the media processing
        media_repo::update::media_status(&mut tx, &media.get_id(), &MediaStatus::Completed).await?;
        tracing::debug!("Media processing completed for media ID: {}", media.get_id());
    }

    tx.commit().await?;
    Ok(())
}

pub async fn get_specific_post(
    State(state): State<SharedState>,
    Path((version, post_id)): Path<(String, i64)>,
    req_auth: RequestAuth,    
) -> Result <Json<PostDTO>, APIError> {
    let api_version = version::parse_version(&version)?;
    tracing::trace!("api version: {}", api_version);

    let user_id = match req_auth.user {
        Some(user) => user.user_id,
        None => return Err(AuthServiceError::InvalidCredentials.into()),
    };

    let mut tx = state.db_pool.begin().await?;
    let post: PostDTO = post_repo::post::get_post_by_id(&mut tx, post_id, user_id).await?.into();
    Ok(Json(post))
}