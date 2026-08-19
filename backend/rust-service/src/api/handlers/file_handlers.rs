use crate::{
    api::APIError, application::{
        service::{
            errors::MediaServiceError, media::{
                processor::{image::ImageProcessor, types::{CropStyle, ImageProcessorType, MediaProcessorOptions, ResizeStyle}, video::video::extract_thumbnail}, types::media_options::MediaCategory, utils::{categorize, get_mime_and_extension},
            },
        }, state::SharedState,
    },
};
use axum::extract::{Path, Query, State};
use axum::{body::Body, response::Response};
use hyper::HeaderMap;
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
    _headers: HeaderMap, // for future usage, e.g., for Range Request for large video files that aren't HLS
) -> Result<Response, APIError> {
    tracing::debug!(
        "Received request for file: {}, version: {}, params: {:?}",
        file,
        version,
        params
    );

    if let Some(format) = &params.format {
        check_format(format)?;
    }

    let storage = &state.storage;
    let mut target_file = storage.read(&file).await?;

    let metadata = target_file
        .metadata()
        .await
        .map_err(|_| MediaServiceError::InternalServer)?;

    tracing::debug!("Successfully read file: {} bytes", metadata.len());

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
    let extension = media_type.1;
    let category = categorize(&mime);

    if category == MediaCategory::Image && (params.width.is_some() || params.height.is_some()) {
        let path = state
            .storage
            .full_path(&file)
            .map_err(|_| MediaServiceError::InternalServer)?;

        let image = {
            if mime == "image/gif" || mime == "image/webp" {
                let opts = VOption::new().set("n", -1);
                VipsImage::new_from_file_with_opts(&path, opts)
                    .map_err(|e| MediaServiceError::LibvipsError(e))?
            } else {
                VipsImage::new_from_file(&path).map_err(|e| MediaServiceError::LibvipsError(e))?
            }
        };

        let is_animated = image.get_n_pages() > 1;
        tracing::debug!(
            "Image is animated: {}, width: {}, height: {}",
            is_animated,
            image.get_width(),
            image.get_height()
        );
        
        drop(image);
        
        if is_animated && !params.format.is_some() {
            let opts = VOption::new().set("n", -1);
            let image = VipsImage::new_from_file_with_opts(&path, opts)
                .map_err(|e| MediaServiceError::LibvipsError(e))?;

            let mut processor = ImageProcessor::new();

            let options = MediaProcessorOptions {
                image_processors: Some(vec![
                    ImageProcessorType::Crop {
                        style: CropStyle::Ratio {
                            width: params.width.unwrap_or(image.get_width()) as u32,
                            height: params.height.unwrap_or(image.get_height()) as u32,
                            scale: 1.0,
                        },
                        position: Some((0.5, 0.5))
                    },
                    ImageProcessorType::Resize {
                        style: ResizeStyle::Absolute {
                            width: params.width.unwrap_or(image.get_width()),
                            height: params.height.unwrap_or(image.get_height()),
                        },
                        upscale: false,
                    }
                ]),
                ..Default::default()
            };

            tracing::debug!("Processing image with options: {:?}", options);
    
            let image = ImageProcessor::transform(
                &mut processor,
                image,
                options.image_processors,
                is_animated,
            )?;

            let response = VipsImage::write_to_buffer(&image, &format!(".{}", params.format.as_deref().unwrap_or(&extension)))
                .map_err(MediaServiceError::LibvipsError)?;

            let content_type = match params.format.as_deref().unwrap_or(&extension).to_lowercase().as_str() {
                "jpg" | "jpeg" => "image/jpeg",
                "png" => "image/png",
                "webp" => "image/webp",
                _ => mime.as_str(),
            };

            return Ok(Response::builder()
                // type of the file, e.g., image/jpeg, image/png, etc.
                .header("Content-Type", content_type)
                // length of the file in bytes
                .header("Content-Length", response.len())
                // cache control headers with a long max-age about 1 week and immutable to indicate that the file won't change
                .header("Cache-Control", "public, max-age=604800, immutable")
                .body(Body::from(response))
                .map_err(|_| MediaServiceError::InternalServer)?)
        } else {
            let opts = VOption::new();
            let image = VipsImage::new_from_file_with_opts(&path, opts)
                .map_err(|e| MediaServiceError::LibvipsError(e))?;

            let width = params.width.unwrap_or(image.get_width());
            let height = params.height.unwrap_or(image.get_height());

            let response = make_static_thumbnail(image, width, height, params.format.as_deref().unwrap_or(&extension))?;
            return Ok(response);
        }


    } else if category == MediaCategory::Video
        && (params.width.is_some() || params.height.is_some())
    {
        // get thumbnail for video
        let input_path = state
            .storage
            .full_path(&file)
            .map_err(|_| MediaServiceError::InternalServer)?;
        let video_thumbnail = extract_thumbnail(&input_path.to_string_lossy(), params.format.as_deref().unwrap_or("webp"), 1)
            .await
            .map_err(|_| MediaServiceError::InternalServer)?;

        let image = VipsImage::new_from_buffer(&video_thumbnail, "")
            .map_err(MediaServiceError::LibvipsError)?;

        let width = params.width.unwrap_or(image.get_width());
        let height = params.height.unwrap_or(image.get_height());

        let response = make_static_thumbnail(image, width, height, params.format.as_deref().unwrap_or("webp"))?;

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
