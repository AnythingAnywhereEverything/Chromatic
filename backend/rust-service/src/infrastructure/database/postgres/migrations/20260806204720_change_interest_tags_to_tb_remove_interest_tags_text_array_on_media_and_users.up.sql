-- Add up migration script here
CREATE TABLE IF NOT EXISTS interest_tags (
    id BIGINT PRIMARY KEY,
    tags_name VARCHAR(64) NOT NULL
);

CREATE TABLE IF NOT EXISTS user_interest(
    users_id BIGINT NOT NULL,
    interest_id BIGINT NOT NULL,
    FOREIGN KEY (users_id) REFERENCES users(id) ON DELETE CASCADE,
    FOREIGN KEY (interest_id) REFERENCES interest_tags(id) ON DELETE CASCADE,
    PRIMARY KEY (users_id, interest_id)
);

CREATE TABLE IF NOT EXISTS media_tags(
    media_id BIGINT NOT NULL,
    interest_id BIGINT NOT NULL,
    FOREIGN KEY (media_id) REFERENCES media_posts(id) ON DELETE CASCADE,
    FOREIGN KEY (interest_id) REFERENCES interest_tags(id) ON DELETE CASCADE
);

CREATE INDEX idx_interest_tags_id ON interest_tags(id);