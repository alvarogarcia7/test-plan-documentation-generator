.PHONY: help build check test lint format clean fmt-check clippy

help:
	@echo "Available targets:"
	@echo "  make build       - Build the project"
	@echo "  make check       - Check the project without building"
	@echo "  make test        - Run tests"
	@echo "  make lint        - Run all linting checks (fmt-check + clippy)"
	@echo "  make fmt         - Format code with rustfmt"
	@echo "  make fmt-check   - Check code formatting without making changes"
	@echo "  make clippy      - Run clippy linter"
	@echo "  make clean       - Remove build artifacts"

build:
	cargo build --release

check:
	cargo check

test:
	cargo test

fmt:
	cargo fmt

fmt-check:
	cargo fmt -- --check

clippy:
	cargo clippy -- -D warnings

lint: fmt-check clippy
	@echo "All linting checks passed!"

clean:
	cargo clean
