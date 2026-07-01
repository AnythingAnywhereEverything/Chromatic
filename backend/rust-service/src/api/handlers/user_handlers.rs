use axum::{
    Json,
    extract::{Multipart, Path, State},
};

use crate::{
    api::{APIError, RequestAuth, dtos::user_dtos::{MediaFullDTO, UserDTO}, version}, application::{
        repository::{media::{self as media_repo, row::MediaStatus}, user::{self as user_repo}}, service::{
            errors::{AuthServiceError, MediaServiceError},
            media::{
                service::MediaService,
                types::{CropStyle, ImageTransform, MediaOptions, MediaProcessingMode},
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

#[derive(serde::Deserialize, Debug)]
pub struct UploadAvatarPayload {
    position_x: f32,
    position_y: f32,
    scale: f32,
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

    let extracted = MediaService::extract_payload_with_type::<UploadAvatarPayload>(
        &state.media_service,
        multipart,
        10_000_000,
    )
    .await?;

    tracing::debug!("Extracted payload: {:?}", extracted.payload);

    let file = extracted.files;

    //log the extracted payload for debugging
    tracing::warn!("Extracted payload: {:?}", extracted.payload);

    let options = MediaOptions {
        max_size: 10_000_000, // 10 MB
        allowed_types: None,
        folder: "avatars".to_string(),
        image_transform: Some(ImageTransform::Crop {
            style: CropStyle::Ratio {
                ratio: (1, 1),
                scale: extracted.payload.scale,
            },
            position: Some((extracted.payload.position_x, extracted.payload.position_y)),
        }),
        mode: MediaProcessingMode::Sanitize,
        hls_fallback: false,
        manual_preview: None, // no auto preview for profile bozo
    };

    if file.is_empty() {
        return Err(MediaServiceError::MediaMissing.into());
    }

    let temp_uploaded = &file[0];

    // local test, doing this for now.
    let mut tx = state.db_pool.begin().await?;
    let uploaded_id = MediaService::save_media(&state.media_service, &mut tx, user_id, temp_uploaded.clone(), options, None)
        .await?;

    user_repo::update::avatar_media_id(&mut tx, user_id, Some(uploaded_id)).await?;

    media_repo::update::media_status(&mut tx, &uploaded_id, &MediaStatus::Completed).await?;

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
