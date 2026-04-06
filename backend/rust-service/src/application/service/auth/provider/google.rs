use reqwest::Client;

use crate::application::service::auth::provider::types::GoogleUserInfo;


pub async fn fetch_google_userinfo(
    access_token: &str,
) -> Result<GoogleUserInfo, reqwest::Error> {

    let client = Client::new();

    let res = client
        .get("https://www.googleapis.com/oauth2/v3/userinfo")
        .bearer_auth(access_token)
        .send()
        .await?;

    res.json::<GoogleUserInfo>().await
}
