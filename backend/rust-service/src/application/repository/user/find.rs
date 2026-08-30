use sqlx::Transaction;

use crate::application::repository::{
    RepositoryResult,
    user::row::{UserProfileMinimalRow, UserProfileRow, UserRow},
};

/// User Repository Data Query Options
///
/// This struct is used to specify which fields to retrieve when querying for user data.
///
/// Helps to optimize the query and reduce the amount of data retrieved from the database.
pub struct URDQOpts {
    pub requester: Option<i64>,

    pub target_id: Option<i64>,
    pub target_username: Option<String>,

    // For privacy reason, we dont always give out user email.
    pub get_email: bool,

    // for JOIN reasons, we need to know if we want to get the avatar and banner or not.
    pub get_avatar: bool,
    pub get_banner: bool,

    pub get_followers_count: bool,
    pub get_following_count: bool,
    pub get_posts_count: bool,

    pub get_pinned_posts: bool,

    pub get_is_follower: bool,
    pub get_is_following: bool,

    pub get_bio: bool,
    pub get_quote: bool,

    // If true, the query will ignore the deleted_at field and return the user even if they are deleted.
    pub ignore_deleted: bool,
}

impl Default for URDQOpts {
    fn default() -> Self {
        Self {
            requester: None,
            target_id: None,
            target_username: None,
            get_avatar: false,
            get_email: false,
            get_banner: false,
            get_followers_count: false,
            get_following_count: false,
            get_posts_count: false,
            get_pinned_posts: false,
            get_is_follower: false,
            get_is_following: false,
            get_bio: false,
            get_quote: false,
            ignore_deleted: false,
        }
    }
}

pub async fn experimental_dynamic_user_query(
    tx: &mut Transaction<'_, sqlx::Postgres>,
    opts: URDQOpts,
) -> RepositoryResult<UserProfileRow> {
    let mut select = vec![
        "u.id::TEXT".to_string(),
        "u.username".to_string(),
    ];

    if opts.get_email {
        select.push("u.email".into());
    } else {
        select.push("NULL::TEXT AS email".into());
    }

    if opts.get_followers_count {
        select.push("up.followers_count".into());
    } else {
        select.push("NULL::BIGINT AS followers_count".into());
    }

    if opts.get_following_count {
        select.push("up.following_count".into());
    } else {
        select.push("NULL::BIGINT AS following_count".into());
    }

    if opts.get_posts_count {
        select.push("up.posts_count".into());
    } else {
        select.push("NULL::BIGINT AS posts_count".into());
    }

    if opts.get_pinned_posts {
        select.push(
            r#"
            (
                SELECT ARRAY_AGG(upp.post_id ORDER BY upp.post_id)
                FROM user_pinned_posts upp
                WHERE upp.user_id = u.id
            ) AS pinned_posts
            "#
            .into(),
        );
    } else {
        select.push("NULL::BIGINT[] AS pinned_posts".into());
    }

    // * Always select display_name, as it is a common field that is often needed for user profiles.
    select.push("up.display_name".into());

    if opts.get_bio {
        select.push("up.bio".into());
    } else {
        select.push("NULL::TEXT AS bio".into());
    }

    if opts.get_quote {
        select.push("us.message AS quote".into());
    } else {
        select.push("NULL::TEXT AS quote".into());
    }

    if opts.get_is_following {
        select.push("uf1.is_following".into());
    } else {
        select.push("NULL::BOOLEAN AS is_following".into());
    }

    if opts.get_is_follower {
        select.push("uf2.is_follower".into());
    } else {
        select.push("NULL::BOOLEAN AS is_follower".into());
    }

    if opts.get_avatar {
        select.push("m1.name AS avatar".into());
        select.push("m1.thumbhash AS avatar_thumbhash".into());
    } else {
        select.push("NULL::TEXT AS avatar".into());
        select.push("NULL::TEXT AS avatar_thumbhash".into());
    }

    if opts.get_banner {
        select.push("m2.name AS banner".into());
        select.push("m2.thumbhash AS banner_thumbhash".into());
    } else {
        select.push("NULL::TEXT AS banner".into());
        select.push("NULL::TEXT AS banner_thumbhash".into());
    }

    // * Always select created_at, as it is a common field that is often needed for user profiles.
    select.push("u.created_at".into());

    let needs_profile = opts.get_followers_count
        || opts.get_following_count
        || opts.get_posts_count
        || opts.get_avatar
        || opts.get_banner;

    let needs_requester = opts.get_is_following || opts.get_is_follower;

    let mut query = format!(
        r#"
        SELECT
            {}
        FROM users u
        "#,
        select.join(",\n            ")
    );

    if needs_profile {
        query.push_str(
            r#"
            LEFT JOIN user_profiles up
                ON u.id = up.user_id
            "#,
        );
    }

    if opts.get_quote {
        query.push_str(
            r#"
            LEFT JOIN user_statuses us
                ON u.id = us.user_id
            "#,
        );
    }

    if opts.get_avatar {
        query.push_str(
            r#"
            LEFT JOIN media_objects m1
                ON up.avatar_media_id = m1.media_id AND m1.kind = 'original'
            "#,
        );
    }

    if opts.get_banner {
        query.push_str(
            r#"
            LEFT JOIN media_objects m2
                ON up.banner_media_id = m2.media_id AND m2.kind = 'original'
            "#,
        );
    }

    if opts.get_is_following {
        query.push_str(
            r#"
            LEFT JOIN (
                SELECT
                    follower_id,
                    user_id,
                    true AS is_following
                FROM user_follow
                WHERE status = 'accepted'
            ) uf1
                ON u.id = uf1.user_id
                AND uf1.follower_id = $2
            "#,
        );
    }

    if opts.get_is_follower {
        query.push_str(
            r#"
            LEFT JOIN (
                SELECT
                    user_id,
                    follower_id,
                    true AS is_follower
                FROM user_follow
                WHERE status = 'accepted'
            ) uf2
                ON u.id = uf2.follower_id
                AND uf2.user_id = $2
            "#,
        );
    }

    query.push_str("WHERE ");

    if opts.target_id.is_some() {
        query.push_str("u.id = $1");
    } else if opts.target_username.is_some() {
        query.push_str("u.username = $1");
    } else {
        return Err(sqlx::Error::Protocol(
            "either target_id or target_username must be provided".into(),
        )
        .into());
    }

    if !opts.ignore_deleted {
        query.push_str(" AND u.deleted_at IS NULL");
    }

    let mut db_query = sqlx::query_as::<_, UserProfileRow>(&query);

    if let Some(target_id) = opts.target_id {
        db_query = db_query.bind(target_id);
    } else if let Some(target_username) = opts.target_username {
        db_query = db_query.bind(target_username);
    }

    if needs_requester {
        db_query = db_query.bind(opts.requester);
    }

    let row = db_query.fetch_one(tx.as_mut()).await?;

    Ok(row)
}

pub async fn profile_full_by_id(
    tx: &mut Transaction<'_, sqlx::Postgres>,
    user_id: i64,
    requester: Option<i64>,
) -> RepositoryResult<UserProfileRow> {
    let row = sqlx::query_as::<_, UserProfileRow>(
        r#"
        SELECT
            u.id::TEXT,
            u.username,
            up.followers_count,
            up.following_count,
            up.posts_count,
            (
                SELECT ARRAY_AGG(upp.post_id ORDER BY upp.post_id)
                FROM user_pinned_posts upp
                WHERE upp.user_id = u.id
            ) AS pinned_posts,
            up.display_name,
            up.bio,
            us.message AS quote,
            uf1.is_following,
            uf2.is_follower,
            m1.name AS avatar,
            m1.thumbhash AS avatar_thumbhash,
            m2.name AS banner,
            m2.thumbhash AS banner_thumbhash,
            u.created_at
        FROM users u
        LEFT JOIN user_statuses us
            ON u.id = us.user_id
        LEFT JOIN user_profiles up
            ON u.id = up.user_id
        LEFT JOIN media_objects m1
            ON up.avatar_media_id = m1.media_id AND m1.kind = 'original'
        LEFT JOIN media_objects m2
            ON up.banner_media_id = m2.media_id AND m2.kind = 'original'
        LEFT JOIN (
            SELECT follower_id, user_id, true AS is_following
            FROM user_follow
            WHERE status = 'accepted'
        ) uf1
            ON u.id = uf1.user_id
            AND uf1.follower_id = $2
        LEFT JOIN (
            SELECT user_id, follower_id, true AS is_follower
            FROM user_follow
            WHERE status = 'accepted'
        ) uf2
            ON u.id = uf2.follower_id
            AND uf2.user_id = $2
        WHERE u.id = $1
            AND u.deleted_at IS NULL;
        "#,
    )
    .bind(user_id)
    .bind(requester)
    .fetch_one(tx.as_mut())
    .await?;

    Ok(row)
}

pub async fn profile_full_by_username(
    tx: &mut Transaction<'_, sqlx::Postgres>,
    username: &str,
    requester: Option<i64>,
) -> RepositoryResult<UserProfileRow> {
    let row = sqlx::query_as::<_, UserProfileRow>(
        r#"
        SELECT
            u.id::TEXT,
            u.username,
            up.followers_count,
            up.following_count,
            up.posts_count,
            (
                SELECT ARRAY_AGG(upp.post_id ORDER BY upp.post_id)
                FROM user_pinned_posts upp
                WHERE upp.user_id = u.id
            ) AS pinned_posts,
            up.display_name,
            up.bio,
            us.message AS quote,
            uf1.is_following,
            uf2.is_follower,
            m1.name AS avatar,
            m1.thumbhash AS avatar_thumbhash,
            m2.name AS banner,
            m2.thumbhash AS banner_thumbhash,
            u.created_at
        FROM users u
        LEFT JOIN user_statuses us
            ON u.id = us.user_id
        LEFT JOIN user_profiles up
            ON u.id = up.user_id
        LEFT JOIN media_objects m1
            ON up.avatar_media_id = m1.media_id AND m1.kind = 'original'
        LEFT JOIN media_objects m2
            ON up.banner_media_id = m2.media_id AND m2.kind = 'original'
        LEFT JOIN (
            SELECT follower_id, user_id, true AS is_following
            FROM user_follow
            WHERE status = 'accepted'
        ) uf1
            ON u.id = uf1.user_id
            AND uf1.follower_id = $2
        LEFT JOIN (
            SELECT user_id, follower_id, true AS is_follower
            FROM user_follow
            WHERE status = 'accepted'
        ) uf2
            ON u.id = uf2.follower_id
            AND uf2.user_id = $2
        WHERE u.username = $1 AND u.deleted_at IS NULL;
        "#,
    )
    .bind(username)
    .bind(requester)
    .fetch_one(tx.as_mut())
    .await?;

    Ok(row)
}

pub async fn profile_with_minimal_by_id(
    tx: &mut Transaction<'_, sqlx::Postgres>,
    user_id: i64,
) -> RepositoryResult<UserProfileMinimalRow> {
    let row = sqlx::query_as::<_, UserProfileMinimalRow>(
        r#"
        SELECT 
            u.id, 
            u.email, 
            u.username, 
            up.display_name, 
            up.bio, 
            m1.name AS avatar,
            m1.thumbhash AS avatar_thumbhash,
            m2.name AS banner,
            m2.thumbhash AS banner_thumbhash,
            u.created_at
        FROM users u
        LEFT JOIN user_profiles up ON u.id = up.user_id
        LEFT JOIN media_objects m1 ON up.avatar_media_id = m1.media_id AND m1.kind = 'original'
        LEFT JOIN media_objects m2 ON up.banner_media_id = m2.media_id AND m2.kind = 'original'
        WHERE u.id = $1
        "#,
    )
    .bind(user_id)
    .fetch_one(tx.as_mut())
    .await?;

    Ok(row)
}

pub async fn by_id(
    tx: &mut Transaction<'_, sqlx::Postgres>,
    user_id: i64,
) -> RepositoryResult<UserRow> {
    let row = sqlx::query_as::<_, UserRow>(
        r#"
        SELECT id, email, username FROM users WHERE id = $1 AND deleted_at IS NULL
        "#,
    )
    .bind(user_id)
    .fetch_one(tx.as_mut())
    .await?;

    Ok(row)
}

pub async fn by_email(
    tx: &mut Transaction<'_, sqlx::Postgres>,
    email: &str,
) -> RepositoryResult<UserRow> {
    let row = sqlx::query_as::<_, UserRow>(
        r#"
        SELECT id, email, username FROM users WHERE email = $1 AND deleted_at IS NULL
        "#,
    )
    .bind(email)
    .fetch_one(tx.as_mut())
    .await?;

    Ok(row)
}

pub async fn by_username(
    tx: &mut Transaction<'_, sqlx::Postgres>,
    username: &str,
) -> RepositoryResult<UserRow> {
    let row = sqlx::query_as::<_, UserRow>(
        r#"
        SELECT id, email, username FROM users WHERE username = $1 AND deleted_at IS NULL
        "#,
    )
    .bind(username)
    .fetch_one(tx.as_mut())
    .await?;

    Ok(row)
}

/// ! lower performance than by_email or by_username, use it only when you want to support login with both email and username
pub async fn by_username_or_email(
    tx: &mut Transaction<'_, sqlx::Postgres>,
    identifier: &str,
) -> RepositoryResult<UserRow> {
    let row = sqlx::query_as::<_, UserRow>(
        r#"
        SELECT id, email, username FROM users WHERE (username = $1 OR email = $1) AND deleted_at IS NULL
        "#,
    )
    .bind(identifier)
    .fetch_one(tx.as_mut())
    .await?;

    Ok(row)
}
