DROP INDEX IF EXISTS tag_parents_child_id_idx;
DROP INDEX IF EXISTS tag_parents_parent_id_idx;
DROP TABLE IF EXISTS tag_parents;

DROP INDEX IF EXISTS tags_category_id_idx;
DROP INDEX IF EXISTS tags_restricted_idx;
DROP TABLE IF EXISTS tags;

DROP TRIGGER IF EXISTS no_delete_permanent ON tag_categories;
DROP TABLE IF EXISTS tag_categories;
DROP FUNCTION IF EXISTS prevent_permanent_category_delete;

DROP INDEX IF EXISTS tag_names_lower_idx;
DROP TABLE IF EXISTS tag_names;
