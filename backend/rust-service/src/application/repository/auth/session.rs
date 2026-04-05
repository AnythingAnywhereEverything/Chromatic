use chrono::NaiveDateTime;
use sqlx::{Postgres, Transaction};

use crate::{application::repository::auth::row::SessionRow, domain::session::SessionToken};

pub async fn save_session(
    tx: &mut Transaction<'_, Postgres>,
    session_id: i64,
    user_id: i64,
    user_agent: &str,
    ip_address: &str,
    session_hashed: &str,
    created_at: NaiveDateTime,
) -> Result<(), sqlx::Error> {
    let result = sqlx::query(
        r#"
        INSERT INTO sessions (id, user_id, user_agent, ip_address, session_hashed, created_at)
        VALUES ($1, $2, $3, $4, $5, $6)
        "#,
    )
    .bind(session_id)
    .bind(user_id)
    .bind(user_agent)
    .bind(ip_address)
    .bind(session_hashed)
    .bind(created_at)
    .execute(tx.as_mut())
    .await?;

    tracing::trace!("save_session rows_affected={}", result.rows_affected());
    Ok(())
}

pub async fn delete_all_sessions_by_user_id(
    tx: &mut Transaction<'_, Postgres>,
    user_id: i64,
) -> Result<(), sqlx::Error> {
    sqlx::query(
        r#"
        DELETE FROM sessions
        WHERE user_id = $1
        "#,
    )
    .bind(user_id)
    .execute(tx.as_mut())
    .await?;
    Ok(())
}

pub async fn delete_session_by_token(
    tx: &mut Transaction<'_, Postgres>,
    session: &SessionToken,
) -> Result<(), sqlx::Error> {
    sqlx::query(
        r#"
        DELETE FROM sessions
        WHERE session_hashed = $1
        "#,
    )
    .bind(session.full_token.clone())
    .execute(tx.as_mut())
    .await?;
    Ok(())
}

pub async fn is_session_exists(
    tx: &mut Transaction<'_, Postgres>,
    user_id: i64,
    created_time: NaiveDateTime,
) -> Result<bool, sqlx::Error> {
    let result = sqlx::query_scalar(
        r#"
        SELECT EXISTS (
            SELECT 1
            FROM sessions
            WHERE user_id = $1 AND created_at = $2
        )
        "#,
    )
    .bind(user_id)
    .bind(created_time)
    .fetch_one(tx.as_mut())
    .await?;

    Ok(result)
}

pub async fn get_by_id(
    tx: &mut Transaction<'_, Postgres>,
    session_id: i64,
) -> Result<SessionRow, sqlx::Error> {
    let row = sqlx::query_as::<_, SessionRow>(
        r#"
        SELECT id, user_id, user_agent, ip_address, session_hashed, created_at
        FROM sessions
        WHERE id = $1
        "#,
    )
    .bind(session_id)
    .fetch_one(tx.as_mut())
    .await?;

    Ok(row)
}

pub async fn delete_by_id(
    tx: &mut Transaction<'_, Postgres>,
    session_id: i64,
) -> Result<(), sqlx::Error> {
    sqlx::query(
        r#"
        DELETE FROM sessions
        WHERE id = $1
        "#,
    )
    .bind(session_id)
    .execute(tx.as_mut())
    .await?;
    Ok(())
}