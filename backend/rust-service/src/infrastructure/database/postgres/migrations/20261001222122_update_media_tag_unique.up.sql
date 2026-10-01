-- Add up migration script here

ALTER TABLE interest_tags
    ADD CONSTRAINT unique_interest_tag UNIQUE (tag_name);

ALTER TABLE tag_attachments
    ADD CONSTRAINT unique_tag_attachment UNIQUE (target_id, target_type, tag_id);