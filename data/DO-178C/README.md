# DO-178C Templates for Test Plan Documentation Generator

This directory contains templates and schemas for generating DO-178C compliant software certification documentation using the Test Plan Documentation Generator (tpdg).

## Overview

DO-178C is a software development standard for safety-critical aviation systems. These templates help generate the required documentation artifacts for software certification.

## Directory Structure

```
DO-178C/
├── plans/              # Software planning documents
│   ├── PSAC/          # Plan for Software Aspects of Certification
│   ├── SDP/           # Software Development Plan
│   ├── SVP/           # Software Verification Plan
│   ├── SCMP/          # Software Configuration Management Plan
│   └── SQAP/          # Software Quality Assurance Plan
├── requirements/       # Requirements documents
│   ├── SRD/           # Software Requirements Data (High-Level Requirements)
│   └── SDD/           # Software Design Description (Low-Level Requirements)
├── standards/          # Development standards
│   ├── SRS/           # Software Requirements Standard
│   ├── SDS/           # Software Design Standard
│   └── SCS/           # Software Code Standard
├── verification/       # Verification documents
│   ├── SVCP/          # Software Verification Cases and Procedures
│   └── SVR/           # Software Verification Results
├── system/             # System-level documents
│   └── SR/            # System Requirements
└── SAS/                # Software Accomplishment Summary

```

## Document Descriptions

### Planning Documents

#### PSAC - Plan for Software Aspects of Certification
Links the applicant to certification authorities. Describes:
- System overview and functions
- Software overview and safety considerations
- Certification considerations (software level, failure conditions)
- Software lifecycle processes
- Software lifecycle data
- Schedule and supplier oversight

#### SDP - Software Development Plan
Describes software development procedures and lifecycle:
- Standards (requirements, design, coding)
- Software lifecycle processes and transition criteria
- Software development environment

#### SVP - Software Verification Plan
Describes verification strategy and procedures:
- Organization and independence
- Verification methods and environment
- Transition criteria
- Partitioning, compiler assumptions
- Reverification methods

#### SCMP - Software Configuration Management Plan
Establishes configuration management methods:
- Environment and activities
- Configuration identification, baseline, traceability
- Problem reporting and change control
- Status accounting and release management

#### SQAP - Software Quality Assurance Plan
Establishes quality assurance methods:
- Environment and authority
- QA activities and timing
- Transition criteria
- QA records and supplier oversight

### Requirements Documents

#### SRD - Software Requirements Data
High-level requirements that satisfy system requirements:
- System overview and allocated requirements
- Operational, functional, performance requirements
- Timing and memory requirements
- Hardware/software interfaces
- Failure detection and partitioning

#### SDD - Software Design Description
Software architecture and low-level requirements:
- Architecture and component allocation
- Interfaces and detailed design
- Dataflow and control flow
- Resource limitations and scheduling
- Derived requirements and safety-related design

### Standards Documents

#### SRS - Software Requirements Standard
Defines methods, notation, and tools for requirements development

#### SDS - Software Design Standard
Defines design methods, tools, architecture constraints, and complexity limitations

#### SCS - Software Code Standard
Defines programming language standards, coding rules, and complexity limitations

### Verification Documents

#### SVCP - Software Verification Cases and Procedures
Details test cases and procedures:
- Review and analysis procedures
- High-Level Tests (HLT) - black box tests
- Low-Level Tests (LLT) - unit tests
- Test environment and pass/fail criteria

#### SVR - Software Verification Results
Records verification results:
- Pass/fail status for reviews, analyses, tests
- Discrepancy tracking via problem reporting

### System Documents

#### SR - System Requirements
System-level requirements that software requirements trace to

### Summary Document

#### SAS - Software Accomplishment Summary
Primary compliance demonstration document:
- System and software overview
- Certification considerations
- Software lifecycle and lifecycle data
- Software identification and characteristics
- Change history and compliance statement

## Usage with tpdg

### Basic Document Generation

```bash
# Generate a Software Verification Plan
./target/release/tpdg \
  --output ./SVP.md \
  --container ./data/DO-178C/plans/SVP/schema.json \
             ./data/DO-178C/plans/SVP/template.j2 \
             ./data/DO-178C/plans/SVP/data.yml \
  --test-case ./data/DO-178C/verification \
              ./data/DO-178C/verification/test_cases/*.yml

# Generate a Software Requirements Document
./target/release/tpdg \
  --output ./SRD.md \
  --container ./data/DO-178C/requirements/SRD/schema.json \
             ./data/DO-178C/requirements/SRD/template.j2 \
             ./data/DO-178C/requirements/SRD/data.yml \
  --test-case ./data/DO-178C/verification \
              ./data/DO-178C/requirements/SRD/requirements/*.yml
```

### AsciiDoc Format

```bash
# Generate in AsciiDoc format
./target/release/tpdg \
  --format asciidoc \
  --output ./SVCP.adoc \
  --container ./data/DO-178C/verification/SVCP/schema.json \
             ./data/DO-178C/verification/SVCP/template_asciidoc.adoc \
             ./data/DO-178C/verification/SVCP/data.yml \
  --test-case ./data/DO-178C/verification \
              ./data/DO-178C/verification/test_procedures/*.yml
```

## DO-178C Compliance Levels

DO-178C defines five software levels based on failure condition severity:

- **Level A (Catastrophic)**: Failure may cause deaths
- **Level B (Hazardous)**: Failure may cause serious injuries
- **Level C (Major)**: Failure may cause discomfort to passengers/crew
- **Level D (Minor)**: Failure has no effect on aircraft operations
- **Level E (No Effect)**: Failure has no effect on safety

Different levels require different verification objectives and documentation depth.

## Customization

Each template can be customized by:

1. **Modifying the template files** (`.j2` or `_asciidoc.adoc`)
2. **Updating the data files** (`.yml`) with project-specific information
3. **Adjusting schemas** (`.json`) to enforce project requirements
4. **Adding verification methods** in the verification directory

## References

- DO-178C: Software Considerations in Airborne Systems and Equipment Certification
- DO-330: Software Tool Qualification Considerations
- DO-331: Model-Based Development and Verification Supplement
- DO-332: Object-Oriented Technology and Related Techniques Supplement
- DO-333: Formal Methods Supplement

## Notes

- All templates follow Tera (Jinja2-like) syntax
- Data validation is performed against JSON schemas
- Both Markdown and AsciiDoc output formats are supported
- Templates can include file references using `include_file()` function
