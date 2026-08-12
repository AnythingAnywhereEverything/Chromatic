use rs_vips::bindings::{VipsInteresting, VipsSize};


#[derive(Clone, Debug)]
pub struct MediaProcessorOptions {
    pub fflags: Option<MediaProcessorFFlags>,
    pub image_processors: Option<Vec<ImageProcessorType>>,
    pub video_processors: Option<Vec<VideoProcessorType>>,
    pub post_processors: Option<PostProcessingType>,
}

impl Default for MediaProcessorOptions {
    fn default() -> Self {
        MediaProcessorOptions {
            fflags: None,
            image_processors: None,
            video_processors: None,
            post_processors: None,
        }
    }
}

#[derive(Clone, Debug)]
pub struct MediaProcessorFFlags {
    pub video_thumbnail: bool,
    pub video_gpu_accel: bool,
    pub video_transcode: bool,
    pub video_post_orig: bool,
    pub image_thumbhash: bool,
}

impl Default for MediaProcessorFFlags {
    fn default() -> Self {
        MediaProcessorFFlags {
            video_thumbnail: true,
            video_gpu_accel: true,
            video_transcode: false,
            video_post_orig: false,
            image_thumbhash: true,
        }
    }
}

#[derive(Clone, Debug)]
pub enum CropStyle {
    Absolute { width: u32, height: u32},
    Normalized { width: f32, height: f32},
    Ratio { width: u32, height: u32, scale: f32 },
}

#[derive(Clone, Debug)]
pub enum ResizeStyle {
    Absolute { width: i32, height: i32 },
    Normalized { width: f32, height: f32 },
}

#[derive(Clone, Debug)]
pub enum ImageProcessorType {
    // Normalized Only,
    // Use thumbnail for absolute position resizing
    Resize {
        style: ResizeStyle,
        upscale: bool,
    },
    Crop {
        style: CropStyle,
        position: Option<(f32, f32)>,
    },
    CreateThumbnail {
        width: i32,
        height: i32,
        no_rotate: bool,
        size: VipsSize,
        crop: VipsInteresting,
    }
}

#[derive(Clone, Debug)]
pub enum VideoProcessorType {
    Trim {
        start_time: f32,
        end_time: f32,
    },
}

#[derive(Clone, Debug)]
pub enum VideoPostProcessorType {
    Transcode {
        // output format, e.g., "mp4", "webm"
        format: String,
        // output codec, e.g., "h264", "vp9"
        codec: String,
        // output preset, e.g., "fast", "slow"
        preset: String,
        // output crf, e.g., 23
        crf: u8,
    },
    HLS {
        segment_time: i32,
    },
}

#[derive(Clone, Debug)]
pub enum ImagePostProcessorType {
    NSFW {
        threshold: f32,
    },
}

#[derive(Clone, Debug)]
pub enum PostProcessingType {
    Video(Vec<VideoPostProcessorType>),
    Image(Vec<ImagePostProcessorType>),
}
// to parse only VideoPostProcessorType from PostProcessingType
impl PostProcessingType {
    pub fn get_video_post_processors(&self) -> Option<&Vec<VideoPostProcessorType>> {
        match self {
            PostProcessingType::Video(processors) => Some(processors),
            _ => None,
        }
    }
    pub fn get_image_post_processors(&self) -> Option<&Vec<ImagePostProcessorType>> {
        match self {
            PostProcessingType::Image(processors) => Some(processors),
            _ => None,
        }
    }
}

