use serde::Serialize;

use crate::{api::dtos::user_dtos::MediaFullDTO, application::repository::post::row::PostRow};


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

impl Into<PostDTO> for PostRow {
    fn into(self) -> PostDTO {
        PostDTO {
            id: self.id.to_string(),
            user_id: self.user_id.to_string(),
            content: self.content,
            total_likes: self.total_likes,
            total_comments: self.total_comments,
            reposted_from: self.reposted_from.map(|id| id.to_string()),
            is_repost: self.is_repost,
            has_attachment: self.has_attachment,
            created_at: Some(self.created_at.to_string()),
            updated_at: Some(self.updated_at.to_string()),
            visibility: self.visibility.to_string(),
            media: self.media_attachment.0.into_iter().map(|media| MediaFullDTO {
                id: media.id.to_string(),
                path: media.path,
                name: media.name,
                thumbhash: media.thumbhash,
                status: media.status,
                created_at: media.created_at,
                file_size: media.file_size,
                mime_type: media.mime_type,
                width: media.width,
                height: media.height,
                duration: media.duration,
            }).collect(),
            tag: self.tags.0.into_iter().map(|tag| TagDTO {
                target_id: tag.target_id.to_string(),
                tag_name: tag.tag_name,
                tag_id: tag.tag_id.to_string()
            }).collect()
        }
    }
}

#[derive(Debug, Serialize)]
pub struct TagDTO {
    pub target_id: String,
    pub tag_name: String,
    pub tag_id: String
}

#[derive(Debug, Serialize)]
pub struct CommentDTO {
    pub id: String,
    pub post_id: String,
    pub user_id: String,
    pub content: String,
    pub total_likes: i32,
    pub has_attachment: bool,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,

    pub media: Vec<MediaFullDTO>
}

#[derive(Debug, Serialize)]
pub struct LikeDTO {
    pub id: String,
    pub total_liked: i32
}