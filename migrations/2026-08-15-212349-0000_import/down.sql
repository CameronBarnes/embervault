DROP INDEX IF EXISTS import_scans_source_started;
DROP INDEX IF EXISTS import_entries_hash;

DROP TABLE IF EXISTS import_entries;
DROP TYPE import_outcome;

DROP TABLE IF EXISTS import_scans;
DROP TYPE import_scan_status;

DROP TABLE IF EXISTS automatic_imports;
DROP TYPE ingest_mode;

DROP TABLE IF EXISTS import_sources;
