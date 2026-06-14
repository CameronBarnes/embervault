-- Drop triggers first
DROP TRIGGER IF EXISTS content_tag_insert_trigger ON content_tag;
DROP TRIGGER IF EXISTS content_tag_delete_trigger ON content_tag;
DROP TRIGGER IF EXISTS tag_restricted_update_trigger ON tags;

-- Drop trigger functions
DROP FUNCTION IF EXISTS content_tag_insert_restricted();
DROP FUNCTION IF EXISTS content_tag_delete_restricted();
DROP FUNCTION IF EXISTS tag_restricted_changed();

-- Drop indexes (optional but explicit)
DROP INDEX IF EXISTS content_tag_tag_idx;
DROP INDEX IF EXISTS content_tag_content_idx;

DROP INDEX IF EXISTS frame_content_phash_idx;
DROP INDEX IF EXISTS frame_content_dhash_idx;
DROP INDEX IF EXISTS frame_phash_idx;
DROP INDEX IF EXISTS frame_dhash_idx;

DROP INDEX IF EXISTS content_file_hash_unique;

DROP INDEX IF EXISTS content_score_idx;
DROP INDEX IF EXISTS content_points_idx;
DROP INDEX IF EXISTS content_views_idx;
DROP INDEX IF EXISTS content_duration_viewed_idx;
DROP INDEX IF EXISTS content_favorite_idx;
DROP INDEX IF EXISTS content_restricted_idx;
DROP INDEX IF EXISTS content_phash_idx;
DROP INDEX IF EXISTS content_dhash_idx;

-- Drop tables in dependency order
DROP TABLE IF EXISTS content_tag;
DROP TABLE IF EXISTS content_frame_hash;
DROP TABLE IF EXISTS content_file_hash;
DROP TABLE IF EXISTS content;
