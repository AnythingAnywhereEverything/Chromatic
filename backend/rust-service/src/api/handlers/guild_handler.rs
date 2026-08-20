// use axum::{Json, extract::{Path, State}};

// use crate::{api::{APIError, RequestAuth, dtos::guild_dtos::FullGuildDTO}, application::state::{self, SharedState}};

#[derive(serde::Deserialize, Debug)]
pub struct CreateGuildRequest {
    pub content: String,
    pub name: String,
    pub description : String,
    pub media_tags: Option<Vec<i64>>
}

// pub async fn create_guild_handler(
//     State(state): State<SharedState>,
//     Path((version, post_id)): Path<(String, i64)>,
//     req_auth: RequestAuth,
//     payload : Json<CreateGuildRequest>
// ) -> Result<FullGuildDTO, APIError> {
//     todo!()
// }