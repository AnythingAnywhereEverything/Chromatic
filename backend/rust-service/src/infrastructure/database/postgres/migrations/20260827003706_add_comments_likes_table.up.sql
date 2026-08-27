-- Add up migration script here

DROP TABLE IF EXISTS media_likes;

CREATE TABLE IF NOT EXISTS media_likes (
    target_id BIGINT NOT NULL,
    user_id BIGINT NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    target_type VARCHAR(32) NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    is_like BOOLEAN,
    PRIMARY KEY (target_id, user_id, target_type)
);