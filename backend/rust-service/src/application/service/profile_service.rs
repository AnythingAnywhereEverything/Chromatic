use crate::{
    application::{
        repository::{
            media::{self as media_repo, row::MediaStatus}, user::{self as user_repo, find::URDQOpts, row::UserProfileRow},
        }, service::{
            errors::ProfileServiceError,
            media::{
                processor::types::{
                    CropStyle, ImageProcessorType, MediaProcessorFFlags, MediaProcessorOptions,
                },
                service::MediaService,
                service_type::{ContainerConfig, MediaServiceOptions},
                types::file::MultipartFile,
            },
        }, state::AppState,
    }, domain::user::types::{Bio, DisplayName, Quotes},
};
pub struct ProfileService;

impl ProfileService {
    pub async fn get_profile(
        state: &AppState,
        user_id: i64,
        requester: Option<i64>,
    ) -> Result<UserProfileRow, ProfileServiceError> {
        let mut tx = state.db_pool.begin().await?;

        let profile = user_repo::find::profile_full_by_id(&mut tx, user_id, requester).await?;

        tx.commit().await?;

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
        uploaded_avatar: Option<MultipartFile>,
        remove_avatar: Option<bool>,
        uploaded_banner: Option<MultipartFile>,
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

        let media_base_options = MediaServiceOptions {
            upload_route: String::new(),
            uploader_id: user_id,
            container: Some(ContainerConfig {
                use_hash_names: true,
                use_animated_image_indicator: true,
                ..Default::default()
            }),
            processor: Some(MediaProcessorOptions {
                fflags: Some(MediaProcessorFFlags {
                    image_thumbhash: true,
                    ..Default::default()
                }),
                image_processors: Some(vec![]), // no processing for now, as the position and scale will be handled on the client side
                video_processors: None,
                post_processors: None,
            }),
        };

        // combine avatar and banner media handling into one transaction to avoid multiple transactions
        let mut to_upload = vec![];

        if uploaded_avatar.is_some() {
            let mut option = media_base_options.clone();
            option.upload_route = format!("avatars/{}", user_id);
            option.processor = Some(MediaProcessorOptions {
                fflags: Some(MediaProcessorFFlags {
                    image_thumbhash: true,
                    ..Default::default()
                }),
                image_processors: Some(vec![ImageProcessorType::Crop {
                    style: CropStyle::Ratio {
                        width: 1,
                        height: 1,
                        scale: 1.0,
                    },
                    position: Some((0.5, 0.5)),
                }]),
                ..Default::default()
            });
            to_upload.push((uploaded_avatar, option));
        }

        if uploaded_banner.is_some() {
            let mut option = media_base_options.clone();
            option.upload_route = format!("banners/{}", user_id);
            option.processor = Some(MediaProcessorOptions {
                fflags: Some(MediaProcessorFFlags {
                    image_thumbhash: true,
                    ..Default::default()
                }),
                image_processors: Some(vec![ImageProcessorType::Crop {
                    style: CropStyle::Ratio {
                        width: 5,
                        height: 2,
                        scale: 1.0,
                    },
                    position: Some((0.5, 0.5)),
                }]),
                ..Default::default()
            });
            to_upload.push((uploaded_banner, option));
        }

        for (uploaded_media, options) in to_upload {
            if let Some(uploaded_media) = uploaded_media {
                let media_service = MediaService::new();
                let uploaded_medias = media_service
                    .save_media(&state, uploaded_media, options.clone())
                    .await?;

                let mut tx = state.db_pool.begin().await?;
                media_repo::update::media_status(
                    &mut tx,
                    &uploaded_medias.get_id(),
                    &MediaStatus::Completed,
                )
                .await?;

                let media_to_delete = if options.upload_route.contains("avatars") {
                    user_repo::update::avatar_media_id(
                        &mut tx,
                        user_id,
                        Some(uploaded_medias.get_id()),
                    )
                    .await?
                } else {
                    user_repo::update::banner_media_id(
                        &mut tx,
                        user_id,
                        Some(uploaded_medias.get_id()),
                    )
                    .await?
                };

                if let Some(media_id) = media_to_delete {
                    let path =
                        media_repo::delete::hard_delete_media_data(&mut tx, media_id).await?;
                    if path.contains("a_") {
                        let animated_path = path.replace(".png", ".webp");
                        state.storage.delete(&animated_path).await;
                    }
                    state.storage.delete(&path).await;
                }

                tx.commit().await?;
            }
        }

        let mut tx = state.db_pool.begin().await?;
        let profile = user_repo::find::profile_full_by_id(&mut tx, user_id, None).await?;
        tx.commit().await?;

        Ok(profile)
    }
}
