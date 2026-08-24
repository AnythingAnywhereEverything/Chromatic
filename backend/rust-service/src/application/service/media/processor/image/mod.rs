use rs_vips::{VipsImage, voption::{Setter, VOption}};
use crate::application::service::{errors::media_service::MediaProcessorError, media::{processor::types::{CropStyle, ImageProcessorType, ResizeStyle}}};

pub mod crop;

#[derive(Clone)]
pub struct PreCalculatedCrop {
    crop_width: u32,
    crop_height: u32,
    top: i32,
    left: i32,
}

pub struct ImageProcessor {
    calculated_resize_scale: Option<f32>,
    calculated_crop: Option<PreCalculatedCrop>,
}

impl ImageProcessor {
    pub fn new() -> Self {
        Self {
            calculated_resize_scale: None,
            calculated_crop: None,
        }
    }

    fn process_crop(
        &mut self,
        image: VipsImage,
        style: CropStyle,
        position: Option<(f32, f32)>,
    ) -> Result<VipsImage, MediaProcessorError> {
        // * Cache the final crop rectangle so every GIF frame uses identical geometry.
        let crop = match &self.calculated_crop {
            Some(crop) => crop.clone(),
            None => crop::compute_crop_dimensions(
                image.get_width() as u32,
                image.get_height() as u32,
                style,
                position,
            )?,
        };

        let PreCalculatedCrop { left, top, crop_width, crop_height } = crop;

        Ok(image.crop(
            left,
            top,
            crop_width as i32,
            crop_height as i32,
        )?)
    }

    fn process_resize(
        &mut self,
        image: VipsImage,
        style: ResizeStyle,
        upscale: bool,
    ) -> Result<VipsImage, MediaProcessorError> {
        // * Calculate once so every GIF frame uses the exact same scale.
        let scale = match self.calculated_resize_scale {
            Some(scale) => scale,
            None => {
                let img_width = image.get_width() as f64;
                let img_height = image.get_height() as f64;

                let scale = match style {
                    ResizeStyle::Absolute { width, height } => {
                        (width as f64 / img_width)
                            .min(height as f64 / img_height)
                    }

                    ResizeStyle::Normalized { width, height } => {
                        let target_width = (width * img_width as f32).round() as f64;
                        let target_height = (height * img_height as f32).round() as f64;

                        (target_width / img_width)
                            .min(target_height / img_height)
                    }
                };

                self.calculated_resize_scale = Some(scale as f32);
                scale as f32
            }
        };

        if scale < 1.0 || upscale {
            Ok(VipsImage::resize(&image, scale as f64)?)
        } else {
            Ok(image)
        }
    }

    fn process_static(
        &mut self,
        image: VipsImage,
        types: Vec<ImageProcessorType>,
    ) -> Result<VipsImage, MediaProcessorError> {
        let mut image = image;

        for processor_type in types {
            match processor_type {
                ImageProcessorType::Resize { style, upscale } => {
                    image = self.process_resize(image, style, upscale)?;
                }

                ImageProcessorType::Crop { style, position } => {
                    image = self.process_crop(image, style, position)?;
                }

                ImageProcessorType::CreateThumbnail { width, height, no_rotate, size, crop } => {
                    let mut options = VOption::new();
                    options.add("height", height as i32);
                    options.add("no_rotate", no_rotate);
                    options.add("crop", crop as i32);
                    options.add("size", size as i32);

                    image = VipsImage::thumbnail_image_with_opts(&image, width as i32, options)?;
                }
            }
        }

        Ok(image)
    }

    fn process_animated(
        &mut self,
        image: VipsImage,
        types: Vec<ImageProcessorType>,
    ) -> Result<VipsImage, MediaProcessorError> {
        let mut image = image;
        let mut frames = Vec::new();

        let n_pages = image.get_n_pages();
        let page_height = image.get_page_height();

        for page in 0..n_pages {
            let frame = VipsImage::extract_area(
                &image,
                0,
                page * page_height,
                image.get_width(),
                page_height,
            )?;

            let frame = self.process_static(frame, types.clone())?;

            frames.push(frame);
        }

        let frame_height = frames[0].get_height();


        let options = VOption::new()
            .set("across", 1)
            .set("hspacing", frames[0].get_width())
            .set("vspacing", frame_height);

        image = VipsImage::arrayjoin_with_opts(&mut frames, options)?;

        image.set_int("n-pages", n_pages as i32)?;
        image.set_int("page-height", frame_height)?;

        Ok(image)
    }

    pub fn transform(
        &mut self,
        image: VipsImage,
        types: Option<Vec<ImageProcessorType>>,
        animated: bool,
    ) -> Result<VipsImage, MediaProcessorError> {

        let types = match types {
            Some(types) => types,
            None => return Ok(image),
        };

        if animated {
            Ok(self.process_animated(image, types)?)
        } else {
            Ok(self.process_static(image, types)?)
        }
    }
}