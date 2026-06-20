-- Add down migration script here

-- ALTER TABLE users
-- DROP CONSTRAINT IF EXISTS users_username_key;

-- CREATE UNIQUE INDEX IF NOT EXISTS uq_users_username_active
-- ON users(username)
-- WHERE deleted_at IS NULL;


DROP INDEX IF EXISTS uq_users_username_active;

ALTER TABLE users
ADD CONSTRAINT users_username_key UNIQUE (username);