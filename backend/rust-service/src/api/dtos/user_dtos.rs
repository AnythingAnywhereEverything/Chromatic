use serde::{Serialize};

use crate::application::{repository::media::row::MediaDataWithMetadataRow, service::media::types::MediaCategory};

#[derive(Debug, Serialize)]
pub struct UserDTO {
    pub id: String, // Use String to avoid issues with JavaScript number precision
    pub email: String,
    pub username: Option<String>,
    pub display_name: Option<String>,
    pub bio: Option<String>,
    pub avatar_media_id: Option<MediaFullDTO>,
    pub banner_media_id: Option<MediaFullDTO>,
    pub created_at: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct MediaFullDTO {
    pub id: String,
    pub media_url: String,
    pub media_preview_url: Option<String>,
    pub media_category: MediaCategory,
    pub file_size: i64,
    pub mime_type: String,
    pub width: Option<i32>,
    pub height: Option<i32>,
    pub duration: Option<f32>,
}

impl From<MediaDataWithMetadataRow> for MediaFullDTO {
    fn from(row: MediaDataWithMetadataRow) -> MediaFullDTO {
        MediaFullDTO {
            id: row.id.to_string(),
            media_url: row.media_url,
            media_preview_url: row.media_preview_url,
            media_category: row.media_category,
            file_size: row.file_size,
            mime_type: row.mime_type,
            width: row.width,
            height: row.height,
            duration: row.duration,
        }
    }
}