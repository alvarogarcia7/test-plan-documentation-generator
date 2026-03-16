# GitHub Actions CI Pipeline Verification Guide

This document provides comprehensive instructions for verifying the GitHub Actions CI pipeline status for the `unmodified_push_2026-03-12` branch.

## Overview

The GitHub Actions CI pipeline runs automatically on push to specific branches and on pull requests. The pipeline includes the following jobs:

### Job 1: build-test-lint
This job performs all the critical build, test, and lint operations:

1. **Check formatting** (`cargo fmt -- --check`)
2. **Run clippy** (`cargo clippy --all-targets --all-features -- -D warnings`)
3. **Check project** (`cargo check`)
4. **Build project** (`cargo build --release`)
5. **Run unit tests** (`cargo test --release --all-features --tests`)
6. **Run E2E test (markdown)** - Tests markdown output generation
7. **Run E2E test (asciidoc test plan)** - Tests asciidoc test plan generation
8. **Run E2E test (asciidoc test results)** - Tests asciidoc test results generation
9. **Build Docker image** - Ensures Docker image builds successfully
10. **Show sccache stats** - Displays build cache statistics

### Job 2: coverage
This job runs test coverage analysis:

1. **Install cargo-tarpaulin** - Code coverage tool
2. **Run tests with coverage** - Generates coverage report
3. **Upload coverage to Codecov** - Uploads coverage data (requires CODECOV_TOKEN)

## Automated Verification Scripts

Two automated verification scripts are provided:

### Bash Script (Linux/macOS)

Use the provided `verify-github-actions.sh` script to automatically verify the CI pipeline status:

```bash
# Verify the specific branch
./verify-github-actions.sh unmodified_push_2026-03-12

# Or verify current branch
./verify-github-actions.sh

# Or use make
make verify-github-actions
```

### Python Script (Cross-platform)

For cross-platform compatibility (Windows/Linux/macOS), use the Python script:

```bash
# Verify the specific branch
python3 verify_github_actions.py unmodified_push_2026-03-12

# Or verify with default branch
python3 verify_github_actions.py
```

**Requirements:**
- Python 3.6 or later
- GitHub CLI (`gh`) installed and authenticated

### Script Features

Both verification scripts provide the same functionality:
- ✅ Checks if GitHub CLI (`gh`) is installed
- ✅ Fetches the latest workflow run for the specified branch
- ✅ Displays overall workflow status and conclusion
- ✅ Lists all jobs and their individual statuses
- ✅ Shows detailed step-by-step results for each job
- ✅ Verifies all critical steps (formatting, clippy, tests, Docker build)
- ✅ Provides actionable commands for troubleshooting failures
- ✅ Color-coded output for easy visual verification

### Script Output

The script provides detailed output including:

```
GitHub Actions CI Pipeline Verification
==========================================

✓ GitHub CLI (gh) is installed

Branch: unmodified_push_2026-03-12
Commit: bf74f35 (bf74f3583f7cf3ad6ae040f7eef3e7e9a1a5820f)

Workflow: CI
Run ID: 1234567890
Status: completed
Conclusion: success
URL: https://github.com/owner/repo/actions/runs/1234567890

✓ Overall workflow conclusion: success

Checking individual jobs...

✓ build-test-lint
✓ coverage

Detailed job steps verification:
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

build-test-lint job steps:
  ✓ Set up job
  ✓ Check formatting
  ✓ Run clippy
  ✓ Build project
  ✓ Run unit tests
  ✓ Run E2E test (markdown)
  ✓ Run E2E test (asciidoc test plan)
  ✓ Run E2E test (asciidoc test results)
  ✓ Build Docker image
  ✓ Complete job

coverage job steps:
  ✓ Set up job
  ✓ Run tests with coverage
  ✓ Upload coverage to Codecov
  ✓ Complete job

✅ VERIFICATION PASSED

All GitHub Actions CI jobs completed successfully for branch 'unmodified_push_2026-03-12'
```

## Manual Verification Methods

### Option 1: Using GitHub CLI (gh)

#### Prerequisites
Install GitHub CLI if not already installed:

```bash
# macOS
brew install gh

# Linux (Debian/Ubuntu)
sudo apt install gh

# Linux (Fedora/RHEL)
sudo dnf install gh

# Windows
winget install GitHub.cli
```

Authenticate with GitHub:
```bash
gh auth login
```

#### Check Workflow Status

```bash
# List recent workflow runs for the branch
gh run list --branch unmodified_push_2026-03-12 --limit 5

# View detailed information about the latest run
gh run view --branch unmodified_push_2026-03-12

# Watch a running workflow
gh run watch <run-id>

# View logs for a specific run
gh run view <run-id> --log

# View only failed logs
gh run view <run-id> --log-failed

# Open the workflow run in browser
gh run view <run-id> --web
```

#### Check Specific Job Status

```bash
# Get the latest run ID
RUN_ID=$(gh run list --branch unmodified_push_2026-03-12 --limit 1 --json databaseId --jq '.[0].databaseId')

# View job details in JSON format
gh run view $RUN_ID --json jobs | jq '.jobs[] | {name: .name, conclusion: .conclusion, status: .status}'

# View specific job logs
gh run view $RUN_ID --log --job <job-id>
```

### Option 2: Using GitHub Web Interface

1. **Navigate to the repository on GitHub**
   ```
   https://github.com/<owner>/<repo>
   ```

2. **Go to Actions tab**
   ```
   https://github.com/<owner>/<repo>/actions
   ```

3. **Filter by branch**
   - Click the "Branch" dropdown
   - Select `unmodified_push_2026-03-12`

4. **Verify latest workflow run**
   - Check that the latest workflow has a ✓ (green checkmark)
   - Click on the workflow run to see details

5. **Verify individual jobs**
   - Expand `build-test-lint` job
     - ✅ Check formatting
     - ✅ Run clippy
     - ✅ Check project
     - ✅ Build project
     - ✅ Run unit tests
     - ✅ Run E2E test (markdown)
     - ✅ Run E2E test (asciidoc test plan)
     - ✅ Run E2E test (asciidoc test results)
     - ✅ Build Docker image
     - ✅ Show sccache stats
   
   - Expand `coverage` job
     - ✅ Install cargo-tarpaulin
     - ✅ Run tests with coverage
     - ✅ Upload coverage to Codecov

### Option 3: Using GitHub API

#### Prerequisites
Create a GitHub personal access token:
1. Go to https://github.com/settings/tokens
2. Generate new token (classic)
3. Select scopes: `repo`, `workflow`
4. Copy the token

#### Check Workflow Status with API

```bash
# Set your GitHub token
export GITHUB_TOKEN="your-token-here"

# Set repository details
OWNER="your-username"
REPO="test-plan-documentation-generator"
BRANCH="unmodified_push_2026-03-12"

# Get latest workflow run for the branch
curl -H "Authorization: token $GITHUB_TOKEN" \
     -H "Accept: application/vnd.github.v3+json" \
     "https://api.github.com/repos/$OWNER/$REPO/actions/runs?branch=$BRANCH&per_page=1" \
     | jq '.workflow_runs[0] | {id: .id, status: .status, conclusion: .conclusion, head_sha: .head_sha, created_at: .created_at}'

# Get workflow run details
RUN_ID="<run-id-from-above>"
curl -H "Authorization: token $GITHUB_TOKEN" \
     -H "Accept: application/vnd.github.v3+json" \
     "https://api.github.com/repos/$OWNER/$REPO/actions/runs/$RUN_ID" \
     | jq '{status: .status, conclusion: .conclusion, html_url: .html_url}'

# Get job details for the run
curl -H "Authorization: token $GITHUB_TOKEN" \
     -H "Accept: application/vnd.github.v3+json" \
     "https://api.github.com/repos/$OWNER/$REPO/actions/runs/$RUN_ID/jobs" \
     | jq '.jobs[] | {name: .name, status: .status, conclusion: .conclusion, steps: [.steps[] | {name: .name, conclusion: .conclusion}]}'
```

### Option 4: Using Git Hooks

Create a post-push hook to automatically open the Actions page:

```bash
# Create the hook file
cat > .git/hooks/post-push << 'EOF'
#!/bin/bash
BRANCH=$(git rev-parse --abbrev-ref HEAD)
REMOTE_URL=$(git remote get-url origin)

# Extract owner and repo from GitHub URL
if [[ $REMOTE_URL =~ github\.com[:/]([^/]+)/([^.]+) ]]; then
    OWNER="${BASH_REMATCH[1]}"
    REPO="${BASH_REMATCH[2]}"
    echo "GitHub Actions: https://github.com/$OWNER/$REPO/actions?query=branch%3A$BRANCH"
fi
EOF

chmod +x .git/hooks/post-push
```

## Expected Results

After verifying the `unmodified_push_2026-03-12` branch, all jobs should show:

### ✅ build-test-lint Job
- **Status**: Completed
- **Conclusion**: Success
- **Duration**: ~5-10 minutes (depending on cache)

**Key Steps:**
1. ✅ Check formatting - Code passes `rustfmt` checks
2. ✅ Run clippy - No clippy warnings (using `-D warnings`)
3. ✅ Check project - `cargo check` succeeds
4. ✅ Build project - Release build completes
5. ✅ Run unit tests - All unit tests pass
6. ✅ Run E2E test (markdown) - Markdown output matches expected
7. ✅ Run E2E test (asciidoc test plan) - AsciiDoc test plan matches expected
8. ✅ Run E2E test (asciidoc test results) - AsciiDoc test results matches expected
9. ✅ Build Docker image - Docker image builds successfully
10. ✅ Show sccache stats - Cache statistics displayed

### ✅ coverage Job
- **Status**: Completed
- **Conclusion**: Success
- **Duration**: ~3-5 minutes

**Key Steps:**
1. ✅ Install cargo-tarpaulin - Coverage tool installed
2. ✅ Run tests with coverage - Tests pass with ≥70% coverage
3. ✅ Upload coverage to Codecov - Coverage data uploaded successfully

## Troubleshooting

### Issue: "No workflow runs found"

**Cause**: The branch hasn't been pushed or no workflows were triggered

**Solution**:
```bash
# Ensure the branch is pushed
git push origin unmodified_push_2026-03-12

# Check if workflows are triggered for this branch
# The workflow triggers on: push to main/master or pull requests
# Check .github/workflows/ci.yml for trigger conditions
```

### Issue: "GitHub CLI not found"

**Cause**: `gh` command is not installed

**Solution**:
```bash
# macOS
brew install gh

# Linux
# See https://github.com/cli/cli#installation
```

### Issue: Workflow is still running

**Cause**: Jobs take time to complete

**Solution**:
```bash
# Watch the workflow run in real-time
gh run watch <run-id>

# Or check status periodically
watch -n 10 'gh run list --branch unmodified_push_2026-03-12 --limit 1'
```

### Issue: Coverage job fails with "CODECOV_TOKEN required"

**Cause**: The Codecov token is not configured in repository secrets

**Solution**:
1. Get your Codecov token from https://codecov.io
2. Add it to GitHub repository secrets:
   - Go to Settings → Secrets and variables → Actions
   - Add new repository secret: `CODECOV_TOKEN`
   
Note: The coverage job may fail if this token is not set, but it doesn't block the main build-test-lint job.

### Issue: E2E tests fail with diff errors

**Cause**: Output doesn't match expected files

**Solution**:
```bash
# Run E2E tests locally to debug
cargo build --release

# Test markdown output
./target/release/tpdg \
  --output ./data/output.actual.md \
  --container ./data/container/schema.json ./data/container/template.j2 ./data/container/data.yml \
  --test-case ./data/verification_methods ./data/test_case/*.yml

# Compare outputs
diff ./data/output.actual.md ./data/output.expected.md
```

### Issue: Docker build fails

**Cause**: Docker daemon not available or Dockerfile issues

**Solution**:
```bash
# Test Docker build locally
make docker-build

# Or manually
docker build -t test:latest .
```

## Verification Checklist

Use this checklist to manually verify all aspects of the CI pipeline:

- [ ] **Workflow triggered**: Workflow run exists for the branch
- [ ] **Workflow completed**: Status is "completed" (not "in_progress" or "queued")
- [ ] **Workflow succeeded**: Conclusion is "success" (not "failure", "cancelled", or "timed_out")

**build-test-lint job:**
- [ ] **Formatting**: `cargo fmt -- --check` passes
- [ ] **Clippy**: `cargo clippy` passes with no warnings
- [ ] **Project check**: `cargo check` succeeds
- [ ] **Build**: `cargo build --release` completes
- [ ] **Unit tests**: `cargo test` passes all tests
- [ ] **E2E markdown**: Markdown output matches expected
- [ ] **E2E asciidoc test plan**: AsciiDoc test plan matches expected
- [ ] **E2E asciidoc test results**: AsciiDoc test results matches expected
- [ ] **Docker build**: Docker image builds successfully

**coverage job:**
- [ ] **Tarpaulin install**: Coverage tool installs successfully
- [ ] **Coverage tests**: Tests run with coverage ≥70%
- [ ] **Codecov upload**: Coverage data uploads (or job skips gracefully)

## Quick Reference Commands

```bash
# Verify using the provided script
./verify-github-actions.sh unmodified_push_2026-03-12

# List recent runs
gh run list --branch unmodified_push_2026-03-12

# View latest run
gh run view --branch unmodified_push_2026-03-12

# Watch running workflow
gh run watch <run-id>

# View failed logs only
gh run view <run-id> --log-failed

# Open in browser
gh run view <run-id> --web

# Get JSON data for automation
gh run list --branch unmodified_push_2026-03-12 --json status,conclusion,headSha,createdAt

# Check specific commit
gh run list --commit bf74f3583f7cf3ad6ae040f7eef3e7e9a1a5820f
```

## Summary

The GitHub Actions CI pipeline for the `unmodified_push_2026-03-12` branch should complete successfully with all jobs passing. Use the provided verification script (`./verify-github-actions.sh`) for automated checking, or use the GitHub CLI/web interface for manual verification.

**Expected outcome**: ✅ All jobs pass, confirming the code is ready for merge/deployment.
