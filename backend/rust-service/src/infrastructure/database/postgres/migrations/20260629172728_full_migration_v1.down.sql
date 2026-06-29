-- Add down migration script here

-- user_profiles: revert column changes
ALTER TABLE user_profiles
ALTER COLUMN display_name TYPE VARCHAR(255);

ALTER TABLE user_profiles
ALTER COLUMN bio TYPE TEXT;

ALTER TABLE user_profiles
ALTER COLUMN avatar_url TYPE VARCHAR(255);

ALTER TABLE user_profiles
DROP COLUMN IF EXISTS banner_url;

-- user_credentials: remove added column
ALTER TABLE user_credentials
DROP COLUMN IF EXISTS two_factor_enabled;

-- users: revert column changes
ALTER TABLE users
ALTER COLUMN username TYPE VARCHAR(255);

ALTER TABLE users
ALTER COLUMN phone_number TYPE VARCHAR(20);

ALTER TABLE users
DROP COLUMN IF EXISTS is_superuser;


DROP TABLE IF EXISTS audit_logs;
DROP TABLE IF EXISTS message_reactions;
DROP TABLE IF EXISTS messages;

DROP TABLE IF EXISTS guild_assets;
DROP TABLE IF EXISTS guild_bans;
DROP TABLE IF EXISTS guild_invites;
DROP TABLE IF EXISTS guild_channel_overrides;
DROP TABLE IF EXISTS guild_channels;
DROP TABLE IF EXISTS guild_member_roles;
DROP TABLE IF EXISTS guild_roles;
DROP TABLE IF EXISTS guild_members;
DROP TABLE IF EXISTS guilds;

DROP TABLE IF EXISTS reports;

DROP TABLE IF EXISTS user_follow;
DROP TABLE IF EXISTS user_blocks;
DROP TABLE IF EXISTS user_friends;
DROP TABLE IF EXISTS user_notifications;
DROP TABLE IF EXISTS user_settings;
DROP TABLE IF EXISTS user_pinned_posts;

DROP TABLE IF EXISTS user_highlight_stories;
DROP TABLE IF EXISTS user_highlights;
DROP TABLE IF EXISTS user_stories;

DROP TABLE IF EXISTS media_comments;
DROP TABLE IF EXISTS media_bookmarks;
DROP TABLE IF EXISTS media_posts;
DROP TABLE IF EXISTS media_attachments;
DROP TABLE IF EXISTS media_metadata;
DROP TABLE IF EXISTS media_data;

DROP TABLE IF EXISTS user_oauth;
DROP TABLE IF EXISTS user_credentials;
DROP TABLE IF EXISTS user_profiles;
DROP TABLE IF EXISTS user_availabilities;
DROP TABLE IF EXISTS user_statuses;
DROP TABLE IF EXISTS staff_roles;

-- INDEX CLEANUP

DROP INDEX IF EXISTS uq_users_username_active;
DROP INDEX IF EXISTS idx_users_email;
DROP INDEX IF EXISTS idx_users_username;
DROP INDEX IF EXISTS idx_sessions_user_id;
DROP INDEX IF EXISTS idx_user_credentials_user_id;
DROP INDEX IF EXISTS idx_user_profiles_user_id;