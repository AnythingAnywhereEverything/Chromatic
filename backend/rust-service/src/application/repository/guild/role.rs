use sqlx::{Postgres, Transaction};

use crate::application::repository::guild::row::GuildRolesRow;



pub async fn create_guild_role(
    tx: &mut Transaction<'_, Postgres>,
    id: i64,
    guild_id: i64,
    name: &str,
    color: &str,
    position: i16,
    permission: i64,
) -> Result<GuildRolesRow, sqlx::Error> {
    sqlx::query_as::<_,GuildRolesRow>(
        r#"
            INSERT INTO guild_roles
            (id,
            guild_id,
            name,
            color,
            postition,
            permission_bitmask
            created_at,
            upadted_at
            )
            VALUES ($1, $2, $3 , $4, $5, $6 , NOW(), NOW())
        "#
    )
    .bind(id)
    .bind(guild_id)
    .bind(name)
    .bind(color)
    .bind(position)
    .bind(permission)
    .fetch_one(tx.as_mut())
    .await
}

pub async fn update_guild_role(
    tx: &mut Transaction<'_, Postgres>,
    id: i64,
    guild_id: i64,
    name: &str,
    color: &str,
    position: i16,
    permission: i64,
) -> Result<GuildRolesRow, sqlx::Error> {
    sqlx::query_as::<_,GuildRolesRow>(
        r#"
            UPDATE guild_roles
            SET 
                name = COALESCE($1, name),
                color = COALESCE($2, color),
                position = COALESCE($3, position),
                permission = COALESCE($4, permission)
            WHERE id = $5 AND guild_id = $6
        "#
    )
    .bind(name)
    .bind(color)
    .bind(position)
    .bind(permission)
    .bind(id)
    .bind(guild_id)
    .fetch_one(tx.as_mut())
    .await
}

pub async fn delete_guild_role(
    tx: &mut Transaction<'_, Postgres>,
    role_id: i64,
    guild_id: i64
) -> Result<(), sqlx::Error> {
  sqlx::query(
    r#"
        DELETE FROM guild_roles
        WHERE role_id = $1 AND guild_id = $2
    "#
  ) 
  .bind(role_id)
  .bind(guild_id)
  .execute(tx.as_mut())
  .await?;
  Ok(())
}
