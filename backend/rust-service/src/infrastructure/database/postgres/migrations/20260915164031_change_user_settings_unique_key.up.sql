-- Add up migration script here
DROP TABLE IF EXISTS user_settings;

CREATE TABLE user_settings (
    user_id BIGINT NOT NULL REFERENCES users(id) ON DELETE CASCADE,

    setting_key VARCHAR(64) NOT NULL,

    setting_value JSONB NOT NULL,

    created_at TIMESTAMPTZ DEFAULT now() NOT NULL,

    updated_at TIMESTAMPTZ DEFAULT now() NOT NULL,

    PRIMARY KEY (user_id, setting_key)
);