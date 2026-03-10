# Tera Template Filter Guide

This guide explains the custom filters available in Tera templates used by the test plan documentation generator.

## Available Filters

### 1. `replace` - String Replacement Filter

Replaces occurrences of a substring with another string.

**Arguments:**
- `old` (required): The substring to find
- `new` (required): The replacement string
- `times` (optional): Number of occurrences to replace. If not specified, replaces all occurrences.

**Examples:**

```jinja2
{# Replace all occurrences #}
{{ "foo bar foo baz foo" | replace(old="foo", new="bar") }}
Output: bar bar bar baz bar

{# Replace first occurrence only #}
{{ "foo bar foo baz foo" | replace(old="foo", new="qux", times=1) }}
Output: qux bar foo baz foo

{# Replace first two occurrences #}
{{ "foo bar foo baz foo" | replace(old="foo", new="xyz", times=2) }}
Output: xyz bar xyz baz foo
```

### 2. `replace_regex` - Regular Expression Replacement Filter

Replaces text matching a regular expression pattern with a replacement string.

**Arguments:**
- `old` (required): Regular expression pattern to match
- `new` (required): The replacement string
- `times` (optional): Number of matches to replace. If not specified, replaces all matches.

**Examples:**

```jinja2
{# Remove all digits #}
{{ "test123abc456def789" | replace_regex(old="[0-9]+", new="") }}
Output: testabcdef

{# Replace all digit sequences with '#' #}
{{ "test123abc456def789" | replace_regex(old="[0-9]+", new="#") }}
Output: test#abc#def#

{# Replace only first digit sequence #}
{{ "test123abc456def789" | replace_regex(old="[0-9]+", new="NUM", times=1) }}
Output: testNUMabc456def789

{# Replace all letter sequences with asterisks #}
{{ "test123abc456def789" | replace_regex(old="[a-z]+", new="*") }}
Output: *123*456*789
```

### 3. `strip` - Whitespace Trimming Filter

Removes leading and trailing whitespace from a string.

**Arguments:** None

**Examples:**

```jinja2
{# Strip whitespace from both ends #}
{{ "  hello world  " | strip }}
Output: hello world

{# Strip works with multiline text too #}
{{ multiline_text | strip }}
Output: (trimmed text without leading/trailing whitespace)
```

## Filter Chaining

Filters can be chained together to perform multiple transformations in sequence:

```jinja2
{# Strip then replace #}
{{ "  hello world  " | strip | replace(old="hello", new="goodbye") }}
Output: goodbye world

{# Replace then strip #}
{{ "  hello world  " | replace(old="world", new="universe") | strip }}
Output: hello universe

{# Complex chain with multiple filters #}
{{ "AAA-BBB-CCC AAA-DDD-EEE" | replace(old="AAA", new="XXX") | replace_regex(old="-[A-Z]+", new="") | strip }}
Output: XXX XXX
```

## Common Use Cases

### Cleaning User Input

```jinja2
{{ user_input | strip }}
```

### Normalizing Text

```jinja2
{{ text | replace(old="\r\n", new="\n") | strip }}
```

### Redacting Information

```jinja2
{{ "User ID: 12345" | replace_regex(old="[0-9]+", new="XXXX") }}
Output: User ID: XXXX
```

### Sanitizing Markdown Table Content

```jinja2
{# Replace pipe characters that would break markdown tables #}
{{ cell_content | replace(old="|", new="\\|") }}
```

### Pattern-Based Text Transformation

```jinja2
{# Remove HTML tags #}
{{ html_text | replace_regex(old="<[^>]+>", new="") }}

{# Extract numbers from text #}
{{ "Price: $123.45" | replace_regex(old="[^0-9.]", new="") }}
Output: 123.45
```

## Tips and Best Practices

1. **Always quote filter arguments** to avoid parsing issues:
   ```jinja2
   {# Good #}
   {{ text | replace(old="foo", new="bar") }}
   
   {# Bad - may cause errors #}
   {{ text | replace(old=foo, new=bar) }}
   ```

2. **Escape special regex characters** when using `replace_regex`:
   ```jinja2
   {# To match literal dots, escape them #}
   {{ text | replace_regex(old="\\.", new="") }}
   ```

3. **Use `strip` before or after other filters** depending on your needs:
   ```jinja2
   {# Strip before replacement to avoid matching whitespace #}
   {{ text | strip | replace(old="hello", new="hi") }}
   ```

4. **The `times` parameter counts replacements, not matches**:
   ```jinja2
   {# This replaces the first match only #}
   {{ text | replace_regex(old="[0-9]+", new="X", times=1) }}
   ```

5. **Chain filters efficiently**:
   ```jinja2
   {# Multiple simple replacements can be chained #}
   {{ text | replace(old="foo", new="bar") | replace(old="baz", new="qux") }}
   ```
