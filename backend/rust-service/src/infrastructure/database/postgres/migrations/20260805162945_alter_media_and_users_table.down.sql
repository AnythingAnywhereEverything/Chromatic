-- Add down migration script here

DROP INDEX IF EXISTS media_posts_tags_x;
DROP INDEX IF EXISTS users_interest_tags_x;

ALTER TABLE media_likes
    DROP COLUMN IF EXISTS is_liked;

ALTER TABLE media_posts
    DROP COLUMN IF EXISTS media_tags;

ALTER TABLE users
    DROP COLUMN IF EXISTS interest_tags;

ALTER TABLE media_posts
    DROP COLUMN IF EXISTS visibility;