use crate::{api::handlers::tags_handler::get_all_tag_attachments, application::state::SharedState};
use axum::{Router, routing::get};

pub fn routes() -> Router<SharedState> {
    Router::new()
    .route("/all", get(get_all_tag_attachments))
}