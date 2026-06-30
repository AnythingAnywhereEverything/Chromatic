use std::{fs, path::Path};

use axum_test::TestServer;

use chromatic::{
    api::server::create_router,
    application::{app::build_state, config},
};
use sqlx::PgPool;


pub struct TestContext {
    pub server: TestServer,
}

impl TestContext {
    pub async fn new(pool: Option<PgPool>, test_storage_id: Option<&str>) -> Self {

        let mut test_config = config::load();

        // create test directories for media and temp storage
        fs::create_dir_all(format!("tests/media/{}", test_storage_id.unwrap_or("default"))).unwrap();
        fs::create_dir_all(format!("tests/temp/{}", test_storage_id.unwrap_or("default"))).unwrap();

        test_config.media_root = format!("tests/media/{}", test_storage_id.unwrap_or("default"));
        test_config.media_temp_root = format!("tests/temp/{}", test_storage_id.unwrap_or("default"));

        let state = build_state(test_config, pool).await;

        let router = create_router(state.clone()).await;
        let server = TestServer::new(router);

        TestContext { server }
    }

    #[allow(dead_code)]
    pub async fn file_cleanup(&self, test_storage_id: Option<&str>) {

        let media_dir = format!("tests/media/{}", test_storage_id.unwrap_or("default"));
        let media_path = Path::new(&media_dir);
        if media_path.exists() {
            fs::remove_dir_all(media_path).unwrap();
        }

        let temp_dir = format!("tests/temp/{}", test_storage_id.unwrap_or("default"));
        let temp_path = Path::new(&temp_dir);
        if temp_path.exists() {
            fs::remove_dir_all(temp_path).unwrap();
        }
    }
}