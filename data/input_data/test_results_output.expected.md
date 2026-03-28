# Input Data Test Results

## Document Information

**Project:** Input Data Verification System

**Test Date:** 2026-03-16T14:33:36.258035Z

**Environment:** Development

**Platform:** Linux x86_64

**Executor:** test-automation-system

**Execution Duration:** 2.5 seconds


---

## Executive Summary

### Test Execution Overview

This document presents the results of test execution for Input Data Verification System.

| Metric | Value |
|--------|-------|
| Total Test Cases | 2 |
| Passed Test Cases | 1 |
| Failed Test Cases | 1 |

### Pass/Fail Summary

**Pass Rate:** 50%

**Overall Status:** ✗ **FAIL**


---

# Detailed Test Results

## Test Case: TEST_FAILING_002

**Description:** Test case with some failing verifications

**Requirement:** TEST_REQ_002

**Item:** 1

**TC:** 2

**Overall Status:** ✗ FAIL

### Execution Summary

| Metric | Count |
|--------|-------|
| Total Steps | 3 |
| Passed Steps | 1 |
| Failed Steps | 2 |
| Not Executed Steps | 0 |


### Test Sequence 1: Mixed Results Sequence

**Sequence Status:** ✗ FAIL

#### Step-by-Step Results

| Step | Description | Status |
|------|-------------|--------|
| 1 | Echo that passes | ✓ Pass |
| 2 | Command that should fail | ✗ Fail |
| 3 | Exit code mismatch | ✗ Fail |



#### Step 2 - Failure Details

**Description:** Command that should fail

**Reason:** Output mismatch: expected 'expected', got 'wrong'

| Property | Expected | Actual |
|----------|----------|--------|
| Result | 0 | 0 |
| Output | expected | wrong |
| Success | true | N/A |

#### Step 3 - Failure Details

**Description:** Exit code mismatch

**Reason:** Success mismatch: expected true, got false

| Property | Expected | Actual |
|----------|----------|--------|
| Result | 0 | 1 |
| Output |  |  |
| Success | true | N/A |




## Test Case: TEST_PASSING_001

**Description:** Test case with all passing verifications

**Requirement:** TEST_REQ_001

**Item:** 1

**TC:** 1

**Overall Status:** ✓ PASS

### Execution Summary

| Metric | Count |
|--------|-------|
| Total Steps | 3 |
| Passed Steps | 3 |
| Failed Steps | 0 |
| Not Executed Steps | 0 |


### Test Sequence 1: Passing Sequence

**Sequence Status:** ✓ PASS

#### Step-by-Step Results

| Step | Description | Status |
|------|-------------|--------|
| 1 | Echo hello | ✓ Pass |
| 2 | True command | ✓ Pass |




### Test Sequence 2: Second Passing Sequence

**Sequence Status:** ✓ PASS

#### Step-by-Step Results

| Step | Description | Status |
|------|-------------|--------|
| 1 | Echo world | ✓ Pass |


---

## Final Verification Verdict

### Overall Test Execution Status

| Verdict | Result |
|---------|--------|
| Overall Status | ✗ **FAIL** |
| Total Test Cases | 2 |
| Passed | 1 |
| Failed | 1 |

### Conclusion

1 test case(s) failed during execution. Review the detailed test results section for failure analysis and remediation actions.


---

## Requirements Summary

requirements_with_detail:
  - requirement: TEST_REQ_001
    items:
      - item: 1
        tc: 1
        id: TEST_PASSING_001
        pass: true
  - requirement: TEST_REQ_002
    items:
      - item: 1
        tc: 2
        id: TEST_FAILING_002
        pass: false
status_per_requirement:
  - requirement: TEST_REQ_001
    pass: true
  - requirement: TEST_REQ_002
    pass: false
requirements_by_status:
  pass:
    - TEST_REQ_001
  fail:
    - TEST_REQ_002

