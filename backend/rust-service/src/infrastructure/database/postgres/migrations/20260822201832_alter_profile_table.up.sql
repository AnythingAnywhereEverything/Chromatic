-- Add up migration script here
ALTER TABLE user_profiles
ADD COLUMN posts_count INT DEFAULT 0;

-- count the posts for each user and update the posts_count column
UPDATE user_profiles
SET
    posts_count = (
        SELECT
            COUNT(*)
        FROM
            media_posts
        WHERE
            media_posts.user_id = user_profiles.user_id
    );