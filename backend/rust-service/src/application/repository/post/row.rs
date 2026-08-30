use chrono::NaiveDateTime;
use serde::Deserialize;
use serde::Serialize;
use sqlx::types::Json;

use crate::api::handlers::post_handler::PostVisibility;
use crate::api::handlers::post_handler::TagTarget;
use crate::application::repository::media::row::Attachment;
use crate::application::repository::media::row::MediaFullDataRow;
use crate::application::repository::user::row::UserProfileRow;

#[derive(sqlx::FromRow, Debug, Deserialize, Serialize)]
pub struct PostRow {
    // media_posts table
    pub author: Json<UserProfileRow>,

    pub post_id: String,
    pub content: String,

    pub total_likes: i32,
    pub total_comments: i32,

    // repost information
    pub is_reposted: bool,
    pub reposted_post: Option<Json<RepostedPostRow>>,

    pub visibility: PostVisibility,
    pub tags: Json<Vec<TagAttachmentFull>>,

    // target user. This indicates whether the target user has liked the post.
    pub is_liked: bool,
    
    // quick access to attachment information and indicate whether the post has any attachments for sql joints.
    pub has_attachment: bool,
    pub attachments: Json<Vec<Attachment>>,
    #[sqlx(skip)]
    pub comments: Json<Vec<CommentRow>>,

    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(sqlx::FromRow, Debug, Deserialize, Serialize)]
pub struct RepostedPostRow {
    pub id: String,
    pub author: Json<UserProfileRow>,
    pub content: String,
    pub is_reposted: bool,
    pub visibility: PostVisibility,

    pub has_attachment: bool,
    pub attachments: Json<Vec<Attachment>>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(sqlx::FromRow, Debug, Deserialize, Serialize)]
pub struct CommentRow {
    pub comment_id: i64,
    pub post_id: i64,
    pub user_id: i64,
    pub username: String,
    pub display_name: String,
    pub total_likes: i32,
    pub is_liked: bool,

    pub avatar_path: Option<String>,
    pub avatar_mime: Option<String>,
    pub avatar_thumbhash: Option<String>,

    pub followers_count: i32,
    pub following_count: i32,
    pub content: String,
    pub has_attachment: bool,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
    pub media_attachment: Json<Vec<MediaFullDataRow>>,
}

#[derive(sqlx::FromRow, Debug, Deserialize)]
pub struct CreatePostRow {
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
    pub media_attachment: Json<Vec<MediaFullDataRow>>,
    pub tags: Json<Vec<TagAttachmentFull>>,
    #[sqlx(skip)]
    pub comments: Json<Vec<CommentRow>>,
}

#[derive(sqlx::FromRow, Deserialize, Serialize)]
pub struct HasAttachmentRow {
    pub target_id: i64,
    pub media_id: i64,
    pub target_type: String,
}

#[derive(sqlx::FromRow, Deserialize)]
pub struct TotalLikesRow {
    pub id: i64,
    pub total_likes: i32,
}

#[derive(sqlx::FromRow, Deserialize)]
pub struct TotalCommentRow {
    pub post_id: i64,
    pub total_comments: i32,
}

#[derive(sqlx::FromRow, Deserialize)]
pub struct PostLikesRow {
    pub id: i64,
    pub user_id: i64,
    pub media_id: i64,
    pub is_liked: bool,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(sqlx::FromRow, Deserialize)]
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

#[derive(sqlx::FromRow, Debug, Serialize, Deserialize)]
pub struct TagAttachmentRow {
    pub target_id: i64,
    pub target_type: TagTarget,
    pub tag_id: i64,
}

#[derive(sqlx::FromRow, Debug, Serialize, Deserialize)]
pub struct TagAttachmentFull {
    pub target_id: i64,
    pub target_type: TagTarget,
    pub tag_id: i64,
    pub tag_name: String,
    pub tag_color: Option<String>,
}
