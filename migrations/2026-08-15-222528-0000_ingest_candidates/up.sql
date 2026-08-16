-- Ingest Candidates
CREATE TABLE ingest_candidates (
	id SERIAL PRIMARY KEY,
	import_entry BIGINT NOT NULL UNIQUE REFERENCES import_entries(id) ON DELETE CASCADE,

	-- primary perceptual hashes, for images or representative frame for gif or video
	dhash BIGINT NOT NULL,
	phash BIGINT NOT NULL
);

CREATE TABLE ingest_candidates_matches (
	candidate_id INT NOT NULL REFERENCES ingest_candidates(id) ON DELETE CASCADE,
	content_id INT NOT NULL REFERENCES content(id) ON DELETE CASCADE,

	dhash_distance INT NOT NULL,
	phash_distance INT NOT NULL,

	PRIMARY KEY (candidate_id, content_id)
);

-- Alter content table
ALTER TABLE content ADD COLUMN reviewed_at TIMESTAMPTZ;
