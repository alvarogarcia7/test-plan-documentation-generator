# DO-178C StrictDoc Templates - Quick Start Guide

## Installation

```bash
# Install StrictDoc
pip install strictdoc

# Verify installation
strictdoc --version
```

## Quick Commands

### View Documentation in Browser

```bash
# Export all DO-178C documents to HTML
cd data/strictdoc/DO-178C
strictdoc export .

# Open in browser
open output/html/index.html
```

### Generate Individual Documents

```bash
# Generate PSAC (Plan for Software Aspects of Certification)
strictdoc export plans/PSAC/PSAC.sdoc

# Generate SVP (Software Verification Plan)
strictdoc export plans/SVP/SVP.sdoc

# Generate SRD (Software Requirements Data)
strictdoc export requirements/SRD/SRD.sdoc

# Generate SVCP (Software Verification Cases and Procedures)
strictdoc export verification/SVCP/SVCP.sdoc

# Generate SAS (Software Accomplishment Summary)
strictdoc export SAS/SAS.sdoc
```

### Generate PDF (Requires LaTeX)

```bash
# Generate PDF for a document
strictdoc export --formats=pdf plans/PSAC/PSAC.sdoc

# Find generated PDF
ls output/pdf/
```

### Generate Excel Traceability

```bash
# Generate Excel traceability matrix
strictdoc export --formats=excel .

# Find generated Excel file
ls output/excel/
```

## Document Structure

### Basic StrictDoc Document

```
[DOCUMENT]
TITLE: My Document Title
UID: DOC-001
VERSION: 1.0

[SECTION]
TITLE: Section 1

[TEXT]
STATEMENT: >>>
Free-form text goes here.
Can span multiple lines.
<<<

[/SECTION]
```

### Requirement Element

```
[REQUIREMENT]
UID: REQ-001
LEVEL: A
STATUS: Approved
VERIFICATION: Test
TITLE: Requirement Title
STATEMENT: >>>
The system shall perform this function.
<<<
RATIONALE: >>>
This is needed because...
<<<
```

### Test Case Element

```
[TEST_CASE]
UID: TC-001
VERIFICATION_METHOD: Requirements-Based Test
TITLE: Test Case Title
OBJECTIVE: >>>
Verify that the system...
<<<
PROCEDURE: >>>
1. Step one
2. Step two
<<<
EXPECTED_RESULT: >>>
System produces correct output
<<<
```

### Low-Level Test Element

```
[LOW_LEVEL_TEST]
UID: LLT-001
TITLE: Unit Test Title
UNIT_UNDER_TEST: myFunction()
OBJECTIVE: >>>
Verify function behavior
<<<
TEST_INPUTS: >>>
| Parameter | Value |
|-----------|-------|
| input1    | 42    |
<<<
EXPECTED_OUTPUTS: >>>
| Parameter | Value |
|-----------|-------|
| output1   | 84    |
<<<
```

## Common Workflows

### 1. Create New Requirement

```
[REQUIREMENT]
UID: SRS-NAV-200
LEVEL: A
STATUS: Draft
VERIFICATION: Test, Analysis
TITLE: Your Requirement Title
STATEMENT: >>>
The Navigation Manager shall...
<<<
```

### 2. Link Test to Requirement

In test case COMMENT or PASS_CRITERIA field:

```
PASS_CRITERIA: >>>
Verifies requirements: SRS-NAV-200, SRS-NAV-201
<<<
```

### 3. Add Section

```
[SECTION]
TITLE: 3. New Section

[TEXT]
STATEMENT: >>>
Introduction text for this section.
<<<

[REQUIREMENT]
UID: REQ-301
...
[/REQUIREMENT]

[/SECTION]
```

### 4. Document Hierarchy

```
[SECTION]
TITLE: Level 1 Section

[SECTION]
TITLE: Level 2 Section

[SECTION]
TITLE: Level 3 Section

[TEXT]
STATEMENT: >>>
Content at level 3
<<<

[/SECTION]  # Close level 3
[/SECTION]  # Close level 2
[/SECTION]  # Close level 1
```

## Key Fields Reference

### Document Header
- `TITLE`: Document title
- `UID`: Unique document identifier
- `VERSION`: Document version number
- `CLASSIFICATION`: DO-178C level and classification

### Requirement Fields
- `UID`: Unique requirement ID (e.g., SRS-NAV-100)
- `LEVEL`: Software level (A, B, C, D, E)
- `STATUS`: Draft, In Review, Approved
- `VERIFICATION`: Test, Analysis, Review, Demonstration, Inspection
- `TITLE`: Short requirement title
- `STATEMENT`: Full requirement text (between `>>>` and `<<<`)
- `RATIONALE`: Justification for requirement
- `COMMENT`: Additional notes, traceability links

### Test Case Fields
- `UID`: Unique test ID (e.g., HLT-NAV-001)
- `VERIFICATION_METHOD`: Requirements-Based Test, Unit Test, etc.
- `TITLE`: Test case title
- `OBJECTIVE`: What the test verifies
- `PREREQUISITES`: Setup and preconditions
- `PROCEDURE`: Step-by-step test steps
- `EXPECTED_RESULT`: Expected outcomes
- `PASS_CRITERIA`: Pass/fail criteria and requirement links

### Low-Level Test Fields
- `UID`: Unique test ID (e.g., LLT-NAV-001)
- `UNIT_UNDER_TEST`: Function or module being tested
- `OBJECTIVE`: Test objective
- `TEST_TYPE`: Normal Range, Boundary, Error Handling, etc.
- `SETUP`: Test setup and stubs
- `TEST_INPUTS`: Input parameters and values (table format)
- `EXPECTED_OUTPUTS`: Expected output values (table format)
- `COVERAGE`: Coverage requirements (MC/DC, decision, statement)

## DO-178C Software Levels

| Level | Description | Verification Objectives |
|-------|-------------|------------------------|
| A | Catastrophic | 71 objectives, MC/DC coverage required |
| B | Hazardous | 69 objectives, decision coverage required |
| C | Major | 62 objectives, statement coverage required |
| D | Minor | 26 objectives, no structural coverage required |
| E | No Effect | 0 objectives |

## Verification Methods

- **Test**: Execute software with inputs to verify outputs
- **Analysis**: Examine artifacts (traceability, coverage, timing, etc.)
- **Review**: Inspect artifacts against standards and checklists
- **Demonstration**: Execute in target environment with observation
- **Inspection**: Visual examination of code, data, procedures

## Traceability Examples

### Forward Traceability (Requirement → Test)

In requirement:
```
[REQUIREMENT]
UID: SRS-NAV-100
COMMENT: >>>
Verified by: HLT-NAV-001, LLT-NAV-001
<<<
```

### Backward Traceability (Test → Requirement)

In test case:
```
[TEST_CASE]
UID: HLT-NAV-001
PASS_CRITERIA: >>>
Verifies requirements: SRS-NAV-100, SRS-NAV-101
<<<
```

## Tips and Best Practices

### 1. Consistent UID Naming
- System requirements: `SYS-XXX-###`
- Software requirements: `SRS-XXX-###`
- Low-level requirements: `LLR-XXX-###`
- High-level tests: `HLT-XXX-###`
- Low-level tests: `LLT-XXX-###`

### 2. Multi-line Text
Always use `>>>` and `<<<` markers for multi-line content:
```
STATEMENT: >>>
First line
Second line
Third line
<<<
```

### 3. Tables in Text
Use Markdown table syntax:
```
STATEMENT: >>>
| Column 1 | Column 2 |
|----------|----------|
| Value 1  | Value 2  |
<<<
```

### 4. Validation
Validate your documents before generating:
```bash
strictdoc check plans/PSAC/PSAC.sdoc
```

### 5. Search and Filter
StrictDoc HTML output includes search and filter capabilities:
- Search by UID, title, or content
- Filter by level, status, or verification method
- Generate filtered views

## Common Issues

### Issue: Unclosed Section
**Error:** "Expected [/SECTION]"

**Fix:** Ensure every `[SECTION]` has a matching `[/SECTION]`

### Issue: Invalid Grammar
**Error:** "Unknown element type"

**Fix:** Ensure grammar.sgra is in the same directory or parent directory

### Issue: UID Conflict
**Error:** "Duplicate UID"

**Fix:** Ensure all UIDs are unique across the document

### Issue: Missing Required Field
**Error:** "Required field X not found"

**Fix:** Add the required field to the element

## Next Steps

1. **Explore Templates**: Review the provided PSAC, SVP, SRD, SVCP, and SAS documents
2. **Customize**: Copy and modify templates for your project
3. **Generate**: Export to HTML/PDF to see formatted output
4. **Iterate**: Update documents as requirements evolve
5. **Trace**: Maintain traceability links between requirements and tests
6. **Verify**: Generate traceability matrices and coverage reports

## Resources

- **StrictDoc Docs**: https://strictdoc.readthedocs.io/
- **StrictDoc GitHub**: https://github.com/strictdoc-project/strictdoc
- **DO-178C Standard**: RTCA DO-178C (2011)
- **Template Examples**: See `plans/`, `requirements/`, `verification/`, and `SAS/` directories

## Getting Help

```bash
# StrictDoc help
strictdoc --help

# Export command help
strictdoc export --help

# Check command help
strictdoc check --help
```
