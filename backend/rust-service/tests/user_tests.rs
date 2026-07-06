mod common;

use axum_test::multipart::{MultipartForm, Part};
use libvips::VipsImage;
use serde_json::json;
use sqlx::PgPool;

use std::{sync::Once};
use tracing_subscriber::{EnvFilter, fmt, prelude::*};

static INIT: Once = Once::new();

// ignore dead_code warnings for init_tracing function
#[allow(dead_code)]
async fn init_tracing() {
    INIT.call_once(|| {
        let filter_layer =
            EnvFilter::try_from_default_env().unwrap_or_else(|_| "chromatic=trace".into());
        let fmt_layer = fmt::layer()
            .compact()
            .with_target(false)
            .with_file(true)
            .with_line_number(true)
            .with_test_writer(); // Use test writer for capturing logs in tests
        tracing_subscriber::registry()
            .with(filter_layer)
            .with(fmt_layer)
            .init();
    });
}

use crate::common::TestContext;

async fn create_user(ctx: &TestContext, username: &str, email: &str, password: &str) -> String {
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

/// test getting user profile
#[sqlx::test]
async fn test_profile_get_user_profile(pool: PgPool) {
    let ctx = TestContext::new(Some(pool), None).await;

    let token = create_user(
        &ctx,
        "testuser1",
        "example@gmail.com",
        "VeryStrongPassword123!",
    )
    .await;

    // Now, use the token to get the user profile
    let profile_response = ctx
        .server
        .get("/v1/users/me")
        .add_header("token", token) // we dont use JWTs
        .await;
    profile_response.assert_status_ok();
}

#[sqlx::test]
async fn test_profile_get_user_profile_without_token(pool: PgPool) {
    let ctx = TestContext::new(Some(pool), None).await;

    // Attempt to get the user profile without a token
    let profile_response = ctx.server.get("/v1/users/me").await;
    profile_response.assert_status_unauthorized();
}

#[sqlx::test]
async fn test_profile_update_user_avatar(pool: PgPool) {
    let test_id = "test_profile_update_user_avatar";
    let ctx = TestContext::new(Some(pool), Some(test_id)).await;

    let token = create_user(
        &ctx,
        "testuser2",
        "example@gmail.com",
        "VeryStrongPassword123!",
    )
    .await;

    let file_content = std::fs::read("tests/fixtures/test_avatar.jpg").unwrap();
    let part = Part::bytes(file_content).file_name("test_avatar.jpg");

    let payload = json!({
        "position_x": 0.0,
        "position_y": 0.5,
        "scale": 0.5
    });

    let form = MultipartForm::new()
        .add_text("payload", payload.to_string()) // crop 256x256 from center of 512x512 image
        .add_part("file", part);

    let avatar_response = ctx
        .server
        .patch("/v1/users/me/avatar")
        .add_header("token", token) // we dont use JWTs
        .multipart(form)
        .await;
    avatar_response.assert_status_ok();
    ctx.file_cleanup(Some(test_id)).await; // Clean up test files after the test
}

#[sqlx::test]
async fn test_profile_update_user_profile_gif(pool: PgPool) {
    let test_id = "test_profile_update_user_profile_gif";
    let ctx = TestContext::new(Some(pool), Some(test_id)).await;

    let token = create_user(
        &ctx,
        "testuser",
        "example@gmail.com",
        "VeryStrongPassword123!",
    )
    .await;

    let file_content = std::fs::read("tests/fixtures/test_avatar.gif").unwrap();
    let part = Part::bytes(file_content)
        .file_name("test_avatar.gif")
        .mime_type("image/gif");

    let payload = json!({
        "position_x": 0.0,
        "position_y": 0.0,
        "scale": 0.5
    });

    let form = MultipartForm::new()
        .add_text("payload", payload.to_string())
        .add_part("file", part);

    let avatar_response = ctx
        .server
        .patch("/v1/users/me/avatar")
        .add_header("token", token) // we dont use JWTs
        .multipart(form)
        .await;
    avatar_response.assert_status_ok();
    ctx.file_cleanup(Some(test_id)).await; // Clean up test files after the test
}

#[sqlx::test]
async fn test_profile_update_user_profile_out_of_bounds_ok(pool: PgPool) {
    let test_id = "test_profile_update_user_profile_out_of_bounds";
    let ctx = TestContext::new(Some(pool), Some(test_id)).await;

    let token = create_user(
        &ctx,
        "testuser",
        "example@gmail.com",
        "VeryStrongPassword123!",
    )
    .await;

    let file_content = std::fs::read("tests/fixtures/test_avatar.jpg").unwrap();
    let part = Part::bytes(file_content).file_name("test_avatar.jpg");

    let payload = json!({
        "position_x": -2.0,
        "position_y": 2.0,
        "scale": 0.5
    });

    let form = MultipartForm::new()
        .add_text("payload", payload.to_string())
        .add_part("file", part);

    let avatar_response = ctx
        .server
        .patch("/v1/users/me/avatar")
        .add_header("token", token) // we dont use JWTs
        .multipart(form)
        .await;
    avatar_response.assert_status_ok();
    ctx.file_cleanup(Some(test_id)).await; // Clean up test files after the test
}

#[sqlx::test]
async fn test_profile_update_user_profile_invalid_scale(pool: PgPool) {
    let test_id = "test_profile_update_user_profile_invalid_scale";
    let ctx = TestContext::new(Some(pool), Some(test_id)).await;

    let token = create_user(
        &ctx,
        "testuser",
        "example@gmail.com",
        "VeryStrongPassword123!",
    )
    .await;

    let file_content = std::fs::read("tests/fixtures/test_avatar.jpg").unwrap();
    let part = Part::bytes(file_content).file_name("test_avatar.jpg");

    let payload = json!({
        "position_x": 0.5,
        "position_y": 0.5,
        "scale": -1.0
    });

    let form = MultipartForm::new()
        .add_text("payload", payload.to_string())
        .add_part("file", part);

    let avatar_response = ctx
        .server
        .patch("/v1/users/me/avatar")
        .add_header("token", token)
        .multipart(form)
        .await;
    avatar_response.assert_status_bad_request();
    ctx.file_cleanup(Some(test_id)).await;
}

#[sqlx::test]
async fn test_profile_update_user_profile_invalid_scale_zero(pool: PgPool) {
    let test_id = "test_profile_update_user_profile_invalid_scale_zero";
    let ctx = TestContext::new(Some(pool), Some(test_id)).await;

    let token = create_user(
        &ctx,
        "testuser",
        "example@gmail.com",
        "VeryStrongPassword123!",
    )
    .await;

    let file_content = std::fs::read("tests/fixtures/test_avatar.jpg").unwrap();
    let part = Part::bytes(file_content).file_name("test_avatar.jpg");

    let payload = json!({
        "position_x": 0.5,
        "position_y": 0.5,
        "scale": 0.0
    });

    let form = MultipartForm::new()
        .add_text("payload", payload.to_string())
        .add_part("file", part);

    let avatar_response = ctx
        .server
        .patch("/v1/users/me/avatar")
        .add_header("token", token)
        .multipart(form)
        .await;
    avatar_response.assert_status_bad_request();
    ctx.file_cleanup(Some(test_id)).await;
}

/// Test updating user profile with a randomly generated image and verifying the avatar image dimensions
/// This test generates a random image, uploads it as the user's avatar, and then verifies that the resulting avatar image dimensions match the expected dimensions based on the crop and scale parameters.
#[sqlx::test]
async fn test_profile_update_user_random_position(pool: PgPool) {
    let test_id = "test_profile_update_user_random_position";
    let ctx = TestContext::new(Some(pool), Some(test_id)).await;

    let token = create_user(
        &ctx,
        "testuser",
        "example@gmail.com",
        "VeryStrongPassword123!",
    )
    .await;

    let (generated_image, verification) = TestContext::generate_testimage((1,1)).await;
    let part = Part::bytes(generated_image.jpeg_bytes).file_name("test_avatar.jpg");

    let payload = json!({
        "position_x": generated_image.normalized_position.0,
        "position_y": generated_image.normalized_position.1,
        "scale": generated_image.normalized_scale
    });

    let form = MultipartForm::new()
        .add_text("payload", payload.to_string())
        .add_part("file", part);

    let avatar_response = ctx
        .server
        .patch("/v1/users/me/avatar")
        .add_header("token", &token)
        .multipart(form)
        .await;
    avatar_response.assert_status_ok();

    // get the user profile and check the avatar media data
    let profile_response = ctx
        .server
        .get("/v1/users/me")
        .add_header("token", token)
        .await;
    profile_response.assert_status_ok();

    // check the avatar media data in the profile response
    let profile_json: serde_json::Value = profile_response.json::<serde_json::Value>();
    let avatar_media = &profile_json["avatar_media_id"];

    // get path of the avatar media from avatar_media object within the test storage directory
    let avatar_media_url = avatar_media["media_url"].as_str().unwrap();

    // read the media data directly from local storage to verify the image size
    let avatar_file_path = format!("tests/media/{}/{}", test_id, avatar_media_url);
    let avatar_file_bytes = std::fs::read(&avatar_file_path).unwrap();
    let avatar_image = VipsImage::new_from_buffer(&avatar_file_bytes, "").unwrap();
    let avatar_image_dimensions: (i32, i32) = (avatar_image.get_width(), avatar_image.get_height());

    // check the avatar image dimensions against the expected dimensions from the verification
    assert_eq!(avatar_image_dimensions, verification.expected_size);
    
    ctx.file_cleanup(Some(test_id)).await;
}
