use axum::{
    Router,
    routing::get,
};

use crate::{
    api::handlers::user_handlers::{
        get_user_handler,
    },
    application::state::SharedState,
};

pub fn routes() -> Router<SharedState> {
    Router::new()
        .route("/me", get(get_user_handler))
}
