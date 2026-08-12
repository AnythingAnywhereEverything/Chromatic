use axum::{
    Json,
    extract::{Multipart, Path, State},
};
use multipart_derive::Multipart;

use crate::{
    api::{
        APIError, RequestAuth,
        dtos::user_dtos::{MediaFullDTO, UserDTO},
        version,
    }, application::{
        repository::{
            media::{self as media_repo, row::MediaStatus},
            user::{self as user_repo},
        }, service::{
            errors::AuthServiceError, media::{
                processor::types::{CropStyle, ImageProcessorType, MediaProcessorFFlags, MediaProcessorOptions}, service::MediaService, service_type::MediaServiceOptions, types::{
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

    let user = user_repo::find::profile_full_by_id(&mut tx, user_id).await?;

    let avatar_media = match user.avatar_media_id {
        Some(media_id) => Some(media_repo::get::media_full_data(&mut tx, &media_id).await?),
        None => None,
    };

    let banner_media = match user.banner_media_id {
        Some(media_id) => Some(media_repo::get::media_full_data(&mut tx, &media_id).await?),
        None => None,
    };

    Ok(Json(UserDTO {
        id: user.id.to_string(),
        email: user.email,
        username: user.username,
        display_name: user.display_name,
        bio: user.bio,
        avatar_media_id: avatar_media.map(MediaFullDTO::from),
        banner_media_id: banner_media.map(MediaFullDTO::from),
        created_at: user.created_at.map(|dt| dt.to_rfc3339()),
    }))
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
        container: None,
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

    let updated_user = user_repo::find::profile_full_by_id(&mut tx, user_id).await?;

    tracing::warn!("Updated user after avatar upload: {:?}", updated_user);
    // get the media urls for the avatar and banner
    let avatar_media = match updated_user.avatar_media_id {
        Some(media_id) => Some(media_repo::get::media_full_data(&mut tx, &media_id).await?),
        None => None,
    };

    tracing::warn!("Avatar media after upload: {:?}", avatar_media);

    let banner_media = match updated_user.banner_media_id {
        Some(media_id) => Some(media_repo::get::media_full_data(&mut tx, &media_id).await?),
        None => None,
    };

    tx.commit().await?;

    Ok(Json(UserDTO {
        id: updated_user.id.to_string(),
        email: updated_user.email,
        username: updated_user.username,
        display_name: updated_user.display_name,
        bio: updated_user.bio,
        avatar_media_id: avatar_media.map(MediaFullDTO::from),
        banner_media_id: banner_media.map(MediaFullDTO::from),
        created_at: updated_user.created_at.map(|dt| dt.to_rfc3339()),
    }))
}
