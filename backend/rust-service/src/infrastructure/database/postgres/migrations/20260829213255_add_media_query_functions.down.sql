-- Add down migration script here
DROP FUNCTION IF EXISTS get_media_by_id(BIGINT);
DROP FUNCTION IF EXISTS get_media_by_id_without_playlists(BIGINT);
DROP FUNCTION IF EXISTS get_post_by_id(BIGINT, BIGINT);
DROP FUNCTION IF EXISTS get_post_amount_for_feed(BIGINT, INT);