-- Add down migration script here
-- DOWN migration

DROP TABLE IF EXISTS media_likes;

CREATE TABLE IF NOT EXISTS media_likes (
    user_id BIGINT NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    media_post_id BIGINT NOT NULL REFERENCES media_posts(id) ON DELETE CASCADE,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    is_liked BOOLEAN,

    PRIMARY KEY (user_id, media_post_id)
);