# DO-178C Templates - Complete Index

## Quick Start

1. **Read**: [README.md](README.md) - Overview and document descriptions
2. **Reference**: [QUICK_REFERENCE.md](QUICK_REFERENCE.md) - Quick reference guide
3. **Examples**: [USAGE_EXAMPLES.md](USAGE_EXAMPLES.md) - Practical usage examples
4. **Details**: [IMPLEMENTATION_NOTES.md](IMPLEMENTATION_NOTES.md) - Implementation details

## Document Templates

### Planning Documents

| Document | Directory | Description | Status |
|----------|-----------|-------------|--------|
| **PSAC** | `plans/PSAC/` | Plan for Software Aspects of Certification | ✅ Complete |
| **SVP** | `plans/SVP/` | Software Verification Plan | ✅ Complete |
| **SDP** | `plans/SDP/` | Software Development Plan | 📁 Directory created |
| **SCMP** | `plans/SCMP/` | Software Configuration Management Plan | 📁 Directory created |
| **SQAP** | `plans/SQAP/` | Software Quality Assurance Plan | 📁 Directory created |

### Verification Documents

| Document | Directory | Description | Status |
|----------|-----------|-------------|--------|
| **SVCP** | `verification/SVCP/` | Software Verification Cases and Procedures | ✅ Complete |
| **SVR** | `verification/SVR/` | Software Verification Results | 📁 Directory created |

### Test Case Types

| Type | Directory | Description | Status |
|------|-----------|-------------|--------|
| **High-Level Test** | `verification/verification_methods/high_level_test/` | Requirements-based tests | ✅ Complete |
| **Low-Level Test** | `verification/verification_methods/low_level_test/` | Unit/white-box tests | ✅ Complete |
| **Review** | `verification/verification_methods/review/` | Review procedures | 📁 Directory created |
| **Analysis** | `verification/verification_methods/analysis/` | Analysis procedures | 📁 Directory created |

### Summary Document

| Document | Directory | Description | Status |
|----------|-----------|-------------|--------|
| **SAS** | `SAS/` | Software Accomplishment Summary | ✅ Complete |

## Example Test Cases

| File | Type | Description |
|------|------|-------------|
| `test_procedures/HLT-NAV-001.yml` | High-Level Test | GPS position computation verification |
| `test_procedures/HLT-NAV-002.yml` | High-Level Test | Multi-sensor navigation verification |
| `test_procedures/LLT-NAV-001.yml` | Low-Level Test | Position computation unit test (normal) |
| `test_procedures/LLT-NAV-002.yml` | Low-Level Test | Position computation (error handling) |
| `test_procedures/LLT-NAV-003.yml` | Low-Level Test | Coordinate validation (boundary) |

## File Structure

```
data/DO-178C/
│
├── 📄 README.md                     Main documentation
├── 📄 QUICK_REFERENCE.md            Quick reference guide
├── 📄 USAGE_EXAMPLES.md             Usage examples
├── 📄 IMPLEMENTATION_NOTES.md       Implementation details
├── 📄 INDEX.md                      This file
│
├── 📁 plans/                        Planning documents
│   ├── 📁 PSAC/                    ✅ Complete
│   │   ├── schema.json
│   │   ├── template.j2
│   │   ├── template_asciidoc.adoc
│   │   └── data.yml
│   ├── 📁 SVP/                     ✅ Complete
│   │   ├── schema.json
│   │   ├── template.j2
│   │   ├── template_asciidoc.adoc
│   │   └── data.yml
│   ├── 📁 SDP/                     📁 Directory created
│   ├── 📁 SCMP/                    📁 Directory created
│   └── 📁 SQAP/                    📁 Directory created
│
├── 📁 verification/                 Verification documents
│   ├── 📁 SVCP/                    ✅ Complete
│   │   ├── schema.json
│   │   ├── template.j2
│   │   ├── template_asciidoc.adoc
│   │   └── data.yml
│   ├── 📁 SVR/                     📁 Directory created
│   ├── 📁 verification_methods/     Verification method templates
│   │   ├── 📁 high_level_test/     ✅ Complete
│   │   │   ├── schema.json
│   │   │   ├── template.j2
│   │   │   └── template_asciidoc.adoc
│   │   ├── 📁 low_level_test/      ✅ Complete
│   │   │   ├── schema.json
│   │   │   ├── template.j2
│   │   │   └── template_asciidoc.adoc
│   │   ├── 📁 review/              📁 Directory created
│   │   └── 📁 analysis/            📁 Directory created
│   └── 📁 test_procedures/          Example test cases
│       ├── HLT-NAV-001.yml         ✅ High-level test 1
│       ├── HLT-NAV-002.yml         ✅ High-level test 2
│       ├── LLT-NAV-001.yml         ✅ Low-level test 1
│       ├── LLT-NAV-002.yml         ✅ Low-level test 2
│       └── LLT-NAV-003.yml         ✅ Low-level test 3
│
└── 📁 SAS/                          Software Accomplishment Summary
    ├── schema.json                  ✅ Complete
    ├── template.j2
    ├── template_asciidoc.adoc
    └── data.yml
```

## Complete Templates (✅)

The following templates are fully implemented with schemas, Markdown templates, AsciiDoc templates, and example data:

1. **PSAC** - Plan for Software Aspects of Certification
2. **SVP** - Software Verification Plan  
3. **SVCP** - Software Verification Cases and Procedures
4. **SAS** - Software Accomplishment Summary
5. **High-Level Test** - Requirements-based test template
6. **Low-Level Test** - Unit test template

## Quick Usage

### Generate a Complete SVCP Document

```bash
./target/release/tpdg \
  --output ./output/SVCP.md \
  --container ./data/DO-178C/verification/SVCP/schema.json \
             ./data/DO-178C/verification/SVCP/template.j2 \
             ./data/DO-178C/verification/SVCP/data.yml \
  --test-case ./data/DO-178C/verification/verification_methods \
              ./data/DO-178C/verification/test_procedures/*.yml
```

### Generate All Planning Documents

```bash
# PSAC
./target/release/tpdg --output ./output/PSAC.md \
  --container ./data/DO-178C/plans/PSAC/schema.json \
             ./data/DO-178C/plans/PSAC/template.j2 \
             ./data/DO-178C/plans/PSAC/data.yml \
  --test-case ./data/DO-178C/verification/verification_methods

# SVP
./target/release/tpdg --output ./output/SVP.md \
  --container ./data/DO-178C/plans/SVP/schema.json \
             ./data/DO-178C/plans/SVP/template.j2 \
             ./data/DO-178C/plans/SVP/data.yml \
  --test-case ./data/DO-178C/verification/verification_methods

# SAS
./target/release/tpdg --output ./output/SAS.md \
  --container ./data/DO-178C/SAS/schema.json \
             ./data/DO-178C/SAS/template.j2 \
             ./data/DO-178C/SAS/data.yml \
  --test-case ./data/DO-178C/verification/verification_methods
```

## Documentation Hierarchy

1. **INDEX.md** (this file) - Quick navigation and overview
2. **README.md** - Comprehensive documentation
3. **QUICK_REFERENCE.md** - DO-178C quick reference
4. **USAGE_EXAMPLES.md** - Practical examples
5. **IMPLEMENTATION_NOTES.md** - Technical implementation details

## Key Features

- ✅ Schema validation for all data files
- ✅ Dual format support (Markdown and AsciiDoc)
- ✅ Test case integration with templates
- ✅ Traceability support in test cases
- ✅ DO-178C software levels (A-E) support
- ✅ Realistic aviation examples (Flight Management System)
- ✅ Multiple test types (normal, boundary, error handling)
- ✅ Coverage requirements (MC/DC, decision, statement)

## Software Levels Supported

| Level | Classification | Templates Support |
|-------|----------------|-------------------|
| A | Catastrophic | ✅ Full support |
| B | Hazardous | ✅ Full support |
| C | Major | ✅ Full support |
| D | Minor | ✅ Full support |
| E | No Effect | ✅ Full support |

## Next Steps for Users

1. **Review Examples**: Look at the example data files to understand structure
2. **Customize Data**: Copy and modify data.yml files for your project
3. **Add Test Cases**: Create test case YAML files following the schemas
4. **Generate Documents**: Run tpdg to generate your documentation
5. **Review Output**: Examine generated Markdown/AsciiDoc
6. **Convert to PDF**: Use pandoc or asciidoctor-pdf for final documents

## Extension Points

For future extensions, the following can be added:

- Additional planning documents (SDP, SCMP, SQAP)
- Requirements documents (SRD, SDD)
- Standards documents (SRS, SDS, SCS)
- Additional verification methods (review, analysis, demonstration, inspection)
- Requirements traceability matrices
- Coverage analysis reports
- Tool qualification data (DO-330)

## Support

- For tool usage: See main project README.md
- For DO-178C questions: See QUICK_REFERENCE.md
- For examples: See USAGE_EXAMPLES.md
- For customization: See template files directly

## License

These templates are provided as part of the Test Plan Documentation Generator project. Refer to the project license for terms of use.
