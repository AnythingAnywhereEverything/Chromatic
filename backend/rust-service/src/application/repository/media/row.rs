use chrono::{NaiveDateTime, Utc};
use serde::Serialize;

#[derive(Debug, sqlx::Type, Serialize)]
#[sqlx(type_name = "media_status", rename_all = "lowercase")]
pub enum MediaStatus {
    // Prioritized Over All Other Statuses
    Locked,
    // Prioritized Over Pending
    Processing,
    // Prioritized Over Failed
    Pending,
    // Prioritized Over Pending but not over Processing
    Ready,
    // Prioritized Over All Other Statuses
    Failed,
    // Completed
    Completed,
}

#[derive(Debug, sqlx::FromRow)]
pub struct MediaDataRow {
    pub id: i64,
    pub uploader_id: i64,
    pub name: String,
    pub path: String,
    pub status: MediaStatus,
    pub thumbhash: Option<String>,
    pub lock_hash: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub lock_expiration: Option<NaiveDateTime>,
    pub deleted_at: Option<NaiveDateTime>,
}

impl Default for MediaDataRow {
    fn default() -> Self {
        MediaDataRow {
            id: 0,
            uploader_id: 0,
            name: String::new(),
            path: String::new(),
            status: MediaStatus::Processing,
            thumbhash: None,
            lock_hash: None,
            created_at: Utc::now().naive_utc(),
            updated_at: Utc::now().naive_utc(),
            lock_expiration: None,
            deleted_at: None,
        }
    }
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

#[derive(Debug, sqlx::FromRow)]
pub struct MediaDataWithMetadataRow {
    pub id: i64,
    pub path: String,
    pub name: String,
    pub thumbhash: Option<String>,
    pub status: MediaStatus,
    pub created_at: chrono::DateTime<Utc>,

    pub file_size: i64,
    pub mime_type: String,
    pub width: Option<i32>,
    pub height: Option<i32>,
    pub duration: Option<f32>,
}