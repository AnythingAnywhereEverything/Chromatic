use std::sync::Arc;
use axum_test::TestServer;

use chromatic::{
    api::server::create_router,
    application::{app::build_state, config, state::AppState},
};

pub struct TestContext {
    pub server: TestServer,
    pub state: Arc<AppState>,
}

impl TestContext {
    pub async fn clear(&self) {
        sqlx::query("TRUNCATE TABLE users, sessions RESTART IDENTITY CASCADE;")
            .execute(&self.state.db_pool)
            .await
            .expect("failed to clear database");
    }
}

pub async fn get_test_context() -> TestContext {
    let test_config = config::load(Some(".env.test"));
    let state = build_state(test_config).await;
    let router = create_router(state.clone()).await;

    let server = TestServer::new(router);

    TestContext {
        server,
        state,
    }
}