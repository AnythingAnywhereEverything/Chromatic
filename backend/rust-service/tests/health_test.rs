mod common;

use serde_json::json;

/// Test the health check endpoint of the API.
#[tokio::test]
async fn test_health_check_endpoint() {
    let ctx = common::get_test_context().await;
    ctx.clear().await;

    let response = ctx.server.get("/v1/health").await;

    response.assert_status_ok();

    response.assert_json(&json!({
        "status": "healthy"
    }));
}
