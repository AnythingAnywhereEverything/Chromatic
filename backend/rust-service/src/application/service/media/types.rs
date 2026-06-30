use std::path::PathBuf;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone)]
pub enum MediaProcessingMode {
    Sanitize,       // transform (webp, strip metadata)
    Hls,
    Raw,            // keep original
}

#[derive(Debug, Clone, Copy)]
pub enum CropStyle {
    /// Freeform cropping based on exact width and height dimensions
    Absolute {
        width: u32,
        height: u32,
    },

    /// Normalized cropping based on a percentage of the original image dimensions (e.g., 0.5 for 50% of the original size)
    Normalized {
        width: f32,
        height: f32,
    },

    /// Proportion-locked cropping using a ratio and a defining dimension (e.g., width)
    Ratio {
        ratio: (u32, u32),
        scale: f32,
    },
}

#[derive(Debug, Clone, Copy)]
pub enum ImageTransform {
    Resize {
        rz_width: u32,
        rz_height: u32,
    },
    Crop {
        style: CropStyle,
        /// The anchor point (x, y). None defaults to center cropping.
        /// The coordinates are normalized (0.0 to 1.0) for Ratio and Normalized styles, and absolute pixel values for Absolute style.
        position: Option<(f32, f32)>,
    },
    None,
}


#[derive(Debug, Clone, Copy)]
pub enum AllowedMediaType {
    Jpeg,
    Png,
    WebP,
    Mp4,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, sqlx::Type, Serialize)]
#[sqlx(type_name = "media_category", rename_all = "lowercase")]
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
pub struct MediaOptions {
    pub folder: String,
    pub max_size: usize,
    pub allowed_types: Option<Vec<AllowedMediaType>>,
    pub image_transform: Option<ImageTransform>,

    pub mode: MediaProcessingMode,
    pub manual_preview: Option<TempUpload>, // * if set, will use this as the preview instead of generating one

    // * only for video
    // * if the mode set to raw, these will be ignore unconditionally
    pub hls_fallback: bool
}

#[derive(Debug, Clone)]
pub struct SavedMedia {
    pub path: String,
    pub preview_path: Option<String>,
    pub category: MediaCategory,

    pub meta: MediaMeta,
    pub video_manifest: Option<VideoManifest>,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VideoVariant {
    pub resolution: i32,   // * height (e.g. 720)
    pub playlist: String,  // * path to m3u8
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VideoManifest {
    pub master: String, // * master.m3u8
    pub variants: Vec<VideoVariant>,
}

#[derive(Debug, Clone)]
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

#[derive(Debug, Clone)]
pub struct ExtractedPayload<T> {
    pub payload: T,
    pub files: Vec<TempUpload>,
}