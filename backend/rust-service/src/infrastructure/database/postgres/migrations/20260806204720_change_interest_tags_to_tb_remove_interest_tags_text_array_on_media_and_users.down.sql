-- Add down migration script here
-- Add down migration script here

DROP INDEX IF EXISTS idx_interest_tags_id;

DROP TABLE IF EXISTS media_tags;
DROP TABLE IF EXISTS user_interest;
DROP TABLE IF EXISTS interest_tags;