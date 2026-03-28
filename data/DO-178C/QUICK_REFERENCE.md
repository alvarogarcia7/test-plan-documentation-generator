# DO-178C Templates - Quick Reference

## Document Acronyms

| Acronym | Full Name | Purpose |
|---------|-----------|---------|
| **PSAC** | Plan for Software Aspects of Certification | Links applicant to certification authority |
| **SDP** | Software Development Plan | Describes development processes and lifecycle |
| **SVP** | Software Verification Plan | Describes verification strategy and procedures |
| **SCMP** | Software Configuration Management Plan | Establishes CM methods and processes |
| **SQAP** | Software Quality Assurance Plan | Establishes QA methods and processes |
| **SRD** | Software Requirements Data | High-level software requirements |
| **SDD** | Software Design Description | Architecture and low-level requirements |
| **SRS** | Software Requirements Standard | Requirements development standard |
| **SDS** | Software Design Standard | Design methods and constraints |
| **SCS** | Software Code Standard | Coding rules and language constraints |
| **SVCP** | Software Verification Cases and Procedures | Test cases and test procedures |
| **SVR** | Software Verification Results | Verification execution results |
| **SAS** | Software Accomplishment Summary | Primary compliance demonstration |

## Software Levels

| Level | Failure Classification | Description |
|-------|----------------------|-------------|
| **A** | Catastrophic | Failure may cause deaths |
| **B** | Hazardous | Failure may cause serious injuries |
| **C** | Major | Failure may cause discomfort |
| **D** | Minor | Failure has slight impact |
| **E** | No Effect | Failure has no effect on safety |

## Verification Objectives by Level

| Objective | Level A | Level B | Level C | Level D |
|-----------|---------|---------|---------|---------|
| Requirements-based testing | Yes | Yes | Yes | Yes |
| Low-level testing | Yes | Yes | Yes | - |
| Structural coverage (MC/DC) | Yes | - | - | - |
| Structural coverage (Decision) | - | Yes | - | - |
| Structural coverage (Statement) | - | - | Yes | - |
| Robustness testing | Yes | Yes | Yes | - |

## Test Types

### High-Level Tests (HLT)
- **Also known as:** Requirements-based tests, black-box tests
- **Purpose:** Verify software requirements
- **Environment:** Integrated software on target or HIL simulator
- **Traceability:** To high-level requirements (SRD)

### Low-Level Tests (LLT)
- **Also known as:** Unit tests, white-box tests
- **Purpose:** Verify software design (low-level requirements)
- **Environment:** Individual units with stubs
- **Traceability:** To low-level requirements (SDD)

## Coverage Metrics

### Statement Coverage
- Every statement executed at least once
- Required for: Level C

### Decision Coverage
- Every decision outcome (true/false) exercised
- Required for: Level B

### Modified Condition/Decision Coverage (MC/DC)
- Every condition independently affects decision outcome
- Required for: Level A
- Most rigorous coverage metric

## Key Processes

### Software Planning Process
- Produces: PSAC, SDP, SVP, SCMP, SQAP

### Software Development Process
- Produces: SRD, SDD, Source Code, Object Code

### Software Verification Process
- Produces: SVCP, SVR, Verification Coverage Analysis

### Software Configuration Management Process
- Produces: SCI, SECI, CM Records

### Software Quality Assurance Process
- Produces: QA Records, SQA Reviews

## Document Dependencies

```
PSAC (Plan)
  ├── SDP → SRD → SDD → Code
  ├── SVP → SVCP → SVR
  ├── SCMP → SCI, SECI
  └── SQAP → QA Records
            
SAS (Summary of all above)
```

## Typical Document Flow

1. **Planning Phase**
   - PSAC (defines certification approach)
   - SDP, SVP, SCMP, SQAP (define processes)
   - SRS, SDS, SCS (define standards)

2. **Requirements Phase**
   - SRD (high-level requirements)
   - Requirements reviews

3. **Design Phase**
   - SDD (low-level requirements, architecture)
   - Design reviews

4. **Implementation Phase**
   - Source code development
   - Code reviews
   - Unit testing (LLT)

5. **Integration Phase**
   - Integration testing
   - Requirements-based testing (HLT)

6. **Verification Phase**
   - SVCP (test procedures)
   - SVR (test results)
   - Coverage analysis

7. **Certification Phase**
   - SAS (accomplishment summary)
   - Certification authority review

## Template File Locations

```
data/DO-178C/
├── plans/
│   ├── PSAC/          # Plan for Software Aspects of Certification
│   ├── SDP/           # Software Development Plan
│   ├── SVP/           # Software Verification Plan
│   ├── SCMP/          # Software Configuration Management Plan
│   └── SQAP/          # Software Quality Assurance Plan
├── verification/
│   ├── SVCP/          # Software Verification Cases and Procedures
│   ├── SVR/           # Software Verification Results
│   ├── verification_methods/
│   │   ├── high_level_test/    # HLT templates and schema
│   │   └── low_level_test/     # LLT templates and schema
│   └── test_procedures/        # Example test case data
├── SAS/               # Software Accomplishment Summary
└── README.md          # Detailed documentation
```

## Quick Commands

### Generate PSAC (Markdown)
```bash
./target/release/tpdg \
  --output ./PSAC.md \
  --container ./data/DO-178C/plans/PSAC/schema.json \
             ./data/DO-178C/plans/PSAC/template.j2 \
             ./data/DO-178C/plans/PSAC/data.yml \
  --test-case ./data/DO-178C/verification/verification_methods
```

### Generate SVCP with Test Cases (AsciiDoc)
```bash
./target/release/tpdg \
  --format asciidoc \
  --output ./SVCP.adoc \
  --container ./data/DO-178C/verification/SVCP/schema.json \
             ./data/DO-178C/verification/SVCP/template_asciidoc.adoc \
             ./data/DO-178C/verification/SVCP/data.yml \
  --test-case ./data/DO-178C/verification/verification_methods \
              ./data/DO-178C/verification/test_procedures/*.yml
```

### Generate SAS (Markdown)
```bash
./target/release/tpdg \
  --output ./SAS.md \
  --container ./data/DO-178C/SAS/schema.json \
             ./data/DO-178C/SAS/template.j2 \
             ./data/DO-178C/SAS/data.yml \
  --test-case ./data/DO-178C/verification/verification_methods
```

## Common Customizations

### Add a Custom Test Case

1. Create YAML file:
```yaml
type: "high_level_test"  # or "low_level_test"
test_id: "HLT-XXX-001"
requirement_id: "SRS-XXX-100"
test_objective: "Your objective here"
# ... rest of fields per schema
```

2. Add to command line:
```bash
--test-case ... your-test.yml
```

### Customize Document Header

Edit the template `.j2` or `.adoc` file and modify the header section.

### Add Organization Logo

For AsciiDoc, add to template header:
```asciidoc
:company-logo: images/logo.png
```

## Validation

All data files are validated against JSON schemas:
- Container data → Container schema
- Test case data → Test case type schema

Validation errors will show:
- File path
- Schema path
- Error details

## Key DO-178C Terminology

| Term | Definition |
|------|------------|
| **Derived Requirement** | Software requirement not directly traceable to system requirements |
| **Dead Code** | Code that cannot be executed in operational configuration |
| **Deactivated Code** | Code not executed but present in executable |
| **Structural Coverage** | Measure of code exercised by tests |
| **Robustness Testing** | Testing of error handling and boundary conditions |
| **Partitioning** | Isolation of software by criticality level |

## References

- **DO-178C:** Software Considerations in Airborne Systems and Equipment Certification (2011)
- **DO-330:** Software Tool Qualification Considerations
- **DO-331:** Model-Based Development and Verification Supplement
- **DO-332:** Object-Oriented Technology Supplement
- **DO-333:** Formal Methods Supplement

## Support

For detailed usage examples, see `USAGE_EXAMPLES.md`  
For template customization, see `README.md`  
For tool documentation, see project root `README.md`
