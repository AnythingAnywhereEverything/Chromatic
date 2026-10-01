use axum::{
    Json, body::Body, extract::{Path, Query, State}, response::Response,
};
use serde::Deserialize;

use crate::{
    api::{
        APIError, RequestAuth,
        dtos::opengraph_dtos::{OpenGraphRequest, OpenGraphResponse},
        version,
    },
    application::{
        service::{errors::OpenGraphServiceError, opengraph_service::OpenGraphService},
        state::SharedState,
    },
};

#[derive(Debug, Deserialize)]
pub struct OpenGraphImageParameters {
    pub url: String,
}

pub async fn get_opengraph_image_handler(
    Query(params): Query<OpenGraphImageParameters>,
) -> Result<Response, APIError> {
    let response = OpenGraphService::fetch_image(&params.url).await?;

    let content_type = response
        .headers()
        .get(reqwest::header::CONTENT_TYPE)
        .and_then(|value| value.to_str().ok())
        .unwrap_or("application/octet-stream")
        .to_string();

    let bytes = response
        .bytes()
        .await
        .map_err(|_| OpenGraphServiceError::ResponseReadFailed)?;

    Response::builder()
        .header("Content-Type", content_type)
        .header("Content-Length", bytes.len())
        .header("Cache-Control", "public, max-age=604800")
        .header("Cross-Origin-Resource-Policy", "cross-origin")
        .body(Body::from(bytes))
        .map_err(|_| OpenGraphServiceError::InternalServer.into())
}

pub async fn opengraph_handler(
    State(_state): State<SharedState>,
    Path(version): Path<String>,
    req_header: RequestAuth,
    Json(payload): Json<OpenGraphRequest>,
) -> Result<Json<OpenGraphResponse>, APIError> {
    let api_version = version::parse_version(&version)?;

    tracing::trace!("api version: {}", api_version);
    tracing::trace!("opengraph request: {:#?}", payload);
    tracing::trace!("request header: {:#?}", req_header);

    let response = OpenGraphService::fetch(&payload.url).await?;

    Ok(Json(response))
}
