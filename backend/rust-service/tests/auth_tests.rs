mod common;

use serde_json::json;
use sqlx::PgPool;

use crate::common::TestContext;

/// test user registration with invalid data
#[sqlx::test]
async fn test_user_registration_with_invalid_data(pool: PgPool) {
    let ctx = TestContext::new(Some(pool)).await;

    let invalid_payload = json!({
        "username": "",
        "email": "invalidemail",
        "password": ""
    });
    let response = ctx
        .server
        .post("/v1/auth/register")
        .json(&invalid_payload)
        .await;
    response.assert_status_bad_request();
}

/// Test duplicate user registration with the same username or email.
#[sqlx::test]
async fn test_user_duplicate_registration(pool: PgPool) {
    let ctx = TestContext::new(Some(pool)).await;

    let payload = json!({
        "username": "testuser1",
        "email": "testuser@example.com",
        "password": "VeryStrongPassword123!"
    });
    let response = ctx.server.post("/v1/auth/register").json(&payload).await;
    response.assert_status_ok();

    // Try to register the same user again
    let response = ctx.server.post("/v1/auth/register").json(&payload).await;
    response.assert_status_conflict();
}

/// Test user login with invalid credentials.
#[sqlx::test]
async fn test_user_login_with_invalid_credentials(pool: PgPool) {
    let ctx = TestContext::new(Some(pool)).await;

    let login_payload = json!({
        "username_or_email": "nonexistentuser",
        "password": "wrongpassword"
    });
    let response = ctx.server.post("/v1/auth/login").json(&login_payload).await;
    response.assert_status_unauthorized();
}

/// Test user registration endpoints.
#[sqlx::test]
async fn test_user_registration(pool: PgPool) {
    let ctx = TestContext::new(Some(pool)).await;

    let registeration_payload = json!({
        "username": "testuser2",
        "email": "testuser2@example.com",
        "password": "VeryStrongPassword123!"
    });
    let registration_response = ctx
        .server
        .post("/v1/auth/register")
        .json(&registeration_payload)
        .await;
    registration_response.assert_status_ok();
}

/// Test user login via email and username.
#[sqlx::test]
async fn test_user_login_via_email_and_username(pool: PgPool) {
    let ctx = TestContext::new(Some(pool)).await;

    let registeration_payload = json!({
        "username": "testuser3",
        "email": "testuser3@example.com",
        "password": "VeryStrongPassword123!"
    });
    let registration_response = ctx
        .server
        .post("/v1/auth/register")
        .json(&registeration_payload)
        .await;
    registration_response.assert_status_ok();

    // Test login via email

    let login_payload = json!({
        "username_or_email": "testuser3@example.com",
        "password": "VeryStrongPassword123!"
    });

    let login_response = ctx.server.post("/v1/auth/login").json(&login_payload).await;
    login_response.assert_status_ok();

    // Test login via username
    let login_payload = json!({
        "username_or_email": "testuser3",
        "password": "VeryStrongPassword123!"
    });
    let login_response = ctx.server.post("/v1/auth/login").json(&login_payload).await;
    login_response.assert_status_ok();
}

// Test insert various invalid registration payloads and assert that they are rejected with appropriate error messages.
#[sqlx::test]
async fn test_user_registration_with_various_invalid_payloads(pool: PgPool) {
    let ctx = TestContext::new(Some(pool)).await;

    let invalid_payloads = vec![
        json!({
            "username": "",
            "email": "invalidemail",
            "password": ""
        }),
        json!({
            "username": "validusername",
            "email": "invalidemail",
            "password": "validpassword"
        }),
        json!({
            "username": "validusername",
            "email": "  ",
            "password": "validpassword"
        }),
        json!({
            "username": "validusername",
            "email": "invalidemail",
            "password": "  "
        }),
        json!({
            "username": "validusername",
            "email": "invalidemail",
            "password": "weakpassword"
        }),
    ];

    for payload in invalid_payloads {
        let response = ctx.server.post("/v1/auth/register").json(&payload).await;
        response.assert_status_bad_request();
    }
}

/// Test username with special characters and assert that it is rejected.
#[sqlx::test]
async fn test_user_registration_with_special_characters_in_username(pool: PgPool) {
    let ctx = TestContext::new(Some(pool)).await;

    let payload = json!({
        "username": "invalid$username",
        "email": "testuser@example.com",
        "password": "VeryStrongPassword123!"
    });
    let response = ctx.server.post("/v1/auth/register").json(&payload).await;
    response.assert_status_bad_request();
}

/// Test weak password and assert that it is rejected.
#[sqlx::test]
async fn test_user_registration_with_weak_password(pool: PgPool) {
    let ctx = TestContext::new(Some(pool)).await;

    let payload = json!({
        "username": "testuser4",
        "email": "testuser4@example.com",
        "password": "weakpassword"
    });
    let response = ctx.server.post("/v1/auth/register").json(&payload).await;
    response.assert_status_bad_request();
}

// Test short password and assert that it is rejected.
#[sqlx::test]
async fn test_user_registration_with_short_password(pool: PgPool) {
    let ctx = TestContext::new(Some(pool)).await;

    let payload = json!({
        "username": "testuser5",
        "email": "testuser5@example.com",
        "password": "short"
    });
    let response = ctx.server.post("/v1/auth/register").json(&payload).await;
    response.assert_status_bad_request();
}

/// Test long but invalid password and assert that it is rejected.
#[sqlx::test]
async fn test_user_registration_with_long_but_invalid_password(pool: PgPool) {
    let ctx = TestContext::new(Some(pool)).await;

    let payload = json!({
        "username": "testuser6",
        "email": "testuser6@example.com",
        "password": "ThisIsAVeryLongPasswordThatExceedsTheMaximumLengthAllowed"
    });
    let response = ctx.server.post("/v1/auth/register").json(&payload).await;
    response.assert_status_bad_request();
}
