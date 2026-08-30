-- Add up migration script here

-- create media type enum
CREATE TYPE media_type AS ENUM (
    'image',
    'video',
    'hls',
    'audio',
    'document',
    'other'
);

CREATE TYPE media_kind AS ENUM (
    'original',
    'thumbnail',
    'preview', -- basically thumbnail
    'transcoded'
);

CREATE TYPE media_processing_state AS ENUM (
    'pending',
    'ready',
    'completed',
    'failed'
);

CREATE TYPE media_post_processing_state AS ENUM (
    'idle',
    'processing',
    'completed',
    'failed'
);

-- a group of media that does not contain the path but is related to the same media item
-- based media group is used to store the derivatives of the media item, such as hls playlist, thumbnails, etc.
CREATE TABLE media (
    id BIGINT PRIMARY KEY,
    uploader_id BIGINT REFERENCES users(id) ON DELETE CASCADE,

    original_name TEXT DEFAULT '',
    original_content_type TEXT DEFAULT '',

    file_type media_type NOT NULL,

    -- nullable value, use only when media have to transfer between services
    -- e.g. Rust to Elixir, or Elixir to Rust, etc.
    lock_hash TEXT,
    lock_expiration TIMESTAMP,

    processing_state media_processing_state NOT NULL DEFAULT 'pending',
    post_processing_state media_post_processing_state NOT NULL DEFAULT 'idle',

    flags BIGINT NOT NULL DEFAULT 0,

    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    deleted_at TIMESTAMP DEFAULT NULL
);

-- the following table are derivatives of the media items

CREATE TABLE media_objects (
    media_id BIGINT REFERENCES media(id) ON DELETE CASCADE,
    PRIMARY KEY (media_id, kind),

    kind media_kind NOT NULL,

    storage_key TEXT NOT NULL,
    content_type TEXT NOT NULL,
    size BIGINT NOT NULL,

    name TEXT NOT NULL,
    thumbhash TEXT,

    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    deleted_at TIMESTAMP DEFAULT NULL
);

CREATE TABLE media_object_metadata (
    media_id BIGINT REFERENCES media(id) ON DELETE CASCADE PRIMARY KEY,

    width INT,
    height INT,
    duration FLOAT, -- in seconds, for video and audio files

    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE media_hls (
    media_id BIGINT REFERENCES media(id) ON DELETE CASCADE PRIMARY KEY,
    master_playlist TEXT NOT NULL,

    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE media_hls_playlists (
    media_id BIGINT REFERENCES media(id) ON DELETE CASCADE,
    PRIMARY KEY (media_id, resolution),

    resolution TEXT NOT NULL,
    playlist_storage_key TEXT NOT NULL,

    segment_count INT NOT NULL,
    segment_duration FLOAT NOT NULL, -- in seconds

    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

-- alter users table to drop avatar_media_id and banner_media_id columns and replace it with media table
ALTER TABLE user_profiles
DROP COLUMN avatar_media_id,
DROP COLUMN banner_media_id,
ADD COLUMN avatar_media_id BIGINT REFERENCES media(id) ON DELETE SET NULL,
ADD COLUMN banner_media_id BIGINT REFERENCES media(id) ON DELETE SET NULL;

-- delete all rows from media_attachments, guild_assets, user_recent_avatar, and user_recent_banner tables to avoid foreign key constraint issues
DELETE FROM media_attachments;
DELETE FROM guild_assets;
DELETE FROM user_recent_avatar;
DELETE FROM user_recent_banner;

-- ALTER drop media_attachments on media_id row and replace it with media table references
ALTER TABLE media_attachments
DROP COLUMN media_id,
ADD COLUMN media_id BIGINT NOT NULL REFERENCES media(id) ON DELETE CASCADE;

-- ALTER guild_assets table drop asset_id column and replace it with media table references
ALTER TABLE guild_assets
DROP COLUMN asset_id,
ADD COLUMN asset_id BIGINT NOT NULL REFERENCES media(id) ON DELETE CASCADE;

-- ALTER user_recent_avatar and user_recent_banner tables to replace avatar_id and banner_id columns with media table references

ALTER TABLE user_recent_avatar
DROP COLUMN avatar_id,
ADD COLUMN avatar_id BIGINT NOT NULL REFERENCES media(id) ON DELETE CASCADE PRIMARY KEY;

ALTER TABLE user_recent_banner
DROP COLUMN banner_id,
ADD COLUMN banner_id BIGINT NOT NULL REFERENCES media(id) ON DELETE CASCADE PRIMARY KEY;

ALTER TABLE media_attachments
ADD CONSTRAINT media_attachments_pkey PRIMARY KEY (target_id, media_id);

-- Drop old media_data and media_metadata tables
DROP TABLE IF EXISTS media_metadata;
DROP TABLE IF EXISTS media_data;

-- drop old media_status type
DROP TYPE IF EXISTS media_status;