use crate::{
    api::APIError, application::{
        service::{
            errors::MediaServiceError, media::{
                processor::video::video::extract_thumbnail, types::media_options::MediaCategory, utils::{categorize, get_mime_and_extension},
            },
        }, state::SharedState,
    },
};
use axum::extract::{Path, Query, State};
use axum::{body::Body, response::Response};
use rs_vips::{
    VipsImage,
    enums::{Interesting, Size},
    voption::{Setter, VOption},
};
use serde::Deserialize;
use tokio::io::AsyncReadExt;
use tokio_util::io::ReaderStream;

#[derive(Deserialize, Debug)]
pub struct FileParameters {
    height: Option<i32>,
    width: Option<i32>,
    format: Option<String>,
}

fn check_format(format: &str) -> Result<bool, MediaServiceError> {
    match format.to_lowercase().as_str() {
        "jpeg" | "jpg" | "png" | "webp" => Ok(true),
        _ => Err(MediaServiceError::UnsupportedFormat(format.to_string())),
    }
}





#[axum::debug_handler]
pub async fn get_files_handler(
    State(state): State<SharedState>,
    Path((version, file)): Path<(String, String)>,
    Query(params): Query<FileParameters>,
) -> Result<Response, APIError> {
    tracing::debug!(
        "Received request for file: {}, version: {}, params: {:?}",
        file,
        version,
        params
    );



    let format = params.format.as_deref().unwrap_or("webp");

    check_format(format)?;

    let storage = &state.storage;
    let mut target_file = storage.read(&file).await?;

    let metadata = target_file
        .metadata()
        .await
        .map_err(|_| MediaServiceError::InternalServer)?;

    tracing::debug!(
        "Successfully read file: {} bytes",
        metadata.len()
    );

    let detect_byte = {
        let mut buf = [0u8; 512];

        let n = target_file
            .read(&mut buf)
            .await
            .map_err(|_| MediaServiceError::InternalServer)?;

        buf[..n].to_vec()
    };

    drop(target_file); // unused

    let media_type = get_mime_and_extension(&detect_byte)?;
    let mime = media_type.0;
    let category = categorize(&mime);

    if category == MediaCategory::Image
        && (params.width.is_some() || params.height.is_some())
    {
        let path = state
            .storage
            .full_path(&file)
            .map_err(|_| MediaServiceError::InternalServer)?;

        let image = VipsImage::new_from_file(&path)
            .map_err(MediaServiceError::LibvipsError)?;

        let width = params.width.unwrap_or(image.get_width());
        let height = params.height.unwrap_or(image.get_height());

        let response = make_static_thumbnail(image, width, height, format)?;

        return Ok(response);

    } else if category == MediaCategory::Video && (params.width.is_some() || params.height.is_some()) {
        // get thumbnail for video
        let input_path = state
            .storage
            .full_path(&file)
            .map_err(|_| MediaServiceError::InternalServer)?;
        let video_thumbnail = extract_thumbnail(&input_path.to_string_lossy(), format, 1).await.map_err(|_| MediaServiceError::InternalServer)?;

        let image = VipsImage::new_from_buffer(&video_thumbnail, "")
            .map_err(MediaServiceError::LibvipsError)?;

        let width = params.width.unwrap_or(image.get_width());
        let height = params.height.unwrap_or(image.get_height());

        let response = make_static_thumbnail(image, width, height, format)?;

        return Ok(response);
    }

    let original_file = storage.read(&file).await?;
    let stream = ReaderStream::new(original_file);

    Ok(Response::builder()
        .header("Content-Type", mime.as_str())
        .header("Content-Length", metadata.len())
        .header("Cache-Control", "public, max-age=604800, immutable")
        .body(Body::from_stream(stream))
        .map_err(|_| MediaServiceError::InternalServer)?)
}

fn make_static_thumbnail(
    image: VipsImage,
    width: i32,
    height: i32,
    format: &str,
) -> Result<Response<Body>, MediaServiceError> {

    let thumbnail = image
        .thumbnail_image_with_opts(
            width,
            VOption::new()
                .set("height", height)
                .set("crop", Interesting::Centre as i32)
                .set("size", Size::Down as i32),
        )
        .map_err(MediaServiceError::LibvipsError)?;

    let format_byte = thumbnail
        .write_to_buffer(&format!(".{}", format))
        .map_err(MediaServiceError::LibvipsError)?;

    let content_type = match format.to_lowercase().as_str() {
        "jpg" | "jpeg" => "image/jpeg",
        "png" => "image/png",
        "webp" => "image/webp",
        _ => unreachable!(),
    };

    Ok(Response::builder()
        // type of the file, e.g., image/jpeg, image/png, etc.
        .header("Content-Type", content_type)
        // length of the file in bytes
        .header("Content-Length", format_byte.len())
        // cache control headers with a long max-age about 1 week and immutable to indicate that the file won't change
        .header("Cache-Control", "public, max-age=604800, immutable")
        .body(Body::from(format_byte))
        .map_err(|_| MediaServiceError::InternalServer)?)
}