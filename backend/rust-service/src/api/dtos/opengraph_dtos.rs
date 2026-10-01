use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct OpenGraphRequest {
    pub url: String,
}

#[derive(Debug, Serialize)]
pub struct OpenGraphResponse {
    pub url: String,
    pub title: Option<String>,
    pub description: Option<String>,
    pub image: Option<String>,
}