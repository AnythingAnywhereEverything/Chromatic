use crate::api::handlers::message_handler::{
    get_followed_user_handler, get_message_chat_handler, get_single_message_handler,
};
use crate::application::state::SharedState;
use axum::Router;
use axum::routing::get;

pub fn routes() -> Router<SharedState> {
    Router::new()
        // Define your message routes here, for example:
        // * get chat messages with a specific target user
        .route("/channel/{target_id}", get(get_message_chat_handler))
        .route("/followed", get(get_followed_user_handler))
        .route("/{message_id}", get(get_single_message_handler))
    // ? elixir impl ?
}
