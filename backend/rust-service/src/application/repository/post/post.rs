use sqlx::{Postgres, Transaction};

use crate::{
    api::handlers::post_handler::{PostVisibility, TagTarget}, application::{
        repository::{
            media::row::MediaDataWithMetadataRow,
            post::row::{
                CreatePostRow, HasAttachmentRow, PostLikesRow, PostRow, TagAttachmentFull,
                TagAttachmentRow, TotalLikesRow,
            },
        },
        service::errors::PostServiceError,
    },
};

// todo: func get YOUR FRIEND post
// todo: func get feed comment :d

// ? How am I gonna balanced the feed between friends and normal since there's no ML for the feed
// * Schuding them for show some of there friends post
// ? Do feed setting to let user edit the feed to show friend first, no friend, normal

// todo: get recent post on pfp.. fetch on profile page???
pub async fn get_feed_public(
    tx: &mut Transaction<'_, sqlx::Postgres>,
    cursor_id: Option<i64>,
    user_id: Option<i64>,
    limit: i32,
) -> Result<Vec<PostRow>, sqlx::Error> {
    sqlx::query_as::<_, PostRow>(
        r#"
             SELECT
                m.id,
                m.user_id,
                u.username,
                up.display_name,

                avatar_md.path AS avatar_path,
                avatar_mdt.mime_type AS avatar_mime,
                avatar_md.thumbhash AS avatar_thumbhash,

                up.followers_count,
                up.following_count,
                m.content,
                m.total_comments,
                m.total_likes,
                m.reposted_from,
                m.is_repost,
                m.has_attachment,
                m.created_at,
                m.updated_at,
                m.visibility,

                EXISTS (
                    SELECT 1
                    FROM media_likes ml
                    WHERE ml.media_post_id = m.id
                      AND ml.user_id = $2
                ) AS is_liked,
                COALESCE(att.attachments, '[]'::json) AS media_attachment,
                COALESCE(tag.tags, '[]'::json) AS tags
            FROM media_posts m

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

                JOIN media_data md ON md.id = a.media_id
                JOIN media_metadata mdt ON mdt.media_id = md.id

                WHERE a.target_id = m.id
                  AND md.status = 'completed'
            ) att ON TRUE

            LEFT JOIN LATERAL (
                SELECT json_agg(
                    json_build_object(
                        'target_id', ta.target_id,
                        'target_type', ta.target_type,
                        'tag_id', it.id,
                        'tag_name', it.tag_name
                    )
                    ORDER BY it.tag_name
                ) AS tags
                FROM tag_attachments ta
                JOIN interest_tags it ON it.id = ta.tag_id
                WHERE ta.target_id = m.id
            ) tag ON TRUE

        LEFT JOIN users u
            ON m.user_id = u.id
        LEFT JOIN user_profiles up
            ON m.user_id = up.user_id

        LEFT JOIN media_data avatar_md
            ON avatar_md.id = up.avatar_media_id
        LEFT JOIN media_metadata avatar_mdt
            ON avatar_mdt.media_id = avatar_md.id

        WHERE
            m.visibility = 'everyone'
            AND m.status != 'inactive'
            AND ($1 IS NULL OR m.id < $1)
            
        ORDER BY m.id DESC
        LIMIT $3;
        "#,
    )
    .bind(cursor_id)
    .bind(user_id)
    .bind(limit)
    .fetch_all(tx.as_mut())
    .await
}

// pub async fn get_friend_post(
//     tx: &mut Transaction<'_,sqlx::Postgres>,
//     user_id: i64,

// ) {

// }

// * case the friends is impl completed
pub async fn get_post_by_id(
    tx: &mut Transaction<'_, sqlx::Postgres>,
    post_id: i64,
    user_id: i64,
) -> Result<PostRow, sqlx::Error> {
    sqlx::query_as::<_, PostRow>(
        r#"
             SELECT
                m.id,
                m.user_id,
                u.username,
                up.display_name,

                avatar_md.path AS avatar_path,
                avatar_mdt.mime_type AS avatar_mime,
                avatar_md.thumbhash AS avatar_thumbhash,

                banner_md.path AS banner_path,
                banner_mdt.mime_type AS banner_mime,
                banner_md.thumbhash AS banner_thumbhash,

                up.followers_count,
                up.following_count,
                m.content,
                m.total_comments,
                m.total_likes,
                m.reposted_from,
                m.is_repost,
                m.has_attachment,
                m.created_at,
                m.updated_at,
                m.visibility,

                EXISTS (
                    SELECT 1
                    FROM media_likes ml
                    WHERE ml.media_post_id = m.id
                      AND ml.user_id = $2
                ) AS is_liked,
                COALESCE(att.attachments, '[]'::json) AS media_attachment,
                COALESCE(tag.tags, '[]'::json) AS tags
            FROM media_posts m

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

                JOIN media_data md ON md.id = a.media_id
                JOIN media_metadata mdt ON mdt.media_id = md.id

                WHERE a.target_id = m.id
                  AND md.status = 'completed'
            ) att ON TRUE

            LEFT JOIN LATERAL (
                SELECT json_agg(
                    json_build_object(
                        'target_id', ta.target_id,
                        'target_type', ta.target_type,
                        'tag_id', it.id,
                        'tag_name', it.tag_name
                    )
                    ORDER BY it.tag_name
                ) AS tags
                FROM tag_attachments ta
                JOIN interest_tags it ON it.id = ta.tag_id
                WHERE ta.target_id = m.id
            ) tag ON TRUE

        LEFT JOIN users u
            ON m.user_id = u.id
        LEFT JOIN user_profiles up
            ON m.user_id = up.user_id

        LEFT JOIN media_data avatar_md
            ON avatar_md.id = up.avatar_media_id
        LEFT JOIN media_metadata avatar_mdt
            ON avatar_mdt.media_id = avatar_md.id

        LEFT JOIN media_data banner_md
            ON banner_md.id = up.banner_media_id
        LEFT JOIN media_metadata banner_mdt
            ON banner_mdt.media_id = banner_md.id
            
        WHERE
            m.id = $1
            AND m.status != 'inactive'
            AND (
                m.user_id = $2
                OR m.visibility = 'everyone'
                OR (
                    m.visibility = 'friends'
                    AND EXISTS (
                        SELECT 1
                        FROM user_friends uf
                        WHERE uf.user_id = $2
                          AND uf.friend_id = m.user_id
        "#,
    )
    .bind(post_id)
    .bind(user_id)
    .fetch_one(tx.as_mut())
    .await
}

pub async fn create_post(
    tx: &mut Transaction<'_, sqlx::Postgres>,
    id: &i64,
    user_id: i64,
    content: &str,
    repost_from: Option<i64>,
    has_attachment: bool,
    is_repost: bool,
    visibility: PostVisibility,
) -> Result<CreatePostRow, sqlx::Error> {
    sqlx::query_as::<_, CreatePostRow>(
        r#"
        INSERT INTO media_posts (
            id,
            user_id,
            content,
            reposted_from,
            is_repost,
            has_attachment,
            created_at,
            updated_at,
            visibility
        )
        VALUES ($1, $2, $3, $4, $5, $6, NOW(), NOW(), $7)
        RETURNING
            id,
            user_id,
            content,
            total_likes,
            total_comments,
            reposted_from,
            is_repost,
            has_attachment,
            created_at,
            updated_at,
            visibility,
            '[]'::json AS media_attachment,
            '[]'::json AS tags
        "#,
    )
    .bind(id)
    .bind(user_id)
    .bind(content)
    .bind(repost_from)
    .bind(is_repost)
    .bind(has_attachment)
    .bind(visibility)
    .fetch_one(&mut **tx)
    .await
}

pub async fn update_post(
    tx: &mut Transaction<'_, sqlx::Postgres>,
    id: i64,
    user_id: i64,
    content: String,
    visibility: PostVisibility,
) -> Result<CreatePostRow, sqlx::Error> {
    sqlx::query_as::<_, CreatePostRow>(
        r#"
            UPDATE media_posts
            SET 
                content = $1, 
                visibility = $2,
                updated_at = NOW()
            WHERE id = $3 AND user_id = $4
            RETURNING *
        "#,
    )
    .bind(content)
    .bind(visibility)
    .bind(id)
    .bind(user_id)
    .fetch_one(&mut **tx)
    .await
}

pub async fn delete_post(
    tx: &mut Transaction<'_, sqlx::Postgres>,
    id: i64,
    user_id: i64,
) -> Result<u64, sqlx::Error> {
    let delete = sqlx::query(
        r#"
            DELETE FROM media_posts
            WHERE id = $1 AND user_id = $2
        "#,
    )
    .bind(id)
    .bind(user_id)
    .execute(&mut **tx)
    .await?;

    Ok(delete.rows_affected())
}
// * ----------------------------------------------
// * Attachment
// * ----------------------------------------------
pub async fn add_has_attachment(
    tx: &mut Transaction<'_, sqlx::Postgres>,
    target_id: i64,
    media_id: i64,
    target_type: String,
) -> Result<Vec<HasAttachmentRow>, sqlx::Error> {
    sqlx::query_as::<_, HasAttachmentRow>(
        r#"
            INSERT INTO media_attachments ( 
                target_id,
                media_id,
                target_type
            )
            VALUES ($1, $2, $3)
            RETURNING *
        "#,
    )
    .bind(target_id)
    .bind(media_id)
    .bind(target_type)
    .fetch_all(&mut **tx)
    .await
}

pub async fn get_post_attachment(
    tx: &mut Transaction<'_, sqlx::Postgres>,
    post_id: i64,
) -> Result<Vec<MediaDataWithMetadataRow>, sqlx::Error> {
    sqlx::query_as::<_, MediaDataWithMetadataRow>(
        r#"
            SELECT 
                md.id,
                md.uploader_id,
                md.path,
                md.thumbhash,
                md.name,
                md.status,
                md.created_at,
                md.updated_at,
                mdt.file_size,
                mdt.mime_type,
                mdt.width,
                mdt.height,
                mdt.duration
                FROM media_attachments ma 
            JOIN media_data md ON ma.media_id = md.id
            JOIN media_metadata mdt ON mdt.media_id = md.id
            WHERE ma.target_id = $1
            ORDER BY md.id
        "#,
    )
    .bind(post_id)
    .fetch_all(&mut **tx) // This re-borrowing is correct!
    .await
}

pub async fn delete_target_attachments(
    tx: &mut Transaction<'_, sqlx::Postgres>,
    target_id: i64,
) -> Result<(), sqlx::Error> {
    sqlx::query(
        r#"
            DELETE FROM media_attachments
            WHERE target_id = $1
        "#,
    )
    .bind(target_id)
    .execute(&mut **tx)
    .await?;

    Ok(())
}

// * ----------------------------------------------
// * Interest tags
// * ----------------------------------------------

pub async fn add_tags_target(
    tx: &mut Transaction<'_, sqlx::Postgres>,
    target_id: i64,
    target_type: TagTarget,
    tag_id: i64,
) -> Result<TagAttachmentRow, sqlx::Error> {
    sqlx::query_as::<_, TagAttachmentRow>(
        r#"
            INSERT INTO tag_attachments (
                target_id,
                target_type,
                tag_id
            )
            VALUES($1, $2, $3)
            RETURNING *
        "#,
    )
    .bind(target_id)
    .bind(target_type)
    .bind(tag_id)
    .fetch_one(tx.as_mut())
    .await
}

pub async fn get_tag_attachments(
    tx: &mut Transaction<'_, Postgres>,
    target_id: i64,
) -> Result<Vec<TagAttachmentFull>, sqlx::Error> {
    sqlx::query_as::<_, TagAttachmentFull>(
        r#"
            SELECT
                ta.target_id,
                ta.target_type,
                ta.tag_id,
                it.tag_name
            FROM tag_attachments ta
            JOIN interest_tags it
                ON it.id = ta.tag_id
            WHERE ta.target_id = $1
        "#,
    )
    .bind(target_id)
    .fetch_all(tx.as_mut())
    .await
}

pub async fn delete_tag_attachment(
    tx: &mut Transaction<'_, Postgres>,
    target_id: i64,
    tag_id: i64,
) -> Result<u64, sqlx::Error> {
    let row = sqlx::query(
        r#"
            DELETE FROM tag_attachments
            WHERE target_id = $1
              AND tag_id = $2
        "#,
    )
    .bind(target_id)
    .bind(tag_id)
    .execute(tx.as_mut())
    .await?;

    Ok(row.rows_affected())
}
// -------------------------------------
// * Small like patch
// -------------------------------------
// ? Does this working fine????

// Pagnigation cursor
pub async fn get_info_like_person(
    tx: &mut Transaction<'_, sqlx::Postgres>,
    post_id: i64,
    cursor_ts: Option<i64>,
) -> Result<Vec<PostLikesRow>, sqlx::Error> {
    sqlx::query_as(
        r#"
            SELECT *
            FROM media_likes
            WHERE media_id = $1
              AND ($2 IS NULL OR created_at < to_timestamp($2))
            ORDER BY created_at DESC
            LIMIT 20
        "#,
    )
    .bind(post_id)
    .bind(cursor_ts)
    .fetch_all(tx.as_mut())
    .await
}

pub async fn like_post_repo(
    tx: &mut Transaction<'_, sqlx::Postgres>,
    user_id: i64,
    target_id: i64,
    is_like: bool,
) -> Result<TotalLikesRow, sqlx::Error> {
    sqlx::query_as::<_, TotalLikesRow>(
        r#"
            WITH previous AS (
                SELECT is_liked
                FROM media_likes
                WHERE user_id = $1
                  AND media_post_id = $2
            ),
            upsert AS (
                INSERT INTO media_likes (
                    user_id,
                    media_post_id,
                    created_at,
                    updated_at,
                    is_liked
                )
                VALUES (
                    $1,
                    $2,
                    NOW(),
                    NOW(),
                    $3
                )
                ON CONFLICT (user_id, media_post_id)
                DO UPDATE SET
                    is_liked = EXCLUDED.is_liked,
                    updated_at = NOW()
                RETURNING is_liked
            )
            UPDATE media_posts
            SET total_likes = total_likes +
                CASE
                    WHEN NOT EXISTS (SELECT 1 FROM previous)
                         AND $3 = TRUE
                        THEN 1

                    WHEN (SELECT is_liked FROM previous) = FALSE
                         AND $3 = TRUE
                        THEN 1

                    WHEN (SELECT is_liked FROM previous) = TRUE
                         AND $3 = FALSE
                        THEN -1

                    ELSE 0
                END
            WHERE id = $2
            RETURNING id, total_likes
        "#,
    )
    .bind(user_id)
    .bind(target_id)
    .bind(is_like)
    .fetch_one(tx.as_mut())
    .await
}

// * -----------------------------------------------------
// * Bookmark
// * -----------------------------------------------------

pub async fn bookmark_post(
    tx: &mut Transaction<'_, sqlx::Postgres>,
    media_id: i64,
    user_id: i64,
) -> Result<(), PostServiceError> {
    let _ = sqlx::query(
        r#"
            INSERT INTO media_bookmarks (post_id, user_id, created_at)
            VALUES ($1, $2 , NOW())
        "#,
    )
    .bind(media_id)
    .bind(user_id)
    .execute(tx.as_mut())
    .await;

    Ok(())
}

pub async fn remove_bookmark_post(
    tx: &mut Transaction<'_, sqlx::Postgres>,
    media_id: i64,
    user_id: i64,
) -> Result<(), PostServiceError> {
    let _ = sqlx::query(
        r#"
            DELETE FROM media_bookmarks
            WHERE post_id = $1 AND user_id = $2
        "#,
    )
    .bind(media_id)
    .bind(user_id)
    .execute(tx.as_mut())
    .await;
    Ok(())
}

// if true insert
pub async fn toggle_bookmark(
    tx: &mut Transaction<'_, sqlx::Postgres>,
    media_id: i64,
    user_id: i64,
    is_bookmarking: bool,
) -> Result<(), PostServiceError> {
    if is_bookmarking {
        sqlx::query(
            r#"
                INSERT INTO media_bookmarks (post_id, user_id, created_at)
                VALUES ($1, $2, NOW())
                ON CONFLICT DO NOTHING
            "#,
        )
        .bind(media_id)
        .bind(user_id)
        .execute(tx.as_mut())
        .await?; // * propagate error instead of ignoring it
    } else {
        sqlx::query(
            r#"
                DELETE FROM media_bookmarks
                WHERE post_id = $1 AND user_id = $2
            "#,
        )
        .bind(media_id)
        .bind(user_id)
        .execute(tx.as_mut())
        .await?; // ! ensure the record exists or handle 0 rows affected if needed
    }

    Ok(())
}
