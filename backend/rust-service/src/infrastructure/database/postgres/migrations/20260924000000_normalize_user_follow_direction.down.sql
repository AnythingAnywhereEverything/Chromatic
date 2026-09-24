-- Reverse of the up migration: swap single-direction rows back to (user_id = requester, follower_id = target).
UPDATE user_follow uf
SET user_id = uf.follower_id,
    follower_id = uf.user_id
WHERE NOT EXISTS (
    SELECT 1
    FROM user_follow rev
    WHERE rev.user_id = uf.follower_id
      AND rev.follower_id = uf.user_id
);