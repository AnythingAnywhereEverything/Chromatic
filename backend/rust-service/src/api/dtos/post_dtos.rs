use serde::Serialize;
use sqlx::prelude::FromRow;

use crate::api::dtos::user_dtos::MediaFullDTO;

#[derive(Debug, Serialize, FromRow)]
pub struct CommentDTO {
    pub id: String,
    pub post_id: String,
    pub user_id: String,

    pub username: String,
    pub display_name: String,

    pub avatar_path: Option<String>,
    pub avatar_mime: Option<String>,
    pub avatar_thumbhash: Option<String>,

    pub followers_count: i32,
    pub following_count: i32,

    pub content: String,
    pub total_likes: i32,
    pub is_liked: bool,
    pub has_attachment: bool,

    pub created_at: Option<String>,
    pub updated_at: Option<String>,

    pub media: Vec<MediaFullDTO>,

    #[sqlx(skip)]
    pub current_user_id: Option<String>,
}