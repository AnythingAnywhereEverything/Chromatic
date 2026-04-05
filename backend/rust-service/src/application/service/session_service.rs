use chrono::{TimeZone, Utc};
use redis::AsyncTypedCommands;

use crate::{
    application::{
        repository::auth::session as session_repo, security::argon, service::errors::SessionServiceError,
        state::AppState,
    },
    domain::session::SessionToken,
};

pub struct SessionService {}

impl SessionService {
    async fn hash_session_token(session_token: &str) -> Result<String, SessionServiceError> {
        argon::hash(session_token.as_bytes()).map_err(|_| SessionServiceError::UnableToHashSession)
    }

    pub async fn create_session(
        state: &AppState,
        user_id: i64,
        user_agent: &str,
        ip_address: &str,
        expiration: u64,
    ) -> Result<SessionToken, SessionServiceError> {
        let token = SessionToken::new(user_id)?;

        tracing::trace!("Generated session token for user_id {}: {:#?}", user_id, token);

        let session_hashed = Self::hash_session_token(&token.full_token).await?;
        let created_at = Utc.timestamp_millis_opt(token.timestamp).unwrap().naive_utc();

        let session_id = state.snowflake_generator.generate_id()?;

        let mut tx = state.db_pool.begin().await?;

        session_repo::save_session(
            &mut tx,
            session_id,
            user_id,
            user_agent,
            ip_address,
            &session_hashed,
            created_at,
        )
        .await?;

        tracing::trace!("Session saved to database for user_id {}: session_id {}, created_at {}", user_id, session_id, created_at);

        tx.commit().await?;

        let mut conn = state.redis.get().await?;
        let key = format!("session_active:{}:{}", user_id, token.timestamp);

        let _: () = conn.set_ex(key, "1", expiration).await?;
        Ok(token)
    }

    pub async fn delete_all_sessions(
        state: &AppState,
        user_id: i64,
    ) -> Result<(), SessionServiceError> {
        let mut tx = state.db_pool.begin().await?;

        session_repo::delete_all_sessions_by_user_id(&mut tx, user_id).await?;

        tx.commit().await?;

        let mut conn = state.redis.get().await?;
        let pattern = format!("session_active:{}:*", user_id);
        let mut cursor: u64 = 0;

        loop {
            let (next_cursor, keys): (u64, Vec<String>) = redis::cmd("SCAN")
                .arg(cursor)
                .arg("MATCH")
                .arg(&pattern)
                .arg("COUNT")
                .arg(100) // batch size
                .query_async(&mut conn)
                .await?;

            if !keys.is_empty() {
                let _: usize = conn.del(keys).await?;
            }

            if next_cursor == 0 {
                break;
            }

            cursor = next_cursor;
        }
        Ok(())
    }

    pub async fn delete_session_token(state: &AppState, token: &str) -> Result<(), SessionServiceError> {
        let session_token = SessionToken::parse(token)?;
        let mut tx = state.db_pool.begin().await?;

        session_repo::delete_session_by_token(&mut tx, &session_token).await?;

        tx.commit().await?;

        let mut conn = state.redis.get().await?;
        let key = format!(
            "session_active:{}:{}",
            session_token.user_id, session_token.timestamp
        );
        let _: usize = conn.del(key).await?;

        Ok(())
    }

    pub async fn validate_session(
        state: &AppState,
        user_id: i64,
        created_time: i64,
        expiration: i64,
        expiration_extend: i64,
    ) -> Result<(), SessionServiceError> {
        let mut tx = state.db_pool.begin().await?;
        let mut conn = state.redis.get().await?;
        let key = format!("session_active:{}:{}", user_id, created_time);

        // * if key exist make cache longer as user stays
        if conn.exists(&key).await? {
            conn.expire(&key, expiration_extend).await?;
            return Ok(())
        }

        let date_time = Utc.timestamp_millis_opt(created_time).unwrap().naive_utc();

        let is_exist = session_repo::is_session_exists(
            &mut tx, 
            user_id, 
            date_time)
        .await?;

        if !is_exist {
            return Err(SessionServiceError::MissingCredential)
        }

        let key = format!("session_active:{}:{}", user_id, created_time);
        let _ : () = conn.set_ex(key, "1", expiration as u64).await?;

        Ok(())
    }

    pub async fn delete_session_user(state: &AppState, session_id: String, user_id: i64) -> Result<(), SessionServiceError> {
        let mut tx = state.db_pool.begin().await?;
        let ses_id = session_id.parse::<i64>().map_err(|_| SessionServiceError::InvalidSessionID)?;

        let session_token: crate::application::repository::auth::row::SessionRow = session_repo::get_by_id(&mut tx, ses_id).await?;
        session_repo::delete_by_id(&mut tx, ses_id).await?;
        
        
        tx.commit().await?;
        
        let mut conn = state.redis.get().await?;
        let created_ts = session_token
        .created_at
        .and_utc()
        .timestamp_millis();
    
        let key = format!(
            "session_active:{}:{}",
            user_id, created_ts
        );
        let _: usize = conn.del(key).await?;

        return Ok(())
    }

}
