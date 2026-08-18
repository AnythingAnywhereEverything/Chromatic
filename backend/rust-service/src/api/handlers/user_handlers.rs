use axum::{
    Json,
    extract::{Multipart, Path, State},
};
use multipart_derive::Multipart;

use crate::{
    api::{
        APIError, RequestAuth,
        dtos::user_dtos::UserDTO,
        version,
    }, application::{
        repository::{
            media::{self as media_repo, row::MediaStatus},
            user::{self as user_repo},
        }, service::{
            errors::AuthServiceError, media::{
                processor::types::{CropStyle, ImageProcessorType, MediaProcessorFFlags, MediaProcessorOptions}, service::MediaService, service_type::{ContainerConfig, MediaServiceOptions}, types::{
                    file::MultipartFile, media_options::{MediaType, MultipartExtractorOptions, ValidationOptions, ValidationType},
                },
            },
        }, state::SharedState,
    },
};

pub async fn get_current_user_handler(
    State(state): State<SharedState>,
    Path(version): Path<String>,
    req_auth: RequestAuth,
) -> Result<Json<UserDTO>, APIError> {
    let api_version = version::parse_version(&version)?;
    tracing::trace!("api version: {}", api_version);

    let mut tx = state.db_pool.begin().await?;

    let user_id = match req_auth.user {
        Some(user) => user.user_id,
        None => return Err(AuthServiceError::InvalidCredentials.into()), // temporary use logout failed error, will create a new error type for this case later
    };

    let user = user_repo::find::profile_with_minimal_media_by_id(&mut tx, user_id).await?;

    Ok(Json(user.into()))
}

#[derive(serde::Deserialize, Debug, Multipart)]
pub struct UploadAvatarPayload {
    position_x: f32,
    position_y: f32,
    scale: f32,

    #[multipart]
    pub uploaded_avatar: MultipartFile,
}

#[axum::debug_handler]
pub async fn upload_avatar_handler(
    State(state): State<SharedState>,
    Path(version): Path<String>,
    req_auth: RequestAuth,
    multipart: Multipart,
) -> Result<Json<UserDTO>, APIError> {
    let api_version = version::parse_version(&version)?;
    tracing::trace!("api version: {}", api_version);

    let user_id = match req_auth.user {
        Some(user) => user.user_id,
        None => return Err(AuthServiceError::InvalidCredentials.into()),
    };

    let options = MultipartExtractorOptions {
        max_file_size: Some(10_000_000), // 10 MB
        max_files: Some(5),
        validation: Some(ValidationOptions {
            validation_type: ValidationType::Whitelisted,
            value: vec![
                MediaType::GenericJpeg,
                MediaType::GenericPng,
                MediaType::GenericWebP,
                MediaType::GenericGif,
            ],
        }),
        ..Default::default()
    };

    let extracted = state
        .multipart_extractor
        .extract::<UploadAvatarPayload>(multipart, options)
        .await?;

    tracing::debug!("Extracted payload: {:?}", extracted);

    let new_media_opts = MediaServiceOptions {
        upload_route: format!("avatars/{}", user_id),
        uploader_id: user_id,
        container: Some(ContainerConfig {
            use_hash_names: true,
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
            image_processors: Some(vec![ImageProcessorType::Crop {
                style: CropStyle::Ratio {
                    width: 1,
                    height: 1,
                    scale: extracted.scale,
                },
                position: Some((extracted.position_x, extracted.position_y)),
            }]),
            video_processors: None,
            post_processors: None,
        }),
    };

    let media_service = MediaService::new();

    // local test, doing this for now.
    let mut tx = state.db_pool.begin().await?;
    let uploaded_medias = media_service.save_media(
        &state,
        extracted.uploaded_avatar,
        new_media_opts,
    )
    .await?;

    user_repo::update::avatar_media_id(&mut tx, user_id, Some(uploaded_medias.get_id())).await?;

    media_repo::update::media_status(&mut tx, &uploaded_medias.get_id(), &MediaStatus::Completed)
        .await?;

    tx.commit().await?;

    let mut tx = state.db_pool.begin().await?;
    let updated_user = user_repo::find::profile_with_minimal_media_by_id(&mut tx, user_id).await?;
    tx.commit().await?;


    Ok(Json(updated_user.into()))
}
