-- Add down migration script here

-- Remove the popularity column from interest_tags
ALTER TABLE interest_tags
    DROP COLUMN popularity;

UPDATE media_posts mp
SET content = NULLIF(
    BTRIM(
        REGEXP_REPLACE(
            mp.content,
            '(^|[[:space:]])(' || tags.hashtags_regex || ')(?=$|[[:space:]])',
            '',
            'g'
        )
    ),
    ''
)
FROM (
    SELECT
        ta.target_id AS post_id,
        STRING_AGG(
            REGEXP_REPLACE('#' || it.tag_name, '([\\.^$|()\\[\\]{}*+?])', '\\\1', 'g'),
            '|'
        ) AS hashtags_regex
    FROM tag_attachments ta
    JOIN interest_tags it
        ON ta.tag_id = it.id
    WHERE ta.target_type = 'post'::tag_attachment_types
    GROUP BY ta.target_id
) tags
WHERE mp.id = tags.post_id
  AND mp.content IS NOT NULL
  AND mp.content ~ ('(^|[[:space:]])(' || tags.hashtags_regex || ')(?=$|[[:space:]])');