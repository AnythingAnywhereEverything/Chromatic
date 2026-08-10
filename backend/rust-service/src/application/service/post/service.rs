use crate::application::{service::{errors::PostServiceError, snowflake_service::SnowflakeGenerator}, state::AppState};

pub struct PostService {
    snowflake: SnowflakeGenerator,
    connection: sqlx::PgPool,
}

impl PostService {
    pub fn new(snowflake: SnowflakeGenerator, connection: sqlx::PgPool) -> Self {
        Self {
            snowflake,
            connection
        }
    }

    /// * 
    // todo: impl the image attachment
    pub async fn create_post(
        &self,
        user_id: i64,
        content: &str,
        status:  &str,
        // ? image: &str  
    ) -> Result<i64, PostServiceError> {

        
        let mut tx = self.connection.begin().await?;
        let post_id = self.snowflake.generate_id()?;

        
        Ok(post_id)
    }

    pub async fn update_post(
        state: &AppState,
        id: i64,
        user_id: i64,
        content: &str
    ) -> Result<(), PostServiceError> {
        let mut tx = state.db_pool.begin().await?;
        
        Ok(())
    }
}