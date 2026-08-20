use axum::{Router, routing::{delete, get, post, put}};

use crate::{
    api::handlers::{
        comment_handler::*, post_handler::{create_new_post_handler, delete_post_handler, get_feed_post_handler, liked_handler, update_post_handler},
    }, application::state::SharedState,
};

pub fn routes() -> Router<SharedState> {
    Router::new()
    // * first page will send NULL
    .route("/feed", get(get_feed_post_handler))
        
        .route("/new", post(create_new_post_handler))
        
        .route("/{id}/update", put(update_post_handler))
        .route("/{id}/delete", delete(delete_post_handler))

        .route("/{id}/like", post(liked_handler))
        
        .route("/{id}/comments", get(get_comment_handler))
        .route("/{id}/comments/new", post(create_new_comment_handler))
        .route("/{id}/comments/update", put(update_comment_handler))
        .route("/{id}/comments/delete", delete(delete_comment_handler))
}