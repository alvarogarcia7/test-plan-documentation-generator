# Test Plan Documentation Generator

A Rust CLI tool that generates test plan documentation from JSON schemas, Jinja2 templates, and YAML data files. It validates data against schemas and renders customizable Markdown or AsciiDoc output for test documentation.

## Overview

The Test Plan Documentation Generator is designed to streamline the creation of test documentation by:

- **Validating** test data against JSON schemas to ensure consistency
- **Rendering** templates using the Tera templating engine (Jinja2-like syntax)
- **Aggregating** test results and requirements into comprehensive reports
- **Supporting** multiple verification methods (test, analysis, demonstration, inspection)
- **Generating** both Markdown and AsciiDoc output formats

The tool processes test data files, validating each against their respective schemas before generating the final documentation.

## Installation

### Prerequisites

- Rust toolchain (1.70 or later)

### Build from Source

```bash
# Clone the repository
git clone <repository-url>
cd tpdg

# Build release binary
cargo build --release

# Binary will be available at:
./target/release/tpdg
```

### Using Pre-built Binary

If a pre-built binary is available, download it and make it executable:

```bash
chmod +x tpdg
```

## Quick Start Guide

The tool supports three modes of operation for different use cases:

### Single Mode - Render One Input File

For rendering a single input file with a schema and template:

```bash
./target/release/tpdg \
  --single ./schemas/test_case.json \
           ./templates/test_case.j2 \
           ./data/test_case/tc001.yml \
  --output ./output/tc001.md
```

This validates `tc001.yml` against the schema and renders it using the template.

### Multiple Mode - Render Many Files with Same Schema

For rendering multiple input files that share the same schema and template:

```bash
./target/release/tpdg \
  --multiple ./schemas/test_case.json \
             ./templates/test_case.j2 \
             ./data/test_case/tc001.yml \
             ./data/test_case/tc002.yml \
             ./data/test_case/tc003.yml \
  --output ./output/all_tests.md
```

All test cases are validated against the same schema, rendered with the same template, and concatenated into a single output.

### Multiple-by-Type Mode - Render Files with Type-Specific Templates

For rendering multiple files where each has a `type` attribute that determines which schema and template to use:

```bash
./target/release/tpdg \
  --multiple-by-type type \
                     ./data/verification_methods \
                     ./data/test_case/tc001.yml \
                     ./data/test_case/tc002.yml \
                     ./data/test_case/an001.yml \
  --output ./output/mixed_cases.md
```

Each input file is validated against `<template_dir>/<type>/schema.json` and rendered with `<template_dir>/<type>/template.j2` (or `.adoc`), then all outputs are concatenated.

### Chaining Commands for Two-Stage Processing

You can chain modes to build complex documentation workflows:

```bash
# Step 1: Generate test cases using multiple-by-type mode
./target/release/tpdg \
  --multiple-by-type type \
                     ./data/verification_methods \
                     ./data/test_case/*.yml \
  --output ./tmp/test_cases.md

# Step 2: Use the generated test cases in a wrapper template
# (Prepare data.yml to include the test cases content)
./target/release/tpdg \
  --single ./data/wrapper/schema.json \
           ./data/wrapper/template.j2 \
           ./data/wrapper/data.yml \
  --output ./test_plan.md
```

Note: The wrapper template can reference the generated test cases by including their content in the data YAML file.

## CLI Reference

### Synopsis

```
# Single mode
tpdg [OPTIONS] --single <SCHEMA> <TEMPLATE> <INPUT>

# Multiple mode
tpdg [OPTIONS] --multiple <SCHEMA> <TEMPLATE> <INPUT_FILES>...

# Multiple-by-type mode
tpdg [OPTIONS] --multiple-by-type <TYPE_ATTR_PATH> <TEMPLATE_DIR> <INPUT_FILES>...
```

### Operating Modes

The tool has three mutually exclusive operating modes. You must specify exactly one of: `--single`, `--multiple`, or `--multiple-by-type`.

### Common Options

#### `-o, --output <FILE>`

Specifies the output file path. If not provided, output is written to stdout.

**Example:**
```bash
--output ./report.md
-o ./docs/test_plan.adoc
```

#### `--format <FORMAT>`

Specifies the output format. Accepted values: `md`, `adoc`

**Default:** `adoc`

**Example:**
```bash
--format md
--format adoc
```

### Mode: Single

**Synopsis:**
```bash
tpdg --single <SCHEMA> <TEMPLATE> <INPUT> [OPTIONS]
```

Validates and renders a single input file using the provided schema and template.

**Arguments:**
- `SCHEMA` - JSON schema file for validation
- `TEMPLATE` - Tera template file (`.j2` or `.adoc`)
- `INPUT` - YAML input data file

**Example:**
```bash
./target/release/tpdg \
  --single ./schemas/test_case.json \
           ./templates/test_case.j2 \
           ./data/test_case/tc001.yml \
  --output ./output/tc001.md
```

### Mode: Multiple

**Synopsis:**
```bash
tpdg --multiple <SCHEMA> <TEMPLATE> <INPUT_FILES>... [OPTIONS]
```

Validates and renders multiple input files using a single schema and template. All rendered outputs are concatenated.

**Arguments:**
- `SCHEMA` - JSON schema file for validation (shared by all inputs)
- `TEMPLATE` - Tera template file (shared by all inputs)
- `INPUT_FILES` - One or more YAML input data files

**Example:**
```bash
./target/release/tpdg \
  --multiple ./schemas/test_case.json \
             ./templates/test_case.j2 \
             ./data/test_case/tc001.yml \
             ./data/test_case/tc002.yml \
             ./data/test_case/tc003.yml \
  --output ./output/all_tests.md
```

### Mode: Multiple-by-Type

**Synopsis:**
```bash
tpdg --multiple-by-type <TYPE_ATTR_PATH> <TEMPLATE_DIR> <INPUT_FILES>... [OPTIONS]
```

Validates and renders multiple input files where each file contains a `type` attribute that determines which schema and template to use. Each input is validated against `<TEMPLATE_DIR>/<type>/schema.json` and rendered with `<TEMPLATE_DIR>/<type>/template.{j2|adoc}`.

**Arguments:**
- `TYPE_ATTR_PATH` - Path to the type attribute in the YAML (e.g., `type` or `metadata.type`)
- `TEMPLATE_DIR` - Directory containing subdirectories for each type
- `INPUT_FILES` - One or more YAML input data files

**Example:**
```bash
./target/release/tpdg \
  --multiple-by-type type \
                     ./data/verification_methods \
                     ./data/test_case/tc001.yml \
                     ./data/test_case/tc002.yml \
                     ./data/test_case/an001.yml \
  --output ./output/mixed_cases.md
```

**Directory Structure Expected:**
```
./data/verification_methods/
├── test/
│   ├── schema.json
│   └── template.j2
├── analysis/
│   ├── schema.json
│   └── template.j2
└── demonstration/
    ├── schema.json
    └── template.j2
```

### Exit Codes

- `0` - Success
- `1` - Usage error (missing or invalid arguments)
- `2` - File not found error
- `3` - Validation error (schema validation failed)

## Template System Overview

The tool uses the [Tera](https://tera.netlify.app/) templating engine, which provides Jinja2-like syntax.

### Template Variables

Templates receive all fields from the input YAML file as top-level variables.

**Example template:**
```jinja2
## Test Case: {{ id | strip }}

**Requirement**: {{ requirement }}
**Item**: {{ item }}
**TC**: {{ tc }}

### Description

{{ description | strip }}

### Test Sequences

{% for ts in test_sequences %}
#### Test Sequence {{ ts.id }}: {{ ts.name | strip }}

{{ ts.description | strip }}

| Step | Action | Expected Result |
|------|--------|-----------------|
{% for step in ts.steps -%}
| {{ step.step }} | {{ step.description }} | {{ step.expected.result }} |
{% endfor %}
{% endfor %}
```

### Template Syntax

Tera supports standard Jinja2 syntax:

**Variables:**
```jinja2
{{ variable_name }}
{{ object.field }}
{{ array[0] }}
```

**Control Structures:**
```jinja2
{% if condition %}
  Content
{% elif other_condition %}
  Other content
{% else %}
  Default content
{% endif %}

{% for item in items %}
  {{ item }}
{% endfor %}
```

**Filters:**
```jinja2
{{ text | upper }}
{{ text | lower }}
{{ array | length }}
{{ text | strip }}
{{ text | replace(old="foo", new="bar") }}
```

## Custom Filter Documentation

The tool provides three custom Tera filters beyond the standard Tera filters.

### `strip` Filter

Removes leading and trailing whitespace from a string.

**Syntax:**
```jinja2
{{ value | strip }}
```

**Parameters:** None

**Example:**
```jinja2
Input:  "  hello world  "
Output: "hello world"

Template: {{ description | strip }}
```

**Use Cases:**
- Cleaning up whitespace in YAML data
- Normalizing test case descriptions
- Formatting IDs and labels

### `replace` Filter

Replaces occurrences of a substring with another string.

**Syntax:**
```jinja2
{{ value | replace(old="search", new="replacement") }}
{{ value | replace(old="search", new="replacement", times=N) }}
```

**Parameters:**
- `old` (required) - String to search for
- `new` (required) - String to replace with
- `times` (optional) - Number of replacements to make (default: all occurrences)

**Examples:**

Replace all occurrences:
```jinja2
Input:  "foo bar foo baz"
Filter: {{ text | replace(old="foo", new="qux") }}
Output: "qux bar qux baz"
```

Replace first occurrence:
```jinja2
Input:  "foo bar foo baz"
Filter: {{ text | replace(old="foo", new="qux", times=1) }}
Output: "qux bar foo baz"
```

Replace first two occurrences:
```jinja2
Input:  "foo bar foo baz foo"
Filter: {{ text | replace(old="foo", new="qux", times=2) }}
Output: "qux bar qux baz foo"
```

Remove prefix:
```jinja2
Input:  "MTD_SEND_COMMAND"
Filter: {{ step.description | replace(old="MTD_", new="") }}
Output: "SEND_COMMAND"
```

**Use Cases:**
- Removing prefixes from command descriptions
- Normalizing text patterns
- Sanitizing content for output

### `replace_regex` Filter

Replaces text matching a regular expression pattern with a replacement string.

**Syntax:**
```jinja2
{{ value | replace_regex(old="pattern", new="replacement") }}
{{ value | replace_regex(old="pattern", new="replacement", times=N) }}
```

**Parameters:**
- `old` (required) - Regular expression pattern to match
- `new` (required) - Replacement string (supports capture groups: `$1`, `$2`, etc.)
- `times` (optional) - Number of replacements to make (default: all matches)

**Examples:**

Remove all digits:
```jinja2
Input:  "test123abc456"
Filter: {{ text | replace_regex(old="[0-9]+", new="") }}
Output: "testabc"
```

Replace digits with placeholder:
```jinja2
Input:  "test123abc456"
Filter: {{ text | replace_regex(old="[0-9]+", new="#") }}
Output: "test#abc#"
```

Replace first digit sequence:
```jinja2
Input:  "test123abc456"
Filter: {{ text | replace_regex(old="[0-9]+", new="NUM", times=1) }}
Output: "testNUMabc456"
```

Sanitize IDs (convert special chars to underscores):
```jinja2
Input:  "TC.Test-01: Basic"
Filter: {{ id | replace_regex(old="[^a-zA-Z0-9_]", new="_") }}
Output: "TC_Test_01__Basic"
```

Remove whitespace:
```jinja2
Input:  "hello   world"
Filter: {{ text | replace_regex(old="\s+", new=" ") }}
Output: "hello world"
```

**Use Cases:**
- Sanitizing test case IDs for use as anchors or file names
- Normalizing whitespace
- Extracting or transforming patterns
- Data cleaning and formatting

### Filter Chaining

Filters can be chained together:

```jinja2
{{ "  TC.Test-01  " | strip | replace_regex(old="[^a-zA-Z0-9_]", new="_") }}
Output: "TC_Test_01"

{{ description | strip | replace(old="MTD_", new="") }}
```

## Project Structure

```
.
├── data/                           # Example data files
│   ├── container/                  # Container-level files
│   │   ├── schema.json            # Container schema
│   │   ├── template.j2            # Container template (Markdown)
│   │   └── data.yml               # Container data
│   ├── verification_methods/       # Verification method definitions
│   │   ├── test/                  # Test verification method
│   │   │   ├── schema.json        # Test case schema
│   │   │   └── template.j2        # Test case template
│   │   ├── analysis/              # Analysis verification method
│   │   ├── demonstration/         # Demonstration verification method
│   │   ├── inspection/            # Inspection verification method
│   │   └── requirement_aggregation_template.j2
│   └── test_case/                 # Example test case files
│       ├── gsma_4.4.2.2_TC.yml    # Test case example
│       ├── gsma_4.4.2.3_TC.yml
│       └── ...
├── src/
│   └── main.rs                    # Single-file CLI implementation
├── tests/
│   └── e2e.rs                     # End-to-end tests
├── Cargo.toml                     # Rust dependencies
└── README.md                      # This file
```

## Example Test Cases

The repository includes example test cases in the `data/test_case/` directory:

### Test Verification Method Examples

- **[gsma_4.4.2.2_TC.yml](data/test_case/gsma_4.4.2.2_TC.yml)** - Test case demonstrating eUICC metadata update operations with multiple test sequences
- **[gsma_4.4.2.3_TC.yml](data/test_case/gsma_4.4.2.3_TC.yml)** - Additional test case example
- **[filter_test_01_TC.yml](data/test_case/filter_test_01_TC.yml)** - Demonstrates custom filter usage (strip, replace, replace_regex)

### Other Verification Methods

- **[gsma_4.4.2.4_AN.yml](data/test_case/gsma_4.4.2.4_AN.yml)** - Analysis verification method example
- **[gsma_4.4.2.5_DM.yml](data/test_case/gsma_4.4.2.5_DM.yml)** - Demonstration verification method example
- **[gsma_4.4.2.6_IN.yml](data/test_case/gsma_4.4.2.6_IN.yml)** - Inspection verification method example

Each test case file includes:
- Test metadata (requirement, item, tc, id)
- Test description
- General and sequence-specific initial conditions
- Test sequences with multiple steps
- Expected results for each step

## Development

### Running Tests

```bash
# Run all tests
make test

# Run unit tests only
cargo test --lib

# Run E2E tests only
cargo test --test e2e
```

### Linting

```bash
# Run formatting check and clippy
make lint

# Auto-fix formatting
cargo fmt
```

### Building

```bash
# Debug build
cargo build

# Release build
cargo build --release
```

### Docker

```bash
# Build Docker image
make docker-build
```

## CI/CD Verification

The project includes scripts to verify CI pipeline status on both GitHub Actions and GitLab CI.

### GitHub Actions Verification

To verify the GitHub Actions CI pipeline status for a specific branch:

```bash
# Verify the unmodified_push_2026-03-12 branch
make verify-github-actions

# Or run the script directly with a custom branch
./verify-github-actions.sh <branch-name>
```

The script checks:
- ✅ Build, lint (fmt-check + clippy)
- ✅ Unit tests
- ✅ E2E tests (markdown and asciidoc for both datasets)
- ✅ Docker build
- ✅ Coverage job

For detailed information, see [GITHUB_ACTIONS_VERIFICATION.md](GITHUB_ACTIONS_VERIFICATION.md).

### GitLab CI Verification

To check the GitLab CI pipeline status:

```bash
# Check GitLab pipeline status
make check-gitlab-pipeline

# Or run the script directly
./check-pipeline-status.sh
```

For detailed information, see [PIPELINE_VERIFICATION.md](PIPELINE_VERIFICATION.md).

## Requirements Management

The project integrates with [StrictDoc](https://strictdoc.readthedocs.io/) for requirements management and traceability. StrictDoc enables you to author, view, and manage requirements in a structured format with full traceability.

### Integration Overview

- Requirements are stored in `.sdoc` files using StrictDoc's markup language
- The `make test` command now includes requirements syntax validation via `strictdoc check`
- Requirements can be viewed in a web interface or exported to various formats

For detailed information on working with requirements, see [docs/STRICTDOC_GUIDE.md](docs/STRICTDOC_GUIDE.md).

### Quick Start Commands

**View Requirements in Web Browser:**
```bash
make strictdoc-server
```
Opens a local web server to browse and navigate requirements documentation.

**Export Requirements to HTML:**
```bash
make strictdoc-export
```
Generates static HTML documentation from requirements files.

**Validate Requirements Syntax:**
```bash
make strictdoc-validate
```
Checks all `.sdoc` files for syntax errors and structural issues.

## Tech Stack

- **Language**: Rust 2021 edition
- **CLI Parser**: [clap](https://docs.rs/clap/) with derive features
- **Templating**: [Tera](https://tera.netlify.app/) (Jinja2-like syntax)
- **Validation**: [jsonschema](https://docs.rs/jsonschema/) for JSON Schema validation
- **Serialization**: serde_json, serde_yaml
- **Testing**: cargo test + [insta](https://docs.rs/insta/) for snapshot testing

## License

For open source projects, specify the license here.
