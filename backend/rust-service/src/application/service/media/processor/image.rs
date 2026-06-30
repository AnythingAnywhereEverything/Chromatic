use libvips::{VipsImage, ops};

use crate::application::service::{errors::MediaServiceError};


pub fn resize_image(
    mut image: VipsImage,
    width: u32,
    height: u32,
    rz_width: u32,
    rz_height: u32,
) -> Result<VipsImage, MediaServiceError> {
    if width > rz_width || height > rz_height {
        let scale = (rz_width as f64 / width as f64)
            .min(rz_height as f64 / height as f64);

        image = ops::resize(&image, scale)?;
    }
    Ok(image)
}

pub fn crop_image_absolute(
    mut image: VipsImage,
    width: u32,
    height: u32,
    cr_width: u32,
    cr_height: u32,
    position: Option<(f32, f32)>,
) -> Result<VipsImage, MediaServiceError> {
    // Calculate the top-left corner of the crop area based on the position parameter
    // patch out of bounds check
    let (left, top) = match position {
        Some((x, y)) => {
            let left = ((width as f32 - cr_width as f32) * x).round() as i32;
            let top = ((height as f32 - cr_height as f32) * y).round() as i32;
            (left.max(0).min((width - cr_width) as i32), top.max(0).min((height - cr_height) as i32))
        }
        None => {
            let left = ((width as f32 - cr_width as f32) / 2.0).round() as i32;
            let top = ((height as f32 - cr_height as f32) / 2.0).round() as i32;
            (left.max(0).min((width - cr_width) as i32), top.max(0).min((height - cr_height) as i32))
        }
    };

    tracing::warn!(
        "Cropping with absolute dimensions: {}x{}, position: ({}, {}), resulting dimensions: {}x{}",
        cr_width, cr_height, left, top, cr_width, cr_height
    );


    
    image = ops::extract_area(&image, left as i32, top as i32, cr_width as i32, cr_height as i32)?;

    Ok(image)
}

pub fn crop_image_normalized(
    image: VipsImage,
    width: u32,
    height: u32,
    norm_width: f32,
    norm_height: f32,
    position: Option<(f32, f32)>,
) -> Result<VipsImage, MediaServiceError> {
    let cr_width = (norm_width * width as f32).round() as u32;
    let cr_height = (norm_height * height as f32).round() as u32;

    crop_image_absolute(image, width, height, cr_width, cr_height, position)
}

pub fn crop_image_ratio(
    image: VipsImage,
    width: u32,
    height: u32,
    ratio: (u32, u32),
    scale: f32,
    position: Option<(f32, f32)>,
) -> Result<VipsImage, MediaServiceError> {
    // example scale : scale = 0.5, ratio = (1, 1), width = 512, height = 512
    // add 0 scale check
    if scale <= 0.0 {
        return Err(MediaServiceError::InvalidScale);
    }

    let (cr_width, cr_height) = match scale {
        scale => {
            let target_width = (width as f32 * scale).round() as u32;
            let target_height = (height as f32 * scale).round() as u32;

            let ratio_width = ratio.0;
            let ratio_height = ratio.1;

            // Calculate the maximum width and height that maintains the aspect ratio
            let max_width = (target_height as f32 * (ratio_width as f32 / ratio_height as f32)).round() as u32;
            let max_height = (target_width as f32 * (ratio_height as f32 / ratio_width as f32)).round() as u32;

            // Determine the final crop dimensions based on the target size and aspect ratio
            if max_width <= target_width {
                (max_width, target_height)
            } else {
                (target_width, max_height)
            }
        }
    };

    tracing::warn!(
        "Cropping with ratio: {:?}, scale: {}, resulting dimensions: {}x{}",
        ratio, scale, cr_width, cr_height
    );

    crop_image_absolute(image, width, height, cr_width, cr_height, position)
}