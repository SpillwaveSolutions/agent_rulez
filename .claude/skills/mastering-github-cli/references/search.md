# Search & Discovery Reference

Comprehensive reference for finding repositories and code on GitHub using `gh` CLI.

## Contents

- [Repository Search](#repository-search)
- [Code Search](#code-search)
- [Path-Based Discovery](#path-based-discovery)
- [Search Qualifiers](#search-qualifiers)
- [Handling Large Results](#handling-large-results)
- [API Search Patterns](#api-search-patterns)
- [Common Recipes](#common-recipes)

---

## Repository Search

### Basic syntax

```bash
gh search repos [query] [flags]
```

### Filter flags

| Flag | Description | Example |
|------|-------------|---------|
| `--language` | Filter by language | `--language=python` |
| `--owner` | Filter by owner/org | `--owner=microsoft` |
| `--topic` | Filter by topic | `--topic=cli` |
| `--stars` | Filter by star count | `--stars=">100"` |
| `--forks` | Filter by fork count | `--forks=">=50"` |
| `--created` | Filter by creation date | `--created=">2024-01-01"` |
| `--pushed` | Filter by last push | `--pushed=">=2024-06-01"` |
| `--archived` | Include/exclude archived | `--archived=false` |
| `--visibility` | public/private/internal | `--visibility=public` |
| `--license` | Filter by license | `--license=mit` |
| `--limit` | Max results (default 30) | `--limit=100` |
| `--sort` | Sort field | `--sort=stars` |
| `--order` | Sort order | `--order=desc` |

### Numeric ranges

```bash
# Exact value
--stars=100

# Greater/less than
--stars=">100"
--stars="<50"
--stars=">=100"

# Range
--stars="100..1000"

# Unlimited upper bound
--stars="100..*"
```

### Date ranges

```bash
# After date
--created=">2024-01-01"

# Before date  
--pushed="<2024-06-01"

# Range
--created="2024-01-01..2024-06-30"
```

### Examples

```bash
# Popular Python CLIs
gh search repos --language=python --topic=cli --stars=">100" --archived=false

# Recently active in org
gh search repos --owner=myorg --pushed=">=2024-01-01"

# With specific license
gh search repos "database" --language=go --license=apache-2.0

# JSON output
gh search repos "machine learning" --json fullName,stargazersCount,url --limit 50
```

---

## Code Search

### Basic syntax

```bash
gh search code [query] [flags]
```

### Filter flags

| Flag | Description | Example |
|------|-------------|---------|
| `--filename` | Match filename | `--filename=Dockerfile` |
| `--extension` | Match extension | `--extension=py` |
| `--language` | Filter by language | `--language=python` |
| `--repo` | Search specific repo | `--repo=owner/name` |
| `--owner` | Search owner/org | `--owner=myorg` |
| `--limit` | Max results | `--limit=100` |

### Query qualifiers (in query string)

| Qualifier | Description | Example |
|-----------|-------------|---------|
| `path:` | Match file path | `"path:src/components"` |
| `path:/` | Match repo root | `"path:/ README"` |
| `filename:` | Match filename | `"filename:config.yml"` |
| `extension:` | Match extension | `"extension:ts"` |
| `language:` | Filter language | `"language:javascript"` |
| `repo:` | Specific repo | `"repo:owner/name"` |
| `org:` | Organization | `"org:github"` |
| `user:` | User repos | `"user:octocat"` |
| `size:` | File size (bytes) | `"size:>1000"` |

### Examples

```bash
# Find specific files
gh search code --filename SKILL.md
gh search code --filename Dockerfile --owner myorg
gh search code --filename pyproject.toml --language python

# Search file contents
gh search code "TODO" --extension py --owner myorg
gh search code "import pandas" --language python

# Combined qualifiers
gh search code "path:.github" --filename workflow.yml
gh search code "config" --extension json --repo owner/repo
```

---

## Path-Based Discovery

Find repositories containing specific directory structures.

### Directory detection patterns

```bash
# Root-level directories
gh search code "path:.skilz"
gh search code "path:.cursor"
gh search code "path:.codex"
gh search code "path:.github"

# Nested paths
gh search code "path:src/components"
gh search code "path:packages/core"
gh search code "path:some/nested/.skilz"

# Any file in directory
gh search code "path:.skilz" --json repository --jq '.[].repository.fullName'
```

### Finding repos with specific structures

```bash
# Repos with .skilz directory
gh search code "path:.skilz" --json repository --jq '[.[].repository.fullName] | unique'

# Repos with .cursor config
gh search code "path:.cursor" --owner myorg --json repository,path

# Repos with specific nested structure
gh search code "path:config/settings" --json repository --jq '[.[].repository.fullName] | unique'
```

### Combining path with filename

```bash
# SKILL.md in .skilz directory
gh search code "path:.skilz" --filename SKILL.md

# workflow.yml in .github/workflows
gh search code "path:.github/workflows" --filename "*.yml"

# Config files in specific paths
gh search code "path:src/config" --extension json
```

### Extracting unique repos

```bash
# Get unique repo names from code search
gh search code "path:.skilz" --json repository --jq '[.[].repository.fullName] | unique | .[]'

# With owner filter
gh search code "path:.cursor" --owner myorg --json repository \
  --jq '[.[].repository.fullName] | unique | .[]'

# Count repos
gh search code "path:.codex" --json repository \
  --jq '[.[].repository.fullName] | unique | length'
```

---

## Search Qualifiers

### Code search qualifiers reference

| Qualifier | Operators | Example |
|-----------|-----------|---------|
| `filename:` | exact match | `filename:Dockerfile` |
| `path:` | prefix match | `path:src/` |
| `extension:` | exact match | `extension:py` |
| `language:` | exact match | `language:python` |
| `repo:` | exact match | `repo:owner/name` |
| `org:` | exact match | `org:github` |
| `user:` | exact match | `user:octocat` |
| `size:` | `>`, `<`, `..` | `size:>10000` |

### Repository search qualifiers reference

| Qualifier | Operators | Example |
|-----------|-----------|---------|
| `language:` | exact match | `language:rust` |
| `stars:` | `>`, `<`, `..` | `stars:>1000` |
| `forks:` | `>`, `<`, `..` | `forks:>=100` |
| `created:` | `>`, `<`, `..` | `created:>2024-01-01` |
| `pushed:` | `>`, `<`, `..` | `pushed:>=2024-06-01` |
| `archived:` | `true`, `false` | `archived:false` |
| `is:` | `public`, `private` | `is:public` |
| `topic:` | exact match | `topic:cli` |
| `license:` | SPDX identifier | `license:mit` |

### Using raw query syntax

For complex queries, use `--` to pass raw query:

```bash
# Multiple exclusions
gh search repos -- "cli language:rust stars:>50 -topic:deprecated"

# NOT operator
gh search repos -- "-topic:linux" --language=python

# OR operator (in query)
gh search code -- "filename:Dockerfile OR filename:docker-compose.yml"
```

---

## Handling Large Results

GitHub Search API limits results to **1,000 maximum**. Workarounds:

### Date partitioning

Split searches by date ranges:

```bash
# First half of year
gh search repos --language=python --created="2024-01-01..2024-06-30" --limit 1000

# Second half of year
gh search repos --language=python --created="2024-07-01..2024-12-31" --limit 1000
```

### Star partitioning

Split by popularity:

```bash
# High stars
gh search repos --topic=cli --stars=">1000" --limit 1000

# Medium stars
gh search repos --topic=cli --stars="100..1000" --limit 1000

# Lower stars
gh search repos --topic=cli --stars="10..100" --limit 1000
```

### Automated partitioning script

Use `scripts/batch-search.sh` for automatic date partitioning:

```bash
./scripts/batch-search.sh "language:python topic:cli" 2024-01-01 2024-12-31
```

### Pagination limitations

**Important:** `gh search` commands do NOT support pagination flags. The `--paginate` flag only works with `gh api`.

```bash
# This does NOT work:
gh search repos --paginate  # ERROR

# Use gh api instead for pagination:
gh api --paginate search/repositories -f q='language:python stars:>100'
```

---

## API Search Patterns

### REST API search

```bash
# Repository search
gh api -X GET search/repositories -f q='language:python stars:>100'

# Code search
gh api -X GET search/code -f q='filename:SKILL.md path:.skilz'

# Issue/PR search
gh api -X GET search/issues -f q='repo:owner/name is:pr is:open'
```

### With jq filtering

```bash
# Extract repo names
gh api search/repositories -f q='topic:cli' \
  --jq '.items[] | {name: .full_name, stars: .stargazers_count}'

# Get total count
gh api search/repositories -f q='language:rust' --jq '.total_count'

# Extract URLs
gh api search/code -f q='filename:Dockerfile' \
  --jq '.items[] | .repository.html_url' | sort -u
```

### GraphQL search

```bash
gh api graphql -f query='
  query($q: String!) {
    search(query: $q, type: REPOSITORY, first: 100) {
      repositoryCount
      nodes {
        ... on Repository {
          nameWithOwner
          stargazerCount
          description
        }
      }
    }
  }
' -f q='language:go stars:>500'
```

### Paginated API search

```bash
# REST with pagination
gh api --paginate search/repositories -f q='topic:cli' \
  --jq '.items[].full_name'

# Note: Still limited to 1000 total results by GitHub
```

---

## Common Recipes

### Find all repos with a specific file

```bash
gh search code --filename SKILL.md --json repository \
  --jq '[.[].repository.fullName] | unique | .[]'
```

### Find repos with directory structure in org

```bash
gh search code "path:.skilz" --owner myorg --json repository,path \
  --jq 'group_by(.repository.fullName) | map({repo: .[0].repository.fullName, files: map(.path)})'
```

### Search and clone matching repos

```bash
gh search code "path:.cursor" --json repository --jq '.[].repository.fullName' | \
  sort -u | while read repo; do
    gh repo clone "$repo" "repos/$(basename $repo)"
done
```

### Find repos matching multiple criteria

```bash
# Has both Dockerfile and pyproject.toml
gh search code --filename Dockerfile --json repository --jq '.[].repository.fullName' | sort -u > /tmp/docker.txt
gh search code --filename pyproject.toml --json repository --jq '.[].repository.fullName' | sort -u > /tmp/python.txt
comm -12 /tmp/docker.txt /tmp/python.txt
```

### Export search results to JSON

```bash
gh search repos --language=python --stars=">100" --limit 100 \
  --json fullName,description,stargazersCount,url \
  > search-results.json
```

### Count repos by language in results

```bash
gh search repos "database" --limit 100 --json primaryLanguage \
  --jq 'group_by(.primaryLanguage.name) | map({lang: .[0].primaryLanguage.name, count: length}) | sort_by(-.count)'
```

---

## Rate Limits

| Endpoint | Limit |
|----------|-------|
| Search API | 30 requests/minute |
| Code Search | 10 requests/minute |
| Max results | 1,000 per query |

### Check limits

```bash
gh api rate_limit --jq '.resources.search'
```

### Handling rate limits

```bash
# Add delay between searches
for query in "${queries[@]}"; do
    gh search code "$query" --json repository
    sleep 6  # Stay under 10/minute for code search
done
```
