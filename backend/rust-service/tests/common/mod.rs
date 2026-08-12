#![allow(dead_code)]

pub mod media;
/// * ------------------------
/// * Common test utilities and helpers for the Chromatic backend service.
/// * ------------------------
pub mod user;
use axum_test::TestServer;
use chromatic::{
    api::server::create_router,
    application::{app::build_state, config},
};
use rs_vips::Vips;
use sqlx::PgPool;
/// * ------------------------
use std::{fs, path::Path, sync::OnceLock};

pub struct TestContext {
    pub server: TestServer,
    pub testing_job_id: String,
}

static VIPS_APP: OnceLock<Vips> = OnceLock::new();

fn init_libvips() {
    VIPS_APP.get_or_init(|| {
        Vips::init("Chromatic Tests").expect("Failed to initialize libvips for tests");

        Vips::concurrency_set(2);
        Vips
    });
}

#[macro_export]
macro_rules! db_test {
    ($name:ident, $body:expr) => {
        #[sqlx::test]
        async fn $name(pool: sqlx::PgPool) {
            $body(pool).await
        }
    };
}

impl TestContext {
    pub async fn new(pool: Option<PgPool>, test_storage_id: Option<&str>) -> Self {
        init_libvips();

        let mut test_config = config::load();

        let test_id = test_storage_id.unwrap_or(uuid::Uuid::new_v4().to_string().as_str()).to_string();

        // create test directories for media and temp storage
        fs::create_dir_all(format!(
            "tests/media/{}",
            test_id
        ))
        .unwrap();
        fs::create_dir_all(format!(
            "tests/temp/{}",
            test_id
        ))
        .unwrap();

        test_config.media_root = format!("tests/media/{}", test_id);
        test_config.media_temp_root =
            format!("tests/temp/{}", test_id);

        let state = build_state(test_config, pool).await;

        let router = create_router(state.clone()).await;
        let server = TestServer::new(router);

        TestContext {
            server,
            testing_job_id: test_id,
        }
    }
}

impl Drop for TestContext {
    fn drop(&mut self) {
        // Clean up test directories after tests
        let media_dir = format!(
            "tests/media/{}",
            &self.testing_job_id
        );
        let media_path = Path::new(&media_dir);
        if media_path.exists() {
            fs::remove_dir_all(media_path).unwrap();
        }

        let temp_dir = format!(
            "tests/temp/{}",
            &self.testing_job_id
        );
        let temp_path = Path::new(&temp_dir);
        if temp_path.exists() {
            fs::remove_dir_all(temp_path).unwrap();
        }
    }
}
