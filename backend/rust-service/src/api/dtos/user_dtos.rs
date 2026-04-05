use serde::{Serialize};

#[derive(Debug, Serialize)]
pub struct UserDTO {
    pub id: String, // Use String to avoid issues with JavaScript number precision
    pub email: String,
    pub username: Option<String>,
    pub display_name: Option<String>,
    pub bio: Option<String>,
    pub avatar_url: Option<String>,
}