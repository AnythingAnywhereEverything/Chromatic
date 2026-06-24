use axum::{Router, routing::post};

use crate::{
    api::handlers::auth_handlers::{
        delete_handler, login_handler, logout_handler, register_handler
    },
    application::state::SharedState,
};

pub fn routes() -> Router<SharedState> {
    Router::new()
        .route("/login", post(login_handler))
        .route("/logout", post(logout_handler))
        .route("/register", post(register_handler))
        
        .route("/delete", post(delete_handler))
}
