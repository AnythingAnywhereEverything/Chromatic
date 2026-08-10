-- * Rename columns
ALTER TABLE media_data RENAME COLUMN user_id TO uploader_id;
ALTER TABLE media_data RENAME COLUMN media_url TO path;
ALTER TABLE media_data RENAME COLUMN media_status TO status;

ALTER TABLE media_data DROP COLUMN media_preview_url;
ALTER TABLE media_data DROP COLUMN media_category;

-- * Remove enum type of media_category
DO $$
BEGIN
    IF EXISTS (SELECT 1 FROM pg_type WHERE typname = 'media_category') THEN
        DROP TYPE media_category;
    END IF;
END $$;

ALTER TABLE media_data ADD COLUMN thumbhash TEXT;
ALTER TABLE media_data ADD COLUMN name TEXT NOT NULL;
ALTER TABLE media_data ADD COLUMN lock_hash TEXT;
ALTER TABLE media_data
    ADD COLUMN lock_expiration TIMESTAMPTZ DEFAULT NOW() + INTERVAL '1 hour';
ALTER TABLE media_data
    ADD COLUMN updated_at TIMESTAMPTZ DEFAULT NOW() NOT NULL;

-- * Change media_status enum
DO $$
BEGIN
    IF EXISTS (SELECT 1 FROM pg_type WHERE typname = 'media_status') THEN
        ALTER TABLE media_data
            ALTER COLUMN status DROP DEFAULT;

        ALTER TYPE media_status RENAME TO media_status_old;

        CREATE TYPE media_status AS ENUM (
            'pending',
            'processing',
            'completed',
            'ready',
            'failed',
            'locked'
        );

        ALTER TABLE media_data
            ALTER COLUMN status TYPE media_status
            USING status::text::media_status;

        -- * Restore the default using the new enum type.
        ALTER TABLE media_data
            ALTER COLUMN status SET DEFAULT 'pending'::media_status;

        DROP TYPE media_status_old;
    END IF;
END $$;