-- Add down migration script here
-- * Removing the color column from interest_tags
ALTER TABLE interest_tags
    DROP COLUMN IF EXISTS tag_color;

-- * Reverting target_type to the original VARCHAR format
ALTER TABLE tag_attachments
    DROP COLUMN IF EXISTS target_type;

ALTER TABLE tag_attachments
    ADD COLUMN target_type VARCHAR(32) NOT NULL;

-- * Restoring interest_tags to users table
-- ! Original type wasn't specified in your snippet, using TEXT[] as a placeholder
ALTER TABLE users
    ADD COLUMN interest_tags TEXT[];

-- * Dropping the custom ENUM type
DROP TYPE IF EXISTS tag_attachment_types;
