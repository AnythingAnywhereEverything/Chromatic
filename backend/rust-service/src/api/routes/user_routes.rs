use axum::{
    Router, routing::{get, patch},
};

use crate::{
    api::handlers::user_handlers::{
        get_current_user_handler, upload_avatar_handler, get_user_profile_handler,
    }, application::state::SharedState,
};

pub fn routes() -> Router<SharedState> {
    Router::new()
        .route("/me", get(get_current_user_handler))
        .route("/me/avatar", patch(upload_avatar_handler))
        .route("/profile/{username}", get(get_user_profile_handler))
}
