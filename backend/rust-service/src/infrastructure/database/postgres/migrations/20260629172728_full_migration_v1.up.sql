-- Current DDL

-- CREATE TABLE public.users ( id int8 NOT NULL, email varchar(255) NOT NULL, username varchar(255) NULL, phone_number varchar(20) NULL, email_verified_at timestamptz NULL, is_active bool DEFAULT true NOT NULL, deleted_at timestamptz NULL, created_at timestamptz DEFAULT now() NOT NULL, updated_at timestamptz DEFAULT now() NOT NULL, CONSTRAINT users_email_key UNIQUE (email), CONSTRAINT users_pkey PRIMARY KEY (id));
-- CREATE INDEX idx_users_email ON public.users USING btree (email);
-- CREATE INDEX idx_users_username ON public.users USING btree (username);
-- CREATE UNIQUE INDEX uq_users_username_active ON public.users USING btree (username) WHERE (deleted_at IS NULL);

-- CREATE TABLE public.sessions ( id int8 NOT NULL, user_id int8 NOT NULL, session_hashed varchar(255) NOT NULL, user_agent text NULL, ip_address varchar(45) NULL, created_at timestamptz DEFAULT now() NOT NULL, CONSTRAINT sessions_pkey PRIMARY KEY (id), CONSTRAINT sessions_session_hashed_key UNIQUE (session_hashed), CONSTRAINT sessions_user_id_fkey FOREIGN KEY (user_id) REFERENCES public.users(id) ON DELETE CASCADE);
-- CREATE INDEX idx_sessions_user_id ON public.sessions USING btree (user_id);

-- CREATE TABLE public.user_credentials ( user_id int8 NOT NULL, password_hash varchar(255) NULL, password_updated_at timestamptz NULL, failed_login_count int4 DEFAULT 0 NOT NULL, lockout_until timestamptz NULL, updated_at timestamptz DEFAULT now() NOT NULL, CONSTRAINT user_credentials_pkey PRIMARY KEY (user_id), CONSTRAINT user_credentials_user_id_fkey FOREIGN KEY (user_id) REFERENCES public.users(id) ON DELETE CASCADE);
-- CREATE INDEX idx_user_credentials_user_id ON public.user_credentials USING btree (user_id);

-- CREATE TABLE public.user_oauth ( id int8 NOT NULL, user_id int8 NOT NULL, provider varchar(50) NOT NULL, provider_user_id varchar(255) NOT NULL, created_at timestamptz DEFAULT now() NOT NULL, CONSTRAINT user_oauth_pkey PRIMARY KEY (id), CONSTRAINT user_oauth_provider_provider_user_id_key UNIQUE (provider, provider_user_id), CONSTRAINT user_oauth_user_id_fkey FOREIGN KEY (user_id) REFERENCES public.users(id) ON DELETE CASCADE);

-- CREATE TABLE public.user_profiles ( user_id int8 NOT NULL, display_name varchar(255) NULL, bio text NULL, avatar_url varchar(255) NULL, updated_at timestamptz DEFAULT now() NOT NULL, CONSTRAINT user_profiles_pkey PRIMARY KEY (user_id), CONSTRAINT user_profiles_user_id_fkey FOREIGN KEY (user_id) REFERENCES public.users(id) ON DELETE CASCADE);
-- CREATE INDEX idx_user_profiles_user_id ON public.user_profiles USING btree (user_id);


-- Alter user table for superuser
-- Alter user username to 32 characters
-- alter user phone number to 32 characters
ALTER TABLE users
ADD COLUMN is_superuser bool DEFAULT false NOT NULL;

ALTER TABLE users
ALTER COLUMN username TYPE VARCHAR(32);

ALTER TABLE users
ALTER COLUMN phone_number TYPE VARCHAR(32);

-- Alter user_profiles display_name to 32 characters
-- Alter Bio to 512 characters
ALTER TABLE user_profiles
ALTER COLUMN display_name TYPE VARCHAR(32);

ALTER TABLE user_profiles
ALTER COLUMN bio TYPE VARCHAR(512);

-- create staff_role table
CREATE TABLE IF NOT EXISTS staff_roles (
    id BIGINT NOT NULL PRIMARY KEY,
    name VARCHAR(255) NOT NULL UNIQUE,
    description TEXT NULL,
    position INT NOT NULL,
    permission_bitmask BIGINT[] NOT NULL,
    created_at TIMESTAMPTZ DEFAULT now() NOT NULL,
    updated_at TIMESTAMPTZ DEFAULT now() NOT NULL
);

-- create user_statuses table
CREATE TABLE IF NOT EXISTS user_statuses (
    user_id BIGINT NOT NULL PRIMARY KEY REFERENCES users(id) ON DELETE CASCADE,
    message VARCHAR(128) NOT NULL,
    clear_at TIMESTAMPTZ NULL,
    created_at TIMESTAMPTZ DEFAULT now() NOT NULL,
    updated_at TIMESTAMPTZ DEFAULT now() NOT NULL
);

-- create user_availabilities table
-- this table will store the availability status of users, such as online, offline, busy, etc.
CREATE TABLE IF NOT EXISTS user_availabilities (
    user_id BIGINT NOT NULL PRIMARY KEY REFERENCES users(id) ON DELETE CASCADE,
    status VARCHAR(32) NOT NULL,
    is_locked BOOLEAN DEFAULT false NOT NULL,
    start_time TIMESTAMPTZ NOT NULL,
    end_time TIMESTAMPTZ NULL,
    created_at TIMESTAMPTZ DEFAULT now() NOT NULL,
    updated_at TIMESTAMPTZ DEFAULT now() NOT NULL
);
-- endtime is null means the user is currently in that status, otherwise the user has left that status.
-- is_locked is used to indicate if the user is locked in that status and cannot change it until the end_time is reached.
-- or they go offline or change their status manually.

-- create media_data table
CREATE TABLE IF NOT EXISTS media_data (
    id BIGINT NOT NULL PRIMARY KEY,
    user_id BIGINT NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    media_url TEXT NOT NULL,
    media_preview_url TEXT NULL,
    media_category VARCHAR(32) NOT NULL,
    created_at TIMESTAMPTZ DEFAULT now() NOT NULL,
    deleted_at TIMESTAMPTZ NULL
);
-- no updated_at column because media data is immutable, if the media data needs to be updated, a new record should be created instead of updating the existing one.

-- create media_metadata table
CREATE TABLE IF NOT EXISTS media_metadata (
    media_id BIGINT NOT NULL PRIMARY KEY REFERENCES media_data(id) ON DELETE CASCADE,
    file_size BIGINT NOT NULL,
    mime_type VARCHAR(64) NOT NULL,
    width INT NULL,
    height INT NULL,
    duration INT NULL
);

-- create media_posts table
CREATE TABLE IF NOT EXISTS media_posts (
    id BIGINT NOT NULL PRIMARY KEY,
    user_id BIGINT NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    content VARCHAR(2500) NULL,
    total_comments INT DEFAULT 0 NOT NULL,
    total_likes INT DEFAULT 0 NOT NULL,
    status VARCHAR(32) DEFAULT 'active' NOT NULL, 
    -- status can be active, deleted, archived, etc.
    reposted_from BIGINT NULL REFERENCES media_posts(id) ON DELETE SET NULL,
    is_repost BOOLEAN DEFAULT false NOT NULL,

    has_attachment BOOLEAN DEFAULT false NOT NULL,

    created_at TIMESTAMPTZ DEFAULT now() NOT NULL,
    updated_at TIMESTAMPTZ DEFAULT now() NOT NULL,
    deleted_at TIMESTAMPTZ NULL,

    -- check if the content is is not empty if the post has no attachments, and if the post has attachments, the content can be empty.
    CONSTRAINT chk_content_or_attachment CHECK (content IS NOT NULL OR has_attachment = true)
);

-- add media_attachments table
CREATE TABLE IF NOT EXISTS media_attachments (
    target_id BIGINT NOT NULL,
    -- target can be messages, posts, comments, etc. and can be used to attach media to those targets.
    target_type VARCHAR(32) NOT NULL,
    media_id BIGINT NOT NULL REFERENCES media_data(id) ON DELETE CASCADE,
    PRIMARY KEY (target_id, media_id)
);

-- create media_comments table
CREATE TABLE IF NOT EXISTS media_comments (
    id BIGINT NOT NULL PRIMARY KEY,
    post_id BIGINT NOT NULL REFERENCES media_posts(id) ON DELETE CASCADE,
    user_id BIGINT NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    content VARCHAR(1000) NULL,

    has_attachment BOOLEAN DEFAULT false NOT NULL,

    total_likes INT DEFAULT 0 NOT NULL,
    status VARCHAR(32) DEFAULT 'active' NOT NULL,
    -- status can be active, deleted, archived, etc.
    created_at TIMESTAMPTZ DEFAULT now() NOT NULL,
    updated_at TIMESTAMPTZ DEFAULT now() NOT NULL,
    deleted_at TIMESTAMPTZ NULL
);

-- create media_bookmarks table
CREATE TABLE IF NOT EXISTS media_bookmarks (
    user_id BIGINT NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    post_id BIGINT NOT NULL REFERENCES media_posts(id) ON DELETE CASCADE,
    created_at TIMESTAMPTZ DEFAULT now() NOT NULL,
    PRIMARY KEY (user_id, post_id)
);

--create user_stories table
CREATE TABLE IF NOT EXISTS user_stories (
    id BIGINT NOT NULL PRIMARY KEY,
    user_id BIGINT NOT NULL REFERENCES users(id) ON DELETE CASCADE,

    canvas_aspect_ratio VARCHAR(16) NOT NULL DEFAULT '16:9',
    canvas_background_color VARCHAR(16) NULL,
    total_duration INT DEFAULT 0 NOT NULL,

    total_views INT DEFAULT 0 NOT NULL,
    total_replies INT DEFAULT 0 NOT NULL,

    layers JSONB NULL,
    -- layers will store the story layers, such as text, images, videos, etc. and their properties, such as position, size, rotation, etc.

    title VARCHAR(255) NULL,
    description VARCHAR(1000) NULL,
    status VARCHAR(32) DEFAULT 'active' NOT NULL,
    
    -- status can be active, deleted, archived, etc.
    created_at TIMESTAMPTZ DEFAULT now() NOT NULL,
    updated_at TIMESTAMPTZ DEFAULT now() NOT NULL,
    deleted_at TIMESTAMPTZ NULL,
    expires_at TIMESTAMPTZ NOT NULL DEFAULT now() + interval '24 hours'
);

-- create user_highlights table
CREATE TABLE IF NOT EXISTS user_highlights (
    id BIGINT NOT NULL PRIMARY KEY,
    user_id BIGINT NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    title VARCHAR(64) NOT NULL,

    cover_story_id BIGINT NULL REFERENCES user_stories(id) ON DELETE SET NULL,
    total_stories INT DEFAULT 0 NOT NULL,

    created_at TIMESTAMPTZ DEFAULT now() NOT NULL,
    updated_at TIMESTAMPTZ DEFAULT now() NOT NULL,
    deleted_at TIMESTAMPTZ NULL
);

-- create user_highlight_stories table
CREATE TABLE IF NOT EXISTS user_highlight_stories (
    highlight_id BIGINT NOT NULL REFERENCES user_highlights(id) ON DELETE CASCADE,
    story_id BIGINT NOT NULL REFERENCES user_stories(id) ON DELETE CASCADE,
    added_at TIMESTAMPTZ DEFAULT now() NOT NULL,
    PRIMARY KEY (highlight_id, story_id)
);

-- ALTER user profiles
ALTER TABLE user_profiles
ADD COLUMN banner_url VARCHAR(255) NULL;

ALTER TABLE user_profiles
ALTER COLUMN avatar_url TYPE VARCHAR(255);

--- ALTER user_credentials
ALTER TABLE user_credentials
ADD COLUMN two_factor_enabled BOOLEAN DEFAULT false NOT NULL;

-- create user_pinned_posts table
CREATE TABLE IF NOT EXISTS user_pinned_posts (
    user_id BIGINT NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    post_id BIGINT NOT NULL REFERENCES media_posts(id) ON DELETE CASCADE,
    pinned_at TIMESTAMPTZ DEFAULT now() NOT NULL,
    PRIMARY KEY (user_id, post_id)
);

-- create user settings table
CREATE TABLE IF NOT EXISTS user_settings (
    user_id BIGINT NOT NULL PRIMARY KEY REFERENCES users(id) ON DELETE CASCADE,
    setting_key VARCHAR(64) NOT NULL,
    setting_value JSONB NOT NULL,
    created_at TIMESTAMPTZ DEFAULT now() NOT NULL,
    updated_at TIMESTAMPTZ DEFAULT now() NOT NULL
);

-- create user_notifications table
CREATE TABLE IF NOT EXISTS user_notifications (
    id BIGINT NOT NULL PRIMARY KEY,
    user_id BIGINT NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    notification_type VARCHAR(64) NOT NULL,
    notification_data JSONB NOT NULL,
    is_read BOOLEAN DEFAULT false NOT NULL,
    created_at TIMESTAMPTZ DEFAULT now() NOT NULL,
    updated_at TIMESTAMPTZ DEFAULT now() NOT NULL
);

-- create user_friends table
CREATE TABLE IF NOT EXISTS user_friends (
    user_id BIGINT NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    friend_id BIGINT NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    status VARCHAR(32) DEFAULT 'pending' NOT NULL,
    -- status can be pending, accepted, blocked, etc.
    created_at TIMESTAMPTZ DEFAULT now() NOT NULL,
    updated_at TIMESTAMPTZ DEFAULT now() NOT NULL,
    PRIMARY KEY (user_id, friend_id)
);

-- create user_blocks table
CREATE TABLE IF NOT EXISTS user_blocks (
    user_id BIGINT NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    blocked_user_id BIGINT NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    created_at TIMESTAMPTZ DEFAULT now() NOT NULL,
    PRIMARY KEY (user_id, blocked_user_id)
);

-- create user followers table
CREATE TABLE IF NOT EXISTS user_follow (
    user_id BIGINT NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    follower_id BIGINT NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    status VARCHAR(32) DEFAULT 'pending' NOT NULL,
    -- status can be pending, accepted, blocked, etc.
    -- following will have privacy settings, such as public, private, etc. and the status will be used to determine if the follower is allowed to follow the user or not.
    created_at TIMESTAMPTZ DEFAULT now() NOT NULL,
    PRIMARY KEY (user_id, follower_id)
);

-- create reports table
CREATE TABLE IF NOT EXISTS reports (
    id BIGINT NOT NULL PRIMARY KEY,
    reporter_id BIGINT NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    reported_target_id BIGINT NOT NULL,
    -- reported_target can be users, posts, comments, etc. and can be used to report those targets.
    reported_target_type VARCHAR(32) NOT NULL,
    report_type VARCHAR(64) NOT NULL,
    report_data JSONB NOT NULL,
    status VARCHAR(32) DEFAULT 'pending' NOT NULL,
    -- status can be pending, reviewed, resolved, etc.
    created_at TIMESTAMPTZ DEFAULT now() NOT NULL,
    updated_at TIMESTAMPTZ DEFAULT now() NOT NULL
);

-- create guilds table
CREATE TABLE IF NOT EXISTS guilds (
    id BIGINT NOT NULL PRIMARY KEY,
    name VARCHAR(128) NOT NULL,
    description TEXT NULL,
    owner_id BIGINT NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    total_members INT DEFAULT 0 NOT NULL,
    total_channels INT DEFAULT 0 NOT NULL,
    created_at TIMESTAMPTZ DEFAULT now() NOT NULL,
    updated_at TIMESTAMPTZ DEFAULT now() NOT NULL,
    deleted_at TIMESTAMPTZ NULL
);

-- create guild_members table
CREATE TABLE IF NOT EXISTS guild_members (
    guild_id BIGINT NOT NULL REFERENCES guilds(id) ON DELETE CASCADE,
    user_id BIGINT NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    joined_at TIMESTAMPTZ DEFAULT now() NOT NULL,
    deleted_at TIMESTAMPTZ NULL,
    PRIMARY KEY (guild_id, user_id)
);

-- create guild_roles table
CREATE TABLE IF NOT EXISTS guild_roles (
    id BIGINT NOT NULL PRIMARY KEY,
    guild_id BIGINT NOT NULL REFERENCES guilds(id) ON DELETE CASCADE,
    name VARCHAR(64) NOT NULL,
    color VARCHAR(16) NULL,
    position INT NOT NULL,
    permission_bitmask BIGINT[] NOT NULL,
    created_at TIMESTAMPTZ DEFAULT now() NOT NULL,
    updated_at TIMESTAMPTZ DEFAULT now() NOT NULL,
    deleted_at TIMESTAMPTZ NULL
);

-- create guild_member_roles table
CREATE TABLE IF NOT EXISTS guild_member_roles (
    guild_id BIGINT NOT NULL REFERENCES guilds(id) ON DELETE CASCADE,
    user_id BIGINT NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    role_id BIGINT NOT NULL REFERENCES guild_roles(id) ON DELETE CASCADE,
    assigned_at TIMESTAMPTZ DEFAULT now() NOT NULL,
    PRIMARY KEY (guild_id, user_id, role_id)
);

-- create guild_channels table
CREATE TABLE IF NOT EXISTS guild_channels (
    id BIGINT NOT NULL PRIMARY KEY,
    guild_id BIGINT NOT NULL REFERENCES guilds(id) ON DELETE CASCADE,
    name VARCHAR(128) NOT NULL,
    channel_type VARCHAR(32) NOT NULL,
    created_at TIMESTAMPTZ DEFAULT now() NOT NULL,
    updated_at TIMESTAMPTZ DEFAULT now() NOT NULL,
    deleted_at TIMESTAMPTZ NULL
);

-- create guild_channel_overrides table
CREATE TABLE IF NOT EXISTS guild_channel_overrides (
    channel_id BIGINT NOT NULL REFERENCES guild_channels(id) ON DELETE CASCADE,
    target_id BIGINT NOT NULL,
    -- target can be users, roles, etc. and can be used to override the permissions for those targets.
    target_type VARCHAR(32) NOT NULL,

    allow_mask BIGINT[] NOT NULL,
    deny_mask BIGINT[] NOT NULL,
    -- allow_mask and deny_mask will be used to override the permissions for the target, and will be used to determine if the target is allowed to perform certain actions or not.
    created_at TIMESTAMPTZ DEFAULT now() NOT NULL,
    updated_at TIMESTAMPTZ DEFAULT now() NOT NULL,
    PRIMARY KEY (channel_id, target_id, target_type)
);

-- create guild_invites table
CREATE TABLE IF NOT EXISTS guild_invites (
    code VARCHAR(16) NOT NULL PRIMARY KEY,
    guild_id BIGINT NOT NULL REFERENCES guilds(id) ON DELETE CASCADE,
    inviter_id BIGINT NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    max_uses INT DEFAULT 0 NOT NULL,
    uses INT DEFAULT 0 NOT NULL,
    expires_at TIMESTAMPTZ NULL,
    created_at TIMESTAMPTZ DEFAULT now() NOT NULL,
    updated_at TIMESTAMPTZ DEFAULT now() NOT NULL
);

-- create guild_bans table
CREATE TABLE IF NOT EXISTS guild_bans (
    guild_id BIGINT NOT NULL REFERENCES guilds(id) ON DELETE CASCADE,
    user_id BIGINT NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    reason VARCHAR(255) NULL,
    banned_at TIMESTAMPTZ DEFAULT now() NOT NULL,
    PRIMARY KEY (guild_id, user_id)
);

-- create guild_assets table
CREATE TABLE IF NOT EXISTS guild_assets (
    id BIGINT NOT NULL PRIMARY KEY,
    guild_id BIGINT NOT NULL REFERENCES guilds(id) ON DELETE CASCADE,
    
    -- Stickers, Emojis, etc.
    asset_id BIGINT NOT NULL REFERENCES media_data(id) ON DELETE CASCADE,
    asset_type VARCHAR(32) NOT NULL,
    asset_name VARCHAR(32) NOT NULL,
    asset_emoji VARCHAR(64) NULL,

    created_at TIMESTAMPTZ DEFAULT now() NOT NULL,
    updated_at TIMESTAMPTZ DEFAULT now() NOT NULL,
    deleted_at TIMESTAMPTZ NULL
);

-- create messages table
CREATE TABLE IF NOT EXISTS messages (
    id BIGINT NOT NULL PRIMARY KEY,
    user_id BIGINT NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    target_id BIGINT NOT NULL,
    -- target can be users, guilds, channels, etc. and can be used to send messages to those targets.
    target_type VARCHAR(32) NOT NULL,
    content VARCHAR(4096) NULL,
    -- depending on the target_type, the content can be different, for example, if the target_type is users, the content can be a direct message, if the target_type is guilds, the content can be a guild message, if the target_type is channels, the content can be a channel message, etc.
    has_attachment BOOLEAN DEFAULT false NOT NULL,
    has_reactions BOOLEAN DEFAULT false NOT NULL,

    created_at TIMESTAMPTZ DEFAULT now() NOT NULL,
    updated_at TIMESTAMPTZ DEFAULT now() NOT NULL,
    deleted_at TIMESTAMPTZ NULL
);

-- create message_reactions table
CREATE TABLE IF NOT EXISTS message_reactions (
    message_id BIGINT NOT NULL REFERENCES messages(id) ON DELETE CASCADE,
    user_id BIGINT NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    
    -- reaction must be an emoji
    -- can be from guild_assets table or default emojis.
    reaction VARCHAR(64) NOT NULL,

    reacted_at TIMESTAMPTZ DEFAULT now() NOT NULL,
    PRIMARY KEY (message_id, user_id, reaction)
);

-- create audit_logs table
CREATE TABLE IF NOT EXISTS audit_logs (
    id BIGINT NOT NULL PRIMARY KEY,
    target_id BIGINT NOT NULL,
    -- target can be users, posts, comments, etc. and can be used to log those targets.
    target_type VARCHAR(32) NOT NULL,
    action VARCHAR(64) NOT NULL,
    performed_by BIGINT NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    action_data JSONB NOT NULL,
    created_at TIMESTAMPTZ DEFAULT now() NOT NULL
);

-- INDEXES
CREATE INDEX IF NOT EXISTS idx_users_email ON public.users USING btree (email);
CREATE INDEX IF NOT EXISTS idx_users_username ON public.users USING btree (username);
CREATE UNIQUE INDEX IF NOT EXISTS uq_users_username_active ON public.users USING btree (username) WHERE (deleted_at IS NULL);
CREATE INDEX IF NOT EXISTS idx_sessions_user_id ON public.sessions USING btree (user_id);
CREATE INDEX IF NOT EXISTS idx_user_credentials_user_id ON public.user_credentials USING btree (user_id);
CREATE INDEX IF NOT EXISTS idx_user_profiles_user_id ON public.user_profiles USING btree (user_id);