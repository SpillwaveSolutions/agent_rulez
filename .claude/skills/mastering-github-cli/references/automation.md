# Automation Patterns Reference

Comprehensive reference for scripting, batch operations, error handling, and agentic workflows with GitHub CLI.

## Contents

- [Workflow Triggering](#workflow-triggering)
- [JSON Output Patterns](#json-output-patterns)
- [Batch Operations](#batch-operations)
- [Error Handling](#error-handling)
- [Retry Patterns](#retry-patterns)
- [Rate Limit Handling](#rate-limit-handling)
- [Environment Setup](#environment-setup)
- [Complete Workflow Examples](#complete-workflow-examples)

---

## Workflow Triggering

### Basic triggering

```bash
gh workflow run <workflow> [flags]
```

| Flag | Description | Example |
|------|-------------|---------|
| `-f` | Input field | `-f environment=staging` |
| `-F` | Input from file | `-F config=@config.json` |
| `--json` | Read inputs from stdin | `--json` |
| `--ref` | Branch/tag/SHA | `--ref feature-branch` |

### Examples

```bash
# Simple trigger
gh workflow run deploy.yml

# With inputs
gh workflow run deploy.yml -f environment=staging -f debug=true

# From specific branch
gh workflow run build.yml --ref develop

# JSON inputs from stdin
echo '{"environment":"prod","version":"1.2.3"}' | gh workflow run deploy.yml --json

# JSON inputs from file
gh workflow run deploy.yml --json < inputs.json
```

### Workflow management

```bash
# List workflows
gh workflow list
gh workflow list --json id,name,state

# View workflow
gh workflow view deploy.yml
gh workflow view deploy.yml --yaml  # Show YAML definition

# Enable/disable
gh workflow enable deploy.yml
gh workflow disable legacy.yml
```

### Run management

```bash
# Rerun failed jobs only
gh run rerun 12345 --failed

# Rerun with debug logging
gh run rerun 12345 --debug

# Rerun specific jobs
gh run rerun 12345 --job job_id

# Cancel run
gh run cancel 12345
```

---

## JSON Output Patterns

### Discovering available fields

```bash
# List all fields (no value = show available)
gh pr list --json
gh run list --json
gh issue list --json
gh search repos --json
```

### Selecting fields

```bash
# Single field
gh pr list --json number

# Multiple fields
gh pr list --json number,title,author,labels

# Nested fields
gh pr list --json author --jq '.[].author.login'
```

### jq filtering patterns

```bash
# Select by condition
gh run list --json status,conclusion \
  --jq '.[] | select(.status == "completed")'

# Filter and extract
gh pr list --json number,labels \
  --jq '.[] | select(.labels | any(.name == "bug")) | .number'

# Count
gh issue list --json number --jq 'length'

# Unique values
gh search code "path:.skilz" --json repository \
  --jq '[.[].repository.fullName] | unique'

# Group by
gh pr list --json state --jq 'group_by(.state) | map({state: .[0].state, count: length})'

# Sort
gh search repos "cli" --json stargazersCount,fullName \
  --jq 'sort_by(-.stargazersCount) | .[:10]'

# Map/transform
gh run list --json databaseId,conclusion \
  --jq 'map({id: .databaseId, result: .conclusion})'
```

### Template formatting

```bash
# Go template
gh pr list --json number,title,updatedAt --template \
  '{{range .}}{{tablerow .number .title (timeago .updatedAt)}}{{end}}'

# Table output
gh pr list --json number,title,author --template \
  '{{tablerow "PR" "Title" "Author"}}{{range .}}{{tablerow .number .title .author.login}}{{end}}'
```

### Exporting data

```bash
# To JSON file
gh search repos "topic:cli" --json fullName,stars --limit 100 > repos.json

# To CSV (via jq)
gh pr list --json number,title,author \
  --jq '.[] | [.number, .title, .author.login] | @csv' > prs.csv

# To TSV
gh issue list --json number,title \
  --jq '.[] | [.number, .title] | @tsv' > issues.tsv
```

---

## Batch Operations

### Iterating over results

```bash
# Process PRs
gh pr list --json number --jq '.[].number' | while read pr; do
    echo "Processing PR #$pr"
    gh pr view "$pr" --json title --jq '.title'
done

# Process repos from search
gh search code "path:.skilz" --json repository --jq '.[].repository.fullName' | \
  sort -u | while read repo; do
    echo "Found: $repo"
done

# Process with xargs (parallel)
gh pr list --label "auto-merge" --json number --jq '.[].number' | \
  xargs -I {} -P 4 gh pr merge {} --squash
```

### Batch PR operations

```bash
# Merge all approved PRs
for pr in $(gh pr list --json number,reviewDecision \
  --jq '.[] | select(.reviewDecision == "APPROVED") | .number'); do
    gh pr merge "$pr" --squash --delete-branch
done

# Add label to multiple PRs
for pr in $(gh pr list --search "is:open author:dependabot" --json number --jq '.[].number'); do
    gh pr edit "$pr" --add-label "dependencies"
done

# Close stale PRs
for pr in $(gh pr list --json number,updatedAt \
  --jq --arg cutoff "$(date -d '30 days ago' -Iseconds)" \
  '.[] | select(.updatedAt < $cutoff) | .number'); do
    gh pr close "$pr" --comment "Closing stale PR"
done
```

### Batch issue operations

```bash
# Assign all unassigned bugs
gh issue list --label bug --json number,assignees \
  --jq '.[] | select(.assignees | length == 0) | .number' | \
while read num; do
    gh issue edit "$num" --add-assignee @me
done

# Add to project
for issue in $(gh issue list --label "priority:high" --json number --jq '.[].number'); do
    gh project item-add 1 --url "https://github.com/owner/repo/issues/$issue"
done
```

### Multi-repo operations

```bash
# Clone matching repos
gh search repos --owner myorg --topic python --json fullName --jq '.[].fullName' | \
while read repo; do
    gh repo clone "$repo" "clones/$(basename $repo)" || true
done

# Create issue across repos
REPOS=("org/repo1" "org/repo2" "org/repo3")
for repo in "${REPOS[@]}"; do
    gh issue create --repo "$repo" \
      --title "Update dependencies" \
      --label "maintenance" \
      --body "Please update dependencies"
done

# Check CI status across repos
gh search repos --owner myorg --json fullName --jq '.[].fullName' | \
while read repo; do
    latest=$(gh run list --repo "$repo" --limit 1 --json conclusion --jq '.[0].conclusion' 2>/dev/null)
    echo "$repo: ${latest:-no runs}"
done
```

---

## Error Handling

### Exit code checking

```bash
#!/bin/bash
set -euo pipefail

# Check command success
if gh pr merge 123 --squash; then
    echo "PR merged successfully"
else
    echo "Failed to merge PR"
    exit 1
fi

# Capture exit code
gh run watch 12345 --exit-status
EXIT_CODE=$?

case $EXIT_CODE in
    0) echo "Success" ;;
    1) echo "Failed" ;;
    2) echo "Cancelled" ;;
    *) echo "Unknown: $EXIT_CODE" ;;
esac
```

### Error output handling

```bash
# Capture stderr
if ! OUTPUT=$(gh pr create --fill 2>&1); then
    echo "Error: $OUTPUT"
    exit 1
fi

# Suppress errors, continue
gh repo clone owner/repo 2>/dev/null || true

# Log errors to file
gh run view 12345 2>> errors.log
```

### Validation before operations

```bash
# Check if PR exists
if ! gh pr view 123 &>/dev/null; then
    echo "PR #123 not found"
    exit 1
fi

# Check if repo accessible
if ! gh repo view owner/repo &>/dev/null; then
    echo "Cannot access repo"
    exit 1
fi

# Check auth status
if ! gh auth status &>/dev/null; then
    echo "Not authenticated"
    exit 1
fi
```

### Safe operations pattern

```bash
#!/bin/bash
set -euo pipefail

safe_merge() {
    local pr=$1
    
    # Validate PR exists and is open
    state=$(gh pr view "$pr" --json state --jq '.state')
    if [ "$state" != "OPEN" ]; then
        echo "PR #$pr is not open (state: $state)"
        return 1
    fi
    
    # Check CI passed
    if ! gh pr checks "$pr" --watch --fail-fast; then
        echo "PR #$pr checks failed"
        return 1
    fi
    
    # Merge
    gh pr merge "$pr" --squash --delete-branch
}

safe_merge 123
```

---

## Retry Patterns

### Simple retry

```bash
retry() {
    local max_attempts=$1
    shift
    local attempt=1
    
    until "$@"; do
        if [ $attempt -ge $max_attempts ]; then
            echo "Failed after $attempt attempts"
            return 1
        fi
        echo "Attempt $attempt failed, retrying..."
        ((attempt++))
        sleep 2
    done
}

retry 3 gh api repos/owner/repo
```

### Exponential backoff

```bash
retry_backoff() {
    local max_attempts=${1:-5}
    shift
    local attempt=0
    local delay=1
    
    until "$@"; do
        ((attempt++))
        if [ $attempt -ge $max_attempts ]; then
            echo "Failed after $attempt attempts"
            return 1
        fi
        echo "Attempt $attempt failed, waiting ${delay}s..."
        sleep $delay
        delay=$((delay * 2))
    done
}

retry_backoff 5 gh workflow run deploy.yml
```

### Retry with jitter

```bash
retry_jitter() {
    local max_attempts=${1:-5}
    shift
    local attempt=0
    local base_delay=1
    
    until "$@"; do
        ((attempt++))
        if [ $attempt -ge $max_attempts ]; then
            return 1
        fi
        # Add random jitter (0-1000ms)
        local jitter=$(( RANDOM % 1000 ))
        local delay=$(( base_delay * attempt + jitter / 1000 ))
        sleep "$delay"
    done
}
```

### Retry specific errors

```bash
retry_on_rate_limit() {
    local max_attempts=5
    local attempt=0
    
    while [ $attempt -lt $max_attempts ]; do
        if output=$("$@" 2>&1); then
            echo "$output"
            return 0
        fi
        
        if echo "$output" | grep -q "rate limit"; then
            ((attempt++))
            echo "Rate limited, waiting 60s (attempt $attempt/$max_attempts)"
            sleep 60
        else
            echo "$output" >&2
            return 1
        fi
    done
    return 1
}

retry_on_rate_limit gh api search/code -f q='filename:SKILL.md'
```

---

## Rate Limit Handling

### Checking limits

```bash
# Core API limits
gh api rate_limit --jq '.resources.core'

# Search API limits
gh api rate_limit --jq '.resources.search'

# Code search limits
gh api rate_limit --jq '.resources.code_search'

# All limits
gh api rate_limit --jq '.resources | to_entries | .[] | "\(.key): \(.value.remaining)/\(.value.limit)"'
```

### Respecting limits in scripts

```bash
#!/bin/bash

check_rate_limit() {
    local resource=${1:-core}
    local remaining=$(gh api rate_limit --jq ".resources.$resource.remaining")
    local reset=$(gh api rate_limit --jq ".resources.$resource.reset")
    
    if [ "$remaining" -lt 10 ]; then
        local wait_time=$((reset - $(date +%s)))
        if [ $wait_time -gt 0 ]; then
            echo "Rate limit low ($remaining remaining), waiting ${wait_time}s"
            sleep $wait_time
        fi
    fi
}

# Use before API-heavy operations
check_rate_limit search
gh search code "path:.skilz" --limit 100
```

### Caching for rate limits

```bash
# Cache API responses
gh api --cache 3600s repos/owner/repo

# Cache for 1 hour
gh api --cache 1h repos/owner/repo/releases

# Check if cached
gh api --cache 3600s repos/owner/repo 2>&1 | grep -q "cached"
```

### Throttling batch operations

```bash
# Add delay between operations
for repo in "${repos[@]}"; do
    gh api "repos/$repo" --jq '.stargazers_count'
    sleep 0.5  # 2 requests per second max
done

# Respect search limit (10/min for code)
for query in "${queries[@]}"; do
    gh search code "$query" --limit 100
    sleep 6  # Stay under 10/minute
done
```

---

## Environment Setup

### Environment variables

| Variable | Purpose | Example |
|----------|---------|---------|
| `GH_TOKEN` | Authentication | `export GH_TOKEN=ghp_xxx` |
| `GH_REPO` | Default repository | `export GH_REPO=owner/repo` |
| `GH_HOST` | GitHub Enterprise | `export GH_HOST=github.mycompany.com` |
| `GH_PROMPT_DISABLED` | Disable prompts | `export GH_PROMPT_DISABLED=1` |
| `GH_DEBUG` | Debug output | `export GH_DEBUG=1` |
| `GH_PAGER` | Pager command | `export GH_PAGER=less` |
| `NO_COLOR` | Disable colors | `export NO_COLOR=1` |

### GitHub Actions setup

```yaml
name: Automation
on: workflow_dispatch

env:
  GH_TOKEN: ${{ secrets.GITHUB_TOKEN }}

jobs:
  automate:
    runs-on: ubuntu-latest
    permissions:
      contents: read
      pull-requests: write
      issues: write
    steps:
      - uses: actions/checkout@v4
      
      - name: Run automation
        run: |
          gh pr list --json number,title
          gh issue create --title "Auto" --body "Created by CI"
```

### Using PAT for cross-repo access

```yaml
env:
  GH_TOKEN: ${{ secrets.PAT_TOKEN }}  # Personal Access Token

steps:
  - name: Access other repos
    run: |
      gh repo clone other-org/other-repo
      gh pr create --repo other-org/other-repo --fill
```

### Script header template

```bash
#!/bin/bash
set -euo pipefail

# Verify authentication
if ! gh auth status &>/dev/null; then
    echo "Error: Not authenticated with GitHub CLI"
    echo "Run: gh auth login"
    exit 1
fi

# Set defaults
: "${GH_REPO:=owner/repo}"
export GH_PROMPT_DISABLED=1

# Your automation here
```

---

## Complete Workflow Examples

### Search, analyze, and report

```bash
#!/bin/bash
set -euo pipefail

# Find all repos with .skilz directory
echo "Searching for repos with .skilz..."
repos=$(gh search code "path:.skilz" --json repository \
  --jq '[.[].repository.fullName] | unique | .[]')

echo "Found $(echo "$repos" | wc -l) repos"

# Analyze each repo
for repo in $repos; do
    echo "=== $repo ==="
    
    # Get repo info
    info=$(gh api "repos/$repo" --jq '{stars: .stargazers_count, language: .language}')
    echo "  Info: $info"
    
    # Check latest CI status
    latest=$(gh run list --repo "$repo" --limit 1 --json conclusion --jq '.[0].conclusion' 2>/dev/null || echo "none")
    echo "  CI: $latest"
    
    sleep 1  # Rate limit courtesy
done
```

### Auto-merge approved PRs

```bash
#!/bin/bash
set -euo pipefail

echo "Finding approved PRs..."
approved=$(gh pr list --json number,reviewDecision,statusCheckRollup \
  --jq '[.[] | select(.reviewDecision == "APPROVED")] | .[].number')

for pr in $approved; do
    echo "Processing PR #$pr..."
    
    # Wait for checks
    if gh pr checks "$pr" --watch --fail-fast; then
        echo "  Checks passed, merging..."
        gh pr merge "$pr" --squash --delete-branch
        echo "  Merged!"
    else
        echo "  Checks failed, skipping"
    fi
done
```

### Trigger deploy and monitor

```bash
#!/bin/bash
set -euo pipefail

ENVIRONMENT=${1:-staging}
TIMEOUT=${2:-3600}

echo "Triggering deploy to $ENVIRONMENT..."
gh workflow run deploy.yml -f environment="$ENVIRONMENT"

# Wait for run to register
sleep 5

# Get run ID
RUN_ID=$(gh run list --workflow=deploy.yml --limit 1 --json databaseId --jq '.[0].databaseId')
echo "Run ID: $RUN_ID"

# Watch with timeout
echo "Watching run (timeout: ${TIMEOUT}s)..."
if timeout "$TIMEOUT" gh run watch "$RUN_ID" --exit-status; then
    echo "Deploy succeeded!"
    
    # Download artifacts
    gh run download "$RUN_ID" -D ./deploy-artifacts
    
    exit 0
else
    echo "Deploy failed!"
    
    # Show failed logs
    gh run view "$RUN_ID" --log-failed
    
    exit 1
fi
```

### Sync forks and create update PRs

```bash
#!/bin/bash
set -euo pipefail

# List of forks to sync
FORKS=(
    "myorg/forked-repo-1"
    "myorg/forked-repo-2"
)

for fork in "${FORKS[@]}"; do
    echo "=== Syncing $fork ==="
    
    # Sync with upstream
    if gh repo sync "$fork" --force; then
        echo "  Synced successfully"
    else
        echo "  Sync failed, skipping"
        continue
    fi
    
    # Clone and check for updates needed
    tmpdir=$(mktemp -d)
    gh repo clone "$fork" "$tmpdir" -- --depth 1
    
    # ... make updates ...
    
    rm -rf "$tmpdir"
done
```

### Comprehensive repo audit

```bash
#!/bin/bash
set -euo pipefail

ORG=${1:-myorg}
OUTPUT=${2:-audit.json}

echo "Auditing $ORG repositories..."

gh search repos --owner "$ORG" --limit 1000 --json fullName,visibility,isArchived \
  --jq '.[] | select(.isArchived == false)' | \
while read -r repo_json; do
    repo=$(echo "$repo_json" | jq -r '.fullName')
    
    # Get additional details
    details=$(gh api "repos/$repo" --jq '{
        has_issues: .has_issues,
        has_wiki: .has_wiki,
        default_branch: .default_branch,
        pushed_at: .pushed_at
    }')
    
    # Get branch protection
    protection=$(gh api "repos/$repo/branches/main/protection" 2>/dev/null || echo '{"enabled": false}')
    
    # Combine and output
    echo "$repo_json" | jq --argjson details "$details" --argjson protection "$protection" \
        '. + $details + {branch_protection: $protection}'
    
    sleep 0.5  # Rate limit
done | jq -s '.' > "$OUTPUT"

echo "Audit complete: $OUTPUT"
```
