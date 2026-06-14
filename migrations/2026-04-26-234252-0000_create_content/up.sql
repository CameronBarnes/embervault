-- Content
CREATE TABLE content (
	id SERIAL PRIMARY KEY,

	-- primary perceptual hashes, for images or prepresentative frame for gif or video
	dhash BIGINT NOT NULL CHECK (dhash >= 0),
	phash BIGINT NOT NULL CHECK (phash >= 0),

	-- user metadata
	title TEXT NOT NULL DEFAULT '',
	description TEXT NOT NULL DEFAULT '',
	favorite BOOLEAN NOT NULL DEFAULT FALSE,
	private BOOLEAN NOT NULL DEFAULT FALSE,

	-- Used to represent if any of the tags on this content is marked as restricted
	restricted_tag_count INT NOT NULL DEFAULT 0 CHECK (restricted_tag_count >= 0),
	restricted BOOLEAN NOT NULL GENERATED ALWAYS AS (restricted_tag_count > 0) STORED,

	duration_viewed_ms BIGINT NOT NULL DEFAULT 0 CHECK (duration_viewed_ms >= 0),
	views INT NOT NULL DEFAULT 0 CHECK (views >= 0),
	points INT NOT NULL DEFAULT 0,
	score INT NOT NULL DEFAULT 0,
	has_manually_added_tags BOOLEAN NOT NULL DEFAULT FALSE,

	-- backend metadata
	file_path TEXT NOT NULL,
	media_type TEXT NOT NULL,
	width INT CHECK (width > 0),
	height INT CHECK (height > 0),
	duration_ms INT CHECK (duration_ms >= 0),

	created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
	updated_at TIMESTAMPTZ NOT NULL DEFAULT now(),

	UNIQUE (file_path)
);

CREATE INDEX content_dhash_idx ON content (dhash);
CREATE INDEX content_phash_idx ON content (phash);
CREATE INDEX content_restricted_idx ON content (restricted);
CREATE INDEX content_favorite_idx ON content (favorite);
CREATE INDEX content_duration_viewed_idx ON content (duration_viewed_ms);
CREATE INDEX content_views_idx ON content (views);
CREATE INDEX content_points_idx ON content (points);
CREATE INDEX content_score_idx ON content (score);

-- Content file hashes
CREATE TABLE content_file_hash (
	id SERIAL PRIMARY KEY,
	content_id INT NOT NULL REFERENCES content(id) ON DELETE CASCADE,

	hash_type TEXT NOT NULL, -- ie blake3 or sha256
	hash_value BYTEA NOT NULL
);

CREATE UNIQUE INDEX content_file_hash_unique ON content_file_hash (hash_type, hash_value);

-- Content Frame hashes
CREATE TABLE content_frame_hash (
	id SERIAL PRIMARY KEY,
	content_id INT NOT NULL REFERENCES content(id) ON DELETE CASCADE,

	frame_index INT NOT NULL,

	dhash BIGINT NOT NULL CHECK (dhash >= 0),
	phash BIGINT NOT NULL CHECK (phash >= 0),

	UNIQUE (content_id, frame_index)
);

CREATE INDEX frame_dhash_idx ON content_frame_hash (dhash);
CREATE INDEX frame_phash_idx ON content_frame_hash (phash);
CREATE INDEX frame_content_dhash_idx ON content_frame_hash (content_id, dhash);
CREATE INDEX frame_content_phash_idx ON content_frame_hash (content_id, phash);

-- Content tags
CREATE TABLE content_tag (
	content_id INT NOT NULL REFERENCES content(id) ON DELETE CASCADE,
	tag_id INT NOT NULL REFERENCES tags(id) ON DELETE CASCADE,

	PRIMARY KEY (content_id, tag_id)
);

CREATE INDEX content_tag_tag_idx ON content_tag (tag_id);
CREATE INDEX content_tag_content_idx ON content_tag (content_id);

-- Trigger for restricted value when a tag is added to content
CREATE OR REPLACE FUNCTION content_tag_insert_restricted()
RETURNS TRIGGER AS $$
BEGIN
  IF (SELECT restricted FROM tags WHERE id = NEW.tag_id) THEN
    UPDATE content
    SET restricted_tag_count = restricted_tag_count + 1
    WHERE id = NEW.content_id;
  END IF;

  RETURN NULL;
END;
$$ LANGUAGE plpgsql;

-- Trigger for when a tag is removed from content
CREATE OR REPLACE FUNCTION content_tag_delete_restricted()
RETURNS TRIGGER AS $$
BEGIN
  IF (SELECT restricted FROM tags WHERE id = OLD.tag_id) THEN
    UPDATE content
    SET restricted_tag_count = restricted_tag_count - 1
    WHERE id = OLD.content_id;
  END IF;

  RETURN NULL;
END;
$$ LANGUAGE plpgsql;

-- Trigger for when a tag's restricted value changes
CREATE OR REPLACE FUNCTION tag_restricted_changed()
RETURNS TRIGGER AS $$
BEGIN
  IF NEW.restricted = OLD.restricted THEN
    RETURN NULL;
  END IF;

  IF NEW.restricted THEN
    UPDATE content c
    SET restricted_tag_count = restricted_tag_count + 1
    WHERE c.id IN (
      SELECT content_id FROM content_tag WHERE tag_id = NEW.id
    );
  ELSE
    UPDATE content c
    SET restricted_tag_count = restricted_tag_count - 1
    WHERE c.id IN (
      SELECT content_id FROM content_tag WHERE tag_id = NEW.id
    );
  END IF;

  RETURN NULL;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER content_tag_insert_trigger
AFTER INSERT ON content_tag
FOR EACH ROW
EXECUTE FUNCTION content_tag_insert_restricted();

CREATE TRIGGER content_tag_delete_trigger
AFTER DELETE ON content_tag
FOR EACH ROW
EXECUTE FUNCTION content_tag_delete_restricted();

CREATE TRIGGER tag_restricted_update_trigger
AFTER UPDATE OF restricted ON tags
FOR EACH ROW
EXECUTE FUNCTION tag_restricted_changed();
