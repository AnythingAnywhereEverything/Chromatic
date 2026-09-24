use redis::AsyncTypedCommands;

use crate::{
    application::{
        repository::{
            media::{self as media_repo, row::ProcessingState},
            user::{
                self as user_repo,
                find::URDQOpts,
                row::{FollowUserRow, PendingFollowRow, SettingsType, UserProfileRow, UserSettingRow},
            },
        },
        service::{
            errors::ProfileServiceError,
            media::{
                model::{
                    FileContainer,
                    container::{ContainerConfig, NamingStrategy, RecentMediaType},
                },
                processor::types::{CropStyle, ImageProcessorType, MediaProcessorOptions},
            },
        },
        state::AppState,
    },
    domain::user::types::{Bio, DisplayName, Quotes},
};
pub struct ProfileService;

impl ProfileService {
    pub async fn get_profile_username(
        state: &AppState,
        username: String,
        requester: Option<i64>,
    ) -> Result<UserProfileRow, ProfileServiceError> {
        let mut conn = state.redis.get().await?;
        let username_key = format!("profile:username:{}", username);

        if let Some(id_key) = conn.get(&username_key).await? {
            if let Some(value) = conn.get(&id_key).await? {
                conn.expire(&username_key, 3600).await?;
                conn.expire(&id_key, 3600).await?;
                let profile: UserProfileRow = serde_json::from_str(&value)?;
                return Ok(profile);
            } else {
                conn.del(&username_key).await?;
            }
        }

        let mut tx = state.db_pool.begin().await?;
        let profile =
            user_repo::find::profile_full_by_username(&mut tx, &username, requester).await?;
        tx.commit().await?;
        // cache the profile in Redis
        let id_key = format!("profile:id:{}", profile.id);
        let value = serde_json::to_string(&profile)?;
        let _: () = conn.set_ex(&username_key, &id_key, 3600).await?;
        let _: () = conn.set_ex(&id_key, &value, 3600).await?;

        Ok(profile)
    }

    pub async fn get_profile_by_id(
        state: &AppState,
        user_id: i64,
        requester: Option<i64>,
    ) -> Result<UserProfileRow, ProfileServiceError> {
        let mut conn = state.redis.get().await?;
        let id_key = format!("profile:id:{}", user_id);

        if let Some(value) = conn.get(&id_key).await? {
            conn.expire(&id_key, 3600).await?;
            let profile: UserProfileRow = serde_json::from_str(&value)?;
            return Ok(profile);
        }

        let mut tx: sqlx::Transaction<'_, sqlx::Postgres> = state.db_pool.begin().await?;
        let profile = user_repo::find::profile_full_by_id(&mut tx, user_id, requester).await?;
        tx.commit().await?;

        // cache the profile in Redis
        let username_key = format!("profile:username:{}", profile.username);
        let value = serde_json::to_string(&profile)?;
        let _: () = conn.set_ex(&id_key, &value, 3600).await?;
        let _: () = conn.set_ex(&username_key, &id_key, 3600).await?;

        Ok(profile)
    }

    pub async fn update_post_counts(
        state: &AppState,
        user_id: i64,
        increment: i32,
    ) -> Result<(), ProfileServiceError> {
        let mut tx = state.db_pool.begin().await?;
        user_repo::update::user_post_counts(&mut tx, user_id, increment).await?;
        tx.commit().await?;

        // update cache in Redis
        let mut conn = state.redis.get().await?;
        let id_key = format!("profile:id:{}", user_id);
        if let Some(value) = conn.get(&id_key).await? {
            let mut profile: UserProfileRow = serde_json::from_str(&value)?;
            profile.posts_count = Some(profile.posts_count.unwrap_or(0) + increment);
            let value = serde_json::to_string(&profile)?;
            let _: () = conn.set_ex(&id_key, &value, 3600).await?;
        }

        Ok(())
    }

    pub async fn get_profile_with_opts(
        state: &AppState,
        opts: URDQOpts,
    ) -> Result<UserProfileRow, ProfileServiceError> {
        let mut tx = state.db_pool.begin().await?;

        let profile = user_repo::find::experimental_dynamic_user_query(&mut tx, opts).await?;

        tx.commit().await?;

        Ok(profile)
    }

    pub async fn update_profile(
        state: &AppState,
        user_id: i64,
        display_name: Option<String>,
        bio: Option<String>,
        quote: Option<String>,
        uploaded_avatar: &mut Option<FileContainer>,
        remove_avatar: Option<bool>,
        uploaded_banner: &mut Option<FileContainer>,
        remove_banner: Option<bool>,
    ) -> Result<UserProfileRow, ProfileServiceError> {
        // if everything is None, return early
        if display_name.is_none()
            && bio.is_none()
            && quote.is_none()
            && uploaded_avatar.is_none()
            && remove_avatar.is_none()
            && uploaded_banner.is_none()
            && remove_banner.is_none()
        {
            return Err(ProfileServiceError::NoUpdateFields);
        }

        if uploaded_avatar.is_some() && remove_avatar.unwrap_or(false) {
            return Err(ProfileServiceError::InvalidAvatarUpdate);
        }
        if uploaded_banner.is_some() && remove_banner.unwrap_or(false) {
            return Err(ProfileServiceError::InvalidBannerUpdate);
        }

        // create pool connection and begin transaction after validation check so we dont have to create and drop instantly
        let mut tx = state.db_pool.begin().await?;
        // update display name
        if let Some(display_name) = display_name {
            // turn displayname to DisplayName type and validate
            let dpn = DisplayName::new(&display_name)?;
            user_repo::update::user_display_name(&mut tx, user_id, &dpn.as_str()).await?;
        }
        // update bio
        if let Some(bio) = bio {
            let bio = Bio::new(&bio)?;
            user_repo::update::user_bio(&mut tx, user_id, &bio.as_str()).await?;
        }
        // update quote
        if let Some(quote) = quote {
            let quote = Quotes::new(&quote)?;
            user_repo::update::user_status(&mut tx, user_id, &quote.as_str()).await?;
        }

        // commit fast, static data first, then handle media after commit to avoid long transactions
        tx.commit().await?;

        let media_base_options = ContainerConfig::new()
            .set_naming_strategy(NamingStrategy::FinalHash)
            .set_generate_thumbhash(true)
            .set_animated_image_indicator(true);

        // combine avatar and banner media handling into one transaction to avoid multiple transactions
        let mut to_upload = vec![];

        if let Some(avatar) = uploaded_avatar {
            let option = media_base_options.clone();

            avatar
                .set_uploader_id(user_id)
                .set_target_path(format!("avatars/{}", user_id))
                .set_config(
                    option
                        .set_check_conflict(RecentMediaType::Avatar)
                        .set_processing_options(MediaProcessorOptions::new().set_image_processors(
                            vec![ImageProcessorType::Crop {
                                style: CropStyle::Ratio {
                                    width: 1,
                                    height: 1,
                                    scale: 1.0,
                                },
                                position: Some((0.5, 0.5)),
                            }],
                        )),
                );

            to_upload.push(avatar);
        }

        if let Some(banner) = uploaded_banner {
            let option = media_base_options.clone();
            banner
                .set_uploader_id(user_id)
                .set_target_path(format!("banners/{}", user_id))
                .set_config(
                    option
                        .set_check_conflict(RecentMediaType::Banner)
                        .set_processing_options(MediaProcessorOptions::new().set_image_processors(
                            vec![ImageProcessorType::Crop {
                                style: CropStyle::Ratio {
                                    width: 5,
                                    height: 2,
                                    scale: 1.0,
                                },
                                position: Some((0.5, 0.5)),
                            }],
                        )),
                );

            to_upload.push(banner);
        }

        for container in to_upload {
            state.media_service.save_media(&state, container).await?;

            let mut tx = state.db_pool.begin().await?;

            let is_avatar = container.target_path()?.contains("avatars");

            for media in container.resolve_files() {
                media_repo::update::processing_state(
                    &mut tx,
                    &media.id,
                    &ProcessingState::Completed,
                )
                .await?;

                let media_to_delete = if is_avatar {
                    user_repo::update::avatar_media_id(&mut tx, user_id, Some(media.id)).await?
                } else {
                    user_repo::update::banner_media_id(&mut tx, user_id, Some(media.id)).await?
                };
                if let Some(media_id) = media_to_delete {
                    let object =
                        media_repo::delete::hard_delete_media_data(&mut tx, media_id).await?;
                    let path = format!("{}/{}", object.storage_key, object.name);
                    if path.contains("a_") {
                        let animated_path = path.replace(".png", ".webp");
                        state.persistent_store.delete(&animated_path).await?;
                    }
                    state.persistent_store.delete(&path).await?;
                }
            }

            tx.commit().await?;
        }

        let mut tx = state.db_pool.begin().await?;
        let profile = user_repo::find::profile_full_by_id(&mut tx, user_id, None).await?;

        // update cache
        let mut conn = state.redis.get().await?;
        let cache_key = format!("profile:id:{}", &profile.id);
        let cache_value = serde_json::to_string(&profile)?;
        conn.set_ex(&cache_key, &cache_value, 3600).await?;

        tx.commit().await?;

        Ok(profile)
    }
    pub async fn unfollow_user(
        state: &AppState,
        user_id: i64,
        target_id: i64,
    ) -> Result<(), ProfileServiceError> {
        let mut tx = state.db_pool.begin().await?;
        let result = user_repo::follow::unfollow_repo(&mut tx, target_id, user_id).await?;
        tracing::info!("Unfollow repository call result: {:?}", result);
        if result {
            user_repo::follow::update_follower_count(&mut tx, target_id, -1).await?;
            user_repo::follow::update_following_count(&mut tx, user_id, -1).await?;
        }
        tx.commit().await?;

        if result {
            let mut tx = state.db_pool.begin().await?;

            let user_profile = user_repo::find::profile_full_by_id(&mut tx, user_id, None).await?;

            let target_profile =
                user_repo::find::profile_full_by_id(&mut tx, target_id, None).await?;

            tx.commit().await?;

            let mut conn = state.redis.get().await?;

            let user_id_key = format!("profile:id:{}", user_profile.id);
            let user_username_key = format!("profile:username:{}", user_profile.username);
            let user_value = serde_json::to_string(&user_profile)?;

            let target_id_key = format!("profile:id:{}", target_profile.id);
            let target_username_key = format!("profile:username:{}", target_profile.username);
            let target_value = serde_json::to_string(&target_profile)?;

            let _: () = conn.set_ex(&user_id_key, &user_value, 3600).await?;

            let _: () = conn.set_ex(&user_username_key, &user_id_key, 3600).await?;

            let _: () = conn.set_ex(&target_id_key, &target_value, 3600).await?;

            let _: () = conn
                .set_ex(&target_username_key, &target_id_key, 3600)
                .await?;
        }

        Ok(())
    }

    pub async fn follow_user(
        state: &AppState,
        user_id: i64,
        target_id: i64,
        status: &str,
    ) -> Result<FollowUserRow, ProfileServiceError> {
        let mut tx = state.db_pool.begin().await?;
        tracing::info!(
            "Following user: {} -> {} with status: {}",
            user_id,
            target_id,
            status
        );
        let result = user_repo::follow::follow_repo(&mut tx, target_id, user_id, &status).await?;
        let notification_id = state.snowflake_generator.generate_id()?;
        tracing::info!("Follow repository call result: {:?}", result);
        let username = Self::get_profile_by_id(state, user_id, None)
            .await?
            .username;
        if status == "followed" {
            user_repo::follow::update_follower_count(&mut tx, target_id, 1).await?;
            user_repo::follow::update_following_count(&mut tx, user_id, 1).await?;

            user_repo::notification::create_notification(
                &mut tx,
                notification_id,
                target_id,
                "follow",
                serde_json::json!({
                    "username": username,
                    "message": "has followed you"
                }),
            )
            .await?;
        } else if status == "pending" {
            user_repo::notification::create_notification(
                &mut tx,
                notification_id,
                target_id,
                "follow_request",
                serde_json::json!({
                    "username": username,
                    "message": "has requested to follow you"
                }),
            )
            .await?;
        }

        tx.commit().await?;
        // A I
        if status == "followed" {
            let mut tx = state.db_pool.begin().await?;

            let user_profile = user_repo::find::profile_full_by_id(&mut tx, user_id, None).await?;

            let target_profile =
                user_repo::find::profile_full_by_id(&mut tx, target_id, None).await?;

            tx.commit().await?;

            let mut conn = state.redis.get().await?;

            let user_id_key = format!("profile:id:{}", user_profile.id);
            let user_username_key = format!("profile:username:{}", user_profile.username);
            let user_value = serde_json::to_string(&user_profile)?;

            let target_id_key = format!("profile:id:{}", target_profile.id);
            let target_username_key = format!("profile:username:{}", target_profile.username);
            let target_value = serde_json::to_string(&target_profile)?;

            let _: () = conn.set_ex(&user_id_key, &user_value, 3600).await?;

            let _: () = conn.set_ex(&user_username_key, &user_id_key, 3600).await?;

            let _: () = conn.set_ex(&target_id_key, &target_value, 3600).await?;

            let _: () = conn
                .set_ex(&target_username_key, &target_id_key, 3600)
                .await?;
        }
        Ok(result)
    }

    pub async fn init_user_settings(
        state: &AppState,
        user_id: i64,
    ) -> Result<(), ProfileServiceError> {
        let mut tx = state.db_pool.begin().await?;
        user_repo::setting::init_setting_message(&mut tx, user_id).await?;
        user_repo::setting::init_setting_privacy(&mut tx, user_id).await?;
        user_repo::setting::init_setting_notification(&mut tx, user_id).await?;
        tx.commit().await?;
        Ok(())
    }

    pub async fn get_user_setting(
        state: &AppState,
        user_id: i64,
        setting_key: SettingsType,
    ) -> Result<UserSettingRow, ProfileServiceError> {
        let mut tx = state.db_pool.begin().await?;
        let setting = user_repo::setting::get_setting_type(&mut tx, user_id, setting_key).await?;
        tx.commit().await?;
        Ok(setting)
    }

    pub async fn update_user_setting(
        state: &AppState,
        user_id: i64,
        setting_key: SettingsType,
        setting_value: serde_json::Value,
    ) -> Result<UserSettingRow, ProfileServiceError> {
        if !setting_value.is_object() {
            return Err(ProfileServiceError::InvalidSettingUpdate);
        }

        let mut tx = state.db_pool.begin().await?;
        let setting =
            user_repo::setting::update_setting_type(&mut tx, user_id, setting_key, setting_value)
                .await?;
        tx.commit().await?;
        Ok(setting)
    }

    pub async fn get_pending_follow_requests(
        state: &AppState,
        user_id: i64,
    ) -> Result<Vec<PendingFollowRow>, ProfileServiceError> {
        let mut tx = state.db_pool.begin().await?;
        let requests = user_repo::follow::list_pending_followers(&mut tx, user_id).await?;
        tx.commit().await?;
        Ok(requests)
    }

    pub async fn accept_follow_request(
        state: &AppState,
        owner_id: i64,
        follower_id: i64,
    ) -> Result<FollowUserRow, ProfileServiceError> {
        let mut tx = state.db_pool.begin().await?;
        tracing::info!(
            "Accepting follow request: {} accepts follower: {}",
            owner_id,
            follower_id
        );
        let result = user_repo::follow::follow_repo(&mut tx, owner_id, follower_id, "followed")
            .await?;
        let username = Self::get_profile_by_id(state, follower_id, None)
            .await?
            .username;
        let notification_id = state.snowflake_generator.generate_id()?;

        user_repo::follow::update_follower_count(&mut tx, owner_id, 1).await?;
        user_repo::follow::update_following_count(&mut tx, follower_id, 1).await?;

        user_repo::notification::create_notification(
            &mut tx,
            notification_id,
            owner_id,
            "follow",
            serde_json::json!({
                "username": username,
                "message": "has followed you"
            }),
        )
        .await?;

        tx.commit().await?;

        let mut tx = state.db_pool.begin().await?;
        let owner_profile = user_repo::find::profile_full_by_id(&mut tx, owner_id, None).await?;
        let follower_profile =
            user_repo::find::profile_full_by_id(&mut tx, follower_id, None).await?;
        tx.commit().await?;

        let mut conn = state.redis.get().await?;
        let owner_id_key = format!("profile:id:{}", owner_profile.id);
        let owner_username_key = format!("profile:username:{}", owner_profile.username);
        let owner_value = serde_json::to_string(&owner_profile)?;

        let follower_id_key = format!("profile:id:{}", follower_profile.id);
        let follower_username_key = format!("profile:username:{}", follower_profile.username);
        let follower_value = serde_json::to_string(&follower_profile)?;

        let _: () = conn.set_ex(&owner_id_key, &owner_value, 3600).await?;
        let _: () = conn.set_ex(&owner_username_key, &owner_id_key, 3600).await?;
        let _: () = conn.set_ex(&follower_id_key, &follower_value, 3600).await?;
        let _: () = conn
            .set_ex(&follower_username_key, &follower_id_key, 3600)
            .await?;

        Ok(result)
    }

    pub async fn reject_follow_request(
        state: &AppState,
        owner_id: i64,
        follower_id: i64,
    ) -> Result<(), ProfileServiceError> {
        let mut tx = state.db_pool.begin().await?;
        let result =
            user_repo::follow::reject_follow_repo(&mut tx, owner_id, follower_id).await?;

        if !result {
            return Err(ProfileServiceError::FollowRequestNotFound);
        }

        tx.commit().await?;
        Ok(())
    }
}
