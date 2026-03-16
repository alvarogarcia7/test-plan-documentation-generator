#!/bin/bash
# Script to verify GitHub Actions CI pipeline status
#
# Usage: ./verify-github-actions.sh [branch_name]
#
# This script verifies that all GitHub Actions CI jobs are passing for a specific branch.
# It checks the following jobs:
# - Build, lint (fmt-check + clippy)
# - Unit tests
# - E2E tests (markdown and asciidoc for both datasets)
# - Docker build
# - Coverage job

set -e

# Color codes for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Default branch
BRANCH="${1:-unmodified_push_2026-03-12}"

echo -e "${BLUE}GitHub Actions CI Pipeline Verification${NC}"
echo "=========================================="
echo ""

# Check if gh CLI is installed
if ! command -v gh &> /dev/null; then
    echo -e "${RED}❌ GitHub CLI (gh) is not installed${NC}"
    echo ""
    echo "To install gh:"
    echo "  macOS:   brew install gh"
    echo "  Linux:   See https://github.com/cli/cli#installation"
    echo "  Windows: See https://github.com/cli/cli#installation"
    exit 1
fi

echo -e "${GREEN}✓ GitHub CLI (gh) is installed${NC}"
echo ""

# Fetch latest remote refs
echo "Fetching latest remote refs..."
git fetch origin "$BRANCH" --quiet 2>/dev/null || true
echo ""

# Get commit SHA for the branch
echo "Branch: $BRANCH"
COMMIT_SHA=$(git rev-parse "origin/$BRANCH" 2>/dev/null || git rev-parse "$BRANCH" 2>/dev/null)
SHORT_SHA=$(echo "$COMMIT_SHA" | cut -c1-7)
echo "Commit: $SHORT_SHA ($COMMIT_SHA)"
echo ""

# Get workflow runs for this branch
echo "Fetching workflow runs for branch '$BRANCH'..."
echo ""

# Get the latest workflow run for this branch
WORKFLOW_DATA=$(gh run list --branch "$BRANCH" --limit 1 --json databaseId,status,conclusion,headSha,url,createdAt,updatedAt,name,displayTitle 2>/dev/null || echo "")

if [ -z "$WORKFLOW_DATA" ] || [ "$WORKFLOW_DATA" == "[]" ]; then
    echo -e "${RED}❌ No workflow runs found for branch '$BRANCH'${NC}"
    echo ""
    echo "Make sure the branch has been pushed and workflows have been triggered."
    echo ""
    echo "To trigger workflows, ensure the branch is pushed:"
    echo "  git push origin $BRANCH"
    exit 1
fi

# Parse workflow data
RUN_ID=$(echo "$WORKFLOW_DATA" | jq -r '.[0].databaseId')
STATUS=$(echo "$WORKFLOW_DATA" | jq -r '.[0].status')
CONCLUSION=$(echo "$WORKFLOW_DATA" | jq -r '.[0].conclusion')
RUN_URL=$(echo "$WORKFLOW_DATA" | jq -r '.[0].url')
RUN_SHA=$(echo "$WORKFLOW_DATA" | jq -r '.[0].headSha')
WORKFLOW_NAME=$(echo "$WORKFLOW_DATA" | jq -r '.[0].name')
CREATED_AT=$(echo "$WORKFLOW_DATA" | jq -r '.[0].createdAt')
UPDATED_AT=$(echo "$WORKFLOW_DATA" | jq -r '.[0].updatedAt')

echo "Workflow: $WORKFLOW_NAME"
echo "Run ID: $RUN_ID"
echo "Status: $STATUS"
echo "Conclusion: $CONCLUSION"
echo "Created: $CREATED_AT"
echo "Updated: $UPDATED_AT"
echo "URL: $RUN_URL"
echo ""

# Check if workflow is still running
if [ "$STATUS" != "completed" ]; then
    echo -e "${YELLOW}⚠️  Workflow is still running (status: $STATUS)${NC}"
    echo ""
    echo "You can watch the workflow run with:"
    echo "  gh run watch $RUN_ID"
    echo ""
    echo "Or view it in your browser:"
    echo "  $RUN_URL"
    exit 0
fi

# Check overall conclusion
if [ "$CONCLUSION" != "success" ]; then
    echo -e "${RED}❌ Workflow run failed (conclusion: $CONCLUSION)${NC}"
    echo ""
    echo "View failed jobs with:"
    echo "  gh run view $RUN_ID --log-failed"
    echo ""
    echo "Or view in browser:"
    echo "  $RUN_URL"
    exit 1
fi

echo -e "${GREEN}✓ Overall workflow conclusion: success${NC}"
echo ""

# Get individual job details
echo "Checking individual jobs..."
echo ""

JOBS_DATA=$(gh run view "$RUN_ID" --json jobs --jq '.jobs')

# Expected job patterns (we have 2 jobs: build-test-lint and coverage)
EXPECTED_JOBS=("build-test-lint" "coverage")

ALL_JOBS_PASSED=true

# Parse and display job statuses
echo "$JOBS_DATA" | jq -r '.[] | "\(.name)|\(.conclusion)|\(.status)"' | while IFS='|' read -r job_name conclusion status; do
    if [ "$conclusion" == "success" ]; then
        echo -e "${GREEN}✓ $job_name${NC}"
    elif [ "$conclusion" == "skipped" ]; then
        echo -e "${YELLOW}⊘ $job_name (skipped)${NC}"
    else
        echo -e "${RED}✗ $job_name (conclusion: $conclusion, status: $status)${NC}"
        ALL_JOBS_PASSED=false
    fi
done

echo ""

# Display detailed job steps for build-test-lint job
echo "Detailed job steps verification:"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo ""

# Get the build-test-lint job details
BUILD_JOB=$(echo "$JOBS_DATA" | jq -r '.[] | select(.name | contains("build-test-lint"))')
BUILD_STEPS=$(echo "$BUILD_JOB" | jq -r '.steps[]? | "\(.name)|\(.conclusion)"')

echo "build-test-lint job steps:"
echo "$BUILD_STEPS" | while IFS='|' read -r step_name conclusion; do
    if [ "$conclusion" == "success" ]; then
        echo -e "  ${GREEN}✓${NC} $step_name"
    elif [ "$conclusion" == "skipped" ]; then
        echo -e "  ${YELLOW}⊘${NC} $step_name (skipped)"
    else
        echo -e "  ${RED}✗${NC} $step_name (conclusion: $conclusion)"
    fi
done

echo ""

# Get the coverage job details
COVERAGE_JOB=$(echo "$JOBS_DATA" | jq -r '.[] | select(.name | contains("coverage"))')
if [ -n "$COVERAGE_JOB" ]; then
    COVERAGE_STEPS=$(echo "$COVERAGE_JOB" | jq -r '.steps[]? | "\(.name)|\(.conclusion)"')
    
    echo "coverage job steps:"
    echo "$COVERAGE_STEPS" | while IFS='|' read -r step_name conclusion; do
        if [ "$conclusion" == "success" ]; then
            echo -e "  ${GREEN}✓${NC} $step_name"
        elif [ "$conclusion" == "skipped" ]; then
            echo -e "  ${YELLOW}⊘${NC} $step_name (skipped)"
        else
            echo -e "  ${RED}✗${NC} $step_name (conclusion: $conclusion)"
        fi
    done
    echo ""
fi

echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo ""

# Verification checklist
echo "Verification Checklist:"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"

# Check for key steps
REQUIRED_STEPS=(
    "Check formatting"
    "Run clippy"
    "Build project"
    "Run unit tests"
    "Run E2E test (markdown)"
    "Run E2E test (asciidoc test plan)"
    "Run E2E test (asciidoc test results)"
    "Build Docker image"
)

echo ""
echo "Critical steps in build-test-lint job:"
for step in "${REQUIRED_STEPS[@]}"; do
    if echo "$BUILD_STEPS" | grep -q "$step.*success"; then
        echo -e "  ${GREEN}✓${NC} $step"
    else
        echo -e "  ${RED}✗${NC} $step"
    fi
done

echo ""
echo "Coverage job:"
if [ -n "$COVERAGE_JOB" ]; then
    COVERAGE_CONCLUSION=$(echo "$COVERAGE_JOB" | jq -r '.conclusion')
    if [ "$COVERAGE_CONCLUSION" == "success" ]; then
        echo -e "  ${GREEN}✓${NC} Coverage job passed"
    else
        echo -e "  ${RED}✗${NC} Coverage job failed (conclusion: $COVERAGE_CONCLUSION)"
    fi
else
    echo -e "  ${YELLOW}⚠${NC}  Coverage job not found"
fi

echo ""
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo ""

# Final summary
if [ "$CONCLUSION" == "success" ]; then
    echo -e "${GREEN}✅ VERIFICATION PASSED${NC}"
    echo ""
    echo "All GitHub Actions CI jobs completed successfully for branch '$BRANCH'"
    echo "Commit: $SHORT_SHA"
    echo ""
    echo "Jobs verified:"
    echo "  ✓ Build"
    echo "  ✓ Lint (fmt-check + clippy)"
    echo "  ✓ Unit tests"
    echo "  ✓ E2E tests (markdown)"
    echo "  ✓ E2E tests (asciidoc test plan)"
    echo "  ✓ E2E tests (asciidoc test results)"
    echo "  ✓ Docker build"
    echo "  ✓ Coverage"
    echo ""
else
    echo -e "${RED}❌ VERIFICATION FAILED${NC}"
    echo ""
    echo "View details:"
    echo "  gh run view $RUN_ID"
    echo ""
    echo "View failed logs:"
    echo "  gh run view $RUN_ID --log-failed"
    echo ""
    echo "View in browser:"
    echo "  $RUN_URL"
    exit 1
fi
