# CI/CD Monitoring Reference

Comprehensive reference for monitoring GitHub Actions workflows, PR checks, and retrieving logs/artifacts.

## Contents

- [Workflow Runs](#workflow-runs)
- [PR Checks](#pr-checks)
- [Logs & Artifacts](#logs--artifacts)
- [Watching & Waiting](#watching--waiting)
- [JSON Fields Reference](#json-fields-reference)
- [Exit Codes](#exit-codes)
- [Common Recipes](#common-recipes)

---

## Workflow Runs

### Listing runs

```bash
gh run list [flags]
```

| Flag | Description | Example |
|------|-------------|---------|
| `--workflow` | Filter by workflow name/file | `--workflow=CI` |
| `--branch` | Filter by branch | `--branch=main` |
| `--status` | Filter by status | `--status=failure` |
| `--event` | Filter by trigger event | `--event=push` |
| `--user` | Filter by user who triggered | `--user=octocat` |
| `--limit` | Max results (default 20) | `--limit=50` |
| `--json` | JSON output | `--json databaseId,status` |

### Status values

| Status | Meaning |
|--------|---------|
| `queued` | Waiting to run |
| `in_progress` | Currently running |
| `completed` | Finished (check conclusion) |
| `waiting` | Waiting for approval |
| `pending` | Not yet started |
| `requested` | Workflow requested |

### Conclusion values (when completed)

| Conclusion | Meaning |
|------------|---------|
| `success` | All jobs passed |
| `failure` | One or more jobs failed |
| `cancelled` | Run was cancelled |
| `skipped` | Run was skipped |
| `timed_out` | Run exceeded time limit |
| `action_required` | Needs manual approval |
| `neutral` | Neutral result |
| `stale` | Outdated run |

### Examples

```bash
# Recent failures on main
gh run list --workflow=CI --branch=main --status=failure --limit 10

# All runs by user
gh run list --user=octocat --limit 20

# JSON for automation
gh run list --json databaseId,status,conclusion,workflowName --limit 50

# Filter completed runs
gh run list --json databaseId,conclusion \
  --jq '.[] | select(.conclusion == "failure")'
```

### Viewing run details

```bash
gh run view <run-id> [flags]
```

| Flag | Description |
|------|-------------|
| `--log` | Show full logs |
| `--log-failed` | Show only failed step logs |
| `--verbose` | Show job steps |
| `--exit-status` | Exit with run's status code |
| `--json` | JSON output |
| `--web` | Open in browser |

### Examples

```bash
# Summary view
gh run view 12345

# With job details
gh run view 12345 --verbose

# Full logs
gh run view 12345 --log

# Only failed logs (most useful for debugging)
gh run view 12345 --log-failed

# JSON output
gh run view 12345 --json jobs,status,conclusion
```

---

## PR Checks

### Checking PR status

```bash
gh pr checks [pr-number] [flags]
```

| Flag | Description |
|------|-------------|
| `--watch` | Block until checks complete |
| `--fail-fast` | Exit on first failure (with --watch) |
| `--required` | Show only required checks |
| `--json` | JSON output |
| `--interval` | Polling interval (default 10s) |

### Examples

```bash
# View current PR checks
gh pr checks

# Specific PR
gh pr checks 123

# Block until complete
gh pr checks --watch

# Exit immediately on failure
gh pr checks --watch --fail-fast

# Only required checks
gh pr checks --required

# JSON output
gh pr checks --json name,state,bucket,completedAt
```

### PR status overview

```bash
gh pr status [flags]
```

Shows overview of:
- PRs created by you
- PRs requesting your review
- PRs on current branch

```bash
# Basic status
gh pr status

# Include merge conflict info
gh pr status --conflict-status

# JSON output
gh pr status --json
```

### PR view for CI details

```bash
# Get check rollup status
gh pr view 123 --json statusCheckRollup

# Get review decision
gh pr view 123 --json reviewDecision

# Combined status info
gh pr view 123 --json statusCheckRollup,reviewDecision,mergeable
```

---

## Logs & Artifacts

### Viewing logs

```bash
# Full logs for run
gh run view 12345 --log

# Only failed step logs
gh run view 12345 --log-failed

# Specific job logs via API
gh api repos/{owner}/{repo}/actions/jobs/{job_id}/logs
```

### Downloading logs via API

```bash
# Download as zip
gh api repos/{owner}/{repo}/actions/runs/12345/logs > logs.zip

# Extract and search
unzip -p logs.zip | grep -i "error"
```

### Downloading artifacts

```bash
gh run download <run-id> [flags]
```

| Flag | Description | Example |
|------|-------------|---------|
| `-n` | Artifact name | `-n build-output` |
| `-p` | Name pattern | `-p "coverage-*"` |
| `-D` | Output directory | `-D ./artifacts` |

### Examples

```bash
# Download all artifacts
gh run download 12345

# Specific artifact
gh run download 12345 -n build-output

# Pattern matching
gh run download 12345 -p "test-results-*"

# To specific directory
gh run download 12345 -n dist -D ./release

# Most recent run's artifacts
gh run download $(gh run list --limit 1 --json databaseId --jq '.[0].databaseId')
```

### Artifact management via API

```bash
# List artifacts for run
gh api repos/{owner}/{repo}/actions/runs/12345/artifacts

# List all repo artifacts
gh api repos/{owner}/{repo}/actions/artifacts

# Download specific artifact
gh api repos/{owner}/{repo}/actions/artifacts/{artifact_id}/zip > artifact.zip

# Delete artifact
gh api -X DELETE repos/{owner}/{repo}/actions/artifacts/{artifact_id}
```

---

## Watching & Waiting

### Blocking watch

```bash
gh run watch <run-id> [flags]
```

| Flag | Description |
|------|-------------|
| `--exit-status` | Exit with run's conclusion code |
| `--interval` | Polling interval (default 3s) |

### Exit codes for watch

| Exit Code | Meaning |
|-----------|---------|
| `0` | Run succeeded |
| `1` | Run failed or error |
| `2` | Run cancelled |

### Examples

```bash
# Watch and exit with status
gh run watch 12345 --exit-status

# Custom interval
gh run watch 12345 --interval 10

# Watch most recent run
gh run watch $(gh run list --limit 1 --json databaseId --jq '.[0].databaseId') --exit-status
```

### Trigger and watch pattern

```bash
# Trigger workflow and wait for completion
gh workflow run deploy.yml -f environment=staging
sleep 5  # Allow run to register
RUN_ID=$(gh run list --workflow=deploy.yml --limit 1 --json databaseId --jq '.[0].databaseId')
gh run watch "$RUN_ID" --exit-status
```

### PR checks watch

```bash
# Block until PR checks complete
gh pr checks --watch

# Exit on first failure
gh pr checks --watch --fail-fast

# With custom interval
gh pr checks --watch --interval 30
```

### Timeout handling

For timeout control, use the script `scripts/wait-for-run.sh`:

```bash
./scripts/wait-for-run.sh 12345 3600  # 1 hour timeout
```

Or with shell timeout:

```bash
timeout 3600 gh run watch 12345 --exit-status
```

---

## JSON Fields Reference

### gh run list fields

| Field | Type | Description |
|-------|------|-------------|
| `databaseId` | number | Run ID |
| `status` | string | Current status |
| `conclusion` | string | Final result |
| `workflowName` | string | Workflow name |
| `workflowDatabaseId` | number | Workflow ID |
| `headBranch` | string | Branch name |
| `headSha` | string | Commit SHA |
| `event` | string | Trigger event |
| `displayTitle` | string | Run title |
| `createdAt` | timestamp | Start time |
| `updatedAt` | timestamp | Last update |
| `url` | string | Web URL |

### gh run view --json jobs fields

| Field | Type | Description |
|-------|------|-------------|
| `jobs` | array | Job details |
| `jobs[].name` | string | Job name |
| `jobs[].status` | string | Job status |
| `jobs[].conclusion` | string | Job result |
| `jobs[].startedAt` | timestamp | Start time |
| `jobs[].completedAt` | timestamp | End time |
| `jobs[].steps` | array | Step details |

### gh pr checks --json fields

| Field | Type | Description |
|-------|------|-------------|
| `name` | string | Check name |
| `state` | string | Check state |
| `bucket` | string | pass/fail/pending |
| `completedAt` | timestamp | Completion time |
| `detailsUrl` | string | Link to details |
| `workflowName` | string | Workflow name |

---

## Exit Codes

### gh run watch

| Code | Meaning |
|------|---------|
| `0` | success |
| `1` | failure or error |
| `2` | cancelled |

### gh pr checks

| Code | Meaning |
|------|---------|
| `0` | All checks passed |
| `1` | Error occurred |
| `8` | Checks pending/in progress |

### Using exit codes in scripts

```bash
#!/bin/bash
set -e

if gh run watch 12345 --exit-status; then
    echo "Run succeeded"
    gh run download 12345 -n artifacts
else
    echo "Run failed"
    gh run view 12345 --log-failed
    exit 1
fi
```

```bash
#!/bin/bash

gh pr checks --watch
EXIT_CODE=$?

case $EXIT_CODE in
    0) echo "All checks passed" ;;
    1) echo "Error occurred" ;;
    8) echo "Checks still pending" ;;
esac
```

---

## Common Recipes

### Wait for latest run on branch

```bash
RUN_ID=$(gh run list --branch=main --limit 1 --json databaseId --jq '.[0].databaseId')
gh run watch "$RUN_ID" --exit-status
```

### Get failed runs from last 24 hours

```bash
gh run list --status=failure --json databaseId,workflowName,createdAt --limit 100 | \
  jq --arg cutoff "$(date -u -d '24 hours ago' +%Y-%m-%dT%H:%M:%SZ)" \
  '.[] | select(.createdAt > $cutoff)'
```

### Download artifacts from successful runs only

```bash
for run_id in $(gh run list --status=completed --json databaseId,conclusion \
  --jq '.[] | select(.conclusion=="success") | .databaseId'); do
    gh run download "$run_id" -D "./artifacts/$run_id" 2>/dev/null || true
done
```

### Monitor multiple workflows

```bash
WORKFLOWS=("ci.yml" "deploy.yml" "test.yml")
for wf in "${WORKFLOWS[@]}"; do
    echo "=== $wf ==="
    gh run list --workflow="$wf" --limit 5 --json status,conclusion,createdAt \
      --jq '.[] | "\(.status) \(.conclusion // "pending") \(.createdAt)"'
done
```

### Alert on PR check failure

```bash
#!/bin/bash
PR_NUM=${1:-$(gh pr view --json number --jq '.number')}

if ! gh pr checks "$PR_NUM" --watch --fail-fast; then
    echo "PR #$PR_NUM checks failed!"
    gh pr checks "$PR_NUM" --json name,state,bucket \
      --jq '.[] | select(.bucket != "pass") | "\(.name): \(.state)"'
    exit 1
fi
echo "PR #$PR_NUM all checks passed"
```

### Get workflow run duration

```bash
gh run view 12345 --json createdAt,updatedAt \
  --jq '((.updatedAt | fromdateiso8601) - (.createdAt | fromdateiso8601)) | "\(. / 60 | floor)m \(. % 60)s"'
```

### Rerun failed and watch

```bash
gh run rerun 12345 --failed
sleep 3
NEW_RUN=$(gh run list --limit 1 --json databaseId --jq '.[0].databaseId')
gh run watch "$NEW_RUN" --exit-status
```
