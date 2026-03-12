#!/bin/bash
# Script to check GitLab CI pipeline status
#
# Usage: ./check-pipeline-status.sh
#
# This script helps verify the GitLab CI pipeline status after pushing changes.
# It requires a GitLab remote to be configured.

set -e

echo "Checking GitLab CI Pipeline Status"
echo "===================================="
echo ""

# Check if we have a GitLab remote configured
GITLAB_REMOTE=$(git remote -v | grep -E '(gitlab\.com|gitlab)' | head -1 | awk '{print $1}')

if [ -z "$GITLAB_REMOTE" ]; then
    echo "❌ No GitLab remote found in this repository."
    echo ""
    echo "To add a GitLab remote, run:"
    echo "  git remote add gitlab <your-gitlab-repo-url>"
    echo ""
    echo "Example:"
    echo "  git remote add gitlab git@gitlab.com:username/tpdg.git"
    exit 1
fi

echo "✓ Found GitLab remote: $GITLAB_REMOTE"
echo ""

# Get the current branch
CURRENT_BRANCH=$(git rev-parse --abbrev-ref HEAD)
echo "Current branch: $CURRENT_BRANCH"
echo ""

# Get the latest commit
COMMIT_SHA=$(git rev-parse HEAD)
SHORT_SHA=$(git rev-parse --short HEAD)
echo "Latest commit: $SHORT_SHA"
echo ""

# Get the GitLab project URL
GITLAB_URL=$(git remote get-url $GITLAB_REMOTE | sed 's/\.git$//' | sed 's/git@\(.*\):/https:\/\/\1\//')
echo "GitLab project URL: $GITLAB_URL"
echo ""

# Instructions for checking pipeline
echo "To verify the pipeline status:"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo ""
echo "1. Push your changes (if not already pushed):"
echo "   git push $GITLAB_REMOTE $CURRENT_BRANCH"
echo ""
echo "2. View pipeline in browser:"
echo "   $GITLAB_URL/-/pipelines"
echo ""
echo "3. Or use GitLab CLI (glab) if installed:"
echo "   glab ci status"
echo "   glab ci view"
echo ""
echo "4. Or use curl with GitLab API (requires GITLAB_TOKEN):"
echo "   curl --header \"PRIVATE-TOKEN: \$GITLAB_TOKEN\" \\"
echo "        \"$GITLAB_URL/-/pipelines?ref=$CURRENT_BRANCH\""
echo ""
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo ""
echo "Expected pipeline jobs:"
echo "  ✓ build-image   - Build Docker images"
echo "  ✓ test-image-app - Test application image"
echo "  ✓ test-image-builder - Test builder image"
echo "  ✓ ci - Run fmt, clippy, build, and tests"
echo ""
echo "All jobs should pass with the Rust 1.83 fixes applied."
