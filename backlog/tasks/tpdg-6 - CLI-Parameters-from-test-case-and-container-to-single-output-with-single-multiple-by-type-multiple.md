---
id: TPDG-6
title: >-
  CLI Parameters: from --test-case and --container to single output with
  --single, --multiple-by-type, --multiple
status: In Progress
assignee: []
created_date: '2026-03-28 11:57'
updated_date: '2026-03-28 16:01'
labels: []
milestone: m-0
dependencies: []
ordinal: 3000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Replace the --container and --test-cases for several invocations of the program:

Keep the TPDG as a program that hydrates a template with data.

For --container, replace it for --single: one input file, one template, one output file
for --test_case, replace it for --multiple-by-type: .type (.type is the path to the attribute in the yml): multiple input files, one template, one output file.

also create --multiple: multiple input files, one template, one output file. --multiple does not need to specify the type, as they are all the same.

----

Replace --container and --test-case arguments with three new invocation modes: --single for one input/template/output, --multiple for multiple same-type inputs with one template, and --multiple-by-type for multiple inputs grouped by type field. The tool remains a template hydration program that validates data against schemas and renders templates, but with simplified and more flexible argument patterns.

## Key Decisions

Replace the combined --container and --test-case workflow with three separate modes (--single, --multiple, --multiple-by-type) to simplify invocation patterns and make the tool more flexible for different use cases

Use --multiple-by-type with a type attribute path parameter (e.g., '.type') to enable dynamic grouping of inputs by any YAML field, not just a hardcoded 'type' field

Remove the test-case aggregation and container-level template rendering from the default workflow, requiring users to explicitly chain multiple invocations or use --multiple modes for aggregation

## Tasks

Add new CLI argument structures to src/main.rs: create SingleArgs struct with schema/template/input/output fields, MultipleArgs struct with schema/template/inputs/output fields, and MultipleByTypeArgs struct with type_path/template_dir/inputs/output fields. Update the main Args enum to use clap subcommands for --single, --multiple, and --multiple-by-type modes, removing the existing --container and --test-case arguments.

Implement the --single mode in src/main.rs: add handle_single_mode function that takes schema/template/input paths, validates the input YAML against the schema, loads and renders the template with the input data, and writes output to file or stdout. Support both .j2 and .adoc template formats based on file extension.

Implement the --multiple mode in src/main.rs: add handle_multiple_mode function that takes schema/template and multiple input files, validates each input against the schema, renders each input with the template, concatenates all rendered outputs with newline separators, and writes the final aggregated output. Ensure deterministic ordering by sorting input file paths.

Implement the --multiple-by-type mode in src/main.rs: add handle_multiple_by_type_mode function that takes a type attribute path (e.g., '.type'), template directory, and input files. Extract the type value from each input YAML using the attribute path, group inputs by type, load type-specific schema and template from {template_dir}/{type}/schema.json and {template_dir}/{type}/template{.j2|_asciidoc.adoc}, validate and render each input, and concatenate all outputs. Handle the --format argument to select template suffix.

Update Makefile test targets to use the new CLI argument patterns: refactor test-e2e-markdown to chain --multiple-by-type invocations (first for test cases, then wrap results with --single for container), update test-e2e-test-plan-asciidoc and test-e2e-test-results-asciidoc similarly, and update all input_data test targets. Create intermediate files in temp directories to pass data between chained invocations.

Update E2E tests in tests/e2e.rs to use the new CLI arguments: modify test_e2e_basic_yaml_rendering to use --single mode, update other E2E tests to use --multiple or --multiple-by-type modes as appropriate, and ensure all snapshot tests pass with the new argument structure. Add new E2E tests for each of the three modes individually.

Update README.md and AGENTS.md documentation: revise CLI Reference section to document the three new modes (--single, --multiple, --multiple-by-type) with syntax and examples, remove references to --container and --test-case arguments, update Quick Start Guide examples to demonstrate the new invocation patterns, and document how to chain commands for container-level aggregation workflows.

Run make test to execute all unit tests and E2E tests with the refactored CLI arguments, verify all tests pass and generated outputs match expected files, then run make docker-build to ensure Docker image builds successfully with the new argument structure.

Verify the output of the files for end to end has not changed in this branch. main and this branch should have the same output, but different ways of invoking the program.
<!-- SECTION:DESCRIPTION:END -->
