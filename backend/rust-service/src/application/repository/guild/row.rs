
#[derive(sqlx::FromRow, Debug)]
pub struct GuildRow{
    pub id: i64,
    pub owner_id: i64,
    pub name: String,
    pub description: String,
    pub total_members: i32,
    pub total_channels: i32,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(sqlx::FromRow, Debug)]
pub struct GuildMembersRow{
    pub guild_id : i64,
    pub user_id : i64,
    pub joined_at: chrono::DateTime<chrono::Utc>,
}

#[derive(sqlx::FromRow, Debug)]
pub struct GuildRolesRow{
    pub id: i64,
    pub guild_id: i64,
    pub name: String,
    pub color: String,
    pub position: i16,
    pub permission_bitmask: i64,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>
}

#[derive(sqlx::FromRow, Debug)]
pub struct GuildMemberRolesRow{
    pub guild_id: i64,
    pub user_id: i64,
    pub role_id: i64,
    pub assigned_at: chrono::DateTime<chrono::Utc>
}

#[derive(sqlx::FromRow, Debug)]
pub struct GuildChannelsRow{
    pub id: i64,
    pub guild_id: i64,
    pub name: String,
    pub channel_type: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>
}

#[derive(sqlx::FromRow, Debug)]
pub struct GuildBans {
    pub guild_id: i64,
    pub user_id: i64,
    pub reason: String,
    pub created_at: chrono::DateTime<chrono::Utc>
}

#[derive(sqlx::FromRow, Debug)]
pub struct GuildChannelOverwritesRow{
    pub channel_id: i64,
    pub target_id: i64,
    pub target_type: String,
    pub allow_mask: u8,
    pub deny_mask: u8,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>
}

// * asset_id is from media_data tb
#[derive(sqlx::FromRow, Debug)]
pub struct GuildAssetsRow{
    pub id: i64,
    pub guild_id: i64,
    pub asset_id: i64,
    pub asset_type: String,
    pub asset_name: String,
    pub asset_emoji: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(sqlx::FromRow, Debug)]
pub struct GuildChannelMessages{
    pub id: i64,
    pub user_id: i64,
    pub target_id: i64,
    pub target_type: String,
    pub content: String,
    pub has_attachment: bool,
    pub has_reaction: bool,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug)]
pub struct GuildPageInformation {
    pub main_info: GuildRow,

    pub guild_channels: Vec<GuildChannelsRow>,
    pub members: Vec<GuildMembersRow>,
    pub member_roles: Vec<GuildMemberRolesRow>,
    pub roles: Vec<GuildRolesRow>,
    pub bans: Vec<GuildBans>,
    pub channel_overwrites: Vec<GuildChannelOverwritesRow>,
    pub assets: Vec<GuildAssetsRow>,
}