# Filters Reference

Complete reference for all custom filters available in the Test Plan Documentation Generator, with detailed examples and use cases.

## Table of Contents

1. [Overview](#overview)
2. [Custom Filters](#custom-filters)
   - [strip](#strip)
   - [replace](#replace)
   - [replace_regex](#replace_regex)
3. [Built-in Tera Filters](#built-in-tera-filters)
4. [Filter Chaining](#filter-chaining)
5. [Use Cases by Scenario](#use-cases-by-scenario)
6. [Performance Considerations](#performance-considerations)

## Overview

Filters in Tera (the templating engine) transform values in templates. They are applied using the pipe (`|`) operator:

```jinja2
{{ value | filter_name }}
{{ value | filter_name(arg1="value1", arg2="value2") }}
```

The Test Plan Documentation Generator provides **three custom filters** in addition to Tera's standard built-in filters.

## Custom Filters

### `strip`

Removes leading and trailing whitespace from a string.

#### Syntax

```jinja2
{{ value | strip }}
```

#### Parameters

None

#### Examples

##### Basic Usage

```jinja2
Input:  "  hello world  "
Filter: {{ text | strip }}
Output: "hello world"
```

##### With YAML Data

```yaml
description: "  This is a test case description.  "
id: "  TC-001  "
```

```jinja2
{{ description | strip }}
{# Output: "This is a test case description." #}

{{ id | strip }}
{# Output: "TC-001" #}
```

##### Multi-line Strings

```yaml
description: |
  This is a multi-line description.
  It has multiple lines.
  
```

```jinja2
{{ description | strip }}
{# Output: "This is a multi-line description.\nIt has multiple lines." #}
{# (trailing newline removed) #}
```

##### In Tables

```jinja2
| ID | Description |
|----|-------------|
{% for test in tests -%}
| {{ test.id | strip }} | {{ test.description | strip }} |
{% endfor %}
```

#### Use Cases

1. **Clean YAML whitespace**: YAML data often has trailing/leading spaces
2. **Table formatting**: Ensure clean table cell content
3. **ID normalization**: Clean test IDs before display
4. **Heading generation**: Create clean section headings
5. **Comparison operations**: Normalize before comparing strings

#### Common Patterns

```jinja2
{# Clean heading #}
## {{ title | strip }}

{# Clean bold text #}
**{{ label | strip }}**: {{ value | strip }}

{# Clean list items #}
{% for item in items %}
- {{ item | strip }}
{% endfor %}

{# Clean inline content #}
The test {{ id | strip }} verifies {{ requirement | strip }}.
```

#### Edge Cases

```jinja2
{# Empty string #}
{{ "" | strip }}
{# Output: "" #}

{# Only whitespace #}
{{ "   " | strip }}
{# Output: "" #}

{# No whitespace #}
{{ "hello" | strip }}
{# Output: "hello" #}

{# Internal whitespace preserved #}
{{ "  hello  world  " | strip }}
{# Output: "hello  world" #}
```

---

### `replace`

Replaces occurrences of a substring with another string, with optional limit on number of replacements.

#### Syntax

```jinja2
{{ value | replace(old="search_string", new="replacement_string") }}
{{ value | replace(old="search_string", new="replacement_string", times=N) }}
```

#### Parameters

- **`old`** (required, string): The substring to search for
- **`new`** (required, string): The replacement string
- **`times`** (optional, integer): Maximum number of replacements (default: unlimited)

#### Examples

##### Replace All Occurrences

```jinja2
Input:  "foo bar foo baz foo"
Filter: {{ text | replace(old="foo", new="qux") }}
Output: "qux bar qux baz qux"
```

##### Replace Limited Occurrences

```jinja2
{# Replace first occurrence only #}
Input:  "foo bar foo baz"
Filter: {{ text | replace(old="foo", new="qux", times=1) }}
Output: "qux bar foo baz"

{# Replace first two occurrences #}
Input:  "foo bar foo baz foo"
Filter: {{ text | replace(old="foo", new="qux", times=2) }}
Output: "qux bar qux baz foo"
```

##### Remove Substring (Replace with Empty String)

```jinja2
Input:  "MTD_SEND_COMMAND"
Filter: {{ text | replace(old="MTD_", new="") }}
Output: "SEND_COMMAND"

Input:  "Test: Basic Validation"
Filter: {{ text | replace(old="Test: ", new="") }}
Output: "Basic Validation"
```

##### Replace Spaces

```jinja2
Input:  "hello world test"
Filter: {{ text | replace(old=" ", new="_") }}
Output: "hello_world_test"

Input:  "hello world test"
Filter: {{ text | replace(old=" ", new="-") }}
Output: "hello-world-test"
```

##### Case-Sensitive Replacement

```jinja2
Input:  "Foo bar foo baz"
Filter: {{ text | replace(old="foo", new="qux") }}
Output: "Foo bar qux baz"
{# Note: "Foo" (uppercase F) is NOT replaced #}
```

#### Use Cases

1. **Remove prefixes**: Strip common prefixes from command descriptions
2. **Normalize separators**: Convert spaces to hyphens or underscores
3. **Clean identifiers**: Remove or replace specific patterns
4. **Format conversion**: Replace markdown syntax for different formats
5. **Data sanitization**: Remove or replace unwanted characters

#### Common Patterns

##### Remove Command Prefix

```jinja2
{# Remove MTD_ prefix from commands #}
{% for step in steps %}
{{ step.description | replace(old="MTD_", new="") }}
{% endfor %}

Input:  "MTD_SENDS_SMS_PP([INSTALL_PERSO_RES_ISDP])"
Output: "SENDS_SMS_PP([INSTALL_PERSO_RES_ISDP])"
```

##### Create Slug from Title

```jinja2
{{ title | strip | lower | replace(old=" ", new="-") }}

Input:  "  Test Case Documentation  "
Output: "test-case-documentation"
```

##### Remove Markdown Formatting

```jinja2
{{ text | replace(old="**", new="") | replace(old="*", new="") }}

Input:  "This is **bold** and *italic*"
Output: "This is bold and italic"
```

##### Replace Domain-Specific Terms

```jinja2
{{ description | replace(old="eUICC", new="Embedded UICC") }}

Input:  "The eUICC device performs provisioning"
Output: "The Embedded UICC device performs provisioning"
```

##### Clean List Markers

```jinja2
{{ item | replace(old="- ", new="") | strip }}

Input:  "- Test item"
Output: "Test item"
```

#### Edge Cases

```jinja2
{# No match found #}
{{ "hello world" | replace(old="foo", new="bar") }}
{# Output: "hello world" #}

{# Empty old string (no replacement) #}
{{ "hello" | replace(old="", new="x") }}
{# Output: "hello" #}

{# Replace with empty string #}
{{ "hello" | replace(old="hello", new="") }}
{# Output: "" #}

{# Times = 0 (no replacement) #}
{{ "foo bar foo" | replace(old="foo", new="bar", times=0) }}
{# Output: "foo bar foo" #}

{# Times greater than occurrences #}
{{ "foo bar" | replace(old="foo", new="bar", times=10) }}
{# Output: "bar bar" (only 1 occurrence replaced) #}
```

---

### `replace_regex`

Replaces text matching a regular expression pattern with a replacement string, with optional limit on number of replacements. Supports capture groups.

#### Syntax

```jinja2
{{ value | replace_regex(old="regex_pattern", new="replacement") }}
{{ value | replace_regex(old="regex_pattern", new="replacement", times=N) }}
```

#### Parameters

- **`old`** (required, string): Regular expression pattern to match
- **`new`** (required, string): Replacement string (can use `$1`, `$2`, etc. for capture groups)
- **`times`** (optional, integer): Maximum number of replacements (default: unlimited)

#### Regular Expression Syntax

The filter uses Rust's `regex` crate, which supports standard regex syntax:

- `.` - Any character except newline
- `\d` - Digit `[0-9]`
- `\D` - Non-digit
- `\s` - Whitespace
- `\S` - Non-whitespace
- `\w` - Word character `[a-zA-Z0-9_]`
- `\W` - Non-word character
- `[abc]` - Character class
- `[^abc]` - Negated character class
- `*` - Zero or more
- `+` - One or more
- `?` - Zero or one
- `{n}` - Exactly n times
- `{n,}` - n or more times
- `{n,m}` - Between n and m times
- `^` - Start of string
- `$` - End of string
- `|` - Alternation (OR)
- `()` - Capture group

#### Examples

##### Remove All Digits

```jinja2
Input:  "test123abc456def"
Filter: {{ text | replace_regex(old="[0-9]+", new="") }}
Output: "testabcdef"

Input:  "Version 1.2.3 Build 456"
Filter: {{ text | replace_regex(old="\\d+", new="X") }}
Output: "Version X.X.X Build X"
```

##### Sanitize IDs for Anchors/Filenames

```jinja2
Input:  "TC.Test-01: Basic Validation"
Filter: {{ id | replace_regex(old="[^a-zA-Z0-9_]", new="_") }}
Output: "TC_Test_01__Basic_Validation"

Input:  "4.2.2.2.1 TC_eUICC_ES6.UpdateMetadata"
Filter: {{ id | replace_regex(old="[^a-zA-Z0-9_-]", new="_") }}
Output: "4_2_2_2_1_TC_eUICC_ES6_UpdateMetadata"
```

##### Normalize Whitespace

```jinja2
{# Replace multiple spaces with single space #}
Input:  "hello    world   test"
Filter: {{ text | replace_regex(old="\\s+", new=" ") }}
Output: "hello world test"

{# Replace all whitespace (including newlines) with space #}
Input:  "line1\n  line2\t\tline3"
Filter: {{ text | replace_regex(old="\\s+", new=" ") }}
Output: "line1 line2 line3"
```

##### Extract or Transform Patterns

```jinja2
{# Remove everything except alphanumeric #}
Input:  "Test-123_ABC.xyz"
Filter: {{ text | replace_regex(old="[^a-zA-Z0-9]", new="") }}
Output: "Test123ABCxyz"

{# Keep only letters #}
Input:  "Test123ABC"
Filter: {{ text | replace_regex(old="[^a-zA-Z]", new="") }}
Output: "TestABC"

{# Remove special characters but keep spaces #}
Input:  "Hello! World? Test."
Filter: {{ text | replace_regex(old="[^a-zA-Z0-9 ]", new="") }}
Output: "Hello World Test"
```

##### Limited Replacements

```jinja2
{# Replace first digit sequence only #}
Input:  "test123abc456"
Filter: {{ text | replace_regex(old="[0-9]+", new="NUM", times=1) }}
Output: "testNUMabc456"

{# Replace first two whitespace sequences #}
Input:  "a  b  c  d"
Filter: {{ text | replace_regex(old="\\s+", new="_", times=2) }}
Output: "a_b_c  d"
```

##### Using Capture Groups

```jinja2
{# Swap first and last word #}
Input:  "hello world"
Filter: {{ text | replace_regex(old="^(\\w+)\\s+(\\w+)$", new="$2 $1") }}
Output: "world hello"

{# Extract and format version #}
Input:  "version-1.2.3-release"
Filter: {{ text | replace_regex(old="version-(\\d+)\\.(\\d+)\\.(\\d+)-.*", new="v$1.$2.$3") }}
Output: "v1.2.3"
```

##### Remove Specific Patterns

```jinja2
{# Remove HTML tags #}
Input:  "<p>Hello <strong>world</strong></p>"
Filter: {{ text | replace_regex(old="<[^>]+>", new="") }}
Output: "Hello world"

{# Remove URLs #}
Input:  "Visit https://example.com for more"
Filter: {{ text | replace_regex(old="https?://[^\\s]+", new="[URL]") }}
Output: "Visit [URL] for more"

{# Remove email addresses #}
Input:  "Contact: user@example.com for info"
Filter: {{ text | replace_regex(old="[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\\.[a-zA-Z]{2,}", new="[EMAIL]") }}
Output: "Contact: [EMAIL] for info"
```

##### Case-Insensitive Replacement

Use `(?i)` flag for case-insensitive matching:

```jinja2
Input:  "Foo bar FOO baz foo"
Filter: {{ text | replace_regex(old="(?i)foo", new="qux") }}
Output: "qux bar qux baz qux"
```

#### Use Cases

1. **ID sanitization**: Convert test IDs to safe anchor/filename format
2. **Whitespace normalization**: Clean up irregular spacing
3. **Pattern extraction**: Extract specific parts of text
4. **Data cleaning**: Remove or replace complex patterns
5. **Format conversion**: Transform text based on patterns
6. **Validation**: Strip invalid characters

#### Common Patterns

##### Safe Anchor ID Generation

```jinja2
{# Convert test case ID to HTML anchor #}
{{ id | strip | replace_regex(old="[^a-zA-Z0-9_-]", new="_") | lower }}

Input:  "4.2.2.2.1 TC_eUICC_ES6.UpdateMetadata"
Output: "4_2_2_2_1_tc_euicc_es6_updatemetadata"
```

##### Filename Safe String

```jinja2
{# Create safe filename #}
{{ title | strip | replace_regex(old="[^a-zA-Z0-9_-]", new="_") }}

Input:  "Test Case: Basic Validation (v1.0)"
Output: "Test_Case__Basic_Validation__v1_0_"
```

##### Clean Command Output

```jinja2
{# Remove ANSI color codes #}
{{ output | replace_regex(old="\\x1b\\[[0-9;]*m", new="") }}

{# Remove control characters #}
{{ text | replace_regex(old="[\\x00-\\x1F\\x7F]", new="") }}
```

##### Format Phone Numbers

```jinja2
{# Format phone number #}
{{ phone | replace_regex(old="(\\d{3})(\\d{3})(\\d{4})", new="($1) $2-$3") }}

Input:  "1234567890"
Output: "(123) 456-7890"
```

##### Collapse Multiple Newlines

```jinja2
{{ text | replace_regex(old="\\n{2,}", new="\\n\\n") }}

Input:  "Line1\n\n\n\nLine2"
Output: "Line1\n\nLine2"
```

#### Edge Cases

```jinja2
{# Invalid regex pattern (will cause error) #}
{# {{ text | replace_regex(old="[invalid", new="x") }} #}
{# Error: Invalid regex pattern #}

{# No match found #}
{{ "hello world" | replace_regex(old="\\d+", new="X") }}
{# Output: "hello world" #}

{# Empty match (be careful) #}
{{ "hello" | replace_regex(old="", new="x") }}
{# May cause infinite loop or error - avoid empty patterns #}

{# Times = 0 #}
{{ "test123" | replace_regex(old="\\d+", new="X", times=0) }}
{# Output: "test123" #}
```

#### Escaping Special Characters

To match literal special characters, escape them with `\\`:

```jinja2
{# Match literal dot #}
{{ "test.file.txt" | replace_regex(old="\\.", new="_") }}
{# Output: "test_file_txt" #}

{# Match literal parentheses #}
{{ "func(arg)" | replace_regex(old="\\(|\\)", new="") }}
{# Output: "funcarg" #}

{# Match literal backslash (need quad backslash in template) #}
{{ "path\\to\\file" | replace_regex(old="\\\\", new="/") }}
{# Output: "path/to/file" #}
```

---

## Built-in Tera Filters

In addition to custom filters, Tera provides many built-in filters. Here are the most commonly used:

### String Filters

```jinja2
{{ text | upper }}              {# Convert to uppercase #}
{{ text | lower }}              {# Convert to lowercase #}
{{ text | capitalize }}         {# Capitalize first letter #}
{{ text | title }}              {# Title Case Each Word #}
{{ text | trim }}               {# Alias for strip (whitespace) #}
{{ text | truncate(length=10) }} {# Truncate to length #}
{{ text | wordcount }}          {# Count words #}
{{ text | split(pat=" ") }}     {# Split into array #}
{{ text | safe }}               {# Mark as safe HTML #}
{{ text | escape }}             {# HTML escape #}
{{ text | urlencode }}          {# URL encode #}
{{ text | slugify }}            {# Create URL slug #}
```

### Array Filters

```jinja2
{{ array | length }}                    {# Get array length #}
{{ array | first }}                     {# Get first element #}
{{ array | last }}                      {# Get last element #}
{{ array | nth(n=2) }}                  {# Get nth element (0-indexed) #}
{{ array | join(sep=", ") }}            {# Join with separator #}
{{ array | reverse }}                   {# Reverse array #}
{{ array | sort }}                      {# Sort array #}
{{ array | unique }}                    {# Remove duplicates #}
{{ array | slice(start=0, end=5) }}     {# Slice array #}
{{ array | concat(with=other_array) }}  {# Concatenate arrays #}
```

### Array Filtering

```jinja2
{# Filter by attribute value #}
{{ tests | filter(attribute="status", value="pass") }}

{# Example #}
{% set passed_tests = test_results | filter(attribute="overall_pass", value=true) %}
{{ passed_tests | length }} tests passed
```

### Number Filters

```jinja2
{{ number | round }}              {# Round to integer #}
{{ number | round(precision=2) }} {# Round to 2 decimal places #}
{{ number | abs }}                {# Absolute value #}
{{ number | filesizeformat }}     {# Format as file size #}
```

### Date/Time Filters

```jinja2
{{ timestamp | date(format="%Y-%m-%d") }}
{{ timestamp | date(format="%B %d, %Y") }}
```

### Other Filters

```jinja2
{{ value | default(value="N/A") }}  {# Default if undefined #}
{{ value | json_encode }}           {# Encode as JSON #}
{{ value | get(key="field") }}      {# Get object property #}
{{ value | as_str }}                {# Convert to string #}
```

---

## Filter Chaining

Filters can be chained together to perform multiple transformations:

### Basic Chaining

```jinja2
{{ text | strip | upper }}
{{ text | strip | lower | capitalize }}
```

### Complex Transformations

```jinja2
{# Clean and sanitize test ID #}
{{ id | strip | replace_regex(old="[^a-zA-Z0-9_]", new="_") | lower }}

Input:  "  TC.Test-01: Basic  "
Output: "tc_test_01__basic"
```

### Multi-step Data Cleaning

```jinja2
{# Remove prefix, normalize whitespace, trim #}
{{ description | replace(old="MTD_", new="") | replace_regex(old="\\s+", new=" ") | strip }}

Input:  "MTD_SEND_COMMAND    with   spaces"
Output: "SEND_COMMAND with spaces"
```

### Create Safe Identifiers

```jinja2
{# Convert title to safe slug #}
{{ title | strip | lower | replace(old=" ", new="-") | replace_regex(old="[^a-z0-9-]", new="") }}

Input:  "  Test Case: Basic (v1.0)  "
Output: "test-case-basic-v10"
```

### Format and Clean Lists

```jinja2
{# Clean and join array #}
{{ items | map(attribute="name") | join(sep=", ") }}

{# Clean each item then join #}
{% set cleaned = [] %}
{% for item in items %}
  {% set_global cleaned = cleaned | concat(with=[item | strip]) %}
{% endfor %}
{{ cleaned | join(sep=", ") }}
```

### Nested Filtering

```jinja2
{# Get passed tests, extract IDs, join #}
{{ test_results | filter(attribute="overall_pass", value=true) | map(attribute="test_case_id") | join(sep=", ") }}
```

### Order Matters

```jinja2
{# Different order, different result #}

{# Order 1: strip then upper #}
{{ "  hello  " | strip | upper }}
{# Output: "HELLO" #}

{# Order 2: upper then strip #}
{{ "  hello  " | upper | strip }}
{# Output: "HELLO" (same result in this case) #}

{# Order matters here: #}
{{ "Hello World" | replace(old="Hello", new="Hi") | lower }}
{# Output: "hi world" #}

{{ "Hello World" | lower | replace(old="Hello", new="Hi") }}
{# Output: "hello world" (no replacement - already lowercase) #}
```

---

## Use Cases by Scenario

### Scenario 1: Cleaning YAML Data

YAML often includes trailing whitespace and formatting issues:

```jinja2
{# Clean all text fields #}
{{ id | strip }}
{{ description | strip }}
{{ requirement | strip }}
{{ name | strip }}

{# Normalize multi-line content #}
{{ procedure | replace_regex(old="\\s+", new=" ") | strip }}
```

### Scenario 2: Creating Markdown Tables

```jinja2
| ID | Description | Status |
|----|-------------|--------|
{% for test in tests -%}
| {{ test.id | strip }} | {{ test.description | strip | truncate(length=50) }} | {{ test.status | upper }} |
{% endfor %}
```

### Scenario 3: Generating HTML Anchors

```jinja2
{# Create safe anchor IDs #}
{% for section in sections %}
<h2 id="{{ section.title | strip | lower | replace(old=' ', new='-') | replace_regex(old='[^a-z0-9-]', new='') }}">
  {{ section.title | strip }}
</h2>
{% endfor %}
```

### Scenario 4: Command Description Cleaning

```jinja2
{# Remove common prefixes and clean spacing #}
{% for step in steps %}
{{ step.description | replace(old="MTD_", new="") | replace_regex(old="\\s+", new=" ") | strip }}
{% endfor %}

Input:  "MTD_SENDS_SMS_PP([INSTALL_PERSO_RES_ISDP];   MTD_STORE_DATA_SCRIPT(#TEST_DATA, TRUE))"
Output: "SENDS_SMS_PP([INSTALL_PERSO_RES_ISDP]; STORE_DATA_SCRIPT(#TEST_DATA, TRUE))"
```

### Scenario 5: Creating File Names

```jinja2
{# Safe filename from test case ID #}
{% set filename = id | strip | replace_regex(old="[^a-zA-Z0-9_-]", new="_") | lower ~ ".md" %}

Input ID: "4.2.2.2.1 TC_eUICC_ES6.UpdateMetadata"
Filename: "4_2_2_2_1_tc_euicc_es6_updatemetadata.md"
```

### Scenario 6: Requirement Aggregation

```jinja2
{# Filter and display passed requirements #}
{% set passed_reqs = test_results | filter(attribute="overall_pass", value=true) | map(attribute="requirement") | unique %}

**Passed Requirements**: {{ passed_reqs | join(sep=", ") }}
```

### Scenario 7: Conditional Formatting

```jinja2
{% for step in steps %}
| {{ step.step }} | {% if step.manual %}**MANUAL**{% else %}AUTO{% endif %} | {{ step.description | replace(old="MTD_", new="") | strip }} |
{% endfor %}
```

### Scenario 8: Version Extraction and Formatting

```jinja2
{# Extract version from string #}
{% if version_string %}
{% set version = version_string | replace_regex(old=".*version[\\s:-]*(\\d+\\.\\d+\\.\\d+).*", new="$1") %}
Version: {{ version }}
{% endif %}
```

---

## Performance Considerations

### Filter Performance

1. **`strip`**: Very fast, minimal overhead
2. **`replace`**: Fast for simple replacements
3. **`replace_regex`**: Slower than `replace` due to regex compilation; avoid in tight loops if possible

### Optimization Tips

#### Cache Regex Results

Instead of:
```jinja2
{% for item in large_list %}
{{ item.text | replace_regex(old="[^a-z0-9]", new="_") }}
{% endfor %}
```

Consider pre-processing if the pattern is complex and list is large.

#### Use Simple Filters When Possible

```jinja2
{# Prefer replace over replace_regex when pattern is literal #}
{{ text | replace(old="_", new="-") }}  {# Fast #}
{{ text | replace_regex(old="_", new="-") }}  {# Slower #}
```

#### Minimize Filter Chains

```jinja2
{# Less efficient #}
{{ text | strip | upper | lower | strip }}

{# More efficient #}
{{ text | strip | lower }}
```

#### Conditional Filtering

```jinja2
{# Only apply expensive filters when needed #}
{% if needs_sanitization %}
{{ text | replace_regex(old="[^a-zA-Z0-9]", new="_") }}
{% else %}
{{ text }}
{% endif %}
```

---

## Summary

This reference covered:

- **Custom Filters**: `strip`, `replace`, `replace_regex` with detailed examples
- **Built-in Filters**: Common Tera filters for strings, arrays, and numbers
- **Filter Chaining**: Combining filters for complex transformations
- **Use Cases**: Practical scenarios for filter application
- **Performance**: Optimization tips for filter usage

### Quick Reference Table

| Filter | Purpose | Example |
|--------|---------|---------|
| `strip` | Remove whitespace | `{{ "  text  " \| strip }}` → `"text"` |
| `replace` | String replacement | `{{ "foo" \| replace(old="o", new="a") }}` → `"faa"` |
| `replace_regex` | Regex replacement | `{{ "test123" \| replace_regex(old="\\d+", new="X") }}` → `"testX"` |
| `upper` | Uppercase | `{{ "text" \| upper }}` → `"TEXT"` |
| `lower` | Lowercase | `{{ "TEXT" \| lower }}` → `"text"` |
| `length` | Get length | `{{ [1,2,3] \| length }}` → `3` |
| `join` | Join array | `{{ ["a","b"] \| join(sep=",") }}` → `"a,b"` |
| `filter` | Filter array | `{{ items \| filter(attribute="x", value=1) }}` |
| `default` | Default value | `{{ var \| default(value="N/A") }}` |

For template authoring guidance, see [template-authoring-guide.md](template-authoring-guide.md).

For schema details, see [schema-reference.md](schema-reference.md).
