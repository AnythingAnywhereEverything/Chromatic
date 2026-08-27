use sqlx::Transaction;

use crate::application::repository::{RepositoryResult, post::row::PostRow};

pub enum FetchMode {
    One,
    All,
}

pub enum PostQueryResult {
    One(PostRow),
    Many(Vec<PostRow>),
}

pub struct PostQOpts {
    pub requester: Option<i64>,

    // * Use target_id only for fetching one specific post.
    pub target_id: Option<i64>,

    // * None means "start from the newest post".
    pub cursor_id: Option<i64>,

    pub get_avatar: bool,
    pub get_media: bool,
    pub get_tags: bool,
    pub get_is_liked: bool,
    pub get_followers_count: bool,
    pub get_following_count: bool,
    pub get_reposted_from: bool,

    pub ignore_deleted: bool,
    pub limit: Option<i32>,
    pub mode: FetchMode,
}

impl Default for PostQOpts {
    fn default() -> Self {
        Self {
            requester: None,
            target_id: None,
            cursor_id: None,

            get_avatar: false,
            get_media: false,
            get_tags: false,
            get_is_liked: false,
            get_followers_count: false,
            get_following_count: false,
            get_reposted_from: false,

            ignore_deleted: false,
            limit: Some(20),
            mode: FetchMode::One,
        }
    }
}

impl PostQOpts {
    pub fn full() -> Self {
        Self {
            requester: None,
            target_id: None,
            cursor_id: None,
            
            get_avatar: true,
            get_media: true,
            get_tags: true,
            get_is_liked: true,
            get_followers_count: true,
            get_following_count: true,
            get_reposted_from: true,

            ignore_deleted: false,
            limit: None,
            mode: FetchMode::All
        }
    }
}

pub async fn get_post_by_id_experiment(
    tx: &mut Transaction<'_, sqlx::Postgres>,
    opts: PostQOpts,
) -> RepositoryResult<PostQueryResult> {
    let mut select = vec![
        "m.id".to_string(),
        "m.user_id".to_string(),
        "u.username".to_string(),
        "up.display_name".to_string(),
        "m.content".to_string(),
        "m.total_likes".to_string(),
        "m.total_comments".to_string(),
        "m.is_repost".to_string(),
        "m.has_attachment".to_string(),
        "m.created_at".to_string(),
        "m.updated_at".to_string(),
        "m.visibility".to_string(),
    ];

    if opts.get_reposted_from {
        select.push("m.reposted_from".into());
    } else {
        select.push("NULL::BIGINT AS reposted_from".into());
    }

    if opts.get_is_liked {
        select.push(
            r#"
                EXISTS (
                    SELECT 1
                    FROM media_likes ml
                    WHERE ml.target_id = m.id
                    AND ml.target_type = 'post'
                    AND ml.user_id = $2
                    AND ml.is_like = TRUE
                ) AS is_liked
            "#
            .into(),
        );
    } else {
        select.push("FALSE AS is_liked".into());
    }

    if opts.get_media {
        select.push(
            r#"
            COALESCE(att.attachments, '[]'::json) AS media_attachment
            "#
            .into(),
        );
    } else {
        select.push("'[]'::json AS media_attachment".into());
    }

    if opts.get_tags {
        select.push(
            r#"
            COALESCE(tag.tags, '[]'::json) AS tags
            "#
            .into(),
        );
    } else {
        select.push("'[]'::json AS tags".into());
    }

    if opts.get_avatar {
        select.push("avatar_md.path AS avatar_path".into());
        select.push("avatar_mdt.mime_type AS avatar_mime".into());
        select.push("avatar_md.thumbhash AS avatar_thumbhash".into());
    } else {
        select.push("NULL::TEXT AS avatar_path".into());
        select.push("NULL::TEXT AS avatar_mime".into());
        select.push("NULL::TEXT AS avatar_thumbhash".into());
    }

    if opts.get_followers_count {
        select.push("up.followers_count".into());
    } else {
        select.push("0::INT AS followers_count".into());
    }

    if opts.get_following_count {
        select.push("up.following_count".into());
    } else {
        select.push("0::INT AS following_count".into());
    }

    let mut query = format!(
        r#"
        SELECT
            {}
        FROM media_posts m

        LEFT JOIN users u
            ON m.user_id = u.id

        LEFT JOIN user_profiles up
            ON m.user_id = up.user_id
        "#,
        select.join(",\n        ")
    );

    if opts.get_avatar {
        query.push_str(
            r#"
            LEFT JOIN media_data avatar_md
                ON avatar_md.id = up.avatar_media_id

            LEFT JOIN media_metadata avatar_mdt
                ON avatar_mdt.media_id = avatar_md.id
            "#,
        );
    }

    if opts.get_media {
        query.push_str(
            r#"
            LEFT JOIN LATERAL (
                SELECT json_agg(
                    json_build_object(
                        'id', md.id::text,
                        'user_id', md.uploader_id,
                        'path', md.path,
                        'created_at', md.created_at,
                        'thumbhash', md.thumbhash,
                        'name', md.name,
                        'updated_at', md.updated_at,
                        'flags', md.flags,
                        'status', md.status,
                        'file_size', mdt.file_size,
                        'mime_type', mdt.mime_type,
                        'width', mdt.width,
                        'height', mdt.height,
                        'duration', mdt.duration
                    )
                    ORDER BY md.id
                ) AS attachments
                FROM media_attachments a
                JOIN media_data md
                    ON md.id = a.media_id
                JOIN media_metadata mdt
                    ON mdt.media_id = md.id
                WHERE a.target_id = m.id
                  AND md.status = 'completed'
            ) att ON TRUE
            "#,
        );
    }

    if opts.get_tags {
        query.push_str(
            r#"
            LEFT JOIN LATERAL (
                SELECT json_agg(
                    json_build_object(
                        'target_id', ta.target_id,
                        'target_type', ta.target_type,
                        'tag_id', it.id,
                        'tag_name', it.tag_name,
                        'tag_color', it.tag_color
                    )
                    ORDER BY it.tag_name
                ) AS tags
                FROM tag_attachments ta
                JOIN interest_tags it
                    ON it.id = ta.tag_id
                WHERE ta.target_id = m.id
            ) tag ON TRUE
            "#,
        );
    }

    match opts.mode {
        FetchMode::One => {
            let target_id = opts.target_id.ok_or_else(|| {
                sqlx::Error::Protocol("target_id must be provided".into())
            })?;

            query.push_str(" WHERE m.id = $1");

            if !opts.ignore_deleted {
                query.push_str(" AND m.deleted_at IS NULL");
            }

            query.push_str(" AND m.status != 'inactive'");

            let limit_idx = if opts.get_is_liked { 3 } else { 2 };

            if opts.limit.is_some() {
                query.push_str(&format!(" LIMIT ${}::int4", limit_idx));
            }

            let mut db_query = sqlx::query_as::<_, PostRow>(&query);

            db_query = db_query.bind(target_id);

            if opts.get_is_liked {
                db_query = db_query.bind(opts.requester);
            }

            if let Some(limit) = opts.limit {
                db_query = db_query.bind(limit);
            }

            let row = db_query.fetch_one(tx.as_mut()).await?;

            Ok(PostQueryResult::One(row))
        }

        FetchMode::All => {
            query.push_str(" WHERE ($1::BIGINT IS NULL OR m.id < $1)");

            if !opts.ignore_deleted {
                query.push_str(" AND m.deleted_at IS NULL");
            }

            query.push_str(" AND m.status != 'inactive'");

            query.push_str(" ORDER BY m.id DESC");

            // * $2 because $1 is cursor_id.
            query.push_str(" LIMIT $2::int4");

            let mut db_query = sqlx::query_as::<_, PostRow>(&query);

            db_query = db_query.bind(opts.cursor_id);
            db_query = db_query.bind(opts.limit.unwrap_or(20));

            let rows = db_query.fetch_all(tx.as_mut()).await?;

            Ok(PostQueryResult::Many(rows))
    }
    }
}