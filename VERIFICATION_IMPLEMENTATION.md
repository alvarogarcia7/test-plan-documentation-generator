# E2E Output Verification Implementation

## Overview

This document describes the implementation of the E2E output verification system for the `refactor-cli-single-multiple-modes` branch. The verification system ensures that the CLI refactoring maintains identical output behavior while changing the invocation interface.

## Background

The `refactor-cli-single-multiple-modes` branch refactors the CLI interface:

### Old Interface (main branch)
```bash
tpdg --container SCHEMA TEMPLATE DATA \
     --test-case VERIFICATION_DIR TEST_FILES...
```

### New Interface (this branch)
```bash
# For multiple test cases with type-based routing
tpdg --multiple-by-type TYPE_ATTR TEMPLATE_DIR TEST_FILES...

# For single template rendering
tpdg --single SCHEMA TEMPLATE DATA
```

## Implementation Components

### 1. Verification Script (`scripts/verify_e2e_outputs.sh`)

A comprehensive bash script that:

- **Builds the project** using `cargo build --release`
- **Runs 8 E2E test scenarios** covering all output types:
  1. Markdown output
  2. AsciiDoc test plan output
  3. AsciiDoc test results output
  4. Markdown test results output
  5. Input data markdown output
  6. Input data AsciiDoc test plan
  7. Input data AsciiDoc test results
  8. Input data markdown test results

- **For each test scenario**:
  1. Generates test cases using `--multiple-by-type` mode
  2. Applies the same sed/perl transformations as the Makefile E2E tests
  3. Creates a combined data file with test cases
  4. Renders the container template using `--single` mode
  5. Compares actual output with expected output using `diff`
  6. Reports detailed results with diffs for failures

- **Provides comprehensive reporting**:
  - Progress indicators (✓/✗) for each test
  - Detailed diff output for failed tests (first 50 lines)
  - Summary with counts of passed/failed tests
  - Proper exit codes (0 for success, 1 for failure)

### 2. Documentation (`scripts/README_VERIFY_E2E.md`)

Comprehensive documentation that covers:
- Purpose and goals of the verification script
- Detailed explanation of what it does
- Usage instructions
- Expected behavior
- Test coverage details
- Output format description
- Exit codes
- Troubleshooting guide
- Relationship to Makefile E2E tests

### 3. Makefile Integration

Added a new Makefile target:

```makefile
verify-e2e-outputs: build
	@echo "Running E2E output verification..."
	@./scripts/verify_e2e_outputs.sh
.PHONY: verify-e2e-outputs
```

Also updated the `help` target to include:
```
make verify-e2e-outputs    - Verify E2E test outputs match expected files
```

## Test Coverage

The verification script tests all the output files that are expected by the E2E tests:

| Test Name | Output File | Expected File |
|-----------|-------------|---------------|
| markdown-output | `data/output.actual.md` | `data/output.expected.md` |
| asciidoc-test-plan | `data/test_plan_output.actual.adoc` | `data/test_plan_output.expected.adoc` |
| asciidoc-test-results | `data/test_results_output.actual.adoc` | `data/test_results_output.expected.adoc` |
| markdown-test-results | `data/test_results_output.actual.md` | `data/test_results_output.expected.md` |
| input-data-markdown | `data/input_data/output.actual.md` | `data/input_data/output.expected.md` |
| input-data-test-plan-asciidoc | `data/input_data/test_plan_output.actual.adoc` | `data/input_data/test_plan_output.expected.adoc` |
| input-data-test-results-asciidoc | `data/input_data/test_results_output.actual.adoc` | `data/input_data/test_results_output.expected.adoc` |
| input-data-test-results-md | `data/input_data/test_results_output.actual.md` | `data/input_data/test_results_output.expected.md` |

## How It Works

### Step-by-Step Process

For each test scenario, the script performs the following steps:

1. **Generate Test Cases**
   ```bash
   tpdg --format FORMAT --output TEMP/test_cases.FORMAT \
        --multiple-by-type .type VERIFICATION_DIR TEST_FILES...
   ```

2. **Post-Process Test Cases**
   - Add spacing around section headers (Analysis, Demonstration, Inspection, Test)
   - Remove leading blank lines
   - Normalize trailing newlines

3. **Create Combined Data File**
   - Start with `test_cases_md: |` header
   - Indent test cases content by 2 spaces
   - Append container data from `container/data.yml`

4. **Render Container Template**
   ```bash
   tpdg --format FORMAT --output OUTPUT_FILE \
        --single SCHEMA TEMPLATE COMBINED_DATA
   ```

5. **Compare Output**
   ```bash
   diff -u EXPECTED_FILE OUTPUT_FILE
   ```

6. **Report Results**
   - Success: Add to PASSED_TESTS array
   - Failure: Add to FAILED_TESTS array with details

### Verification Logic

The script uses the same processing steps as the Makefile E2E tests:

- **Markdown files**: Use `## ` for section headers
- **AsciiDoc files**: Use `== ` for section headers
- **sed transformations**: Add spacing around headers
- **perl transformation**: Normalize trailing newlines

This ensures that the verification is testing the exact same transformation pipeline as the production E2E tests.

## Usage

### Running the Verification

```bash
# Using Make
make verify-e2e-outputs

# Direct execution
./scripts/verify_e2e_outputs.sh
```

### Expected Output

When all tests pass:
```
==========================================
SUMMARY
==========================================

Passed tests: 8
  ✓ markdown-output
  ✓ asciidoc-test-plan
  ✓ asciidoc-test-results
  ✓ markdown-test-results
  ✓ input-data-markdown
  ✓ input-data-test-plan-asciidoc
  ✓ input-data-test-results-asciidoc
  ✓ input-data-test-results-md

All tests passed! ✓

The output files generated by the new CLI interface (--single and --multiple-by-type)
match the expected output files, confirming that the refactoring maintains
the same behavior as the original --container and --test-case modes.
```

### When Tests Fail

If any test fails, the script will:
1. Show which command failed
2. Display the first 50 lines of the diff
3. Save the complete diff to `.tmp/verify-e2e-{test-name}/diff.txt`
4. Exit with code 1

## Benefits

1. **Confidence in Refactoring**: Proves that the new CLI interface produces identical outputs
2. **Automated Verification**: Can be run as part of CI/CD pipeline
3. **Detailed Diagnostics**: Provides clear feedback on what changed
4. **Comprehensive Coverage**: Tests all output formats and scenarios
5. **Easy to Use**: Simple `make` command or script execution
6. **Well Documented**: Clear documentation for maintenance and troubleshooting

## Maintenance

### Adding New Tests

To add a new verification test:

1. Add a new test section in `verify_e2e_outputs.sh`
2. Follow the existing pattern:
   - Create temporary directory
   - Run `--multiple-by-type` for test cases
   - Process test cases
   - Create combined data file
   - Run `--single` for container
   - Call `run_test` function
3. Update the documentation in `README_VERIFY_E2E.md`

### Updating Expected Files

If the output intentionally changes:

1. Review the diff to ensure changes are correct
2. Update the expected files in `data/` directories
3. Re-run verification to confirm

## Conclusion

The E2E output verification system provides a robust, automated way to ensure that the CLI refactoring maintains identical behavior while changing the interface. The implementation is comprehensive, well-documented, and easy to use, making it a valuable tool for validating the correctness of the refactoring effort.
