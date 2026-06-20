use std::sync::Arc;

use crate::{
    api::server,
    application::{config, service::{media::service::MediaService, snowflake_service::{SnowflakeGenerator, SnowflakeKind}}, state::AppState},
    infrastructure::{database::Database, redis},
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

    let media_service = MediaService::new(
        SnowflakeGenerator::new(config.server_worker_id, SnowflakeKind::Image).expect("Failed to create snowflake generator for media"),
        config.clone(), 
    );

    // Build the application state.
    Arc::new(AppState {
        config,
        db_pool,
        redis,
        snowflake_generator,
        media_service
    })
}

pub async fn run() {

    let config = config::load();
    let shared_state = build_state(config, None).await;

    server::start(shared_state).await;
}
