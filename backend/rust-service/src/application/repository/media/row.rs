use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

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

impl ToString for MediaStatus {
    fn to_string(&self) -> String {
        match self {
            MediaStatus::Locked => "locked".to_string(),
            MediaStatus::Processing => "processing".to_string(),
            MediaStatus::Pending => "pending".to_string(),
            MediaStatus::Ready => "ready".to_string(),
            MediaStatus::Failed => "failed".to_string(),
            MediaStatus::Completed => "completed".to_string(),
        }
    }
}


#[derive(Debug, sqlx::Type, Serialize)]
#[sqlx(type_name = "media_processing_state", rename_all = "lowercase")]
pub enum ProcessingState {
    Pending,
    Ready,
    Failed,
    Completed,
}

#[derive(Debug, sqlx::Type, Serialize)]
#[sqlx(type_name = "media_post_processing_state", rename_all = "lowercase")]
pub enum PostProcessingState {
    Idle,
    Processing,
    Failed,
    Completed,
}

impl<'de> Deserialize<'de> for ProcessingState {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = String::deserialize(deserializer)?;

        match value.to_lowercase().as_str() {
            "pending" => Ok(Self::Pending),
            "ready" => Ok(Self::Ready),
            "failed" => Ok(Self::Failed),
            "completed" => Ok(Self::Completed),
            _ => Err(serde::de::Error::unknown_variant(
                &value,
                &["Pending", "Ready", "Failed", "Completed"],
            )),
        }
    }
}
impl<'de> Deserialize<'de> for PostProcessingState {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = String::deserialize(deserializer)?;

        match value.to_lowercase().as_str() {
            "idle" => Ok(Self::Idle),
            "processing" => Ok(Self::Processing),
            "failed" => Ok(Self::Failed),
            "completed" => Ok(Self::Completed),
            _ => Err(serde::de::Error::unknown_variant(
                &value,
                &["Idle", "Processing", "Failed", "Completed"],
            )),
        }
    }
}

#[derive(Debug, Serialize, sqlx::Type, Clone, PartialEq, Eq)]
#[sqlx(type_name = "media_type", rename_all = "lowercase")]
#[serde(rename_all = "PascalCase")]
pub enum MediaType {
    Image,
    Video,
    Hls,
    Audio,
    Document,
    Other,
}

impl<'de> Deserialize<'de> for MediaType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = String::deserialize(deserializer)?;

        match value.to_lowercase().as_str() {
            "image" => Ok(Self::Image),
            "video" => Ok(Self::Video),
            "hls" => Ok(Self::Hls),
            "audio" => Ok(Self::Audio),
            "document" => Ok(Self::Document),
            "other" => Ok(Self::Other),
            _ => Err(serde::de::Error::unknown_variant(
                &value,
                &["Image", "Video", "Hls", "Audio", "Document", "Other"],
            )),
        }
    }
}

#[derive(Debug, Serialize, sqlx::Type, Clone, PartialEq, Eq)]
#[sqlx(type_name = "media_kind", rename_all = "lowercase")]
#[serde(rename_all = "PascalCase")]
pub enum MediaKind {
    Original,
    Thumbnail,
    Preview,
    Transcoded,
}

impl<'de> Deserialize<'de> for MediaKind {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = String::deserialize(deserializer)?;

        match value.to_lowercase().as_str() {
            "original" => Ok(Self::Original),
            "thumbnail" => Ok(Self::Thumbnail),
            "preview" => Ok(Self::Preview),
            "transcoded" => Ok(Self::Transcoded),
            _ => Err(serde::de::Error::unknown_variant(
                &value,
                &["Original", "Thumbnail", "Preview", "Transcoded"],
            )),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct MediaRow {
    pub id: i64,
    pub uploader_id: i64,

    pub original_name: String,
    pub original_content_type: String,

    pub file_type: MediaType,
    pub lock_hash: Option<String>,
    pub lock_expiration: Option<NaiveDateTime>,

    pub processing_state: ProcessingState,
    pub post_processing_state: PostProcessingState,

    pub flags: i64,

    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub deleted_at: Option<NaiveDateTime>,
}

impl Default for MediaRow {
    fn default() -> Self {
        MediaRow {
            id: 0,
            uploader_id: 0,
            original_name: String::new(),
            original_content_type: String::new(),
            file_type: MediaType::Other,
            lock_hash: None,
            lock_expiration: None,
            processing_state: ProcessingState::Pending,
            post_processing_state: PostProcessingState::Idle,
            flags: 0,
            created_at: chrono::Utc::now().naive_utc(),
            updated_at: chrono::Utc::now().naive_utc(),
            deleted_at: None,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct MediaObjectsRow {
    pub kind: MediaKind,
    pub storage_key: String,
    pub content_type: String,

    pub size: i64,
    pub name: String,
    pub thumbhash: Option<String>,

    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub deleted_at: Option<NaiveDateTime>,
}
#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct MediaObjectMetadataRow {
    pub width: Option<i32>,
    pub height: Option<i32>,
    pub duration: Option<f64>,

    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct MediaHls {
    pub master_playlist: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct MediaHlsPlaylist {
    pub resolution: String,
    pub playlist_storage_key: String,

    pub segment_count: i32,
    pub segment_duration: f32,

    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl Default for MediaHlsPlaylist {
    fn default() -> Self {
        MediaHlsPlaylist {
            resolution: String::new(),
            playlist_storage_key: String::new(),
            segment_count: 0,
            segment_duration: 0.0,
            created_at: chrono::Utc::now().naive_utc(),
            updated_at: chrono::Utc::now().naive_utc(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct MediaFullDataRow {
    pub id: String,
    pub uploader_id: String,
    pub original_name: String,
    pub original_content_type: String,
    pub file_type: MediaType,
    pub lock_hash: Option<String>,
    pub lock_expiration: Option<NaiveDateTime>,
    pub processing_state: ProcessingState,
    pub post_processing_state: PostProcessingState,
    pub flags: String,
    pub media_objects: Vec<MediaObjectsRow>,
    pub media_object_metadata: Option<MediaObjectMetadataRow>,
    pub media_hls: Option<MediaHls>,
    pub media_hls_playlists: Vec<MediaHlsPlaylist>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub deleted_at: Option<NaiveDateTime>,
}
#[derive(sqlx::FromRow, Debug, Serialize, Deserialize)]
pub struct Attachment {
    pub id: String,
    pub uploader_id: Option<String>,
    pub original_name: Option<String>,
    pub original_content_type: Option<String>,
    pub file_type: MediaType,
    pub processing_state: ProcessingState,
    pub post_processing_state: PostProcessingState,
    pub flags: String,
    pub media_objects: Vec<MediaObjectsRow>,
    pub media_object_metadata: Option<MediaObjectMetadataRow>,
    pub media_hls: Option<MediaHls>,
    pub media_hls_playlists: Vec<MediaHlsPlaylist>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl Into<Attachment> for MediaFullDataRow {
    fn into(self) -> Attachment {
        Attachment {
            id: self.id,
            uploader_id: Some(self.uploader_id),
            original_name: Some(self.original_name),
            original_content_type: Some(self.original_content_type),
            file_type: self.file_type,
            processing_state: self.processing_state,
            post_processing_state: self.post_processing_state,
            flags: self.flags,
            media_objects: self.media_objects,
            media_object_metadata: self.media_object_metadata,
            media_hls: self.media_hls,
            media_hls_playlists: self.media_hls_playlists,
            created_at: self.created_at,
            updated_at: self.updated_at,
        }
    }
}