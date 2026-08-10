use crate::application::repository::media::row::MediaDataRow;

#[derive(sqlx::FromRow)]
pub struct PostRow {
    // media_posts tb
    pub id: i64,
    pub user_id: i64,
    pub content: Option<String>,
    pub total_likes: i32,
    pub reposted_from: Option<i64>,
    pub is_repost: bool,
    pub has_attachment: bool,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
    pub media_tags: Vec<String>,
    #[sqlx(skip)]
    pub media_attachment: Option<Vec<MediaDataRow>>
}

#[derive(sqlx::FromRow)]
pub struct TotalLikedRow{
    pub id: i64,
    pub total_liked: i32
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

#[derive(sqlx::FromRow)]
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