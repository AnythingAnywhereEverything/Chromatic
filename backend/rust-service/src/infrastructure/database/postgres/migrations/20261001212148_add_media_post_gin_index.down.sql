-- Add down migration script here
-- Remove the gin index on the content column of media_posts
DROP INDEX IF EXISTS idx_media_posts_content_trgm;