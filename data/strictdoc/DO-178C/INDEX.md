# DO-178C StrictDoc Templates - Index

## Documentation

| Document | Description |
|----------|-------------|
| [README.md](README.md) | Comprehensive overview and documentation |
| [QUICK_START.md](QUICK_START.md) | Quick start guide and common commands |
| [INDEX.md](INDEX.md) | This file - complete index |

## Grammar and Configuration

| File | Description |
|------|-------------|
| [grammar.sgra](grammar.sgra) | DO-178C grammar definition for StrictDoc |

## Planning Documents

| Document | File | Status | Description |
|----------|------|--------|-------------|
| PSAC | [plans/PSAC/PSAC.sdoc](plans/PSAC/PSAC.sdoc) | ✅ Complete | Plan for Software Aspects of Certification |
| SVP | [plans/SVP/SVP.sdoc](plans/SVP/SVP.sdoc) | ✅ Complete | Software Verification Plan |
| SDP | plans/SDP/ | 📋 Planned | Software Development Plan |
| SCMP | plans/SCMP/ | 📋 Planned | Software Configuration Management Plan |
| SQAP | plans/SQAP/ | 📋 Planned | Software Quality Assurance Plan |

## Requirements Documents

| Document | File | Status | Description |
|----------|------|--------|-------------|
| SRD | [requirements/SRD/SRD.sdoc](requirements/SRD/SRD.sdoc) | ✅ Complete | Software Requirements Data (Navigation Manager) |
| SDD | requirements/SDD/ | 📋 Planned | Software Design Description |

## Verification Documents

| Document | File | Status | Description |
|----------|------|--------|-------------|
| SVCP | [verification/SVCP/SVCP.sdoc](verification/SVCP/SVCP.sdoc) | ✅ Complete | Software Verification Cases and Procedures |
| SVR | verification/SVR/ | 📋 Planned | Software Verification Results |

## Summary Documents

| Document | File | Status | Description |
|----------|------|--------|-------------|
| SAS | [SAS/SAS.sdoc](SAS/SAS.sdoc) | ✅ Complete | Software Accomplishment Summary |

## Document Statistics

### PSAC (Plan for Software Aspects of Certification)
- **Elements**: 15 PLAN elements
- **Sections**: 9 major sections
- **Coverage**: System overview, software components, certification considerations, lifecycle, schedule, suppliers

### SVP (Software Verification Plan)
- **Elements**: 9 PLAN elements
- **Sections**: 8 major sections
- **Coverage**: Organization, verification methods, environment, transition criteria, partitioning, reverification

### SRD (Software Requirements Data)
- **Elements**: 20 REQUIREMENT elements
- **Sections**: 6 major sections
- **Coverage**: Functional requirements, performance, interfaces, safety
- **Traceability**: All requirements link to system requirements

### SVCP (Software Verification Cases and Procedures)
- **Elements**: 5 TEST_CASE + 3 LOW_LEVEL_TEST elements
- **Sections**: 7 major sections
- **Coverage**: Review procedures, test overview, high-level tests, low-level tests, coverage analysis

### SAS (Software Accomplishment Summary)
- **Elements**: 15 PLAN elements (lifecycle data)
- **Sections**: 12 major sections
- **Coverage**: Software identification, compliance statement, verification status, approval

## Requirements Coverage

### Software Requirements (SRD)
- Total Requirements: 20
- Level A Requirements: 20
- Approved Requirements: 20
- Draft Requirements: 0

### Requirement Categories
- Functional Requirements: 9
- Performance Requirements: 3
- Interface Requirements: 6
- Safety Requirements: 3

### Verification Methods
- Test: 18 requirements
- Analysis: 8 requirements
- Review: 0 requirements (applied to all via reviews)

## Test Coverage

### High-Level Tests (Requirements-Based)
- Total HLT Cases: 2
- Requirements Coverage: SRS-NAV-100, 101, 110, 111, 115, 120

### Low-Level Tests (Unit Tests)
- Total LLT Cases: 3
- Test Types:
  - Normal Range: 1
  - Error Handling: 1
  - Boundary: 1
- Coverage: 100% MC/DC required and achieved

## Traceability Matrix

### Forward Traceability (Requirements → Tests)

| Requirement | Test Cases |
|-------------|------------|
| SRS-NAV-100 | HLT-NAV-001, LLT-NAV-001 |
| SRS-NAV-101 | HLT-NAV-001 |
| SRS-NAV-110 | HLT-NAV-002, LLT-NAV-001 |
| SRS-NAV-111 | HLT-NAV-002 |
| SRS-NAV-115 | HLT-NAV-002, LLT-NAV-002 |
| SRS-NAV-120 | HLT-NAV-002, LLT-NAV-002 |
| SRS-NAV-130 | LLT-NAV-003 |
| SRS-NAV-131 | LLT-NAV-003 |

### Backward Traceability (Tests → Requirements)

| Test Case | Requirements Verified |
|-----------|----------------------|
| HLT-NAV-001 | SRS-NAV-100, SRS-NAV-101 |
| HLT-NAV-002 | SRS-NAV-110, SRS-NAV-111, SRS-NAV-115, SRS-NAV-120 |
| LLT-NAV-001 | SRS-NAV-100, SRS-NAV-110 |
| LLT-NAV-002 | SRS-NAV-115, SRS-NAV-120 |
| LLT-NAV-003 | SRS-NAV-130, SRS-NAV-131 |

## Software Components

### Navigation Manager (Level A)
The example focuses on the Navigation Manager component with:

**Interfaces:**
- GPS Interface (input)
- IRS Interface (input)
- Guidance Controller (output)
- Diagnostic System (output)

**Functions:**
- Position computation from GPS
- Multi-sensor fusion (GPS + IRS)
- Sensor failure detection and handling
- Coordinate validation
- Navigation mode management

**Performance:**
- 10 Hz update rate
- 5ms WCET
- 2KB stack limit
- 64KB memory footprint

## DO-178C Compliance

### Objectives Satisfied
For Level A software (71 total objectives):
- Planning Process: 10 objectives
- Development Process: 15 objectives
- Verification Process: 28 objectives
- Configuration Management: 7 objectives
- Quality Assurance: 7 objectives
- Certification Liaison: 4 objectives

### Coverage Achieved
- Statement Coverage: 100%
- Decision Coverage: 100%
- MC/DC Coverage: 100%
- Requirements Coverage: 100%

## Quick Commands

### Generate All Documents
```bash
cd data/strictdoc/DO-178C
strictdoc export .
open output/html/index.html
```

### Generate Individual Documents
```bash
# PSAC
strictdoc export plans/PSAC/PSAC.sdoc

# SVP
strictdoc export plans/SVP/SVP.sdoc

# SRD
strictdoc export requirements/SRD/SRD.sdoc

# SVCP
strictdoc export verification/SVCP/SVCP.sdoc

# SAS
strictdoc export SAS/SAS.sdoc
```

### Generate Traceability Matrix
```bash
strictdoc export --formats=excel .
```

## File Structure Summary

```
strictdoc/DO-178C/
├── grammar.sgra              # Grammar definition
├── README.md                 # Main documentation
├── QUICK_START.md           # Quick start guide
├── INDEX.md                 # This index file
│
├── plans/
│   ├── PSAC/
│   │   └── PSAC.sdoc        # ✅ 15 PLAN elements, 9 sections
│   └── SVP/
│       └── SVP.sdoc         # ✅ 9 PLAN elements, 8 sections
│
├── requirements/
│   └── SRD/
│       └── SRD.sdoc         # ✅ 20 REQUIREMENT elements, 6 sections
│
├── verification/
│   └── SVCP/
│       └── SVCP.sdoc        # ✅ 5 TEST_CASE + 3 LOW_LEVEL_TEST, 7 sections
│
└── SAS/
    └── SAS.sdoc             # ✅ 15 PLAN elements, 12 sections
```

## Element Type Reference

| Element Type | Usage | Example UID |
|--------------|-------|-------------|
| PLAN | Planning items, lifecycle data | PSAC-PROC-001 |
| REQUIREMENT | Software requirements | SRS-NAV-100 |
| TEST_CASE | High-level test cases | HLT-NAV-001 |
| LOW_LEVEL_TEST | Unit test cases | LLT-NAV-001 |
| SECTION | Document sections | (no UID) |
| TEXT | Free-form text | (no UID) |

## Verification Methods Summary

| Method | Description | Elements Using |
|--------|-------------|----------------|
| Test | Execute with inputs to verify outputs | 18 requirements |
| Analysis | Examine artifacts (coverage, traceability, etc.) | 8 requirements |
| Review | Inspect against standards and checklists | All artifacts |
| Demonstration | Execute in target environment | Selective use |
| Inspection | Visual examination of code and data | Build procedures |

## Next Steps

1. ✅ **Review Documentation**: Read [README.md](README.md) for overview
2. ✅ **Quick Start**: Follow [QUICK_START.md](QUICK_START.md) to generate HTML
3. ✅ **Explore Templates**: Open `.sdoc` files in text editor
4. 🔄 **Customize**: Adapt templates for your project
5. 🔄 **Generate**: Export to HTML/PDF for review
6. 🔄 **Extend**: Add SDP, SCMP, SQAP, SDD documents as needed

## Resources

- **StrictDoc**: https://github.com/strictdoc-project/strictdoc
- **DO-178C Standard**: RTCA DO-178C (2011)
- **StrictDoc Templates**: https://github.com/strictdoc-project/strictdoc-templates
- **DO-178C Templates (original)**: https://github.com/strictdoc-project/strictdoc-templates/tree/main/templates/DO-178C

## Status Legend

- ✅ Complete - Fully implemented with examples
- 🔄 In Progress - Partially implemented
- 📋 Planned - Directory created, content pending
- ❌ Not Planned - Not in current scope
