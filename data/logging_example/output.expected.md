# Template Loading Logging Example

Version: 1.0.0

---

## Template Loading Demonstration

This example demonstrates the template loading logging feature that outputs absolute file paths to stderr when templates are loaded.

The following templates are loaded during the execution of this example:

1. **Verification Method Templates** - Loaded based on the `type` field in test case YAML files
2. **Requirement Aggregation Template** - Optional template for aggregating requirements
3. **Container Template** - This template (the main container)
4. **Include File Templates** - Loaded via `include_file()` function calls

## Test Cases

### Analysis: AN-LOG-001

**Name**: Analysis Case

**Description**: This demonstrates loading a different verification method type

_This is an analysis-type verification method._

---


### Test Case: TC-LOG-001

**Name**: First Test Case

**Description**: This test case demonstrates verification method template loading

_This section was loaded via include_file() function._

**Included Template Details:**
- This demonstrates nested template loading
- The absolute path of this file is logged to stderr when loaded


---


### Test Case: TC-LOG-002

**Name**: Second Test Case

**Description**: Another test case showing template loading with include_file

_This section was loaded via include_file() function._

**Included Template Details:**
- This demonstrates nested template loading
- The absolute path of this file is logged to stderr when loaded


---


---

End of document.
