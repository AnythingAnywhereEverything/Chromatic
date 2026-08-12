-- Add down migration script here
-- 1. Restore id as BIGINT for Snowflake IDs
ALTER TABLE media_likes 
    ADD COLUMN id BIGINT PRIMARY KEY;

-- 2. Drop the status column and the custom ENUM type
-- (Postgres requires dropping the column before the TYPE)
ALTER TABLE media_posts 
    DROP COLUMN IF EXISTS status;

-- If you created a custom type for the ENUM, drop it here:
DROP TYPE IF EXISTS post_status; 

-- 3. Restore the old status column (Assuming it was TEXT/VARCHAR)
ALTER TABLE media_posts 
    ADD COLUMN status TEXT;

-- 4. Restore media_tags
-- Using JSONB as it's the gold standard for tags in Postgres
ALTER TABLE media_posts 
    ADD COLUMN media_tags TEXT[];


ALTER TABLE media_posts
    DROP COLUMN IF EXISTS visibility;

ALTER TABLE media_posts
    ADD visibility varchar(20);

DROP TYPE IF EXISTS post_status;
DROP TYPE IF EXISTS post_visibility;

DROP TABLE IF EXISTS tags_attachments;

CREATE TABLE IF NOT EXISTS user_interest (
    user_id BIGINT NOT NULL,
    tag_id BIGINT NOT NULL REFERENCES interest_tags(id) ON DELETE CASCADE,
    PRIMARY KEY (user_id, tag_id)
);

CREATE TABLE IF NOT EXISTS media_tags (
    media_id BIGINT NOT NULL,
    tag_id BIGINT NOT NULL REFERENCES interest_tags(id) ON DELETE CASCADE,
    PRIMARY KEY (media_id, tag_id)
);


ALTER TABLE interest_tags
    RENAME COLUMN tag_name TO tags_name;