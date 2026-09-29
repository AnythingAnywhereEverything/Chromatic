use axum::{
    Json,
    extract::{Path, Query, State},
};
use serde::Deserialize;

use crate::{
    api::{version, AdminUser, APIError},
    application::{
        repository::admin::row::{
            AdminAuditRow, AdminStatsRow, AdminUserPage, AdminUserRow,
        },
        service::admin::{service::AdminService, types::AdminUserDetail},
        state::SharedState,
    },
};

/// Default rows per page for the user table. Matches what a moderation grid
/// shows without scrolling.
const DEFAULT_LIMIT: i64 = 50;
/// Default audit entries embedded in a user detail response. The standalone
/// audit endpoint is the way to page further back.
const DEFAULT_AUDIT_LIMIT: i64 = 20;

/// `GET /{version}/admin/users`
///
/// Query struct, not a `HashMap`, so a typo'd parameter is ignored by serde
/// rather than silently forwarded. `q` is the free-text term: it matches a
/// username, an email, or a numeric id.
///
/// `is_active` and `is_superuser` are tri-state — absent means "no filter",
/// which is why they are `Option<bool>` and not `bool`. A `?is_active=false`
/// is a real request for suspended users and must not collapse into "no filter".
#[derive(Deserialize, Debug)]
pub struct AdminUserListQuery {
    pub q: Option<String>,
    pub is_active: Option<bool>,
    pub is_superuser: Option<bool>,
    /// Keyset cursor. `before_id` is only meaningful alongside `before`, and
    /// supplying one without the other is ignored rather than an error, because
    /// a half-sent cursor degrades to the first page, which is harmless.
    pub before: Option<chrono::DateTime<chrono::Utc>>,
    pub before_id: Option<i64>,
    pub limit: Option<i64>,
}

/// `?audit_limit=` for the user detail endpoint.
#[derive(Deserialize, Debug)]
pub struct AdminUserDetailQuery {
    pub audit_limit: Option<i64>,
}

/// `?limit=` for the standalone audit feed.
#[derive(Deserialize, Debug)]
pub struct AdminAuditQuery {
    pub limit: Option<i64>,
}

/// Body of `PATCH /{version}/admin/users/{id}/role`.
///
/// A bare bool with no wrapper object, matching `LikeRequest` and
/// `BookmarkRequest` in `post_handler.rs`.
#[derive(Deserialize, Debug)]
pub struct SetRolePayload {
    pub is_superuser: bool,
}

/// One page of users, exact matches first.
pub async fn list_users_handler(
    State(state): State<SharedState>,
    Path(version): Path<String>,
    _admin: AdminUser,
    query: Query<AdminUserListQuery>,
) -> Result<Json<AdminUserPage>, APIError> {
    let api_version = version::parse_version(&version)?;
    tracing::trace!("api version: {}", api_version);

    let page = AdminService::list_users(
        &state,
        query.q.as_deref(),
        query.is_active,
        query.is_superuser,
        query.before,
        query.before_id,
        query.limit.unwrap_or(DEFAULT_LIMIT),
    )
    .await?;

    Ok(Json(page))
}

/// One user plus their moderation history.
///
/// Returns 200 with `deleted_at` set for a soft-deleted account, because a
/// moderator still needs to inspect one. An id that was never a user is a 404.
pub async fn get_user_detail_handler(
    State(state): State<SharedState>,
    Path((version, target_id)): Path<(String, i64)>,
    _admin: AdminUser,
    query: Query<AdminUserDetailQuery>,
) -> Result<Json<AdminUserDetail>, APIError> {
    let api_version = version::parse_version(&version)?;
    tracing::trace!("api version: {}", api_version);

    let detail = AdminService::get_user_detail(
        &state,
        target_id,
        query.audit_limit.unwrap_or(DEFAULT_AUDIT_LIMIT),
    )
    .await?;

    Ok(Json(detail))
}

/// `POST /{version}/admin/users/{id}/suspend`
///
/// Bodyless on purpose: a suspend is a decision, not an edit, and there is no
/// field here that a client could get wrong. Suspending yourself is refused by
/// the service with a 403.
pub async fn suspend_user_handler(
    State(state): State<SharedState>,
    Path((version, target_id)): Path<(String, i64)>,
    admin: AdminUser,
) -> Result<Json<AdminUserRow>, APIError> {
    let api_version = version::parse_version(&version)?;
    tracing::trace!("api version: {}", api_version);

    let user = AdminService::set_user_active(&state, admin.user_id, target_id, false).await?;

    Ok(Json(user))
}

/// `POST /{version}/admin/users/{id}/activate`
///
/// The mirror of `suspend_user_handler`, and the same service call with the
/// flag flipped. Sessions are not touched: there is nothing to restore, and a
/// suspended user's old rows were already deleted.
pub async fn activate_user_handler(
    State(state): State<SharedState>,
    Path((version, target_id)): Path<(String, i64)>,
    admin: AdminUser,
) -> Result<Json<AdminUserRow>, APIError> {
    let api_version = version::parse_version(&version)?;
    tracing::trace!("api version: {}", api_version);

    let user = AdminService::set_user_active(&state, admin.user_id, target_id, true).await?;

    Ok(Json(user))
}

/// `PATCH /{version}/admin/users/{id}/role`
///
/// `true` grants superuser, `false` revokes it. Revoking your own role is a
/// 403; revoking the last remaining superuser is a 409.
pub async fn update_user_role_handler(
    State(state): State<SharedState>,
    Path((version, target_id)): Path<(String, i64)>,
    admin: AdminUser,
    Json(payload): Json<SetRolePayload>,
) -> Result<Json<AdminUserRow>, APIError> {
    let api_version = version::parse_version(&version)?;
    tracing::trace!("api version: {}", api_version);

    let user = AdminService::update_user_role(
        &state,
        admin.user_id,
        target_id,
        payload.is_superuser,
    )
    .await?;

    Ok(Json(user))
}

/// `GET /{version}/admin/stats`
///
/// Dashboard counters in a single round trip.
pub async fn get_stats_handler(
    State(state): State<SharedState>,
    Path(version): Path<String>,
    _admin: AdminUser,
) -> Result<Json<AdminStatsRow>, APIError> {
    let api_version = version::parse_version(&version)?;
    tracing::trace!("api version: {}", api_version);

    let stats = AdminService::get_stats(&state).await?;

    Ok(Json(stats))
}

/// `GET /{version}/admin/audit`
///
/// Newest first, across every target. This is the pageless global feed; the
/// per-user subset is embedded in the detail response.
pub async fn list_audit_handler(
    State(state): State<SharedState>,
    Path(version): Path<String>,
    _admin: AdminUser,
    query: Query<AdminAuditQuery>,
) -> Result<Json<Vec<AdminAuditRow>>, APIError> {
    let api_version = version::parse_version(&version)?;
    tracing::trace!("api version: {}", api_version);

    let entries = AdminService::list_audit(&state, query.limit.unwrap_or(DEFAULT_LIMIT)).await?;

    Ok(Json(entries))
}
