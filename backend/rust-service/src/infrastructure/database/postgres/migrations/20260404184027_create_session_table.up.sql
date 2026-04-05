-- Add up migration script here

CREATE TABLE sessions (
    id BIGINT PRIMARY KEY,
    user_id BIGINT NOT NULL,
    session_hashed VARCHAR(255) NOT NULL UNIQUE,
    user_agent TEXT,
    ip_address VARCHAR(45),
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE
);

CREATE INDEX idx_sessions_user_id ON sessions(user_id);