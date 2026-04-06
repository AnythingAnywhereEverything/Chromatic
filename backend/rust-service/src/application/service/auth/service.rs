
use crate::{
    api::RequestAuth, application::{
        repository::{auth, user},
        security::argon,
        service::{auth::types::AuthResponse, errors::AuthServiceError, session_service::SessionService},
        state::AppState,
    }, domain::user::{User, types::{DisplayName, Email, Username}}
};

pub struct AuthService;

impl AuthService {
    pub async fn oauth(
        state: &AppState,
        provider: &str,
        provider_user_id: &str,
        email: &str,
        
    ) -> Result<i64, AuthServiceError> {
        let mut tx = state.db_pool.begin().await?;

        if let Some(user_oauth_row) =
            auth::oauth::find_by_provider_and_user_id(&mut tx, provider, provider_user_id).await?
        {
            tx.commit().await?;
            return Ok(user_oauth_row.user_id);
        }

        let user_id = if let Ok(existing_user) = user::find::by_email(&mut tx, email).await {
            existing_user.id
        } else {
            let new_user_id = state.snowflake_generator.generate_id()?;

            // * generate username from email, if existed, leave it blank and let user update it later
            let base_username = Username::from_email(email)?;

            let mut user = User {
                id: new_user_id,
                email: Email::new(email)?,
                username: Some(base_username.clone()),
            };

            // * validate username, if exist
            if user::check::username_taken(&mut tx, base_username.as_str()).await? {
                tracing::warn!(
                    "Generated username {} from email {} is already taken, fallback to blank username",
                    base_username.as_str(),
                    email
                );

                user.username = None;
            }

            match user::create::user(&mut tx, &user).await {
                Ok(_) => {
                    // create default profile for the new user
                    user::create::user_profile(
                        &mut tx,
                        new_user_id,
                        None,
                        None,
                        None,
                    ).await?;

                    new_user_id
                },
                Err(_) => {
                    tracing::warn!(
                        "Failed to create user with generated username {}, fallback to blank username",
                        base_username.as_str()
                    );

                    user.username = None;

                    user::create::user(&mut tx, &user).await?;

                    new_user_id
                }
            }
        };

        let oauth_id = state.snowflake_generator.generate_id()?;

        auth::oauth::link_oauth_account(&mut tx, oauth_id, user_id, provider, provider_user_id)
            .await?;

        tx.commit().await?;

        // * create session for the user
        

        Ok(user_id)
    }

    pub async fn logout(state: &AppState, token: &str) -> Result<(), AuthServiceError> {
        SessionService::delete_session_token(state, token)
            .await
            .map_err(|_| AuthServiceError::LogoutFailed)?;
        Ok(())
    }

    pub async fn register(
        state: &AppState,
        display_name: &Option<String>,
        username: &str,
        password: &str,
        email: &str,
    ) -> Result<(), AuthServiceError> {
        let mut tx = state.db_pool.begin().await?;
        let user_id = state.snowflake_generator.generate_id()?;

        let username = Username::new(&username)?;
        if user::check::username_taken(&mut tx, &username.as_str()).await? {
            return Err(AuthServiceError::UsernameAlreadyTaken);
        }

        let email = Email::new(&email)?;
        if user::check::email_taken(&mut tx, &email.as_str()).await? {
            return Err(AuthServiceError::EmailAlreadyRegistered);
        }

        let user = User {
            id: user_id,
            email,
            username: Some(username),
        };

        user::create::user(&mut tx, &user).await?;
        
        // * Prepare credential
        let password_hash =
            argon::hash(password.as_bytes()).map_err(|_| AuthServiceError::UnableToHashSession)?;

        auth::credential::create_credential(&mut tx, user_id, &password_hash).await?;

        // * Prepare profile
        user::create::user_profile(
            &mut tx,
            user_id,
            display_name.as_ref().map(|d| DisplayName::new(d)).transpose()?,
            None,
            None,
        ).await?;
        
        tx.commit().await?;
        Ok(())
    }

    pub async fn login(
        state: &AppState,
        username_or_email: &str,
        input_password: &str,
        auth_header: RequestAuth,
    ) -> Result<AuthResponse, AuthServiceError> {
        let mut tx = state.db_pool.begin().await?;

        tracing::trace!("Attempting login for identifier: {}", username_or_email);

        if let Ok(user) = auth::credential::find_login_user_by_username_or_email(&mut tx, username_or_email).await {
            tracing::trace!("User found for identifier {}: {}", username_or_email, user.username.as_deref().unwrap_or("N/A"));
            
            tracing::trace!("Verifying password for user: {}", user.username.as_deref().unwrap_or("N/A"));

            let is_password_match = argon::verify(
                input_password.as_bytes(),
                &user.password_hash.unwrap_or_default(),
            )
            .expect("Username or password invalid");

            if user.is_active && is_password_match {
                tracing::trace!("access granted, user: {}", user.username.as_deref().unwrap_or("N/A"));

                // create session
                let token = SessionService::create_session(
                    &state,
                    user.id,
                    &auth_header.user_agent,
                    &auth_header.ip_address,
                    60 * 60,
                )
                .await?;

                let response = AuthResponse {
                    token: token.full_token,
                    user_id: user.id,
                };

                return Ok(response);
            }
        }

        Err(AuthServiceError::InvalidCredentials)?
    }

    //TODO: implement Email Token
}
