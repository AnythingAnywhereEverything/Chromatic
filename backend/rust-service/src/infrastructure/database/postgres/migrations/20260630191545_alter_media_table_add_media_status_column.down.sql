-- Add down migration script here

ALTER TABLE media_data
    DROP COLUMN IF EXISTS media_status;

DROP INDEX IF EXISTS idx_media_status;
DROP INDEX IF EXISTS idx_media_created_at;

-- revert media_category enum type to original values
DO $$
BEGIN
    IF EXISTS (SELECT 1 FROM pg_type WHERE typname = 'media_category') THEN
        ALTER TYPE media_category RENAME TO media_category_old;
        CREATE TYPE media_category AS ENUM ('image', 'video', 'audio', 'document', 'code', 'archive');
        ALTER TABLE media_data
            ALTER COLUMN media_category TYPE media_category USING media_category::text::media_category;
        DROP TYPE media_category_old;
    END IF;
END $$;

-- remove enum type
DROP TYPE IF EXISTS media_status;
DROP TYPE IF EXISTS media_category;