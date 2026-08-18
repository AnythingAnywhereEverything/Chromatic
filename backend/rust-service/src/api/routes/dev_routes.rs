use axum::{
    Router, routing::{ get, post},
};

use crate::{
    api::handlers::dev_handlers::{files_upload_handler, get_specific_post}, application::state::SharedState,
};

pub fn routes() -> Router<SharedState> {
    Router::new()
        .route("/upload", post(files_upload_handler))
        .route("/{id}/post", get(get_specific_post))
}
