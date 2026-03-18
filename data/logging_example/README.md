# Template Loading Logging Example

## Purpose

This example demonstrates the template loading logging feature that outputs the absolute file path to stderr immediately before each template is loaded and rendered.

## What This Example Demonstrates

When you run this example, you will see logging output to stderr showing the absolute paths of all templates being loaded:

1. **Verification Method Templates**
   - `test/template.j2` - Loaded for test cases (TC-LOG-001, TC-LOG-002)
   - `analysis/template.j2` - Loaded for analysis cases (AN-LOG-001)

2. **Include File Template**
   - `test/header.j2` - Loaded via `include_file()` function in the test template

3. **Requirement Aggregation Template**
   - `requirement_aggregation_template.j2` - Loaded based on the output format

4. **Container Template**
   - `container/template.j2` - The main container template

## Directory Structure

```
data/logging_example/
├── README.md                                    # This file
├── container/
│   ├── schema.json                             # Container data schema
│   ├── data.yml                                # Container data
│   └── template.j2                             # Container template (main)
├── verification_methods/
│   ├── test/
│   │   ├── schema.json                         # Test verification method schema
│   │   ├── template.j2                         # Test verification method template
│   │   └── header.j2                           # Included template file
│   ├── analysis/
│   │   ├── schema.json                         # Analysis verification method schema
│   │   └── template.j2                         # Analysis verification method template
│   └── requirement_aggregation_template.j2     # Requirement aggregation template
├── test_case/
│   ├── test_case_01.yml                        # First test case (type: test)
│   ├── test_case_02.yml                        # Second test case (type: test)
│   └── analysis_case_01.yml                    # Analysis case (type: analysis)
└── output.expected.md                           # Expected output

```

## Running the Example

Execute the example using the Makefile target:

```bash
make test-logging-example
```

## Expected Logging Output

When the example runs, you should see output similar to this on stderr:

```
Loading verification method template: /full/path/to/data/logging_example/verification_methods/analysis/template.j2
Loading verification method template: /full/path/to/data/logging_example/verification_methods/test/template.j2
Loading template via include_file: /full/path/to/data/logging_example/verification_methods/test/header.j2
Loading template via include_file: /full/path/to/data/logging_example/verification_methods/test/header.j2
Loading requirement aggregation template: /full/path/to/data/logging_example/verification_methods/requirement_aggregation_template.j2
Loading container template: /full/path/to/data/logging_example/container/template.j2
```

Note: The actual paths will be absolute paths on your system.

## Key Features Demonstrated

1. **Dynamic Template Loading** - Different templates loaded based on the `type` field in YAML files
2. **Nested Template Loading** - The `include_file()` function loads additional templates
3. **Requirement Aggregation** - Optional template loaded based on format
4. **Container Template** - The main template that brings everything together

## Verification

The example includes an expected output file (`output.expected.md`) that the actual output is compared against using `diff` to ensure correctness.
