---
id: TPDG-1
title: Tera filters for report
status: Done
assignee: []
created_date: '2026-03-10 08:10'
updated_date: '2026-03-23 10:07'
labels: []
milestone: m-0
dependencies: []
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Implement custom Tera filters: replace, replace_regex, and strip

Add three custom Tera template filters (replace, replace_regex, strip) to the CLI tool by implementing the Filter trait, registering them with all Tera instances (test cases, requirements aggregation, and container templates), and creating comprehensive unit tests to verify functionality including edge cases.
<!-- SECTION:DESCRIPTION:END -->

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
* Implement filters using the modern Filter trait pattern requiring Sync + Send instead of function pointers, ensuring thread-safety for template rendering
* Add the regex crate dependency for replace_regex filter implementation rather than using string-based replacements to support proper regex patterns
* Register custom filters on all three Tera instances created in main() (test case templates, requirement aggregation template, and container template) to ensure filters are available everywhere in the template rendering pipeline
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
## Tasks

Add regex crate as a dependency in Cargo.toml (version 1.x) to enable regex pattern matching for the replace_regex filter.

Implement three filter structs (ReplaceFilter, ReplaceRegexFilter, StripFilter) in src/main.rs that implement the tera::Filter trait with filter() method signature fn filter(&self, value: &Value, args: &HashMap<String, Value>) -> Result<Value>, extracting string values using value.as_str(), handling named arguments (old, new, times for replace filters), and returning tera::Result<Value> with proper error handling for type mismatches and missing arguments.

Create helper function register_custom_filters(tera: &mut Tera) in src/main.rs that calls tera.register_filter() for all three custom filters (replace, replace_regex, strip), and invoke this helper on all Tera instances created in main() (lines 238, 452, 485) and in the test helper render_template() function (line 36) to ensure filters are available in all template contexts.

Add comprehensive unit tests in src/main.rs tests module covering: replace filter with basic string replacement, times parameter limiting replacements, missing required arguments; replace_regex filter with regex patterns, capture groups, times parameter, invalid regex patterns; strip filter removing leading/trailing whitespace, handling strings with no whitespace, and non-string input error handling.

Run make test to execute all unit tests and E2E tests, verify all tests pass including the new filter tests, then run make docker-build to ensure the Docker image builds successfully with the new regex dependency and filter implementations.

## Testing

Test Plan for Custom Tera Filters Implementation

Comprehensive testing of three custom Tera filters (replace, replace_regex, strip) including unit test verification, integration testing with real templates, Docker build validation, and manual E2E workflow verification to ensure filters work correctly across all template contexts.

### Tasks

Code review for test-plan-documentation-generator/add-tera-custom-filters: Review the branch diff to analyze code quality, identify potential bugs, security vulnerabilities, and verify adherence to project conventions. Fix any issues found.

Verify custom filter unit tests: run cargo test --release --tests test_replace_filter test_replace_regex_filter test_strip_filter to execute all 67 unit tests for the three custom filters (replace, replace_regex, strip), confirming basic string replacement, multiple occurrences, times parameter limiting, edge cases (empty strings, no matches), error handling (missing arguments, non-string inputs), regex patterns (character classes, capture groups, anchors, quantifiers), and whitespace handling (leading/trailing/mixed).

Integration test for custom filters in real templates: create test templates in data/test_case/ that use the new filters (replace, replace_regex, strip) in realistic scenarios (e.g., sanitizing test IDs, normalizing whitespace, formatting requirements text), then run cargo test --release --tests test_e2e_dataset_4_gsma to verify filters work correctly when rendering test plans with the existing dataset.

Execute full test suite: run make test to execute all 108 unit tests and 11 E2E tests (test-e2e, test-e2e-test-plan-asciidoc, test-e2e-test-results-asciidoc), verifying that all tests pass including the 67 new custom filter tests, existing requirement aggregation tests, schema validation tests, and E2E rendering tests with both Markdown and AsciiDoc output formats.

Docker build validation: run make docker-build to build the Docker image, verify the regex crate dependency is correctly resolved during the Docker build process, confirm all tests pass inside the container (including unit tests and E2E tests), and validate that sccache compilation caching works correctly with the new regex dependency.

## Acceptance test

Add comprehensive E2E test in tests/e2e.rs that creates a template using all three custom filters (replace, replace_regex, strip) with test data, validates correct filter functionality through multiple assertions, and includes edge cases like chained filters and filter combinations.

### Key Decisions

Use E2E test approach with Command execution instead of unit tests to verify filters work in real template rendering pipeline end-to-end

Test all three filters together in a single comprehensive test rather than separate tests per filter to validate filter interaction and chaining

Include both simple assertions on specific outputs and a snapshot assertion to detect any regression in filter behavior

### Tasks

Add test_e2e_custom_tera_filters E2E test to tests/e2e.rs that creates temporary YAML data file with string fields for filter testing (including text with whitespace, text with patterns, text with regex matches), creates container template using all three custom filters (replace, replace_regex, strip) in various combinations including chained filters, creates verification methods directory structure, executes the binary with the test data, and validates output contains expected transformed strings (e.g., text with replaced patterns, regex-transformed text, stripped whitespace) using specific assertions and a snapshot.
<!-- SECTION:NOTES:END -->
