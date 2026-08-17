use axum::{
    Router, routing::{get},
};

use crate::{
    api::handlers::file_handlers::get_files_handler,
    application::state::SharedState,
};

pub fn routes() -> Router<SharedState> {
    Router::new()
        .route("/{*file}", get(get_files_handler))
}
