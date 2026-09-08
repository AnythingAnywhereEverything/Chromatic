use redis::AsyncTypedCommands;

use crate::{
    application::{
        repository::{
            media::{self as media_repo, row::ProcessingState},
            user::{self as user_repo, find::URDQOpts, row::UserProfileRow},
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
        let key = format!("profile:{}", username);

        if let Some(value) = conn.get(&key).await? {
            // update the expiration time for the cached profile
            conn.expire(&key, 3600).await?;
            let profile: UserProfileRow = serde_json::from_str(&value)?;
            return Ok(profile);
        }

        let mut tx = state.db_pool.begin().await?;
        let profile =
            user_repo::find::profile_full_by_username(&mut tx, &username, requester).await?;
        tx.commit().await?;
        // cache the profile in Redis
        let value = serde_json::to_string(&profile)?;
        let _: () = conn.set_ex(key, value, 3600).await?;

        Ok(profile)
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
                    let path =
                        media_repo::delete::hard_delete_media_data(&mut tx, media_id).await?;
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
        let cache_key = format!(
            "profile:{}",
            &profile.username.clone().unwrap_or_else(|| "".to_string())
        );
        let cache_value = serde_json::to_string(&profile)?;
        conn.set_ex(&cache_key, &cache_value, 3600).await?;

        tx.commit().await?;

        Ok(profile)
    }
}
