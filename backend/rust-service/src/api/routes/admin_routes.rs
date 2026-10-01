use axum::{
    Router,
    routing::{get, patch, post},
};

use crate::{
    api::handlers::admin_handlers::{
        activate_user_handler, get_stats_handler, get_user_detail_handler, list_audit_handler,
        list_users_handler, suspend_user_handler, update_user_role_handler,
    },
    api::routes::staff_role_routes,
    application::state::SharedState,
};

/// Suspend and activate are separate bodyless `POST`s rather than one
/// `PATCH /active` with a boolean, so the two admin buttons cannot be called
pub fn routes() -> Router<SharedState> {
    Router::new()
        .route("/users", get(list_users_handler))
        .route("/users/{user_id}", get(get_user_detail_handler))
        .route("/users/{user_id}/suspend", post(suspend_user_handler))
        .route("/users/{user_id}/activate", post(activate_user_handler))
        .route("/users/{user_id}/role", patch(update_user_role_handler))
        .route("/stats", get(get_stats_handler))
        .route("/audit", get(list_audit_handler))
        .merge(staff_role_routes::routes())
}
