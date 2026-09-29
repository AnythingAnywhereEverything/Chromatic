use axum::{
    extract::{FromRef, FromRequestParts},
    http::{header::USER_AGENT, request::Parts, StatusCode},
};
use axum_client_ip::ClientIp;
use tracing::warn;

use crate::{
    api::{APIError, APIErrorCode, APIErrorEntry, APIErrorKind},
    application::{
        repository::admin::find,
        service::{
            errors::{AuthServiceError, SessionServiceError},
            session_service::SessionService,
        },
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

/// The caller of a superuser-only endpoint, proven to be a live administrator.
///
/// Handlers take this instead of `RequestAuth` so the gate cannot be forgotten:
/// a handler that wants admin data has to name the extractor, and the extractor
/// is the only thing that decides whether the request is allowed.
///
/// The role is re-read from postgres on every request rather than trusted from
/// the token. That is the point. A token minted *before* a promotion gains
/// access as soon as the role lands, and a token held by an admin who is
/// suspended or demoted loses it on the very next call — the session is still
/// valid, only the permission is gone. `RequestAuth` cannot tell us any of that,
/// because `SessionToken` carries a user id and nothing about roles.
///
/// Adds no redis calls: the session is validated by `RequestAuth` on the normal
/// shared path, and this only reads postgres.
#[derive(Clone, Debug)]
pub struct AdminUser {
    /// The acting administrator, passed to the service as `performed_by` so the
    /// audit trail records who acted rather than just what happened.
    pub user_id: i64,
}

impl<S> FromRequestParts<S> for AdminUser
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
        let req_auth = RequestAuth::from_request_parts(parts, state).await?;

        let Some(user) = req_auth.user else {
            return Err(AuthServiceError::InvalidCredentials.into());
        };

        let mut tx = shared_state.db_pool.begin().await?;

        let gate = find::find_by_id(&mut tx, user.user_id).await?;

        // Read-only, so no commit: the transaction is dropped and rolled back.
        drop(tx);

        // A missing user, an inactive one, a deleted one, and a user who was
        // never an admin are all the same 403. Distinguishing them would turn
        // this endpoint into an oracle for which ids are real and which are
        // administrators.
        let authorized = gate
            .is_some_and(|row| row.is_active && row.is_superuser && row.deleted_at.is_none());

        if !authorized {
            return Err(Self::forbidden());
        }

        Ok(Self { user_id: user.user_id })
    }
}

impl AdminUser {
    fn forbidden() -> APIError {
        APIError::from((
            StatusCode::FORBIDDEN,
            APIErrorEntry::new("Superuser privileges are required.")
                .code(APIErrorCode::AuthenticationForbidden)
                .kind(APIErrorKind::AuthenticationError),
        ))
    }
}
