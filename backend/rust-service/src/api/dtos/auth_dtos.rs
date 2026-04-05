use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct LoginRequest {
    pub username_or_email: String,
    pub password: String,
}

#[derive(Debug, Serialize)]
pub struct LoginResponse {
    pub token: String,
    pub user_id: String, // Use String to avoid issues with JavaScript number precision
}

#[derive(Debug, Deserialize)]
pub struct RegisterRequest {
    pub display_name: Option<String>,
    pub username: String,
    pub email: String,
    pub password: String,
}

#[derive(Debug, Serialize)]
pub struct RegisterResponse {
    pub token: String,
    pub user_id: String,
}