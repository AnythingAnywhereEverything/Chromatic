use axum_test::multipart::{MultipartForm, Part as AxPart};
use sqlx::PgPool;

use crate::common::{TestContext, user::create_user};

/// * --------------------------------------------------------
/// * Create Post With Text And All Visibility Type Tests
/// * --------------------------------------------------------

#[sqlx::test]
async fn test_create_post(pool: PgPool) {
    let test_id = "test_create_post_normal";
    let ctx = TestContext::new(Some(pool), Some(test_id)).await;
    let token = create_user(
        &ctx,
        "testuser",
        "example@gmail.com",
        "VeryStrongPassword123!",
    )
    .await;

    let test_cases = vec![
    ("Hello world", "Everyone"),
    ("Hello", "Friend"),
    ("World", "Private"),
    ];

    for (content, visibility) in test_cases {
        let form = MultipartForm::new()
            .add_text("content", content)
            .add_text("visibility", visibility);

        let response = ctx
            .server
            .post("/v2/posts/new")
            .add_header("token", &token)
            .multipart(form)
            .await;

        // uncomment to see the json
        // let json_response: serde_json::Value = response.json();    
        // println!("Response for '{}': {:#?}", content, json_response);

        response.assert_status_ok();
    }

}

/// * --------------------------------------------------------
/// * Create Post With Images and Gifs
/// * --------------------------------------------------------
#[sqlx::test]
async fn test_create_post_with_image(pool: PgPool) {
    let test_id = "test_create_post_with_image";
    let ctx = TestContext::new(Some(pool), Some(test_id)).await;
    let token = create_user(
        &ctx,
        "testuser",
        "example@gmail.com",
        "VeryStrongPassword123!",
    )
    .await;

    let file_content = std::fs::read("tests/fixtures/test_avatar.jpg").unwrap();
    let part = AxPart::bytes(file_content).file_name("test_avatar.jpg");

    let form = MultipartForm::new()
        .add_text("content", "Hello world")
        .add_part("media_src", part)
        .add_text("visibility", "Everyone");
        
    let response = ctx
        .server
        .post("/v2/posts/new")
        .add_header("token", &token)
        .multipart(form)
        .await;
    
     response.assert_status_ok();

    let file_content = std::fs::read("tests/fixtures/test_avatar.jpg").unwrap();
    let part = AxPart::bytes(file_content).file_name("test_avatar.jpg");

    let file_content = std::fs::read("tests/fixtures/test_avatar.gif").unwrap();
    let gif_part = AxPart::bytes(file_content)
        .file_name("test_avatar.gif")
        .mime_type("image/gif");

    
    let form = MultipartForm::new()
        .add_text("content", "Hello world")
        .add_part("media_src", part)
        .add_part("media_src", gif_part)
        .add_text("visibility", "Everyone");
    
    let reponse = ctx
        .server
        .post("/v2/posts/new")
        .add_header("token", token)
        .multipart(form)
        .await;
    reponse.assert_status_ok();

    // uncomment to see the json
    // let json_response: serde_json::Value = reponse3.json();    
    // println!("Response for {:#?}", json_response);
}

#[sqlx::test]
async fn test_create_post_fail_on_fake_img(pool: PgPool) {
    let test_id = "test_create_post_fail_on_fake_img";
    let ctx = TestContext::new(Some(pool), Some(test_id)).await;
    let token = create_user(
        &ctx,
        "testuser",
        "example@gmail.com",
        "VeryStrongPassword123!",
    )
    .await;

    let file_content = std::fs::read("tests/fixtures/test_fake_img.png")
        .expect("Failed to read test fixture");
    let part = AxPart::bytes(file_content).file_name("test_fake_img.png");

    let form = MultipartForm::new()
        .add_text("content", "Hello world")
        .add_part("media_src", part)
        .add_text("visibility", "Everyone");

    let response = ctx
        .server
        .post("/v2/posts/new")
        .add_header("token", token)
        .multipart(form)
        .await;

    let json: serde_json::Value = response.json::<serde_json::Value>();
    println!("Json values {:?}", json);
    response.assert_status_bad_request();
}
