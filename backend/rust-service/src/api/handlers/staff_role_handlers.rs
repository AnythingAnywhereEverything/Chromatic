use axum::{
    Json,
    extract::{Path, Query, State},
};
use serde::Deserialize;

use crate::{
    api::{version, AdminUser, APIError},
    application::{
        repository::staff_role::row::{
            StaffRoleDetail, StaffRoleMembersPage, StaffRolePage, StaffRoleRow,
        },
        service::staff_role::{service::StaffRoleService, types::UserStaffRoles},
        state::SharedState,
    },
};
const DEFAULT_LIMIT: i64 = 50;
const DEFAULT_MEMBER_LIMIT: i64 = 25;
const DEFAULT_POSITION: i32 = 0;

#[derive(Deserialize, Debug)]
pub struct StaffRoleListQuery {
    pub q: Option<String>,
    pub before: Option<i32>,
    pub before_id: Option<i64>,
    pub limit: Option<i64>,
}

/// `?before_user_id=&limit=` for the member list of one role.
#[derive(Deserialize, Debug)]
pub struct StaffRoleMembersQuery {
    pub before_user_id: Option<i64>,
    pub limit: Option<i64>,
}

#[derive(Deserialize, Debug)]
pub struct CreateStaffRolePayload {
    pub name: String,
    pub description: Option<String>,
    pub position: Option<i32>,
    pub permission_bitmask: Vec<i64>,
}

#[derive(Deserialize, Debug)]
pub struct UpdateStaffRolePayload {
    pub name: Option<String>,
    pub description: Option<String>,
    pub position: Option<i32>,
    pub permission_bitmask: Option<Vec<i64>>,
}

#[derive(Deserialize, Debug)]
pub struct SetUserStaffRolesPayload {
    pub role_ids: Vec<i64>,
}

pub async fn list_roles_handler(
    State(state): State<SharedState>,
    Path(version): Path<String>,
    _admin: AdminUser,
    query: Query<StaffRoleListQuery>,
) -> Result<Json<StaffRolePage>, APIError> {
    let api_version = version::parse_version(&version)?;
    tracing::trace!("api version: {}", api_version);

    let page = StaffRoleService::list_roles(
        &state,
        query.q.as_deref(),
        query.before,
        query.before_id,
        query.limit.unwrap_or(DEFAULT_LIMIT),
    )
    .await?;

    Ok(Json(page))
}

pub async fn get_role_handler(
    State(state): State<SharedState>,
    Path((version, role_id)): Path<(String, i64)>,
    _admin: AdminUser,
) -> Result<Json<StaffRoleDetail>, APIError> {
    let api_version = version::parse_version(&version)?;
    tracing::trace!("api version: {}", api_version);

    let detail = StaffRoleService::get_role(&state, role_id).await?;

    Ok(Json(detail))
}

pub async fn list_role_members_handler(
    State(state): State<SharedState>,
    Path((version, role_id)): Path<(String, i64)>,
    _admin: AdminUser,
    query: Query<StaffRoleMembersQuery>,
) -> Result<Json<StaffRoleMembersPage>, APIError> {
    let api_version = version::parse_version(&version)?;
    tracing::trace!("api version: {}", api_version);

    let page = StaffRoleService::list_role_members(
        &state,
        role_id,
        query.before_user_id,
        query.limit.unwrap_or(DEFAULT_MEMBER_LIMIT),
    )
    .await?;

    Ok(Json(page))
}

pub async fn create_role_handler(
    State(state): State<SharedState>,
    Path(version): Path<String>,
    admin: AdminUser,
    Json(payload): Json<CreateStaffRolePayload>,
) -> Result<Json<StaffRoleRow>, APIError> {
    let api_version = version::parse_version(&version)?;
    tracing::trace!("api version: {}", api_version);

    let role = StaffRoleService::create_role(
        &state,
        admin.user_id,
        &payload.name,
        payload.description.as_deref(),
        payload.position.unwrap_or(DEFAULT_POSITION),
        &payload.permission_bitmask,
    )
    .await?;

    Ok(Json(role))
}

pub async fn update_role_handler(
    State(state): State<SharedState>,
    Path((version, role_id)): Path<(String, i64)>,
    admin: AdminUser,
    Json(payload): Json<UpdateStaffRolePayload>,
) -> Result<Json<StaffRoleRow>, APIError> {
    let api_version = version::parse_version(&version)?;
    tracing::trace!("api version: {}", api_version);

    let role = StaffRoleService::update_role(
        &state,
        admin.user_id,
        role_id,
        payload.name.as_deref(),
        payload.description.as_deref(),
        payload.position,
        payload.permission_bitmask.as_deref(),
    )
    .await?;

    Ok(Json(role))
}

pub async fn delete_role_handler(
    State(state): State<SharedState>,
    Path((version, role_id)): Path<(String, i64)>,
    admin: AdminUser,
) -> Result<Json<serde_json::Value>, APIError> {
    let api_version = version::parse_version(&version)?;
    tracing::trace!("api version: {}", api_version);

    StaffRoleService::delete_role(&state, admin.user_id, role_id).await?;

    Ok(Json(serde_json::json!({ "deleted": true })))
}

/// `GET /{version}/admin/users/{user_id}/staff-roles`
pub async fn get_user_staff_roles_handler(
    State(state): State<SharedState>,
    Path((version, user_id)): Path<(String, i64)>,
    _admin: AdminUser,
) -> Result<Json<UserStaffRoles>, APIError> {
    let api_version = version::parse_version(&version)?;
    tracing::trace!("api version: {}", api_version);

    let roles = StaffRoleService::get_user_staff_roles(&state, user_id).await?;

    Ok(Json(roles))
}

pub async fn set_user_staff_roles_handler(
    State(state): State<SharedState>,
    Path((version, user_id)): Path<(String, i64)>,
    admin: AdminUser,
    Json(payload): Json<SetUserStaffRolesPayload>,
) -> Result<Json<UserStaffRoles>, APIError> {
    let api_version = version::parse_version(&version)?;
    tracing::trace!("api version: {}", api_version);

    let roles = StaffRoleService::set_user_staff_roles(&state, admin.user_id, user_id, &payload.role_ids)
        .await?;

    Ok(Json(roles))
}