// * since the tags is in Post folder it might cause some confusion regarding the module structure
// * you might want to consider moving it to its own module in the future

use axum::Json;
use axum::extract::{Path, State};

use crate::api::{APIError, version};
use crate::application::repository::post::row::{TagRow};
use crate::application::repository::tags::{self as tags_repo};
use crate::application::state::SharedState;

// currently the tag attachments row is in post module
pub async fn get_all_tag_attachments(
    State(state): State<SharedState>,
    Path(version): Path<String>,
) -> Result<Json<Vec<TagRow>>, APIError> {
    let api_version = version::parse_version(&version)?;
    tracing::trace!("api version: {}", api_version);

    let mut tx = state.db_pool.begin().await?;
    // oh no
    let tag_attachments = tags_repo::get::get_all_tag_attachments_repo(&mut tx).await?;
    tx.commit().await?;

    Ok(Json(tag_attachments))
}
