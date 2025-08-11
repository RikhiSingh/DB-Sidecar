Feytra — Postgres Schema Intelligence Sidecar (Rust)

Feytra is a Rust sidecar service that provides schema intelligence for PostgreSQL. It captures DDL changes and lightweight stats, builds canonical DDL hashes and a Merkle root, maintains a dependency graph, creates embeddings for schema/context packs, and exposes REST endpoints for Q&A and advisor suggestions.

Why

- Natural-language Q&A and advisor features benefit from structured, canonicalized schema context and embeddings.
- Operates as a sidecar outside the critical path. No proxying for queries.

Quick start

Prerequisites

- Rust 1.78+
- Docker

Steps

1. Copy environment: `cp .env.example .env`
2. Start Postgres: `make up`
3. Serve API:
   - Unix: `make serve`
   - Windows PowerShell: run `cargo run -p sidecar -- serve`
4. Bootstrap snapshot:
   - `curl -X POST localhost:8080/v1/bootstrap`
5. Open Swagger at `http://localhost:8080/docs`
6. Refresh later: `curl -X POST localhost:8080/v1/refresh`

Commands

- make up / make down: start/stop dockerized Postgres
- make serve: run the API server (alias to `cargo run -p sidecar -- serve`)
- make bootstrap: run bootstrap once via CLI
- make refresh: run refresh via CLI
- make test: `cargo test --all`
- make lint: `cargo clippy --all -- -D warnings`
- make fmt: `cargo fmt --all`
- make build: `cargo build --all`
- make docs: export OpenAPI JSON to `openapi.json`
- make qdrant-up / qdrant-down: start/stop optional Qdrant
- make psql: open psql shell in the dockerized Postgres

Architecture

Components

- catalog: Postgres connectors, snapshotting, polling, canonical DDL builders
- fingerprint: canonicalizer + per-object hash + Merkle tree
- stats: rowcounts, pg_stats, masked sampling
- graph: dependency graph (tables, FKs, views, indexes)
- embed: text builders + pluggable embedding backends (LocalStub, OpenAI, Gemini)
- vectors: HNSW adapter + optional Qdrant client (feature flag)
- api: Axum handlers + OpenAPI
- jobs: bootstrap, refresh, periodic health scan
- config: .env/env vars

Data model (SQLite)

- objects(id TEXT PK, kind TEXT, qualified_name TEXT UNIQUE, canonical_ddl TEXT, hash TEXT, version INTEGER, changed_at TEXT)
- relations(src_id TEXT, dst_id TEXT, kind TEXT)
- schema_versions(version INTEGER PK, merkle_root TEXT, created_at TEXT)
- object_stats(object_id TEXT, metric TEXT, value TEXT, collected_at TEXT)
- embeddings(object_id TEXT, embedding BLOB, provider TEXT, dim INTEGER, updated_at TEXT)
- queries(object_id TEXT, normalized_sql TEXT, example BOOLEAN, added_at TEXT)

Canonicalization & hashing

- Lowercase unquoted identifiers, preserve quoted
- Normalize whitespace; stably sort order-insensitive parts
- Inline defaults; normalize now() to CURRENT_TIMESTAMP
- Include indexes/constraints in canonical form
- Hash: SHA256(canonical_ddl)
- Merkle: sorted (qualified_name, hash) → tree root stored per version

Change detection

- Event-driven via NOTIFY (optional) + periodic polling fallback

Embeddings & vectors

- Default embedding provider: LocalStub (deterministic)
- Default vector index: in-process HNSW; optional Qdrant via feature flag

Feature flags

- Enable Qdrant vector backend at build time:
  - `cargo build --features feytra-core/qdrant`
  - Requires `QDRANT_URL` and `VECTOR_BACKEND=qdrant`

Security & privacy

- Use read-only DB creds
- Mask sensitive samples via config rules
  - Configure `MASKING_RULES` as comma-separated regex=kind pairs, e.g. `MASKING_RULES="email=.*email.*,name=.*name.*,phone=.*phone.*"`
  - Kinds supported: email, name, phone, id (generic). Others default to `***`.

Roadmap

- v0.2: minimal /ask NL→SQL using local rule/template
- v0.3: BYOK for providers (OpenAI/Gemini) + Qdrant default
- v0.4: MySQL connector (same traits)
- v0.5: advisor index analysis, unused index detection

License

Apache-2.0

OpenAPI

- Served at `/docs`. The raw spec is available at `/api-doc/openapi.json`.

Endpoints

- GET `/v1/objects/{id}/stats`: latest collected metrics per object (estimated_rows, pg_stats per column)
- GET `/v1/objects/{id}/sample`: masked sample rows for the object

Dev tips

- Install Rust via `https://rustup.rs`.
- Build: `cargo build --all`
- Test: `cargo test --all`
- Lint: `cargo clippy --all -- -D warnings`
- Format: `cargo fmt --all`

CLI

- `sidecar serve` — start HTTP API on `PORT` (default 8080)
- `sidecar bootstrap` — run snapshot, compute canonical DDL, hashes, Merkle root, persist version
- `sidecar refresh` — detect diffs (event+poll; stub now), update changed objects, bump version

Endpoints

- `GET /health`
- `POST /v1/bootstrap`
- `POST /v1/refresh`
- `GET /v1/schema/{version}`
- `GET /v1/objects/{id}`
- `POST /v1/ask`
- `POST /v1/advice`
- `GET /v1/diff?from=&to=`

Change detection

- Polling: background job compares latest snapshot with previous (by hashes) every `POLL_INTERVAL_SECS`.
- NOTIFY: optional `ENABLE_NOTIFY=true` to listen on `NOTIFY_CHANNEL` (install event trigger with `INSTALL_PG_EVENT_TRIGGER=true`).
