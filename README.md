Feytra — Postgres Schema Intelligence Sidecar (Rust)

Feytra is a Rust sidecar service that provides schema intelligence for PostgreSQL. It captures DDL changes and lightweight stats, builds canonical DDL hashes and a Merkle root, maintains a dependency graph, creates embeddings for schema/context packs, and exposes REST endpoints for Q&A and advisor suggestions.

Why

- Natural-language Q&A and advisor features benefit from structured, canonicalized schema context and embeddings.
- Operates as a sidecar outside the critical path. No proxying for queries.

Quick start

Prerequisites

- Rust 1.78+
- Docker (optional, for local PostgreSQL)

Steps

1. Copy environment: `cp .env.example .env` (or create `.env` with the keys shown below)
2. Start your PostgreSQL database (or use Docker: `make up`)
3. Initialize schema intelligence: `cargo run -p sidecar -- init`
4. Start the AI assistant: `cargo run -p sidecar -- chat`
5. Ask questions about your database!

Example session:

```bash
$ cargo run -p sidecar -- chat
🤖 Welcome to Feytra - Your AI Database Assistant!
==================================================
Database: postgresql://admin:admin@localhost:5432/myapp
Intelligence: local_stub embeddings, hnsw vectors

✅ Schema intelligence ready! Found 15 objects

💬 Ask me anything about your database schema!
Examples:
  • "What tables do I have?"
  • "Show me the structure of the users table"
  • "What are the relationships between tables?"
  • "Find tables related to authentication"
  • "refresh" - Update schema data
  • "exit" - End session

💭 What tables do I have?
🤔 📋 Found 8 tables:

1. public.users
2. public.orders
3. public.products
4. public.categories
5. public.order_items
6. public.user_sessions
7. public.product_reviews
8. public.shipping_addresses
```

Commands

## CLI Commands

- `cargo run -p sidecar -- chat` - Start AI database assistant (main feature)
- `cargo run -p sidecar -- init` - Initialize schema intelligence
- `cargo run -p sidecar -- refresh` - Refresh schema data

## Development Commands

- make up / make down: start/stop dockerized Postgres
- make test: `cargo test --all`
- make lint: `cargo clippy --all -- -D warnings`
- make fmt: `cargo fmt --all`
- make build: `cargo build --all`
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

Embeddings, vectors & LLMs

- Embedding providers (pluggable via `EMBED_PROVIDER`):
  - `local_stub` (default): deterministic pseudo-embeddings; no external calls
  - `openai`: real embeddings via OpenAI (`OPENAI_API_KEY`, `OPENAI_EMBED_MODEL`, `EMBED_DIM`)
- Vector index backends (`VECTOR_BACKEND`):
  - `hnsw` (default, in-process ephemeral)
  - `qdrant` (optional feature flag + `QDRANT_URL`)
- LLM SQL proposal (planned): set `LLM_PROVIDER=openai` and `OPENAI_MODEL=...` to enable model-backed SQL generation in `/v1/ask`.

Environment (.env)

Required or useful keys:

```
DATABASE_URL=postgresql://admin:admin@localhost:5432/feytra
SIDECAR_DB=./data/sidecar.sqlite

POLL_INTERVAL_SECS=300
ENABLE_NOTIFY=true
NOTIFY_CHANNEL=feytra_ddl
INSTALL_PG_EVENT_TRIGGER=false
BOOTSTRAP_ON_START=true

SAMPLE_ROWS=200
MASKING_RULES="email=.*email.*,name=.*name.*,phone=.*phone.*"

EMBED_PROVIDER=local_stub
EMBED_DIM=768
VECTOR_BACKEND=hnsw
# QDRANT_URL=http://localhost:6333

# OpenAI (optional)
OPENAI_API_KEY=
OPENAI_EMBED_MODEL=text-embedding-3-small
OPENAI_MODEL=gpt-4o-mini
LLM_PROVIDER=none

PORT=8080
RUST_LOG=info
```

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

- `feytra chat` — start AI database assistant (main feature)
- `feytra init` — initialize schema intelligence
- `feytra refresh` — refresh schema data

Chat Commands

- `tables` or `list tables` - Show all tables
- `schema` or `schema info` - Database overview
- `refresh` - Update schema data
- `help` or `?` - Show help
- `exit` or `quit` - End session

Change detection

- Polling: background job compares latest snapshot with previous (by hashes) every `POLL_INTERVAL_SECS`.
- NOTIFY: optional `ENABLE_NOTIFY=true` to listen on `NOTIFY_CHANNEL` (install event trigger with `INSTALL_PG_EVENT_TRIGGER=true`).
