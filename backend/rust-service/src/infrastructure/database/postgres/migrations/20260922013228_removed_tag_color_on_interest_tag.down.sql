-- Add down migration script here
DELETE FROM interest_tags;

ALTER TABLE interest_tags
ADD COLUMN tag_color VARCHAR(255);