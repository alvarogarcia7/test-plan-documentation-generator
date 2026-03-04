# GitLab CI Pipeline Verification Guide

This document provides instructions for verifying the GitLab CI pipeline status after applying the Rust version fixes.

## Changes Made

The following files were updated to fix the CI pipeline:

1. **Dockerfile**
   - Changed `FROM rust:1.92-bookworm` to `FROM rust:1.83-bookworm` (2 instances)
   
2. **.gitlab-ci.yml**
   - Changed `image: rust:latest` to `image: rust:1.83-bookworm` in the `ci` job

## Verification Steps

### Option 1: Using GitLab Web Interface

1. **Push your changes to GitLab:**
   ```bash
   git add .gitlab-ci.yml Dockerfile
   git commit -m "Fix CI pipeline: Update Rust version from 1.92 to 1.83"
   git push origin main
   ```

2. **View pipeline in browser:**
   - Navigate to your GitLab project
   - Go to **CI/CD > Pipelines**
   - Look for the pipeline triggered by your latest commit
   - Verify all jobs pass:
     - ✅ `build-image` - Builds Docker images
     - ✅ `test-image-app` - Tests application Docker image
     - ✅ `test-image-builder` - Tests builder Docker image
     - ✅ `ci` - Runs fmt, clippy, build, and tests

### Option 2: Using GitLab CLI (glab)

If you have `glab` installed:

```bash
# Install glab if needed
# macOS: brew install glab
# Linux: https://gitlab.com/gitlab-org/cli/-/releases

# Authenticate with GitLab
glab auth login

# View pipeline status
glab ci status

# View detailed pipeline information
glab ci view

# Watch pipeline in real-time
glab ci view --web
```

### Option 3: Using GitLab API

```bash
# Set your GitLab personal access token
export GITLAB_TOKEN="your-token-here"

# Get your project ID (replace with your actual project path)
export PROJECT_ID="username/test-plan-doc-gen"

# Check pipeline status
curl --header "PRIVATE-TOKEN: $GITLAB_TOKEN" \
     "https://gitlab.com/api/v4/projects/${PROJECT_ID//\//%2F}/pipelines?ref=main" \
     | jq '.[0] | {id: .id, status: .status, ref: .ref, sha: .sha}'

# Get detailed job information for latest pipeline
PIPELINE_ID=$(curl --header "PRIVATE-TOKEN: $GITLAB_TOKEN" \
     "https://gitlab.com/api/v4/projects/${PROJECT_ID//\//%2F}/pipelines?ref=main" \
     | jq '.[0].id')

curl --header "PRIVATE-TOKEN: $GITLAB_TOKEN" \
     "https://gitlab.com/api/v4/projects/${PROJECT_ID//\//%2F}/pipelines/$PIPELINE_ID/jobs" \
     | jq '.[] | {name: .name, status: .status, stage: .stage}'
```

### Option 4: Using Git Hooks

Create a post-push hook to automatically open the pipeline URL:

```bash
# Create .git/hooks/post-push
cat > .git/hooks/post-push << 'EOF'
#!/bin/bash
REMOTE_URL=$(git remote get-url origin)
PROJECT_URL=$(echo $REMOTE_URL | sed 's/\.git$//' | sed 's/git@\(.*\):/https:\/\/\1\//')
echo "Pipeline URL: $PROJECT_URL/-/pipelines"
EOF

chmod +x .git/hooks/post-push
```

## Expected Results

After the Rust version fix, all pipeline jobs should succeed:

### ✅ build-image job
- Builds Docker images with `rust:1.83-bookworm`
- Creates both `builder` and `app` images
- Pushes images to container registry
- Runs basic smoke test on images

### ✅ test-image-app job
- Runs the app image
- Verifies `test-plan-doc-gen --help` works

### ✅ test-image-builder job
- Verifies the builder image environment

### ✅ ci job
- **cargo fmt --check**: Verifies code formatting ✅
- **cargo clippy**: Runs linter with warnings as errors ✅
- **cargo build --release**: Builds in release mode ✅
- **cargo test --release --verbose**: Runs all tests ✅

## Troubleshooting

### If pipeline still fails:

1. **Check Docker Hub availability:**
   - Verify `rust:1.83-bookworm` exists: https://hub.docker.com/_/rust/tags?name=1.83-bookworm
   
2. **Check runner compatibility:**
   - Ensure GitLab runners have Docker access
   - Verify runners support multi-stage Docker builds

3. **Review job logs:**
   - Click on failed job in GitLab UI
   - Check for specific error messages
   - Look for network/registry connection issues

4. **Local Docker build test:**
   ```bash
   docker build -t test:local .
   docker run test:local test-plan-doc-gen --help
   ```

## Summary

The pipeline fixes ensure:
- ✅ Consistent Rust toolchain (1.83) across all CI jobs
- ✅ Docker images build successfully
- ✅ All tests pass
- ✅ Linting passes
- ✅ Reproducible CI results
