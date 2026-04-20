-- We need to make sure names are unique across all tags, groups, and aliases
CREATE TABLE tag_names (
	id SERIAL PRIMARY KEY,
	name TEXT NOT NULL
);

CREATE UNIQUE INDEX tag_names_lower_idx
ON tag_names (LOWER(name));

-- Tag categories here
CREATE TABLE tag_categories (
	id SERIAL PRIMARY KEY,
	name TEXT NOT NULL UNIQUE,
	description TEXT,
	permanent BOOLEAN NOT NULL DEFAULT FALSE
);

CREATE FUNCTION prevent_permanent_category_delete()
RETURNS trigger AS $$
BEGIN
  IF OLD.permanent THEN
    RAISE EXCEPTION 'Cannot delete permanent category: %', OLD.name;
  END IF;
  RETURN OLD;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER no_delete_permanent
BEFORE DELETE ON tag_categories
FOR EACH ROW
EXECUTE FUNCTION prevent_permanent_category_delete();

-- Always want to make sure the default case exists, gets id = 1
INSERT INTO tag_categories (name, permanent) VALUES ('Content', TRUE);

-- Cover the other expected cases
INSERT INTO tag_categories (name, description) VALUES
	('Author', 'The creator, author, or artist responsible for a work'),
	('Character', 'An individual character or person in the media');


-- Tag table here
CREATE TABLE tags (
	id SERIAL PRIMARY KEY,
	name_id INTEGER NOT NULL UNIQUE
		REFERENCES tag_names(id) ON DELETE CASCADE,
	restricted BOOLEAN NOT NULL DEFAULT FALSE,
	category_id INTEGER NOT NULL DEFAULT 1,
	FOREIGN KEY (category_id)
		REFERENCES tag_categories(id)
		ON DELETE SET DEFAULT
);

CREATE INDEX tags_category_id_idx ON tags (category_id);

CREATE INDEX tags_restricted_idx
ON tags (id)
WHERE restricted = TRUE;

CREATE TABLE tag_parents (
	child_id INTEGER NOT NULL REFERENCES tags(id) ON DELETE CASCADE,
	parent_id INTEGER NOT NULL REFERENCES tags(id) ON DELETE CASCADE,
	PRIMARY KEY (child_id, parent_id),
	CHECK (child_id <> parent_id)
);

CREATE INDEX tag_parents_parent_id_idx ON tag_parents (parent_id);
CREATE INDEX tag_parents_child_id_idx ON tag_parents (child_id);
