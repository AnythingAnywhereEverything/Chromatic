-- Add down migration script here
ALTER TABLE user_profiles
DROP COLUMN posts_count;