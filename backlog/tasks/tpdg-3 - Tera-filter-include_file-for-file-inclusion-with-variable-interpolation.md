---
id: TPDG-3
title: 'Tera filter: include_file for file inclusion with variable interpolation'
status: Done
assignee: []
created_date: '2026-03-16 13:12'
updated_date: '2026-03-23 10:09'
labels: []
milestone: m-0
dependencies: []
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Implement a custom Tera function include_file that reads a file from a local path (relative to the current working directory or verification methods directory) and renders it with variable interpolation from the current template context. The function will use a nested Tera instance to parse and render the included file's content, allowing templates to dynamically include external file content with full access to template variables.
<!-- SECTION:DESCRIPTION:END -->

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
Key Decisions

Use the Function trait with a closure-based approach capturing the base path, rather than a struct-based implementation, to simplify path resolution for file inclusion relative to the verification methods directory

Create a nested Tera instance inside the function to render included file content with the current context, enabling full variable interpolation rather than simple string substitution

Accept a single path argument in the function call syntax {{ include_file(path="file.txt") }} rather than positional arguments to maintain consistency with Tera's keyword argument convention
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
## Tasks

Implement IncludeFileFunction struct in src/main.rs that implements the tera::Function trait with call(&self, args: &HashMap<String, Value>) -> tera::Result<Value> method, extracting the path argument from args, reading the file content using fs::read_to_string, creating a temporary Tera instance with register_custom_filters applied, adding the file content as a raw template, rendering it with the provided context (passed via a thread-local or stored context reference), and returning the rendered result as a Value::String.

Update register_custom_filters function to accept an optional base path parameter, rename it to register_custom_filters_and_functions, and call tera.register_function("include_file", IncludeFileFunction::new(base_path)) to register the include_file function on all Tera instances created in main() (lines 340, 555, 589) and in the test helper render_template() function.

Add comprehensive unit tests in src/main.rs tests module covering: basic file inclusion with include_file(path="file.txt") reading a temporary file, variable interpolation where the included file contains {{ name }} and context has name="World", missing file path argument error, file not found error handling, nested variable access in included files, and included files using custom filters (replace, strip, replace_regex).

Run make test to execute all unit tests and E2E tests, verify all tests pass including new include_file function tests, then run make docker-build to ensure Docker image builds successfully with the new function implementation.
<!-- SECTION:NOTES:END -->
