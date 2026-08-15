-- Content Storage
CREATE TYPE storage_type AS ENUM ('managed', 'user_controlled');
CREATE TYPE path_format AS ENUM ('unix', 'windows');

CREATE TABLE storage_roots (
	id SERIAL PRIMARY KEY,
	kind storage_type NOT NULL,
	path_format path_format NOT NULL,
	path BYTEA NOT NULL,

	UNIQUE (path_format, path)
);

CREATE TYPE file_availability AS ENUM ('present', 'missing');

CREATE TABLE content_files (
	content_id INT PRIMARY KEY REFERENCES content(id) ON DELETE CASCADE,
	storage_root_id INT NOT NULL REFERENCES storage_roots(id),
	relative_path BYTEA NOT NULL,

	file_size BIGINT NOT NULL,
	modified_at TIMESTAMPTZ NOT NULL,

	availability file_availability NOT NULL DEFAULT 'present',
	UNIQUE (storage_root_id, relative_path)
);
