include mk/strictdocs.mk

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
	@echo "  make test		- Run all tests (unit + E2E + input_data + logging example)"
	@echo "  make test-logging-example - Run template loading logging example"
	@echo "  make lint		- Run all linting checks (fmt-check + clippy)"
	@echo "  make fmt		 - Format code with rustfmt"
	@echo "  make fmt-check   - Check code formatting without making changes"
	@echo "  make clippy	  - Run clippy linter"
	@echo "  make coverage	- Run code coverage and print report"
	@echo "  make clean	   - Remove build artifacts"
	@echo "  make docker-build	- Build Docker image"
	@echo "  make install-sccache - Install sccache locally"
	@echo "  make sccache-stats   - Show sccache statistics"
	@echo "  make sccache-clean   - Clean sccache directories"
	@echo "  make verify-github-actions - Verify GitHub Actions CI pipeline status"
	@echo "  make check-gitlab-pipeline - Check GitLab CI pipeline status"
	@echo ""
	@echo "StrictDoc requirements management:"
	@echo "  make strictdoc-help  - Show StrictDoc-specific commands"
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
	$(MAKE) validate-testcases-data
	$(MAKE) test-logging-example
	$(MAKE) strictdoc-validate
	echo "All steps in test passing"
.PHONY: test

test-e2e:
	$(MAKE) test-e2e-markdown
	$(MAKE) test-e2e-asciidoc
	$(MAKE) test-e2e-input-data
	$(MAKE) test-e2e-input-data-asciidoc
.PHONY: test-e2e

test-e2e-markdown:
	@mkdir -p .tmp/tpdg-test-e2e-markdown
	RUST_LOG=debug ./target/release/tpdg \
	--format md \
	--output .tmp/tpdg-test-e2e-markdown/test_cases.md \
	--multiple-by-type .type ./data/verification_methods ./data/test_case/filter_test_01_TC.yml ./data/test_case/filter_test_02_AN.yml ./data/test_case/filter_test_03_IN.yml ./data/test_case/filter_test_04_DM.yml ./data/test_case/gsma_4.4.2.2_TC.yml ./data/test_case/gsma_4.4.2.3_TC.yml ./data/test_case/gsma_4.4.2.4_AN.yml ./data/test_case/gsma_4.4.2.5_DM.yml ./data/test_case/gsma_4.4.2.6_IN.yml \
	>log.log 2>&1
	./tests/not_empty.sh "${PWD}/log.log"
	@sed 's/^\(## \(Analysis\|Demonstration\|Inspection\|Test\):\)/\n\n\1/g' .tmp/tpdg-test-e2e-markdown/test_cases.md | sed '1{/^$$/d;}; 1{/^$$/d;}' > .tmp/tpdg-test-e2e-markdown/test_cases_fixed.tmp
	@perl -0777 -pe 's/\n+$$/\n/' .tmp/tpdg-test-e2e-markdown/test_cases_fixed.tmp > .tmp/tpdg-test-e2e-markdown/test_cases_fixed.md
	@echo "test_cases_md: |" > .tmp/tpdg-test-e2e-markdown/data_with_test_cases.yml
	@sed 's/^/  /' .tmp/tpdg-test-e2e-markdown/test_cases_fixed.md >> .tmp/tpdg-test-e2e-markdown/data_with_test_cases.yml
	@cat ./data/container/data.yml >> .tmp/tpdg-test-e2e-markdown/data_with_test_cases.yml
	RUST_LOG=debug ./target/release/tpdg \
	--format md \
	--output ./data/output.actual.md \
	--single ./data/container/schema.json ./data/container/template.j2 .tmp/tpdg-test-e2e-markdown/data_with_test_cases.yml \
	>>log.log 2>&1
	./tests/not_empty.sh "${PWD}/log.log"
	diff ./data/output.actual.md ./data/output.expected.md
	@rm -rf .tmp/tpdg-test-e2e-markdown
.PHONY: test-e2e-markdown

test-e2e-asciidoc:
	$(MAKE) test-e2e-test-plan-asciidoc
	$(MAKE) test-e2e-test-results-asciidoc
	$(MAKE) test-e2e-test-results-md
.PHONY: test-e2e-asciidoc

test-e2e-input-data-all:
	$(MAKE) test-e2e-input-data
	$(MAKE) test-e2e-input-data-test-plan-asciidoc
	$(MAKE) test-e2e-input-data-test-results-asciidoc
.PHONY: test-e2e-input-data-all


test-e2e-test-plan-asciidoc:
	@mkdir -p .tmp/tpdg-test-e2e-test-plan-asciidoc
	RUST_LOG=debug ./target/release/tpdg \
	--format adoc \
	--output .tmp/tpdg-test-e2e-test-plan-asciidoc/test_cases.adoc \
	--multiple-by-type .type ./data/verification_methods ./data/test_case/filter_test_01_TC.yml ./data/test_case/filter_test_02_AN.yml ./data/test_case/filter_test_03_IN.yml ./data/test_case/filter_test_04_DM.yml ./data/test_case/gsma_4.4.2.2_TC.yml ./data/test_case/gsma_4.4.2.3_TC.yml ./data/test_case/gsma_4.4.2.4_AN.yml ./data/test_case/gsma_4.4.2.5_DM.yml ./data/test_case/gsma_4.4.2.6_IN.yml \
	>log.log 2>&1
	./tests/not_empty.sh "${PWD}/log.log"
	@sed 's/^\(== \(Analysis\|Demonstration\|Inspection\|Test\):\)/\n\n\1/g' .tmp/tpdg-test-e2e-test-plan-asciidoc/test_cases.adoc | sed '1{/^$$/d;}; 1{/^$$/d;}' > .tmp/tpdg-test-e2e-test-plan-asciidoc/test_cases_fixed.tmp
	@printf '%s\n' "$$(cat .tmp/tpdg-test-e2e-test-plan-asciidoc/test_cases_fixed.tmp)" > .tmp/tpdg-test-e2e-test-plan-asciidoc/test_cases_fixed.adoc
	@echo "test_cases_md: |" > .tmp/tpdg-test-e2e-test-plan-asciidoc/data_with_test_cases.yml
	@sed 's/^/  /' .tmp/tpdg-test-e2e-test-plan-asciidoc/test_cases_fixed.adoc >> .tmp/tpdg-test-e2e-test-plan-asciidoc/data_with_test_cases.yml
	@cat ./data/container/data.yml >> .tmp/tpdg-test-e2e-test-plan-asciidoc/data_with_test_cases.yml
	RUST_LOG=debug ./target/release/tpdg \
	--format adoc \
	--output ./data/test_plan_output.actual.adoc \
	--single ./data/container/schema.json ./data/container/template_asciidoc.adoc .tmp/tpdg-test-e2e-test-plan-asciidoc/data_with_test_cases.yml \
	>>log.log 2>&1
	./tests/not_empty.sh "${PWD}/log.log"
	diff ./data/test_plan_output.actual.adoc ./data/test_plan_output.expected.adoc
	@rm -rf .tmp/tpdg-test-e2e-test-plan-asciidoc
.PHONY: test-e2e-test-plan-asciidoc

test-e2e-test-results-asciidoc:
	@mkdir -p .tmp/tpdg-test-e2e-test-results-asciidoc
	RUST_LOG=debug ./target/release/tpdg \
	--format adoc \
	--output .tmp/tpdg-test-e2e-test-results-asciidoc/test_cases.adoc \
	--multiple-by-type .type ./data/verification_methods ./data/test_results/sample_gsma_4.4.2.2_TC.yml ./data/test_results/sample_gsma_4.4.2.3_TC.yml ./data/test_results/sample_gsma_4.4.2.4_AN.yml ./data/test_results/sample_gsma_4.4.2.5_DM.yml ./data/test_results/sample_gsma_4.4.2.6_IN.yml \
	>log.log 2>&1
	./tests/not_empty.sh "${PWD}/log.log"
	@sed 's/^\(== \(Analysis\|Demonstration\|Inspection\|Test\):\)/\n\n\1/g' .tmp/tpdg-test-e2e-test-results-asciidoc/test_cases.adoc | sed '1{/^$$/d;}; 1{/^$$/d;}' > .tmp/tpdg-test-e2e-test-results-asciidoc/test_cases_fixed.tmp
	@printf '%s\n' "$$(cat .tmp/tpdg-test-e2e-test-results-asciidoc/test_cases_fixed.tmp)" > .tmp/tpdg-test-e2e-test-results-asciidoc/test_cases_fixed.adoc
	@echo "test_cases_md: |" > .tmp/tpdg-test-e2e-test-results-asciidoc/data_with_test_cases.yml
	@sed 's/^/  /' .tmp/tpdg-test-e2e-test-results-asciidoc/test_cases_fixed.adoc >> .tmp/tpdg-test-e2e-test-results-asciidoc/data_with_test_cases.yml
	@cat ./data/test_results/container_data.yml >> .tmp/tpdg-test-e2e-test-results-asciidoc/data_with_test_cases.yml
	RUST_LOG=debug ./target/release/tpdg \
	--format adoc \
	--output ./data/test_results_output.actual.adoc \
	--single ./data/test_results/container_schema.json ./data/test_results/container_template_asciidoc.adoc .tmp/tpdg-test-e2e-test-results-asciidoc/data_with_test_cases.yml \
	>>log.log 2>&1
	./tests/not_empty.sh "${PWD}/log.log"
	diff ./data/test_results_output.actual.adoc ./data/test_results_output.expected.adoc
	@rm -rf .tmp/tpdg-test-e2e-test-results-asciidoc
.PHONY: test-e2e-test-results-asciidoc

test-e2e-test-results-md:
	@mkdir -p .tmp/tpdg-test-e2e-test-results-md
	RUST_LOG=debug ./target/release/tpdg \
	--format md \
	--output .tmp/tpdg-test-e2e-test-results-md/test_cases.md \
	--multiple-by-type .type ./data/verification_methods ./data/test_results/sample_gsma_4.4.2.2_TC.yml ./data/test_results/sample_gsma_4.4.2.3_TC.yml ./data/test_results/sample_gsma_4.4.2.4_AN.yml ./data/test_results/sample_gsma_4.4.2.5_DM.yml ./data/test_results/sample_gsma_4.4.2.6_IN.yml \
	>log.log 2>&1
	./tests/not_empty.sh "${PWD}/log.log"
	@sed 's/^\(## \(Analysis\|Demonstration\|Inspection\|Test\):\)/\n\n\1/g' .tmp/tpdg-test-e2e-test-results-md/test_cases.md | sed '1{/^$$/d;}; 1{/^$$/d;}' > .tmp/tpdg-test-e2e-test-results-md/test_cases_fixed.tmp
	@printf '%s\n' "$$(cat .tmp/tpdg-test-e2e-test-results-md/test_cases_fixed.tmp)" > .tmp/tpdg-test-e2e-test-results-md/test_cases_fixed.md
	@echo "test_cases_md: |" > .tmp/tpdg-test-e2e-test-results-md/data_with_test_cases.yml
	@sed 's/^/  /' .tmp/tpdg-test-e2e-test-results-md/test_cases_fixed.md >> .tmp/tpdg-test-e2e-test-results-md/data_with_test_cases.yml
	@cat ./data/test_results/container_data.yml >> .tmp/tpdg-test-e2e-test-results-md/data_with_test_cases.yml
	RUST_LOG=debug ./target/release/tpdg \
	--format md \
	--output ./data/test_results_output.actual.md \
	--single ./data/test_results/container_schema.json ./data/test_results/container_template.j2 .tmp/tpdg-test-e2e-test-results-md/data_with_test_cases.yml \
	>>log.log 2>&1
	./tests/not_empty.sh "${PWD}/log.log"
	diff ./data/test_results_output.actual.md ./data/test_results_output.expected.md
	@rm -rf .tmp/tpdg-test-e2e-test-results-md
.PHONY: test-e2e-test-results-md

test-e2e-input-data:
	@mkdir -p .tmp/tpdg-test-e2e-input-data
	RUST_LOG=debug ./target/release/tpdg \
	--format md \
	--output .tmp/tpdg-test-e2e-input-data/test_cases.md \
	--multiple-by-type .type ./data/input_data/verification_methods ./data/input_data/test_case/TEST_PASSING_001.yml ./data/input_data/test_case/TEST_FAILING_002.yml ./data/input_data/test_case/gsma_4.4.2.2_TC.yml ./data/input_data/test_case/gsma_4.4.2.3_TC.yml \
	>log.log 2>&1
	./tests/not_empty.sh "${PWD}/log.log"
	@sed 's/^\(## \(Analysis\|Demonstration\|Inspection\|Test\):\)/\n\n\1/g' .tmp/tpdg-test-e2e-input-data/test_cases.md | sed '1{/^$$/d;}; 1{/^$$/d;}' > .tmp/tpdg-test-e2e-input-data/test_cases_fixed.tmp
	@printf '%s\n' "$$(cat .tmp/tpdg-test-e2e-input-data/test_cases_fixed.tmp)" > .tmp/tpdg-test-e2e-input-data/test_cases_fixed.md
	@echo "test_cases_md: |" > .tmp/tpdg-test-e2e-input-data/data_with_test_cases.yml
	@sed 's/^/  /' .tmp/tpdg-test-e2e-input-data/test_cases_fixed.md >> .tmp/tpdg-test-e2e-input-data/data_with_test_cases.yml
	@cat ./data/input_data/container/data.yml >> .tmp/tpdg-test-e2e-input-data/data_with_test_cases.yml
	RUST_LOG=debug ./target/release/tpdg \
	--format md \
	--output ./data/input_data/output.actual.md \
	--single ./data/input_data/container/schema.json ./data/input_data/container/template.j2 .tmp/tpdg-test-e2e-input-data/data_with_test_cases.yml \
	>>log.log 2>&1
	./tests/not_empty.sh "${PWD}/log.log"
	diff ./data/input_data/output.actual.md ./data/input_data/output.expected.md
	@rm -rf .tmp/tpdg-test-e2e-input-data
.PHONY: test-e2e-input-data

test-e2e-input-data-asciidoc:
	$(MAKE) test-e2e-input-data-test-plan-asciidoc
	$(MAKE) test-e2e-input-data-test-results-asciidoc
	$(MAKE) test-e2e-input-data-test-results-md
.PHONY: test-e2e-input-data-asciidoc

test-e2e-input-data-test-plan-asciidoc:
	@mkdir -p .tmp/tpdg-test-e2e-input-data-test-plan-asciidoc
	RUST_LOG=debug ./target/release/tpdg \
	--format adoc \
	--output .tmp/tpdg-test-e2e-input-data-test-plan-asciidoc/test_cases.adoc \
	--multiple-by-type .type ./data/input_data/verification_methods ./data/input_data/test_case/TEST_PASSING_001.yml ./data/input_data/test_case/TEST_FAILING_002.yml ./data/input_data/test_case/gsma_4.4.2.2_TC.yml ./data/input_data/test_case/gsma_4.4.2.3_TC.yml \
	>log.log 2>&1
	./tests/not_empty.sh "${PWD}/log.log"
	@sed 's/^\(== \(Analysis\|Demonstration\|Inspection\|Test\):\)/\n\n\1/g' .tmp/tpdg-test-e2e-input-data-test-plan-asciidoc/test_cases.adoc | sed '1{/^$$/d;}; 1{/^$$/d;}' > .tmp/tpdg-test-e2e-input-data-test-plan-asciidoc/test_cases_fixed.tmp
	@printf '%s\n' "$$(cat .tmp/tpdg-test-e2e-input-data-test-plan-asciidoc/test_cases_fixed.tmp)" > .tmp/tpdg-test-e2e-input-data-test-plan-asciidoc/test_cases_fixed.adoc
	@echo "test_cases_md: |" > .tmp/tpdg-test-e2e-input-data-test-plan-asciidoc/data_with_test_cases.yml
	@sed 's/^/  /' .tmp/tpdg-test-e2e-input-data-test-plan-asciidoc/test_cases_fixed.adoc >> .tmp/tpdg-test-e2e-input-data-test-plan-asciidoc/data_with_test_cases.yml
	@cat ./data/input_data/container/data.yml >> .tmp/tpdg-test-e2e-input-data-test-plan-asciidoc/data_with_test_cases.yml
	RUST_LOG=debug ./target/release/tpdg \
	--format adoc \
	--output ./data/input_data/test_plan_output.actual.adoc \
	--single ./data/input_data/container/schema.json ./data/input_data/container/template_asciidoc.adoc .tmp/tpdg-test-e2e-input-data-test-plan-asciidoc/data_with_test_cases.yml \
	>>log.log 2>&1
	./tests/not_empty.sh "${PWD}/log.log"
	diff ./data/input_data/test_plan_output.actual.adoc ./data/input_data/test_plan_output.expected.adoc
	@rm -rf .tmp/tpdg-test-e2e-input-data-test-plan-asciidoc
.PHONY: test-e2e-input-data-test-plan-asciidoc

test-e2e-input-data-test-results-asciidoc:
	@mkdir -p .tmp/tpdg-test-e2e-input-data-test-results-asciidoc
	RUST_LOG=debug ./target/release/tpdg \
	--format adoc \
	--output .tmp/tpdg-test-e2e-input-data-test-results-asciidoc/test_cases.adoc \
	--multiple-by-type .type ./data/input_data/verification_methods ./data/input_data/test_results/RESULT_TEST_PASSING_001.yml ./data/input_data/test_results/RESULT_TEST_FAILING_002.yml \
	>log.log 2>&1
	./tests/not_empty.sh "${PWD}/log.log"
	@sed 's/^\(== \(Analysis\|Demonstration\|Inspection\|Test\):\)/\n\n\1/g' .tmp/tpdg-test-e2e-input-data-test-results-asciidoc/test_cases.adoc | sed '1{/^$$/d;}; 1{/^$$/d;}' > .tmp/tpdg-test-e2e-input-data-test-results-asciidoc/test_cases_fixed.tmp
	@printf '%s\n' "$$(cat .tmp/tpdg-test-e2e-input-data-test-results-asciidoc/test_cases_fixed.tmp)" > .tmp/tpdg-test-e2e-input-data-test-results-asciidoc/test_cases_fixed.adoc
	@echo "test_cases_md: |" > .tmp/tpdg-test-e2e-input-data-test-results-asciidoc/data_with_test_cases.yml
	@sed 's/^/  /' .tmp/tpdg-test-e2e-input-data-test-results-asciidoc/test_cases_fixed.adoc >> .tmp/tpdg-test-e2e-input-data-test-results-asciidoc/data_with_test_cases.yml
	@cat ./data/input_data/test_results/container_data.yml >> .tmp/tpdg-test-e2e-input-data-test-results-asciidoc/data_with_test_cases.yml
	RUST_LOG=debug ./target/release/tpdg \
	--format adoc \
	--output ./data/input_data/test_results_output.actual.adoc \
	--single ./data/input_data/test_results/container_schema.json ./data/input_data/test_results/container_template_asciidoc.adoc .tmp/tpdg-test-e2e-input-data-test-results-asciidoc/data_with_test_cases.yml \
	>>log.log 2>&1
	./tests/not_empty.sh "${PWD}/log.log"
	diff ./data/input_data/test_results_output.actual.adoc ./data/input_data/test_results_output.expected.adoc
	@rm -rf .tmp/tpdg-test-e2e-input-data-test-results-asciidoc
.PHONY: test-e2e-input-data-test-results-asciidoc

test-e2e-input-data-test-results-md:
	@mkdir -p .tmp/tpdg-test-e2e-input-data-test-results-md
	RUST_LOG=debug ./target/release/tpdg \
	--format md \
	--output .tmp/tpdg-test-e2e-input-data-test-results-md/test_cases.md \
	--multiple-by-type .type ./data/input_data/verification_methods ./data/input_data/test_results/RESULT_TEST_PASSING_001.yml ./data/input_data/test_results/RESULT_TEST_FAILING_002.yml \
	>log.log 2>&1
	./tests/not_empty.sh "${PWD}/log.log"
	@sed 's/^\(## \(Analysis\|Demonstration\|Inspection\|Test\):\)/\n\n\1/g' .tmp/tpdg-test-e2e-input-data-test-results-md/test_cases.md | sed '1{/^$$/d;}; 1{/^$$/d;}' > .tmp/tpdg-test-e2e-input-data-test-results-md/test_cases_fixed.tmp
	@printf '%s\n' "$$(cat .tmp/tpdg-test-e2e-input-data-test-results-md/test_cases_fixed.tmp)" > .tmp/tpdg-test-e2e-input-data-test-results-md/test_cases_fixed.md
	@echo "test_cases_md: |" > .tmp/tpdg-test-e2e-input-data-test-results-md/data_with_test_cases.yml
	@sed 's/^/  /' .tmp/tpdg-test-e2e-input-data-test-results-md/test_cases_fixed.md >> .tmp/tpdg-test-e2e-input-data-test-results-md/data_with_test_cases.yml
	@cat ./data/input_data/test_results/container_data.yml >> .tmp/tpdg-test-e2e-input-data-test-results-md/data_with_test_cases.yml
	RUST_LOG=debug ./target/release/tpdg \
	--format md \
	--output ./data/input_data/test_results_output.actual.md \
	--single ./data/input_data/test_results/container_schema.json ./data/input_data/test_results/container_template.j2 .tmp/tpdg-test-e2e-input-data-test-results-md/data_with_test_cases.yml \
	>>log.log 2>&1
	./tests/not_empty.sh "${PWD}/log.log"
	diff ./data/input_data/test_results_output.actual.md ./data/input_data/test_results_output.expected.md
	@rm -rf .tmp/tpdg-test-e2e-input-data-test-results-md
.PHONY: test-e2e-input-data-test-results-md

validate-testcases-data:
	@echo "Validating testcases_results_container data against schema..."
	@uv run python scripts/validate_schema.py \
		./data/testcase_results_container/schema.json \
		./data/testcase_results_container/data_sample.yml
.PHONY: validate-testcases-data

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

verify-github-actions:
	@chmod +x verify-github-actions.sh
	@./verify-github-actions.sh unmodified_push_2026-03-12
.PHONY: verify-github-actions

check-gitlab-pipeline:
	@chmod +x check-pipeline-status.sh
	@./check-pipeline-status.sh
.PHONY: check-gitlab-pipeline

test-logging-example:
	@echo ""
	@echo "=========================================="
	@echo "Running Template Loading Logging Example"
	@echo "=========================================="
	@echo ""
	@echo "This example demonstrates template loading logging."
	@echo "Watch stderr for log messages showing absolute template paths."
	@echo ""
	RUST_LOG=debug ./target/release/tpdg \
	--format md \
	--output ./data/logging_example/output.actual.md \
	--container ./data/logging_example/container/schema.json ./data/logging_example/container/template.j2 ./data/logging_example/container/data.yml \
	--test-case ./data/logging_example/verification_methods ./data/logging_example/test_case/analysis_case_01.yml ./data/logging_example/test_case/test_case_01.yml ./data/logging_example/test_case/test_case_02.yml \
	2>&1 | tee /dev/stderr | grep "Loading"
	@echo ""
	@echo "Verifying output matches expected..."
	diff ./data/logging_example/output.actual.md ./data/logging_example/output.expected.md
	@echo ""
	@echo "✓ Template loading logging example passed!"
	@echo ""
.PHONY: test-logging-example

