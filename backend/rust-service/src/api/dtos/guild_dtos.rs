use serde::Deserialize;

// * profile guild, banner
#[derive(Deserialize,Debug)]
pub struct FullGuildDTO {
    pub id: String,
    pub name: String,
    pub description: String,
    pub owner_id: String,
    pub owner_name:String,
    pub total_members: i32,
    pub total_channels: i32,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>
}