use std::sync::Arc;
use crate::{
    api::server, application::{config, service::{media::{multipart_ex::MultipartExtractor, service::MediaService, storage::{MediaStorage, local::LocalStorage, r2::R2Storage}}, snowflake_service::{SnowflakeGenerator, SnowflakeKind}}, state::AppState}, infrastructure::{database::Database, redis},
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
    let storage: Arc<dyn MediaStorage> = match driver.as_str() {
        "r2" => {
            let r2 = R2Storage::new();
            Arc::new(r2)
        }
        _ => {
            let root = config.clone().media_root;
            let temp_root = config.clone().media_temp_root;
            Arc::new(LocalStorage::new(root, temp_root))
        }
    };

    let media_service = MediaService::new(
        SnowflakeGenerator::new(config.server_worker_id, SnowflakeKind::Image).expect("Failed to create snowflake generator for media"),
        storage.clone(),
        db_pool.clone()
    );

    let multipart_extractor = MultipartExtractor::new(storage);

    // Build the application state.
    Arc::new(AppState {
        config,
        db_pool,
        redis,
        snowflake_generator,
        media_service,
        multipart_extractor
    })
}

pub async fn run() {

    let config = config::load();
    let shared_state = build_state(config, None).await;

    server::start(shared_state).await;
}
