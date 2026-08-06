-- Add down migration script here
ALTER TABLE media_likes 
    DROP is_liked IF EXISTS;

ALTER TABLE media_likes
    DROP CONSTRAINT media_likes_pkey;

ALTER TABLE media_likes
    DROP CONSTRAINT IF EXISTS media_likes_user_id_media_post_id_key;

ALTER TABLE media_posts
    DROP media_tags IF EXISTS;

ALTER TABLE users
    DROP interest_tags IF EXISTS;

ALTER TABLE users  
    DROP visibility IF EXISTS;

ALTER TABLE media_posts
    DROP visibility IF EXISTS;

ALTER TABLE media_likes 
    DROP CONSTRAINT media_likes_pkey IF EXISTS;

ALTER TABLE media_likes
    DROP id IF EXISTS;
