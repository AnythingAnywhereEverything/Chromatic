-- Add down migration script here
ALTER TABLE user_profiles
DROP COLUMN followers_count,
DROP COLUMN following_count;