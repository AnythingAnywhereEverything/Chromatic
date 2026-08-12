use axum::{
    Router, routing::{ post},
};

use crate::{
    api::handlers::dev_handlers::files_upload_handler,
    application::state::SharedState,
};

pub fn routes() -> Router<SharedState> {
    Router::new()
        .route("/upload", post(files_upload_handler))
}
