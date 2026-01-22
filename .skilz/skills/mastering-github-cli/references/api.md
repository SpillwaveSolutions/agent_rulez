# API Access Reference

Direct GitHub API access via `gh api` for operations not covered by built-in commands.

## Contents

- [REST API Basics](#rest-api-basics)
- [GraphQL Patterns](#graphql-patterns)
- [Pagination](#pagination)
- [Caching](#caching)
- [Common Endpoints](#common-endpoints)
- [Advanced Patterns](#advanced-patterns)

---

## REST API Basics

### Syntax

```bash
gh api <endpoint> [flags]
```

| Flag | Description | Example |
|------|-------------|---------|
| `-X` | HTTP method | `-X POST`, `-X DELETE` |
| `-f` | String field | `-f title="Issue"` |
| `-F` | Typed field (file, bool, int) | `-F draft=true` |
| `-H` | Header | `-H "Accept: application/json"` |
| `--jq` | jq filter | `--jq '.name'` |
| `--template` | Go template | `--template '{{.name}}'` |
| `--paginate` | Auto-paginate | `--paginate` |
| `--cache` | Cache duration | `--cache 3600s` |
| `--silent` | No output | `--silent` |

### HTTP Methods

```bash
# GET (default)
gh api repos/{owner}/{repo}

# POST
gh api repos/{owner}/{repo}/issues -f title="Bug" -f body="Description"

# PATCH
gh api repos/{owner}/{repo} -X PATCH -f description="Updated"

# PUT
gh api repos/{owner}/{repo}/topics -X PUT -f names='["topic1","topic2"]'

# DELETE
gh api repos/{owner}/{repo}/issues/1/labels/bug -X DELETE
```

### Field types

```bash
# String field
-f title="My Issue"

# Integer field
-F per_page=100

# Boolean field
-F draft=true

# JSON field
-f names='["a","b","c"]'

# File contents
-F body=@file.md

# Null value
-F value=null
```

### Path parameters

```bash
# Use {owner} and {repo} as placeholders
gh api repos/{owner}/{repo}

# They resolve from current repo, or specify:
gh api repos/octocat/hello-world

# With GH_REPO set
export GH_REPO=owner/repo
gh api repos/{owner}/{repo}  # Uses owner/repo
```

---

## GraphQL Patterns

### Basic query

```bash
gh api graphql -f query='
  query {
    viewer {
      login
      name
    }
  }
'
```

### With variables

```bash
gh api graphql -f query='
  query($owner: String!, $name: String!) {
    repository(owner: $owner, name: $name) {
      stargazerCount
      description
    }
  }
' -f owner=octocat -f name=hello-world
```

### Mutations

```bash
gh api graphql -f query='
  mutation($id: ID!) {
    addStar(input: {starrableId: $id}) {
      starrable {
        ... on Repository {
          nameWithOwner
        }
      }
    }
  }
' -f id=REPO_NODE_ID
```

### Pagination in GraphQL

```bash
gh api graphql --paginate -f query='
  query($endCursor: String) {
    viewer {
      repositories(first: 100, after: $endCursor) {
        pageInfo {
          hasNextPage
          endCursor
        }
        nodes {
          nameWithOwner
          stargazerCount
        }
      }
    }
  }
'
```

### Common fragments

```bash
# Repository info
gh api graphql -f query='
  query($owner: String!, $name: String!) {
    repository(owner: $owner, name: $name) {
      nameWithOwner
      description
      stargazerCount
      forkCount
      primaryLanguage { name }
      licenseInfo { spdxId }
      defaultBranchRef { name }
    }
  }
' -f owner=owner -f name=repo
```

---

## Pagination

### REST pagination

```bash
# Auto-paginate all results
gh api --paginate repos/{owner}/{repo}/issues

# With jq processing
gh api --paginate repos/{owner}/{repo}/issues --jq '.[].title'

# Slurp into single array
gh api --paginate --slurp repos/{owner}/{repo}/issues | jq 'flatten'
```

### Manual pagination

```bash
# First page
gh api repos/{owner}/{repo}/issues -F per_page=100 -F page=1

# Subsequent pages
gh api repos/{owner}/{repo}/issues -F per_page=100 -F page=2
```

### Handling large datasets

```bash
# Paginate and process incrementally
page=1
while true; do
    results=$(gh api repos/{owner}/{repo}/issues -F per_page=100 -F page=$page)
    count=$(echo "$results" | jq 'length')
    
    [ "$count" -eq 0 ] && break
    
    echo "$results" | jq '.[] | .title'
    
    ((page++))
done
```

### GraphQL cursor pagination

```bash
# Collect all with cursor
cursor=""
while true; do
    if [ -z "$cursor" ]; then
        result=$(gh api graphql -f query='...(first: 100)...')
    else
        result=$(gh api graphql -f query='...(first: 100, after: $cursor)...' -f cursor="$cursor")
    fi
    
    # Process nodes
    echo "$result" | jq '.data.repository.issues.nodes[]'
    
    # Check for next page
    has_next=$(echo "$result" | jq -r '.data.repository.issues.pageInfo.hasNextPage')
    [ "$has_next" != "true" ] && break
    
    cursor=$(echo "$result" | jq -r '.data.repository.issues.pageInfo.endCursor')
done
```

---

## Caching

### Cache responses

```bash
# Cache for 1 hour
gh api --cache 3600s repos/{owner}/{repo}

# Cache for 1 hour (alternative)
gh api --cache 1h repos/{owner}/{repo}

# Cache for 24 hours
gh api --cache 24h repos/{owner}/{repo}/releases
```

### When to cache

- Repo metadata that doesn't change often
- Release information
- User/org profiles
- Rate limit sensitive operations

### Cache considerations

- Cache is per-URL
- Mutations (POST/PATCH/DELETE) bypass cache
- Use for read-heavy operations

---

## Common Endpoints

### Repository

```bash
# Get repo
gh api repos/{owner}/{repo}

# Update repo
gh api repos/{owner}/{repo} -X PATCH -f description="New desc"

# Get topics
gh api repos/{owner}/{repo}/topics

# Set topics
gh api repos/{owner}/{repo}/topics -X PUT -f names='["topic1","topic2"]'

# Get languages
gh api repos/{owner}/{repo}/languages

# Get contributors
gh api repos/{owner}/{repo}/contributors
```

### Issues & PRs

```bash
# List issues
gh api repos/{owner}/{repo}/issues

# Create issue
gh api repos/{owner}/{repo}/issues -f title="Bug" -f body="Details"

# Update issue
gh api repos/{owner}/{repo}/issues/1 -X PATCH -f state="closed"

# List PR files
gh api repos/{owner}/{repo}/pulls/1/files

# Merge PR
gh api repos/{owner}/{repo}/pulls/1/merge -X PUT -f merge_method="squash"
```

### Workflows & Runs

```bash
# List workflows
gh api repos/{owner}/{repo}/actions/workflows

# List runs
gh api repos/{owner}/{repo}/actions/runs

# Get run
gh api repos/{owner}/{repo}/actions/runs/{run_id}

# Download logs
gh api repos/{owner}/{repo}/actions/runs/{run_id}/logs > logs.zip

# List artifacts
gh api repos/{owner}/{repo}/actions/runs/{run_id}/artifacts

# Trigger workflow
gh api repos/{owner}/{repo}/actions/workflows/{workflow_id}/dispatches \
  -f ref="main" -f inputs='{"environment":"staging"}'
```

### Users & Orgs

```bash
# Current user
gh api user

# Specific user
gh api users/{username}

# User repos
gh api users/{username}/repos

# Org info
gh api orgs/{org}

# Org repos
gh api orgs/{org}/repos

# Org members
gh api orgs/{org}/members
```

### Search

```bash
# Search repos
gh api search/repositories -f q='language:python stars:>100'

# Search code
gh api search/code -f q='filename:Dockerfile org:myorg'

# Search issues
gh api search/issues -f q='repo:owner/name is:issue is:open'
```

---

## Advanced Patterns

### Conditional requests (ETag)

```bash
# Get ETag from first request
response=$(gh api repos/{owner}/{repo} -i)
etag=$(echo "$response" | grep -i etag | cut -d' ' -f2 | tr -d '\r')

# Use ETag for conditional request
gh api repos/{owner}/{repo} -H "If-None-Match: $etag"
# Returns 304 if unchanged
```

### Preview headers

```bash
# Some APIs require preview headers
gh api repos/{owner}/{repo}/topics \
  -H "Accept: application/vnd.github.mercy-preview+json"
```

### Rate limit info from headers

```bash
# Get headers with response
gh api repos/{owner}/{repo} -i 2>&1 | grep -i "x-ratelimit"
```

### Batch with jq

```bash
# Get multiple repos in one script
repos=("owner/repo1" "owner/repo2" "owner/repo3")
for repo in "${repos[@]}"; do
    gh api "repos/$repo" --jq '{name: .full_name, stars: .stargazers_count}'
done | jq -s '.'
```

### Error handling

```bash
# Check for errors in response
if response=$(gh api repos/nonexistent/repo 2>&1); then
    echo "Success: $response"
else
    echo "Error: $response"
fi

# Parse error message
gh api repos/nonexistent/repo 2>&1 | jq -r '.message // "Unknown error"'
```

### Creating complex objects

```bash
# Create issue with labels and assignees
gh api repos/{owner}/{repo}/issues \
  -f title="Complex issue" \
  -f body="Description here" \
  -f labels='["bug","priority:high"]' \
  -f assignees='["user1","user2"]' \
  -F milestone=1
```

### Webhooks

```bash
# List webhooks
gh api repos/{owner}/{repo}/hooks

# Create webhook
gh api repos/{owner}/{repo}/hooks \
  -f name="web" \
  -f config='{"url":"https://example.com/webhook","content_type":"json"}' \
  -f events='["push","pull_request"]' \
  -F active=true

# Delete webhook
gh api repos/{owner}/{repo}/hooks/{hook_id} -X DELETE
```
