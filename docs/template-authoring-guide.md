# Template Authoring Guide

This guide provides comprehensive documentation for authoring templates in the Test Plan Documentation Generator, covering all verification methods (test, analysis, demonstration, inspection) and advanced template syntax.

## Table of Contents

1. [Overview](#overview)
2. [Template Engine (Tera)](#template-engine-tera)
3. [Verification Method Schemas](#verification-method-schemas)
4. [Template Variables & Context](#template-variables--context)
5. [Template Syntax Reference](#template-syntax-reference)
6. [Custom Filters](#custom-filters)
7. [Container Templates](#container-templates)
8. [Verification Method Templates](#verification-method-templates)
9. [Best Practices](#best-practices)
10. [Common Patterns](#common-patterns)

## Overview

The Test Plan Documentation Generator uses the **Tera** templating engine, which provides Jinja2-like syntax for rendering test documentation. Templates are used at two levels:

- **Verification Method Templates**: Render individual test cases based on their type (test, analysis, demonstration, inspection)
- **Container Templates**: Aggregate all rendered test cases into a final document with metadata

### Template Processing Flow

```
YAML Data Files → Schema Validation → Template Rendering → Final Output
     ↓                    ↓                    ↓              ↓
Test Cases          verification          Type-specific    Container
Container Data      schemas               templates        template
```

## Template Engine (Tera)

Tera is a template engine inspired by Jinja2 and Django templates. It supports:

- Variable interpolation
- Control structures (if, for, etc.)
- Filters for data transformation
- Template inheritance (not used in this project)
- Whitespace control
- Comments

### File Extensions

- **Markdown templates**: `.j2`
- **AsciiDoc templates**: `_asciidoc.adoc` or `.adoc`

The `--format` CLI flag determines which template files are loaded for verification methods.

## Verification Method Schemas

Each verification method has a specific schema that defines the structure of test case data.

### Test Schema (`test/schema.json`)

Used for executable test cases with sequences and steps.

**Required Fields:**
- `type`: Must be "test"
- `requirement`: Requirement identifier (e.g., "XXX100")
- `item`: Integer item number
- `tc`: Integer test case number
- `id`: Test case identifier string
- `description`: Test case description
- `general_initial_conditions`: Object with arrays of condition strings
- `initial_conditions`: Object with arrays of condition strings
- `test_sequences`: Array of test sequence objects

**Test Sequence Structure:**
```yaml
test_sequences:
  - id: 1                          # Integer sequence ID
    name: "Sequence Name"           # String name
    description: "Description"      # Multi-line description
    initial_conditions:             # Conditions specific to sequence
      entity_name:
        - "Condition 1"
    steps:                          # Array of test steps
      - step: 1                     # Step number
        manual: true                # Optional: boolean (default false)
        description: "Action"       # Step description
        command: "ssh"              # Command to execute
        expected:                   # Expected results
          success: true             # Optional: boolean (default true)
          result: "SW=0x9000"       # Expected result string
          output: "Success message" # Expected output
```

### Analysis Schema (`analysis/schema.json`)

Used for analytical verification through calculations and modeling.

**Required Fields:**
- `type`: Must be "analysis"
- `requirement`: Requirement identifier
- `id`: Analysis case identifier
- `description`: Analysis description
- `method`: Analysis method description
- `calculations`: Array of calculation objects
- `models`: Array of model objects
- `acceptance_criteria`: Array of criteria strings

**Calculation Structure:**
```yaml
calculations:
  - name: "Calculation name"
    formula: "Mathematical formula"
    parameters:                    # Optional
      param_name: "Description"
```

**Model Structure:**
```yaml
models:
  - name: "Model name"
    type: "Model type (e.g., 'Statistical')"
    description: "Model description"
```

### Demonstration Schema (`demonstration/schema.json`)

Used for live demonstration verification.

**Required Fields:**
- `type`: Must be "demonstration"
- `requirement`: Requirement identifier
- `id`: Demonstration identifier
- `description`: Demonstration description
- `procedure`: Procedure description (can be multi-line)
- `observations`: Array of observation strings
- `acceptance_criteria`: Array of criteria strings

### Inspection Schema (`inspection/schema.json`)

Used for manual inspection and review verification.

**Required Fields:**
- `type`: Must be "inspection"
- `requirement`: Requirement identifier
- `id`: Inspection identifier
- `description`: Inspection description
- `inspection_method`: Method description
- `checklist`: Array of checklist item strings
- `acceptance_criteria`: Array of criteria strings

## Template Variables & Context

### Test Case Template Context

All test case templates receive:

1. **Top-level field access**: All YAML fields are available as variables
   ```jinja2
   {{ type }}
   {{ requirement }}
   {{ id }}
   {{ description }}
   ```

2. **The `data` variable**: Complete data structure
   ```jinja2
   {{ data.requirement }}
   {{ data.test_sequences }}
   ```

### Container Template Context

Container templates receive:

1. **Container YAML fields**: All fields from container data file
   ```jinja2
   {{ date }}
   {{ product }}
   {{ description }}
   ```

2. **Rendered test cases**:
   - `test_cases_md`: String containing all rendered test cases
   - `test_cases_path`: Path to temporary file with rendered content

3. **Requirements summary** (if requirement aggregation template exists):
   - `requirements_summary_md`: For markdown format
   - `requirements_summary_adoc`: For AsciiDoc format

## Template Syntax Reference

### Variable Interpolation

Basic variable output:
```jinja2
{{ variable_name }}
```

Access object properties:
```jinja2
{{ object.property }}
{{ nested.object.property }}
```

Access array elements:
```jinja2
{{ array[0] }}
{{ array[index] }}
```

### Control Structures

#### Conditionals

```jinja2
{% if condition %}
  Content when true
{% endif %}

{% if condition %}
  True content
{% else %}
  False content
{% endif %}

{% if condition1 %}
  Content 1
{% elif condition2 %}
  Content 2
{% else %}
  Default content
{% endif %}
```

**Conditions:**
```jinja2
{% if variable %}                    {# Truthy check #}
{% if variable == "value" %}         {# Equality #}
{% if variable != "value" %}         {# Inequality #}
{% if num > 5 %}                     {# Comparison #}
{% if array | length > 0 %}          {# Using filters #}
{% if var1 and var2 %}               {# Logical AND #}
{% if var1 or var2 %}                {# Logical OR #}
{% if not var %}                     {# Negation #}
```

#### Loops

Basic loop:
```jinja2
{% for item in items %}
  {{ item }}
{% endfor %}
```

Loop with index:
```jinja2
{% for item in items %}
  {{ loop.index }}: {{ item }}
{% endfor %}
```

Loop over object/mapping:
```jinja2
{% for key, value in object %}
  {{ key }}: {{ value }}
{% endfor %}
```

**Loop variables:**
- `loop.index`: Current iteration (1-indexed)
- `loop.index0`: Current iteration (0-indexed)
- `loop.first`: True on first iteration
- `loop.last`: True on last iteration
- `loop.length`: Total number of items

### Whitespace Control

Control whitespace with `-`:

```jinja2
{%- if condition %}    {# Strip whitespace before #}
{% if condition -%}    {# Strip whitespace after #}
{%- if condition -%}   {# Strip both #}

{{- variable }}        {# Strip whitespace before output #}
{{ variable -}}        {# Strip whitespace after output #}
```

Example usage:
```jinja2
{% for item in items -%}
| {{ item }} |
{% endfor %}
```

### Comments

```jinja2
{# This is a comment and won't appear in output #}

{# 
Multi-line comment
Also won't appear
#}
```

### Set Variables

```jinja2
{% set variable = "value" %}
{% set count = items | length %}
{% set result = value1 + value2 %}
```

## Custom Filters

The generator provides three custom filters in addition to Tera's built-in filters.

### `strip` Filter

Removes leading and trailing whitespace.

**Syntax:**
```jinja2
{{ value | strip }}
```

**Example:**
```jinja2
Input:  "  Test Case ID  "
Output: "Test Case ID"

Template: {{ id | strip }}
```

### `replace` Filter

String replacement with optional limit.

**Syntax:**
```jinja2
{{ value | replace(old="search", new="replacement") }}
{{ value | replace(old="search", new="replacement", times=N) }}
```

**Examples:**
```jinja2
{# Replace all occurrences #}
{{ "foo bar foo" | replace(old="foo", new="baz") }}
{# Output: "baz bar baz" #}

{# Replace first occurrence only #}
{{ "foo bar foo" | replace(old="foo", new="baz", times=1) }}
{# Output: "baz bar foo" #}

{# Remove prefix #}
{{ "MTD_SEND_COMMAND" | replace(old="MTD_", new="") }}
{# Output: "SEND_COMMAND" #}
```

### `replace_regex` Filter

Regular expression replacement with optional limit.

**Syntax:**
```jinja2
{{ value | replace_regex(old="pattern", new="replacement") }}
{{ value | replace_regex(old="pattern", new="replacement", times=N) }}
```

**Examples:**
```jinja2
{# Sanitize ID for use as anchor #}
{{ "TC.Test-01: Basic" | replace_regex(old="[^a-zA-Z0-9_]", new="_") }}
{# Output: "TC_Test_01__Basic" #}

{# Normalize whitespace #}
{{ "hello   world" | replace_regex(old="\\s+", new=" ") }}
{# Output: "hello world" #}

{# Remove all digits #}
{{ "test123abc456" | replace_regex(old="[0-9]+", new="") }}
{# Output: "testabc" #}

{# Replace first match only #}
{{ "test123abc456" | replace_regex(old="[0-9]+", new="NUM", times=1) }}
{# Output: "testNUMabc456" #}
```

### Built-in Tera Filters

Common built-in filters:

```jinja2
{{ text | upper }}              {# UPPERCASE #}
{{ text | lower }}              {# lowercase #}
{{ text | capitalize }}         {# Capitalize first letter #}
{{ text | title }}              {# Title Case #}

{{ array | length }}            {# Get length #}
{{ array | first }}             {# First element #}
{{ array | last }}              {# Last element #}
{{ array | join(sep=", ") }}    {# Join with separator #}

{{ array | filter(attribute="key", value="val") }}  {# Filter array #}
{{ array | reverse }}           {# Reverse array #}
{{ array | sort }}              {# Sort array #}

{{ text | truncate(length=10) }} {# Truncate string #}
{{ text | wordcount }}          {# Count words #}

{{ value | default(value="N/A") }} {# Default if undefined #}
```

### Filter Chaining

Filters can be chained for complex transformations:

```jinja2
{{ "  TC.Test-01  " | strip | replace_regex(old="[^a-zA-Z0-9_]", new="_") }}
{# Output: "TC_Test_01" #}

{{ step.description | replace(old="MTD_", new="") | strip | lower }}

{{ id | strip | replace(old=" ", new="-") | lower }}
```

## Container Templates

Container templates aggregate all rendered test cases and provide document structure.

### Basic Container Template Structure

```jinja2
# Document Title

Date: {{ date }}
Product: {{ product }}

## Overview

{{ description }}

## Test Cases

{{ test_cases_md }}

## Requirements Summary

{% if requirements_summary_md %}
{{ requirements_summary_md }}
{% endif %}

---
© Copyright Notice
```

### Accessing Container Data

```jinja2
{# All fields from container YAML are accessible #}
Title: {{ title }}
Version: {{ version }}
Date: {{ date }}
Author: {{ author }}
```

### Including Rendered Test Cases

```jinja2
{# Insert all rendered test cases #}
{{ test_cases_md }}

{# Or read from file path if needed #}
{# (though direct inclusion is recommended) #}
Path: {{ test_cases_path }}
```

## Verification Method Templates

### Test Template Example

```jinja2
## Test Case: {{ id | strip }}

**Requirement**: {{ requirement | strip }}
**Item**: {{ item }}
**TC**: {{ tc }}

### Description

{{ description | strip }}

### General Initial Conditions

{% for entity, conditions in general_initial_conditions %}
**{{ entity }}**:
{% for condition in conditions %}
- {{ condition | strip }}
{% endfor %}
{% endfor %}

{% for sequence in test_sequences %}
### Test Sequence {{ sequence.id }}: {{ sequence.name | strip }}

{{ sequence.description | strip }}

**Initial Conditions**:
{% for entity, conditions in sequence.initial_conditions %}
**{{ entity }}**:
{% for condition in conditions %}
- {{ condition | strip }}
{% endfor %}
{% endfor %}

**Test Steps**:

| Step | Manual | Description | Expected Result | Expected Output |
|------|--------|-------------|-----------------|-----------------|
{% for step in sequence.steps -%}
| {{ step.step }} | {% if step.manual %}Yes{% else %}No{% endif %} | {{ step.description | replace(old="MTD_", new="") | strip }} | {{ step.expected.result | strip }} | {{ step.expected.output | strip }} |
{% endfor %}

{% endfor %}
```

### Analysis Template Example

```jinja2
## Analysis: {{ id | strip }}

**Requirement**: {{ requirement | strip }}

### Description

{{ description | strip }}

### Method

{{ method | strip }}

### Calculations

{% for calc in calculations %}
#### {{ calc.name | strip }}

**Formula**: `{{ calc.formula }}`

{% if calc.parameters %}
**Parameters**:
{% for param, desc in calc.parameters %}
- **{{ param }}**: {{ desc }}
{% endfor %}
{% endif %}
{% endfor %}

### Models

| Model Name | Type | Description |
|------------|------|-------------|
{% for model in models -%}
| {{ model.name | strip }} | {{ model.type | strip }} | {{ model.description | strip }} |
{% endfor %}

### Acceptance Criteria

{% for criterion in acceptance_criteria %}
- {{ criterion | strip }}
{% endfor %}
```

### Demonstration Template Example

```jinja2
## Demonstration: {{ id | strip }}

**Requirement**: {{ requirement | strip }}

### Description

{{ description | strip }}

### Procedure

{{ procedure | strip }}

### Observations

{% for observation in observations %}
- {{ observation | strip }}
{% endfor %}

### Acceptance Criteria

{% for criterion in acceptance_criteria %}
- {{ criterion | strip }}
{% endfor %}
```

### Inspection Template Example

```jinja2
## Inspection: {{ id | strip }}

**Requirement**: {{ requirement | strip }}

### Description

{{ description | strip }}

### Inspection Method

{{ inspection_method | strip }}

### Checklist

{% for item in checklist %}
- [ ] {{ item | strip }}
{% endfor %}

### Acceptance Criteria

{% for criterion in acceptance_criteria %}
- {{ criterion | strip }}
{% endfor %}
```

## Best Practices

### 1. Always Use `strip` Filter

YAML data often contains trailing whitespace. Always use `strip` on text fields:

```jinja2
{{ description | strip }}
{{ id | strip }}
{{ name | strip }}
```

### 2. Handle Optional Fields

Use conditionals to check for optional fields:

```jinja2
{% if step.manual %}
Manual: Yes
{% endif %}

{% if calc.parameters %}
Parameters: {{ calc.parameters }}
{% endif %}
```

### 3. Whitespace Control in Tables

Use `-` to control whitespace in markdown tables:

```jinja2
| Column1 | Column2 |
|---------|---------|
{% for item in items -%}
| {{ item.col1 }} | {{ item.col2 }} |
{% endfor %}
```

### 4. Sanitize IDs for Anchors

Create safe HTML/Markdown anchors:

```jinja2
{# Create anchor from ID #}
<a id="{{ id | replace_regex(old='[^a-zA-Z0-9_-]', new='_') }}"></a>

{# Link to test case #}
[See TC {{ tc }}](#{{ id | strip | lower | replace(old=' ', new='-') }})
```

### 5. Default Values

Provide defaults for potentially undefined values:

```jinja2
{{ variable | default(value="N/A") }}

{% if variable %}
  {{ variable }}
{% else %}
  Not specified
{% endif %}
```

### 6. Multi-line Content

For multi-line YAML content (using `|` or `>`), use strip and normalize:

```jinja2
{{ description | replace_regex(old="\\s+", new=" ") | strip }}
```

### 7. Consistent Formatting

Keep formatting consistent across verification method templates:

```jinja2
{# Always use ## for test case titles #}
## Test Case: {{ id | strip }}

{# Always use ### for major sections #}
### Description

{# Always use #### for subsections #}
#### Calculation Details
```

## Common Patterns

### Pattern 1: Conditional Table Rows

```jinja2
| Step | Action | Expected |
|------|--------|----------|
{% for step in steps -%}
{% if step.manual -%}
| {{ step.step }} | **[MANUAL]** {{ step.description }} | {{ step.expected }} |
{% else -%}
| {{ step.step }} | {{ step.description }} | {{ step.expected }} |
{% endif -%}
{% endfor %}
```

### Pattern 2: Nested Iteration

```jinja2
{% for sequence in test_sequences %}
## Sequence {{ sequence.id }}

{% for step in sequence.steps %}
### Step {{ step.step }}
{{ step.description }}
{% endfor %}
{% endfor %}
```

### Pattern 3: Grouped Content

```jinja2
{% for entity, conditions in initial_conditions %}
**{{ entity }}**:
{% for condition in conditions %}
- {{ condition | strip }}
{% endfor %}

{% endfor %}
```

### Pattern 4: Index-based Formatting

```jinja2
{% for item in items %}
{% if loop.first %}
## First Item: {{ item }}
{% elif loop.last %}
## Last Item: {{ item }}
{% else %}
## Item {{ loop.index }}: {{ item }}
{% endif %}
{% endfor %}
```

### Pattern 5: Inline Conditionals

```jinja2
| Manual | Success |
|--------|---------|
| {% if step.manual %}Yes{% else %}No{% endif %} | {% if step.expected.success %}✓{% else %}✗{% endif %} |
```

### Pattern 6: Filter Combinations for Data Cleaning

```jinja2
{# Remove prefix, normalize whitespace, and trim #}
{{ step.description | replace(old="MTD_", new="") | replace_regex(old="\\s+", new=" ") | strip }}

{# Create safe filename from test ID #}
{{ id | strip | lower | replace(old=" ", new="_") | replace_regex(old="[^a-zA-Z0-9_]", new="_") }}
```

### Pattern 7: Conditional Sections

```jinja2
{% if test_sequences | length > 0 %}
## Test Sequences

{% for seq in test_sequences %}
### {{ seq.name }}
{{ seq.description }}
{% endfor %}
{% else %}
_No test sequences defined._
{% endif %}
```

### Pattern 8: Parameter Lists

```jinja2
{% if calc.parameters %}
**Parameters**:
{% for param_name, param_desc in calc.parameters %}
- `{{ param_name }}`: {{ param_desc }}
{% endfor %}
{% endif %}
```

## Advanced Examples

### Multi-format Output Support

```jinja2
{# Detect format from context or use conditional templates #}
{% if format == "markdown" %}
## {{ title }}
{% elif format == "asciidoc" %}
== {{ title }}
{% endif %}
```

### Dynamic ID Generation

```jinja2
{# Create unique anchor ID #}
{% set anchor_id = requirement | strip | lower | replace(old=".", new="_") ~ "_" ~ item ~ "_" ~ tc %}

<a id="{{ anchor_id }}"></a>

{# Reference it later #}
See [requirement {{ requirement }}](#{{ anchor_id }})
```

### Complex Table Generation

```jinja2
| Req | Item | TC | Status | Pass Rate |
|-----|------|----|----|-----------|
{% for result in test_results -%}
{% set total = result.total_steps %}
{% set passed = result.passed_steps %}
{% set rate = (passed / total * 100) | round %}
| {{ result.requirement }} | {{ result.item }} | {{ result.tc }} | {% if result.overall_pass %}PASS{% else %}FAIL{% endif %} | {{ rate }}% |
{% endfor %}
```

### Hierarchical Numbering

```jinja2
{% for seq in test_sequences %}
{{ requirement }}.{{ item }}.{{ tc }}.{{ seq.id }} {{ seq.name }}

{% for step in seq.steps %}
  {{ requirement }}.{{ item }}.{{ tc }}.{{ seq.id }}.{{ step.step }} {{ step.description | strip }}
{% endfor %}
{% endfor %}
```

## Troubleshooting

### Issue: Extra Whitespace in Output

**Solution**: Use whitespace control `-`

```jinja2
{# Instead of #}
{% for item in items %}
{{ item }}
{% endfor %}

{# Use #}
{% for item in items -%}
{{ item }}
{% endfor %}
```

### Issue: Undefined Variable Error

**Solution**: Use default filter or conditional check

```jinja2
{{ variable | default(value="") }}

{% if variable %}
{{ variable }}
{% endif %}
```

### Issue: Special Characters Breaking Format

**Solution**: Use `replace_regex` to sanitize

```jinja2
{{ text | replace_regex(old="[^a-zA-Z0-9 ]", new="") }}
```

### Issue: Complex Nested Data Access

**Solution**: Use set to simplify

```jinja2
{% for sequence in test_sequences %}
{% set seq_id = sequence.id %}
{% set seq_name = sequence.name | strip %}

## Sequence {{ seq_id }}: {{ seq_name }}
{% endfor %}
```

## Summary

This guide covered:

- Template engine basics (Tera/Jinja2 syntax)
- All four verification method schemas (test, analysis, demonstration, inspection)
- Template variable access and context
- Control structures, loops, and conditionals
- Custom filters (strip, replace, replace_regex)
- Container and verification method template patterns
- Best practices and common patterns
- Troubleshooting tips

For filter details and examples, see [filters-reference.md](filters-reference.md).

For schema specifications, see [schema-reference.md](schema-reference.md).
