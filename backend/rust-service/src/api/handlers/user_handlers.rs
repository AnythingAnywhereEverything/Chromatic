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
    }, application::{
        repository::{media::row::MediaType, user::{self as user_repo, find::URDQOpts}}, service::{
            errors::AuthServiceError, media::{extractor::{ExtractorFileOptions, ValidationOptions}, inspector::FileType, model::FileContainer}, profile_service::ProfileService,
        }, state::SharedState,
    },
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

    let user = user_repo::find::profile_full_by_id(&mut tx, user_id, None).await?;

    Ok(Json(user.into()))
}

/// Update full profile of the current user
#[derive(Deserialize, Debug, Multipart)]
struct UpdateUserProfilePayload {
    pub display_name: Option<String>,
    pub bio: Option<String>,
    pub quote: Option<String>,
    #[multipart]
    pub uploaded_avatar: Option<FileContainer>,
    pub remove_avatar: Option<bool>,
    #[multipart]
    pub uploaded_banner: Option<FileContainer>,
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
    let ext_opts = ExtractorFileOptions {
        max_size: Some(10_000_000), // 10 MB
        max_files: Some(5),
        validation: Some(
            ValidationOptions::new_whitelist().add_type(FileType::Category(MediaType::Image)),
        ),
        ..Default::default()
    };

    let mut extracted = state
        .multi_extractor
        .extract::<UpdateUserProfilePayload>(multipart, Some(ext_opts))
        .await?;

    let uploaded_profile = ProfileService::update_profile(
        &state,
        user_id,
        extracted.display_name,
        extracted.bio,
        extracted.quote,
        &mut extracted.uploaded_avatar,
        extracted.remove_avatar,
        &mut extracted.uploaded_banner,
        extracted.remove_banner,
    )
    .await?;

    Ok(Json(uploaded_profile.into()))
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


pub async fn get_user_minimal_handler(
    State(state): State<SharedState>,
    Path((version, user_id)): Path<(String, i64)>,
) -> Result<Json<PublicUserProfileDTO>, APIError> {
    let api_version = version::parse_version(&version)?;
    tracing::trace!("api version: {}", api_version);

    let opts = URDQOpts {
        target_id: Some(user_id),
        target_username: None,
        get_email: false,
        get_avatar: true,
        ..Default::default()
    };

    let user = ProfileService::get_profile_with_opts(&state, opts).await?;

    Ok(Json(user.into()))
}