-- Your SQL goes here
-- Tag aliases
CREATE TABLE tag_aliases (
	id SERIAL PRIMARY KEY,
	name_id INTEGER NOT NULL UNIQUE
		REFERENCES tag_names(id) ON DELETE CASCADE,
	tag_id INTEGER NOT NULL REFERENCES tags(id) ON DELETE CASCADE
);

CREATE INDEX tag_aliases_tag_id_idx ON tag_aliases (tag_id);

-- Tag groups
CREATE TABLE tag_groups (
	id SERIAL PRIMARY KEY,
	name_id INTEGER NOT NULL UNIQUE
		REFERENCES tag_names(id) ON DELETE CASCADE,
	descripton TEXT
);

CREATE TABLE tag_group_members (
	group_id INTEGER NOT NULL REFERENCES tag_groups(id) ON DELETE CASCADE,
	tag_id INTEGER NOT NULL REFERENCES tags(id) ON DELETE CASCADE,
	PRIMARY KEY (group_id, tag_id)
);

CREATE INDEX tag_group_members_tag_id_idx ON tag_group_members (tag_id);

-- Tag group aliases
CREATE TABLE tag_group_aliases (
	id SERIAL PRIMARY KEY,
	name_id INTEGER NOT NULL UNIQUE
		REFERENCES tag_names(id) ON DELETE CASCADE,
	group_id INTEGER NOT NULL REFERENCES tag_groups(id) ON DELETE CASCADE
);

CREATE INDEX tag_group_aliases_group_id_idx ON tag_group_aliases (group_id);
