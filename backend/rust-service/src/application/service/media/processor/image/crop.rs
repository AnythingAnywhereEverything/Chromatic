use crate::application::service::errors::MediaServiceError;
use crate::application::service::media::processor::image::{CropStyle, PreCalculatedCrop};

fn compute_normalize(width: f32, height: f32, image_width: u32, image_height: u32) -> (u32, u32) {
    (
        (width * image_width as f32).round() as u32,
        (height * image_height as f32).round() as u32,
    )
}

fn compute_ratio(
    ratio_width: u32,
    ratio_height: u32,
    scale: f32,
    image_width: u32,
    image_height: u32,
) -> Result<(u32, u32), MediaServiceError> {
    if scale <= 0.0 {
        return Err(MediaServiceError::InvalidCropScale(scale));
    }

    // * Calculate the maximum crop dimensions that fit within the source image while maintaining the desired aspect ratio.
    let aspect = ratio_width as f32 / ratio_height as f32;

    let (max_crop_width, max_crop_height) =
        if image_width as f32 / image_height as f32 >= aspect {
            let crop_height = image_height as f32;
            let crop_width = crop_height * aspect;

            (crop_width.round() as u32, crop_height.round() as u32)
        } else {
            let crop_width = image_width as f32;
            let crop_height = crop_width / aspect;

            (crop_width.round() as u32, crop_height.round() as u32)
        };

    Ok((
        (max_crop_width as f32 * scale).round() as u32,
        (max_crop_height as f32 * scale).round() as u32,
    ))
}

pub fn compute_crop_dimensions(
    image_width: u32,
    image_height: u32,
    style: CropStyle,
    position: Option<(f32, f32)>,
) -> Result<PreCalculatedCrop, MediaServiceError> {
    let (crop_width, crop_height) = match style {
        CropStyle::Absolute { width, height } => (width, height),

        CropStyle::Normalized { width, height } => {
            compute_normalize(width, height, image_width, image_height)
        }

        CropStyle::Ratio {
            width: ratio_width,
            height: ratio_height,
            scale,
        } => {
            compute_ratio(
                ratio_width,
                ratio_height,
                scale,
                image_width,
                image_height,
            )?
        }
    };

    // * Keep the crop inside the source image.
    let crop_width = crop_width.min(image_width);
    let crop_height = crop_height.min(image_height);

    let max_left = image_width.saturating_sub(crop_width) as i32;
    let max_top = image_height.saturating_sub(crop_height) as i32;

    let (left, top) = match position {
        Some((x, y)) => {
            // * Position is normalized across the available crop movement range.
            let left = (x.clamp(0.0, 1.0) * max_left as f32).round() as i32;
            let top = (y.clamp(0.0, 1.0) * max_top as f32).round() as i32;

            (left, top)
        }

        None => (max_left / 2, max_top / 2),
    };

    let crop = PreCalculatedCrop {
        left,
        top,
        crop_width,
        crop_height,
    };

    Ok(crop)
}