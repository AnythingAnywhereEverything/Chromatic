use axum::{
    extract::{FromRef, FromRequestParts},
    http::{header::USER_AGENT, request::Parts},
};
use axum_client_ip::ClientIp;
use tracing::warn;

use crate::{
    api::APIError,
    application::{
        service::{errors::SessionServiceError, session_service::SessionService},
        state::SharedState,
    },
    domain::session::SessionToken,
};

#[derive(Clone, Debug)]
pub struct AuthUser {
    pub user_id: i64,
    pub token: String,
}

#[derive(Clone, Debug)]
pub struct RequestAuth {
    pub user_agent: String,
    pub ip_address: String,
    pub user: Option<AuthUser>,
}

impl<S> FromRequestParts<S> for RequestAuth
where
    SharedState: FromRef<S>,
    S: Send + Sync,
{
    type Rejection = APIError;

    async fn from_request_parts(
        parts: &mut Parts,
        state: &S,
    ) -> Result<Self, Self::Rejection> {
        let shared_state = SharedState::from_ref(state);

        let user_agent = parts
            .headers
            .get(USER_AGENT)
            .and_then(|v| v.to_str().ok())
            .unwrap_or("unknown")
            .to_string();

        let ip_address = match ClientIp::from_request_parts(parts, state).await {
            Ok(ip) => ip.0.to_string(),
            Err(err) => {
                warn!(
                    error = ?err,
                    x_real_ip = ?parts.headers.get("x-real-ip"),
                    x_forwarded_for = ?parts.headers.get("x-forwarded-for"),
                    "failed to extract client ip"
                );
                "unknown".to_string()
            }
        };

        let user = match parts.headers.get("token").and_then(|t| t.to_str().ok()) {
            None | Some("") => None,
            Some(token) => {
                let parsed = SessionToken::parse(token).map_err(SessionServiceError::from)?;

                SessionService::validate_session(
                    &shared_state,
                    &parsed,
                    60 * 60,
                    60 * 60,
                )
                .await?;

                Some(AuthUser {
                    user_id: parsed.user_id,
                    token: token.to_string(),
                })
            }
        };

        Ok(Self {
            user_agent,
            ip_address,
            user,
        })
    }
}