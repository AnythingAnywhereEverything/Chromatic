use axum::{
    Json,
    extract::{Multipart, Path, State},
    http::StatusCode,
};
use multipart_derive::Multipart;
use serde::Deserialize;

use crate::{
    api::{
        APIError, RequestAuth,
        dtos::user_dtos::{PublicUserProfileDTO, UserDTO},
        version,
    }, application::{
        repository::{
            media::row::MediaType, user::{
                self as user_repo, find::URDQOpts, row::{FollowUserRow, PendingFollowRow, SettingsType, UserSettingRow},
            },
        }, service::{
            errors::{AuthServiceError, ProfileServiceError},
            media::{
                extractor::{ExtractorFileOptions, ValidationOptions},
                inspector::FileType,
                model::FileContainer,
            },
            profile_service::ProfileService,
            report::service::ReportService,
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

    let user = ProfileService::get_profile_username(&state, username, user_id).await?;

    Ok(Json(user.into()))
}
#[axum::debug_handler]
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

    let user = user_repo::find::profile_full_by_id(&mut tx, user_id, Some(user_id)).await?;

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

#[derive(serde::Serialize, Deserialize, Debug)]
pub struct UpdateUserSettingPayload {
    pub setting_value: serde_json::Value,
}
pub async fn get_user_setting_handler(
    State(state): State<SharedState>,
    Path((version, setting_type)): Path<(String, SettingsType)>,
    req_auth: RequestAuth,
) -> Result<Json<UserSettingRow>, APIError> {
    let api_version = version::parse_version(&version)?;
    tracing::trace!("api version: {}", api_version);

    let user_id = match req_auth.user {
        Some(user) => user.user_id,
        None => return Err(AuthServiceError::InvalidCredentials.into()),
    };

    let setting =
        ProfileService::get_user_setting(&state, user_id, setting_type).await?;

    Ok(Json(setting.into()))
}

pub async fn update_user_setting_handler(
    State(state): State<SharedState>,
    Path((version, setting_type)): Path<(String, SettingsType)>,
    req_auth: RequestAuth,
    Json(payload): Json<UpdateUserSettingPayload>,
) -> Result<Json<UserSettingRow>, APIError> {
    let api_version = version::parse_version(&version)?;
    tracing::trace!("api version: {}", api_version);

    let user_id = match req_auth.user {
        Some(user) => user.user_id,
        None => return Err(AuthServiceError::InvalidCredentials.into()),
    };

    let setting =
        ProfileService::update_user_setting(&state, user_id, setting_type, payload.setting_value)
            .await?;

    Ok(Json(setting.into()))
}

pub async fn get_pending_follow_requests_handler(
    State(state): State<SharedState>,
    Path(version): Path<String>,
    req_auth: RequestAuth,
) -> Result<Json<Vec<PendingFollowRow>>, APIError> {
    let api_version = version::parse_version(&version)?;
    tracing::trace!("api version: {}", api_version);

    let user_id = match req_auth.user {
        Some(user) => user.user_id,
        None => return Err(AuthServiceError::InvalidCredentials.into()),
    };

    let requests = ProfileService::get_pending_follow_requests(&state, user_id).await?;

    Ok(Json(requests))
}

// ? I don't know I should doing it seperate or in the same handler as follow_user_handler
#[axum::debug_handler]
pub async fn follow_user_handler(
    State(state): State<SharedState>,
    Path((version, following_id)): Path<(String, i64)>,
    req_auth: RequestAuth,
) -> Result<Json<FollowUserRow>, APIError> {
    let api_version = version::parse_version(&version)?;
    tracing::trace!("api version: {}", api_version);

    let user_id = match req_auth.user {
        Some(user) => user.user_id,
        None => return Err(AuthServiceError::InvalidCredentials.into()),
    };

    if user_id == following_id {
        return Err(ProfileServiceError::CannotFollowYourself.into());
    }

    let target_follow_setting =
        ProfileService::get_user_setting(&state, following_id, SettingsType::Privacy).await?;

    if target_follow_setting.setting_value["who_can_follow_me"] == "request" {
        let result = ProfileService::follow_user
        (&state, user_id, following_id, "pending").await?;
        return Ok(Json(result));
    }

    let result = ProfileService::follow_user(&state, user_id, following_id, "followed").await?;

    Ok(Json(result))
}

pub async fn unfollow_user_handler(
    State(state): State<SharedState>,
    Path((version, following_id)): Path<(String, i64)>,
    req_auth: RequestAuth,
) -> Result<(), APIError> {
    let api_version = version::parse_version(&version)?;
    tracing::trace!("api version: {}", api_version);

    let user_id = match req_auth.user {
        Some(user) => user.user_id,
        None => return Err(AuthServiceError::InvalidCredentials.into()),
    };

    ProfileService::unfollow_user(&state, user_id, following_id).await?;

    Ok(())
}

pub async fn follow_user_accept_handler(
    State(state): State<SharedState>,
    Path((version, follower_id)): Path<(String, i64)>,
    req_auth: RequestAuth,
) -> Result<Json<FollowUserRow>, APIError> {
    let api_version = version::parse_version(&version)?;
    tracing::trace!("api version: {}", api_version);

    let user_id = match req_auth.user {
        Some(user) => user.user_id,
        None => return Err(AuthServiceError::InvalidCredentials.into()),
    };

    if user_id == follower_id {
        return Err(ProfileServiceError::CannotFollowYourself.into());
    }

    let result = ProfileService::accept_follow_request(&state, user_id, follower_id).await?;

    Ok(Json(result))
}

pub async fn follow_user_reject_handler(
    State(state): State<SharedState>,
    Path((version, follower_id)): Path<(String, i64)>,
    req_auth: RequestAuth,
) -> Result<(), APIError> {
    let api_version = version::parse_version(&version)?;
    tracing::trace!("api version: {}", api_version);

    let user_id = match req_auth.user {
        Some(user) => user.user_id,
        None => return Err(AuthServiceError::InvalidCredentials.into()),
    };

    if user_id == follower_id {
        return Err(ProfileServiceError::CannotFollowYourself.into());
    }

    ProfileService::reject_follow_request(&state, user_id, follower_id).await?;

    Ok(())
}

/// File a report against a user, post, or comment.
///
/// The body names the target rather than the path, because all three target
/// types share one route. A path parameter would force three endpoints that
/// differ only in which column they validate against.
///
/// `reporter_id` comes from the token and is not a body field, so a client
/// cannot file a report as someone else.
///
/// Returns 201 with no body: the created row is not echoed back. It carries no
/// information the caller did not send, and `load_target` has just read the
/// target — returning a snapshot of it would only invite a caller to treat a
/// report receipt as moderation evidence.
/// Body for `POST /{version}/users/report`.
///
/// `target_type` is a string rather than an enum so a client sending garbage
/// gets a 400 with `report_invalid_target_type` naming the offending field,
/// instead of axum's own rejection, which arrives before this handler runs and
/// says nothing about which field was wrong. The service matches it against a
/// fixed list either way.
///
/// `description` is optional. `report_data` is `JSONB NOT NULL`, so "no
/// description" is `null` inside the stored object rather than a missing column.
#[derive(Deserialize, Debug)]
pub struct ReportPayload {
    #[serde(deserialize_with = "deserialize_i64")]
    pub target_id: i64,
    pub target_type: String,
    pub report_type: String,
    pub description: Option<String>,
}

pub async fn report_user_handler(
    State(state): State<SharedState>,
    Path(version): Path<String>,
    req_auth: RequestAuth,
    Json(payload): Json<ReportPayload>,
) -> Result<StatusCode, APIError> {
    let api_version = version::parse_version(&version)?;
    tracing::trace!("api version: {}", api_version);

    let user_id = match req_auth.user {
        Some(user) => user.user_id,
        None => return Err(AuthServiceError::InvalidCredentials.into()),
    };

    ReportService::submit(
        &state,
        user_id,
        payload.target_id,
        &payload.target_type,
        &payload.report_type,
        payload.description.as_deref(),
    )
    .await?;

    Ok(StatusCode::CREATED)
}

// just let it work first
fn deserialize_i64<'de, D>(deserializer: D) -> Result<i64, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let value = String::deserialize(deserializer)?;
    value.parse::<i64>().map_err(serde::de::Error::custom)
}