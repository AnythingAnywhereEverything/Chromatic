use axum::{Json, extract::{Multipart, Path, State}};
use multipart_derive::Multipart;
use serde::Serialize;

use crate::{api::{APIError, dtos::post_dtos::PostDTO, version}, application::{service::media::{multipart_ex::MultipartLimits, types::TempUpload}, state::SharedState}};

#[derive(serde::Deserialize, Debug,  Multipart)]

pub struct CreatePostRequest {
    pub content: String,
    #[multipart]
    pub multipart: Option<Vec<TempUpload>>,
    pub repost_from: Option<String>,
}

pub async fn create_new_post_handler(
    State(state): State<SharedState>,
    Path(version): Path<String>,
    multipart: Multipart
) -> Result<(), APIError> {
    let api_version = version::parse_version(&version)?;
    tracing::trace!("api version: {}", api_version);
    
    let limits = MultipartLimits{
        max_file_size:512_000_000,
        max_files: 5
    };
    let extracted = state.multipart_extractor.extract::<CreatePostRequest>(multipart, limits).await?;
    tracing::debug!("Extracted payload: {:?}", extracted);
    
    Ok(())
}