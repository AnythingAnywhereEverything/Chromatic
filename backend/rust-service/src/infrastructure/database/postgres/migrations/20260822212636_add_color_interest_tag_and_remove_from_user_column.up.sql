-- Add up migration script here
CREATE TYPE tag_attachment_types AS ENUM ('user', 'post', 'guild');

ALTER TABLE users
    DROP COLUMN IF EXISTS interest_tags;

ALTER TABLE tag_attachments
    DROP COLUMN IF EXISTS target_type;

ALTER TABLE tag_attachments
    ADD COLUMN target_type tag_attachment_types DEFAULT 'user';

ALTER TABLE interest_tags
    ADD COLUMN tag_color VARCHAR(32) NOT NULL DEFAULT '#ffffff';

