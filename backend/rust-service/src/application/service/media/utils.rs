use crate::application::service::{errors::MediaServiceError, media::types::{MediaCategory, MediaType, ValidationOptions, ValidationType}};

pub fn categorize(mime: &str) -> MediaCategory {
    match mime {
        // categorize based on mime type
        m if m.starts_with("image/") => MediaCategory::Image,
        m if m.starts_with("video/") => MediaCategory::Video,
        m if m.starts_with("audio/") => MediaCategory::Audio,

        "application/pdf" => MediaCategory::Document,

        "application/zip" | "application/x-tar" | "application/x-rar-compressed" => {
            MediaCategory::Archive
        }

        "text/plain"
        | "application/json"
        | "application/javascript"
        | "text/x-rust"
        | "text/x-python"
        | "text/x-java"
        | "text/x-c++"
        | "text/x-c" => MediaCategory::Code,

        _ => MediaCategory::Unknown,
    }
}

pub fn get_media_types_from_mime(mime: &str) -> Vec<MediaType> {
    match mime {
        "image/jpeg" => vec![MediaType::GenericJpeg, MediaType::Image],
        "image/png" => vec![MediaType::GenericPng, MediaType::Image],
        "image/webp" => vec![MediaType::GenericWebP, MediaType::Image],
        "video/mp4" => vec![MediaType::GenericMp4, MediaType::Video],
        "image/gif" => vec![MediaType::GenericGif, MediaType::Image],

        m if m.starts_with("image/") => vec![MediaType::Image],
        m if m.starts_with("video/") => vec![MediaType::Video],
        m if m.starts_with("audio/") => vec![MediaType::Audio],

        _ => vec![],
    }
}

pub fn validate_media_type(
    mime: &str,
    validation: &Option<ValidationOptions>,
) -> Result<(), MediaServiceError> {
    if let Some(validation) = validation {
        let validation_type = &validation.validation_type;
        let category = super::utils::categorize(mime);

        let is_disallowed = validation.value.iter().any(|disallowed| match disallowed {
            MediaType::GenericJpeg => mime == "image/jpeg",
            MediaType::GenericPng => mime == "image/png",
            MediaType::GenericWebP => mime == "image/webp",
            MediaType::GenericMp4 => mime == "video/mp4",
            MediaType::GenericGif => mime == "image/gif",

            // Categorized types
            MediaType::Image => category == MediaCategory::Image,
            MediaType::Video => category == MediaCategory::Video,
            MediaType::Audio => category == MediaCategory::Audio,
        });

        match validation_type {
            ValidationType::Whitelisted => {
                if !is_disallowed {
                    return Err(MediaServiceError::InvalidMediaType);
                }
            }
            ValidationType::Blacklisted => {
                if is_disallowed {
                    return Err(MediaServiceError::InvalidMediaType);
                }
            }
        }
    }

    Ok(())
}
