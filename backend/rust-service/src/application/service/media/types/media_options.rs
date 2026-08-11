use serde::{Deserialize, Serialize};

#[derive(Debug, Clone)]

// * ----------------------------
// * Enums
// * ----------------------------

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

// * ----------------------------
// * Media Options
// * ----------------------------

#[derive(Debug, Clone)]
pub enum OnProcessingType {
    ImageResize {
        style: super::image_transform::ResizeStyle,
        upscale: bool,
    },
    ImageCrop {
        style: super::image_transform::CropStyle,
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
        style: super::image_transform::CropStyle,
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

#[derive(Debug, Clone)]
pub struct MediaProcessing {
    pub options: ProcessingOptions,
    pub on_processing: Option<Vec<OnProcessingType>>,
    pub post_processing: Option<Vec<PostProcessingType>>,
}

#[derive(Debug, Clone)]
pub struct ValidationOptions {
    pub validation_type: ValidationType,
    pub value: Vec<MediaType>,
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

// * ----------------------------
// * Media Blobs
// * ----------------------------

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

#[derive(Debug, Clone, Serialize)]
pub struct MediaMeta {
    pub size: usize,
    pub mime: String,

    pub width: Option<i32>,
    pub height: Option<i32>,
    pub duration: Option<f32>,
}

// * ----------------------------
// * Multipart Extractor Options
// * ----------------------------

pub trait MultipartSchema {
    fn multipart_fields() -> &'static [MultipartField];

    fn multipart_field(name: &str) -> Option<&'static MultipartField> {
        Self::multipart_fields()
            .iter()
            .find(|field| field.name == name)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MultipartFieldKind {
    Text,
    File,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MultipartFieldCardinality {
    Single,
    Optional,
    Many,
    OptionalMany,
}

#[derive(Debug, Clone, Copy)]
pub struct MultipartField {
    pub name: &'static str,
    pub kind: MultipartFieldKind,
    pub cardinality: MultipartFieldCardinality,
}

pub struct MultipartLimits {
    pub max_file_size: usize,
    pub max_files: usize,
}

pub struct MultipartExtractorOptions {
    // Limit for other types of files
    pub limits: MultipartLimits,

    // Optional validation options for uploaded files
    pub validation: Option<ValidationOptions>,

    // Optional size filter gates for uploaded files
    // This allows for different size limits based on the media type of the uploaded file.
    pub size_filter_gate: Option<Vec<FileSizeGate>>,
}

// * ----------------------------
// * Uploaded File Metas
// * ----------------------------

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ProcessObject {
    pub path: String,
    pub size: usize,
    pub data: Option<RawFileValue>,
    pub thumbnail: Option<TempUpload>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RawFileValue {
    pub name: String,
    pub extension: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct TempUpload {
    pub path: String,
    pub size: usize,
    pub mime: String,
    pub extension: String,
}

#[derive(Debug, Clone)]
pub struct FileSizeGate {
    pub media_type: MediaType,
    pub max_size: usize,
}

// * ----------------------------
// * Default Implementations
// * ----------------------------

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

impl Default for MediaProcessing {
    fn default() -> Self {
        MediaProcessing {
            options: ProcessingOptions::default(),
            on_processing: None,
            post_processing: None,
        }
    }
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

impl Default for MultipartExtractorOptions {
    fn default() -> Self {
        Self {
            limits: MultipartLimits {
                max_file_size: 10 * 1024 * 1024, // 10 MB
                max_files: 5,
            },
            validation: None,
            size_filter_gate: None,
        }
    }
}

impl Into<ProcessObject> for TempUpload {
    fn into(self) -> ProcessObject {
        ProcessObject {
            path: self.path,
            size: self.size,
            data: None,
            thumbnail: None,
        }
    }
}
