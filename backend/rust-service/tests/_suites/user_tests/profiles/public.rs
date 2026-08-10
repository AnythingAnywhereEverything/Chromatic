use crate::common::{TestContext, user::create_user};
use sqlx::PgPool;



#[sqlx::test]
async fn get_user_profile(pool: PgPool) {
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
async fn get_user_profile_no_token(pool: PgPool) {
    let ctx = TestContext::new(Some(pool), None).await;

    // Attempt to get the user profile without a token
    let profile_response = ctx.server.get("/v1/users/me").await;
    profile_response.assert_status_unauthorized();
}
