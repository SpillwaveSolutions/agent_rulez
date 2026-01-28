# Large Repository Optimization

> **Summary:** Clone strategies (blobless/treeless/shallow), Scalar, sparse checkout, bare repo + worktree layout, performance configuration, and CI/CD optimization.

Scaling Git for monorepos, enterprise repos, and performance-critical environments.

## Table of Contents

1. [Clone Strategies](#clone-strategies)
2. [Scalar](#scalar)
3. [Sparse Checkout](#sparse-checkout)
4. [Bare Repository + Worktree Layout](#bare-repository--worktree-layout)
5. [Performance Configuration](#performance-configuration)
6. [CI/CD Optimization](#cicd-optimization)

---

## Clone Strategies

Different clone types optimize for different use cases.

### Full Clone (Default)

```bash
git clone https://github.com/company/monorepo.git
```

- Downloads entire history and all files
- Full offline capability
- Can be GBs for large repos
- Best for: small repos, offline work

### Shallow Clone

```bash
git clone --depth 1 https://github.com/company/repo.git
git clone --depth 10 --single-branch https://github.com/company/repo.git
```

- Downloads only recent commits
- Fast but limited:
  - Can't push without unshallowing
  - `git log`, `git blame` incomplete
  - Server-side expensive on fetch
- Best for: CI builds needing only HEAD

### Blobless Clone (Recommended for Developers)

```bash
git clone --filter=blob:none https://github.com/company/monorepo.git
```

- Downloads all commits and trees immediately
- Blobs (file contents) fetched on-demand during checkout
- Full history for `git log`, `git blame`
- Significant size reduction
- Best for: developers on large repos

### Treeless Clone

```bash
git clone --filter=tree:0 https://github.com/company/monorepo.git
```

- Downloads only commits
- Trees and blobs fetched on-demand
- Minimal initial download
- Best for: CI needing commit history but limited files

### Comparison

| Clone Type | Download Size | Offline Log | Offline Blame | Best For |
|------------|--------------|-------------|---------------|----------|
| Full | 100% | Yes | Yes | Small repos |
| Shallow | ~1% | No | No | Throwaway CI |
| Blobless | ~30% | Yes | Yes | Developers |
| Treeless | ~10% | Yes | On-demand | CI with history |

---

## Scalar

Microsoft's performance enhancements for large repos, now part of core Git.

### What Scalar Does

Scalar is a configuration wrapper that enables:
- **Partial clone** (blobless by default)
- **Sparse checkout** (only checkout needed directories)
- **FSMonitor** (daemon watches filesystem for changes)
- **Background maintenance** (scheduled prefetch, commit-graph)
- **Commit-graph** (faster log traversal)

### Installation

Scalar ships with Git 2.38+:

```bash
# Verify scalar is available
scalar version
```

### Clone with Scalar

```bash
scalar clone https://github.com/company/monorepo.git
```

This automatically:
1. Does a blobless clone
2. Enables sparse checkout
3. Starts FSMonitor
4. Registers for background maintenance

### Register Existing Repo

```bash
cd existing-repo
scalar register
```

### Scalar Configuration

```bash
scalar list              # Show registered repos
scalar unregister        # Stop managing this repo
scalar run maintenance   # Trigger maintenance
scalar diagnose          # Generate diagnostic bundle
```

### Impact

On a 100GB monorepo:
- Clone: 100GB → ~5GB initial download
- `git status`: minutes → milliseconds
- `git log`: instant with commit-graph

---

## Sparse Checkout

Work with a subset of files in a large repository.

### Cone Mode (Recommended)

Cone mode uses directory patterns for O(1) matching:

```bash
# Initialize sparse checkout
git sparse-checkout init --cone

# Add directories to checkout
git sparse-checkout set backend/api frontend/shared

# Add more directories
git sparse-checkout add docs/

# List current patterns
git sparse-checkout list

# Disable sparse checkout
git sparse-checkout disable
```

### What Gets Checked Out

With `git sparse-checkout set backend/api frontend/shared`:
- `/backend/api/**` - all files recursively
- `/frontend/shared/**` - all files recursively
- Root files (`README.md`, etc.) - always included
- Parent directories (structure only)

### Non-Cone Mode (Legacy)

Pattern-based, like `.gitignore`:

```bash
git sparse-checkout init --no-cone
git sparse-checkout set '*.md' 'src/**/*.js'
```

Slower than cone mode - avoid for large repos.

### Sparse Checkout with Worktrees

Combine for powerful monorepo workflows:

```bash
# Main worktree: full repo
cd ~/monorepo

# Frontend worktree: sparse
git worktree add ../frontend-only main
cd ../frontend-only
git sparse-checkout init --cone
git sparse-checkout set frontend/

# Backend worktree: sparse
git worktree add ../backend-only main
cd ../backend-only
git sparse-checkout init --cone
git sparse-checkout set backend/
```

### Sparse Index

For even faster performance on sparse repos:

```bash
git config core.sparseCheckoutCone true
git config index.sparse true
```

The sparse index only tracks entries in the sparse checkout, reducing index size dramatically.

---

## Bare Repository + Worktree Layout

Professional setup treating all branches as equal peers.

### The Problem with Normal Clones

```
~/project/          # main branch, contains .git/
~/project-feature/  # clone #2, duplicates .git/objects
~/project-hotfix/   # clone #3, duplicates .git/objects
```

Each clone duplicates the entire object store.

### Bare Repo + Worktrees Solution

```
~/project/
├── .bare/          # Git database (shared by all worktrees)
├── .git            # Pointer file to .bare
├── main/           # Worktree for main branch
├── feature-a/      # Worktree for feature-a
└── hotfix/         # Worktree for hotfix
```

### Setup

```bash
mkdir my-project && cd my-project

# Clone as bare repo into hidden directory
git clone --bare git@github.com:company/repo.git .bare

# Create pointer file
echo "gitdir: ./.bare" > .git

# Fix fetch to get all remote branches
git config --file .bare/config remote.origin.fetch "+refs/heads/*:refs/remotes/origin/*"

# Fetch all branches
git fetch origin

# Create worktrees
git worktree add main
git worktree add feature-a
git worktree add hotfix origin/hotfix
```

### Benefits

- **Single object store**: No duplication
- **Shared refs**: Fetch once, update all
- **Equal branches**: No "primary" worktree
- **Clean structure**: Organized directory layout

### Script: Setup Bare Layout

```bash
#!/bin/bash
# setup-bare-layout.sh <repo-url> <project-name>
REPO_URL=$1
PROJECT_NAME=$2

mkdir "$PROJECT_NAME" && cd "$PROJECT_NAME"
git clone --bare "$REPO_URL" .bare
echo "gitdir: ./.bare" > .git
git config --file .bare/config remote.origin.fetch "+refs/heads/*:refs/remotes/origin/*"
git fetch origin
git worktree add main main
echo "Created bare layout in $PROJECT_NAME/"
```

---

## Performance Configuration

Essential settings for large repositories.

### Core Settings

```bash
# FSMonitor: daemon watches for filesystem changes
git config --global core.fsmonitor true
git config --global core.untrackedCache true

# Multi-pack index for faster object access
git config --global core.multiPackIndex true

# Preload index for faster status
git config --global core.preloadIndex true

# Enable many-files optimizations
git config --global feature.manyFiles true
```

### Fetch/Clone Settings

```bash
# Prune deleted remote branches
git config --global fetch.prune true

# Parallel fetch (experimental in 2025)
git config --global fetch.parallel 4

# Reduce memory on partial clones
git config --global fetch.writeCommitGraph true
```

### Diff/Status Settings

```bash
# Faster diff algorithm
git config --global diff.algorithm histogram

# Colorize moved lines in diff
git config --global diff.colorMoved default

# Skip optional locks for status
git config --global core.skipOptionalLocks true
```

### Display Settings

```bash
# Columnar branch/tag listing
git config --global column.ui auto

# Sort branches by commit date
git config --global branch.sort -committerdate
```

### Full Performance .gitconfig

```ini
[core]
    fsmonitor = true
    untrackedCache = true
    multiPackIndex = true
    preloadIndex = true
    skipOptionalLocks = true

[feature]
    manyFiles = true

[fetch]
    prune = true
    writeCommitGraph = true

[diff]
    algorithm = histogram
    colorMoved = default

[index]
    sparse = true

[column]
    ui = auto
```

---

## CI/CD Optimization

Strategies for efficient Git operations in pipelines.

### Choose the Right Clone

| Scenario | Strategy |
|----------|----------|
| Build only HEAD | Shallow clone `--depth 1` |
| Need version info | Treeless clone + `git describe` |
| Need blame/history | Blobless clone |
| Multi-job pipeline | Clone once, artifact cache |

### Shallow Clone for Speed

```yaml
# GitHub Actions
- uses: actions/checkout@v4
  with:
    fetch-depth: 1

# GitLab CI
variables:
  GIT_DEPTH: 1
```

### Fetch Only Needed Refs

```bash
# Instead of fetching all branches
git fetch origin main:main
git fetch origin +refs/heads/feature-*:refs/remotes/origin/feature-*
```

### Pre-Clone Bundle

For massive repos, pre-package a bundle:

```bash
# Create bundle (run periodically)
git bundle create repo-bundle.bundle --all
# Upload to S3/GCS

# In CI: download and extract
curl -O https://storage/repo-bundle.bundle
git clone repo-bundle.bundle repo/
git remote set-url origin git@github.com:company/repo.git
git fetch origin  # Only incremental changes
```

### Cache Between Builds

```yaml
# GitHub Actions - cache .git
- uses: actions/cache@v4
  with:
    path: .git
    key: git-${{ hashFiles('.git/HEAD') }}
    restore-keys: git-
```

### Avoid Common Mistakes

```bash
# DON'T: Full clone for simple builds
git clone https://github.com/company/huge-repo.git  # 10GB, 5 minutes

# DO: Shallow clone for builds
git clone --depth 1 https://github.com/company/huge-repo.git  # 500MB, 30 seconds

# DON'T: Fetch all branches
git fetch --all  # Downloads everything

# DO: Fetch only needed branch
git fetch origin main  # Just what you need
```

### Git LFS in CI

```yaml
# Skip LFS in builds that don't need assets
variables:
  GIT_LFS_SKIP_SMUDGE: 1

# Or fetch only specific patterns
script:
  - git lfs fetch --include="*.png"
```

### Performance Metrics

Track CI Git performance:

```bash
GIT_TRACE_PERFORMANCE=1 git clone ...
```

Look for:
- Clone time
- Checkout time
- Fetch negotiation rounds
