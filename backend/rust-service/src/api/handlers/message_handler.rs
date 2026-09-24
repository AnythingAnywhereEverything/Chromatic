use axum::{
    Json,
    extract::{Multipart, Path, Query, State},
};

use crate::{
    api::{APIError, RequestAuth, version},
    application::{
        repository::{messages::row::MessageRow, user::row::UserProfileRow},
        service::{errors::AuthServiceError, message_service::MessageService},
        state::SharedState,
    },
};

#[derive(serde::Deserialize, Debug)]
pub struct MessageQuery {
    pub limit: Option<i32>,
    pub before: Option<chrono::DateTime<chrono::Utc>>,
    pub before_id: Option<i64>,
}

pub async fn get_message_chat_handler(
    State(state): State<SharedState>,
    Path((version, target_id)): Path<(String, i64)>,
    req_auth: RequestAuth,
    query: Query<MessageQuery>,
) -> Result<Json<Vec<MessageRow>>, APIError> {
    let api_version = version::parse_version(&version);
    tracing::trace!("api_version: {:?}", api_version);

    let user_id = match req_auth.user {
        Some(user) => user.user_id,
        None => return Err(AuthServiceError::InvalidCredentials.into()),
    };

    let before = query.before.unwrap_or_else(chrono::Utc::now);
    let limit = query.limit.map(|limit| limit.clamp(1, 100)).unwrap_or(11);

    let message = MessageService
        .get_messages_chat(&state, user_id, target_id, before, query.before_id, limit)
        .await?;
    Ok(Json(message))
}

pub async fn send_message_handler(
    State(state): State<SharedState>,
    Path((version, target_id)): Path<(String, i64)>,
    req_auth: RequestAuth,
    payload: Multipart,
) -> Result<Json<MessageRow>, APIError> {
    let api_version = version::parse_version(&version);
    tracing::trace!("api_version: {:?}", api_version);

    let user_id = match req_auth.user {
        Some(user) => user.user_id,
        None => return Err(AuthServiceError::InvalidCredentials.into()),
    };

    let message = MessageService
        .create_message(&state, user_id, target_id, payload)
        .await?;
    Ok(Json(message))
}

pub async fn update_message_handler(
    State(state): State<SharedState>,
    Path((version, message_id)): Path<(String, i64)>,
    req_auth: RequestAuth,
    payload: Multipart,
) -> Result<Json<MessageRow>, APIError> {
    let api_version = version::parse_version(&version);
    tracing::trace!("api_version: {:?}", api_version);

    let user_id = match req_auth.user {
        Some(user) => user.user_id,
        None => return Err(AuthServiceError::InvalidCredentials.into()),
    };

    let message = MessageService
        .update_message(&state, user_id, message_id, payload)
        .await?;
    Ok(Json(message))
}

pub async fn delete_message_handler(
    State(state): State<SharedState>,
    Path((version, message_id)): Path<(String, i64)>,
    req_auth: RequestAuth,
) -> Result<(), APIError> {
    let api_version = version::parse_version(&version);
    tracing::trace!("api_version: {:?}", api_version);

    let user_id = match req_auth.user {
        Some(user) => user.user_id,
        None => return Err(AuthServiceError::InvalidCredentials.into()),
    };

    MessageService
        .delete_message(&state, message_id, user_id)
        .await?;
    Ok(())
}

pub async fn get_followed_user_handler(
    State(state): State<SharedState>,
    Path(version): Path<String>,
    req_auth: RequestAuth,
) -> Result<Json<Vec<UserProfileRow>>, APIError> {
    let api_version = version::parse_version(&version);
    tracing::trace!("api_version: {:?}", api_version);

    let user_id = match req_auth.user {
        Some(user) => user.user_id,
        None => return Err(AuthServiceError::InvalidCredentials.into()),
    };

    tracing::trace!("Getting followed users for user_id: {}", user_id);
    let followed_users = MessageService.get_followed_users(&state, user_id).await?;
    Ok(Json(followed_users))
}

pub async fn get_single_message_handler(
    State(state): State<SharedState>,
    Path((version, message_id)): Path<(String, i64)>,
    req_auth: RequestAuth,
) -> Result<Json<Option<MessageRow>>, APIError> {
    let api_version = version::parse_version(&version);
    tracing::trace!("api_version: {:?}", api_version);

    let user_id = match req_auth.user {
        Some(user) => user.user_id,
        None => return Err(AuthServiceError::InvalidCredentials.into()),
    };

    let message = MessageService
        .get_message(&state, message_id, user_id)
        .await?;

    Ok(Json(message))
}
