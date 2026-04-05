use std::path::PathBuf;

use serde::{Deserialize, Serialize};

pub enum MediaProcessingMode {
    Sanitize,       // transform (webp, strip metadata)
    Hls,
    Raw,            // keep original
}

pub enum ImageTransform {
    Resize {
        max_width: i32,
        max_height: i32,
    },
    Crop {
        max_width: i32,
        max_height: i32,
        ratio: Option<(i32, i32)>,
    },
    None,
}

pub enum AllowedMediaType {
    Jpeg,
    Png,
    WebP,
    Mp4,
}

pub enum MediaCategory {
    Image,
    Video,
    Audio,
    Document,
    Code,
    Archive,
    Unknown,
}

pub struct MediaOptions {
    pub folder: String,
    pub max_size: usize,
    pub allowed_types: Option<Vec<AllowedMediaType>>,
    pub image_transform: Option<ImageTransform>,

    pub mode: MediaProcessingMode,

    // * only for video
    // * if the mode set to raw, these will be ignore unconditionally
    pub hls_fallback: bool
}

pub struct SavedMedia {
    pub path: String,
    pub category: MediaCategory,

    pub meta: MediaMeta,
    pub video_manifest: Option<VideoManifest>,
}


#[derive(Debug, Serialize, Deserialize)]
pub struct VideoVariant {
    pub resolution: i32,   // * height (e.g. 720)
    pub playlist: String,  // * path to m3u8
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VideoManifest {
    pub master: String, // * master.m3u8
    pub variants: Vec<VideoVariant>,
}

#[derive(Debug)]
pub struct MediaMeta {
    pub size: usize,
    pub mime: String,

    pub width: Option<i32>,
    pub height: Option<i32>,
    pub duration: Option<f32>,
}

#[derive(Debug, Clone)]
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

#[derive(Debug)]
pub struct ExtractedPayload<T> {
    pub payload: T,
    pub files: Vec<TempUpload>,
}