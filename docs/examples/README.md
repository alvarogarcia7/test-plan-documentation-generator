# Documentation Examples

This directory contains annotated example test cases demonstrating all verification methods and common patterns for the Test Plan Documentation Generator.

## Overview

Each example file is heavily annotated with:
- Detailed field descriptions
- Common patterns and best practices
- Template rendering tips
- Validation requirements
- Real-world usage examples

## Example Files

### Verification Method Examples

| File | Type | Description |
|------|------|-------------|
| [test-case-example.yml](test-case-example.yml) | Test | Complete test case with multiple sequences, steps, and conditions |
| [analysis-case-example.yml](analysis-case-example.yml) | Analysis | Analytical verification with calculations and models |
| [demonstration-case-example.yml](demonstration-case-example.yml) | Demonstration | Live demonstration procedure with observations |
| [inspection-case-example.yml](inspection-case-example.yml) | Inspection | Security compliance inspection with detailed checklist |
| [filter-usage-example.yml](filter-usage-example.yml) | Test | Demonstrates custom filter usage patterns |

### Quick Reference

#### Test Case Structure

```yaml
type: test
requirement: "REQ-ID"
item: 1
tc: 1
id: "TC-001"
description: "Test description"
general_initial_conditions: { }
initial_conditions: { }
test_sequences:
  - id: 1
    name: "Sequence name"
    description: "Sequence description"
    initial_conditions: { }
    steps:
      - step: 1
        description: "Step description"
        command: "command"
        expected:
          result: "Expected result"
          output: "Expected output"
```

#### Analysis Case Structure

```yaml
type: analysis
requirement: "REQ-ID"
id: "AN-001"
description: "Analysis description"
method: "Analysis method"
calculations:
  - name: "Calculation name"
    formula: "Mathematical formula"
    parameters: { }
models:
  - name: "Model name"
    type: "Model type"
    description: "Model description"
acceptance_criteria:
  - "Criterion 1"
```

#### Demonstration Case Structure

```yaml
type: demonstration
requirement: "REQ-ID"
id: "DM-001"
description: "Demonstration description"
procedure: "Step-by-step procedure"
observations:
  - "Observation 1"
acceptance_criteria:
  - "Criterion 1"
```

#### Inspection Case Structure

```yaml
type: inspection
requirement: "REQ-ID"
id: "IN-001"
description: "Inspection description"
inspection_method: "Inspection method"
checklist:
  - "Check item 1"
acceptance_criteria:
  - "Criterion 1"
```

## Common Patterns

### 1. Whitespace Handling

YAML preserves trailing spaces. Always use the `strip` filter:

```jinja2
{{ id | strip }}
{{ description | strip }}
```

### 2. ID Sanitization

Create safe HTML/Markdown anchors:

```jinja2
{{ id | replace_regex(old="[^a-zA-Z0-9_]", new="_") }}
```

### 3. Command Prefix Removal

Remove common prefixes from descriptions:

```jinja2
{{ description | replace(old="MTD_", new="") }}
```

### 4. Multi-line Content Normalization

Normalize whitespace in multi-line content:

```jinja2
{{ procedure | replace_regex(old="\\s+", new=" ") | strip }}
```

### 5. Conditional Rendering

Handle optional fields:

```jinja2
{% if step.manual %}Manual Step{% endif %}
{% if calc.parameters %}
  Parameters: ...
{% endif %}
```

### 6. Filter Chaining

Combine multiple transformations:

```jinja2
{{ id | strip | lower | replace(old=" ", new="-") }}
```

## How to Use These Examples

### As Learning Resources

1. **Read the annotations**: Each example includes inline comments explaining fields and patterns
2. **Study the structure**: Understand the required vs. optional fields
3. **Review tips sections**: Each file has a "COMMON PATTERNS AND TIPS" section at the end

### As Templates

1. **Copy and modify**: Use examples as starting points for your own test cases
2. **Remove annotations**: Strip out comments for production use
3. **Adapt patterns**: Apply the demonstrated patterns to your specific needs

### For Testing

1. **Validation testing**: Use examples to test schema validation
2. **Template testing**: Use with actual templates to verify rendering
3. **Integration testing**: Run through the complete tool workflow

## Validation

All examples conform to their respective schemas and will pass validation:

```bash
# Test validation (example)
./target/release/test-plan-doc-gen \
  --container data/container/schema.json \
             data/container/template.j2 \
             data/container/data.yml \
  --test-case data/verification_methods \
              docs/examples/test-case-example.yml
```

## Field Reference Quick Links

- **Test Schema**: See [../schema-reference.md#test-schema](../schema-reference.md#test-schema)
- **Analysis Schema**: See [../schema-reference.md#analysis-schema](../schema-reference.md#analysis-schema)
- **Demonstration Schema**: See [../schema-reference.md#demonstration-schema](../schema-reference.md#demonstration-schema)
- **Inspection Schema**: See [../schema-reference.md#inspection-schema](../schema-reference.md#inspection-schema)

## Template Authoring

For detailed template authoring guidance, see:
- [Template Authoring Guide](../template-authoring-guide.md)
- [Filters Reference](../filters-reference.md)

## Tips for Creating Test Cases

### Best Practices

1. **Be Specific**: Use precise language and measurable criteria
2. **Be Consistent**: Use consistent terminology and structure
3. **Be Complete**: Include all required fields
4. **Be Clear**: Write descriptions that non-experts can understand
5. **Use Strip**: Always apply strip filter in templates to YAML text fields

### Common Mistakes to Avoid

1. **Wrong data types**: Using `item: "1"` instead of `item: 1`
2. **Missing required fields**: Check schema for required fields
3. **Trailing whitespace**: YAML preserves it; use strip filter
4. **Invalid enum values**: `type` must exactly match schema enum
5. **Inconsistent structure**: Follow the hierarchical structure exactly

### Validation Tips

1. **Validate early**: Check schema compliance before rendering
2. **Use schema-aware editor**: VSCode with YAML extension helps
3. **Check error messages**: Validation errors are specific and helpful
4. **Test incrementally**: Build up complex cases step by step

## Real-World Examples

For real production examples from the project, see:

- `data/test_case/gsma_4.4.2.2_TC.yml` - Real test case
- `data/test_case/gsma_4.4.2.4_AN.yml` - Real analysis case
- `data/test_case/gsma_4.4.2.5_DM.yml` - Real demonstration case
- `data/test_case/gsma_4.4.2.6_IN.yml` - Real inspection case
- `data/test_case/filter_test_01_TC.yml` - Filter usage example

These are production test cases used in the project's end-to-end tests.

## Contributing

When adding new examples:

1. Include comprehensive annotations
2. Cover common use cases
3. Demonstrate best practices
4. Validate against schemas
5. Test with actual templates
6. Update this README

## Summary

These examples provide:
- **Complete documentation** of all verification method types
- **Annotated guidance** for each field and structure
- **Best practices** for authoring test cases
- **Common patterns** for template rendering
- **Real-world usage** scenarios

Use them as learning resources, templates, and testing fixtures to master the Test Plan Documentation Generator.
