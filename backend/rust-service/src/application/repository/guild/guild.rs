use sqlx::{Postgres, Transaction};

use crate::application::repository::guild::row::{ GuildChannelMessages, GuildChannelsRow, GuildMembersRow, GuildRolesRow, GuildRow};

// todo: CRUD guild
// ?: A change to implement the tags into the guild

// * Using Offset since just changing the page 
// ? sort by tags?? (if there's tags migration) including The Recommmendation

pub async fn get_guild_for_join (
    tx: &mut Transaction<'_, Postgres>,
    page: i64
) -> Result<Vec<GuildRow>, sqlx::Error>{
    let limit = 20;
    let safe_page = if page < 1 { 1 } else { page };
    let offset = (safe_page - 1) * limit; 
    sqlx::query_as::<_, GuildRow>(
        r#"
            SELECT * 
            FROM guilds
            LIMIT $1
            OFFSET $2
        "#
    )
    .bind(limit)
    .bind(offset)
    .fetch_all(tx.as_mut())
    .await
}

// * check the permission 
// * impl soon
// ? I'll think about it, later.
pub async fn get_owner_guild(
) {
    
}

pub async fn create_guild(
    tx: &mut Transaction<'_, Postgres>,
    id: i64,
    owner_id: i64,
    name: &str,
    description: &str
) -> Result<GuildRow, sqlx::Error> {
    sqlx::query_as::<_,GuildRow>(
        r#"
            INSERT INTO guilds 
            (id, 
            name, 
            description, 
            owner_id, 
            total_members, 
            created_at, 
            updated_at)

            VALUES ($1, $2, $3 , 1 , NOW(), NOW())
            "#
    )
    .bind(id)
    .bind(name)
    .bind(description)
    .bind(owner_id)
    .fetch_one(&mut **tx)
    .await
}

pub async fn join_guild(
    tx: &mut Transaction<'_, Postgres>,
    user_id:i64,
    guild_id:i64
) -> Result<GuildMembersRow, sqlx::Error> {
    sqlx::query_as::<_,GuildMembersRow>(
        r#"
            INSERT INTO guild_members
            (guild_id,
            user_id,
            joined_at
            )

            VALUES ($1, $2, NOW())
        "#
    )
    .bind(guild_id)
    .bind(user_id)
    .fetch_one(&mut **tx)
    .await
}

pub async fn delete_guild(
    tx: &mut Transaction<'_, Postgres>,
    user_id:i64,
    guild_id:i64,
) -> Result<(), sqlx::Error> {
    sqlx::query(
        r#"
            DELETE FROM guilds
            WHERE guild_id = $1 AND user_id = $2
        "#
    )
    .bind(guild_id)
    .bind(user_id)
    .execute(tx.as_mut())
    .await?;

    Ok(())
}

//----------------------------------------------------------------------//
// * Guild Info Thingy                                                  //
//----------------------------------------------------------------------//
// todo: get the guild info
// ? impl with exilir BEAM for loading 
// ! DON'T fetch everything at once
/// * Guild info -> channels -> current channel messages -> guild roles -> members (panigation) -> guild asset
/// * seperate their roles if role is visible on showing [Front-end]
/// * member data will fetch on click their profile
/// * 
pub async fn get_guild_info (
    tx: &mut Transaction<'_, Postgres>,
    id: i64
) -> Result<GuildRow , sqlx::Error> {
    sqlx::query_as::<_, GuildRow>(
        r#"
            SELECT id,
            owner_id,
            name,
            description,
            total_members,
            total_channels,
            created_at,
            updated_at

            FROM guilds
            WHERE id = $1 
        "#
  )
  .bind(id) 
  .fetch_one(tx.as_mut())
  .await
}

pub async fn get_guild_channels(
    tx: &mut Transaction<'_, Postgres>,
    guild_id: i64,
) -> Result<Vec<GuildChannelsRow>, sqlx::Error> {
    sqlx::query_as::<_,GuildChannelsRow>(
        r#"
            SELECT id,
            guild_id,
            name,
            channel_type,
            created_at,
            updated_at
            FROM guild_channels
            WHERE guild_id = $1
            ORDER BY id DESC 
        "#
    )
    .bind(guild_id)
    .fetch_all(tx.as_mut())
    .await
}

pub async fn get_guild_channel_messages (
    tx: &mut Transaction<'_, Postgres>,
    channel_id: i64,
    cursor_id: Option<i64>
) -> Result<Vec<GuildChannelMessages>, sqlx::Error> {
    sqlx::query_as::<_, GuildChannelMessages>(
    r#"
        SELECT id,
        user_id,
        target_type,
        content,
        has_attachment,
        has_reaction,
        created_at,
        updated_at
        FROM messages
        WHERE target_id = $1
            AND($2::bigint IS NULL or id < $2)
        ORDER BY id DESC
        LIMIT 10
    "#
  )
  .bind(channel_id)
  .bind(cursor_id)
  .fetch_all(tx.as_mut())
  .await
}

// * LET THE FRONT END DO PAGNIGATION lol
pub async fn get_guild_roles(
    tx: &mut Transaction<'_, Postgres>,
    guild_id: i64,
) ->Result<Vec<GuildRolesRow>, sqlx::Error> {
    sqlx::query_as::<_, GuildRolesRow>(
        r#"
            SELECT *
            FROM guild_roles
            WHERE guild_id = $1 AND deleted_at = NULL
            ORDER BY position 
        "#
    )
    .bind(guild_id)
    .fetch_all(tx.as_mut())
    .await
}

// * JOIN users table for some username and avatar, yum yum.
/// ! recheck again

pub async fn get_guild_members (
      tx: &mut Transaction<'_, Postgres>,
      guild_id : i64,
) -> Result< Vec<GuildMembersRow>, sqlx::Error> {
  sqlx::query_as::<_,GuildMembersRow>(
    r#"
        SELECT
            guild_id,
            user_id,
            joined_at
        FROM guild_members
        WHERE guild_id = $1
          AND ($2::bigint IS NULL OR user_id > $2)
        ORDER BY user_id ASC
        LIMIT 20
    "#
  )
  .bind(guild_id)
  .fetch_all(tx.as_mut())
  .await
}


// * members, roles, channels,

// * Simple guild update
pub async fn update_guild(
) {
    
}