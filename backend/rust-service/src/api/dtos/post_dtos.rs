use serde::Serialize;

use crate::application::repository::media::row::{MediaDataWithMetadataRow, MediaStatus};


#[derive(Debug, Serialize)]
pub struct PostDTO {
    pub id:String,
    pub user_id: String,
    pub content: String,
    pub total_comment: i32,
    pub reposted_from: String,
    pub is_repost: bool,
    pub has_attachment: bool,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
    pub media_tags: Option<Vec<String>>,
    pub visibility: String
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