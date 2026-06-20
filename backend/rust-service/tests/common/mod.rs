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
    pub async fn new(pool: Option<PgPool>) -> Self {

        let test_config = config::load();
        let state = build_state(test_config, pool).await;

        let router = create_router(state.clone()).await;
        let server = TestServer::new(router);

        TestContext { server }
    }
}