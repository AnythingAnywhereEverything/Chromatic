use libvips::{VipsImage, ops};

use crate::application::service::errors::MediaServiceError;

/// Resizes the image to fit within the specified width and height while maintaining the aspect ratio.
pub fn image_resize_keep_ratio(
    mut image: VipsImage,
    width: u32,
    height: u32,
    rz_width: u32,
    rz_height: u32,
    upscale: bool,
) -> Result<VipsImage, MediaServiceError> {
    let scale = (rz_width as f64 / width as f64).min(rz_height as f64 / height as f64);

    if scale < 1.0 || upscale {
        image = ops::resize(&image, scale)?;
    }
    Ok(image)
}

pub fn image_resize_normalized(
    mut image: VipsImage,
    width: u32,
    height: u32,
    rz_width: f32,
    rz_height: f32,
    upscale: bool,
) -> Result<VipsImage, MediaServiceError> {
    let target_width = (rz_width * width as f32).round() as u32;
    let target_height = (rz_height * height as f32).round() as u32;

    let scale = (target_width as f64 / width as f64).min(target_height as f64 / height as f64);
    
    if scale < 1.0 || upscale {
        image = ops::resize(&image, scale)?;
    }

    Ok(image)
}

/// if the image is 512x512
/// and the target is 215x100
/// the image will first decide a cropping ratio of 215:100
/// then it will crop the image to 215x100
/// and resize it to 215x100
pub fn image_resize_absolute(
    mut image: VipsImage,
    width: u32,
    height: u32,
    rz_width: u32,
    rz_height: u32,
    upscale: bool,
) -> Result<VipsImage, MediaServiceError> {
    let target_aspect = rz_width as f64 / rz_height as f64;
    let current_aspect = width as f64 / height as f64;

    let (crop_width, crop_height) = if (current_aspect - target_aspect).abs() > f64::EPSILON {
        if current_aspect > target_aspect {
            let crop_width = (height as f64 * target_aspect).round() as u32;
            (crop_width, height)
        } else {
            let crop_height = (width as f64 / target_aspect).round() as u32;
            (width, crop_height)
        }
    } else {
        (width, height)
    };

    if (current_aspect - target_aspect).abs() > f64::EPSILON {
        let left = ((width - crop_width) / 2) as i32;
        let top = ((height - crop_height) / 2) as i32;

        image = ops::extract_area(&image, left, top, crop_width as i32, crop_height as i32)?;
    }

    // Finally, resize the image to the target dimensions
    let scale = (rz_width as f64 / crop_width as f64).min(rz_height as f64 / crop_height as f64);
    if scale < 1.0 || upscale {
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
    let max_left = (width - cr_width) as i32;
    let max_top = (height - cr_height) as i32;

    let (left, top) = match position {
        Some((x, y)) => {
            let left = (x * width as f32).round() as i32;
            let top = (y * height as f32).round() as i32;

            (left.clamp(0, max_left), top.clamp(0, max_top))
        }
        None => (max_left / 2, max_top / 2),
    };

    image = ops::extract_area(
        &image,
        left as i32,
        top as i32,
        cr_width as i32,
        cr_height as i32,
    )?;

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

    let aspect = ratio.0 as f32 / ratio.1 as f32;

    let (max_crop_width, max_crop_height) = if width as f32 / height as f32 >= aspect {
        let h = height as f32;
        let w = h * aspect;
        (w.round() as u32, h.round() as u32)
    } else {
        let w = width as f32;
        let h = w / aspect;
        (w.round() as u32, h.round() as u32)
    };

    let cr_width = (max_crop_width as f32 * scale).round() as u32;
    let cr_height = (max_crop_height as f32 * scale).round() as u32;

    crop_image_absolute(image, width, height, cr_width, cr_height, position)
}
