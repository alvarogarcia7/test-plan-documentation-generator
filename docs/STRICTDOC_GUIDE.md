# StrictDoc Guide for Test Plan Documentation Generator

This guide covers the use of StrictDoc for requirements management in the Test Plan Documentation Generator project.

## Table of Contents

- [Introduction to StrictDoc](#introduction-to-strictdoc)
- [Requirements Levels](#requirements-levels)
- [Installation](#installation)
- [Directory Structure](#directory-structure)
- [Creating Requirements](#creating-requirements)
- [Requirement Traceability](#requirement-traceability)
- [Generating Documentation](#generating-documentation)
- [Integration with Rust Test Plan Generator](#integration-with-rust-test-plan-generator)
- [Workflow Examples](#workflow-examples)
- [Troubleshooting](#troubleshooting)

## Introduction to StrictDoc

[StrictDoc](https://strictdoc.readthedocs.io/) is a software requirements management tool that enables you to write, organize, and trace requirements using a text-based format. It supports:

- Text-based requirements in `.sdoc` format
- Hierarchical requirement organization
- Bidirectional traceability between requirements
- Multiple requirement types (System, High-Level, Low-Level)
- HTML and PDF documentation generation
- Web-based editing interface
- Source code integration and traceability

StrictDoc uses a simple, readable text format that integrates well with version control systems and supports collaborative requirements engineering.

## Requirements Levels

This project uses three levels of requirements following a standard software engineering hierarchy:

### System Requirements (SYSREQ)

**Location:** `requirements/sysreq/system_requirements.sdoc`

System requirements define **what** the system must do from a high-level perspective. They describe the system's capabilities, constraints, and overall behavior without specifying implementation details.

**Characteristics:**
- High-level functional and non-functional requirements
- Technology-agnostic where possible
- Stakeholder-focused
- Traceable to business needs or standards

**Example:**
```
[REQUIREMENT]
UID: SYSREQ-001
TITLE: Test Documentation Generation
STATEMENT: The system shall generate test plan documentation from structured data files.
RATIONALE: Automated generation ensures consistency and reduces manual documentation effort.
```

### High-Level Requirements (HLR)

**Location:** `requirements/hlr/high_level_requirements.sdoc`

High-level requirements refine system requirements by introducing architectural and technology choices. They describe **how** the system will satisfy system requirements at a design level.

**Characteristics:**
- Architectural decisions and component definitions
- Technology stack choices (e.g., "use Tera template engine")
- Major design patterns and approaches
- Traceable to system requirements via REFS field

**Example:**
```
[REQUIREMENT]
UID: HLR-001
TITLE: Markdown Output Format
STATEMENT: The system shall generate test documentation in Markdown format.
RATIONALE: Markdown is widely supported and human-readable.
REFS:
- TYPE: Parent
  VALUE: SYSREQ-005
```

### Low-Level Requirements (LLR)

**Location:** `requirements/llr/low_level_requirements.sdoc`

Low-level requirements specify implementation details and specific technical approaches. They describe **exactly how** features will be implemented in code.

**Characteristics:**
- Implementation-specific details
- API choices, function signatures, data structures
- Algorithm specifications
- Directly traceable to code
- Traceable to high-level requirements via REFS field

**Example:**
```
[REQUIREMENT]
UID: LLR-001
TITLE: Clap Derive Macro for CLI
STATEMENT: The system shall use clap's derive macro to define CLI argument structure.
RATIONALE: Derive macro provides type-safe and maintainable CLI definition.
REFS:
- TYPE: Parent
  VALUE: HLR-011
```

### Traceability Flow

The typical traceability flow is:

```
SYSREQ → HLR → LLR → Source Code
```

Each level traces to its parent, creating a complete chain from high-level needs to implementation.

## Installation

StrictDoc is installed as an optional dependency using Python's package management system.

### Prerequisites

- Python 3.8 or later
- `uv` package manager (recommended) or `pip`

### Installation Steps

The project includes a `pyproject.toml` file that defines StrictDoc as a dependency. Install it using:

```bash
# Using uv (recommended - faster installation)
uv sync

# Or using standard pip
pip install .
```

This command:
1. Reads the `pyproject.toml` file in the project root
2. Installs the `strictdoc` package and its dependencies
3. Creates a virtual environment with all required packages

### Verify Installation

Check that StrictDoc is installed correctly:

```bash
# If using uv sync
uv run strictdoc --version

# If using pip
strictdoc --version
```

You should see the StrictDoc version information.

## Directory Structure

The project's requirements are organized in the following structure:

```
requirements/
├── strictdoc_config.py         # StrictDoc project configuration
├── sysreq/                     # System Requirements
│   └── system_requirements.sdoc
├── hlr/                        # High-Level Requirements
│   └── high_level_requirements.sdoc
├── llr/                        # Low-Level Requirements
│   └── low_level_requirements.sdoc
└── output/                     # Generated documentation (auto-created)
    ├── html/                   # HTML export
    └── _source_files/          # Processed source files
```

### Configuration File

The `strictdoc_config.py` file configures the StrictDoc project:

```python
from strictdoc.backend.sdoc.models.document_config import DocumentConfig
from strictdoc.backend.sdoc.models.project_config import ProjectConfig

project_config = ProjectConfig(
    project_title="Test Plan Documentation Generator",
    dir_for_sdoc_files="requirements",
    output_dir="requirements/output",
    enable_traceability=True,
    include_doc_types=["REQUIREMENT", "SECTION"],
    source_root_path_for_code_traceability="src",
)
```

**Key settings:**
- `project_title`: Title of the StrictDoc project
- `dir_for_sdoc_files`: Root directory for `.sdoc` files
- `output_dir`: Where generated documentation is written
- `enable_traceability`: Enables traceability checking
- `include_doc_types`: Types of requirement elements to process
- `source_root_path_for_code_traceability`: Directories to scan for source code traceability

## Creating Requirements

Requirements can be created using the StrictDoc web interface or by manually editing `.sdoc` files.

### Method 1: Using StrictDoc Web Interface (Recommended)

The web interface provides a user-friendly way to create and edit requirements with real-time validation.

#### Start the Server

```bash
cd requirements
# If using uv
uv run strictdoc server

# Or if using pip/venv
strictdoc server
```

This starts a local web server (typically at `http://localhost:5111`) with the StrictDoc UI.

#### Using the Web Interface

1. **Navigate to the server URL** in your browser (usually `http://localhost:5111`)
2. **Browse documents** - Click through the document tree to view existing requirements
3. **Add new requirements**:
   - Navigate to the document where you want to add a requirement
   - Click the "Add Requirement" button
   - Fill in the fields:
     - **UID**: Unique identifier (e.g., `SYSREQ-010`, `HLR-025`, `LLR-050`)
     - **TITLE**: Short descriptive title
     - **STATEMENT**: Detailed requirement statement
     - **RATIONALE**: (Optional) Justification for the requirement
     - **REFS**: (Optional) References to parent requirements
4. **Save changes** - Changes are automatically saved to the `.sdoc` files
5. **Stop the server** - Press `Ctrl+C` in the terminal when done

**Advantages:**
- Real-time validation
- User-friendly interface
- Automatic formatting
- Prevents syntax errors
- Visualizes traceability

### Method 2: Manual .sdoc Editing

You can also edit `.sdoc` files directly in a text editor. This is useful for bulk edits or when working offline.

#### Basic Structure

Every `.sdoc` file has three main sections:

1. **Document Header** - Metadata about the document
2. **Grammar Definition** - Defines the structure of requirements
3. **Requirements** - The actual requirement entries

#### Example: Creating a New System Requirement

Edit `requirements/sysreq/system_requirements.sdoc`:

```
[REQUIREMENT]
UID: SYSREQ-010
TITLE: Error Handling
STATEMENT: The system shall provide clear error messages when validation fails.
RATIONALE: Clear error messages improve user experience and reduce debugging time.
```

**Field descriptions:**
- `UID`: Unique identifier following naming convention (SYSREQ-XXX, HLR-XXX, LLR-XXX)
- `TITLE`: Brief requirement title (1-10 words)
- `STATEMENT`: Complete requirement statement using "shall" language
- `RATIONALE`: (Optional) Explanation of why this requirement exists

#### Example: Creating a High-Level Requirement with Traceability

Edit `requirements/hlr/high_level_requirements.sdoc`:

```
[REQUIREMENT]
UID: HLR-025
TITLE: JSON Schema Validation Library
STATEMENT: The system shall use the jsonschema crate for validating input data.
RATIONALE: The jsonschema crate provides comprehensive JSON Schema Draft 7 support.
REFS:
- TYPE: Parent
  VALUE: SYSREQ-003
```

The `REFS` field creates a traceability link to parent requirement `SYSREQ-003`.

#### Example: Creating a Low-Level Requirement

Edit `requirements/llr/low_level_requirements.sdoc`:

```
[REQUIREMENT]
UID: LLR-050
TITLE: Schema Validation Error Reporting
STATEMENT: The system shall use JSONSchema::validate() and return validation errors with line numbers and paths.
RATIONALE: Detailed error information helps users correct data file issues quickly.
REFS:
- TYPE: Parent
  VALUE: HLR-025
- TYPE: Parent
  VALUE: HLR-010
```

LLRs can trace to multiple parent HLRs if they implement aspects of several high-level requirements.

#### Adding Sections

Organize requirements using sections:

```
[SECTION]
TITLE: Command Line Interface

[REQUIREMENT]
UID: LLR-051
TITLE: CLI Argument Parsing
STATEMENT: The system shall use clap version 4 with derive macros for CLI argument parsing.

[REQUIREMENT]
UID: LLR-052
TITLE: Container Argument Validation
STATEMENT: The system shall validate that exactly three files are provided to --container.

[/SECTION]
```

Sections help organize related requirements and improve documentation readability.

### Naming Conventions

Follow these conventions for requirement UIDs:

- **System Requirements**: `SYSREQ-001`, `SYSREQ-002`, ...
- **High-Level Requirements**: `HLR-001`, `HLR-002`, ...
- **Low-Level Requirements**: `LLR-001`, `LLR-002`, ...

Use sequential numbering within each category. Leave gaps (e.g., 010, 020, 030) if you anticipate adding requirements between existing ones.

## Requirement Traceability

Traceability links requirements together, creating a chain from high-level needs to implementation.

### Establishing Traceability with REFS Field

The `REFS` field creates parent-child relationships between requirements.

#### Basic Syntax

```
REFS:
- TYPE: Parent
  VALUE: <parent-requirement-UID>
```

#### Single Parent Example

```
[REQUIREMENT]
UID: HLR-003
TITLE: Tera Template Engine
STATEMENT: The system shall use the Tera template engine for rendering templates.
RATIONALE: Tera provides Jinja2-like syntax familiar to many users.
REFS:
- TYPE: Parent
  VALUE: SYSREQ-002
```

This creates a traceability link: `HLR-003 → SYSREQ-002`

#### Multiple Parents Example

A single requirement can satisfy multiple parent requirements:

```
[REQUIREMENT]
UID: LLR-020
TITLE: Template Rendering Function
STATEMENT: The system shall implement a render_template() function that accepts a Tera instance, template name, and context.
RATIONALE: Centralized rendering logic promotes code reuse.
REFS:
- TYPE: Parent
  VALUE: HLR-003
- TYPE: Parent
  VALUE: HLR-015
```

This creates: `LLR-020 → HLR-003` and `LLR-020 → HLR-015`

### Traceability Best Practices

1. **Every HLR should trace to at least one SYSREQ**
   - Ensures high-level requirements support system needs

2. **Every LLR should trace to at least one HLR**
   - Ensures implementation details support architectural decisions

3. **Avoid circular references**
   - Parent relationships should flow in one direction: SYSREQ → HLR → LLR

4. **One level at a time**
   - LLRs should trace to HLRs, not directly to SYSREQs
   - Maintains clear hierarchy

5. **Use meaningful relationships**
   - Only link requirements that have a genuine parent-child relationship
   - Don't link requirements just for the sake of coverage

### Checking Traceability

StrictDoc automatically validates traceability when generating documentation. If a requirement references a non-existent parent, StrictDoc will report an error.

## Generating Documentation

StrictDoc can export requirements as HTML or PDF documentation.

**Note:** If you installed StrictDoc using `uv sync`, prefix all `strictdoc` commands with `uv run`, for example: `uv run strictdoc export .`

### Export HTML Documentation

Generate a complete HTML documentation website:

```bash
strictdoc export .
```

Or from the requirements directory:

```bash
cd requirements
strictdoc export .
```

**Output location:** `requirements/output/html/`

The HTML export includes:
- Interactive document browser
- Traceability matrices
- Document tree visualization
- Search functionality
- Deep traceability views
- Coverage statistics

**View the documentation:**

```bash
# Open in default browser (macOS)
open requirements/output/html/index.html

# Open in default browser (Linux)
xdg-open requirements/output/html/index.html

# Or navigate manually to: requirements/output/html/index.html
```

### Export PDF Documentation

StrictDoc can export to PDF via HTML intermediate format (requires additional dependencies):

```bash
strictdoc export . --format=pdf
```

**Note:** PDF export may require additional tools like `weasyprint` or `wkhtmltopdf`. Install with:

```bash
pip install weasyprint
```

### Export Options

```bash
# Export with specific output directory
strictdoc export . --output-dir custom-output

# Export single document
strictdoc export requirements/sysreq/system_requirements.sdoc

# Export with custom project config
strictdoc export . --config custom-strictdoc_config.py
```

### Viewing Generated Documentation

The HTML export creates a complete static website you can:

1. **Browse locally** - Open `output/html/index.html` in a web browser
2. **Host on web server** - Deploy `output/html/` directory to any static web host
3. **Share via file system** - Zip and share the HTML directory

**Navigation:**
- **Document Tree** - Left sidebar shows document hierarchy
- **Traceability** - Click requirement UIDs to see parent/child relationships
- **Search** - Use search bar to find requirements by keyword
- **Deep Traceability** - View complete traceability chains

## Integration with Rust Test Plan Generator

StrictDoc requirements management integrates with the Rust-based test plan generator to ensure test documentation traces to requirements.

### How They Work Together

1. **Requirements define what to test** - SYSREQ and HLR documents specify system capabilities
2. **Test data references requirements** - Test case YAML files include requirement IDs
3. **Documentation shows traceability** - Generated test plans link test cases to requirements
4. **Coverage analysis** - Identify requirements without test coverage

### Workflow Integration

#### Step 1: Define Requirements in StrictDoc

Create system and high-level requirements:

```
[REQUIREMENT]
UID: SYSREQ-006
TITLE: Multiple Test Cases
STATEMENT: The system shall process multiple test case files in a single execution.
```

#### Step 2: Reference Requirements in Test Data

In your test case YAML file (e.g., `data/test_case/test_001.yml`):

```yaml
type: test
requirement: SYSREQ-006
item: "Test Plan Generator"
tc: "TC-001"
id: "Multi-File Processing Test"
description: |
  Verify that the system can process multiple test case files in a single run.
```

The `requirement` field links the test case to `SYSREQ-006`.

#### Step 3: Generate Test Documentation

```bash
./target/release/tpdg \
  --output test_plan.md \
  --container ./data/container/schema.json \
             ./data/container/template.j2 \
             ./data/container/data.yml \
  --test-case ./data/verification_methods \
              ./data/test_case/test_001.yml \
              ./data/test_case/test_002.yml
```

#### Step 4: Generate Requirements Documentation

```bash
cd requirements
strictdoc export .
```

#### Step 5: Cross-Reference

Review both outputs:
- **Test Plan** (`test_plan.md`) - Shows which test cases verify which requirements
- **Requirements Documentation** (`requirements/output/html/`) - Shows requirement hierarchy and status

### Requirements Aggregation Template

The test plan generator includes a requirements aggregation template at:
`data/verification_methods/requirement_aggregation_template.j2`

This template aggregates requirements from all test cases and can generate a requirements traceability matrix in the test plan output.

### Verifying Coverage

Manually verify that:
1. **All testable requirements have test cases** - Review HLRs and LLRs to ensure test coverage
2. **All test cases reference valid requirements** - Check that requirement IDs in YAML match StrictDoc UIDs
3. **Traceability is complete** - Every requirement should trace through the hierarchy

## Workflow Examples

### Workflow 1: Adding a New Feature Requirement

This workflow demonstrates adding a new feature from requirement to implementation.

#### 1. Add System Requirement

Edit `requirements/sysreq/system_requirements.sdoc`:

```
[REQUIREMENT]
UID: SYSREQ-020
TITLE: Custom Filter Support
STATEMENT: The system shall support custom template filters beyond standard Tera filters.
RATIONALE: Custom filters enable domain-specific text transformations in templates.
```

#### 2. Add High-Level Requirements

Edit `requirements/hlr/high_level_requirements.sdoc`:

```
[REQUIREMENT]
UID: HLR-040
TITLE: Strip Filter Implementation
STATEMENT: The system shall provide a 'strip' filter that removes leading and trailing whitespace.
RATIONALE: Whitespace stripping is commonly needed for YAML data cleanup.
REFS:
- TYPE: Parent
  VALUE: SYSREQ-020

[REQUIREMENT]
UID: HLR-041
TITLE: Replace Filter Implementation
STATEMENT: The system shall provide a 'replace' filter for substring replacement.
RATIONALE: String replacement is a common text transformation need.
REFS:
- TYPE: Parent
  VALUE: SYSREQ-020
```

#### 3. Add Low-Level Requirements

Edit `requirements/llr/low_level_requirements.sdoc`:

```
[REQUIREMENT]
UID: LLR-080
TITLE: Tera Custom Filter Registration
STATEMENT: The system shall register custom filters using tera.register_filter() during Tera instance initialization.
RATIONALE: Filter registration must occur before template rendering.
REFS:
- TYPE: Parent
  VALUE: HLR-040
- TYPE: Parent
  VALUE: HLR-041
```

#### 4. Generate and Review Documentation

```bash
cd requirements
strictdoc export .
open output/html/index.html
```

Review the traceability: `SYSREQ-020 → HLR-040, HLR-041 → LLR-080`

#### 5. Implement the Feature

Implement the custom filters in `src/main.rs` according to LLR-080.

#### 6. Create Test Cases

Create test case YAML files that reference `SYSREQ-020` or related HLRs.

### Workflow 2: Bug Fix with Traceability

When fixing a bug, trace it to requirements to verify if requirements need updating.

#### 1. Identify the Bug

Example: Schema validation errors don't show file paths.

#### 2. Find Related Requirements

Search StrictDoc for validation-related requirements:

```bash
cd requirements
strictdoc server
# Use web interface search: "validation error"
```

#### 3. Update or Add Requirements

If requirement doesn't exist, add it:

```
[REQUIREMENT]
UID: LLR-085
TITLE: Validation Error File Paths
STATEMENT: Schema validation errors shall include the file path of the invalid data file.
RATIONALE: File paths help users quickly locate and fix validation errors.
REFS:
- TYPE: Parent
  VALUE: HLR-025
```

#### 4. Fix the Bug

Implement the fix according to the requirement.

#### 5. Update Documentation

```bash
cd requirements
strictdoc export .
```

### Workflow 3: Requirements Review

Periodic review ensures requirements remain accurate and complete.

#### 1. Generate Documentation

```bash
cd requirements
strictdoc export .
```

#### 2. Review Traceability

Open `output/html/index.html` and check:
- All HLRs trace to SYSREQs
- All LLRs trace to HLRs
- No orphaned requirements
- No broken references

#### 3. Check Coverage

Compare requirements to source code:
- Are all LLRs implemented?
- Does source code have features not documented in requirements?

#### 4. Update as Needed

Add missing requirements, remove obsolete ones, update outdated statements.

## Troubleshooting

### Common Issues and Solutions

#### Issue: `strictdoc: command not found`

**Cause:** StrictDoc not installed or not in PATH.

**Solution:**
```bash
# Install StrictDoc
uv sync

# Verify installation
uv run strictdoc --version
```

#### Issue: `ModuleNotFoundError: No module named 'strictdoc'`

**Cause:** StrictDoc not installed in current environment.

**Solution:**
```bash
# Install dependencies
uv sync

# Or if using pip
pip install .
```

#### Issue: Server won't start - "Address already in use"

**Cause:** Another process is using port 5111.

**Solution:**
```bash
# Find and kill process using port 5111
lsof -ti:5111 | xargs kill -9

# Or specify different port
strictdoc server --port 5112
```

#### Issue: "Document not found" error

**Cause:** Running `strictdoc` from wrong directory or `strictdoc_config.py` misconfigured.

**Solution:**
```bash
# Ensure you're in project root or requirements directory
cd /path/to/project/requirements

# Verify strictdoc_config.py exists
ls strictdoc_config.py

# Check configuration
cat strictdoc_config.py
```

#### Issue: Broken traceability references

**Symptom:** Error like "Reference to non-existent requirement: HLR-999"

**Cause:** REFS field references a UID that doesn't exist.

**Solution:**
```bash
# Search for the referenced UID
grep -r "HLR-999" requirements/

# Either:
# 1. Fix the REFS field to point to correct UID
# 2. Create the missing requirement
# 3. Remove the invalid reference
```

#### Issue: Changes not appearing in export

**Cause:** Cached output or StrictDoc not detecting file changes.

**Solution:**
```bash
# Remove old output
rm -rf requirements/output

# Re-export
cd requirements
strictdoc export .
```

#### Issue: Grammar validation errors

**Symptom:** "Field 'STATEMENT' is required but not found"

**Cause:** Missing required fields in requirement.

**Solution:**
- Check the GRAMMAR section of your `.sdoc` file
- Ensure all required fields are present in each REQUIREMENT
- Required fields typically include: UID, TITLE, STATEMENT

#### Issue: Formatting errors in .sdoc files

**Symptom:** Parse errors when loading documents

**Solution:**
- Ensure proper indentation (no tabs, consistent spacing)
- Verify bracket matching: `[REQUIREMENT]` and no unmatched brackets
- Check for special characters in field values
- Use the web interface (`strictdoc server`) to avoid syntax errors

#### Issue: Export takes very long

**Cause:** Large number of requirements or source files.

**Solution:**
```bash
# Temporarily disable source code scanning
# Edit strictdoc_config.py and remove or comment out:
# source_root_path_for_code_traceability="src",

# Or export specific documents
strictdoc export requirements/sysreq/system_requirements.sdoc
```

#### Issue: HTML export not rendering correctly

**Cause:** Browser cache or incomplete export.

**Solution:**
```bash
# Hard refresh browser (Ctrl+Shift+R or Cmd+Shift+R)
# Or clear output and re-export
rm -rf requirements/output
strictdoc export .
```

### Getting Help

If you encounter issues not covered here:

1. **Check StrictDoc documentation**: https://strictdoc.readthedocs.io/
2. **Review StrictDoc examples**: https://github.com/strictdoc-project/strictdoc/tree/main/docs/sphinx/source/strictdoc_examples
3. **Search StrictDoc issues**: https://github.com/strictdoc-project/strictdoc/issues
4. **Use `strictdoc --help`** for command reference
5. **Check `.sdoc` file syntax** by starting the server and viewing error messages

### Best Practices to Avoid Issues

1. **Use the web interface** for creating/editing requirements - it prevents syntax errors
2. **Commit regularly** - version control helps recover from mistakes
3. **Validate after changes** - run `strictdoc export .` to check for errors
4. **Follow naming conventions** - consistent UID patterns prevent confusion
5. **Test traceability** - verify REFS point to valid UIDs before committing
6. **Keep output directory gitignored** - regenerate documentation, don't commit it
