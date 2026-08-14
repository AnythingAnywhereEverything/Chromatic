use crate::{api::handlers::post_handler::PostVisibility, application::repository::media::row::MediaDataRow};

#[derive(sqlx::FromRow, Debug)]
pub struct PostRow {
    // media_posts tb
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
    #[sqlx(skip)]
    pub media_attachment: Option<Vec<MediaDataRow>>
}
#[derive(sqlx::FromRow)]
pub struct HasAttachmentRow{
    pub target_id: i64,
    pub media_id: i64,
    pub target_type: String
}

#[derive(sqlx::FromRow)]
pub struct TotalLikedRow{
    pub id: i64,
    pub total_liked: i32
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
    pub media_attachment: Option<Vec<MediaDataRow>>
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

#[derive(sqlx::FromRow)]
pub struct TagAttachmentRow{
    pub target_id: i64,
    pub target_type: String,
    pub tag_id: i64
}

#[derive(sqlx::FromRow)]
pub struct TagAttachmentFull{
    pub target_id: i64,
    pub target_type: String,
    pub tag_id: i64,
    pub tag_name: String
}