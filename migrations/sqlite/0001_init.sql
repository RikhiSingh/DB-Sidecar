PRAGMA journal_mode=WAL;

CREATE TABLE IF NOT EXISTS objects (
    id TEXT PRIMARY KEY,
    kind TEXT NOT NULL,
    qualified_name TEXT UNIQUE NOT NULL,
    canonical_ddl TEXT NOT NULL,
    hash TEXT NOT NULL,
    version INTEGER NOT NULL,
    changed_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS relations (
    src_id TEXT NOT NULL,
    dst_id TEXT NOT NULL,
    kind TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS schema_versions (
    version INTEGER PRIMARY KEY,
    merkle_root TEXT NOT NULL,
    created_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS object_stats (
    object_id TEXT NOT NULL,
    metric TEXT NOT NULL,
    value TEXT NOT NULL,
    collected_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS embeddings (
    object_id TEXT NOT NULL,
    embedding BLOB NOT NULL,
    provider TEXT NOT NULL,
    dim INTEGER NOT NULL,
    updated_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS queries (
    object_id TEXT NOT NULL,
    normalized_sql TEXT NOT NULL,
    example INTEGER NOT NULL,
    added_at TEXT NOT NULL
);

CREATE INDEX IF NOT EXISTS idx_objects_version ON objects(version);
CREATE INDEX IF NOT EXISTS idx_embeddings_object ON embeddings(object_id);
CREATE INDEX IF NOT EXISTS idx_stats_object ON object_stats(object_id);


