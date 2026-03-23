---
id: TPDG-4
title: Add StrictDoc Documentation for TPDG Project
status: In Progress
assignee: []
created_date: '2026-03-23 09:49'
updated_date: '2026-03-23 10:07'
labels: []
milestone: m-0
dependencies: []
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Set up StrictDoc, a Python-based requirements management tool, within this Rust project by creating a pyproject.toml for Python dependencies, updating the Dockerfile to install StrictDoc using uv, creating requirements directory structure with HLR/LLR/SYSREQ document templates, providing comprehensive usage documentation, and integrating requirements syntax validation into the test suite.
<!-- SECTION:DESCRIPTION:END -->

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
Use pyproject.toml with setuptools backend to manage StrictDoc as a Python development dependency in a primarily Rust project, avoiding complexity of Poetry/PDM while maintaining standard Python packaging conventions

Install StrictDoc in Dockerfile using uv package installer for faster Python dependency resolution and installation compared to pip, while maintaining multi-stage build approach

Create requirements directory structure with separate .sdoc files for HLR, LLR, and SYSREQ following StrictDoc conventions, enabling traceability between requirement levels through parent-child relationships
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Create pyproject.toml in project root with [build-system] section using setuptools backend, [project] section with minimal metadata (name='tpdg-requirements', version='0.1.0', description), and [project.optional-dependencies] requirements section containing 'strictdoc' dependency for managing requirements documentation alongside the Rust codebase.

Update Dockerfile to add Python and StrictDoc installation using uv: add Python3 to the apt-get install command in the deps stage (around line 8-10), install uv using curl -LsSf https://astral.sh/uv/install.sh | sh and add uv to PATH, copy pyproject.toml to /app directory, run uv pip install --system --no-cache .[requirements] to install StrictDoc, and verify StrictDoc CLI is available by running strictdoc --version.

Create requirements/ directory structure at project root containing subdirectories hlr/, llr/, and sysreq/ for organizing requirements by level, and create initial .sdoc document files: requirements/sysreq/system_requirements.sdoc for system-level requirements with SYSREQ- prefix, requirements/hlr/high_level_requirements.sdoc for high-level requirements with HLR- prefix referencing parent SYSREQ requirements, and requirements/llr/low_level_requirements.sdoc for low-level requirements with LLR- prefix referencing parent HLR requirements. Each file should include [DOCUMENT] metadata, [GRAMMAR] section with custom UID PREFIX, example requirements demonstrating UID/TITLE/STATEMENT/RATIONALE fields, and [REQUIREMENT] examples showing parent-child traceability using REFS field.

Create requirements/strictdoc.toml configuration file defining project title, document paths (dir_for_sdoc_files='requirements'), output directory (output_dir='requirements/output'), selected features (requirements.include=['REQUIREMENT', 'SECTION'], traceability.enable=true), and source paths for code traceability if needed.

Create comprehensive docs/STRICTDOC_GUIDE.md documentation covering: introduction to StrictDoc and requirements levels (HLR/LLR/SYSREQ), installation instructions (uv pip install --system .[requirements]), directory structure overview, creating requirements with examples (using strictdoc server command for web UI and manual .sdoc editing), establishing requirement traceability with REFS field examples, generating documentation output (strictdoc export command for HTML/PDF), integration with Rust test plan generator, workflow examples, and troubleshooting common issues.

Update .gitignore to exclude StrictDoc output directories and cache files by adding entries: requirements/output/, requirements/.strictdoc/, *.pyc, __pycache__/, .strictdoc_cache/, .venv/, and venv/.

Create makefile/strictdocs.mk containing StrictDoc-related targets: strictdoc-server to launch web interface (strictdoc server requirements/), strictdoc-export to generate HTML documentation (strictdoc export requirements/ --output-dir requirements/output/), strictdoc-validate to check requirements syntax and consistency using strictdoc check requirements/ which validates .sdoc file syntax, grammar compliance, and reference integrity without validating whether the actual requirements content is correct or complete for the program, and strictdoc-help to display available StrictDoc commands with descriptions. Include comprehensive comments documenting each target's purpose.

Update main Makefile to include the StrictDoc makefile by adding include makefile/strictdocs.mk near the top of the file, update the help target to mention StrictDoc-related commands and reference make strictdoc-help for StrictDoc-specific help, and modify the test target to add $(MAKE) strictdoc-validate as a new step after existing tests to verify requirements syntax correctness (checking .sdoc format, grammar rules, UID uniqueness, and reference validity) without verifying whether the requirements themselves accurately describe the program's actual behavior or completeness.

Update README.md to add a 'Requirements Management' section after the 'CI/CD Verification' section, describing the StrictDoc integration, explaining that make test now includes requirements syntax validation, linking to docs/STRICTDOC_GUIDE.md for detailed usage, and providing quick start commands for viewing and editing requirements (make strictdoc-server, make strictdoc-export, make strictdoc-validate).

Run make test to verify that all existing tests pass and that the new strictdoc-validate step successfully validates the requirements syntax, then run make docker-build to verify that the Dockerfile builds successfully with StrictDoc installed via uv, and verify that StrictDoc commands work inside the container by running docker run --rm test:latest strictdoc --version.
<!-- SECTION:NOTES:END -->
