-- Import sources
CREATE TABLE import_sources (
	id SERIAL PRIMARY KEY,
	path_format path_format NOT NULL,
	path BYTEA NOT NULL,

	UNIQUE (path_format, path)
);

CREATE TYPE ingest_mode AS ENUM ('in_place', 'managed');

CREATE TABLE automatic_imports (
	id INT PRIMARY KEY REFERENCES import_sources(id) ON DELETE CASCADE,
	ingest_mode ingest_mode NOT NULL
);

CREATE TYPE import_scan_status AS ENUM (
    'running',
    'completed',
    'failed',
    'cancelled'
);

CREATE TABLE import_scans (
	id BIGSERIAL PRIMARY KEY,
	source_id INT NOT NULL REFERENCES import_sources(id) ON DELETE CASCADE,

	started_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
	finished_at TIMESTAMPTZ,

	status import_scan_status NOT NULL DEFAULT 'running',

	UNIQUE (id, source_id)
);

CREATE TYPE import_outcome AS ENUM (
	'unresolved',
    'admitted',
    'pending_duplicate_review',
    'duplicate_ignored',
    'duplicate_deleted'
);

CREATE TABLE import_entries (
	id BIGSERIAL PRIMARY KEY,
	source_id INT NOT NULL REFERENCES import_sources(id) ON DELETE CASCADE,
	relative_path BYTEA NOT NULL,

	-- Cheap change detection
	file_size BIGINT NOT NULL,
	file_modified_at TIMESTAMPTZ NOT NULL,
	file_identity BYTEA,
	change_token BYTEA,

	-- Exact duplicate detection
	hash_type TEXT NOT NULL, -- ie blake3 or sha256
	hash_value BYTEA NOT NULL,

	last_seen_scan_id BIGINT NOT NULL,
	FOREIGN KEY (last_seen_scan_id, source_id)
		REFERENCES import_scans(id, source_id),

	outcome import_outcome NOT NULL,
	admitted_content_id INT REFERENCES content(id) ON DELETE SET NULL,
	duplicate_content_id INT REFERENCES content(id) ON DELETE SET NULL,

	UNIQUE (source_id, relative_path)
);

CREATE INDEX import_entries_unique_hash ON import_entries (hash_type, hash_value);
CREATE INDEX import_scans_source_started ON import_scans (source_id, started_at DESC);
