use std::sync::Arc;
use crate::{
    api::server, application::{config, service::{media::{extractor::MultipartExtractor, service::MediaService, storage::{LocalStorage, PersistentStore, TempStore}}, snowflake_service::{SnowflakeGenerator, SnowflakeKind}}, state::AppState}, infrastructure::{database::Database, redis},
};

pub async fn build_state(config: config::Config, db_pool: Option<sqlx::PgPool>) -> Arc<AppState> {
    // Load configuration.

    // Connect to Redis.
    let redis = redis::open(&config).await;

    // Connect to PostgreSQL.
    let db_pool = match db_pool {
        Some(pool) => pool,
        None => Database::connect(config.clone().into())
            .await
            .expect("Failed to connect to the database."),
    };

    // Run migrations.
    Database::migrate(&db_pool)
        .await
        .expect("Failed to run database migrations.");

    // Initialize snowflake generator.
    let snowflake_generator = SnowflakeGenerator::new(
        config.server_worker_id,
        SnowflakeKind::Api,
    )    
    .expect("Failed to create snowflake generator.");

    // initialize Medai service

    let driver = config.clone().media_driver;
    let persistent_store: Arc<dyn PersistentStore> = match driver.as_str() {
        "r2" => {
            panic!("R2 storage is not yet implemented.");
        }
        _ => {
            let root = config.clone().media_root;
            let temp_root = config.clone().media_temp_root;
            Arc::new(LocalStorage::new(&temp_root, &root))
        }
    };

    let temporary_store: Arc<dyn TempStore> = match driver.as_str() {
        "r2" => {
            panic!("R2 storage is not yet implemented.");
        }
        _ => {

            let root = config.clone().media_root;
            let temp_root = config.clone().media_temp_root;
            Arc::new(LocalStorage::new(&temp_root, &root)) as Arc<dyn TempStore>
        }
    };

    let media_service_snowflake = SnowflakeGenerator::new(
        config.server_worker_id,
        SnowflakeKind::Media,
    ).expect("Failed to create media service snowflake generator.");

    let media_service = MediaService::new(
        media_service_snowflake,
        temporary_store.clone(),
        persistent_store.clone(),
    );

    let multi_extractor = MultipartExtractor::new(temporary_store.clone());

    // Build the application state.
    Arc::new(AppState {
        config,
        db_pool,
        redis,
        snowflake_generator,
        multi_extractor,
        persistent_store,
        temporary_store,
        media_service,
    })
}

pub async fn run() {

    let config = config::load();
    let shared_state = build_state(config, None).await;

    server::start(shared_state).await;
}
