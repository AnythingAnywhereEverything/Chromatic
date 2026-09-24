use sqlx::{Postgres, Transaction};

use crate::application::repository::{
    RepositoryResult,
    post::row::{CommentBaseRow, CommentIdsRow, FeedRow, PostBaseRow, UserPostsRow},
};

pub async fn base_post(
    tx: &mut Transaction<'_, Postgres>,
    post_id: i64,
    user_id: Option<i64>,
) -> RepositoryResult<Option<PostBaseRow>> {
    let post = sqlx::query_as::<_, PostBaseRow>(
        r#"
        SELECT
            p.user_id AS author_id,
            p.id::text AS post_id,
            p.content,

            p.total_likes,
            p.total_comments,

            p.is_repost as is_reposted,
            (
                SELECT jsonb_build_object(
                    'id', rp.id::text,
                    'content', rp.content,
                    'author', (
                        SELECT jsonb_build_object(
                            'id', u.id::text,
                            'username', u.username,
                            'display_name', up.display_name,
                            'avatar', mo.name,
                            'avatar_placeholder', mo.thumbhash,
                            'created_at', u.created_at
                        )
                        FROM users u
                        JOIN user_profiles up ON up.user_id = u.id
                        LEFT JOIN media_objects mo ON mo.media_id = up.avatar_media_id
                        WHERE u.id = rp.user_id
                    ),
                    'visibility', rp.visibility,
                    'has_attachment', rp.has_attachment,
                    'attachments', COALESCE(
                        (
                            SELECT jsonb_agg(
                                jsonb_build_object(
                                    'id', gma.id::text,
                                    'file_type', gma.file_type,
                                    'processing_state', gma.processing_state,
                                    'post_processing_state', gma.post_processing_state,
                                    'flags', gma.flags::text,
                                    'media_objects', gma.media_objects,
                                    'media_object_metadata', gma.media_object_metadata,
                                    'media_hls', gma.media_hls,
                                    'media_hls_playlists', gma.media_hls_playlists,
                                    'created_at', gma.created_at,
                                    'updated_at', gma.updated_at
                                )
                                ORDER BY gma.id
                            )
                            FROM media_attachments ma
                            JOIN get_media_by_id_without_playlists(ma.media_id) gma
                                ON gma.id = ma.media_id
                            WHERE ma.target_id = rp.id
                            AND gma.deleted_at IS NULL
                        ),
                        '[]'::jsonb
                    ),
                    'created_at', rp.created_at,
                    'updated_at', rp.updated_at
                )
                FROM media_posts rp
                WHERE rp.id = p.reposted_from
                AND rp.deleted_at IS NULL
            ) AS reposted_post,
            p.visibility,
            COALESCE((
                SELECT jsonb_agg(
                    jsonb_build_object(
                        'target_id', ta.target_id::text,
                        'target_type', ta.target_type,
                        'tag_id', ta.tag_id::text,
                        'tag_name', it.tag_name
                    )
                )
                FROM tag_attachments ta
                JOIN interest_tags it
                    ON it.id = ta.tag_id
                WHERE ta.target_id = p.id
            ), '[]'::jsonb) AS tags,
            (
                EXISTS (
                    SELECT 1
                    FROM media_likes ml
                    WHERE ml.target_id = p.id
                    AND ml.target_type = 'post'
                    AND ml.user_id = $2
                    AND ml.is_like = TRUE
                )
            ) AS is_liked,
            p.has_attachment,
            COALESCE(
                (
                    SELECT jsonb_agg(
                        jsonb_build_object(
                            'id', gma.id::text,
                            'file_type', gma.file_type,
                            'processing_state', gma.processing_state,
                            'post_processing_state', gma.post_processing_state,
                            'flags', gma.flags::text,

                            'media_objects', gma.media_objects,
                            'media_object_metadata', gma.media_object_metadata,

                            'media_hls', gma.media_hls,
                            'media_hls_playlists', gma.media_hls_playlists,

                            'created_at', gma.created_at,
                            'updated_at', gma.updated_at
                        )
                        ORDER BY gma.id
                    )
                    FROM media_attachments ma
                    JOIN get_media_by_id_without_playlists(ma.media_id) gma
                        ON gma.id = ma.media_id
                    WHERE ma.target_id = p.id
                    AND gma.deleted_at IS NULL
                ),
                '[]'::jsonb
            ) AS attachments,
            p.created_at,
            p.updated_at
        FROM media_posts p
        WHERE p.id = $1
        AND p.deleted_at IS NULL
        AND (
                (
                    $2 IS NULL
                    AND p.visibility = 'everyone'::post_visibility
                )

                OR 

                (
                    -- The post belongs to the current user
                    $2 IS NOT NULL
                    AND p.user_id = $2
                )

                OR

                (
                    $2 IS NOT NULL
                    AND (
                        -- Everyone can see it
                        p.visibility = 'everyone'::post_visibility

                        OR

                        -- Both users follow each other
                        (
                            p.visibility = 'friend'::post_visibility
                            AND EXISTS (
                                SELECT 1
                                FROM user_follow uf1
                                JOIN user_follow uf2
                                    ON uf1.follower_id = uf2.user_id
                                AND uf2.follower_id = uf1.user_id
                                WHERE uf1.user_id = p.user_id
                                AND uf2.user_id = $2
                            )
                        )
                    )
                )
            );
        "#,
    )
    .bind(post_id)
    .bind(user_id)
    .fetch_optional(tx.as_mut())
    .await?;

    Ok(post)
}

pub async fn feed(
    tx: &mut Transaction<'_, Postgres>,
    user_id: i64,
    before: Option<chrono::DateTime<chrono::Utc>>,
    before_id: Option<i64>,
    limit: i32,
) -> RepositoryResult<Vec<i64>> {
    let posts = sqlx::query_as::<_, FeedRow>(
        r#"
        SELECT p.id AS post_id
        FROM media_posts p
        WHERE p.deleted_at IS NULL
        AND (
                p.visibility = 'everyone'::post_visibility
                OR (
                    p.visibility = 'friend'::post_visibility
                    AND EXISTS (
                        SELECT 1
                        FROM user_follow uf1
                        JOIN user_follow uf2
                            ON uf1.follower_id = uf2.user_id
                        AND uf2.follower_id = uf1.user_id
                        WHERE uf1.user_id = p.user_id
                        AND uf2.user_id = $1
                    )
                )
                OR p.user_id = $1
            )
        AND (
                $2::timestamptz IS NULL
                OR p.created_at < $2
                OR (p.created_at = $2 AND p.id < $3)
            )
        ORDER BY p.created_at DESC, p.id DESC
        LIMIT $4
        "#,
    )
    .bind(user_id)
    .bind(before)
    .bind(before_id)
    .bind(limit)
    .fetch_all(tx.as_mut())
    .await?;

    Ok(posts.into_iter().map(|feed_row| feed_row.post_id).collect())
}

pub async fn user_posts(
    tx: &mut Transaction<'_, Postgres>,
    target_id: i64,
    requester_id: Option<i64>,
    before: Option<chrono::DateTime<chrono::Utc>>,
    before_id: Option<i64>,
    limit: i32,
) -> RepositoryResult<Vec<i64>> {
    let posts = sqlx::query_as::<_, UserPostsRow>(
        r#"
            SELECT p.id AS post_id
            FROM media_posts p
            WHERE p.deleted_at IS NULL
                AND p.user_id = $1
                AND (
                        $3::timestamptz IS NULL
                        OR p.created_at < $3
                        OR (p.created_at = $3 AND p.id < $4)
                    )
                AND p.deleted_at IS NULL
                AND (
                        (
                            $2 IS NULL
                            AND p.visibility = 'everyone'::post_visibility
                        )

                        OR 

                        (
                            -- The post belongs to the current user
                            $2 IS NOT NULL
                            AND p.user_id = $2
                        )

                        OR

                        (
                            $2 IS NOT NULL
                            AND (
                                -- Everyone can see it
                                p.visibility = 'everyone'::post_visibility

                                OR

                                -- Both users follow each other
                                (
                                    p.visibility = 'friend'::post_visibility
                                    AND EXISTS (
                                        SELECT 1
                                        FROM user_follow uf1
                                        JOIN user_follow uf2
                                            ON uf1.follower_id = uf2.user_id
                                        AND uf2.follower_id = uf1.user_id
                                        WHERE uf1.user_id = p.user_id
                                        AND uf2.user_id = $2
                                    )
                                )
                            )
                        )
                    )
            ORDER BY p.created_at DESC, p.id DESC
            LIMIT $5;
        "#,
    )
    .bind(target_id)
    .bind(requester_id)
    .bind(before)
    .bind(before_id)
    .bind(limit)
    .fetch_all(tx.as_mut())
    .await?;
    Ok(posts
        .into_iter()
        .map(|user_posts_row| user_posts_row.post_id)
        .collect())
}

pub async fn base_comment(
    tx: &mut Transaction<'_, Postgres>,
    comment_id: i64,
    requester_id: i64,
) -> RepositoryResult<CommentBaseRow> {
    let comment = sqlx::query_as::<_, CommentBaseRow>(
        r#"
        SELECT
            c.user_id as author_id,
            c.id::TEXT as id,
            c.post_id::TEXT as post_id,
            c.content,
            (
                EXISTS (
                    SELECT 1
                    FROM media_likes ml
                    WHERE ml.target_id = c.id
                      AND ml.target_type = 'comment'
                      AND ml.user_id = $2
                      AND ml.is_like = TRUE
                )
            ) AS is_liked,
            c.total_likes,
            c.has_attachment,
            COALESCE(
                (
                    SELECT jsonb_agg(
                        jsonb_build_object(
                            'id', gma.id::text,
                            'file_type', gma.file_type,
                            'processing_state', gma.processing_state,
                            'post_processing_state', gma.post_processing_state,
                            'flags', gma.flags::text,

                            'media_objects', gma.media_objects,
                            'media_object_metadata', gma.media_object_metadata,

                            'media_hls', gma.media_hls,
                            'media_hls_playlists', gma.media_hls_playlists,

                            'created_at', gma.created_at,
                            'updated_at', gma.updated_at
                        )
                        ORDER BY gma.id
                    )
                    FROM media_attachments ma
                    JOIN get_media_by_id_without_playlists(ma.media_id) gma
                        ON gma.id = ma.media_id
                    WHERE ma.target_id = c.id
                      AND gma.deleted_at IS NULL
                ),
                '[]'::jsonb
            ) AS attachments,
            c.created_at,
            c.updated_at
        FROM media_comments c
        WHERE c.id = $1
          AND c.deleted_at IS NULL
          AND c.status != 'inactive'
        "#,
    )
    .bind(comment_id)
    .bind(requester_id)
    .fetch_one(tx.as_mut())
    .await?;
    Ok(comment)
}

pub async fn post_comment_ids(
    tx: &mut Transaction<'_, Postgres>,
    post_id: i64,
    requester_id: i64,
    before: chrono::DateTime<chrono::Utc>,
    limit: i64,
) -> RepositoryResult<Vec<i64>> {
    let comments = sqlx::query_as::<_, CommentIdsRow>(
        r#"
            SELECT c.id AS comment_id
            FROM media_comments c
            JOIN media_posts p
                ON p.id = c.post_id
            WHERE c.post_id = $1
                AND c.deleted_at IS NULL
                AND p.id = c.post_id
                AND p.deleted_at IS NULL
                AND (
                        (
                            $2 IS NULL
                            AND p.visibility = 'everyone'::post_visibility
                        )

                        OR 

                        (
                            -- The post belongs to the current user
                            $2 IS NOT NULL
                            AND p.user_id = $2
                        )

                        OR

                        (
                            $2 IS NOT NULL
                            AND (
                                -- Everyone can see it
                                p.visibility = 'everyone'::post_visibility

                                OR

                                -- Both users follow each other
                                (
                                    p.visibility = 'friend'::post_visibility
                                    AND EXISTS (
                                        SELECT 1
                                        FROM user_follow uf1
                                        JOIN user_follow uf2
                                            ON uf1.follower_id = uf2.user_id
                                        AND uf2.follower_id = uf1.user_id
                                        WHERE uf1.user_id = p.user_id
                                        AND uf2.user_id = $2
                                    )
                                )
                            )
                        )
                    )
            AND ($3 IS NULL OR c.created_at < $3)
            ORDER BY c.created_at DESC
            LIMIT $4;
        "#,
    )
    .bind(post_id)
    .bind(requester_id)
    .bind(before)
    .bind(limit)
    .fetch_all(tx.as_mut())
    .await?;
    Ok(comments
        .into_iter()
        .map(|comment_row| comment_row.comment_id)
        .collect())
}

pub async fn query_explore_posts(
    tx: &mut sqlx::Transaction<'_, sqlx::Postgres>,
    before: Option<chrono::DateTime<chrono::Utc>>,
    before_id: Option<i64>,
    tag_id: Option<i64>,
    limit: i64,
) -> RepositoryResult<Vec<i64>> {
    let ids = sqlx::query_as::<_, UserPostsRow>(
        r#"
            SELECT p.id AS post_id
            FROM media_posts p
            WHERE p.deleted_at IS NULL
                AND p.visibility = 'everyone'::post_visibility
                AND (
                        $1::timestamptz IS NULL
                        OR p.created_at < $1
                        OR (p.created_at = $1 AND p.id < $2)
                    )
                AND (
                        $3::bigint IS NULL
                        OR EXISTS (
                            SELECT 1
                            FROM tag_attachments ta
                            WHERE ta.target_id = p.id
                                AND ta.target_type = 'post'
                                AND ta.tag_id = $3
                        )
                    )
            ORDER BY p.created_at DESC, p.id DESC
            LIMIT $4;
        "#,
    )
    .bind(before)
    .bind(before_id)
    .bind(tag_id)
    .bind(limit)
    .fetch_all(tx.as_mut())
    .await?;
    Ok(ids
        .into_iter()
        .map(|post_row| post_row.post_id)
        .collect())
}
