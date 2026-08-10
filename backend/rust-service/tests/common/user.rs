#![allow(dead_code)]

use crate::common::TestContext;
use serde_json::json;

pub async fn create_user(ctx: &TestContext, username: &str, email: &str, password: &str) -> String {
    let payload = json!({
        "username": username,
        "email": email,
        "password": password
    });
    let response = ctx.server.post("/v1/auth/register").json(&payload).await;
    response.assert_status_ok();

    let login_payload = json!({
        "username_or_email": username,
        "password": password
    });
    let login_response = ctx.server.post("/v1/auth/login").json(&login_payload).await;
    login_response.assert_status_ok();

    let login_json: serde_json::Value = login_response.json::<serde_json::Value>();
    let token = login_json["token"].as_str().unwrap();
    token.to_string()
}