use axum::{
    Json,
    extract::{Path, State},
};

use crate::{
    api::{
        APIError, RequestAuth,
        dtos::auth_dtos::{LoginRequest, LoginResponse, OauthRequest, OauthResponse, RegisterRequest, RegisterResponse},
        version,
    }, application::{
        service::{auth::{self, provider::errors::ProviderError, service::AuthService}, errors::AuthServiceError, profile_service::ProfileService, session_service::SessionService}, state::SharedState,
    },
};

pub async fn oauth_handler(
    State(state): State<SharedState>,
    Path(version): Path<String>,
    req_header: RequestAuth,
    Json(payload): Json<OauthRequest>,
) -> Result<Json<OauthResponse>, APIError> {
    let api_version = version::parse_version(&version)?;
    tracing::trace!("api version: {}", api_version);
    tracing::trace!("oauth request: {:#?}", payload);
    tracing::trace!("request header: {:#?}", req_header);

    match payload.provider.as_str() {
        "google" => {
            let google_userinfo =
                auth::provider::google::fetch_google_userinfo(&payload.access_token)
                    .await
                    .map_err(|e| {
                        tracing::error!("Failed to fetch Google user info: {}", e);
                        AuthServiceError::ProviderError(ProviderError::FetchUserInfoFailed)
                    })?;

            tracing::trace!("Google user info: {:#?}", google_userinfo);

            let res_userid = AuthService::oauth(
                &state,
                "google",
                &google_userinfo.sub,
                &google_userinfo.email,
            )
            .await?;

            // create session for the user
            let session_token = SessionService::create_session(
                &state,
                res_userid,
                &req_header.user_agent,
                &req_header.ip_address,
                60 * 60,
            )
            .await?;

            Ok(Json(OauthResponse {
                token: session_token.full_token,
                user_id: res_userid.to_string(),
            }))
        }
        _ => return Err(AuthServiceError::UnsupportedProvider.into()),
    }
}

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

    ProfileService::init_user_settings(&state, login_res.user_id).await?;

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

pub async fn delete_handler(
    State(state): State<SharedState>,
    Path(version): Path<String>,
    req_header: RequestAuth,
) -> Result<(), APIError> {
    let api_version = version::parse_version(&version)?;
    tracing::trace!("api version: {}", api_version);
    tracing::trace!("delete account request header: {:#?}", req_header);

    let user_id = match req_header.user {
        Some(user) => user.user_id,
        None => return Err(AuthServiceError::DeleteAccountFailed.into()),
    };

    AuthService::delete_account(&state, user_id).await?;
    Ok(())
}