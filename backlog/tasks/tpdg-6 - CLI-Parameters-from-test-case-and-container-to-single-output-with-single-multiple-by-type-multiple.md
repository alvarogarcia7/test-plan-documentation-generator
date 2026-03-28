---
id: TPDG-6
title: >-
  CLI Parameters: from --test-case and --container to single output with
  --single, --multiple-by-type, --multiple
status: In Progress
assignee: []
created_date: '2026-03-28 11:57'
labels: []
dependencies: []
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Replace the --container and --test-cases for several invocations of the program:

Keep the TPDG as a program that hydrates a template with data.

For --container, replace it for --single: one input file, one template, one output file
for --test_case, replace it for --multiple-by-type: .type (.type is the path to the attribute in the yml): multiple input files, one template, one output file.

also create --multiple: multiple input files, one template, one output file. --multiple does not need to specify the type, as they are all the same.
<!-- SECTION:DESCRIPTION:END -->
