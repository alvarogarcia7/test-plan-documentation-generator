---
id: TPDG-2
title: 'Gitlab pipeline: one single docker layer'
status: Done
assignee: []
created_date: '2026-03-11 13:44'
updated_date: '2026-03-12 07:52'
labels: []
dependencies: []
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
* Export docker image as a single image (instead of builder/app)
* The image can work offline (no issues with crates.io)
* You can build on top of this image offline as well.
* Contents of the docker image:
  * compile tests
  * run tests
<!-- SECTION:DESCRIPTION:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
* Implemented vendoring (https://doc.rust-lang.org/cargo/commands/cargo-vendor.html)
<!-- SECTION:NOTES:END -->
