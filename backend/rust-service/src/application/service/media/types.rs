use std::path::PathBuf;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy)]
pub enum CropStyle {
    /// Freeform cropping based on exact width and height dimensions
    Absolute { width: u32, height: u32 },

    /// Normalized cropping based on a percentage of the original image dimensions (e.g., 0.5 for 50% of the original size)
    Normalized { width: f32, height: f32 },

    /// Proportion-locked cropping using a ratio and a defining dimension (e.g., width)
    Ratio { ratio: (u32, u32), scale: f32 },
}

#[derive(Debug, Clone, Copy)]
pub enum ResizeStyle {
    /// Resize to exact width and height dimensions
    /// will be cropped if the aspect ratio is different from the original image
    AbsoluteWithCrop { width: u32, height: u32 },

    /// Resize based on a percentage of the original image dimensions (e.g., 0.5 for 50% of the original size)
    Normalized { width: f32, height: f32 },

    /// Proportion-locked resizing using a ratio and a defining dimension (e.g., width)
    AbsoluteKeepsRatio { width: u32, height: u32 },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub enum MediaCategory {
    Image,
    Video,
    Audio,
    Document,
    Code,
    Archive,
    Unknown,
}



#[derive(Debug, Clone)]
pub enum ValidationType {
    Whitelisted,
    Blacklisted,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MediaType {
    // Blacklist & Whitelist
    GenericJpeg,
    GenericPng,
    GenericWebP,
    GenericMp4,
    GenericGif,

    // Categorized
    Image,
    Video,
    Audio,
}

#[derive(Debug, Clone)]
pub struct ProcessingOptions {
    pub use_video_transcoding: bool,
    pub use_generated_id_as_container: bool,
    pub use_gpu_acceleration: bool,
    pub use_raw_name: bool,
    pub use_raw_name_with_extension: bool,
    // this will add a_ at the first of the file name, to indicate that this is an animated image, and will be used for the thumbnail generation
    pub use_animated_image_indicator: bool,
    pub use_hash_as_name: bool,
    pub use_thumbhash_generation: bool,
    pub locked: bool,
}

impl Default for ProcessingOptions {
    fn default() -> Self {
        ProcessingOptions {
            use_video_transcoding: false,
            use_generated_id_as_container: false,
            use_gpu_acceleration: false,
            use_raw_name: false,
            use_raw_name_with_extension: false,
            use_animated_image_indicator: false,
            use_hash_as_name: false,
            use_thumbhash_generation: true,
            locked: false,
        }
    }
}

#[derive(Debug, Clone)]
pub enum OnProcessingType {
    ImageResize {
        style: ResizeStyle,
        upscale: bool,
    },
    ImageCrop {
        style: CropStyle,
        position: Option<(f32, f32)>,
    },

    // Video transformations
    // * Not implemented yet
    VideoTrim {
        start_time: f32,
        end_time: f32,
    },
}
#[derive(Debug, Clone)]
pub enum PostProcessingType {
    VideoResize {
        width: u32,
        height: u32,
    },
    VideoCrop {
        style: CropStyle,
        position: Option<(f32, f32)>,
    },
    VideoHls {
        segment_duration: u32,
    },
    NSFWImageDetection {
        onyx_model_path: String,
    },
}
#[derive(Debug, Clone)]
pub struct MediaProcessing {
    pub options: ProcessingOptions,
    pub on_processing: Option<Vec<OnProcessingType>>,
    pub post_processing: Option<Vec<PostProcessingType>>,
}

impl Default for MediaProcessing {
    fn default() -> Self {
        MediaProcessing {
            options: ProcessingOptions::default(),
            on_processing: None,
            post_processing: None,
        }
    }
}

#[derive(Debug, Clone)]
pub struct ValidationOptions {
    pub validation_type: ValidationType,
    pub value: Vec<MediaType>,
}

#[derive(Debug, Clone)]
pub struct FileSizeGate {
    pub media_type: MediaType,
    pub max_size: usize,
}

#[derive(Debug, Clone)]
pub struct MediaOptions {
    // Identify the destination folder for media files
    pub folder: String,

    pub size_gate: Option<Vec<FileSizeGate>>,

    // Optional validation settings to enforce specific media types or categories
    pub validation: Option<ValidationOptions>,

    // Optional processing order for media transformations
    pub processing_order: MediaProcessing,
}
#[derive(Debug, Clone, Serialize)]
pub struct ProcessedMedia {
    // The unique identifier for the processed media file
    pub file_id: i64,
    // The relative path where the processed media file is stored
    pub path: String,
    // The name of the processed media file
    pub file_name: String,
    // The category of the processed media file (e.g., image, video, audio)
    pub category: MediaCategory,
    // Thumbhash for the processed media file, if applicable (e.g., for images)
    pub thumbhash: Option<String>,
    // Metadata associated with the processed media file, including size, MIME type, dimensions, and duration
    pub meta: MediaMeta,
    // Optional directory for post-processing jobs (e.g., HLS processing)
    pub post_job_dir: Option<String>,
}

impl Default for MediaOptions {
    fn default() -> Self {
        MediaOptions {
            folder: String::new(),
            size_gate: None,
            validation: None,
            processing_order: MediaProcessing::default(),
        }
    }
}

/// --------------------------------
/// Media Data Group
/// --------------------------------
pub struct MediaData {
    pub id: i64,
    pub uploader_id: i64,
    pub path: String,
    pub name: String,
    pub meta: MediaMeta,
    pub is_animated: bool,
    pub locked: bool,
    pub lock_hashed: Option<String>,
    pub thumbhash: Option<String>,
}

///* Note: master.m3u8 will provide the variant playlist for HLS streaming, can be extracted in frontend
///* Often get from Hls.level in React.

#[derive(Debug, Clone, Serialize)]
pub struct MediaMeta {
    pub size: usize,
    pub mime: String,

    pub width: Option<i32>,
    pub height: Option<i32>,
    pub duration: Option<f32>,
}

/// --------------------------------
/// Media Extractor Types
/// --------------------------------
/// No refactor needed. (thankfully)

/// ProcessObject is the object that service will use to process media files.
/// Allowing to add extra information for the file to work with, such as the name of the file, and the path of the file.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ProcessObject {
    pub path: String,
    pub size: usize,
    pub data: Option<RawFileValue>,
    pub thumbnail_path: Option<String>,
    pub thumbnail_size: Option<usize>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RawFileValue {
    pub name: String,
    pub extension: String,
}


/// TempUpload is the object that service will use to extract media files from the multipart form data.
/// Easier for some file that doesnt need extra information, such as the name of the file, and the path of the file.
impl Into<ProcessObject> for TempUpload {
    fn into(self) -> ProcessObject {
        ProcessObject {
            path: self.path,
            size: self.size,
            data: None,
            thumbnail_path: None,
            thumbnail_size: None,
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct TempUpload {
    pub path: String,
    pub size: usize,
}

#[derive(Debug, Clone)]
pub struct LocalTempUpload {
    pub full_path: PathBuf,
    pub relative_path: String,
    pub size: usize,
}

#[derive(Debug, Clone)]
pub struct ExtractedPayload<T> {
    pub payload: T,
    pub files: Vec<TempUpload>,
}
