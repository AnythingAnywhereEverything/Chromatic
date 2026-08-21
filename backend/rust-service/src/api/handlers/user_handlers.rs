use axum::{
    Json,
    extract::{Multipart, Path, State},
};
use multipart_derive::Multipart;
use serde::Deserialize;

use crate::{
    api::{
        APIError, RequestAuth,
        dtos::user_dtos::{PublicUserProfileDTO, UserDTO},
        version,
    },
    application::{
        repository::{
            media::{self as media_repo, row::MediaStatus},
            user::{self as user_repo},
        },
        service::{
            errors::AuthServiceError,
            media::{
                processor::types::{
                    CropStyle, ImageProcessorType, MediaProcessorFFlags, MediaProcessorOptions,
                },
                service::MediaService,
                service_type::{ContainerConfig, MediaServiceOptions},
                types::{
                    file::MultipartFile,
                    media_options::{
                        MediaType, MultipartExtractorOptions, ValidationOptions, ValidationType,
                    },
                },
            },
        },
        state::SharedState,
    },
    domain::user::types::DisplayName,
};

/// Get user profile by username
/// * This is public endpoint, no authentication required
pub async fn get_user_profile_handler(
    State(state): State<SharedState>,
    Path((version, username)): Path<(String, String)>,
    req_auth: RequestAuth,
) -> Result<Json<PublicUserProfileDTO>, APIError> {
    let api_version = version::parse_version(&version)?;
    tracing::trace!("api version: {}", api_version);

    let user_id = match req_auth.user {
        Some(user) => Some(user.user_id),
        None => None,
    };

    let mut tx = state.db_pool.begin().await?;

    let user = user_repo::find::profile_full_by_username(&mut tx, &username, user_id).await?;

    Ok(Json(user.into()))
}

pub async fn get_current_user_profile_handler(
    State(state): State<SharedState>,
    Path(version): Path<String>,
    req_auth: RequestAuth,
) -> Result<Json<PublicUserProfileDTO>, APIError> {
    let api_version = version::parse_version(&version)?;
    tracing::trace!("api version: {}", api_version);

    let user_id = match req_auth.user {
        Some(user) => user.user_id,
        None => return Err(AuthServiceError::InvalidCredentials.into()), // temporary use logout failed error, will create a new error type for this case later
    };

    let mut tx = state.db_pool.begin().await?;

    let user = user_repo::find::profile_full_by_id(&mut tx, user_id).await?;

    Ok(Json(user.into()))
}

/// Update full profile of the current user
#[derive(Deserialize, Debug, Multipart)]
struct UpdateUserProfilePayload {
    pub display_name: Option<String>,
    pub bio: Option<String>,
    #[multipart]
    pub uploaded_avatar: Option<MultipartFile>,
    pub remove_avatar: Option<bool>,
    #[multipart]
    pub uploaded_banner: Option<MultipartFile>,
    pub remove_banner: Option<bool>,
}

pub async fn update_current_user_profile_handler(
    State(state): State<SharedState>,
    Path(version): Path<String>,
    req_auth: RequestAuth,
    multipart: Multipart,
) -> Result<Json<PublicUserProfileDTO>, APIError> {
    let api_version = version::parse_version(&version)?;
    tracing::trace!("api version: {}", api_version);

    let user_id = match req_auth.user {
        Some(user) => user.user_id,
        None => return Err(AuthServiceError::InvalidCredentials.into()), // temporary use logout failed error, will create a new error type for this case later
    };

    // extract multipart data
    let ext_opts = MultipartExtractorOptions {
        max_file_size: Some(10_000_000), // 10 MB
        max_files: Some(5),
        validation: Some(ValidationOptions {
            validation_type: ValidationType::Whitelisted,
            value: vec![
                MediaType::Image, // any image type
            ],
        }),
        ..Default::default()
    };

    let extracted = state
        .multipart_extractor
        .extract::<UpdateUserProfilePayload>(multipart, ext_opts)
        .await?;

    if extracted.display_name.is_none()
        && extracted.bio.is_none()
        && extracted.uploaded_avatar.is_none()
        && extracted.uploaded_banner.is_none()
        && extracted.remove_avatar.unwrap_or(false) == false
        && extracted.remove_banner.unwrap_or(false) == false
    {
        return Err((
            hyper::StatusCode::BAD_REQUEST,
            crate::api::APIErrorEntry {
                code: Some("no_fields_to_update".to_string()),
                message: "No fields to update".to_string(),
                ..Default::default()
            },
        )
            .into());
    }

    let mut tx = state.db_pool.begin().await?;

    if extracted.remove_avatar.unwrap_or(false) {
        user_repo::update::avatar_media_id(&mut tx, user_id, None).await?;
    }
    if extracted.remove_banner.unwrap_or(false) {
        user_repo::update::banner_media_id(&mut tx, user_id, None).await?;
    }

    if let Some(display_name) = &extracted.display_name {
        let dpn = DisplayName::new(display_name)?;
        user_repo::update::user_display_name(&mut tx, user_id, dpn.as_str()).await?;
    }
    if let Some(bio) = &extracted.bio {
        user_repo::update::user_bio(&mut tx, user_id, bio).await?;
    }

    tx.commit().await?;

    let mut new_media_opt = MediaServiceOptions {
        upload_route: format!("avatars/{}", user_id),
        uploader_id: user_id,
        container: Some(ContainerConfig {
            use_hash_names: true,
            use_animated_image_indicator: true,
            ..Default::default()
        }),
        processor: Some(MediaProcessorOptions {
            fflags: Some(MediaProcessorFFlags {
                image_thumbhash: true,
                ..Default::default()
            }),
            image_processors: Some(vec![]), // no processing for now, as the position and scale will be handled on the client side
            video_processors: None,
            post_processors: None,
        }),
    };

    // image are positioned and scaled on the client side
    // we just validating it, confirming that the image is valid and then saving it to the media service, and updating the user profile with the new media id
    if let Some(uploaded_avatar) = extracted.uploaded_avatar {
        new_media_opt.processor = Some(MediaProcessorOptions {
            fflags: Some(MediaProcessorFFlags {
                image_thumbhash: true,
                ..Default::default()
            }),
            image_processors: Some(vec![ImageProcessorType::Crop {
                style: CropStyle::Ratio {
                    width: 1,
                    height: 1,
                    scale: 1.0,
                },
                position: Some((0.5, 0.5)),
            }]),
            ..Default::default()
        });

        let media_service = MediaService::new();

        let mut tx = state.db_pool.begin().await?;
        let uploaded_medias = media_service
            .save_media(&state, uploaded_avatar, new_media_opt.clone())
            .await?;

        user_repo::update::avatar_media_id(&mut tx, user_id, Some(uploaded_medias.get_id()))
            .await?;

        media_repo::update::media_status(
            &mut tx,
            &uploaded_medias.get_id(),
            &MediaStatus::Completed,
        )
        .await?;

        tx.commit().await?;
    }

    if let Some(uploaded_banner) = extracted.uploaded_banner {
        new_media_opt.processor = Some(MediaProcessorOptions {
            fflags: Some(MediaProcessorFFlags {
                image_thumbhash: true,
                ..Default::default()
            }),
            image_processors: Some(vec![ImageProcessorType::Crop {
                style: CropStyle::Ratio {
                    width: 5,
                    height: 2,
                    scale: 1.0,
                },
                position: Some((0.5, 0.5)),
            }]),
            ..Default::default()
        });

        new_media_opt.upload_route = format!("banners/{}", user_id);

        let media_service: MediaService = MediaService::new();

        let mut tx = state.db_pool.begin().await?;
        let uploaded_medias = media_service
            .save_media(&state, uploaded_banner, new_media_opt)
            .await?;

        user_repo::update::banner_media_id(&mut tx, user_id, Some(uploaded_medias.get_id()))
            .await?;

        media_repo::update::media_status(
            &mut tx,
            &uploaded_medias.get_id(),
            &MediaStatus::Completed,
        )
        .await?;

        tx.commit().await?;
    }

    let mut tx = state.db_pool.begin().await?;

    let user = user_repo::find::profile_full_by_id(&mut tx, user_id).await?;

    Ok(Json(user.into()))
}

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

    let user = user_repo::find::profile_with_minimal_by_id(&mut tx, user_id).await?;

    Ok(Json(user.into()))
}