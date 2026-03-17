# Template System Documentation

This document describes the template injection patterns, best practices, and validation requirements for the tpdg (Test Plan Documentation Generator) template system.

## Table of Contents

1. [Overview](#overview)
2. [Container Template Injection](#container-template-injection)
3. [Test Results Template Injection](#test-results-template-injection)
4. [Include File Function](#include-file-function)
5. [Requirement Aggregation Templates](#requirement-aggregation-templates)
6. [Validation Requirements](#validation-requirements)
7. [Custom Filters](#custom-filters)

## Overview

The tpdg tool uses the Tera templating engine (Jinja2-like syntax) to render test documentation. The system supports two output formats:
- **Markdown**: Uses `.j2` template file extension
- **AsciiDoc**: Uses `_asciidoc.adoc` template file extension

## Container Template Injection

Container templates serve as the main document wrapper that combines metadata with rendered test case content.

### Injected Variables

Container templates automatically receive the following injected variables:

#### `test_cases_md`
- **Type**: String
- **Description**: Contains the fully rendered markdown/asciidoc content from all test case files
- **Source**: Concatenation of all rendered test case templates
- **Usage**: Inject this variable where you want the test cases to appear in the final document

**Example (Markdown)**:
```jinja2
# Test Plan

{{ date }}

## Test Cases

{{ test_cases_md }}

(C) GSMA 2024. All rights reserved.
```

**Example (AsciiDoc)**:
```asciidoc
= {{ title }}
:doctype: book

== Test Execution Overview

{{ test_cases_md }}

<<<

== Final Verification Verdict
```

#### `test_cases_path`
- **Type**: String (file path)
- **Description**: Absolute path to the temporary file containing the rendered test cases
- **Source**: Generated in system temp directory (e.g., `/tmp/tpdg-<timestamp>/output.md`)
- **Usage**: Reference if you need to know the physical location of the intermediate output

#### Container Data Variables

All fields from your container YAML/JSON data file are automatically injected as template variables:

**Example Container Data** (`container_data.yml`):
```yaml
date: "2024-03-15"
product: "eUICC Platform"
description: "Remote SIM Provisioning Test Plan"
```

**Available in Template**:
```jinja2
Date: {{ date }}
Product: {{ product }}
Description: {{ description }}
```

### Container Template Structure

Container templates typically follow this pattern:

```jinja2
{# Header and metadata #}
Title: {{ title }}
Date: {{ date }}

{# Main content injection point #}
{{ test_cases_md }}

{# Footer #}
(C) Company {{ year }}
```

## Test Results Template Injection

Test results container templates (used with `--format asciidoc`) have additional injected variables for aggregating requirement verification data.

### Injected Variables

#### `requirements_summary_adoc` / `requirements_summary_md`
- **Type**: String (rendered template content)
- **Description**: Aggregated requirements summary rendered from the requirement aggregation template
- **Conditional**: Only injected if `requirement_aggregation_template.adoc` (or `.j2`) exists in the verification methods directory
- **Variable Name**: 
  - `requirements_summary_adoc` for AsciiDoc format
  - `requirements_summary_md` for Markdown format
  - `requirements_summary` as generic fallback

**Example Usage**:
```asciidoc
== Requirements Summary

{{ requirements_summary_adoc }}
```

#### `test_results`
- **Type**: Array of objects
- **Description**: Array of test result entries from the container data file
- **Source**: Parsed from the `test_results` field in the container YAML/JSON
- **Usage**: Available for custom template logic and passed to requirement aggregation templates

**Example Structure**:
```yaml
test_results:
  - requirement: "XXX100"
    item: 1
    tc: 4
    test_case_id: "TC-001"
    description: "Test description"
    overall_pass: true
    sequences: [...]
```

### Test Results Container Template Example

```asciidoc
= {{ title }}

== Test Execution Overview

Total Test Cases: {{ test_results | length }}
Passed: {{ test_results | filter(attribute="overall_pass", value=true) | length }}

== Detailed Test Results

{{ test_cases_md }}

== Requirements Summary

{{ requirements_summary_adoc }}
```

## Include File Function

The `include_file` function enables file inclusion with variable interpolation, supporting modular template composition.

### Function Signature

```jinja2
{{ include_file(path="relative/path/to/file.ext") }}
```

### Parameters

- **path** (required): Relative file path from the template's directory
  - Type: String
  - Must be within or relative to the template's base directory
  - Security: Path traversal is prevented (cannot escape base directory)

### Features

1. **Variable Interpolation**: Variables from the current template context are accessible in the included file
2. **Nested Rendering**: Included files are rendered as Tera templates with full access to filters and functions
3. **Context Inheritance**: The included file has access to all variables defined in the parent context
4. **Security**: Enforces canonical path checking to prevent directory traversal attacks

### Usage Examples

#### Basic File Inclusion

**Main Template**:
```jinja2
# Test Report

{{ include_file(path="sections/header.md") }}

## Test Cases
{{ test_cases_md }}
```

**Included File** (`sections/header.md`):
```jinja2
**Date**: {{ date }}
**Project**: {{ project }}
```

**Rendered Output**:
```markdown
# Test Report

**Date**: 2024-03-15
**Project**: eUICC Testing

## Test Cases
...
```

#### Variable Interpolation

**Main Template**:
```jinja2
{{ include_file(path="user_info.txt") }}
```

**Included File** (`user_info.txt`):
```jinja2
Hello, {{ name }}!
User Age: {{ user.age }}
```

**Context**:
```json
{
  "name": "Alice",
  "user": {
    "age": 30
  }
}
```

**Rendered Output**:
```
Hello, Alice!
User Age: 30
```

#### Using Filters in Included Files

**Included File** (`formatted_id.txt`):
```jinja2
Sanitized ID: {{ id | replace_regex(old="[^a-zA-Z0-9_]", new="_") }}
```

### Security Considerations

- All file paths are resolved against the template's base directory
- Canonical path validation prevents access outside the base directory
- Attempting to access files outside the base directory results in an error
- Example of blocked path: `{{ include_file(path="../../../etc/passwd") }}`

### Error Handling

The function will fail with an error if:
- The `path` argument is missing
- The file does not exist
- The file path escapes the base directory
- The included file contains invalid template syntax

## Requirement Aggregation Templates

Requirement aggregation templates process test results to generate requirement-level summaries and verification status reports.

### Location and Naming

- **Directory**: Verification methods directory (e.g., `data/verification_methods/`)
- **Filename (AsciiDoc)**: `requirement_aggregation_template.adoc`
- **Filename (Markdown)**: `requirement_aggregation_template.j2`

### Available Context

Requirement aggregation templates have access to all container data variables, including:

- `test_results`: Array of test result entries
- All metadata from the container data file
- All standard Tera filters

### Structure and Pattern

The template typically generates three types of requirement summaries:

#### 1. Requirements with Detail

Lists all test cases grouped by requirement:

```jinja2
requirements_with_detail:
{%- set reqs = ["XXX100", "XXX200", "XXX300", "XXX400"] %}
{%- for req in reqs %}
{%- set filtered = test_results | filter(attribute="requirement", value=req) %}
{%- if filtered | length > 0 %}
  - requirement: {{ req }}
    items:
{%- for item in filtered %}
      - item: {% if item.item %}{{ item.item }}{% else %}null{% endif %}
        tc: {% if item.tc %}{{ item.tc }}{% else %}null{% endif %}
        id: {{ item.test_case_id }}
        pass: {% if item.overall_pass %}true{% else %}false{% endif %}
{%- endfor %}
{%- endif %}
{%- endfor %}
```

#### 2. Status Per Requirement

Aggregates pass/fail status for each requirement:

```jinja2
status_per_requirement:
{%- for req in reqs %}
{%- set filtered = test_results | filter(attribute="requirement", value=req) %}
{%- if filtered | length > 0 %}
{%- set all_pass = filtered | filter(attribute="overall_pass", value=true) | length == filtered | length %}
  - requirement: {{ req }}
    pass: {% if all_pass %}true{% else %}false{% endif %}
{%- endif %}
{%- endfor %}
```

#### 3. Requirements by Status

Groups requirements into pass/fail categories:

```jinja2
requirements_by_status:
  pass:
{%- for req in reqs %}
{%- set filtered = test_results | filter(attribute="requirement", value=req) %}
{%- if filtered | length > 0 %}
{%- set all_pass = filtered | filter(attribute="overall_pass", value=true) | length == filtered | length %}
{%- if all_pass %}
    - {{ req }}
{%- endif %}
{%- endif %}
{%- endfor %}
  fail:
{%- for req in reqs %}
{%- set filtered = test_results | filter(attribute="requirement", value=req) %}
{%- if filtered | length > 0 %}
{%- set all_pass = filtered | filter(attribute="overall_pass", value=true) | length == filtered | length %}
{%- if not all_pass %}
    - {{ req }}
{%- endif %}
{%- endif %}
{%- endfor %}
```

### Customizing Requirements List

To customize which requirements are processed, modify the `reqs` variable:

```jinja2
{# Static list #}
{%- set reqs = ["REQ-001", "REQ-002", "REQ-003"] %}

{# Dynamic list from test_results #}
{%- set reqs = test_results | map(attribute="requirement") | unique | sort %}

{# Conditional list #}
{%- set reqs = ["REQ-A", "REQ-B"] if environment == "production" else ["REQ-X", "REQ-Y"] %}
```

### Output Format

The requirement aggregation template output is injected into the container template as:
- `requirements_summary_adoc` (for AsciiDoc format)
- `requirements_summary_md` (for Markdown format)

## Validation Requirements

### Container Template Validation

Container schemas must validate the structure of container data files.

#### Required Schema Fields

**Minimum Container Schema** (`schema.json`):
```json
{
  "$schema": "http://json-schema.org/draft-04/schema#",
  "type": "object",
  "properties": {
    "date": {"type": "string"},
    "product": {"type": "string"},
    "description": {"type": "string"}
  },
  "required": ["date", "product", "description"]
}
```

#### Test Results Container Schema

For test results containers, include the `test_results` array:

```json
{
  "$schema": "http://json-schema.org/draft-07/schema#",
  "type": "object",
  "properties": {
    "title": {"type": "string"},
    "project": {"type": "string"},
    "test_date": {"type": "string", "format": "date-time"},
    "test_results": {
      "type": "array",
      "items": {"type": "object"}
    },
    "metadata": {
      "type": "object",
      "properties": {
        "environment": {"type": "string"},
        "platform": {"type": "string"},
        "executor": {"type": "string"}
      },
      "required": ["environment", "platform", "executor"]
    }
  },
  "required": ["title", "project", "test_date", "test_results", "metadata"]
}
```

### Verification Schema Validation

The tool performs two-level validation for test results:

1. **Container Level**: Validates the overall container structure against `container_schema.json`
2. **Test Result Entry Level**: Validates individual `test_results` array entries against `verification_schema.json`

#### Verification Schema Location

- Must be in the same directory as the container schema
- Filename: `verification_schema.json`
- Applied to each entry in the `test_results` array

#### Example Verification Schema

```json
{
  "$schema": "http://json-schema.org/draft-07/schema#",
  "type": "object",
  "properties": {
    "test_case_id": {"type": "string"},
    "description": {"type": "string"},
    "overall_pass": {"type": "boolean"},
    "total_steps": {"type": "integer", "minimum": 0},
    "passed_steps": {"type": "integer", "minimum": 0},
    "failed_steps": {"type": "integer", "minimum": 0},
    "not_executed_steps": {"type": "integer", "minimum": 0},
    "requirement": {"type": "string"},
    "item": {"type": "integer"},
    "tc": {"type": "integer"}
  },
  "required": [
    "test_case_id",
    "description", 
    "overall_pass",
    "total_steps",
    "passed_steps",
    "failed_steps",
    "not_executed_steps"
  ]
}
```

### Test Case Schema Validation

Each test case type requires:

1. **Schema File**: `<type>/schema.json` in verification methods directory
2. **Template Files**: 
   - `<type>/template.j2` for Markdown
   - `<type>/template_asciidoc.adoc` for AsciiDoc

#### Required Schema Elements

All test case schemas must include:
- `type` field with appropriate enum constraint
- All fields referenced in the template
- Proper type definitions for arrays, objects, strings, integers, booleans

**Example Test Case Schema**:
```json
{
  "$schema": "http://json-schema.org/draft-04/schema#",
  "type": "object",
  "properties": {
    "type": {"type": "string", "enum": ["test"]},
    "requirement": {"type": "string"},
    "item": {"type": "integer"},
    "tc": {"type": "integer"},
    "id": {"type": "string"},
    "description": {"type": "string"}
  },
  "required": ["type", "requirement", "item", "tc", "id", "description"]
}
```

### Validation Workflow

1. **Container Validation**: Container data validated against container schema
2. **Test Results Entry Validation**: Each `test_results` entry validated against verification schema (if present)
3. **Test Case Validation**: Each test case file validated against its type-specific schema
4. **Template Rendering**: Only proceeds if all validations pass

### Error Handling

Validation errors include:
- File path of the data file
- Schema path used for validation
- Detailed error messages for each validation failure
- Exit code 3 for validation failures

## Custom Filters

The template system provides custom filters beyond standard Tera filters.

### replace

Replace all occurrences of a substring (or limit to N occurrences).

**Syntax**:
```jinja2
{{ string | replace(old="old_text", new="new_text") }}
{{ string | replace(old="old_text", new="new_text", times=2) }}
```

**Parameters**:
- `old` (required): Substring to find
- `new` (required): Replacement string
- `times` (optional): Maximum number of replacements

**Example**:
```jinja2
{{ "hello world world" | replace(old="world", new="universe") }}
{# Output: hello universe universe #}

{{ "test_test_test" | replace(old="test", new="X", times=2) }}
{# Output: X_X_test #}
```

### replace_regex

Replace using regular expression patterns.

**Syntax**:
```jinja2
{{ string | replace_regex(old="regex_pattern", new="replacement") }}
{{ string | replace_regex(old="regex_pattern", new="replacement", times=1) }}
```

**Parameters**:
- `old` (required): Regular expression pattern
- `new` (required): Replacement string
- `times` (optional): Maximum number of replacements

**Example**:
```jinja2
{{ "TC-001-X" | replace_regex(old="[^a-zA-Z0-9_]", new="_") }}
{# Output: TC_001_X #}

{{ id | replace_regex(old="[^a-zA-Z0-9_]", new="_") }}
{# Sanitizes ID for use as identifier #}
```

### strip

Trim whitespace from both ends of a string.

**Syntax**:
```jinja2
{{ string | strip }}
```

**Example**:
```jinja2
{{ "  hello world  " | strip }}
{# Output: hello world #}

{{ description | strip }}
{# Commonly used to clean up multiline descriptions #}
```

## Best Practices

### Template Organization

1. **Separation of Concerns**: Keep container templates, test case templates, and requirement aggregation templates separate
2. **Modular Design**: Use `include_file` for reusable sections (headers, footers, common tables)
3. **Format Consistency**: Maintain parallel template structures for Markdown and AsciiDoc formats

### Variable Naming

1. **Container Variables**: Use descriptive names in container data (e.g., `test_date`, `project`, `title`)
2. **Reserved Variables**: Avoid overriding `test_cases_md`, `test_cases_path`, `requirements_summary_adoc`
3. **Test Result Fields**: Use consistent field names across test results (`test_case_id`, `overall_pass`, `requirement`)

### Schema Design

1. **Explicit Types**: Always specify types for all fields
2. **Required Fields**: Mark all essential fields as required
3. **Constraints**: Use `minimum`, `enum`, `format` constraints where applicable
4. **Documentation**: Include `description` fields for clarity

### Error Prevention

1. **Null Handling**: Use conditional checks for optional fields:
   ```jinja2
   {% if item.value %}{{ item.value }}{% else %}null{% endif %}
   ```

2. **Array Safety**: Check array length before iteration:
   ```jinja2
   {%- if test_results | length > 0 %}
   {# Process results #}
   {%- endif %}
   ```

3. **Filter Chains**: Validate intermediate results:
   ```jinja2
   {%- set filtered = test_results | filter(attribute="req", value="XXX100") %}
   {%- if filtered | length > 0 %}
   {# Safe to process #}
   {%- endif %}
   ```

### Template Testing

1. **Sample Data**: Create representative sample data files for each test case type
2. **Edge Cases**: Test with empty arrays, missing optional fields, special characters
3. **Both Formats**: Verify both Markdown and AsciiDoc templates render correctly
4. **Validation**: Ensure all sample data passes schema validation

## Summary

The tpdg template system provides:
- **Container injection** of rendered test cases via `test_cases_md`
- **Requirement aggregation** through customizable templates
- **File inclusion** with variable interpolation via `include_file()`
- **Multi-level validation** (container, verification entries, test cases)
- **Custom filters** for string manipulation
- **Dual format support** (Markdown and AsciiDoc)

Following these patterns and practices ensures consistent, validated, and maintainable test documentation.
