-- Add down migration script here

ALTER TABLE user_profiles
    DROP CONSTRAINT IF EXISTS fk_avatar_media_id;

ALTER TABLE user_profiles
    DROP CONSTRAINT IF EXISTS fk_banner_media_id;

ALTER TABLE user_profiles
    RENAME COLUMN avatar_media_id TO avatar_url;

ALTER TABLE user_profiles
    RENAME COLUMN banner_media_id TO banner_url;

ALTER TABLE user_profiles
    ALTER COLUMN avatar_url TYPE TEXT USING avatar_url::TEXT

ALTER TABLE user_profiles
    ALTER COLUMN banner_url TYPE TEXT USING banner_url::TEXT;