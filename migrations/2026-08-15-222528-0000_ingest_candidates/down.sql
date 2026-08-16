ALTER TABLE content DROP COLUMN reviewed_at;

DROP TABLE IF EXISTS ingest_candidates_matches;
DROP TABLE IF EXISTS ingest_candidates;
