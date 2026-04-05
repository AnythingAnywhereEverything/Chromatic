-- Add up migration script here

-- Create user, credentials, and profile tables
CREATE TABLE users (
    id BIGINT PRIMARY KEY,
    email VARCHAR(255) NOT NULL UNIQUE,
    username VARCHAR(255) UNIQUE,
    phone_number VARCHAR(20),
    email_verified_at TIMESTAMPTZ, -- nullable, set when email is verified
    is_active BOOLEAN NOT NULL DEFAULT TRUE,
    deleted_at TIMESTAMPTZ,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TABLE user_credentials (
    user_id BIGINT NOT NULL PRIMARY KEY,
    password_hash VARCHAR(255), -- nullable for users created via OAuth
    password_updated_at TIMESTAMPTZ, -- nullable for users created via OAuth
    failed_login_count INT NOT NULL DEFAULT 0,
    lockout_until TIMESTAMPTZ, -- nullable, set when account is locked
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE
);

CREATE TABLE user_profiles (
    user_id BIGINT NOT NULL PRIMARY KEY,
    display_name VARCHAR(255),
    bio TEXT,
    avatar_url VARCHAR(255),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE
);

-- Add indexes for performance
CREATE INDEX idx_users_email ON users(email);
CREATE INDEX idx_users_username ON users(username);
CREATE INDEX idx_user_credentials_user_id ON user_credentials(user_id);
CREATE INDEX idx_user_profiles_user_id ON user_profiles(user_id);