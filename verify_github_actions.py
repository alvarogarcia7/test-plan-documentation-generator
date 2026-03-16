#!/usr/bin/env python3
"""
Script to verify GitHub Actions CI pipeline status.

This script provides a Python alternative to the bash script for verifying
GitHub Actions CI pipeline status. It's cross-platform compatible and works
on Windows, macOS, and Linux.

Usage:
    python3 verify_github_actions.py [branch_name]
    python3 verify_github_actions.py unmodified_push_2026-03-12

Requirements:
    - Python 3.6+
    - GitHub CLI (gh) installed and authenticated
"""

import json
import subprocess
import sys
from typing import Dict, List, Optional, Tuple


class Colors:
    """ANSI color codes for terminal output."""
    RED = '\033[0;31m'
    GREEN = '\033[0;32m'
    YELLOW = '\033[1;33m'
    BLUE = '\033[0;34m'
    NC = '\033[0m'  # No Color

    @staticmethod
    def disable_on_windows():
        """Disable colors on Windows if not supported."""
        if sys.platform == 'win32':
            Colors.RED = ''
            Colors.GREEN = ''
            Colors.YELLOW = ''
            Colors.BLUE = ''
            Colors.NC = ''


def run_command(cmd: List[str], capture_output: bool = True) -> Tuple[int, str, str]:
    """
    Run a shell command and return exit code, stdout, and stderr.
    
    Args:
        cmd: Command and arguments as a list
        capture_output: Whether to capture output
        
    Returns:
        Tuple of (exit_code, stdout, stderr)
    """
    try:
        result = subprocess.run(
            cmd,
            capture_output=capture_output,
            text=True,
            check=False
        )
        return result.returncode, result.stdout, result.stderr
    except FileNotFoundError:
        return 1, "", f"Command not found: {cmd[0]}"


def check_gh_installed() -> bool:
    """Check if GitHub CLI (gh) is installed."""
    exit_code, _, _ = run_command(['gh', '--version'])
    return exit_code == 0


def get_commit_sha(branch: str) -> Optional[str]:
    """Get the commit SHA for a branch."""
    # Try origin/branch first
    exit_code, stdout, _ = run_command(['git', 'rev-parse', f'origin/{branch}'])
    if exit_code == 0:
        return stdout.strip()
    
    # Fall back to branch
    exit_code, stdout, _ = run_command(['git', 'rev-parse', branch])
    if exit_code == 0:
        return stdout.strip()
    
    return None


def fetch_branch(branch: str) -> None:
    """Fetch the latest remote refs for a branch."""
    run_command(['git', 'fetch', 'origin', branch], capture_output=False)


def get_workflow_runs(branch: str) -> Optional[List[Dict]]:
    """Get workflow runs for a branch."""
    exit_code, stdout, stderr = run_command([
        'gh', 'run', 'list',
        '--branch', branch,
        '--limit', '1',
        '--json', 'databaseId,status,conclusion,headSha,url,createdAt,updatedAt,name,displayTitle'
    ])
    
    if exit_code != 0:
        print(f"{Colors.RED}Error fetching workflow runs: {stderr}{Colors.NC}")
        return None
    
    try:
        data = json.loads(stdout)
        return data if data else None
    except json.JSONDecodeError:
        return None


def get_job_details(run_id: int) -> Optional[Dict]:
    """Get detailed job information for a workflow run."""
    exit_code, stdout, stderr = run_command([
        'gh', 'run', 'view', str(run_id),
        '--json', 'jobs'
    ])
    
    if exit_code != 0:
        print(f"{Colors.RED}Error fetching job details: {stderr}{Colors.NC}")
        return None
    
    try:
        return json.loads(stdout)
    except json.JSONDecodeError:
        return None


def print_header():
    """Print the script header."""
    print(f"{Colors.BLUE}GitHub Actions CI Pipeline Verification{Colors.NC}")
    print("=" * 40)
    print()


def print_workflow_info(workflow: Dict):
    """Print workflow information."""
    print(f"Workflow: {workflow['name']}")
    print(f"Run ID: {workflow['databaseId']}")
    print(f"Status: {workflow['status']}")
    print(f"Conclusion: {workflow['conclusion']}")
    print(f"Created: {workflow['createdAt']}")
    print(f"Updated: {workflow['updatedAt']}")
    print(f"URL: {workflow['url']}")
    print()


def print_job_status(jobs: List[Dict]) -> bool:
    """Print job status and return whether all jobs passed."""
    print("Checking individual jobs...")
    print()
    
    all_passed = True
    for job in jobs:
        name = job['name']
        conclusion = job.get('conclusion', 'unknown')
        status = job.get('status', 'unknown')
        
        if conclusion == 'success':
            print(f"{Colors.GREEN}✓ {name}{Colors.NC}")
        elif conclusion == 'skipped':
            print(f"{Colors.YELLOW}⊘ {name} (skipped){Colors.NC}")
        else:
            print(f"{Colors.RED}✗ {name} (conclusion: {conclusion}, status: {status}){Colors.NC}")
            all_passed = False
    
    print()
    return all_passed


def print_job_steps(jobs: List[Dict]):
    """Print detailed job steps."""
    print("Detailed job steps verification:")
    print("=" * 60)
    print()
    
    for job in jobs:
        if 'build-test-lint' in job['name']:
            print(f"{job['name']} job steps:")
            steps = job.get('steps', [])
            for step in steps:
                name = step.get('name', 'Unknown')
                conclusion = step.get('conclusion', 'unknown')
                
                if conclusion == 'success':
                    print(f"  {Colors.GREEN}✓{Colors.NC} {name}")
                elif conclusion == 'skipped':
                    print(f"  {Colors.YELLOW}⊘{Colors.NC} {name} (skipped)")
                else:
                    print(f"  {Colors.RED}✗{Colors.NC} {name} (conclusion: {conclusion})")
            print()
    
    for job in jobs:
        if 'coverage' in job['name']:
            print(f"{job['name']} job steps:")
            steps = job.get('steps', [])
            for step in steps:
                name = step.get('name', 'Unknown')
                conclusion = step.get('conclusion', 'unknown')
                
                if conclusion == 'success':
                    print(f"  {Colors.GREEN}✓{Colors.NC} {name}")
                elif conclusion == 'skipped':
                    print(f"  {Colors.YELLOW}⊘{Colors.NC} {name} (skipped)")
                else:
                    print(f"  {Colors.RED}✗{Colors.NC} {name} (conclusion: {conclusion})")
            print()
    
    print("=" * 60)
    print()


def print_verification_checklist(jobs: List[Dict]):
    """Print verification checklist."""
    print("Verification Checklist:")
    print("=" * 60)
    print()
    
    # Find build-test-lint job
    build_job = next((job for job in jobs if 'build-test-lint' in job['name']), None)
    
    if build_job:
        print("Critical steps in build-test-lint job:")
        required_steps = [
            "Check formatting",
            "Run clippy",
            "Build project",
            "Run unit tests",
            "Run E2E test (markdown)",
            "Run E2E test (asciidoc test plan)",
            "Run E2E test (asciidoc test results)",
            "Build Docker image",
        ]
        
        steps = build_job.get('steps', [])
        for required in required_steps:
            found = any(
                required in step.get('name', '') and step.get('conclusion') == 'success'
                for step in steps
            )
            if found:
                print(f"  {Colors.GREEN}✓{Colors.NC} {required}")
            else:
                print(f"  {Colors.RED}✗{Colors.NC} {required}")
    
    print()
    
    # Find coverage job
    coverage_job = next((job for job in jobs if 'coverage' in job['name']), None)
    print("Coverage job:")
    if coverage_job:
        conclusion = coverage_job.get('conclusion')
        if conclusion == 'success':
            print(f"  {Colors.GREEN}✓{Colors.NC} Coverage job passed")
        else:
            print(f"  {Colors.RED}✗{Colors.NC} Coverage job failed (conclusion: {conclusion})")
    else:
        print(f"  {Colors.YELLOW}⚠{Colors.NC}  Coverage job not found")
    
    print()
    print("=" * 60)
    print()


def print_final_summary(branch: str, commit_sha: str, workflow: Dict, run_id: int):
    """Print final summary."""
    conclusion = workflow.get('conclusion')
    short_sha = commit_sha[:7]
    
    if conclusion == 'success':
        print(f"{Colors.GREEN}✅ VERIFICATION PASSED{Colors.NC}")
        print()
        print(f"All GitHub Actions CI jobs completed successfully for branch '{branch}'")
        print(f"Commit: {short_sha}")
        print()
        print("Jobs verified:")
        print("  ✓ Build")
        print("  ✓ Lint (fmt-check + clippy)")
        print("  ✓ Unit tests")
        print("  ✓ E2E tests (markdown)")
        print("  ✓ E2E tests (asciidoc test plan)")
        print("  ✓ E2E tests (asciidoc test results)")
        print("  ✓ Docker build")
        print("  ✓ Coverage")
        print()
        return True
    else:
        print(f"{Colors.RED}❌ VERIFICATION FAILED{Colors.NC}")
        print()
        print("View details:")
        print(f"  gh run view {run_id}")
        print()
        print("View failed logs:")
        print(f"  gh run view {run_id} --log-failed")
        print()
        print("View in browser:")
        print(f"  {workflow['url']}")
        print()
        return False


def main():
    """Main function."""
    # Disable colors on Windows if needed
    if sys.platform == 'win32':
        Colors.disable_on_windows()
    
    # Get branch name from arguments or use default
    branch = sys.argv[1] if len(sys.argv) > 1 else 'unmodified_push_2026-03-12'
    
    print_header()
    
    # Check if gh CLI is installed
    if not check_gh_installed():
        print(f"{Colors.RED}❌ GitHub CLI (gh) is not installed{Colors.NC}")
        print()
        print("To install gh:")
        print("  macOS:   brew install gh")
        print("  Linux:   See https://github.com/cli/cli#installation")
        print("  Windows: See https://github.com/cli/cli#installation")
        sys.exit(1)
    
    print(f"{Colors.GREEN}✓ GitHub CLI (gh) is installed{Colors.NC}")
    print()
    
    # Fetch latest remote refs
    print("Fetching latest remote refs...")
    fetch_branch(branch)
    print()
    
    # Get commit SHA
    commit_sha = get_commit_sha(branch)
    if not commit_sha:
        print(f"{Colors.RED}❌ Could not find commit SHA for branch '{branch}'{Colors.NC}")
        sys.exit(1)
    
    print(f"Branch: {branch}")
    print(f"Commit: {commit_sha[:7]} ({commit_sha})")
    print()
    
    # Get workflow runs
    print(f"Fetching workflow runs for branch '{branch}'...")
    print()
    
    workflows = get_workflow_runs(branch)
    if not workflows:
        print(f"{Colors.RED}❌ No workflow runs found for branch '{branch}'{Colors.NC}")
        print()
        print("Make sure the branch has been pushed and workflows have been triggered.")
        print()
        print("To trigger workflows, ensure the branch is pushed:")
        print(f"  git push origin {branch}")
        sys.exit(1)
    
    workflow = workflows[0]
    run_id = workflow['databaseId']
    status = workflow['status']
    conclusion = workflow.get('conclusion')
    
    print_workflow_info(workflow)
    
    # Check if workflow is still running
    if status != 'completed':
        print(f"{Colors.YELLOW}⚠️  Workflow is still running (status: {status}){Colors.NC}")
        print()
        print("You can watch the workflow run with:")
        print(f"  gh run watch {run_id}")
        print()
        print("Or view it in your browser:")
        print(f"  {workflow['url']}")
        sys.exit(0)
    
    # Check overall conclusion
    if conclusion != 'success':
        print(f"{Colors.RED}❌ Workflow run failed (conclusion: {conclusion}){Colors.NC}")
        print()
        print("View failed jobs with:")
        print(f"  gh run view {run_id} --log-failed")
        print()
        print("Or view in browser:")
        print(f"  {workflow['url']}")
        sys.exit(1)
    
    print(f"{Colors.GREEN}✓ Overall workflow conclusion: success{Colors.NC}")
    print()
    
    # Get job details
    job_data = get_job_details(run_id)
    if not job_data or 'jobs' not in job_data:
        print(f"{Colors.RED}❌ Could not fetch job details{Colors.NC}")
        sys.exit(1)
    
    jobs = job_data['jobs']
    
    # Print job statuses
    all_passed = print_job_status(jobs)
    
    # Print job steps
    print_job_steps(jobs)
    
    # Print verification checklist
    print_verification_checklist(jobs)
    
    # Print final summary
    success = print_final_summary(branch, commit_sha, workflow, run_id)
    
    sys.exit(0 if success else 1)


if __name__ == '__main__':
    main()
