use serde::Deserialize;
use serde::Serialize;
use sqlx::types::Json;
use chrono::DateTime;
use chrono::Utc;

use crate::api::handlers::post_handler::TagTarget;
use crate::{api::handlers::post_handler::PostVisibility, application::repository::media::row::MediaDataRow};

#[derive(sqlx::FromRow, Debug)]
pub struct PostRow {
    // media_posts tb
    pub id: i64,
    pub user_id: i64,
    pub username: String,
    pub display_name: Option<String>,
    pub content: String,
    pub total_likes: i32,
    pub total_comments: i32,
    pub reposted_from: Option<i64>,
    pub is_repost: bool,
    pub has_attachment: bool,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
    pub visibility: PostVisibility,
    pub media_attachment: Json<Vec<MediaAttachment>>,
    pub tags: Json<Vec<TagAttachmentFull>>,
    pub is_liked: bool,

    pub avatar_path : Option<String>,
    pub avatar_mime : Option<String>,
    pub avatar_thumbhash : Option<String>,

    pub followers_count: i32,
    pub following_count: i32,
        #[sqlx(skip)]
    pub comments: Json<Vec<CommentRow>>
}

#[derive(sqlx::FromRow, Debug)]
pub struct CreatePostRow{
    pub id: i64,
    pub user_id: i64,
    pub content: String,
    pub total_likes: i32,
    pub total_comments: i32,
    pub reposted_from: Option<i64>,
    pub is_repost: bool,
    pub has_attachment: bool,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
    pub visibility: PostVisibility,
    pub media_attachment: Json<Vec<MediaAttachment>>,
    pub tags: Json<Vec<TagAttachmentFull>>,
    #[sqlx(skip)]
    pub comments: Json<Vec<CommentRow>>
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MediaAttachment {
    pub id: String,
    pub user_id: i64,
    pub path: String,
    pub created_at: DateTime<Utc>,
    pub thumbhash: Option<String>,
    pub name: String,
    pub updated_at: DateTime<Utc>,
    pub status: String,
    pub file_size: i64,
    pub mime_type: String,
    pub width: Option<i32>,
    pub height: Option<i32>,
    pub duration: Option<f32>,
}

#[derive(sqlx::FromRow)]
pub struct HasAttachmentRow{
    pub target_id: i64,
    pub media_id: i64,
    pub target_type: String
}

#[derive(sqlx::FromRow)]
pub struct TotalLikesRow{
    pub id: i64,
    pub total_likes: i32,
}

#[derive(sqlx::FromRow)]
pub struct TotaCommentRow{
    pub id: i64,
    pub total_comment: i32
}

#[derive(sqlx::FromRow)]
pub struct PostLikesRow {   
    pub id: i64,
    pub user_id: i64,
    pub media_id: i64,
    pub is_liked: bool,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(sqlx::FromRow, Debug)]
pub struct CommentRow{
    pub id: i64,
    pub post_id: i64,
    pub user_id: i64,
    pub content: String,
    pub total_likes: i32,
    pub has_attachment: bool,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
    #[sqlx(skip)]
    pub media_attachment: Json<Vec<MediaAttachment>>,
}

#[derive(sqlx::FromRow)]
pub struct CreateCommentResult {
    pub id: i64,
    pub post_id: i64,
    pub user_id: i64,
    pub content: String,
    pub has_attachment: bool,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
    pub total_comments: i32,
    pub total_likes: i32,
}

// Tag attachment

#[derive(sqlx::FromRow,Debug, Serialize)]
pub struct TagAttachmentRow{
    pub target_id: i64,
    pub target_type: TagTarget,
    pub tag_id: i64
}

#[derive(sqlx::FromRow, Debug, Serialize, Deserialize)]
pub struct TagAttachmentFull{
    pub target_id: i64,
    pub target_type: TagTarget,
    pub tag_id: i64,
    pub tag_name: String
}