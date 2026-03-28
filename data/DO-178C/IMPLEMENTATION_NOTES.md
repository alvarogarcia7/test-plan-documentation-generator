# DO-178C Templates - Implementation Notes

## Overview

This implementation provides a comprehensive set of DO-178C templates for the Test Plan Documentation Generator (tpdg). The templates enable generation of aviation software certification documentation compliant with DO-178C standards.

## What's Implemented

### Planning Documents (5 documents)

1. **PSAC** - Plan for Software Aspects of Certification
   - Complete schema with all required DO-178C sections
   - Markdown and AsciiDoc templates
   - Example data file with comprehensive Flight Management System example
   - Location: `data/DO-178C/plans/PSAC/`

2. **SVP** - Software Verification Plan
   - Schema covering organization, verification methods, environment
   - Templates supporting test case injection via `test_cases_md`
   - Example data with Level A verification approach
   - Location: `data/DO-178C/plans/SVP/`

3. **SDP** - Software Development Plan (directory created, templates TBD)
4. **SCMP** - Software Configuration Management Plan (directory created, templates TBD)
5. **SQAP** - Software Quality Assurance Plan (directory created, templates TBD)

### Verification Documents (2 documents)

1. **SVCP** - Software Verification Cases and Procedures
   - Schema for document-level data
   - Templates that inject rendered test cases
   - Review and analysis procedure sections
   - Test overview sections
   - Location: `data/DO-178C/verification/SVCP/`

2. **SVR** - Software Verification Results (directory created, templates TBD)

### Test Case Types (2 types)

1. **High-Level Test (HLT)**
   - Schema for requirements-based black-box tests
   - Markdown and AsciiDoc templates
   - Support for test procedures, prerequisites, pass criteria
   - Traceability to requirements and design components
   - Location: `data/DO-178C/verification/verification_methods/high_level_test/`
   - Example test cases: HLT-NAV-001.yml, HLT-NAV-002.yml

2. **Low-Level Test (LLT)**
   - Schema for unit/white-box tests
   - Templates with test setup, inputs, outputs
   - Support for stubs and coverage requirements (MC/DC, decision, statement)
   - Traceability to low-level and high-level requirements
   - Location: `data/DO-178C/verification/verification_methods/low_level_test/`
   - Example test cases: LLT-NAV-001.yml, LLT-NAV-002.yml, LLT-NAV-003.yml

### Summary Document

1. **SAS** - Software Accomplishment Summary
   - Complete schema with compliance tracking
   - Templates for certification statement
   - Lifecycle data tracking
   - Change history and open issues tracking
   - Location: `data/DO-178C/SAS/`

### Documentation

1. **README.md** - Comprehensive overview of DO-178C templates
2. **USAGE_EXAMPLES.md** - Practical command-line examples
3. **QUICK_REFERENCE.md** - Quick reference guide for DO-178C
4. **IMPLEMENTATION_NOTES.md** - This file

## Directory Structure

```
data/DO-178C/
├── README.md                    # Main documentation
├── USAGE_EXAMPLES.md           # Usage examples
├── QUICK_REFERENCE.md          # Quick reference
├── IMPLEMENTATION_NOTES.md     # Implementation details
│
├── plans/                      # Planning documents
│   ├── PSAC/                  # Plan for Software Aspects of Certification
│   │   ├── schema.json        # JSON schema for validation
│   │   ├── template.j2        # Markdown template
│   │   ├── template_asciidoc.adoc  # AsciiDoc template
│   │   └── data.yml           # Example data
│   ├── SVP/                   # Software Verification Plan
│   │   ├── schema.json
│   │   ├── template.j2
│   │   ├── template_asciidoc.adoc
│   │   └── data.yml
│   ├── SDP/                   # (Directory created)
│   ├── SCMP/                  # (Directory created)
│   └── SQAP/                  # (Directory created)
│
├── verification/               # Verification documents
│   ├── SVCP/                  # Software Verification Cases and Procedures
│   │   ├── schema.json
│   │   ├── template.j2
│   │   ├── template_asciidoc.adoc
│   │   └── data.yml
│   ├── SVR/                   # (Directory created)
│   ├── verification_methods/   # Verification method templates
│   │   ├── high_level_test/   # High-level test templates
│   │   │   ├── schema.json
│   │   │   ├── template.j2
│   │   │   └── template_asciidoc.adoc
│   │   ├── low_level_test/    # Low-level test templates
│   │   │   ├── schema.json
│   │   │   ├── template.j2
│   │   │   └── template_asciidoc.adoc
│   │   ├── review/            # (Directory created)
│   │   └── analysis/          # (Directory created)
│   └── test_procedures/        # Example test case data
│       ├── HLT-NAV-001.yml    # High-level test example 1
│       ├── HLT-NAV-002.yml    # High-level test example 2
│       ├── LLT-NAV-001.yml    # Low-level test example 1
│       ├── LLT-NAV-002.yml    # Low-level test example 2
│       └── LLT-NAV-003.yml    # Low-level test example 3
│
└── SAS/                        # Software Accomplishment Summary
    ├── schema.json
    ├── template.j2
    ├── template_asciidoc.adoc
    └── data.yml
```

## Key Features

### 1. Schema Validation
- All documents have JSON schemas for data validation
- Schemas enforce DO-178C required fields
- Enum constraints for software levels (A-E)
- Enum constraints for failure classifications

### 2. Dual Format Support
- Every document template available in both Markdown (.j2) and AsciiDoc (.adoc)
- Consistent structure across formats
- Easy conversion to PDF via pandoc or asciidoctor-pdf

### 3. Test Case Integration
- Test cases are separate YAML files
- Rendered and injected via `test_cases_md` variable
- Supports multiple test case types (HLT, LLT)
- Extensible to additional verification methods

### 4. Traceability
- Test cases include traceability fields
- Requirements IDs link to SRD/SDD
- Design components linked to architecture
- Bi-directional traceability support

### 5. DO-178C Compliance
- Software levels A-E supported
- Failure condition classifications
- Coverage requirements (MC/DC, decision, statement)
- All key DO-178C document types

### 6. Realistic Examples
- Flight Management System (FMS) as example project
- Navigation subsystem test cases
- Level A software verification examples
- Practical verification procedures

## Usage Pattern

The typical usage pattern is:

1. **Select Document Type**: Choose which DO-178C document to generate (PSAC, SVP, SVCP, etc.)

2. **Customize Data**: Edit or create a data.yml file with project-specific information

3. **Add Test Cases** (for SVCP): Create test case YAML files following the schema

4. **Run tpdg**: Execute tpdg with appropriate command-line arguments

5. **Review Output**: Review generated Markdown or AsciiDoc

6. **Convert to PDF** (optional): Use pandoc or asciidoctor-pdf for final format

Example:
```bash
./target/release/tpdg \
  --output ./SVCP.md \
  --container ./data/DO-178C/verification/SVCP/schema.json \
             ./data/DO-178C/verification/SVCP/template.j2 \
             ./data/DO-178C/verification/SVCP/data.yml \
  --test-case ./data/DO-178C/verification/verification_methods \
              ./data/DO-178C/verification/test_procedures/*.yml
```

## Design Decisions

### 1. Separation of Container and Test Cases
- Container documents (PSAC, SVP, SVCP, SAS) are separate from test cases
- Test cases are modular and reusable
- Container templates inject test cases via `test_cases_md`
- This matches DO-178C practice of referencing test procedures

### 2. Schema-First Approach
- JSON schemas define data structure
- Validation happens before rendering
- Catches errors early in the process
- Provides clear documentation of required fields

### 3. Dual Format Templates
- Both Markdown and AsciiDoc provided
- Markdown for readability and version control
- AsciiDoc for professional PDF generation
- Consistent content across formats

### 4. Practical Examples
- Realistic Flight Management System example
- Navigation subsystem (common in avionics)
- Multiple test types (normal, boundary, error handling)
- Demonstrates traceability and coverage

### 5. Extensibility
- Additional document types can be added (SDP, SCMP, SQAP)
- New verification methods (review, analysis) can be added
- Schemas can be extended with custom fields
- Templates can be customized per organization

## Testing the Templates

To test the templates:

1. **Build tpdg**:
   ```bash
   cargo build --release
   ```

2. **Generate SVCP with test cases**:
   ```bash
   ./target/release/tpdg \
     --output /tmp/SVCP-test.md \
     --container ./data/DO-178C/verification/SVCP/schema.json \
                ./data/DO-178C/verification/SVCP/template.j2 \
                ./data/DO-178C/verification/SVCP/data.yml \
     --test-case ./data/DO-178C/verification/verification_methods \
                 ./data/DO-178C/verification/test_procedures/HLT-NAV-001.yml \
                 ./data/DO-178C/verification/test_procedures/LLT-NAV-001.yml
   ```

3. **Review output**:
   ```bash
   cat /tmp/SVCP-test.md
   ```

4. **Validate schema**:
   The tool will automatically validate data.yml against schema.json

## Future Enhancements

The following could be added in the future:

1. **Additional Documents**:
   - SDP (Software Development Plan)
   - SCMP (Software Configuration Management Plan)
   - SQAP (Software Quality Assurance Plan)
   - SRD (Software Requirements Data)
   - SDD (Software Design Description)
   - SRS, SDS, SCS (Standards documents)

2. **Additional Verification Methods**:
   - Review templates and schemas
   - Analysis templates and schemas
   - Demonstration templates and schemas
   - Inspection templates and schemas

3. **Requirements Traceability**:
   - Requirement aggregation templates
   - Traceability matrix generation
   - Gap analysis support

4. **Tool Qualification**:
   - Tool qualification data templates
   - DO-330 compliance documentation

5. **Change Management**:
   - Problem report tracking
   - Change request templates
   - Impact analysis templates

## Integration with Existing Project

These DO-178C templates are designed to work alongside the existing templates in the project:

- Located in separate `data/DO-178C/` directory
- Uses same tpdg tool and command-line interface
- Uses same schema validation approach
- Uses same template engine (Tera)
- Compatible with existing verification methods structure

## Compliance Notes

These templates provide a starting point for DO-178C documentation. Each certification project should:

1. **Review with Certification Authority**: Ensure templates meet specific authority requirements
2. **Customize for Organization**: Add organization-specific sections and policies
3. **Tailor by Software Level**: Adjust content based on software level (A, B, C, D, E)
4. **Maintain Traceability**: Ensure all requirements are traced and tested
5. **Follow Standards**: Adhere to organization coding, design, and requirements standards

## References

- DO-178C: Software Considerations in Airborne Systems and Equipment Certification (RTCA, 2011)
- DO-330: Software Tool Qualification Considerations (RTCA, 2011)
- DO-331: Model-Based Development and Verification Supplement (RTCA, 2011)
- DO-332: Object-Oriented Technology and Related Techniques Supplement (RTCA, 2011)
- DO-333: Formal Methods Supplement (RTCA, 2011)

## Acknowledgments

Template structure inspired by:
- https://github.com/strictdoc-project/strictdoc-templates (DO-178C outline)
- Real-world DO-178C certification projects
- Aviation software development best practices
