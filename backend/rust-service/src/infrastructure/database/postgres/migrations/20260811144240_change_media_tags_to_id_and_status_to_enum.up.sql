-- Add up migration script here
-- 1. Create the custom ENUM type first
CREATE TYPE post_status AS ENUM ('pending', 'active', 'inactive', 'moderate');
CREATE TYPE post_visibility AS ENUM ('everyone', 'friend', 'private');

-- 2. Modify media_posts
ALTER TABLE media_posts
    DROP COLUMN IF EXISTS media_tags,
    DROP COLUMN IF EXISTS status;

ALTER TABLE media_posts
    ADD COLUMN status post_status DEFAULT 'active';

-- 3. Modify media_likes
ALTER TABLE media_likes
    DROP COLUMN IF EXISTS id;

ALTER TABLE media_posts
    DROP COLUMN IF EXISTS visibility;

ALTER TABLE media_posts
    ADD COLUMN visibility post_visibility DEFAULT 'everyone';

DROP TABLE IF EXISTS user_interest;
DROP TABLE IF EXISTS media_tags;

CREATE TABLE IF NOT EXISTS tag_attachments(
    target_id BIGINT NOT NULL,
    -- user OR media
    target_type VARCHAR(32) NOT NULL,
    tag_id BIGINT NOT NULL REFERENCES interest_tags(id) ON DELETE CASCADE,
    PRIMARY KEY(target_id, tag_id, target_type)
);

ALTER TABLE interest_tags
    RENAME COLUMN tags_name TO tag_name;