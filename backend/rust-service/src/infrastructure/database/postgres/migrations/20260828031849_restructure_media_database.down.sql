-- Add down migration script here
-- * revert the old table back first

CREATE TYPE media_status AS ENUM (
	'pending',
	'processing',
	'completed',
	'ready',
	'failed',
	'locked');

CREATE TABLE media_data (
	id int8 NOT NULL,
	uploader_id int8 NOT NULL,
	"path" text NOT NULL,
	created_at timestamptz DEFAULT now() NOT NULL,
	deleted_at timestamptz NULL,
	status media_status DEFAULT 'pending'::media_status NOT NULL,
	thumbhash text NULL,
	"name" text NOT NULL,
	lock_hash text NULL,
	lock_expiration timestamptz DEFAULT now() + '01:00:00'::interval NULL,
	updated_at timestamptz DEFAULT now() NOT NULL,
	flags int8 DEFAULT 0 NOT NULL,
	original_name text DEFAULT ''::text NOT NULL,
	original_content_type text DEFAULT ''::text NOT NULL,
	CONSTRAINT media_data_created_at_not_null NOT NULL created_at,
	CONSTRAINT media_data_flags_not_null NOT NULL flags,
	CONSTRAINT media_data_id_not_null NOT NULL id,
	CONSTRAINT media_data_media_status_not_null NOT NULL status,
	CONSTRAINT media_data_media_url_not_null NOT NULL path,
	CONSTRAINT media_data_name_not_null NOT NULL name,
	CONSTRAINT media_data_original_content_type_not_null NOT NULL original_content_type,
	CONSTRAINT media_data_original_name_not_null NOT NULL original_name,
	CONSTRAINT media_data_pkey PRIMARY KEY (id),
	CONSTRAINT media_data_updated_at_not_null NOT NULL updated_at,
	CONSTRAINT media_data_user_id_not_null NOT NULL uploader_id
);
CREATE INDEX idx_media_created_at ON media_data USING btree (created_at);
CREATE INDEX idx_media_status ON media_data USING btree (status);

CREATE TABLE media_metadata (
	media_id int8 NOT NULL,
	file_size int8 NOT NULL,
	mime_type varchar(64) NOT NULL,
	width int4 NULL,
	height int4 NULL,
	duration int4 NULL,
	CONSTRAINT media_metadata_file_size_not_null NOT NULL file_size,
	CONSTRAINT media_metadata_media_id_not_null NOT NULL media_id,
	CONSTRAINT media_metadata_mime_type_not_null NOT NULL mime_type,
	CONSTRAINT media_metadata_pkey PRIMARY KEY (media_id)
);

ALTER TABLE media_data ADD CONSTRAINT media_data_user_id_fkey FOREIGN KEY (uploader_id) REFERENCES users(id) ON DELETE CASCADE;
ALTER TABLE media_metadata ADD CONSTRAINT media_metadata_media_id_fkey FOREIGN KEY (media_id) REFERENCES media_data(id) ON DELETE CASCADE;


-- * remove all rows from tables that have foreign key references to media_data to avoid constraint issues
DELETE FROM media_attachments;
DELETE FROM guild_assets;
DELETE FROM user_recent_avatar;
DELETE FROM user_recent_banner;

ALTER TABLE media_attachments
DROP COLUMN media_id,
ADD COLUMN media_id BIGINT NOT NULL REFERENCES media_data(id) ON DELETE CASCADE;

ALTER TABLE guild_assets
DROP COLUMN asset_id,
ADD COLUMN asset_id BIGINT NOT NULL REFERENCES media_data(id) ON DELETE CASCADE;

ALTER TABLE user_profiles
DROP COLUMN avatar_media_id,
DROP COLUMN banner_media_id,
ADD COLUMN avatar_media_id BIGINT REFERENCES media_data(id) ON DELETE SET NULL,
ADD COLUMN banner_media_id BIGINT REFERENCES media_data(id) ON DELETE SET NULL;

ALTER TABLE user_recent_avatar
DROP COLUMN avatar_id,
ADD COLUMN avatar_id BIGINT NOT NULL REFERENCES media_data(id) ON DELETE CASCADE PRIMARY KEY;

ALTER TABLE user_recent_banner
DROP COLUMN banner_id,
ADD COLUMN banner_id BIGINT NOT NULL REFERENCES media_data(id) ON DELETE CASCADE PRIMARY KEY;

-- set primary and constrant for affected tables

-- add primary key as target_id with media_id
ALTER TABLE media_attachments
ADD CONSTRAINT media_attachments_pkey PRIMARY KEY (target_id, media_id);


-- * Remove child tables before their parent tables.

DROP TABLE IF EXISTS media_hls_playlists;
DROP TABLE IF EXISTS media_hls;
DROP TABLE IF EXISTS media_object_metadata;
DROP TABLE IF EXISTS media_objects;
DROP TABLE IF EXISTS media;

-- * Remove enums after all dependent tables/functions are gone.

DROP TYPE IF EXISTS media_post_processing_state;
DROP TYPE IF EXISTS media_processing_state;
DROP TYPE IF EXISTS media_kind;
DROP TYPE IF EXISTS media_type;