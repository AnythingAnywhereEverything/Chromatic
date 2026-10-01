-- Add up migration script here
CREATE TABLE IF NOT EXISTS user_staff_roles (
    user_id BIGINT NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    role_id BIGINT NOT NULL REFERENCES staff_roles(id) ON DELETE CASCADE,
    assigned_by BIGINT NOT NULL,
    assigned_at TIMESTAMPTZ DEFAULT now() NOT NULL,
    PRIMARY KEY (user_id, role_id)
);

CREATE INDEX IF NOT EXISTS idx_user_staff_roles_role_id ON user_staff_roles (role_id);
