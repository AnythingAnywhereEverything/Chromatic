use crate::application::service::errors::media_service::InspectionError;
use content_inspector::ContentType;
use std::{path::Path, process::Command};

#[derive(Debug, PartialEq, Clone, serde::Serialize, serde::Deserialize)]
pub enum MediaKind {
    Image,
    Video,
    Audio,
    Code,
    Document,
    Archive,
    Application,
    Generic,
}

#[derive(Debug, PartialEq, Clone, serde::Serialize, serde::Deserialize)]
pub enum GenericKind {
    Jpeg,
    Png,
    Gif,
    Webp,
    Bmp,
    Tiff,
    Avif,
    Mp4,
    Webm,
    Mkv,
    Mov,
    Avi,
    Flv,
    Mpeg,
}

#[derive(Debug, PartialEq, Clone, serde::Serialize, serde::Deserialize)]
pub enum FileType {
    Category(MediaKind),
    Generic(GenericKind),
}

pub fn get_file_type(mime: &str) -> Vec<FileType> {
    match mime {
        "image/jpeg" => vec![FileType::Generic(GenericKind::Jpeg), FileType::Category(MediaKind::Image)],
        "image/png" => vec![FileType::Generic(GenericKind::Png), FileType::Category(MediaKind::Image)],
        "image/gif" => vec![FileType::Generic(GenericKind::Gif), FileType::Category(MediaKind::Image)],
        "image/webp" => vec![FileType::Generic(GenericKind::Webp), FileType::Category(MediaKind::Image)],
        "image/bmp" => vec![FileType::Generic(GenericKind::Bmp), FileType::Category(MediaKind::Image)],
        "image/tiff" => vec![FileType::Generic(GenericKind::Tiff), FileType::Category(MediaKind::Image)],
        "image/avif" => vec![FileType::Generic(GenericKind::Avif), FileType::Category(MediaKind::Image)],
        "video/mp4" => vec![FileType::Generic(GenericKind::Mp4), FileType::Category(MediaKind::Video)],
        "video/webm" => vec![FileType::Generic(GenericKind::Webm), FileType::Category(MediaKind::Video)],
        "video/x-matroska" => vec![FileType::Generic(GenericKind::Mkv), FileType::Category(MediaKind::Video)],
        "video/quicktime" => vec![FileType::Generic(GenericKind::Mov), FileType::Category(MediaKind::Video)],
        "video/x-msvideo" => vec![FileType::Generic(GenericKind::Avi), FileType::Category(MediaKind::Video)],
        "video/x-flv" => vec![FileType::Generic(GenericKind::Flv), FileType::Category(MediaKind::Video)],
        "video/mpeg" => vec![FileType::Generic(GenericKind::Mpeg), FileType::Category(MediaKind::Video)],
        _ => {
            let kind = match mime.split('/').next() {
                Some("image") => MediaKind::Image,
                Some("video") => MediaKind::Video,
                Some("audio") => MediaKind::Audio,
                Some("text") => MediaKind::Code,
                Some("application") => MediaKind::Application,
                Some("document") => MediaKind::Document,
                Some("archive") => MediaKind::Archive,
                _ => MediaKind::Generic,
            };
            vec![FileType::Category(kind)]
        }
    }
}

#[derive(Debug, PartialEq, Clone, serde::Serialize, serde::Deserialize)]
pub struct MediaEarlyInspection {
    pub mime: String,
    pub extension: String,
    pub kind: MediaKind,
}

#[derive(Debug, PartialEq, Clone, serde::Serialize, serde::Deserialize)]
pub struct MediaInspection {
    pub mime: String,
    pub extension: String,
    pub kind: MediaKind,
    pub size_bytes: u64,
    pub width: Option<i32>,
    pub height: Option<i32>,
    pub duration: Option<f64>,
}

/// Uses in Early Media Inspection to determine the media type and category based on the provided bytes.
pub async fn inspect_bytes(bytes: &[u8]) -> Result<MediaEarlyInspection, InspectionError> {
    let (mime, extention) = get_mime_and_extension(bytes)?;
    let kind = categorize(&mime);
    Ok(MediaEarlyInspection {
        mime,
        extension: extention.to_string(),
        kind,
    })
}

pub fn get_mime_and_extension(bytes: &[u8]) -> Result<(String, String), InspectionError> {
    let (mime, extension) = if let Some(kind) = infer::get(bytes) {
        (kind.mime_type(), kind.extension())
    } else {
        match content_inspector::inspect(bytes) {
            ContentType::UTF_8
            | ContentType::UTF_8_BOM
            | ContentType::UTF_16LE
            | ContentType::UTF_16BE
            | ContentType::UTF_32LE
            | ContentType::UTF_32BE => ("text/plain", "txt"),

            ContentType::BINARY => {
                return Ok(("unknown/unknown".to_string(), "".to_string()));
            }
        }
    };

    Ok((mime.to_string(), extension.to_string()))
}

pub fn categorize(mime: &str) -> MediaKind {
    match mime {
        m if m.starts_with("image/") => MediaKind::Image,
        m if m.starts_with("video/") => MediaKind::Video,
        m if m.starts_with("audio/") => MediaKind::Audio,

        "application/pdf" => MediaKind::Document,

        "application/zip" | "application/x-tar" | "application/x-rar-compressed" => {
            MediaKind::Archive
        }

        "text/plain"
        | "application/json"
        | "application/javascript"
        | "text/x-rust"
        | "text/x-python"
        | "text/x-java"
        | "text/x-c++"
        | "text/x-c" => MediaKind::Code,

        _ => MediaKind::Generic,
    }
}

pub fn probe_image(path: &Path) -> Result<Option<(i32, i32)>, InspectionError> {
    let image = rs_vips::VipsImage::new_from_file(path.to_string_lossy().as_ref())
        .map_err(InspectionError::LibvipsError)?;
    Ok(Some((image.get_width(), image.get_height())))
}

pub async fn probe_video(
    path: &Path,
) -> Result<(Option<i32>, Option<i32>, Option<f64>), InspectionError> {
    let output = Command::new("ffprobe")
        .args([
            "-v",
            "error",
            "-select_streams",
            "v:0",
            "-show_entries",
            "stream=width,height",
            "-show_entries",
            "format=duration",
            "-of",
            "json",
        ])
        .arg(path)
        .output()
        .map_err(InspectionError::IoError)?;

    if !output.status.success() {
        return Err(InspectionError::InspectionFailed);
    }

    let value: serde_json::Value =
        serde_json::from_slice(&output.stdout)
            .map_err(|_| InspectionError::InspectionFailed)?;

    let stream = value["streams"]
        .get(0);

    let w = stream
        .and_then(|s| s["width"].as_i64())
        .map(|v| v as i32);

    let h = stream
        .and_then(|s| s["height"].as_i64())
        .map(|v| v as i32);

    let d = value["format"]["duration"]
        .as_str()
        .and_then(|v| v.parse::<f64>().ok());

    Ok((w, h, d))
}
