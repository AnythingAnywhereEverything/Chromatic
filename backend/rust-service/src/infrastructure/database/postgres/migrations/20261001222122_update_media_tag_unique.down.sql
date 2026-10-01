-- Add down migration script here

ALTER TABLE interest_tags
    DROP CONSTRAINT unique_interest_tag;

ALTER TABLE tag_attachments
    DROP CONSTRAINT unique_tag_attachment;