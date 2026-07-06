use chrono::{NaiveDateTime, Utc};

use crate::application::service::media::types::MediaCategory;


#[derive(Debug, sqlx::Type)]
#[sqlx(type_name = "media_status", rename_all = "lowercase")]
pub enum MediaStatus {
    Processing,
    Completed,
    Failed,
}

#[derive(Debug, sqlx::FromRow)]
pub struct MediaDataRow {
    pub id: i64,
    pub user_id: i64,
    pub media_url: String,
    pub media_preview_url: Option<String>,
    pub media_category: MediaCategory,
    pub media_status: MediaStatus,
    pub created_at: NaiveDateTime,
}

#[derive(Debug, sqlx::FromRow)]
pub struct MediaMetadataRow {
    pub media_id: i64,
    pub file_size: i64, // * in bytes
    pub mime_type: String,
    pub width: Option<i32>,
    pub height: Option<i32>,
    pub duration: Option<f32>,
}

// video manifest row
#[derive(Debug, sqlx::FromRow)]
pub struct VideoManifestRow {
    pub media_id: i64,
    pub variant_resolution: i32, // * height (e.g. 720). master.m3u8 will be in media_data table, and the variants will be in this table
}

#[derive(Debug, sqlx::FromRow)]
pub struct MediaDataWithMetadataRow {
    pub id: i64,
    pub media_url: String,
    pub media_preview_url: Option<String>,
    pub media_category: MediaCategory,
    pub media_status: MediaStatus,
    pub created_at: chrono::DateTime<Utc>,

    pub file_size: i64, // * in bytes
    pub mime_type: String,
    pub width: Option<i32>,
    pub height: Option<i32>,
    pub duration: Option<f32>,
}