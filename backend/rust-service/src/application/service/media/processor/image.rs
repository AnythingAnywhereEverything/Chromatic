use libvips::{VipsImage, ops};

use crate::application::service::errors::MediaServiceError;

pub fn resize_image(
    mut image: VipsImage,
    width: u32,
    height: u32,
    rz_width: u32,
    rz_height: u32,
) -> Result<VipsImage, MediaServiceError> {
    if width > rz_width || height > rz_height {
        let scale = (rz_width as f64 / width as f64).min(rz_height as f64 / height as f64);

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

    tracing::warn!(
        "Cropping with\n\t absolute dimensions: {}x{},\n\t position: ({}, {}),\n\t resulting dimensions: {}x{}",
        cr_width,
        cr_height,
        left,
        top,
        cr_width,
        cr_height
    );

    tracing::warn!(
        "Cropping with\n\t original dimensions: {}x{},\n\t position: ({}, {}),\n\t resulting dimensions: {}x{}",
        width,
        height,
        left,
        top,
        cr_width,
        cr_height
    );

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

    tracing::warn!(
        "Cropping with ratio: {:?}, scale: {}, resulting dimensions: {}x{}",
        ratio,
        scale,
        cr_width,
        cr_height
    );

    crop_image_absolute(image, width, height, cr_width, cr_height, position)
}
