use serde::Deserialize;
use serde::Serialize;
use serde_with::skip_serializing_none;
use sqlx::types::Json;

use crate::application::repository::media::row::Attachment;
use crate::application::repository::media::row::MediaFullDataRow;
use crate::application::repository::user::row::UserProfileRow;

/// use before caching
#[derive(sqlx::FromRow, Debug, Deserialize, Serialize)]
pub struct PostBaseRow {
    pub author_id: i64,
    pub post_id: String,
    pub content: Option<String>,

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

    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}


#[derive(sqlx::FromRow, Debug, Deserialize, Serialize)]
pub struct FeedRow {
    pub post_id: i64,
}

#[derive(sqlx::FromRow, Debug, Deserialize, Serialize)]
pub struct UserPostsRow {
    pub post_id: i64,
}

/// Full post data, after caching
#[derive(sqlx::FromRow, Debug, Deserialize, Serialize)]
pub struct PostRow {
    // media_posts table
    pub author: Json<UserProfileRow>,

    pub post_id: String,
    pub content: Option<String>,

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

    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

#[skip_serializing_none]
#[derive(sqlx::FromRow, Debug, Deserialize, Serialize)]
pub struct RepostedPostRow {
    pub id: String,
    pub author: Json<UserProfileRow>,
    pub content: Option<String>,
    pub is_reposted: bool,
    pub visibility: PostVisibility,

    pub has_attachment: bool,
    pub attachments: Json<Vec<Attachment>>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}


#[derive(sqlx::FromRow, Debug, Deserialize, Serialize)]
pub struct CommentRow {
    pub id: String,
    pub post_id: String,
    
    pub author: Json<UserProfileRow>,
    pub total_likes: i32,
    pub is_liked: bool,

    pub content: Option<String>,
    pub has_attachment: bool,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
    pub attachments: Json<Vec<Attachment>>,

}

#[derive(sqlx::FromRow, Debug, Deserialize)]
pub struct CreatePostRow {
    pub id: i64,
    pub user_id: i64,
    pub content: Option<String>,
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
    pub content: Option<String>,
    pub has_attachment: bool,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
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
    pub target_id: String,
    pub target_type: TagTarget,
    pub tag_id: String,
    pub tag_name: String,
    pub tag_color: Option<String>,
}

#[derive(serde::Deserialize, sqlx::Type, Debug)]
#[sqlx(rename_all = "lowercase")]
pub enum MediaTypeAttachment {
    Post,
    Comment,
    Message,
    Community,
}

impl MediaTypeAttachment {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Post => "post",
            Self::Comment => "comment",
            Self::Message => "message",
            Self::Community => "community",
        }
    }
}

#[derive(serde::Deserialize, sqlx::Type, Debug, serde::Serialize)]
#[sqlx(type_name = "post_visibility", rename_all = "lowercase")]
#[serde(rename_all = "lowercase")]
pub enum PostVisibility {
    Everyone,
    Friend,
    Private,
}

#[derive(serde::Deserialize, serde::Serialize, sqlx::Type, Debug)]
#[sqlx(type_name = "tag_attachment_types", rename_all = "lowercase")]
#[serde(rename_all = "lowercase")]
pub enum TagTarget {
    User,
    Post,
    Guild,
}

impl ToString for TagTarget {
    fn to_string(&self) -> String {
        match self {
            TagTarget::User => "user".to_string(),
            TagTarget::Post => "post".to_string(),
            TagTarget::Guild => "guild".to_string(),
        }
    }
}

impl ToString for PostVisibility {
    fn to_string(&self) -> String {
        match self {
            PostVisibility::Everyone => "everyone".to_string(),
            PostVisibility::Friend => "friend".to_string(),
            PostVisibility::Private => "private".to_string(),
        }
    }
}