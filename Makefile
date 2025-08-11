SHELL := /usr/bin/env bash

.PHONY: up down dev test lint fmt build docs

up:
	docker compose up -d postgres

down:
	docker compose down

dev:
	cargo run -p sidecar -- serve

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


