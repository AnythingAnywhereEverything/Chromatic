-- Add up migration script here

ALTER TABLE media_data
ADD COLUMN flags BIGINT NOT NULL DEFAULT 0,
ADD COLUMN original_name TEXT NOT NULL DEFAULT '',
ADD COLUMN original_content_type TEXT NOT NULL DEFAULT '';

-- if name contain _a then we assume it animated.
UPDATE media_data
SET flags = flags | 1
WHERE name LIKE '%_a%';

-- if path ended with / and mime is video then we assume it is a HLS stream.
-- or contain t_ indicate that it is a thumbnail of a video.
-- Set HLS and thumbnail flags for video media
UPDATE media_data
SET flags = flags | 6
FROM media_metadata mm
WHERE media_data.id = mm.media_id
AND (media_data.path LIKE '%/' OR media_data.path LIKE '%t_%')
AND mm.mime_type LIKE 'video/%';