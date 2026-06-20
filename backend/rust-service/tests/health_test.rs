mod common;

use serde_json::json;

use crate::common::TestContext;

/// Test the health check endpoint of the API.
#[tokio::test]
async fn test_health_check_endpoint() {
    let ctx = TestContext::new(None).await;

    let response = ctx.server.get("/v1/health").await;

    response.assert_status_ok();

    response.assert_json(&json!({
        "status": "healthy"
    }));
}
