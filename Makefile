.PHONY: help build check test lint format clean fmt-check clippy

help:
	@echo "Available targets:"
	@echo "  make build	   - Build the project"
	@echo "  make check	   - Check the project without building"
	@echo "  make test		- Run tests"
	@echo "  make lint		- Run all linting checks (fmt-check + clippy)"
	@echo "  make fmt		 - Format code with rustfmt"
	@echo "  make fmt-check   - Check code formatting without making changes"
	@echo "  make clippy	  - Run clippy linter"
	@echo "  make clean	   - Remove build artifacts"

build:
	cargo build --release

check:
	cargo check

test: build
	cargo test --release --all-features --tests
	$(MAKE) test-e2e
.PHONY: test

test-e2e:
	./target/release/test-plan-doc-gen \
	--output ./data/dataset_4_GSMA/output.actual.md \
	--container ./data/dataset_4_GSMA/container/schema.json ./data/dataset_4_GSMA/container/template.j2 ./data/dataset_4_GSMA/container/data.yml \
	--test-case ./data/dataset_4_GSMA/test_case/schema.json ./data/dataset_4_GSMA/test_case/template.j2 ./data/dataset_4_GSMA/test_case/*yml \
	3>log_3.log
	diff ./data/dataset_4_GSMA/output.actual.md ./data/dataset_4_GSMA/output.expected.md
.PHONY: test-e2e

fmt:
	cargo fmt

fmt-check:
	cargo fmt -- --check

clippy:
	cargo clippy --all-targets --all-features -- -D warnings

lint: fmt-check clippy
	@echo "All linting checks passed!"

clean:
	cargo clean
