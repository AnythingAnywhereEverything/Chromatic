use axum::{
    Router,
    routing::{get, patch},
};

use crate::{
    api::handlers::user_handlers::{
        get_current_user_handler, get_current_user_profile_handler, get_user_profile_handler,
        update_current_user_profile_handler, upload_avatar_handler, upload_banner_handler,
    },
    application::state::SharedState,
};

pub fn routes() -> Router<SharedState> {
    Router::new()
        .route("/me", get(get_current_user_handler))
        .route("/me/avatar", patch(upload_avatar_handler))
        .route("/me/banner", patch(upload_banner_handler))
        .route("/me/profile", get(get_current_user_profile_handler))
        .route("/me/profile", patch(update_current_user_profile_handler))
        .route("/profile/{username}", get(get_user_profile_handler))
}
