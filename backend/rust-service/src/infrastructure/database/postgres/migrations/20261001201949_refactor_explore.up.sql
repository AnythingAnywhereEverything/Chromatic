-- Add up migration script here
-- change how tags works

-- lowercase all tag names
UPDATE interest_tags
SET tag_name = LOWER(tag_name);
ALTER TABLE interest_tags
    ADD COLUMN popularity INT DEFAULT 0;


-- count existing popularity for each tag
UPDATE interest_tags
SET popularity = (
    SELECT COUNT(*)
    FROM tag_attachments
    WHERE tag_attachments.tag_id = interest_tags.id
    AND target_type = 'post'::tag_attachment_types
);

UPDATE media_posts mp
SET content = CASE
    WHEN mp.content IS NULL OR mp.content = '' THEN tags.hashtags
    ELSE mp.content || ' ' || tags.hashtags
END
FROM (
    SELECT
        ta.target_id AS post_id,
        STRING_AGG('#' || it.tag_name, ' ') AS hashtags
    FROM tag_attachments ta
    JOIN interest_tags it
        ON ta.tag_id = it.id
    WHERE ta.target_type = 'post'::tag_attachment_types
    GROUP BY ta.target_id
) tags
WHERE mp.id = tags.post_id;