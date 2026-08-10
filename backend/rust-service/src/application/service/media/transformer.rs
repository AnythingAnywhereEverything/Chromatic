use std::sync::Arc;

use libvips::VipsImage;

use crate::application::service::{
    errors::MediaServiceError, media::{
        processor, storage, types::{CropStyle, OnProcessingType, PostProcessingType, ResizeStyle},
    },
};

pub struct Transformer;

pub struct ImageTransformer;

impl Transformer {
    fn image_resize(
        style: ResizeStyle,
        upscale: bool,
        image: VipsImage,
    ) -> Result<VipsImage, MediaServiceError> {
        let img_width = image.get_width();
        let img_height = image.get_height();

        match style {
            ResizeStyle::AbsoluteKeepsRatio { width: rw, height: rh } => processor::image::image_resize_keep_ratio(
                image,
                img_width as u32,
                img_height as u32,
                rw,
                rh,
                upscale
            ),
            ResizeStyle::AbsoluteWithCrop { width, height } => processor::image::image_resize_absolute(
                image,
                img_width as u32,
                img_height as u32,
                width,
                height,
                upscale
            ),

            ResizeStyle::Normalized { width, height } => processor::image::image_resize_normalized(
                image,
                img_width as u32,
                img_height as u32,
                width,
                height,
                upscale
            ),
        }
    }

    fn image_crop(
        crop_style: CropStyle,
        position: Option<(f32, f32)>,
        image: VipsImage,
    ) -> Result<VipsImage, MediaServiceError> {
        let width = image.get_width();
        let height = image.get_height();

        match crop_style {
            CropStyle::Absolute { width, height } => Ok(processor::image::crop_image_absolute(
                image,
                width as u32,
                height as u32,
                width,
                height,
                position,
            )?),
            CropStyle::Normalized { width, height } => Ok(processor::image::crop_image_normalized(
                image,
                width as u32,
                height as u32,
                width,
                height,
                position,
            )?),
            CropStyle::Ratio {
                ratio: (rw, rh),
                scale,
            } => Ok(processor::image::crop_image_ratio(
                image,
                width as u32,
                height as u32,
                (rw, rh),
                scale,
                position,
            )?),
        }
    }

    async fn video_hls(
        segment_duration: u32,
        job_dir_path: String,
        source_path: String,
        storage: Arc<dyn storage::MediaStorage>,
    ) -> Result<(), MediaServiceError> {
        processor::video::process_video_hls(segment_duration, job_dir_path, source_path, storage).await?;
        Ok(())
    }

    pub fn transform_image(
        image: VipsImage,
        transform: OnProcessingType,
    ) -> Result<VipsImage, MediaServiceError> {
        match transform {
            OnProcessingType::ImageResize { style, upscale } => {
                Self::image_resize(style, upscale, image)
            }
            OnProcessingType::ImageCrop { style, position } => {
                Self::image_crop(style, position, image)
            }
            _ => Ok(image), // If it's a wrong transform type, we can just return the original image without any transformation
        }
    }

    pub async fn transform_video_post(
        job_dir_path: String,
        transform: PostProcessingType,
        source_path: String,
        storage: Arc<dyn storage::MediaStorage>,
    ) -> Result<(), MediaServiceError> {
        match transform {
            PostProcessingType::VideoHls { segment_duration } => {
                Self::video_hls(segment_duration, job_dir_path, source_path, storage).await?;
                Ok(())
            }
            _ => Ok(()), // If it's a wrong or unsupported PostProcessingType, we can just return Ok without any transformation
        }
    }

    // This function is used to transform videos instantly without waiting for a job queue. It will be used for OnProcessingType::VideoTrim.
    pub async fn transform_video_instant(
        transform: OnProcessingType,
        input_path: String,
        file_ext: String,
        storage: Arc<dyn storage::MediaStorage>,
    ) -> Result<(), MediaServiceError> {
        match transform {
            OnProcessingType::VideoTrim { start_time, end_time } => {
                let output_path = format!("{}.trimmed.{}", input_path, file_ext);
                processor::video::process_video_trim(
                    input_path,
                    start_time,
                    end_time,
                    output_path,
                    storage,
                ).await?;
                Ok(())
            }
            _ => Ok(()), // If it's a wrong OnProcessingType, we can just return Ok without any transformation
        }
    }
}
