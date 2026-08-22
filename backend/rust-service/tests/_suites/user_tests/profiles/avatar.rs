use crate::common::{TestContext, user::create_user};
use crate::common::media::generate_testimage;
use axum_test::multipart::{MultipartForm, Part};
use rs_vips::VipsImage;
use sqlx::PgPool;

#[sqlx::test]
async fn test_upload_avatar(pool: PgPool) {
    let test_id = "test_upload_avatar";
    let mut ctx = TestContext::new(Some(pool), Some(test_id)).await;
    let user = create_user(
        &mut ctx,
        "testuser",
        "example@gmail.com",
        "VeryStrongPassword123!",
    ).await;
}