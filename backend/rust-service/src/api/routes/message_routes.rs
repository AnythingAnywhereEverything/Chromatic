use axum::Router;
use axum::routing::get;
use crate::api::handlers::message_handler::{get_message_chat_handler, get_followed_user_handler};
use crate::application::state::SharedState;

pub fn routes() -> Router<SharedState> {
    Router::new()
    // Define your message routes here, for example:
    // * get chat messages with a specific target user
    .route("/{target_id}", get(get_message_chat_handler))
    .route("/followed", get(get_followed_user_handler))
    // ? elixir impl ? 
}