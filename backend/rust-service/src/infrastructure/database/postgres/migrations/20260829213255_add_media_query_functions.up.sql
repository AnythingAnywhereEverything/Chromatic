-- Add up migration script here

-- create function to query media by id

CREATE OR REPLACE FUNCTION get_media_by_id(p_media_id BIGINT)
RETURNS TABLE (
    id BIGINT,
    uploader_id BIGINT,
    original_name TEXT,
    original_content_type TEXT,
    file_type media_type,
    lock_hash TEXT,
    lock_expiration TIMESTAMP,
    processing_state media_processing_state,
    post_processing_state media_post_processing_state,
    flags BIGINT,

    media_objects JSONB,
    media_object_metadata JSONB,

    media_hls JSONB,
    media_hls_playlists JSONB,

    created_at TIMESTAMP,
    updated_at TIMESTAMP,
    deleted_at TIMESTAMP
)
LANGUAGE SQL
STABLE
AS $$
    SELECT
        m.id,
        m.uploader_id,
        m.original_name,
        m.original_content_type,
        m.file_type,
        m.lock_hash,
        m.lock_expiration,
        m.processing_state,
        m.post_processing_state,
        m.flags,

        COALESCE(
            (
                SELECT jsonb_agg(
                    jsonb_build_object(
                        'kind', o.kind,
                        'storage_key', o.storage_key,
                        'content_type', o.content_type,
                        'size', o.size,
                        'name', o.name,
                        'thumbhash', o.thumbhash,
                        'created_at', o.created_at,
                        'updated_at', o.updated_at,
                        'deleted_at', o.deleted_at
                    )
                    ORDER BY o.kind
                )
                FROM media_objects o
                WHERE o.media_id = m.id
            ),
            '[]'::jsonb
        ) AS media_objects,

        (
            SELECT jsonb_build_object(
                'width', md.width,
                'height', md.height,
                'duration', md.duration,
                'created_at', md.created_at,
                'updated_at', md.updated_at
            )
            FROM media_object_metadata md
            WHERE md.media_id = m.id
        ) AS media_object_metadata,

        CASE
            WHEN m.file_type = 'hls'::media_type THEN (
                SELECT jsonb_build_object(
                    'master_playlist', h.master_playlist,
                    'created_at', h.created_at,
                    'updated_at', h.updated_at
                )
                FROM media_hls h
                WHERE h.media_id = m.id
            )
            ELSE NULL
        END AS media_hls,

        CASE
            WHEN m.file_type = 'hls'::media_type THEN COALESCE(
                (
                    SELECT jsonb_agg(
                        jsonb_build_object(
                            'resolution', hp.resolution,
                            'playlist_storage_key', hp.playlist_storage_key,
                            'segment_count', hp.segment_count,
                            'segment_duration', hp.segment_duration,
                            'created_at', hp.created_at,
                            'updated_at', hp.updated_at
                        )
                        ORDER BY hp.resolution
                    )
                    FROM media_hls_playlists hp
                    WHERE hp.media_id = m.id
                ),
                '[]'::jsonb
            )
            ELSE '[]'::jsonb
        END AS media_hls_playlists,

        m.created_at,
        m.updated_at,
        m.deleted_at

    FROM media m
    WHERE m.id = p_media_id;
$$;

CREATE OR REPLACE FUNCTION get_media_by_id_without_playlists(p_media_id BIGINT)
RETURNS TABLE (
    id BIGINT,
    uploader_id BIGINT,
    original_name TEXT,
    original_content_type TEXT,
    file_type media_type,
    lock_hash TEXT,
    lock_expiration TIMESTAMP,
    processing_state media_processing_state,
    post_processing_state media_post_processing_state,
    flags BIGINT,

    media_objects JSONB,
    media_object_metadata JSONB,

    media_hls JSONB,
    media_hls_playlists JSONB,

    created_at TIMESTAMP,
    updated_at TIMESTAMP,
    deleted_at TIMESTAMP
)
LANGUAGE SQL
STABLE
AS $$
    SELECT
        m.id,
        m.uploader_id,
        m.original_name,
        m.original_content_type,
        m.file_type,
        m.lock_hash,
        m.lock_expiration,
        m.processing_state,
        m.post_processing_state,
        m.flags,

        COALESCE(
            (
                SELECT jsonb_agg(
                    jsonb_build_object(
                        'kind', o.kind,
                        'storage_key', o.storage_key,
                        'content_type', o.content_type,
                        'size', o.size,
                        'name', o.name,
                        'thumbhash', o.thumbhash,
                        'created_at', o.created_at,
                        'updated_at', o.updated_at,
                        'deleted_at', o.deleted_at
                    )
                    ORDER BY o.kind
                )
                FROM media_objects o
                WHERE o.media_id = m.id
            ),
            '[]'::jsonb
        ) AS media_objects,

        (
            SELECT jsonb_build_object(
                'width', md.width,
                'height', md.height,
                'duration', md.duration,
                'created_at', md.created_at,
                'updated_at', md.updated_at
            )
            FROM media_object_metadata md
            WHERE md.media_id = m.id
        ) AS media_object_metadata,

        CASE
            WHEN m.file_type = 'hls'::media_type THEN (
                SELECT jsonb_build_object(
                    'master_playlist', h.master_playlist,
                    'created_at', h.created_at,
                    'updated_at', h.updated_at
                )
                FROM media_hls h
                WHERE h.media_id = m.id
            )
            ELSE NULL
        END AS media_hls,

        CASE
            WHEN m.file_type = 'hls'::media_type
                 AND EXISTS (
                    SELECT 1
                    FROM media_hls h
                    WHERE h.media_id = m.id
                 )
            THEN '[]'::jsonb

            WHEN m.file_type = 'hls'::media_type THEN COALESCE(
                (
                    SELECT jsonb_agg(
                        jsonb_build_object(
                            'resolution', hp.resolution,
                            'playlist_storage_key', hp.playlist_storage_key,
                            'segment_count', hp.segment_count,
                            'segment_duration', hp.segment_duration,
                            'created_at', hp.created_at,
                            'updated_at', hp.updated_at
                        )
                        ORDER BY hp.resolution
                    )
                    FROM media_hls_playlists hp
                    WHERE hp.media_id = m.id
                ),
                '[]'::jsonb
            )

            ELSE '[]'::jsonb
        END AS media_hls_playlists,

        m.created_at,
        m.updated_at,
        m.deleted_at

    FROM media m
    WHERE m.id = p_media_id;
$$;

-- post id query function

CREATE OR REPLACE FUNCTION get_post_by_id(
    p_post_id BIGINT,
    p_user_id BIGINT DEFAULT NULL
)
RETURNS TABLE(
    author JSONB,

    post_id TEXT,
    content TEXT,

    total_likes INT,
    total_comments INT,

    is_reposted BOOLEAN,
    reposted_post JSONB,

    visibility post_visibility,
    tags JSONB,

    is_liked BOOLEAN,

    has_attachment BOOLEAN,
    attachments JSONB,

    created_at TIMESTAMP,
    updated_at TIMESTAMP
)
LANGUAGE sql
STABLE
AS $$
    SELECT
        (
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
            JOIN media_objects mo ON mo.media_id = up.avatar_media_id
            WHERE u.id = p.user_id
            AND u.deleted_at IS NULL
        ) AS author,

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
                    JOIN media_objects mo ON mo.media_id = up.avatar_media_id
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
                AND ml.user_id = p_user_id
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
        ) AS attachment,
        p.created_at,
        p.updated_at
    FROM media_posts p
    WHERE p.id = p_post_id
    AND p.deleted_at IS NULL
    AND (
            (
                p_user_id IS NULL
                AND p.visibility = 'everyone'::post_visibility
            )

            OR

            (
                p_user_id IS NOT NULL
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
                            AND uf2.user_id = p_user_id
                        )
                    )
                )
            )
        );
$$;


-- get post amount for feed
-- * this is temporary and will be replace with LTR later

CREATE OR REPLACE FUNCTION get_post_amount_for_feed(
    p_user_id BIGINT,
    p_limit INT
)
RETURNS TABLE(
    author JSONB,
    post_id TEXT,
    content TEXT,
    total_likes INT,
    total_comments INT,
    is_reposted BOOLEAN,
    reposted_post JSONB,
    visibility post_visibility,
    tags JSONB,
    is_liked BOOLEAN,
    has_attachment BOOLEAN,
    attachments JSONB,
    created_at TIMESTAMP,
    updated_at TIMESTAMP
)
LANGUAGE sql
STABLE
AS $$
    SELECT gp.*
    FROM media_posts p
    CROSS JOIN LATERAL get_post_by_id(p.id, p_user_id) gp
    WHERE p.deleted_at IS NULL
    ORDER BY p.created_at DESC
    LIMIT p_limit;
$$;