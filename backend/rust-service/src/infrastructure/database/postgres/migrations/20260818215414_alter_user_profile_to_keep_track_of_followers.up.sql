-- Add up migration script here
ALTER TABLE user_profiles
ADD COLUMN followers_count INT DEFAULT 0,
ADD COLUMN following_count INT DEFAULT 0;

-- count from follower_following table
UPDATE user_profiles up
SET followers_count = (SELECT COUNT(*) FROM user_follow uf WHERE uf.user_id = up.user_id AND uf.status = 'accepted'),
following_count = (SELECT COUNT(*) FROM user_follow uf WHERE uf.follower_id = up.user_id AND uf.status = 'accepted');