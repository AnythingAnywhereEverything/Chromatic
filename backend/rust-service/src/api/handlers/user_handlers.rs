use axum::{
    Json,
    extract::{Path, State},
};

use crate::{
    api::{
        APIError, RequestAuth, dtos::user_dtos::UserDTO, version
    },
    application::{
        repository::user::{self as user_repo}, service::errors::AuthServiceError, state::SharedState
    }
};

#[axum::debug_handler]
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
        None => return Err(AuthServiceError::LogoutFailed.into()), // temporary use logout failed error, will create a new error type for this case later
    };

    let user = user_repo::find::profile_full_by_id(&mut tx, user_id).await?;

    Ok(Json(UserDTO {
        id: user.id.to_string(),
        email: user.email,
        username: user.username,
        display_name: user.display_name,
        bio: user.bio,
        avatar_url: user.avatar_url,
    }))
}