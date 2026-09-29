use std::sync::Arc;
use crate::{
    api::server, application::{config, repository::admin as admin_repo, service::{media::{extractor::MultipartExtractor, service::MediaService, storage::{LocalStorage, PersistentStore, TempStore}}, snowflake_service::{SnowflakeGenerator, SnowflakeKind}}, state::AppState}, infrastructure::{database::Database, redis},
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

    // Seed the first superuser, if one is configured and none exists yet.
    // Every user starts with `is_superuser = false` and there is no
    // admin-creation endpoint, so without this the panel is unreachable.
    //
    // Deliberately *after* the snowflake block: creating the account needs a
    // snowflake id. One is generated up front even for the promote/no-op
    // branches; wasting an id at boot is cheaper than running a second
    // existence check on the caller side. With `BOOTSTRAP_ADMIN_PASSWORD`
    // set the account is *created* when it does not exist, so a fresh database
    // has a working admin on first boot instead of needing a registration
    // plus a restart.
    //
    // Errors are logged rather than fatal: a database hiccup at boot should not
    // stop the API from serving, and the only thing lost is the panel. Failures
    // also cover a typo'd username or a password that breaks the validation
    // rules, which is exactly the failure that would otherwise leave the panel
    // silently unreachable.
    if let Some(username) = &config.bootstrap_admin {
        let user_id = snowflake_generator
            .generate_id()
            .expect("failed to generate snowflake id for the bootstrap admin");
        let outcome = admin_repo::create::bootstrap_create(
            &db_pool,
            username,
            config.bootstrap_admin_password.as_deref(),
            &config.bootstrap_admin_email,
            user_id,
        )
        .await;
        match outcome {
            admin_repo::create::BootstrapOutcome::Created => {
                tracing::warn!("created '{username}' as the first superuser via BOOTSTRAP_ADMIN")
            }
            admin_repo::create::BootstrapOutcome::Promoted => {
                tracing::warn!("promoted '{username}' to superuser via BOOTSTRAP_ADMIN")
            }
            admin_repo::create::BootstrapOutcome::Noop => {
                tracing::warn!(
                    "BOOTSTRAP_ADMIN='{username}' did not promote: a superuser already exists"
                )
            }
            admin_repo::create::BootstrapOutcome::Failed(error) => {
                tracing::error!("BOOTSTRAP_ADMIN bootstrap failed: {error}")
            }
        }
    }

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
