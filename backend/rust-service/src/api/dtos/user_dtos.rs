use serde::{Serialize};

use crate::application::repository::{media::row::MediaDataWithMetadataRow, user::row::{UserProfileRow, UserProfileMinimalRow}};

#[derive(Debug, Serialize)]
pub struct UserDTO {
    pub id: String, // Use String to avoid issues with JavaScript number precision
    pub email: String,
    pub username: Option<String>,
    pub display_name: Option<String>,
    pub bio: Option<String>,
    pub avatar: Option<String>, // hash name
    pub avatar_thumbhash: Option<String>, // thumbhash
    pub banner: Option<String>, // hash name
    pub banner_thumbhash: Option<String>, // thumbhash
    pub created_at: Option<String>,
}

impl Into<UserDTO> for UserProfileMinimalRow {
    fn into(self) -> UserDTO {
        UserDTO {
            id: self.id.to_string(),
            email: self.email,
            username: self.username,
            display_name: self.display_name,
            bio: self.bio,
            avatar: self.avatar,
            avatar_thumbhash: self.avatar_thumbhash,
            banner: self.banner,
            banner_thumbhash: self.banner_thumbhash,
            created_at: self.created_at.map(|dt| dt.to_rfc3339()),
        }
    }
}

#[derive(Debug, Serialize)]
pub struct PublicUserProfileDTO {
    pub id: String,
    pub email: String,
    pub username: Option<String>,
    pub display_name: Option<String>,
    pub bio: Option<String>,
    pub avatar: Option<String>, // hash name
    pub avatar_thumbhash: Option<String>, // thumbhash
    pub banner: Option<String>, // hash name
    pub banner_thumbhash: Option<String>, // thumbhash
    pub is_blocked: bool,
    pub is_following: bool,
    pub is_follower: bool,
    pub followers_count: i32,
    pub following_count: i32,
    pub created_at: String,
}

impl Into<PublicUserProfileDTO> for UserProfileRow {
    fn into(self) -> PublicUserProfileDTO {
        PublicUserProfileDTO {
            id: self.id.to_string(),
            email: self.email,
            username: self.username,
            display_name: self.display_name,
            bio: self.bio,
            avatar: self.avatar,
            avatar_thumbhash: self.avatar_thumbhash,
            banner: self.banner,
            banner_thumbhash: self.banner_thumbhash,
            is_blocked: false, // This should be set based on the context of the request
            is_following: self.is_following.unwrap_or(false),
            is_follower: self.is_follower.unwrap_or(false),
            followers_count: self.followers_count,
            following_count: self.following_count,
            created_at: self.created_at.to_rfc3339(),
        }
    }
}

#[derive(Debug, Serialize)]
pub struct MediaFullDTO {
    pub id: String,
    pub path: String,
    pub name: String,
    pub thumbhash: Option<String>,
    pub status: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub file_size: i64,
    pub mime_type: String,
    pub width: Option<i32>,
    pub height: Option<i32>,
    pub duration: Option<f32>,
}

impl From<MediaDataWithMetadataRow> for MediaFullDTO {
    fn from(row: MediaDataWithMetadataRow) -> MediaFullDTO {
        MediaFullDTO {
            id: row.id.to_string(),
            path: row.path,
            name: row.name,
            thumbhash: row.thumbhash,
            status: row.status.to_string(),
            created_at: row.created_at,
            file_size: row.file_size,
            mime_type: row.mime_type,
            width: row.width,
            height: row.height,
            duration: row.duration,
        }
    }
}