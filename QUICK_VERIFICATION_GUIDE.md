# Quick Verification Guide

## TL;DR - Verify CI Pipeline Status

### GitHub Actions (Recommended)

```bash
# Quick verification using make
make verify-github-actions

# Or using the bash script directly
./verify-github-actions.sh unmodified_push_2026-03-12

# Or using Python (cross-platform)
python3 verify_github_actions.py unmodified_push_2026-03-12
```

### GitLab CI (Alternative)

```bash
# Quick verification
make check-gitlab-pipeline

# Or using the script directly
./check-pipeline-status.sh
```

## What Gets Verified?

### ✅ GitHub Actions CI Pipeline

**Job 1: build-test-lint** (runs all critical checks)
- [x] Code formatting (`cargo fmt -- --check`)
- [x] Linting (`cargo clippy --all-targets --all-features -- -D warnings`)
- [x] Project check (`cargo check`)
- [x] Release build (`cargo build --release`)
- [x] Unit tests (`cargo test --release --all-features --tests`)
- [x] E2E test - Markdown output
- [x] E2E test - AsciiDoc test plan
- [x] E2E test - AsciiDoc test results  
- [x] Docker image build
- [x] sccache statistics

**Job 2: coverage** (optional but recommended)
- [x] Install cargo-tarpaulin
- [x] Run tests with ≥70% coverage
- [x] Upload to Codecov (requires CODECOV_TOKEN)

## Expected Output

### ✅ Success

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

✓ Overall workflow conclusion: success

Checking individual jobs...

✓ build-test-lint
✓ coverage

✅ VERIFICATION PASSED

All GitHub Actions CI jobs completed successfully
```

### ❌ Failure

```
GitHub Actions CI Pipeline Verification
==========================================

❌ Workflow run failed (conclusion: failure)

View failed jobs with:
  gh run view 1234567890 --log-failed

Or view in browser:
  https://github.com/owner/repo/actions/runs/1234567890
```

### ⚠️ Still Running

```
GitHub Actions CI Pipeline Verification
==========================================

⚠️  Workflow is still running (status: in_progress)

You can watch the workflow run with:
  gh run watch 1234567890

Or view it in your browser:
  https://github.com/owner/repo/actions/runs/1234567890
```

## Prerequisites

### Required Tools

1. **Git** - Must be installed
2. **GitHub CLI (`gh`)** - Required for automated verification

### Install GitHub CLI

```bash
# macOS
brew install gh

# Linux (Debian/Ubuntu)
sudo apt install gh

# Linux (Fedora/RHEL)  
sudo dnf install gh

# Windows
winget install GitHub.cli

# Or download from: https://github.com/cli/cli#installation
```

### Authenticate with GitHub

```bash
gh auth login
```

Follow the prompts to authenticate.

## Troubleshooting

### "No workflow runs found"

**Cause:** Branch hasn't been pushed or workflows not triggered

**Solution:**
```bash
git push origin unmodified_push_2026-03-12
```

### "GitHub CLI not found"

**Cause:** `gh` command not installed

**Solution:** Install GitHub CLI (see above)

### "Authentication required"

**Cause:** Not authenticated with GitHub

**Solution:**
```bash
gh auth login
```

### Workflow still running

**Cause:** Jobs take time to complete (5-10 minutes typically)

**Solution:** Wait or watch in real-time:
```bash
gh run watch <run-id>
```

## Manual Verification (Web Interface)

If you prefer to verify manually:

1. **Open GitHub Actions page:**
   ```
   https://github.com/<owner>/<repo>/actions
   ```

2. **Filter by branch:**
   - Click "Branch" dropdown
   - Select `unmodified_push_2026-03-12`

3. **Check latest run:**
   - Look for green ✓ checkmark
   - Click to see detailed job results

4. **Verify jobs:**
   - Expand `build-test-lint` - all steps should be ✓
   - Expand `coverage` - all steps should be ✓

## Using GitHub CLI Commands Directly

### List workflow runs
```bash
gh run list --branch unmodified_push_2026-03-12
```

### View latest run
```bash
gh run view --branch unmodified_push_2026-03-12
```

### Watch a running workflow
```bash
gh run watch <run-id>
```

### View failed logs only
```bash
gh run view <run-id> --log-failed
```

### Open in browser
```bash
gh run view <run-id> --web
```

### Get JSON data
```bash
gh run list --branch unmodified_push_2026-03-12 --json status,conclusion,headSha
```

## Complete Verification Checklist

Use this checklist to manually verify all aspects:

### Pre-verification
- [ ] Git repository is clean
- [ ] GitHub CLI (`gh`) is installed
- [ ] Authenticated with GitHub (`gh auth login`)
- [ ] Branch exists remotely (`git fetch origin`)

### Workflow Status
- [ ] Workflow run exists for the branch
- [ ] Workflow status is "completed" (not "in_progress" or "queued")
- [ ] Workflow conclusion is "success" (not "failure", "cancelled", "timed_out")

### build-test-lint Job Steps
- [ ] Check formatting - PASSED
- [ ] Run clippy - PASSED
- [ ] Check project - PASSED  
- [ ] Build project - PASSED
- [ ] Run unit tests - PASSED
- [ ] Run E2E test (markdown) - PASSED
- [ ] Run E2E test (asciidoc test plan) - PASSED
- [ ] Run E2E test (asciidoc test results) - PASSED
- [ ] Build Docker image - PASSED
- [ ] Show sccache stats - PASSED

### coverage Job Steps
- [ ] Install cargo-tarpaulin - PASSED
- [ ] Run tests with coverage - PASSED (≥70%)
- [ ] Upload coverage to Codecov - PASSED (or skipped if token not set)

### Final Verification
- [ ] All jobs completed successfully
- [ ] No failed steps
- [ ] Ready for merge/deployment

## Additional Resources

- **Detailed Guide:** [GITHUB_ACTIONS_VERIFICATION.md](GITHUB_ACTIONS_VERIFICATION.md)
- **GitLab CI Guide:** [PIPELINE_VERIFICATION.md](PIPELINE_VERIFICATION.md)
- **CI Workflow:** [.github/workflows/ci.yml](.github/workflows/ci.yml)
- **Agent Guide:** [AGENTS.md](AGENTS.md)

## Support

If verification fails:

1. **Check logs:**
   ```bash
   gh run view <run-id> --log-failed
   ```

2. **Run tests locally:**
   ```bash
   make test
   make lint
   make docker-build
   ```

3. **View in browser:**
   ```bash
   gh run view <run-id> --web
   ```

4. **Re-run failed jobs:**
   - Open workflow run in browser
   - Click "Re-run failed jobs"

## Summary

**Quick verification command:**
```bash
make verify-github-actions
```

**Expected result:** ✅ All jobs pass

**Time to complete:** ~5-10 minutes (depending on cache)

**What this validates:** Build, lint, unit tests, E2E tests, Docker build, and coverage
