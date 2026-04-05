use axum::{
    Json,
    extract::{Path, State},
};

use crate::{
    api::{
        APIError, RequestAuth,
        dtos::auth_dtos::{LoginRequest, LoginResponse, RegisterRequest, RegisterResponse},
        version,
    },
    application::{
        service::{auth::service::AuthService, errors::AuthServiceError},
        state::SharedState,
    },
};

pub async fn login_handler(
    State(state): State<SharedState>,
    Path(version): Path<String>,
    req_header: RequestAuth,
    Json(payload): Json<LoginRequest>,
) -> Result<Json<LoginResponse>, APIError> {
    let api_version = version::parse_version(&version)?;
    tracing::trace!("api version: {}", api_version);
    tracing::trace!("login request: {:#?}", payload);
    tracing::trace!("request header: {:#?}", req_header);

    let res = AuthService::login(
        &state,
        &payload.username_or_email,
        &payload.password,
        req_header,
    )
    .await?;

    Ok(Json(LoginResponse {
        token: res.token,
        user_id: res.user_id.to_string(),
    }))
}

pub async fn register_handler(
    State(state): State<SharedState>,
    Path(version): Path<String>,
    req_header: RequestAuth, // from extractors, contains optional(user_token), user agent and IP address
    Json(payload): Json<RegisterRequest>,
) -> Result<Json<RegisterResponse>, APIError> {
    let api_version = version::parse_version(&version)?;
    tracing::trace!("api version: {}", api_version);
    tracing::trace!("register request: {:#?}", payload);

    AuthService::register(
        &state,
        &payload.display_name,
        &payload.username,
        &payload.password,
        &payload.email,
    )
    .await?;

    // * After successful registration, we can directly log the user in by creating a session for them.
    let login_res =
        AuthService::login(
            &state, 
            &payload.username.to_lowercase(), // ensure username is in lowercase for login, as we store it in lowercase in the database
            &payload.password,
            req_header).await?;

    Ok(Json(RegisterResponse {
        token: login_res.token,
        user_id: login_res.user_id.to_string(),
    }))
}

pub async fn logout_handler(
    State(state): State<SharedState>,
    Path(version): Path<String>,
    req_header: RequestAuth,
) -> Result<(), APIError> {
    let api_version = version::parse_version(&version)?;
    tracing::trace!("api version: {}", api_version);
    tracing::trace!("logout request header: {:#?}", req_header);

    let token = match req_header.user {
        Some(user) => user.token,
        None => return Err(AuthServiceError::LogoutFailed.into()),
    };

    AuthService::logout(&state, &token).await?;

    Ok(())
}
