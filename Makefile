init: install-prek-hooks
.PHONY: init

install-prek-hooks:
	@echo "Installing pre-commit hooks..."
	prek install
.PHONY: install-prek-hooks

install-sccache:
	@echo "Installing sccache..."
	@chmod +x scripts/install-sccache.sh
	@scripts/install-sccache.sh
.PHONY: install-sccache

help:
	@echo "Available targets:"
	@echo "  make install-sccache - Install sccache for build caching"
	@echo "  make build	   - Build the project"
	@echo "  make check	   - Check the project without building"
	@echo "  make test		- Run tests"
	@echo "  make lint		- Run all linting checks (fmt-check + clippy)"
	@echo "  make fmt		 - Format code with rustfmt"
	@echo "  make fmt-check   - Check code formatting without making changes"
	@echo "  make clippy	  - Run clippy linter"
	@echo "  make coverage	- Run code coverage and print report"
	@echo "  make clean	   - Remove build artifacts"
	@echo "  make install-sccache - Install sccache locally"
	@echo "  make sccache-stats   - Show sccache statistics"
	@echo "  make sccache-clean   - Clean sccache directories"
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
	./target/release/tpdg \
	--output ./data/output.actual.md \
	--container ./data/container/schema.json ./data/container/template.j2 ./data/container/data.yml \
	--test-case ./data/verification_methods ./data/test_case/filter_test_01_TC.yml ./data/test_case/filter_test_02_AN.yml ./data/test_case/filter_test_03_IN.yml ./data/test_case/filter_test_04_DM.yml ./data/test_case/gsma_4.4.2.2_TC.yml ./data/test_case/gsma_4.4.2.3_TC.yml ./data/test_case/gsma_4.4.2.4_AN.yml ./data/test_case/gsma_4.4.2.5_DM.yml ./data/test_case/gsma_4.4.2.6_IN.yml \
	3>log_3.log
	diff ./data/output.actual.md ./data/output.expected.md
.PHONY: test-e2e

test-e2e-asciidoc:
	$(MAKE) test-e2e-test-plan-asciidoc
	$(MAKE) test-e2e-test-results-asciidoc
.PHONY: test-e2e-asciidoc


test-e2e-test-plan-asciidoc:
	./target/release/tpdg \
	--format asciidoc \
	--output ./data/test_plan_output.actual.adoc \
	--container ./data/container/schema.json ./data/container/template_asciidoc.adoc ./data/container/data.yml \
	--test-case ./data/verification_methods ./data/test_case/filter_test_01_TC.yml ./data/test_case/filter_test_02_AN.yml ./data/test_case/filter_test_03_IN.yml ./data/test_case/filter_test_04_DM.yml ./data/test_case/gsma_4.4.2.2_TC.yml ./data/test_case/gsma_4.4.2.3_TC.yml ./data/test_case/gsma_4.4.2.4_AN.yml ./data/test_case/gsma_4.4.2.5_DM.yml ./data/test_case/gsma_4.4.2.6_IN.yml \
	3>log_3.log
	diff ./data/test_plan_output.actual.adoc ./data/test_plan_output.expected.adoc
.PHONY: test-e2e-test-plan-asciidoc

test-e2e-test-results-asciidoc:
	./target/release/tpdg \
	--format asciidoc \
	--output ./data/test_results_output.actual.adoc \
	--container ./data/test_results/container_schema.json ./data/test_results/container_template_asciidoc.adoc ./data/test_results/container_data.yml \
	--test-case ./data/verification_methods ./data/test_results/sample_gsma_4.4.2.2_TC.yml ./data/test_results/sample_gsma_4.4.2.3_TC.yml ./data/test_results/sample_gsma_4.4.2.4_AN.yml ./data/test_results/sample_gsma_4.4.2.5_DM.yml ./data/test_results/sample_gsma_4.4.2.6_IN.yml \
	3>log_3.log
	diff ./data/test_results_output.actual.adoc ./data/test_results_output.expected.adoc
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
	rm -rf .sccache
.PHONY: clean

docker-build:
	mkdir -p .sccache/host
	docker build -t test:latest .
.PHONY: docker-build

install-sccache:
	scripts/install-sccache.sh --local
.PHONY: install-sccache

sccache-stats:
	sccache --show-stats
.PHONY: sccache-stats

sccache-clean:
	rm -rf .sccache/host .sccache/docker
.PHONY: sccache-clean

