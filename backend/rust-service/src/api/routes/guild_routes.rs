use axum::{
    Router,
    routing::{delete, get, post},
};

use crate::{
    api::handlers::guild_handler::{
        browse_guilds_handler, create_guild_handler, delete_guild_handler, join_guild_handler,
    },
    application::state::SharedState,
};

pub fn routes() -> Router<SharedState> {
    Router::new()
        .route("/", get(browse_guilds_handler).post(create_guild_handler))
        .route("/{guild_id}", delete(delete_guild_handler))
        .route("/{guild_id}/join", post(join_guild_handler))
}