ifneq (,$(wildcard ./.env))
    include .env
    export
endif

fetch:
	cargo fetch

test:
	cargo test

build:
	cargo build

start:
	cargo run

.PHONY: fetch test build start
