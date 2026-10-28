---
id: TPDG-8
title: Binary releases from GitLab CI for Linux amd64 and arm64
status: To Do
assignee: []
created_date: '2026-10-28T04:57:00Z'
labels: []
dependencies: []
ordinal: 4000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Build tpdg release archives for Linux amd64 and arm64 on version tags, publish them to the GitLab package registry and a GitLab Release, with checksums. Modeled on tcms-cli commit 8aa4c827 (TCMS-39). arm64 is built but disabled behind the RELEASE_ARM64 variable (default off).
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 Pushing a tag matching ^v.+ (for example v0.1.0) starts the release stage; pushing a branch or a non-v tag runs no release job.
- [ ] #2 release:build and release:build-arm64 wait for the ci and test-image jobs; a failing one blocks the release.
- [ ] #3 Each amd64 archive is named tpdg-<tag>-linux-amd64.zip and contains the tpdg binary and README.md, plus LICENSE when the repo has one.
- [ ] #4 SHA256SUMS lists every archive, and 'sha256sum -c SHA256SUMS' passes on the downloaded files.
- [ ] #5 From the extracted amd64 archive, './tpdg --help' exits 0.
- [ ] #6 'readelf -V tpdg | grep -o GLIBC_[0-9.]* | sort -V | tail -1' reports the highest glibc symbol version of each Linux binary. The amd64 value is recorded in the release description. The arm64 value must not exceed GLIBC_2.17.
- [ ] #7 The arm64 binary is ELF aarch64: 'file tpdg' reports aarch64 for the extracted arm64 archive.
- [ ] #8 With RELEASE_ARM64 unset or false, no arm64 job appears in the pipeline and the GitLab Release links only the amd64 archive and SHA256SUMS.
- [ ] #9 With RELEASE_ARM64=true, an arm64 build failure fails the release pipeline; it is not skipped with a warning.
- [ ] #10 With RELEASE_ARM64=true, the GitLab Release links both archives, and each link returns HTTP 200.
- [ ] #11 The release job image and the Dockerfile base image are both rust:1.92.
- [ ] #12 make test and make docker-build pass after the change.
- [ ] #13 .github/workflows/release.yml is unchanged.
<!-- AC:END -->
