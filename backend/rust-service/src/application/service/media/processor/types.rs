use rs_vips::bindings::{VipsInteresting, VipsSize};


#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
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

impl MediaProcessorOptions {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn get_fflags(&self) -> Option<&MediaProcessorFFlags> {
        self.fflags.as_ref()
    }

    pub fn get_image_processors(&self) -> Option<&Vec<ImageProcessorType>> {
        self.image_processors.as_ref()
    }

    pub fn get_video_processors(&self) -> Option<&Vec<VideoProcessorType>> {
        self.video_processors.as_ref()
    }

    pub fn get_post_processors(&self) -> Option<&PostProcessingType> {
        self.post_processors.as_ref()
    }

    pub fn set_image_processors(mut self, processors: Vec<ImageProcessorType>) -> Self {
        self.image_processors = Some(processors);
        self
    }

    pub fn set_video_processors(mut self, processors: Vec<VideoProcessorType>) -> Self {
        self.video_processors = Some(processors);
        self
    }

    pub fn set_post_video_processors(mut self, processors: Vec<VideoPostProcessorType>) -> Self {
        self.post_processors = Some(PostProcessingType::Video(processors));
        self
    }

    pub fn get_post_video_processors(&self) -> Option<&Vec<VideoPostProcessorType>> {
        match &self.post_processors {
            Some(PostProcessingType::Video(processors)) => Some(processors),
            _ => None,
        }
    }

    pub fn set_post_image_processors(mut self, processors: Vec<ImagePostProcessorType>) -> Self {
        self.post_processors = Some(PostProcessingType::Image(processors));
        self
    }

    pub fn get_post_image_processors(&self) -> Option<&Vec<ImagePostProcessorType>> {
        match &self.post_processors {
            Some(PostProcessingType::Image(processors)) => Some(processors),
            _ => None,
        }
    }

    // * FastFlags setters

    pub fn set_fflags_video_transcode(mut self, enabled: bool) -> Self {
        if let Some(fflags) = &mut self.fflags {
            fflags.video_transcode = enabled;
        } else {
            self.fflags = Some(MediaProcessorFFlags {
                video_transcode: enabled,
                ..Default::default()
            });
        }
        self
    }

    pub fn set_fflags_video_post_orig(mut self, enabled: bool) -> Self {
        if let Some(fflags) = &mut self.fflags {
            fflags.video_post_orig = enabled;
        } else {
            self.fflags = Some(MediaProcessorFFlags {
                video_post_orig: enabled,
                ..Default::default()
            });
        }
        self
    }

    pub fn set_fflags_video_thumbnail(mut self, enabled: bool) -> Self {
        if let Some(fflags) = &mut self.fflags {
            fflags.video_thumbnail = enabled;
        } else {
            self.fflags = Some(MediaProcessorFFlags {
                video_thumbnail: enabled,
                ..Default::default()
            });
        }
        self
    }

    pub fn set_fflags_video_gpu_accel(mut self, enabled: bool) -> Self {
        if let Some(fflags) = &mut self.fflags {
            fflags.video_gpu_accel = enabled;
        } else {
            self.fflags = Some(MediaProcessorFFlags {
                video_gpu_accel: enabled,
                ..Default::default()
            });
        }
        self
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MediaProcessorFFlags {
    pub video_thumbnail: bool,
    pub video_gpu_accel: bool,
    pub video_transcode: bool,
    pub video_post_orig: bool,
}

impl Default for MediaProcessorFFlags {
    fn default() -> Self {
        MediaProcessorFFlags {
            video_thumbnail: true,
            video_gpu_accel: true,
            video_transcode: false,
            video_post_orig: false,
        }
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub enum CropStyle {
    Absolute { width: u32, height: u32},
    Normalized { width: f32, height: f32},
    Ratio { width: u32, height: u32, scale: f32 },
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub enum ResizeStyle {
    Absolute { width: i32, height: i32 },
    Normalized { width: f32, height: f32 },
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
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

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub enum VideoProcessorType {
    Trim {
        start_time: f32,
        end_time: f32,
    },
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq)]
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

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub enum ImagePostProcessorType {
    NSFW {
        threshold: f32,
    },
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
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

