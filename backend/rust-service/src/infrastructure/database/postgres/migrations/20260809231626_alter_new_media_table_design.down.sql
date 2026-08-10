-- Add down migration script here

-- * Revert column renames
ALTER TABLE media_data RENAME COLUMN uploader_id TO user_id;
ALTER TABLE media_data RENAME COLUMN path TO media_url;
ALTER TABLE media_data RENAME COLUMN status TO media_status;

-- * Restore removed columns
ALTER TABLE media_data ADD COLUMN media_preview_url TEXT;

-- * Restore media_category enum
DO $$
BEGIN
    IF NOT EXISTS (SELECT 1 FROM pg_type WHERE typname = 'media_category') THEN
        CREATE TYPE media_category AS ENUM (
            'image',
            'video',
            'audio',
            'document',
            'code',
            'archive',
            'unknown'
        );
    END IF;
END $$;

ALTER TABLE media_data
    ADD COLUMN media_category media_category;

-- * Remove columns added by up migration
ALTER TABLE media_data DROP COLUMN thumbhash;
ALTER TABLE media_data DROP COLUMN name;
ALTER TABLE media_data DROP COLUMN lock_hash;
ALTER TABLE media_data DROP COLUMN lock_expiration;
ALTER TABLE media_data DROP COLUMN updated_at;

-- * Revert media_status enum
DO $$
BEGIN
    IF EXISTS (SELECT 1 FROM pg_type WHERE typname = 'media_status') THEN

        -- * Remove the default because it belongs to the current enum type.
        ALTER TABLE media_data
            ALTER COLUMN media_status DROP DEFAULT;

        -- * Map statuses that do not exist in the old enum.
        UPDATE media_data
        SET media_status = CASE media_status::text
            WHEN 'ready' THEN 'completed'
            WHEN 'processing' THEN 'pending'
            WHEN 'locked' THEN 'pending'
            ELSE media_status::text
        END::media_status;

        ALTER TYPE media_status RENAME TO media_status_old;

        CREATE TYPE media_status AS ENUM (
            'pending',
            'completed',
            'failed'
        );

        ALTER TABLE media_data
            ALTER COLUMN media_status TYPE media_status
            USING media_status::text::media_status;

        -- * Restore the old default using the new enum type.
        ALTER TABLE media_data
            ALTER COLUMN media_status SET DEFAULT 'pending'::media_status;

        DROP TYPE media_status_old;
    END IF;
END $$;
