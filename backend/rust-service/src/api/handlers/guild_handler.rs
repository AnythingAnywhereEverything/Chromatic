use axum::{
    Json,
    extract::{Path, Query, State},
};
use serde::Deserialize;

use crate::{
    api::{APIError, RequestAuth, version},
    application::{
        repository::guild::row::{GuildPage, GuildRow},
        service::{errors::AuthServiceError, guild::service::GuildService},
        state::SharedState,
    },
};

/// Default rows per page for the browse list.
const DEFAULT_LIMIT: i64 = 20;

/// Body of `POST /{version}/guilds`.
///
/// `description` is optional; an all-whitespace value is collapsed to `NULL`
/// by the service, and an absent field is already `None`.
#[derive(Deserialize, Debug)]
pub struct CreateGuildRequest {
    pub name: String,
    pub description: Option<String>,
}

/// `?before=&before_id=&limit=` for the browse list.
///
/// Keyset cursor, same shape as the admin and post list endpoints: `before` is
/// the `created_at` of the last row the client has seen, `before_id` its id.
/// `before_id` without `before` is ignored rather than an error, degrading to
/// the first page, which is harmless.
#[derive(Deserialize, Debug)]
pub struct BrowseGuildsQuery {
    pub before: Option<chrono::DateTime<chrono::Utc>>,
    pub before_id: Option<i64>,
    pub limit: Option<i64>,
}

/// `POST /{version}/guilds`
///
/// Requires a logged-in user, who becomes the owner and first member. The
/// response is the freshly-created guild with `total_members = 1`,
/// `total_channels = 1` (the auto-created `#general` channel) and
/// `is_owner = true`.
pub async fn create_guild_handler(
    State(state): State<SharedState>,
    Path(version): Path<String>,
    req_auth: RequestAuth,
    payload: Json<CreateGuildRequest>,
) -> Result<Json<GuildRow>, APIError> {
    let api_version = version::parse_version(&version)?;
    tracing::trace!("api version: {}", api_version);

    let user_id = match req_auth.user {
        Some(user) => user.user_id,
        None => return Err(AuthServiceError::InvalidCredentials.into()),
    };

    let guild = GuildService::create(
        &state,
        user_id,
        &payload.name,
        payload.description.as_deref(),
    )
    .await?;

    Ok(Json(guild))
}

/// `POST /{version}/guilds/{guild_id}/join`
///
/// Requires a logged-in user. A guild that is missing or soft-deleted is a
/// 404; an existing membership is a 409 that leaves `total_members` untouched.
/// The response is the guild re-read after the join, so `total_members`
/// reflects the new member and `is_owner` says whether the joiner owns the
/// guild — which for a join that got this far is always `false`, since the
/// owner is already seated by the create flow.
pub async fn join_guild_handler(
    State(state): State<SharedState>,
    Path((version, guild_id)): Path<(String, i64)>,
    req_auth: RequestAuth,
) -> Result<Json<GuildRow>, APIError> {
    let api_version = version::parse_version(&version)?;
    tracing::trace!("api version: {}", api_version);

    let user_id = match req_auth.user {
        Some(user) => user.user_id,
        None => return Err(AuthServiceError::InvalidCredentials.into()),
    };

    let guild = GuildService::join(&state, user_id, guild_id).await?;

    Ok(Json(guild))
}

/// `GET /{version}/guilds`
///
/// The open-join browse list. Public by design — being a member is not a
/// precondition for seeing what exists — so the requester is optional and an
/// anonymous caller still gets the list. Every row carries `is_owner`, which is
/// `false` for them. Soft-deleted guilds are excluded by the repository.
pub async fn browse_guilds_handler(
    State(state): State<SharedState>,
    Path(version): Path<String>,
    req_auth: RequestAuth,
    query: Query<BrowseGuildsQuery>,
) -> Result<Json<GuildPage>, APIError> {
    let api_version = version::parse_version(&version)?;
    tracing::trace!("api version: {}", api_version);

    let page = GuildService::browse(
        &state,
        req_auth.user.map(|user| user.user_id),
        query.before,
        query.before_id,
        query.limit.unwrap_or(DEFAULT_LIMIT),
    )
    .await?;

    Ok(Json(page))
}

/// `DELETE /{version}/guilds/{guild_id}`
///
/// Soft-deletes a guild. Owner only: an anonymous caller is 401, a member who
/// is not the owner is 403 `not_guild_owner`, and a missing or already-deleted
/// guild is 404. Soft rather than hard because `guilds` carries a `deleted_at`,
/// so the row and its members survive and the action is recoverable.
///
/// Returns 200 with `{ "deleted": true }` rather than 204, because every other
/// handler here returns `Json<...>` and the guild client parses the success body
/// unconditionally — a 204 would throw a `SyntaxError` in the browser instead of
/// resolving. `NoContent` is the more correct status for a delete with no
/// payload; this trades that for a uniform response contract, the same trade
/// `delete_role_handler` makes.
pub async fn delete_guild_handler(
    State(state): State<SharedState>,
    Path((version, guild_id)): Path<(String, i64)>,
    req_auth: RequestAuth,
) -> Result<Json<serde_json::Value>, APIError> {
    let api_version = version::parse_version(&version)?;
    tracing::trace!("api version: {}", api_version);

    let user_id = match req_auth.user {
        Some(user) => user.user_id,
        None => return Err(AuthServiceError::InvalidCredentials.into()),
    };

    GuildService::delete(&state, user_id, guild_id).await?;

    Ok(Json(serde_json::json!({ "deleted": true })))
}