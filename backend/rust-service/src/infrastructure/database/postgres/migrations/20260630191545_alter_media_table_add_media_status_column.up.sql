-- if media was uploaded but never attached or used, we force delete it after 1 day;

CREATE TYPE media_status AS ENUM ('pending', 'completed', 'failed');
CREATE TYPE media_category AS ENUM (
    'image',
    'video',
    'audio',
    'document',
    'code',
    'archive',
    'unknown'
);

ALTER TABLE media_data
    ADD COLUMN media_status media_status NOT NULL DEFAULT 'pending';

ALTER TABLE media_data
    ALTER COLUMN media_category
    TYPE media_category
    USING media_category::media_category;

CREATE INDEX idx_media_status ON media_data (media_status);
CREATE INDEX idx_media_created_at ON media_data (created_at);