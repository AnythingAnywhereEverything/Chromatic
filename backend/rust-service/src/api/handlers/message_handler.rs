use axum::{
    Json,
    extract::{Multipart, Path, State, Query},
};

use crate::{
    api::{APIError, RequestAuth, version},
    application::{
        repository::messages::row::MessageRow,
        service::{errors::AuthServiceError, message_service::MessageService},
        state::SharedState,
    },
};

#[derive(serde::Deserialize, Debug)]
pub struct MessageQuery {
    pub limit: Option<i32>,
    pub before: chrono::DateTime<chrono::Utc>,
}

#[axum::debug_handler]
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

    let message = MessageService
        .get_messages_chat(
            &state,
            user_id,
            target_id,
            query.before,
            query.limit.unwrap_or(11) as i32,
        )
        .await?;
    Ok(Json(message))
}

#[axum::debug_handler]
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

#[axum::debug_handler]
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

#[axum::debug_handler]
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