-- Add up migration script here
-- use gin on the content column of media_posts for faster text search
CREATE EXTENSION IF NOT EXISTS pg_trgm;

CREATE INDEX IF NOT EXISTS idx_media_posts_content_trgm
ON media_posts 
USING gin (content gin_trgm_ops);
