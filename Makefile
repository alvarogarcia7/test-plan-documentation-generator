init: install-prek-hooks
.PHONY: init

install-prek-hooks:
	@echo "Installing pre-commit hooks..."
	prek install
.PHONY: install-prek-hooks

help:
	@echo "Available targets:"
	@echo "  make build	   - Build the project"
	@echo "  make check	   - Check the project without building"
	@echo "  make test		- Run tests"
	@echo "  make lint		- Run all linting checks (fmt-check + clippy)"
	@echo "  make fmt		 - Format code with rustfmt"
	@echo "  make fmt-check   - Check code formatting without making changes"
	@echo "  make clippy	  - Run clippy linter"
	@echo "  make coverage	- Run code coverage and print report"
	@echo "  make clean	   - Remove build artifacts"
.PHONY: help

build:
	cargo build --release
.PHONY: build

check:
	cargo check
.PHONY: check

test: build
	cargo test --release --all-features --tests
	$(MAKE) test-e2e
	$(MAKE) test-e2e-asciidoc
	echo "All steps in test passing"
.PHONY: test

test-e2e:
	./target/release/test-plan-doc-gen \
	--output ./data/dataset_4_GSMA/output.actual.md \
	--container ./data/dataset_4_GSMA/container/schema.json ./data/dataset_4_GSMA/container/template.j2 ./data/dataset_4_GSMA/container/data.yml \
	--test-case ./data/dataset_4_GSMA/verification_methods ./data/dataset_4_GSMA/test_case/gsma_4.4.2.2_TC.yml ./data/dataset_4_GSMA/test_case/gsma_4.4.2.3_TC.yml ./data/dataset_4_GSMA/test_case/gsma_4.4.2.4_AN.yml ./data/dataset_4_GSMA/test_case/gsma_4.4.2.5_DM.yml ./data/dataset_4_GSMA/test_case/gsma_4.4.2.6_IN.yml \
	3>log_3.log
	diff ./data/dataset_4_GSMA/output.actual.md ./data/dataset_4_GSMA/output.expected.md
.PHONY: test-e2e

test-e2e-asciidoc:
	$(MAKE) test-e2e-test-plan-asciidoc
	$(MAKE) test-e2e-test-results-asciidoc
.PHONY: test-e2e-asciidoc


test-e2e-test-plan-asciidoc:
	./target/release/test-plan-doc-gen \
	--format asciidoc \
	--output ./data/dataset_4_GSMA/test_plan_output.actual.adoc \
	--container ./data/dataset_4_GSMA/container/schema.json ./data/dataset_4_GSMA/container/template_asciidoc.adoc ./data/dataset_4_GSMA/container/data.yml \
	--test-case ./data/dataset_4_GSMA/verification_methods ./data/dataset_4_GSMA/test_case/gsma_4.4.2.2_TC.yml ./data/dataset_4_GSMA/test_case/gsma_4.4.2.3_TC.yml ./data/dataset_4_GSMA/test_case/gsma_4.4.2.4_AN.yml ./data/dataset_4_GSMA/test_case/gsma_4.4.2.5_DM.yml ./data/dataset_4_GSMA/test_case/gsma_4.4.2.6_IN.yml \
	3>log_3.log
	diff ./data/dataset_4_GSMA/test_plan_output.actual.adoc ./data/dataset_4_GSMA/test_plan_output.expected.adoc
.PHONY: test-e2e-test-plan-asciidoc

test-e2e-test-results-asciidoc:
	./target/release/test-plan-doc-gen \
	--format asciidoc \
	--output ./data/dataset_4_GSMA/test_results_output.actual.adoc \
	--container ./data/dataset_4_GSMA/test_results/container_schema.json ./data/dataset_4_GSMA/test_results/container_template_asciidoc.adoc ./data/dataset_4_GSMA/test_results/container_data.yml \
	--test-case ./data/dataset_4_GSMA/verification_methods ./data/dataset_4_GSMA/test_results/sample_gsma_4.4.2.2_TC.yml ./data/dataset_4_GSMA/test_results/sample_gsma_4.4.2.3_TC.yml ./data/dataset_4_GSMA/test_results/sample_gsma_4.4.2.4_AN.yml ./data/dataset_4_GSMA/test_results/sample_gsma_4.4.2.5_DM.yml ./data/dataset_4_GSMA/test_results/sample_gsma_4.4.2.6_IN.yml \
	3>log_3.log
	diff ./data/dataset_4_GSMA/test_results_output.actual.adoc ./data/dataset_4_GSMA/test_results_output.expected.adoc
.PHONY: test-e2e-test-results-asciidoc

fmt:
	cargo fmt
.PHONY: fmt

fmt-check:
	cargo fmt -- --check
.PHONY: fmt-check

clippy:
	cargo clippy --all-targets --all-features -- -D warnings
.PHONY: clippy

lint: fmt-check clippy
	@echo "All linting checks passed!"
.PHONY: lint

coverage:
	@echo "Running code coverage analysis..."
	cargo tarpaulin --release --all-features --out Stdout --skip-clean --timeout 300
	@echo ""
	@echo "Coverage report generated!"
.PHONY: coverage

clean:
	cargo clean
.PHONY: clean

docker-build:
	docker build -t test:latest .
.PHONY: docker-build

