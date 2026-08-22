-- Add up migration script here

-- Showflakes
CREATE TABLE user_recent_avatar (
    avatar_id BIGINT NOT NULL REFERENCES media_data(id) PRIMARY KEY,
    user_id BIGINT NOT NULL REFERENCES users(id),
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE user_recent_banner (
    banner_id BIGINT NOT NULL REFERENCES media_data(id) PRIMARY KEY,
    user_id BIGINT NOT NULL REFERENCES users(id),
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

-- add indexes for performance
CREATE INDEX idx_user_recent_avatar_user_id ON user_recent_avatar (user_id);
CREATE INDEX idx_user_recent_banner_user_id ON user_recent_banner (user_id);