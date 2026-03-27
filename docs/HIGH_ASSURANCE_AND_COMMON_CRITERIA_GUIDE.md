# High Assurance and Common Criteria Verification Methods Guide

## Overview

This guide documents the High Assurance and Common Criteria verification method templates added to support aerospace and security-critical systems certification requirements.

## High Assurance Verification Method

### Purpose

The High Assurance verification method supports verification activities for safety-critical aerospace software following DO-178C/DO-254 standards. It is designed for systems requiring high levels of design assurance, particularly Design Assurance Level (DAL) A through E software.

### Location

- **Schema**: `data/verification_methods/high_assurance/schema.json`
- **Markdown Template**: `data/verification_methods/high_assurance/template.j2`
- **AsciiDoc Template**: `data/verification_methods/high_assurance/template_asciidoc.adoc`
- **Example**: `docs/examples/high-assurance-case-example.yml`

### Key Features

The High Assurance template captures:

1. **Design Assurance Level (DAL)**: Specifies the criticality level (A-E) per DO-178C
2. **Verification Objectives**: DO-178C Table A-6/A-7 objectives with independence requirements
3. **Traceability**: Bi-directional traceability from requirements through source code
4. **Structural Coverage**: Statement, Decision, and MC/DC coverage metrics
5. **Verification Activities**: Detailed activities with DO-178C references
6. **Configuration Management**: Baseline tracking and change management

### Design Assurance Levels

- **DAL-A**: Catastrophic - Software failure could cause fatalities
- **DAL-B**: Hazardous/Severe-Major - Serious or fatal injuries
- **DAL-C**: Major - Significant operational limitations
- **DAL-D**: Minor - Operating limitations, increased crew workload
- **DAL-E**: No Safety Effect

### Structural Coverage Requirements

| DAL Level | Coverage Requirements |
|-----------|----------------------|
| DAL-A | Statement, Decision, MC/DC |
| DAL-B | Statement, Decision |
| DAL-C | Statement |
| DAL-D | No specific requirement |
| DAL-E | No specific requirement |

### Schema Fields

**Required Fields**:
- `type`: Must be "high_assurance"
- `requirement`: Requirement identifier
- `id`: Verification activity identifier
- `description`: Description of verification activity
- `dal_level`: DAL level (A, B, C, D, or E)
- `verification_objectives`: Array of verification objectives with:
  - `objective_id`: Objective identifier (e.g., "VO-6.4.4.1")
  - `description`: Objective description
  - `independence_level`: "independent", "organizational", or "system"
  - `verification_methods`: Array of methods used
- `traceability`: Traceability data with:
  - `high_level_requirements`: Array of HLRs
  - `low_level_requirements`: Array of LLRs
  - `design_components`: Array of design components
  - `source_code`: Array of source code references
- `structural_coverage`: Coverage metrics with:
  - `statement_coverage`: Percentage (0-100)
  - `decision_coverage`: Percentage (0-100)
  - `mc_dc_coverage`: Percentage (0-100, optional for DAL-C/D/E)
  - `data_coupling`: Description (optional)
  - `control_coupling`: Description (optional)
- `verification_activities`: Array of detailed activities with:
  - `activity_id`: Activity identifier
  - `activity_name`: Activity name
  - `do178c_reference`: DO-178C section reference
  - `description`: Activity description
  - `completion_criteria`: Array of criteria
  - `evidence`: Array of evidence documents
- `configuration_management`: CM data with:
  - `baseline_id`: Baseline identifier
  - `change_tracking`: Array of changes (optional)
  - `problem_reports`: Array of PRs (optional)
- `acceptance_criteria`: Array of acceptance criteria

### Usage Example

```bash
./target/release/tpdg \
  --output ./verification_report.md \
  --container ./data/container/schema.json \
             ./data/container/template.j2 \
             ./data/container/data.yml \
  --test-case ./data/verification_methods \
              ./data/test_case/high_assurance_verification.yml
```

## Common Criteria Verification Method

### Purpose

The Common Criteria verification method supports security certification activities following ISO/IEC 15408 Common Criteria standards. It is designed for security-critical systems requiring formal security evaluation at various Evaluation Assurance Levels (EAL).

### Location

- **Schema**: `data/verification_methods/common_criteria/schema.json`
- **Markdown Template**: `data/verification_methods/common_criteria/template.j2`
- **AsciiDoc Template**: `data/verification_methods/common_criteria/template_asciidoc.adoc`
- **Example**: `docs/examples/common-criteria-case-example.yml`

### Key Features

The Common Criteria template captures:

1. **Evaluation Assurance Level (EAL)**: EAL1 through EAL7
2. **Protection Profile**: Reference to PP or Security Target
3. **Security Functional Requirements (SFR)**: Security functions the TOE must provide
4. **Security Assurance Requirements (SAR)**: Development and evaluation assurance
5. **TOE Security Functions (TSF)**: Implementation of security mechanisms
6. **Test Coverage Analysis**: Functional, interface, and SFR coverage metrics
7. **Vulnerability Assessment**: Penetration testing, covert/side channel analysis
8. **Evidence Documents**: Supporting documentation for evaluation

### Evaluation Assurance Levels

- **EAL1**: Functionally Tested - Basic assurance
- **EAL2**: Structurally Tested - Low to moderate assurance
- **EAL3**: Methodically Tested and Checked - Moderate assurance
- **EAL4**: Methodically Designed, Tested, and Reviewed - Moderate to high assurance
- **EAL5**: Semiformally Designed and Tested - High assurance
- **EAL6**: Semiformally Verified Design and Tested - Very high assurance
- **EAL7**: Formally Verified Design and Tested - Extremely high assurance

### Common Criteria Terminology

- **TOE**: Target of Evaluation (the product being certified)
- **PP**: Protection Profile (reusable security requirements)
- **ST**: Security Target (TOE-specific security claims)
- **SFR**: Security Functional Requirements (what security the TOE provides)
- **SAR**: Security Assurance Requirements (how TOE is developed/evaluated)
- **TSF**: TOE Security Functions (implementation of SFRs)
- **TSFI**: TSF Interfaces (external interfaces to security functions)

### Schema Fields

**Required Fields**:
- `type`: Must be "common_criteria"
- `requirement`: Requirement identifier
- `id`: Evaluation identifier
- `description`: TOE and evaluation description
- `evaluation_assurance_level`: EAL level (EAL1-EAL7)
- `protection_profile`: PP or ST reference
- `security_functional_requirements`: Array of SFRs with:
  - `sfr_id`: SFR identifier (e.g., "FCS_CKM.1")
  - `family`: CC Part 2 family name
  - `component`: Component name
  - `description`: SFR description
  - `evaluation_activities`: Array of evaluation activities
- `security_assurance_requirements`: Array of SARs with:
  - `sar_id`: SAR identifier (e.g., "ADV_ARC.1")
  - `class`: SAR class
  - `family`: SAR family
  - `component`: Component name
  - `description`: SAR description
  - `developer_actions`: Array of developer actions
  - `evaluator_actions`: Array of evaluator actions
- `toe_security_functions`: Array of TSFs with:
  - `tsf_id`: TSF identifier
  - `name`: TSF name
  - `description`: TSF description
  - `sfr_mapping`: Array of SFR IDs this TSF implements
- `test_coverage_analysis`: Coverage metrics with:
  - `functional_coverage`: Percentage (0-100)
  - `interface_coverage`: Percentage (0-100)
  - `sfr_coverage`: Percentage (0-100)
  - `security_mechanisms_tested`: Array of mechanisms
- `vulnerability_assessment`: Assessment results with:
  - `penetration_testing`: Array of pen test results with:
    - `test_id`: Test identifier
    - `attack_vector`: Attack description
    - `result`: Test result
    - `mitigation`: Mitigation description (optional)
  - `covert_channel_analysis`: Array of analyses (optional)
  - `side_channel_analysis`: Array of analyses (optional)
- `evidence_documents`: Array of evidence with:
  - `doc_id`: Document identifier
  - `title`: Document title
  - `version`: Version number
  - `relevance`: Why document is relevant
- `acceptance_criteria`: Array of acceptance criteria

### Usage Example

```bash
./target/release/tpdg \
  --format asciidoc \
  --output ./security_evaluation.adoc \
  --container ./data/container/schema.json \
             ./data/container/template_asciidoc.adoc \
             ./data/container/data.yml \
  --test-case ./data/verification_methods \
              ./data/test_case/cc_evaluation.yml
```

## Integration with Existing Verification Methods

The High Assurance and Common Criteria verification methods complement the existing methods:

| Method | Purpose | Standards |
|--------|---------|-----------|
| **test** | Functional testing | General |
| **analysis** | Mathematical/computational analysis | General |
| **demonstration** | Operational demonstration | General |
| **inspection** | Code/design review | General |
| **high_assurance** | Safety-critical verification | DO-178C, DO-254, ARP4754A |
| **common_criteria** | Security certification | ISO/IEC 15408, Common Criteria |
| **result** | Test execution results | General |

## Aerospace Standards Reference

### DO-178C
Software Considerations in Airborne Systems and Equipment Certification
- Primary standard for airborne software certification
- Defines software development and verification processes
- Specifies DAL-based objectives and verification independence

### DO-254
Design Assurance Guidance for Airborne Electronic Hardware
- Companion to DO-178C for hardware verification
- Similar DAL-based assurance levels

### ARP4754A
Guidelines for Development of Civil Aircraft and Systems
- System-level development and safety assessment
- Integrates with DO-178C and DO-254

### Common Criteria (ISO/IEC 15408)
Common Criteria for Information Technology Security Evaluation
- International standard for security certification
- Defines Protection Profiles and Security Targets
- Specifies EAL-based assurance levels

## Best Practices

### High Assurance Verification

1. **Establish Traceability Early**: Maintain bi-directional traceability throughout development
2. **Plan for Independence**: Identify activities requiring independent verification
3. **Structural Coverage Planning**: For DAL-A/B, plan MC/DC coverage from design phase
4. **Configuration Management**: Maintain rigorous baseline and change control
5. **Evidence Documentation**: Document all verification activities and results

### Common Criteria Evaluation

1. **Select Appropriate EAL**: Choose EAL based on threat environment and assurance needs
2. **Use Protection Profiles**: Leverage existing PPs when available
3. **Security Architecture**: Design clear security domains and TSF boundaries
4. **Vulnerability Testing**: Plan for comprehensive penetration testing
5. **Evidence Completeness**: Ensure all required evaluation deliverables are complete

## Summary

The High Assurance and Common Criteria verification methods provide comprehensive templates for documenting safety-critical and security-critical system verification activities. They support aerospace industry standards (DO-178C/DO-254) and international security certification (Common Criteria/ISO 15408), enabling organizations to generate consistent, auditable verification documentation for certification purposes.
