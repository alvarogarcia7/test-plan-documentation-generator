## Test Case: TEST_FAILING_002

**Requirement**: TEST_REQ_002

**Item**: 1

**Sanitized Test ID**: TEST_FAILING_002

## Description

Test case with some failing verifications


## General Initial Conditions

| **Entity** | **Description of the general initial condition** |
| ------------- | --------- |
| System | Ready |



## Test Sequence 1 Mixed Results Sequence

Some steps pass, some fail


### Initial Conditions for This Sequence

| **Entity** | **Description** |
| ---------- | --------------- |
| LPA | Active |



| **Step Number** | **Action** | **Expected Result** | **Expected Output** |
| --------------- | ---------- | ------------------- | --------------------|
| 1 | Echo that passes | 0 | pass |
| 2 | Command that should fail | 0 | expected |
| 3 | Exit code mismatch | 0 |  |




## Test Case: TEST_PASSING_001

**Requirement**: TEST_REQ_001

**Item**: 1

**Sanitized Test ID**: TEST_PASSING_001

## Description

Test case with all passing verifications


## General Initial Conditions

| **Entity** | **Description of the general initial condition** |
| ------------- | --------- |
| System | Ready |



## Test Sequence 1 Passing Sequence

All steps should pass


### Initial Conditions for This Sequence

| **Entity** | **Description** |
| ---------- | --------------- |
| LPA | Active |



| **Step Number** | **Action** | **Expected Result** | **Expected Output** |
| --------------- | ---------- | ------------------- | --------------------|
| 1 | Echo hello | 0 | hello |
| 2 | True command | 0 |  |


## Test Sequence 2 Second Passing Sequence

Another sequence that passes


### Initial Conditions for This Sequence

| **Entity** | **Description** |
| ---------- | --------------- |
| System | Ready |



| **Step Number** | **Action** | **Expected Result** | **Expected Output** |
| --------------- | ---------- | ------------------- | --------------------|
| 1 | Echo world | 0 | world |




## Test Case: 4.2.2.2.1 TC_eUICC_ES6.UpdateMetadata

**Requirement**: XXX100

**Item**: 1

**Sanitized Test ID**: 4_2_2_2_1_TC_eUICC_ES6_UpdateMetadata

## Description

Throughout all the ES6.UpdateMetadata test cases, SMS is used as the secure OTA channel.


## General Initial Conditions

| **Entity** | **Description of the general initial condition** |
| ------------- | --------- |
| eUICC | The profile The PROFILE_OPERATIONAL1 with #METADATA_WITH_PPRS_AND_ICON is loaded on the eUICC. |



## Test Sequence 1 Test Sequence #01 Nominal: Unset PPR1

This test case verifies that the eUICC correctly processes an ES6.UpdateMetadata command to unset PPR1
when the profile is in the operational state and PPR1 is currently set.


### Initial Conditions for This Sequence

| **Entity** | **Description** |
| ---------- | --------------- |
| eUICC | The PROFILE_OPERATIONAL3 is Enabled. |



| **Step Number** | **Action** | **Expected Result** | **Expected Output** |
| --------------- | ---------- | ------------------- | --------------------|
| 1 | SENDS_SMS_PP([INSTALL_PERSO_RES_ISDP]; STORE_DATA_SCRIPT(#REMOVE_PPR1, FALSE)) | SW=0x91XX | This operation was successful. |
| 2 | Fetch 'XX' | MTD_CHECK_SMS_POR(0x9000) | This operation was successful. |


## Test Sequence 2 Test Sequence #02 Nominal: Unset PPPR2 and update icon

The purpose of this test is to verify that the MNO can unset PPR2 and update the icon and
icon type values from a Profile.


### Initial Conditions for This Sequence

| **Entity** | **Description** |
| ---------- | --------------- |
| eUICC | The PROFILE_OPERATIONAL3 is Enabled. |



| **Step Number** | **Action** | **Expected Result** | **Expected Output** |
| --------------- | ---------- | ------------------- | --------------------|
| 1 | SENDS_SMS_PP([INSTALL_PERSO_RES_ISDP]; STORE_DATA_SCRIPT(#REMOVE_PPR1, FALSE)) | SW=0x91XX | This operation was successful. |
| 2 | Fetch 'XX' | MTD_CHECK_SMS_POR(0x9000) | This operation was successful. |




## Test Case: 4.2.2.3 ANOTHER ONE

**Requirement**: XXX100

**Item**: 1

**Sanitized Test ID**: 4_2_2_3_ANOTHER_ONE

## Description

Throughout all the ES6.UpdateMetadata test cases, SMS is used as the secure OTA channel.


## General Initial Conditions

| **Entity** | **Description of the general initial condition** |
| ------------- | --------- |
| eUICC | The profile The PROFILE_OPERATIONAL1 with #METADATA_WITH_PPRS_AND_ICON is loaded on the eUICC. |



## Test Sequence 1 Test Sequence #01 Nominal: Unset PPR1

This test case verifies that the eUICC correctly processes an ES6.UpdateMetadata command to unset PPR1
when the profile is in the operational state and PPR1 is currently set.


### Initial Conditions for This Sequence

| **Entity** | **Description** |
| ---------- | --------------- |
| eUICC | The PROFILE_OPERATIONAL3 is Enabled. |



| **Step Number** | **Action** | **Expected Result** | **Expected Output** |
| --------------- | ---------- | ------------------- | --------------------|
| 1 | SENDS_SMS_PP([INSTALL_PERSO_RES_ISDP]; STORE_DATA_SCRIPT(#REMOVE_PPR1, FALSE)) | SW=0x91XX | This operation was successful. |
| 2 | Fetch 'XX' | MTD_CHECK_SMS_POR(0x9000) | This operation was successful. |


## Test Sequence 2 Test Sequence #02 Nominal: Unset PPPR2 and update icon

The purpose of this test is to verify that the MNO can unset PPR2 and update the icon and
icon type values from a Profile.


### Initial Conditions for This Sequence

| **Entity** | **Description** |
| ---------- | --------------- |
| eUICC | The PROFILE_OPERATIONAL3 is Enabled. |



| **Step Number** | **Action** | **Expected Result** | **Expected Output** |
| --------------- | ---------- | ------------------- | --------------------|
| 1 | SENDS_SMS_PP([INSTALL_PERSO_RES_ISDP]; STORE_DATA_SCRIPT(#REMOVE_PPR1, FALSE)) | SW=0x91XX | This operation was successful. |
| 2 | Fetch 'XX' | MTD_CHECK_SMS_POR(0x9000) | This operation was successful. |

