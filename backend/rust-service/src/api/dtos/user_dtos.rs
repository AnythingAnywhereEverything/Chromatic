use serde::{Serialize};

use crate::application::{repository::media::row::{MediaDataWithMetadataRow, MediaStatus}};

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

pub struct PublicUserProfileDTO {
    pub id: String,
    pub email: String,
    pub username: Option<String>,
    pub display_name: Option<String>,
    pub bio: Option<String>,
    pub avatar_media_id: Option<MediaFullDTO>,
    pub banner_media_id: Option<MediaFullDTO>,
    pub is_verified: bool,
    pub is_private: bool,
    pub is_blocked: bool,
    pub is_following: bool,
    pub is_friend: bool,
    pub followers_count: i64,
    pub following_count: i64,
    pub created_at: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct MediaFullDTO {
    pub id: String,
    pub path: String,
    pub name: String,
    pub thumbhash: Option<String>,
    pub status: MediaStatus,
    pub created_at: chrono::DateTime<chrono::Utc>,
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
            path: row.path,
            name: row.name,
            thumbhash: row.thumbhash,
            status: row.status,
            created_at: row.created_at,
            file_size: row.file_size,
            mime_type: row.mime_type,
            width: row.width,
            height: row.height,
            duration: row.duration,
        }
    }
}