use crate::common::{TestContext, user::create_user};
use crate::common::media::generate_testimage;
use axum_test::multipart::{MultipartForm, Part};
use libvips::VipsImage;
use sqlx::PgPool;

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

    let (generated_image, verification) = generate_testimage((1,1)).await;
    let part = Part::bytes(generated_image.jpeg_bytes).file_name("test_avatar.jpg");

    let form = MultipartForm::new()
        .add_text("position_x", generated_image.normalized_position.0.to_string())
        .add_text("position_y", generated_image.normalized_position.1.to_string())
        .add_text("scale", generated_image.normalized_scale.to_string())
        .add_part("uploaded_avatar", part);

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
    // print the profile response for debugging

    // check the avatar media data in the profile response
    let profile_json = profile_response.json::<serde_json::Value>();

    println!("Profile JSON: {}", profile_json);

    let avatar_media = &profile_json["avatar_media_id"];

    // get path of the avatar media from avatar_media object within the test storage directory
    let avatar_media_url = avatar_media["path"].as_str().unwrap();

    // read the media data directly from local storage to verify the image size
    let avatar_file_path = format!("tests/media/{}/{}", test_id, avatar_media_url);
    println!("Avatar file path: {}", avatar_file_path);
    let avatar_file_bytes = std::fs::read(&avatar_file_path).unwrap();
    let avatar_image = VipsImage::new_from_buffer(&avatar_file_bytes, "").unwrap();
    let avatar_image_dimensions: (i32, i32) = (avatar_image.get_width(), avatar_image.get_height());

    // check the avatar image dimensions against the expected dimensions from the verification
    assert_eq!(avatar_image_dimensions, verification.expected_size);
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

    let form = MultipartForm::new()
        .add_text("position_x", 0.0)
        .add_text("position_y", 0.5)
        .add_text("scale", 0.5)
        .add_part("uploaded_avatar", part);

    let avatar_response: axum_test::TestResponse = ctx
        .server
        .patch("/v1/users/me/avatar")
        .add_header("token", token) // we dont use JWTs
        .multipart(form)
        .await;
    avatar_response.assert_status_ok();
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

    let form = MultipartForm::new()
        .add_text("position_x", 0.0)
        .add_text("position_y", 0.0)
        .add_text("scale", 0.5)
        .add_part("uploaded_avatar", part);

    let avatar_response = ctx
        .server
        .patch("/v1/users/me/avatar")
        .add_header("token", token) // we dont use JWTs
        .multipart(form)
        .await;
    avatar_response.assert_status_ok();
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

    let form = MultipartForm::new()
        .add_text("position_x", -2.0)
        .add_text("position_y", 2.0)
        .add_text("scale", 0.5)
        .add_part("uploaded_avatar", part);


    let avatar_response = ctx
        .server
        .patch("/v1/users/me/avatar")
        .add_header("token", token) // we dont use JWTs
        .multipart(form)
        .await;
    avatar_response.assert_status_ok();
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

    let form = MultipartForm::new()
        .add_text("position_x", 0.5)
        .add_text("position_y", 0.5)
        .add_text("scale", -1.0)
        .add_part("uploaded_avatar", part);

    let avatar_response = ctx
        .server
        .patch("/v1/users/me/avatar")
        .add_header("token", token)
        .multipart(form)
        .await;
    avatar_response.assert_status_bad_request();
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

    let form = MultipartForm::new()
        .add_text("position_x", 0.5)
        .add_text("position_y", 0.5)
        .add_text("scale", 0.0)
        .add_part("uploaded_avatar", part);

    let avatar_response = ctx
        .server
        .patch("/v1/users/me/avatar")
        .add_header("token", token)
        .multipart(form)
        .await;
    avatar_response.assert_status_bad_request();
}
