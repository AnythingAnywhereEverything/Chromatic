use std::sync::Arc;

use deadpool_redis::Pool;

use crate::{
    application::{
        config::Config,
        service::{
            media::{
                extractor::MultipartExtractor,
                service::MediaService,
                storage::{PersistentStore, TempStore},
            },
            snowflake_service::SnowflakeGenerator,
        },
    },
    infrastructure::database::DatabasePool,
};

pub type SharedState = Arc<AppState>;

pub struct AppState {
    pub config: Config,
    pub db_pool: DatabasePool,
    pub redis: Pool,
    pub snowflake_generator: SnowflakeGenerator,
    pub temporary_store: Arc<dyn TempStore>,
    pub persistent_store: Arc<dyn PersistentStore>,
    pub multi_extractor: MultipartExtractor,
    pub media_service: MediaService,
}
