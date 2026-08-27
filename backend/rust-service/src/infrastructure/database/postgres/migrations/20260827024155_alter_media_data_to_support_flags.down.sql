-- Add down migration script here
ALTER TABLE media_data
DROP COLUMN flags,
DROP COLUMN original_name,
DROP COLUMN original_content_type;