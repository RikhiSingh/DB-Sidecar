SHELL := /usr/bin/env bash

.PHONY: up down dev serve bootstrap refresh test lint fmt build docs qdrant-up qdrant-down psql

up:
	docker compose up -d postgres

down:
	docker compose down

dev:
	cargo run -p sidecar -- serve

serve:
	cargo run -p sidecar -- serve

bootstrap:
	cargo run -p sidecar -- bootstrap

refresh:
	cargo run -p sidecar -- refresh

test:
	cargo test --all

lint:
	cargo clippy --all -- -D warnings

fmt:
	cargo fmt --all

build:
	cargo build --all

docs:
	curl -s http://localhost:8080/api-doc/openapi.json | jq . > openapi.json

qdrant-up:
	docker compose up -d qdrant

qdrant-down:
	docker compose stop qdrant

psql:
	docker compose exec -it postgres psql -U postgres -d postgres


