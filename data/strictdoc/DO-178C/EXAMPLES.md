# DO-178C StrictDoc Examples

This document provides practical examples for using the DO-178C StrictDoc templates.

## Example 1: Viewing Complete Documentation

Generate HTML for the entire DO-178C documentation tree:

```bash
cd data/strictdoc/DO-178C

# Generate HTML with navigation
strictdoc export .

# Open in browser
open output/html/index.html
# Or on Linux:
xdg-open output/html/index.html
# Or on Windows:
start output/html/index.html
```

**Result:** Interactive HTML documentation with:
- Document tree navigation
- Requirement search and filtering
- Traceability links (clickable UIDs)
- Export to Excel

## Example 2: Generating PDF Documentation

Generate PDF for certification submission:

```bash
# Install LaTeX (if not already installed)
# macOS: brew install --cask mactex
# Ubuntu: sudo apt-get install texlive-full
# Windows: Download MiKTeX from https://miktex.org/

# Generate PDF for PSAC
strictdoc export --formats=pdf plans/PSAC/PSAC.sdoc

# Find generated PDF
ls output/pdf/plans/PSAC/PSAC.pdf

# Open PDF
open output/pdf/plans/PSAC/PSAC.pdf
```

## Example 3: Traceability Matrix in Excel

Generate Excel spreadsheet with traceability matrix:

```bash
# Generate Excel export
strictdoc export --formats=excel .

# Find generated Excel file
ls output/excel/

# Open Excel file
open output/excel/traceability.xlsx
```

**Excel Contents:**
- Sheet 1: All requirements with UIDs and properties
- Sheet 2: Forward traceability (requirements → tests)
- Sheet 3: Backward traceability (tests → requirements)
- Sheet 4: Coverage analysis

## Example 4: Filtering by Software Level

View only Level A components:

```bash
# Create filtered view (requires StrictDoc v0.0.40+)
strictdoc export --filter="LEVEL:A" .

# Or use grep to list Level A requirements
grep -r "LEVEL: A" requirements/
```

## Example 5: Adding a New Requirement

Add a new requirement to SRD:

1. Open `requirements/SRD/SRD.sdoc` in text editor

2. Add requirement in appropriate section:

```
[REQUIREMENT]
UID: SRS-NAV-150
LEVEL: A
STATUS: Draft
VERIFICATION: Test
TITLE: Position Hold on Sensor Failure
STATEMENT: >>>
When all sensors become invalid, the Navigation Manager shall maintain 
the last valid position for up to 5 seconds to allow for temporary 
sensor interruptions.
<<<
RATIONALE: >>>
Brief sensor outages should not cause immediate loss of navigation capability.
<<<
COMMENT: >>>
Derived requirement for improved robustness.
Related to SRS-NAV-141.
<<<
```

3. Save file and regenerate documentation:

```bash
strictdoc export requirements/SRD/SRD.sdoc
```

## Example 6: Adding a New Test Case

Add a new test case to SVCP:

1. Open `verification/SVCP/SVCP.sdoc`

2. Add test case in appropriate section:

```
[TEST_CASE]
UID: HLT-NAV-003
LEVEL: A
STATUS: Draft
VERIFICATION_METHOD: Requirements-Based Test
TITLE: Position Hold Verification
OBJECTIVE: >>>
Verify that navigation position is held for 5 seconds after all sensors 
become invalid.
<<<
PREREQUISITES: >>>
- FMS software loaded on target hardware
- HIL simulator configured
- System in DUAL navigation mode
<<<
PROCEDURE: >>>
1. Establish valid navigation with GPS and IRS
   Input: GPS valid, IRS valid
   Expected: Position being computed normally

2. Simultaneously invalidate GPS and IRS
   Input: GPS status = INVALID, IRS status = INVALID
   Expected: Navigation mode = INVALID, position held

3. Verify position remains stable
   Input: Wait 5 seconds
   Expected: Position does not change from last valid value

4. Verify position marked invalid after timeout
   Input: Wait additional 1 second
   Expected: Position validity flag = false
<<<
EXPECTED_RESULT: >>>
Position is held for 5 seconds after sensor failure, then marked invalid.
<<<
PASS_CRITERIA: >>>
Verifies requirement: SRS-NAV-150
<<<
```

3. Save and regenerate:

```bash
strictdoc export verification/SVCP/SVCP.sdoc
```

## Example 7: Creating Requirement Hierarchy

Create parent-child requirement relationships:

```
[SECTION]
TITLE: 2.1 Position Computation

[REQUIREMENT]
UID: SRS-NAV-100
TITLE: GPS Position Computation
STATEMENT: >>>
Parent requirement...
<<<

[REQUIREMENT]
UID: SRS-NAV-101
TITLE: Position Update Rate
STATEMENT: >>>
Derived requirement from SRS-NAV-100...
<<<
COMMENT: >>>
Parent: SRS-NAV-100
<<<

[/SECTION]
```

## Example 8: Linking Documents

Reference other documents:

```
[TEXT]
STATEMENT: >>>
The Navigation Manager architecture is described in the Software Design 
Description (SDD-NAV-001, Section 2). The system-level position accuracy 
requirement is defined in System Requirements Specification (SyRS-NAV-001).

See also:
- PSAC-001: Plan for Software Aspects of Certification
- SVP-001: Software Verification Plan, Section 3.2
<<<
```

## Example 9: Multi-line Tables in Requirements

Use tables in requirements:

```
[REQUIREMENT]
UID: SRS-NAV-160
TITLE: Sensor Fusion Weights
STATEMENT: >>>
The Navigation Manager shall use the following sensor fusion weights 
based on navigation mode:

| Mode | GPS Weight | IRS Weight |
|------|-----------|-----------|
| DUAL | 0.7 | 0.3 |
| GPS_ONLY | 1.0 | 0.0 |
| IRS_ONLY | 0.0 | 1.0 |
| INVALID | 0.0 | 0.0 |
<<<
```

## Example 10: Coverage Analysis Workflow

Track verification coverage:

```bash
# Step 1: Generate traceability matrix
strictdoc export --formats=excel .

# Step 2: Open Excel file
open output/excel/traceability.xlsx

# Step 3: Check for:
# - Requirements without test cases (coverage gaps)
# - Test cases without requirements (orphaned tests)
# - Requirements marked "Not Verified"

# Step 4: Add missing test cases or update traceability links

# Step 5: Regenerate and verify
strictdoc export --formats=excel .
```

## Example 11: Document Templates for New Components

Create documentation for a new component (e.g., Guidance Controller):

```bash
# Copy SRD template
cp requirements/SRD/SRD.sdoc requirements/SRD-GDC/SRD-GDC.sdoc

# Edit the new file
# - Change UID: SRD-NAV-001 → SRD-GDC-001
# - Change TITLE to "Guidance Controller"
# - Update all requirement UIDs: SRS-NAV-xxx → SRS-GDC-xxx
# - Update content for guidance controller

# Add to strictdoc.py document list
# Edit strictdoc.py and add to the documents list:
# {
#     "path": "requirements/SRD-GDC/SRD-GDC.sdoc",
#     "title": "Software Requirements Data - Guidance Controller"
# },

# Generate documentation
strictdoc export .
```

## Example 12: Validation and Checking

Validate documents before committing:

```bash
# Check single document
strictdoc check plans/PSAC/PSAC.sdoc

# Check all documents
strictdoc check .

# Common errors and fixes:
# - "Expected [/SECTION]" → Add missing closing tag
# - "Duplicate UID" → Ensure all UIDs are unique
# - "Unknown element type" → Check grammar.sgra is present
# - "Required field missing" → Add required field to element
```

## Example 13: Custom Grammar Element

Add a custom element type for hazard analysis:

1. Edit `grammar.sgra`:

```
[ELEMENT]
NAME: HAZARD
FIELDS:
- TITLE: UID
  TYPE: String
  REQUIRED: True
- TITLE: HAZARD_DESCRIPTION
  TYPE: String
  REQUIRED: True
- TITLE: SEVERITY
  TYPE: String
  REQUIRED: True
- TITLE: LIKELIHOOD
  TYPE: String
  REQUIRED: True
- TITLE: MITIGATION
  TYPE: String
  REQUIRED: True
```

2. Use in document:

```
[HAZARD]
UID: HAZ-NAV-001
HAZARD_DESCRIPTION: >>>
Incorrect position computation leading to navigation failure
<<<
SEVERITY: Catastrophic
LIKELIHOOD: Remote
MITIGATION: >>>
- Redundant sensors (GPS + IRS)
- Sensor cross-checking and discrepancy detection
- Fail-safe position hold
- 100% MC/DC test coverage
<<<
```

## Example 14: Batch Export

Export all documents in batch:

```bash
#!/bin/bash
# export_all.sh

# Export HTML
strictdoc export --formats=html .

# Export PDF (if LaTeX installed)
strictdoc export --formats=pdf plans/PSAC/PSAC.sdoc
strictdoc export --formats=pdf plans/SVP/SVP.sdoc
strictdoc export --formats=pdf requirements/SRD/SRD.sdoc
strictdoc export --formats=pdf requirements/SDD/SDD.sdoc
strictdoc export --formats=pdf verification/SVCP/SVCP.sdoc
strictdoc export --formats=pdf SAS/SAS.sdoc

# Export Excel
strictdoc export --formats=excel .

# Create deliverables package
mkdir -p deliverables
cp -r output/html deliverables/
cp output/pdf/**/*.pdf deliverables/
cp output/excel/*.xlsx deliverables/

echo "Deliverables ready in deliverables/"
ls -lh deliverables/
```

Run batch export:

```bash
chmod +x export_all.sh
./export_all.sh
```

## Example 15: Search and Navigation

Search in HTML output:

1. Generate HTML: `strictdoc export .`
2. Open in browser: `open output/html/index.html`
3. Use search box in top-right
4. Search examples:
   - "GPS" → Find all references to GPS
   - "SRS-NAV-100" → Jump to specific requirement
   - "Level A" → Filter Level A items
   - "Test" → Find test-related content

## Example 16: Version Control Integration

Track changes with Git:

```bash
# Initialize repository (if not already done)
git init
git add strictdoc.py grammar.sgra
git add plans/ requirements/ verification/ SAS/
git add *.md

# Commit baseline
git commit -m "Initial DO-178C documentation baseline"

# Make changes to requirements
vim requirements/SRD/SRD.sdoc

# View changes
git diff requirements/SRD/SRD.sdoc

# Commit changes
git add requirements/SRD/SRD.sdoc
git commit -m "Add SRS-NAV-150: Position hold requirement"

# View history
git log --oneline requirements/SRD/SRD.sdoc

# Compare versions
git diff HEAD~1 requirements/SRD/SRD.sdoc
```

## Example 17: Certification Package

Create final certification package:

```bash
#!/bin/bash
# create_certification_package.sh

VERSION="2.1.0"
PACKAGE_NAME="FMS-Certification-${VERSION}"

# Create package directory
mkdir -p "${PACKAGE_NAME}"

# Generate all documentation
strictdoc export --formats=html,pdf,excel .

# Copy documents
cp -r output/html "${PACKAGE_NAME}/"
cp -r output/pdf "${PACKAGE_NAME}/"
cp output/excel/*.xlsx "${PACKAGE_NAME}/"

# Copy source .sdoc files (for audit)
cp -r plans requirements verification SAS "${PACKAGE_NAME}/source/"

# Add README
cat > "${PACKAGE_NAME}/README.txt" << EOF
Flight Management System Certification Package
Version: ${VERSION}
Date: $(date +%Y-%m-%d)

Contents:
- html/: Interactive HTML documentation
- pdf/: PDF documents for certification
- excel/: Traceability matrices
- source/: Source StrictDoc files

Open html/index.html in a web browser to view documentation.
EOF

# Create archive
tar czf "${PACKAGE_NAME}.tar.gz" "${PACKAGE_NAME}"
zip -r "${PACKAGE_NAME}.zip" "${PACKAGE_NAME}"

echo "Certification package created:"
echo "  ${PACKAGE_NAME}.tar.gz"
echo "  ${PACKAGE_NAME}.zip"
```

## Example 18: Compliance Checking

Check DO-178C compliance:

```bash
# Count requirements by level
echo "Requirements by Level:"
grep -h "LEVEL: " requirements/**/*.sdoc | sort | uniq -c

# Count verification methods
echo "Verification Methods:"
grep -h "VERIFICATION: " requirements/**/*.sdoc | sort | uniq -c

# Find requirements without verification
echo "Requirements without verification method:"
grep -A 5 "^\[REQUIREMENT\]" requirements/**/*.sdoc | \
  grep -B 5 "VERIFICATION:" | grep "UID:" || echo "None found"

# Check test coverage
echo "Test cases:"
grep -h "^\[TEST_CASE\]" verification/**/*.sdoc | wc -l
grep -h "^\[LOW_LEVEL_TEST\]" verification/**/*.sdoc | wc -l
```

## Tips and Best Practices

1. **Regular Regeneration**: Regenerate HTML after each change to see updates
   ```bash
   strictdoc export . && open output/html/index.html
   ```

2. **UID Consistency**: Use consistent UID patterns:
   - System requirements: `SYS-XXX-###`
   - Software requirements: `SRS-XXX-###`
   - Low-level requirements: `LLR-XXX-####`
   - High-level tests: `HLT-XXX-###`
   - Low-level tests: `LLT-XXX-###`

3. **Section Organization**: Use hierarchical sections for clarity

4. **Traceability**: Always include traceability links in COMMENT or RATIONALE

5. **Status Tracking**: Update STATUS field as requirements mature:
   - Draft → In Review → Approved → Baseline

6. **Version Control**: Commit frequently with descriptive messages

7. **Automated Exports**: Use scripts for batch exports

8. **Review Workflow**: 
   - Generate HTML for reviews
   - Generate PDF for formal documentation
   - Generate Excel for gap analysis

## Next Steps

- Review [README.md](README.md) for comprehensive documentation
- See [QUICK_START.md](QUICK_START.md) for quick commands
- Check [INDEX.md](INDEX.md) for complete document index
- Explore `.sdoc` files to understand structure
- Customize templates for your project
- Add your project-specific requirements and test cases
