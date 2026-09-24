use axum::{
    Router, routing::{delete, get, patch, post},
};

use crate::{
    api::handlers::user_handlers::{
        follow_user_accept_handler, follow_user_handler, follow_user_reject_handler,
        get_current_user_handler, get_current_user_profile_handler,
        get_pending_follow_requests_handler, get_user_minimal_handler, get_user_profile_handler,
        get_user_setting_handler, unfollow_user_handler, update_current_user_profile_handler,
        update_user_setting_handler,
    }, application::state::SharedState,
};

pub fn routes() -> Router<SharedState> {
    Router::new()
        .route("/me", get(get_current_user_handler))
        .route("/me/profile", get(get_current_user_profile_handler))
        .route("/me/profile", patch(update_current_user_profile_handler))
        .route("/profile/{username}", get(get_user_profile_handler))
        .route("/follow/{target_id}", post(follow_user_handler))
        .route("/follow/requests", get(get_pending_follow_requests_handler))
        .route("/follow/{follower_id}/accept", post(follow_user_accept_handler))
        .route("/follow/{follower_id}/reject", delete(follow_user_reject_handler))
        .route("/unfollow/{target_id}", delete(unfollow_user_handler))
        .route("/settings/{setting_type}", get(get_user_setting_handler))
        .route("/settings/{setting_type}", patch(update_user_setting_handler))
        // for testing purposes, we can get user profile by id, but this should be removed or altered after confirmation.
        .route("/profile/id/{user_id}", get(get_user_minimal_handler))
}
