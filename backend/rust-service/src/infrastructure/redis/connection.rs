use deadpool_redis::{Config as RedisConfig, Pool, Runtime};
use crate::application::config::Config;

pub async fn open(config: &Config) -> Pool {
    let cfg = RedisConfig::from_url(config.redis_url());

    match cfg.create_pool(Some(Runtime::Tokio1)) {
        Ok(pool) => {
            tracing::info!("Connected to redis (deadpool)");
            pool
        }
        Err(e) => {
            tracing::error!("Could not create redis pool: {}", e);
            std::process::exit(1);
        }
    }
}