use axum_test::multipart::MultipartForm;
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

    let form = MultipartForm::new()
    .add_text("content", "Hello world")
    .add_text("visibility", "Everyone");

    let create_post_reponse = ctx
    .server
    .post("/v2/posts/new")
    .add_header("token", &token)
    .multipart(form)
    .await;

    create_post_reponse.assert_status_ok();

    let post_json = create_post_reponse.json::<serde_json::Value>();

    println!("Post JSON: {}", post_json);
}
