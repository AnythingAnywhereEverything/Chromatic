use axum::{Router, routing::{delete, get, post, put}};

use crate::{
    api::handlers::{
        comment_handler::*, post_handler::{create_new_post_handler, delete_post_handler, get_feed_post_handler, get_info_post_handler, get_user_posts_handler, post_liked_handler, update_post_handler},
    }, application::state::SharedState,
};

pub fn routes() -> Router<SharedState> {
    Router::new()
    // * first page will send NULL
    .route("/feed", get(get_feed_post_handler))

    .route("/new", post(create_new_post_handler))
    
    .route("/{id}", get(get_info_post_handler))
    .route("/{id}", put(update_post_handler))
    .route("/{id}", delete(delete_post_handler))
    .route("/{id}/like", post(post_liked_handler))
    
    .route("/{id}/comments", get(get_comment_handler))
    .route("/{id}/comments", post(create_new_comment_handler))
    .route("/{id}/comments/{comment_id}", put(update_comment_handler))
    .route("/{id}/comments/{comment_id}", delete(delete_comment_handler))
    .route("/{id}/comments/{comment_id}/like", post(liked_comment_handler))

    .route("/user/{target_id}", get(get_user_posts_handler))
}