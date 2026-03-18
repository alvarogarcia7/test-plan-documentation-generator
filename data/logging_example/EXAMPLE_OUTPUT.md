# Template Loading Logging Example - Sample Output

## Command

```bash
make test-logging-example
```

## Expected Stderr Logging Output

When the example runs, you should see the following logging messages on stderr (showing the absolute paths of all templates being loaded):

```
Loading verification method template: /absolute/path/to/data/logging_example/verification_methods/analysis/template.j2
Loading verification method template: /absolute/path/to/data/logging_example/verification_methods/test/template.j2
Loading template via include_file: /absolute/path/to/data/logging_example/verification_methods/test/header.j2
Loading template via include_file: /absolute/path/to/data/logging_example/verification_methods/test/header.j2
Loading requirement aggregation template: /absolute/path/to/data/logging_example/verification_methods/requirement_aggregation_template.j2
Loading container template: /absolute/path/to/data/logging_example/container/template.j2
```

## Explanation of Each Log Line

1. **`Loading verification method template: .../analysis/template.j2`**
   - This is the template for the "analysis" verification method type
   - Loaded because `analysis_case_01.yml` has `type: analysis`

2. **`Loading verification method template: .../test/template.j2`**
   - This is the template for the "test" verification method type
   - Loaded because `test_case_01.yml` and `test_case_02.yml` have `type: test`

3. **`Loading template via include_file: .../test/header.j2`** (appears twice)
   - This template is loaded via the `include_file()` function call in `test/template.j2`
   - Appears twice because it's included for both test case files

4. **`Loading requirement aggregation template: .../requirement_aggregation_template.j2`**
   - This optional template aggregates requirements across test cases
   - Loaded based on the output format (markdown in this case)

5. **`Loading container template: .../container/template.j2`**
   - This is the main container template that brings everything together
   - Loaded last to render the final output document

## Template Loading Order

The templates are loaded in the following order:

1. **Verification method templates** - Loaded per test case type (sorted alphabetically)
2. **Include file templates** - Loaded when `include_file()` is called during rendering
3. **Requirement aggregation template** - Loaded if present
4. **Container template** - Loaded as the final step

## Verification

After the templates are loaded and rendered, the output is compared against the expected output:

```bash
diff ./data/logging_example/output.actual.md ./data/logging_example/output.expected.md
```

If there are no differences, the test passes with:
```
✓ Template loading logging example passed!
```

## Files Involved

### Input Files
- `container/schema.json` - Container data validation schema
- `container/data.yml` - Container data
- `container/template.j2` - Main container template
- `verification_methods/test/schema.json` - Test verification method schema
- `verification_methods/test/template.j2` - Test verification method template
- `verification_methods/test/header.j2` - Included template file
- `verification_methods/analysis/schema.json` - Analysis verification method schema
- `verification_methods/analysis/template.j2` - Analysis verification method template
- `verification_methods/requirement_aggregation_template.j2` - Requirement aggregation template
- `test_case/analysis_case_01.yml` - Analysis test case data
- `test_case/test_case_01.yml` - First test case data
- `test_case/test_case_02.yml` - Second test case data

### Output Files
- `output.actual.md` - Generated output (temporary)
- `output.expected.md` - Expected output for comparison

## Key Takeaways

This example demonstrates:

1. **All template loading points are logged** with absolute paths to stderr
2. **Different template types** (verification methods, container, requirement aggregation)
3. **Dynamic loading** based on configuration (type field, format)
4. **Nested loading** via the `include_file()` function
5. **Non-interference** - Logs go to stderr while output goes to stdout/file
