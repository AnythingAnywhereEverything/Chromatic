-- Normalize user_follow row direction to (user_id = followed, follower_id = follower).
-- Older code written rows in the reverse order (user_id = requester, follower_id = target).
-- Rows that already have their reverse row present (mutual follows) are left untouched
-- because both orientations already exist; single-direction rows are swapped.
UPDATE user_follow uf
SET user_id = uf.follower_id,
    follower_id = uf.user_id
WHERE NOT EXISTS (
    SELECT 1
    FROM user_follow rev
    WHERE rev.user_id = uf.follower_id
      AND rev.follower_id = uf.user_id
);