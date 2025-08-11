# Feytra Roadmap

## v0.1 (current)

- [x] Postgres-only sidecar (no proxying)
- [x] Bootstrap crawl → canonicalize per-object DDL → hash → Merkle root
- [ ] Change detection: polling; NOTIFY plumbing in place (optional)
- [ ] Important data: rowcount estimates, pg_stats, masked sampling
- [x] Vector store: in-process HNSW; optional Qdrant (feature flag)
- [x] REST API (Axum) with OpenAPI/Swagger
- [x] Minimal CLI to run bootstrap/refresh/serve
- [x] SQLite metadata storage and migrations

## v0.2

- [ ] Minimal /ask NL→SQL using rule/template or small local model
- [ ] Expand canonicalization coverage for indexes/constraints/triggers/views/indexes
- [ ] Advisor: name/style lint and missing FKs

## v0.3

- [ ] BYOK for embeddings (OpenAI, Gemini)
- [ ] Qdrant as easier default via feature flag

## v0.4

- [ ] MySQL connector via same `CatalogProvider` trait

## v0.5

- [ ] Advisor: unused index detection and index analysis

## Stretch

- [ ] Live NOTIFY-driven fast-path refresh
- [ ] Richer sampling with column-level masking rules

## Single-maintainer notes

- Prioritize correctness of canonicalization and reproducible hashing.
- Keep `make up` and `make serve` flows fast; add tests per feature.
