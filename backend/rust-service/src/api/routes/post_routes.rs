use axum::{Router, routing::post};

use crate::{api::handlers::post_handler::create_new_post_handler, application::state::SharedState};

pub fn routes() -> Router<SharedState> {
    Router::new()
        .route("/new", post(create_new_post_handler))
}