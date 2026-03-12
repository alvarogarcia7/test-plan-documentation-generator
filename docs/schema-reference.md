# Schema Reference

Complete documentation for all JSON schemas used in the Test Plan Documentation Generator, including container schema and all verification method schemas (test, analysis, demonstration, inspection).

## Table of Contents

1. [Overview](#overview)
2. [Schema Validation](#schema-validation)
3. [Container Schema](#container-schema)
4. [Verification Method Schemas](#verification-method-schemas)
   - [Test Schema](#test-schema)
   - [Analysis Schema](#analysis-schema)
   - [Demonstration Schema](#demonstration-schema)
   - [Inspection Schema](#inspection-schema)
5. [Common Patterns](#common-patterns)
6. [Validation Rules](#validation-rules)
7. [Examples](#examples)

## Overview

The Test Plan Documentation Generator uses JSON Schema (Draft-04) to validate YAML data files before rendering templates. This ensures data consistency and catches errors early in the documentation generation process.

### Schema Locations

All schemas are located in the `data/` directory structure:

```
data/
├── container/
│   └── schema.json              # Container-level schema
└── verification_methods/
    ├── test/
    │   └── schema.json          # Test verification schema
    ├── analysis/
    │   └── schema.json          # Analysis verification schema
    ├── demonstration/
    │   └── schema.json          # Demonstration verification schema
    └── inspection/
        └── schema.json          # Inspection verification schema
```

### Schema Compliance

All schemas follow JSON Schema Draft-04 specification:
- `$schema`: `"http://json-schema.org/draft-04/schema#"`
- Standard validation keywords: `type`, `properties`, `required`, `enum`, `items`, etc.

## Schema Validation

### Validation Process

1. **Load Schema**: Read JSON schema from file
2. **Compile Schema**: Validate schema syntax and compile for validation
3. **Load Data**: Read YAML data file
4. **Convert**: Convert YAML to JSON for validation
5. **Validate**: Check data against compiled schema
6. **Report**: Display validation errors if any

### Validation Errors

When validation fails, the tool provides detailed error messages:

```
Error: Validation failed for file: data/test_case/example.yml
Against schema: data/verification_methods/test/schema.json
  - "type" is a required property
  - "test_sequences[0].steps[1].expected.result" is a required property
```

### Exit Codes

- `0` - Validation successful
- `3` - Validation failed (schema violation)

## Container Schema

The container schema defines the structure for the top-level test plan document metadata.

### Location

`data/container/schema.json`

### Schema Definition

```json
{
  "$schema": "http://json-schema.org/draft-04/schema#",
  "type": "object",
  "properties": {
    "date": {
      "type": "string"
    },
    "product": {
      "type": "string"
    },
    "description": {
      "type": "string"
    }
  },
  "required": [
    "date",
    "product",
    "description"
  ]
}
```

### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `date` | string | Yes | Document date (any string format) |
| `product` | string | Yes | Product name or identifier |
| `description` | string | Yes | Document/product description |

### Example YAML

```yaml
date: "2024-03-15"
product: "eUICC Test Platform v2.0"
description: "Comprehensive test plan for eUICC provisioning system"
```

### Notes

- The container schema is minimal by design
- Additional fields can be added without validation errors (no `additionalProperties: false`)
- Templates can add custom fields as needed

### Extended Example

```yaml
# Required fields
date: "2024-03-15"
product: "eUICC Platform"
description: "Test plan for GSMA SGP.22 compliance"

# Optional custom fields (not validated but accessible in templates)
version: "1.0.0"
author: "Test Engineering Team"
revision: "Rev 2"
title: "eUICC Test Plan"
test_results:
  - test_case_id: "TC-001"
    requirement: "XXX100"
    overall_pass: true
```

## Verification Method Schemas

### Schema Selection

The tool automatically selects the appropriate schema based on the `type` field in the test case YAML file:

- `type: "test"` → `verification_methods/test/schema.json`
- `type: "analysis"` → `verification_methods/analysis/schema.json`
- `type: "demonstration"` → `verification_methods/demonstration/schema.json`
- `type: "inspection"` → `verification_methods/inspection/schema.json`

---

## Test Schema

Used for executable test cases with sequences and steps.

### Location

`data/verification_methods/test/schema.json`

### Schema Definition

```json
{
  "$schema": "http://json-schema.org/draft-04/schema#",
  "type": "object",
  "properties": {
    "type": {
      "type": "string",
      "enum": ["test"]
    },
    "requirement": {
      "type": "string"
    },
    "item": {
      "type": "integer"
    },
    "tc": {
      "type": "integer"
    },
    "id": {
      "type": "string"
    },
    "description": {
      "type": "string"
    },
    "general_initial_conditions": {
      "type": "object",
      "additionalProperties": {
        "type": "array",
        "items": {
          "type": "string"
        }
      }
    },
    "initial_conditions": {
      "type": "object",
      "additionalProperties": {
        "type": "array",
        "items": {
          "type": "string"
        }
      }
    },
    "test_sequences": {
      "type": "array",
      "items": {
        "type": "object",
        "properties": {
          "id": {
            "type": "integer"
          },
          "name": {
            "type": "string"
          },
          "description": {
            "type": "string"
          },
          "initial_conditions": {
            "type": "object",
            "additionalProperties": {
              "type": "array",
              "items": {
                "type": "string"
              }
            }
          },
          "steps": {
            "type": "array",
            "items": {
              "type": "object",
              "properties": {
                "step": {
                  "type": "integer"
                },
                "manual": {
                  "type": "boolean"
                },
                "description": {
                  "type": "string"
                },
                "command": {
                  "type": "string"
                },
                "expected": {
                  "type": "object",
                  "properties": {
                    "success": {
                      "type": "boolean"
                    },
                    "result": {
                      "type": "string"
                    },
                    "output": {
                      "type": "string"
                    }
                  },
                  "required": [
                    "result",
                    "output"
                  ]
                }
              },
              "required": [
                "step",
                "description",
                "command",
                "expected"
              ]
            }
          }
        },
        "required": [
          "id",
          "name",
          "description",
          "initial_conditions",
          "steps"
        ]
      }
    }
  },
  "required": [
    "type",
    "requirement",
    "item",
    "tc",
    "id",
    "description",
    "general_initial_conditions",
    "initial_conditions",
    "test_sequences"
  ]
}
```

### Top-Level Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `type` | string | Yes | Must be "test" |
| `requirement` | string | Yes | Requirement identifier (e.g., "XXX100") |
| `item` | integer | Yes | Item number |
| `tc` | integer | Yes | Test case number |
| `id` | string | Yes | Test case identifier |
| `description` | string | Yes | Test case description |
| `general_initial_conditions` | object | Yes | General preconditions (see below) |
| `initial_conditions` | object | Yes | Test-specific conditions (see below) |
| `test_sequences` | array | Yes | Array of test sequences (see below) |

### Initial Conditions Structure

Both `general_initial_conditions` and `initial_conditions` use the same structure:

```yaml
entity_name:
  - "Condition 1"
  - "Condition 2"
another_entity:
  - "Condition A"
```

- **Keys**: Entity names (strings)
- **Values**: Arrays of condition strings

### Test Sequence Structure

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `id` | integer | Yes | Sequence number |
| `name` | string | Yes | Sequence name |
| `description` | string | Yes | Sequence description |
| `initial_conditions` | object | Yes | Sequence-specific conditions |
| `steps` | array | Yes | Array of test steps |

### Test Step Structure

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `step` | integer | Yes | Step number |
| `manual` | boolean | No | Whether step is manual (default: false) |
| `description` | string | Yes | Step description |
| `command` | string | Yes | Command to execute |
| `expected` | object | Yes | Expected results object |

### Expected Results Structure

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `success` | boolean | No | Expected success state (default: true) |
| `result` | string | Yes | Expected result message |
| `output` | string | Yes | Expected output |

### Complete Example

```yaml
type: test
requirement: "XXX100"
item: 1
tc: 4
id: '4.2.2.2.1 TC_eUICC_ES6.UpdateMetadata'
description: 'Test case for metadata update operations'

general_initial_conditions:
  eUICC:
    - "The profile PROFILE_OPERATIONAL1 is loaded on the eUICC"
    - "System is in ready state"
  SM-DP+:
    - "Server is accessible and responding"

initial_conditions:
  eUICC:
    - "PROFILE_OPERATIONAL1 is Enabled"
  Network:
    - "Secure channel is established"

test_sequences:
  - id: 1
    name: "Nominal Flow: Update Metadata"
    description: |
      This test verifies that the eUICC correctly processes 
      an ES6.UpdateMetadata command.
    initial_conditions:
      eUICC:
        - "Profile is in operational state"
    steps:
      - step: 1
        manual: false
        description: "Send UpdateMetadata command"
        command: "ssh"
        expected:
          success: true
          result: "SW=0x9000"
          output: "Operation successful"
      
      - step: 2
        manual: true
        description: "Verify metadata was updated"
        command: "inspect"
        expected:
          result: "Metadata matches expected values"
          output: "All fields updated correctly"
```

---

## Analysis Schema

Used for analytical verification through calculations and modeling.

### Location

`data/verification_methods/analysis/schema.json`

### Schema Definition

```json
{
  "$schema": "http://json-schema.org/draft-04/schema#",
  "type": "object",
  "properties": {
    "type": {
      "type": "string",
      "enum": ["analysis"]
    },
    "requirement": {
      "type": "string"
    },
    "id": {
      "type": "string"
    },
    "description": {
      "type": "string"
    },
    "method": {
      "type": "string"
    },
    "calculations": {
      "type": "array",
      "items": {
        "type": "object",
        "properties": {
          "name": {
            "type": "string"
          },
          "formula": {
            "type": "string"
          },
          "parameters": {
            "type": "object",
            "additionalProperties": {
              "type": "string"
            }
          }
        },
        "required": [
          "name",
          "formula"
        ]
      }
    },
    "models": {
      "type": "array",
      "items": {
        "type": "object",
        "properties": {
          "name": {
            "type": "string"
          },
          "type": {
            "type": "string"
          },
          "description": {
            "type": "string"
          }
        },
        "required": [
          "name",
          "type",
          "description"
        ]
      }
    },
    "acceptance_criteria": {
      "type": "array",
      "items": {
        "type": "string"
      }
    }
  },
  "required": [
    "type",
    "requirement",
    "id",
    "description",
    "method",
    "calculations",
    "models",
    "acceptance_criteria"
  ]
}
```

### Top-Level Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `type` | string | Yes | Must be "analysis" |
| `requirement` | string | Yes | Requirement identifier |
| `id` | string | Yes | Analysis case identifier |
| `description` | string | Yes | Analysis description |
| `method` | string | Yes | Analysis method description |
| `calculations` | array | Yes | Array of calculation objects |
| `models` | array | Yes | Array of model objects |
| `acceptance_criteria` | array | Yes | Array of criteria strings |

### Calculation Object

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `name` | string | Yes | Calculation name |
| `formula` | string | Yes | Mathematical formula |
| `parameters` | object | No | Parameter definitions (key-value pairs) |

### Model Object

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `name` | string | Yes | Model name |
| `type` | string | Yes | Model type (e.g., "Statistical", "Queuing theory") |
| `description` | string | Yes | Model description |

### Complete Example

```yaml
type: analysis
requirement: "XXX200"
id: '4.2.2.4 AN_eUICC_Performance_Analysis'
description: 'Analysis of eUICC performance under various load conditions'
method: 'Statistical analysis and mathematical modeling'

calculations:
  - name: "Maximum throughput"
    formula: "T_max = (N * S) / (L + O)"
    parameters:
      N: "Number of parallel channels"
      S: "Signal processing speed (ops/sec)"
      L: "Latency per operation (sec)"
      O: "Overhead factor"
  
  - name: "Response time percentile"
    formula: "P95 = μ + (1.645 * σ)"
    parameters:
      μ: "Mean response time"
      σ: "Standard deviation"

models:
  - name: "Load balancing model"
    type: "Queuing theory"
    description: "M/M/c queue model for analyzing multi-channel processing"
  
  - name: "Resource utilization model"
    type: "Statistical"
    description: "Time-series analysis of CPU and memory utilization"

acceptance_criteria:
  - "Maximum throughput must exceed 1000 operations per second"
  - "95th percentile response time must be below 200ms"
  - "System must maintain stability under 80% resource utilization"
```

---

## Demonstration Schema

Used for live demonstration verification.

### Location

`data/verification_methods/demonstration/schema.json`

### Schema Definition

```json
{
  "$schema": "http://json-schema.org/draft-04/schema#",
  "type": "object",
  "properties": {
    "type": {
      "type": "string",
      "enum": ["demonstration"]
    },
    "requirement": {
      "type": "string"
    },
    "id": {
      "type": "string"
    },
    "description": {
      "type": "string"
    },
    "procedure": {
      "type": "string"
    },
    "observations": {
      "type": "array",
      "items": {
        "type": "string"
      }
    },
    "acceptance_criteria": {
      "type": "array",
      "items": {
        "type": "string"
      }
    }
  },
  "required": [
    "type",
    "requirement",
    "id",
    "description",
    "procedure",
    "observations",
    "acceptance_criteria"
  ]
}
```

### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `type` | string | Yes | Must be "demonstration" |
| `requirement` | string | Yes | Requirement identifier |
| `id` | string | Yes | Demonstration identifier |
| `description` | string | Yes | Demonstration description |
| `procedure` | string | Yes | Procedure description (can be multi-line) |
| `observations` | array | Yes | Array of observation strings |
| `acceptance_criteria` | array | Yes | Array of criteria strings |

### Complete Example

```yaml
type: demonstration
requirement: "XXX300"
id: '4.2.2.5 DM_eUICC_Profile_Management'
description: 'Demonstration of profile lifecycle management operations'

procedure: |
  1. Power on the device with eUICC installed
  2. Establish secure connection to SM-DP+ server
  3. Initiate profile download using activation code
  4. Monitor profile installation progress
  5. Verify profile is listed in available profiles
  6. Enable the newly installed profile
  7. Perform basic connectivity test
  8. Disable profile and switch to another profile
  9. Delete the test profile
  10. Verify profile removal from eUICC

observations:
  - "Profile download initiated successfully with activation code scan"
  - "Installation progress displayed real-time status updates"
  - "Profile appeared in device settings within 30 seconds"
  - "Network connectivity established immediately after enable"
  - "Profile switching completed without device reboot"
  - "Profile deletion removed all associated credentials"

acceptance_criteria:
  - "Profile download and installation must complete within 2 minutes"
  - "Profile must be usable immediately after enable"
  - "Profile switching must occur without device restart"
  - "Profile deletion must completely remove all profile data"
```

---

## Inspection Schema

Used for manual inspection and review verification.

### Location

`data/verification_methods/inspection/schema.json`

### Schema Definition

```json
{
  "$schema": "http://json-schema.org/draft-04/schema#",
  "type": "object",
  "properties": {
    "type": {
      "type": "string",
      "enum": ["inspection"]
    },
    "requirement": {
      "type": "string"
    },
    "id": {
      "type": "string"
    },
    "description": {
      "type": "string"
    },
    "inspection_method": {
      "type": "string"
    },
    "checklist": {
      "type": "array",
      "items": {
        "type": "string"
      }
    },
    "acceptance_criteria": {
      "type": "array",
      "items": {
        "type": "string"
      }
    }
  },
  "required": [
    "type",
    "requirement",
    "id",
    "description",
    "inspection_method",
    "checklist",
    "acceptance_criteria"
  ]
}
```

### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `type` | string | Yes | Must be "inspection" |
| `requirement` | string | Yes | Requirement identifier |
| `id` | string | Yes | Inspection identifier |
| `description` | string | Yes | Inspection description |
| `inspection_method` | string | Yes | Method description |
| `checklist` | array | Yes | Array of checklist item strings |
| `acceptance_criteria` | array | Yes | Array of criteria strings |

### Complete Example

```yaml
type: inspection
requirement: "XXX400"
id: '4.2.2.6 IN_eUICC_Security_Compliance'
description: 'Inspection of eUICC implementation for security compliance'
inspection_method: 'Code review, configuration audit, and documentation verification'

checklist:
  - "Verify cryptographic algorithms comply with GSMA SGP.22"
  - "Confirm secure channel protocol implementation follows TLS 1.2+"
  - "Check certificate chain validation includes root CA verification"
  - "Validate private keys are stored in secure element"
  - "Review authentication mechanisms for SM-DP+ communication"
  - "Inspect error handling to ensure no information leakage"
  - "Verify secure boot implementation"
  - "Check memory protection mechanisms"
  - "Confirm audit logging captures security events"
  - "Review source code for common vulnerabilities"

acceptance_criteria:
  - "All cryptographic algorithms must be from approved list"
  - "No cryptographic keys stored in plaintext"
  - "Certificate validation must reject invalid certificates"
  - "All security operations must be logged"
  - "No critical security vulnerabilities in code review"
```

---

## Common Patterns

### Pattern 1: Type Discrimination

All verification method schemas use the `type` field with an enum to ensure correct type:

```json
"type": {
  "type": "string",
  "enum": ["test"]
}
```

This enables automatic schema selection based on the YAML `type` field.

### Pattern 2: Flexible Conditions Object

Initial conditions use `additionalProperties` to allow any entity names:

```json
"initial_conditions": {
  "type": "object",
  "additionalProperties": {
    "type": "array",
    "items": {
      "type": "string"
    }
  }
}
```

This allows:
```yaml
initial_conditions:
  eUICC:
    - "Condition 1"
  SM-DP+:
    - "Condition A"
  Any_Entity_Name:
    - "Condition X"
```

### Pattern 3: String Arrays for Lists

Observations, acceptance criteria, and checklists use string arrays:

```json
"acceptance_criteria": {
  "type": "array",
  "items": {
    "type": "string"
  }
}
```

### Pattern 4: Nested Required Fields

Complex nested structures specify required fields at each level:

```json
"expected": {
  "type": "object",
  "properties": {
    "success": { "type": "boolean" },
    "result": { "type": "string" },
    "output": { "type": "string" }
  },
  "required": ["result", "output"]
}
```

Note: `success` is optional, but `result` and `output` are required.

### Pattern 5: Optional Parameters

Use required/optional distinction for flexible data:

```json
"parameters": {
  "type": "object",
  "additionalProperties": {
    "type": "string"
  }
}
```

This field is optional (not in `required` array), and when present, allows any parameters.

## Validation Rules

### Type Validation

- Strings must be actual strings, not numbers
- Integers must be whole numbers
- Booleans must be `true` or `false`
- Arrays must be actual arrays `[]`
- Objects must be actual objects `{}`

### Common Validation Errors

#### Missing Required Field

```
Error: "type" is a required property
```

**Solution**: Add the missing field to your YAML file.

#### Wrong Type

```
Error: "item" is not of type "integer"
```

**Solution**: Ensure `item: 1` (not `item: "1"` as string).

#### Invalid Enum Value

```
Error: "type" must be one of ["test"]
```

**Solution**: Ensure `type: test` matches the expected enum value.

#### Missing Nested Required Field

```
Error: "test_sequences[0].steps[1].expected.result" is a required property
```

**Solution**: Add `result` field to the `expected` object in step 2 of sequence 1.

### Best Practices

1. **Use Explicit Types**: Don't mix types (e.g., don't use `"1"` when schema expects `1`)
2. **Include All Required Fields**: Check schema for required fields
3. **Follow Structure**: Nested objects must match schema structure
4. **Validate Early**: Validate data files before rendering
5. **Use Schema-Aware Editor**: IDE with JSON Schema support can help

## Examples

### Minimal Valid Test Case

```yaml
type: test
requirement: "REQ-001"
item: 1
tc: 1
id: "TC-001"
description: "Basic test"
general_initial_conditions:
  System:
    - "Ready"
initial_conditions:
  Device:
    - "Operational"
test_sequences:
  - id: 1
    name: "Sequence 1"
    description: "Test sequence"
    initial_conditions:
      Device:
        - "Initialized"
    steps:
      - step: 1
        description: "Execute command"
        command: "run"
        expected:
          result: "Success"
          output: "OK"
```

### Minimal Valid Analysis Case

```yaml
type: analysis
requirement: "REQ-002"
id: "AN-001"
description: "Performance analysis"
method: "Statistical analysis"
calculations:
  - name: "Throughput"
    formula: "T = N / t"
models:
  - name: "Performance model"
    type: "Mathematical"
    description: "Linear model"
acceptance_criteria:
  - "Throughput > 100 ops/sec"
```

### Minimal Valid Demonstration

```yaml
type: demonstration
requirement: "REQ-003"
id: "DM-001"
description: "Feature demonstration"
procedure: "1. Start system\n2. Run test"
observations:
  - "System started successfully"
acceptance_criteria:
  - "System must start within 10 seconds"
```

### Minimal Valid Inspection

```yaml
type: inspection
requirement: "REQ-004"
id: "IN-001"
description: "Code review"
inspection_method: "Manual review"
checklist:
  - "Check code quality"
acceptance_criteria:
  - "No critical issues found"
```

## Summary

This reference covered:

- **Container Schema**: Document metadata structure
- **Test Schema**: Executable test cases with sequences and steps
- **Analysis Schema**: Analytical verification with calculations and models
- **Demonstration Schema**: Live demonstration documentation
- **Inspection Schema**: Manual inspection and review
- **Validation Rules**: Common errors and best practices
- **Examples**: Minimal and complete examples for each schema type

For template usage with these schemas, see [template-authoring-guide.md](template-authoring-guide.md).

For data transformation during rendering, see [filters-reference.md](filters-reference.md).
