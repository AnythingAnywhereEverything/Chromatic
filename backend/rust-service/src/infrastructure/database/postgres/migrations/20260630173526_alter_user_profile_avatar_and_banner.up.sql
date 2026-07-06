-- Add up migration script here

ALTER TABLE user_profiles
    RENAME COLUMN avatar_url TO avatar_media_id;

ALTER TABLE user_profiles
    RENAME COLUMN banner_url TO banner_media_id;

ALTER TABLE user_profiles
    ALTER COLUMN avatar_media_id TYPE BIGINT USING avatar_media_id::BIGINT;

ALTER TABLE user_profiles
    ALTER COLUMN banner_media_id TYPE BIGINT USING banner_media_id::BIGINT;

ALTER TABLE user_profiles
    ADD CONSTRAINT fk_avatar_media_id FOREIGN KEY (avatar_media_id) REFERENCES media_data(id) ON DELETE SET NULL;

ALTER TABLE user_profiles
    ADD CONSTRAINT fk_banner_media_id FOREIGN KEY (banner_media_id) REFERENCES media_data(id) ON DELETE SET NULL;