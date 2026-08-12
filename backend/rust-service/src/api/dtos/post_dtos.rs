use serde::Serialize;

use crate::{api::dtos::user_dtos::MediaFullDTO};


#[derive(Debug, Serialize)]
pub struct PostDTO {
    pub id:String,
    pub user_id: String,
    pub content: String,
    pub total_likes: i32,
    pub total_comments: i32,
    pub reposted_from: Option<String>,
    pub is_repost: bool,
    pub has_attachment: bool,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
    pub visibility: String,

    pub media: Vec<MediaFullDTO>,
    pub tag: Vec<TagDTO>
}

#[derive(Debug, Serialize)]
pub struct TagDTO {
    pub target_id: String,
    pub tag_name: String,
    pub tag_id: String
}