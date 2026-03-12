# Advanced Documentation

This directory contains comprehensive documentation for the Test Plan Documentation Generator, including detailed guides for template authoring, filter usage, schema specifications, and annotated examples.

## Documentation Structure

```
docs/
├── README.md                          # This file - documentation overview
├── template-authoring-guide.md        # Complete template authoring guide
├── filters-reference.md               # Detailed filter documentation
├── schema-reference.md                # All JSON schema specifications
└── examples/                          # Annotated example test cases
    ├── README.md                      # Examples overview
    ├── test-case-example.yml          # Annotated test case
    ├── analysis-case-example.yml      # Annotated analysis case
    ├── demonstration-case-example.yml # Annotated demonstration case
    ├── inspection-case-example.yml    # Annotated inspection case
    └── filter-usage-example.yml       # Filter usage patterns
```

## Quick Start

### For Template Authors

If you're creating or modifying templates:

1. **Start with**: [Template Authoring Guide](template-authoring-guide.md)
   - Complete guide to Tera/Jinja2 syntax
   - All verification method schemas explained
   - Template variable access patterns
   - Best practices and common patterns

2. **Reference**: [Filters Reference](filters-reference.md)
   - All custom filters with examples
   - Built-in Tera filters
   - Filter chaining patterns
   - Use cases by scenario

3. **Examples**: [examples/](examples/)
   - Real, annotated examples
   - Common patterns demonstrated
   - Filter usage in context

### For Test Case Authors

If you're writing test case YAML files:

1. **Start with**: [Schema Reference](schema-reference.md)
   - All JSON schema specifications
   - Required vs. optional fields
   - Field types and structures
   - Validation rules

2. **Reference**: [Examples](examples/)
   - Annotated examples for each verification method
   - Best practices and tips
   - Common mistakes to avoid

3. **Validate**: Check your YAML against schemas
   ```bash
   # The tool validates automatically
   ./target/release/tpdg --test-case ...
   ```

### For Integration

If you're integrating the tool into a workflow:

1. **Read**: Main [README.md](../README.md) - CLI usage and options
2. **Reference**: [AGENTS.md](../AGENTS.md) - Build, test, and validation commands
3. **Examples**: Study example test cases in `data/test_case/`

## Documentation Files

### [Template Authoring Guide](template-authoring-guide.md)

**Purpose**: Comprehensive guide for creating and modifying templates

**Contents**:
- Template engine (Tera) overview and syntax
- All verification method schemas (test, analysis, demonstration, inspection)
- Template variables and context
- Control structures (if, for, loops)
- Custom filters usage
- Container vs. verification method templates
- Best practices and common patterns
- Troubleshooting tips

**When to use**: Creating new templates, modifying existing templates, understanding template rendering

---

### [Filters Reference](filters-reference.md)

**Purpose**: Complete reference for all available filters

**Contents**:
- Custom filters: `strip`, `replace`, `replace_regex`
- Detailed syntax and parameters
- Extensive examples for each filter
- Built-in Tera filters
- Filter chaining techniques
- Use cases by scenario
- Performance considerations

**When to use**: Applying data transformations in templates, sanitizing IDs, cleaning whitespace, pattern matching

---

### [Schema Reference](schema-reference.md)

**Purpose**: Complete documentation of all JSON schemas

**Contents**:
- Container schema specification
- Test verification method schema
- Analysis verification method schema
- Demonstration verification method schema
- Inspection verification method schema
- Field definitions and requirements
- Validation rules and common errors
- Complete examples for each schema type

**When to use**: Writing test case YAML files, understanding data structure requirements, debugging validation errors

---

### [Examples](examples/)

**Purpose**: Annotated examples demonstrating all verification methods and patterns

**Contents**:
- Fully annotated test case example
- Fully annotated analysis case example
- Fully annotated demonstration case example
- Fully annotated inspection case example
- Comprehensive filter usage examples
- Common patterns and best practices
- Tips for each verification method

**When to use**: Learning by example, using as templates, understanding best practices

## Verification Methods

The tool supports four verification methods, each with its own schema and typical use case:

### Test (`type: test`)

**Purpose**: Executable test cases with automated or manual steps

**Use when**: 
- Testing requires specific steps to execute
- Verification involves command execution
- Multiple test sequences needed
- Expected results can be specified

**Key features**:
- Multiple test sequences
- Steps with expected results
- Initial conditions (general and specific)
- Manual vs. automated step distinction

**Example**: `examples/test-case-example.yml`

---

### Analysis (`type: analysis`)

**Purpose**: Analytical verification through calculations and modeling

**Use when**:
- Verification requires mathematical analysis
- Performance or capacity needs calculation
- Statistical or probabilistic models needed
- Theoretical verification is appropriate

**Key features**:
- Mathematical calculations with formulas
- Analytical models (statistical, queuing, etc.)
- Parameters and acceptance criteria
- No executable steps

**Example**: `examples/analysis-case-example.yml`

---

### Demonstration (`type: demonstration`)

**Purpose**: Live demonstration of functionality

**Use when**:
- Verification best shown through live demo
- End-to-end user workflows need demonstration
- Visual or interactive verification needed
- Real-world scenario demonstration required

**Key features**:
- Step-by-step procedure
- Observations from demonstration
- Acceptance criteria
- No automated execution

**Example**: `examples/demonstration-case-example.yml`

---

### Inspection (`type: inspection`)

**Purpose**: Manual inspection and review

**Use when**:
- Code review or audit needed
- Compliance checking required
- Manual verification is appropriate
- Checklist-based verification needed

**Key features**:
- Detailed checklist of items to inspect
- Inspection method description
- Acceptance criteria
- No automated execution

**Example**: `examples/inspection-case-example.yml`

## Common Use Cases

### Use Case 1: Create Test Documentation

**Goal**: Generate test plan documentation from YAML test cases

**Steps**:
1. Write test cases using appropriate verification method (see [examples/](examples/))
2. Validate data structure against schema (see [schema-reference.md](schema-reference.md))
3. Create or use existing templates (see [template-authoring-guide.md](template-authoring-guide.md))
4. Run tool to generate documentation

**Resources**:
- [Schema Reference](schema-reference.md) - Data structure
- [Examples](examples/) - Starting templates
- Main README.md - CLI usage

---

### Use Case 2: Customize Template Output

**Goal**: Modify templates to change documentation format

**Steps**:
1. Understand template syntax (see [template-authoring-guide.md](template-authoring-guide.md))
2. Review available filters (see [filters-reference.md](filters-reference.md))
3. Modify template files
4. Test with example data

**Resources**:
- [Template Authoring Guide](template-authoring-guide.md) - Template syntax
- [Filters Reference](filters-reference.md) - Data transformations
- [Examples](examples/) - See filter usage in context

---

### Use Case 3: Add New Verification Method

**Goal**: Create new verification method type

**Steps**:
1. Define JSON schema for new type
2. Create template for rendering
3. Add example test cases
4. Update documentation

**Resources**:
- [Schema Reference](schema-reference.md) - Schema patterns
- [Template Authoring Guide](template-authoring-guide.md) - Template creation
- Existing schemas in `data/verification_methods/`

---

### Use Case 4: Validate Test Data

**Goal**: Ensure test case YAML files are valid

**Steps**:
1. Review schema requirements (see [schema-reference.md](schema-reference.md))
2. Write or modify YAML files
3. Run tool (automatic validation)
4. Fix any validation errors

**Resources**:
- [Schema Reference](schema-reference.md) - Validation rules
- [Examples](examples/) - Valid examples
- Error messages from tool

## Quick Reference

### Template Variables

**Container template receives**:
```jinja2
{{ date }}                    {# From container YAML #}
{{ product }}                 {# From container YAML #}
{{ test_cases_md }}           {# Rendered test cases #}
{{ requirements_summary_md }} {# If aggregation template exists #}
```

**Test case template receives**:
```jinja2
{{ type }}              {# Verification method type #}
{{ requirement }}       {# Requirement ID #}
{{ id }}                {# Test case ID #}
{{ description }}       {# Description text #}
{# Plus all other fields from YAML #}
```

### Custom Filters

```jinja2
{{ text | strip }}                                      {# Remove whitespace #}
{{ text | replace(old="X", new="Y") }}                  {# Replace string #}
{{ text | replace_regex(old="[0-9]+", new="NUM") }}     {# Regex replace #}
```

### Common Patterns

```jinja2
{# Clean YAML data #}
{{ field | strip }}

{# Remove prefix #}
{{ description | replace(old="MTD_", new="") }}

{# Create anchor ID #}
{{ id | replace_regex(old="[^a-zA-Z0-9_]", new="_") }}

{# Chain filters #}
{{ text | strip | lower | replace(old=" ", new="-") }}

{# Conditional #}
{% if step.manual %}Manual{% else %}Auto{% endif %}

{# Loop #}
{% for item in items %}
- {{ item | strip }}
{% endfor %}
```

## Best Practices

### For Template Authors

1. **Always use strip filter** on YAML text fields
2. **Handle optional fields** with conditionals
3. **Use whitespace control** (`-`) in loops and tables
4. **Test with real data** from examples
5. **Document template variables** in comments
6. **Follow existing patterns** in provided templates

### For Test Case Authors

1. **Validate against schema** before committing
2. **Use examples as templates** for consistency
3. **Be specific in descriptions** and criteria
4. **Include all required fields** per schema
5. **Use correct data types** (integer vs. string)
6. **Keep formatting consistent** across test cases

### For Both

1. **Review documentation** before starting
2. **Study examples** for patterns
3. **Test incrementally** during development
4. **Use version control** for changes
5. **Document deviations** from standards

## Troubleshooting

### Common Issues

**Validation fails with "required property" error**
- Check [Schema Reference](schema-reference.md) for required fields
- Ensure all required fields are present in YAML
- Verify field names match schema exactly

**Template rendering fails**
- Check [Template Authoring Guide](template-authoring-guide.md) for syntax
- Verify variable names exist in context
- Use conditionals for optional fields

**Unexpected output format**
- Review [Filters Reference](filters-reference.md) for transformations
- Check filter order in chaining
- Test filters individually

**Schema validation passes but template fails**
- Optional fields may be undefined in template
- Use `{% if field %}` before accessing
- Use `| default(value="")` filter

## Additional Resources

### In This Repository

- **Main README**: [../README.md](../README.md) - Overview and CLI usage
- **Agent Guide**: [../AGENTS.md](../AGENTS.md) - Build and test commands
- **Example Data**: `../data/` - Real test cases and schemas
- **Source Code**: `../src/main.rs` - Implementation reference

### External Resources

- **Tera Documentation**: https://tera.netlify.app/
- **JSON Schema**: https://json-schema.org/
- **YAML Specification**: https://yaml.org/spec/
- **Jinja2 (syntax reference)**: https://jinja.palletsprojects.com/

## Contributing to Documentation

When updating documentation:

1. **Keep examples current** with code changes
2. **Add new patterns** as they emerge
3. **Update cross-references** when reorganizing
4. **Test examples** to ensure validity
5. **Maintain consistent formatting** and style
6. **Add screenshots or diagrams** where helpful

## Summary

This documentation provides:

- **Complete template authoring guide** with syntax and patterns
- **Detailed filter reference** with examples and use cases
- **All JSON schema specifications** with validation rules
- **Annotated examples** for all verification methods
- **Best practices** and troubleshooting guidance
- **Quick references** for common tasks

Use these resources to:
- Author templates effectively
- Write valid test case data
- Understand the tool's capabilities
- Troubleshoot issues
- Learn by example

For questions or issues not covered here, refer to the main README.md or examine the source code in `src/main.rs`.
