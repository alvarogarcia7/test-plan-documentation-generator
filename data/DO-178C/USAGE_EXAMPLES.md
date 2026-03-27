# DO-178C Templates - Usage Examples

This document provides practical examples of how to use the DO-178C templates with the Test Plan Documentation Generator (tpdg).

## Prerequisites

Build the tpdg tool:
```bash
cargo build --release
```

The binary will be available at `./target/release/tpdg`.

## Example 1: Generate Software Verification Plan (SVP)

The SVP is a planning document that doesn't necessarily include test cases, but can reference verification procedures.

```bash
./target/release/tpdg \
  --output ./output/SVP.md \
  --container ./data/DO-178C/plans/SVP/schema.json \
             ./data/DO-178C/plans/SVP/template.j2 \
             ./data/DO-178C/plans/SVP/data.yml \
  --test-case ./data/DO-178C/verification/verification_methods
```

For AsciiDoc format:
```bash
./target/release/tpdg \
  --format asciidoc \
  --output ./output/SVP.adoc \
  --container ./data/DO-178C/plans/SVP/schema.json \
             ./data/DO-178C/plans/SVP/template_asciidoc.adoc \
             ./data/DO-178C/plans/SVP/data.yml \
  --test-case ./data/DO-178C/verification/verification_methods
```

## Example 2: Generate Plan for Software Aspects of Certification (PSAC)

```bash
./target/release/tpdg \
  --output ./output/PSAC.md \
  --container ./data/DO-178C/plans/PSAC/schema.json \
             ./data/DO-178C/plans/PSAC/template.j2 \
             ./data/DO-178C/plans/PSAC/data.yml \
  --test-case ./data/DO-178C/verification/verification_methods
```

## Example 3: Generate Software Verification Cases and Procedures (SVCP)

This document contains actual test cases and procedures. It demonstrates the full power of tpdg by combining a container document with multiple test case files.

### Markdown Output

```bash
./target/release/tpdg \
  --output ./output/SVCP.md \
  --container ./data/DO-178C/verification/SVCP/schema.json \
             ./data/DO-178C/verification/SVCP/template.j2 \
             ./data/DO-178C/verification/SVCP/data.yml \
  --test-case ./data/DO-178C/verification/verification_methods \
              ./data/DO-178C/verification/test_procedures/HLT-NAV-001.yml \
              ./data/DO-178C/verification/test_procedures/HLT-NAV-002.yml \
              ./data/DO-178C/verification/test_procedures/LLT-NAV-001.yml \
              ./data/DO-178C/verification/test_procedures/LLT-NAV-002.yml \
              ./data/DO-178C/verification/test_procedures/LLT-NAV-003.yml
```

### AsciiDoc Output

```bash
./target/release/tpdg \
  --format asciidoc \
  --output ./output/SVCP.adoc \
  --container ./data/DO-178C/verification/SVCP/schema.json \
             ./data/DO-178C/verification/SVCP/template_asciidoc.adoc \
             ./data/DO-178C/verification/SVCP/data.yml \
  --test-case ./data/DO-178C/verification/verification_methods \
              ./data/DO-178C/verification/test_procedures/HLT-NAV-001.yml \
              ./data/DO-178C/verification/test_procedures/HLT-NAV-002.yml \
              ./data/DO-178C/verification/test_procedures/LLT-NAV-001.yml \
              ./data/DO-178C/verification/test_procedures/LLT-NAV-002.yml \
              ./data/DO-178C/verification/test_procedures/LLT-NAV-003.yml
```

## Example 4: Generate Software Accomplishment Summary (SAS)

```bash
./target/release/tpdg \
  --output ./output/SAS.md \
  --container ./data/DO-178C/SAS/schema.json \
             ./data/DO-178C/SAS/template.j2 \
             ./data/DO-178C/SAS/data.yml \
  --test-case ./data/DO-178C/verification/verification_methods
```

## Example 5: Generate All Documents (Batch Processing)

Create a script to generate all DO-178C documents:

```bash
#!/bin/bash

OUTPUT_DIR="./output/DO-178C"
mkdir -p "$OUTPUT_DIR"

# Generate PSAC
./target/release/tpdg \
  --output "$OUTPUT_DIR/PSAC.md" \
  --container ./data/DO-178C/plans/PSAC/schema.json \
             ./data/DO-178C/plans/PSAC/template.j2 \
             ./data/DO-178C/plans/PSAC/data.yml \
  --test-case ./data/DO-178C/verification/verification_methods

# Generate SVP
./target/release/tpdg \
  --output "$OUTPUT_DIR/SVP.md" \
  --container ./data/DO-178C/plans/SVP/schema.json \
             ./data/DO-178C/plans/SVP/template.j2 \
             ./data/DO-178C/plans/SVP/data.yml \
  --test-case ./data/DO-178C/verification/verification_methods

# Generate SVCP with test cases
./target/release/tpdg \
  --output "$OUTPUT_DIR/SVCP.md" \
  --container ./data/DO-178C/verification/SVCP/schema.json \
             ./data/DO-178C/verification/SVCP/template.j2 \
             ./data/DO-178C/verification/SVCP/data.yml \
  --test-case ./data/DO-178C/verification/verification_methods \
              ./data/DO-178C/verification/test_procedures/*.yml

# Generate SAS
./target/release/tpdg \
  --output "$OUTPUT_DIR/SAS.md" \
  --container ./data/DO-178C/SAS/schema.json \
             ./data/DO-178C/SAS/template.j2 \
             ./data/DO-178C/SAS/data.yml \
  --test-case ./data/DO-178C/verification/verification_methods

echo "All DO-178C documents generated in $OUTPUT_DIR"
```

## Customizing Templates

### Adding Your Own Test Cases

1. Create a new YAML file in `data/DO-178C/verification/test_procedures/`
2. Use the appropriate type: `high_level_test` or `low_level_test`
3. Follow the schema defined in the verification method directory
4. Add the file to the command line when generating SVCP

Example high-level test:
```yaml
type: "high_level_test"
test_id: "HLT-GUID-001"
requirement_id: "SRS-GUID-200"
test_objective: "Verify guidance controller computes correct steering commands"
test_level: "Integration"
prerequisites:
  - "FMS initialized"
  - "Navigation data valid"
test_procedure:
  - step: 1
    action: "Set target waypoint"
    input: "Waypoint coordinates"
    expected_output: "Steering command generated"
pass_criteria: "Steering commands within ±0.5° of expected"
```

### Modifying Document Templates

1. Edit the `.j2` (Markdown) or `_asciidoc.adoc` (AsciiDoc) template files
2. Use Tera (Jinja2-like) syntax for variables and control flow
3. Available variables are defined in the schema and data YAML files
4. Test your changes by regenerating the document

### Customizing Data

1. Edit the `data.yml` file for the document you want to customize
2. Ensure your data conforms to the JSON schema
3. The schema validation will catch any errors

## Converting to PDF

### From Markdown

Use pandoc:
```bash
pandoc output/SVCP.md -o output/SVCP.pdf \
  --toc \
  --number-sections \
  -V geometry:margin=1in
```

### From AsciiDoc

Use asciidoctor-pdf:
```bash
asciidoctor-pdf output/SVCP.adoc -o output/SVCP.pdf
```

## Integration with CI/CD

Add document generation to your CI/CD pipeline:

```yaml
# .gitlab-ci.yml example
generate-docs:
  stage: documentation
  script:
    - cargo build --release
    - ./scripts/generate_do178c_docs.sh
  artifacts:
    paths:
      - output/DO-178C/*.md
      - output/DO-178C/*.adoc
    expire_in: 30 days
```

## Tips and Best Practices

### 1. Version Control
- Keep template files in version control
- Version your data files separately from templates
- Use semantic versioning for document versions

### 2. Traceability
- Maintain consistent requirement IDs across all documents
- Use the traceability fields in test cases
- Generate traceability matrices from the data

### 3. Review Process
- Review generated documents before releasing
- Keep approval signatures in a separate document or system
- Track document changes in the change history

### 4. Automation
- Automate document generation in your build process
- Generate documents on every release
- Archive generated documents with software releases

### 5. Customization
- Create organization-specific templates by copying and modifying
- Add custom sections as needed for your certification authority
- Maintain a style guide for consistency

## Troubleshooting

### Schema Validation Errors

If you see validation errors:
1. Check that your YAML file structure matches the JSON schema
2. Verify all required fields are present
3. Check that enum values match exactly (case-sensitive)

### Template Rendering Errors

If templates fail to render:
1. Verify variable names match those in the data YAML
2. Check for syntax errors in template conditionals
3. Ensure all referenced variables exist or use conditionals

### Missing Test Cases

If test cases don't appear:
1. Verify the test case YAML files have the correct `type` field
2. Check that corresponding template exists in verification_methods
3. Ensure files are listed on the command line

## Additional Resources

- DO-178C Standard: Software Considerations in Airborne Systems and Equipment Certification
- Tera Template Documentation: https://tera.netlify.app/
- JSON Schema Validation: https://json-schema.org/
- AsciiDoc Syntax: https://docs.asciidoctor.org/asciidoc/latest/

## Support

For issues with the tpdg tool itself, see the main README.md in the project root.

For DO-178C template-specific questions, refer to the DO-178C standard and your certification authority's guidance.
