use axum::{Router, routing::{post, put}};

use crate::{
    api::handlers::{
        comment_handler::*,
        post_handler::{create_new_post_handler, update_post_handler},
    },
    application::state::SharedState,
};

pub fn routes() -> Router<SharedState> {
    Router::new()
        .route("/new", post(create_new_post_handler))
        .route("/{id}/update", put(update_post_handler))
        .route("/{id}/comments/new", post(create_new_comment_handler))
        .route("/{id}/comments/update", put(update_comment_handler))
}