use sqlx::Transaction;

use crate::application::repository::RepositoryResult;

pub async fn user_display_name(
    tx: &mut Transaction<'_, sqlx::Postgres>,
    user_id: i64,
    new_display_name: &str,
) -> RepositoryResult<()> {
    sqlx::query(
        r#"
        UPDATE user_profiles
        SET display_name = $1,
            updated_at = now()
        WHERE user_id = $2
        "#,
    )
    .bind(new_display_name)
    .bind(user_id)
    .execute(tx.as_mut())
    .await?;

    Ok(())
}

/// status got renamed to quote due to making the site fit the theme
pub async fn user_status(
    tx: &mut Transaction<'_, sqlx::Postgres>,
    user_id: i64,
    new_quote: &str,
) -> RepositoryResult<()> {
    // if not exists, insert new row, else update existing row
    sqlx::query(
        r#"
        INSERT INTO user_profiles (user_id, quote, updated_at)
        VALUES ($2, $1, now())
        ON CONFLICT (user_id)
        DO UPDATE SET quote = $1, updated_at = now()
        "#,
    )
    .bind(new_quote)
    .bind(user_id)
    .execute(tx.as_mut())
    .await?;

    Ok(())
}

pub async fn user_bio(
    tx: &mut Transaction<'_, sqlx::Postgres>,
    user_id: i64,
    new_bio: &str,
) -> RepositoryResult<()> {
    sqlx::query(
        r#"
        UPDATE user_profiles
        SET bio = $1,
            updated_at = now()
        WHERE user_id = $2
        "#,
    )
    .bind(new_bio)
    .bind(user_id)
    .execute(tx.as_mut())
    .await?;

    Ok(())
}

pub async fn banner_media_id(
    tx: &mut Transaction<'_, sqlx::Postgres>,
    user_id: i64,
    banner_media_id: Option<i64>,
) -> RepositoryResult<Option<i64>> {
    let row = sqlx::query_scalar::<_, Option<i64>>(
        r#"
        WITH updated_profile AS (
            UPDATE user_profiles
            SET
                banner_media_id = $1,
                updated_at = now()
            WHERE user_id = $2
            RETURNING banner_media_id
        ),
        banner_to_delete AS (
            SELECT urb.banner_id
            FROM user_recent_banner urb
            WHERE urb.user_id = $2
              AND $1 IS NOT NULL
              AND NOT EXISTS (
                  SELECT 1
                  FROM user_recent_banner existing
                  WHERE existing.user_id = $2
                    AND existing.banner_id = $1
              )
              AND (
                  SELECT COUNT(*)
                  FROM user_recent_banner
                  WHERE user_id = $2
              ) >= 6
            ORDER BY urb.updated_at ASC, urb.banner_id ASC
            LIMIT 1
        ),
        deleted_banner AS (
            DELETE FROM user_recent_banner
            WHERE banner_id IN (
                SELECT banner_id
                FROM banner_to_delete
            )
            RETURNING banner_id
        ),
        upserted_banner AS (
            INSERT INTO user_recent_banner (
                banner_id,
                user_id,
                updated_at
            )
            SELECT
                banner_media_id,
                $2,
                now()
            FROM updated_profile
            WHERE banner_media_id IS NOT NULL
            ON CONFLICT (banner_id)
            DO UPDATE SET
                user_id = EXCLUDED.user_id,
                updated_at = now()
            RETURNING banner_id
        )
        SELECT banner_id
        FROM deleted_banner

        UNION ALL

        SELECT NULL::BIGINT
        WHERE NOT EXISTS (
            SELECT 1
            FROM deleted_banner
        )

        LIMIT 1
        "#,
    )
    .bind(banner_media_id)
    .bind(user_id)
    .fetch_one(tx.as_mut())
    .await?;

    Ok(row)
}

pub async fn avatar_media_id(
    tx: &mut Transaction<'_, sqlx::Postgres>,
    user_id: i64,
    avatar_media_id: Option<i64>,
) -> RepositoryResult<Option<i64>> {
    let row = sqlx::query_scalar::<_, Option<i64>>(
        r#"
        WITH updated_profile AS (
            UPDATE user_profiles
            SET
                avatar_media_id = $1,
                updated_at = now()
            WHERE user_id = $2
            RETURNING avatar_media_id
        ),
        avatar_to_delete AS (
            SELECT ura.avatar_id
            FROM user_recent_avatar ura
            WHERE ura.user_id = $2
              AND $1 IS NOT NULL
              AND NOT EXISTS (
                  SELECT 1
                  FROM user_recent_avatar existing
                  WHERE existing.user_id = $2
                    AND existing.avatar_id = $1
              )
              AND (
                  SELECT COUNT(*)
                  FROM user_recent_avatar
                  WHERE user_id = $2
              ) >= 6
            ORDER BY ura.updated_at ASC, ura.avatar_id ASC
            LIMIT 1
        ),
        deleted_avatar AS (
            DELETE FROM user_recent_avatar
            WHERE avatar_id IN (
                SELECT avatar_id
                FROM avatar_to_delete
            )
            RETURNING avatar_id
        ),
        upserted_avatar AS (
            INSERT INTO user_recent_avatar (
                avatar_id,
                user_id,
                updated_at
            )
            SELECT
                avatar_media_id,
                $2,
                now()
            FROM updated_profile
            WHERE avatar_media_id IS NOT NULL
            ON CONFLICT (avatar_id)
            DO UPDATE SET
                user_id = EXCLUDED.user_id,
                updated_at = now()
            RETURNING avatar_id
        )
        SELECT avatar_id
        FROM deleted_avatar

        UNION ALL

        SELECT NULL::BIGINT
        WHERE NOT EXISTS (
            SELECT 1
            FROM deleted_avatar
        )

        LIMIT 1
        "#,
    )
    .bind(avatar_media_id)
    .bind(user_id)
    .fetch_one(tx.as_mut())
    .await?;

    Ok(row)
}