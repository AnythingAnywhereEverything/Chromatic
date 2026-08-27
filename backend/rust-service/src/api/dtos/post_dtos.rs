use serde::Serialize;
use sqlx::prelude::FromRow;

use crate::{api::dtos::user_dtos::MediaFullDTO, application::repository::post::row::{CommentRow, PostRow}};


#[derive(Debug, Serialize,FromRow)]
pub struct PostDTO {
    pub id:String,
    pub user_id: String,
    pub username: String,
    pub display_name: String,
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
    pub tag: Vec<TagDTO>,
    pub is_liked: bool,

    pub avatar_path : Option<String>,
    pub avatar_mime : Option<String>,
    pub avatar_thumbhash : Option<String>,

    pub followers_count: i32,
    pub following_count: i32,
    #[sqlx(skip)]
    pub current_user_id: Option<String>, // Will default to None
}

impl Into<PostDTO> for PostRow {
    fn into(self) -> PostDTO {
        PostDTO {
            id: self.id.to_string(),
            user_id: self.user_id.to_string(),
            username: self.username,
            display_name: self.display_name.unwrap_or_default(),
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
                flags: media.flags,
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
            }).collect(),
            is_liked: self.is_liked,
            current_user_id : None,
            avatar_path: self.avatar_path,
            avatar_mime: self.avatar_mime,
            avatar_thumbhash: self.avatar_thumbhash,
            followers_count: self.followers_count,
            following_count: self.following_count,
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
    pub total_liked: i32,
}

impl Into<CommentDTO> for CommentRow {
    fn into(self) -> CommentDTO {
        CommentDTO {
            id: self.id.to_string(),
            post_id: self.post_id.to_string(),
            user_id: self.user_id.to_string(),
            content: self.content,
            total_likes: self.total_likes,
            has_attachment: self.has_attachment,
            created_at: Some(self.created_at.to_string()),
            updated_at: Some(self.updated_at.to_string()),
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
                flags: media.flags,
            }).collect(),
        }
    }
}