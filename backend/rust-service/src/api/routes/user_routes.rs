use axum::{
    Router,
    routing::{get, patch},
};

use crate::{
    api::handlers::user_handlers::{
        get_current_user_handler, get_current_user_profile_handler, get_user_minimal_handler, get_user_profile_handler, update_current_user_profile_handler,
    }, application::state::SharedState,
};

pub fn routes() -> Router<SharedState> {
    Router::new()
        .route("/me", get(get_current_user_handler))
        .route("/me/profile", get(get_current_user_profile_handler))
        .route("/me/profile", patch(update_current_user_profile_handler))
        .route("/profile/{username}", get(get_user_profile_handler))

        // for testing purposes, we can get user profile by id, but this should be removed or altered after confirmation.
        .route("/profile/id/{user_id}", get(get_user_minimal_handler))
}
