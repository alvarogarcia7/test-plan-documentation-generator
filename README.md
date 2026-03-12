# Test Plan Documentation Generator

A Rust CLI tool that generates test plan documentation from JSON schemas, Jinja2 templates, and YAML data files. It validates data against schemas and renders customizable Markdown or AsciiDoc output for test documentation.

## Overview

The Test Plan Documentation Generator is designed to streamline the creation of test documentation by:

- **Validating** test data against JSON schemas to ensure consistency
- **Rendering** templates using the Tera templating engine (Jinja2-like syntax)
- **Aggregating** test results and requirements into comprehensive reports
- **Supporting** multiple verification methods (test, analysis, demonstration, inspection)
- **Generating** both Markdown and AsciiDoc output formats

The tool processes container-level data (test plan metadata) and multiple test case files, validating each against their respective schemas before generating the final documentation.

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

### Basic Usage Example

```bash
./target/release/tpdg \
  --output ./test_plan.md \
  --container ./data/container/schema.json \
             ./data/container/template.j2 \
             ./data/container/data.yml \
  --test-case ./data/verification_methods \
              ./data/test_case/test1.yml \
              ./data/test_case/test2.yml
```

This command:
1. Validates `data.yml` against `schema.json`
2. Validates each test case file against its type-specific schema
3. Renders test cases using their type-specific templates
4. Renders the final output using the container template
5. Writes the result to `test_plan.md`

### Example with AsciiDoc Format

```bash
./target/release/tpdg \
  --format asciidoc \
  --output ./test_plan.adoc \
  --container ./data/container/schema.json \
             ./data/container/template_asciidoc.adoc \
             ./data/container/data.yml \
  --test-case ./data/verification_methods \
              ./data/test_case/test1.yml \
              ./data/test_case/test2.yml
```

### Output to stdout

Omit the `--output` flag to print results to stdout:

```bash
./target/release/tpdg \
  --container ./data/container/schema.json \
             ./data/container/template.j2 \
             ./data/container/data.yml \
  --test-case ./data/verification_methods \
              ./data/test_case/test1.yml
```

## CLI Reference

### Synopsis

```
tpdg [OPTIONS] --container <FILES> --test-case <FILES>
```

### Options

#### `-o, --output <FILE>`

Specifies the output file path. If not provided, output is written to stdout.

**Example:**
```bash
--output ./report.md
-o ./docs/test_plan.adoc
```

#### `--container <SCHEMA> <TEMPLATE> <DATA>`

**Required.** Specifies the container-level schema, template, and data file (in that order).

**Arguments:**
- `SCHEMA` - JSON schema file for validating the container data
- `TEMPLATE` - Tera template file (`.j2` for Markdown, `.adoc` for AsciiDoc)
- `DATA` - YAML data file containing test plan metadata

**Example:**
```bash
--container ./schemas/container.json \
            ./templates/container.j2 \
            ./data/test_plan_data.yml
```

#### `--test-case <DIR> <FILE> [FILES...]`

**Required.** Specifies the verification methods directory followed by one or more test case data files.

**Arguments:**
- `DIR` - Directory containing verification method subdirectories (test, analysis, demonstration, inspection)
- `FILE` - One or more YAML test case data files

Each test case file must have a `type` field that corresponds to a subdirectory under the verification methods directory.

**Example:**
```bash
--test-case ./verification_methods \
            ./test_cases/tc001.yml \
            ./test_cases/tc002.yml \
            ./test_cases/tc003.yml
```

#### `--format <FORMAT>`

Specifies the output format. Accepted values: `markdown`, `asciidoc`

**Default:** `markdown`

**Example:**
```bash
--format markdown
--format asciidoc
```

### Exit Codes

- `0` - Success
- `1` - Usage error (missing or invalid arguments)
- `2` - File not found error
- `3` - Validation error (schema validation failed)

## Template System Overview

The tool uses the [Tera](https://tera.netlify.app/) templating engine, which provides Jinja2-like syntax.

### Template Variables

#### Container Template Variables

Container templates receive:

- All fields from the container data YAML file as top-level variables
- `test_cases_md` - Rendered markdown/AsciiDoc from all test cases (string)
- `test_cases_path` - Path to temporary file containing rendered test cases (string)
- `requirements_summary_md` or `requirements_summary_adoc` - Rendered requirements aggregation (if template exists)

**Example container template:**
```jinja2
# {{ title }}

Date: {{ date }}
Version: {{ version }}

## Test Cases

{{ test_cases_md }}

## Requirements Summary

{{ requirements_summary_md }}
```

#### Test Case Template Variables

Test case templates receive:

- All fields from the test case YAML file as top-level variables
- `data` - Complete test case data structure

**Example test case template:**
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

## Tech Stack

- **Language**: Rust 2021 edition
- **CLI Parser**: [clap](https://docs.rs/clap/) with derive features
- **Templating**: [Tera](https://tera.netlify.app/) (Jinja2-like syntax)
- **Validation**: [jsonschema](https://docs.rs/jsonschema/) for JSON Schema validation
- **Serialization**: serde_json, serde_yaml
- **Testing**: cargo test + [insta](https://docs.rs/insta/) for snapshot testing

## License

For open source projects, specify the license here.
