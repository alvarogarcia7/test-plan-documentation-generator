# DO-178C Templates for StrictDoc

This directory contains DO-178C compliant documentation templates in StrictDoc format for aviation software certification.

## Overview

DO-178C is the primary standard for safety-critical aviation software development. These templates provide a complete set of documentation artifacts required for software certification, formatted for use with [StrictDoc](https://github.com/strictdoc-project/strictdoc) - a requirements management and documentation tool.

## What is StrictDoc?

StrictDoc is a software requirements specification and documentation tool that uses a text-based DSL (Domain-Specific Language) for managing requirements and technical documentation. It provides:

- Requirements management with unique IDs and traceability
- Documentation generation in multiple formats (HTML, PDF, Excel)
- Version control friendly (plain text format)
- Customizable grammars for domain-specific documentation
- Automatic traceability matrices and coverage reports

## Directory Structure

```
strictdoc/DO-178C/
├── grammar.sgra                    # DO-178C grammar definition
├── README.md                       # This file
│
├── plans/                          # Planning Documents
│   ├── PSAC/
│   │   └── PSAC.sdoc              # Plan for Software Aspects of Certification
│   └── SVP/
│       └── SVP.sdoc               # Software Verification Plan
│
├── requirements/                   # Requirements Documents
│   ├── SRD/
│   │   └── SRD.sdoc               # Software Requirements Data
│   └── SDD/
│       └── SDD.sdoc               # Software Design Description (TBD)
│
├── verification/                   # Verification Documents
│   └── SVCP/
│       └── SVCP.sdoc              # Software Verification Cases and Procedures
│
└── SAS/
    └── SAS.sdoc                   # Software Accomplishment Summary
```

## Document Descriptions

### Planning Documents

#### PSAC - Plan for Software Aspects of Certification
**File:** `plans/PSAC/PSAC.sdoc`

Links the applicant to certification authorities. Describes:
- System overview and functions
- Software overview and component design assurance levels
- Certification considerations (failure conditions, software level)
- Software lifecycle processes
- Software lifecycle data items and schedule
- Supplier oversight and additional considerations

#### SVP - Software Verification Plan
**File:** `plans/SVP/SVP.sdoc`

Describes verification strategy and procedures:
- Organization and independence requirements
- Verification methods (reviews, analysis, test, demonstration, inspection)
- Verification environment (hardware, software tools)
- Transition criteria between lifecycle phases
- Partitioning verification approach
- Compiler assumptions and reverification methods

### Requirements Documents

#### SRD - Software Requirements Data
**File:** `requirements/SRD/SRD.sdoc`

High-level software requirements that satisfy system requirements:
- Functional requirements (position computation, data validation, navigation modes)
- Performance requirements (execution time, stack usage, memory)
- Interface requirements (GPS, IRS, outputs)
- Safety requirements (fail-safe behavior, watchdog, memory protection)
- Traceability to system requirements

### Verification Documents

#### SVCP - Software Verification Cases and Procedures
**File:** `verification/SVCP/SVCP.sdoc`

Details test cases and procedures:
- Review and analysis procedures
- High-Level Tests (requirements-based black-box tests)
- Low-Level Tests (unit/white-box tests with MC/DC coverage)
- Test environment description
- Pass/fail criteria and coverage analysis
- Traceability matrix

### Summary Document

#### SAS - Software Accomplishment Summary
**File:** `SAS/SAS.sdoc`

Primary compliance demonstration document:
- Software identification (part number, version, checksum)
- System and software overview
- Certification considerations and software level
- Software lifecycle processes
- Lifecycle data summary (all documents)
- Compliance statement (objectives satisfied)
- Change history and verification status

## Grammar Definition

The `grammar.sgra` file defines custom element types for DO-178C documentation:

- **PLAN**: Planning items with level, status, and statement
- **REQUIREMENT**: Software requirements with verification method and rationale
- **TEST_CASE**: High-level test cases with prerequisites and procedures
- **LOW_LEVEL_TEST**: Unit test cases with inputs, outputs, and coverage
- **SECTION**: Document sections for organization
- **TEXT**: Free-form text content

## Using These Templates

### Prerequisites

1. Install StrictDoc:
   ```bash
   pip install strictdoc
   ```

2. Clone or download this repository

### Viewing Documentation

Generate HTML documentation from any `.sdoc` file:

```bash
# Generate HTML for a single document
strictdoc export plans/PSAC/PSAC.sdoc

# Generate HTML for entire DO-178C documentation set
strictdoc export .

# Open the generated HTML
open output/html/index.html
```

### Generating PDF

Generate PDF documentation:

```bash
# Requires Sphinx and LaTeX
strictdoc export --formats=pdf plans/PSAC/PSAC.sdoc
```

### Customization

1. **Edit the grammar** (`grammar.sgra`) to add custom fields or element types
2. **Modify document templates** (`.sdoc` files) to match your project needs
3. **Update requirement UIDs** to match your project's naming convention
4. **Add traceability** by referencing UIDs in `COMMENT` or `RATIONALE` fields

### Creating New Documents

Use existing documents as templates:

```bash
# Copy an existing document
cp plans/SVP/SVP.sdoc plans/SDP/SDP.sdoc

# Edit the new document
# Update TITLE, UID, VERSION, and content
```

## DO-178C Compliance Levels

DO-178C defines five software levels based on failure condition severity:

| Level | Classification | Failure Effect | Objectives |
|-------|----------------|----------------|------------|
| **A** | Catastrophic | Loss of aircraft or multiple fatalities | 71 objectives |
| **B** | Hazardous | Serious injuries or significant reduction in safety margins | 69 objectives |
| **C** | Major | Discomfort to passengers/crew or increase in workload | 62 objectives |
| **D** | Minor | Minor impact on aircraft operations | 26 objectives |
| **E** | No Effect | No impact on aircraft or crew | 0 objectives |

These templates are configured for Level A (most stringent) requirements.

## Key Features

### Requirements Traceability

All requirements include unique IDs (UIDs) for traceability:

```
[REQUIREMENT]
UID: SRS-NAV-100
LEVEL: A
VERIFICATION: Test, Analysis
TITLE: GPS Position Computation
STATEMENT: >>>
The Navigation Manager shall compute aircraft position...
<<<
```

### Test Coverage Tracking

Test cases link to requirements:

```
[TEST_CASE]
UID: HLT-NAV-001
TITLE: GPS Position Computation Verification
...
PASS_CRITERIA: >>>
Verifies requirements: SRS-NAV-100, SRS-NAV-101
<<<
```

### Multi-Level Documentation

Documents support hierarchical sections:

```
[SECTION]
TITLE: 2. Verification Methods

[SECTION]
TITLE: 2.1 Reviews
...
[/SECTION]

[SECTION]
TITLE: 2.2 Analysis
...
[/SECTION]

[/SECTION]
```

### Structured Data

Custom grammar elements capture domain-specific information:

- Software levels (A, B, C, D, E)
- Verification methods (Test, Analysis, Review, etc.)
- Test types (Normal Range, Boundary, Error Handling, etc.)
- Coverage requirements (Statement, Decision, MC/DC)

## Example: Flight Management System

These templates use a realistic Flight Management System (FMS) as an example:

- **Navigation Manager**: Level A component for position computation
- **Guidance Controller**: Level A component for flight path computation
- **Performance Calculator**: Level B component for optimization
- **Database Manager**: Level C component for navigation data
- **Display Interface**: Level C component for pilot interface

### Sample Requirements

- SRS-NAV-100: GPS position computation (±10 feet accuracy)
- SRS-NAV-110: Multi-sensor fusion (GPS + IRS blending)
- SRS-NAV-130: Coordinate validation (range checking)
- SRS-NAV-200: Execution time (5ms WCET)
- SRS-NAV-400: Fail-safe behavior

### Sample Test Cases

- HLT-NAV-001: GPS position computation verification
- HLT-NAV-002: Multi-sensor navigation verification
- LLT-NAV-001: Position computation unit test (normal range)
- LLT-NAV-002: GPS invalid error handling test
- LLT-NAV-003: Coordinate validation boundary test

## Verification Methods

### Reviews
Inspection of requirements, design, code, and test procedures against standards and checklists.

### Analysis
- Traceability analysis (requirements to tests)
- Structural coverage analysis (MC/DC for Level A)
- Data/control flow analysis (dead code, uninitialized variables)
- Timing analysis (WCET)
- Stack usage analysis

### Testing
- **High-Level Tests (HLT)**: Requirements-based black-box tests on integrated software
- **Low-Level Tests (LLT)**: Unit/white-box tests with coverage instrumentation
- **Integration Tests**: Software-hardware integration verification
- **Robustness Tests**: Error handling and boundary conditions

### Demonstration
Execution in target environment for features requiring human observation.

### Inspection
Visual examination of build procedures, configuration data, and object code.

## Traceability

StrictDoc automatically generates traceability matrices showing:

- System requirements → Software requirements
- Software requirements → Test cases
- Software requirements → Low-level requirements
- Test cases → Verification results
- Requirements → Design components

## Best Practices

1. **Unique UIDs**: Use consistent UID naming convention (e.g., SRS-NAV-###)
2. **Version Control**: Keep `.sdoc` files in Git for change tracking
3. **Reviews**: Review generated HTML/PDF to verify formatting
4. **Validation**: Use StrictDoc's validation features to check document structure
5. **Incremental Updates**: Update documentation incrementally with code changes
6. **Traceability Links**: Maintain traceability in COMMENT and RATIONALE fields

## Advanced Features

### Custom Grammar Extensions

Extend `grammar.sgra` to add custom fields:

```
[ELEMENT]
NAME: HAZARD
FIELDS:
- TITLE: UID
  TYPE: String
  REQUIRED: True
- TITLE: SEVERITY
  TYPE: String
  REQUIRED: True
- TITLE: MITIGATION
  TYPE: String
  REQUIRED: True
```

### Document Includes

Reference external documents or sections:

```
[SECTION]
TITLE: 5. External Reference

[TEXT]
STATEMENT: >>>
See System Requirements Specification (SyRS) Section 3.2
for hardware interface details.
<<<

[/SECTION]
```

### Filtering and Views

Generate filtered views by software level:

```bash
# Generate documentation for Level A components only
strictdoc export --filter="LEVEL:A" .
```

## Certification Notes

These templates provide a starting point for DO-178C certification:

1. **Customize for your project**: Update UIDs, names, and content
2. **Review with DER**: Have a Designated Engineering Representative review
3. **Tailor by software level**: Adjust objectives based on software level (A-E)
4. **Maintain CM**: Keep documentation under configuration management
5. **Update continuously**: Keep docs synchronized with code changes

## Support and Resources

### StrictDoc Resources
- StrictDoc GitHub: https://github.com/strictdoc-project/strictdoc
- StrictDoc Documentation: https://strictdoc.readthedocs.io/

### DO-178C Resources
- DO-178C Standard (RTCA, 2011)
- DO-330: Software Tool Qualification Considerations
- DO-331: Model-Based Development and Verification Supplement
- DO-333: Formal Methods Supplement

### Template Repository
- StrictDoc Templates: https://github.com/strictdoc-project/strictdoc-templates

## License

These templates are provided as examples for DO-178C documentation. Customize and adapt them for your specific certification needs.

## Contributing

Contributions and improvements to these templates are welcome:
1. Fork the repository
2. Create a feature branch
3. Make your improvements
4. Submit a pull request

## Changelog

- **v1.0** (2024-03-27): Initial release
  - PSAC, SVP, SRD, SVCP, SAS documents
  - DO-178C grammar definition
  - Flight Management System example
  - Level A requirements and test cases
