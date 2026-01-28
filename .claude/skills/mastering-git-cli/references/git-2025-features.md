# Git 2025 Features and Modern Practices

> **Summary:** SHA-256 transition, Reftable backend, SSH signing, git maintenance, range-diff, bisect run, merge queues, and pre-commit framework.

New features, optimizations, and best practices for Git in 2025.

## Table of Contents

1. [SHA-256 Transition](#sha-256-transition)
2. [Reftable Backend](#reftable-backend)
3. [Modern Commands: Switch and Restore](#modern-commands-switch-and-restore)
4. [SSH Commit Signing](#ssh-commit-signing)
5. [Git Maintenance](#git-maintenance)
6. [Range-Diff](#range-diff)
7. [Automated Bisect](#automated-bisect)
8. [Merge Queues](#merge-queues)
9. [Pre-Commit Framework](#pre-commit-framework)

---

## SHA-256 Transition

Git is migrating from SHA-1 (160-bit) to SHA-256 (256-bit) for stronger cryptographic integrity.

### Status in 2025

- **Git 2.51+**: SHA-256 fully production-ready
- **Git 3.0 (2026)**: SHA-256 will be the default
- **Current state**: Most repos remain SHA-1 for compatibility

### Key Differences

| Feature | SHA-1 | SHA-256 |
|---------|-------|---------|
| Hash Length | 40 hex chars | 64 hex chars |
| Security | Broken (collisions feasible) | Secure |
| Default | Current default | Git 3.0 default |

### Creating SHA-256 Repositories

```bash
# Initialize new repo with SHA-256
git init --object-format=sha256 my-secure-repo

# Check repo hash format
git rev-parse --show-object-format
```

### Script Compatibility Warning

Scripts using 40-character regex for commit hashes will break with SHA-256. Update to handle variable-length or 64-character hashes:

```bash
# Old (breaks with SHA-256)
grep -E '^[a-f0-9]{40}$'

# New (works with both)
grep -E '^[a-f0-9]{40,64}$'
```

### Interoperability

SHA-256 repos can communicate with SHA-1 repos via translation layers, but this adds overhead. For high-security environments (fintech, government), initialize new projects with SHA-256.

---

## Reftable Backend

Modern binary storage replacing filesystem-based refs for massive performance gains.

### The Problem with Traditional Refs

Traditional refs store each branch as a file in `.git/refs/`:
- 500K branches = 500K files
- Massive I/O and inode consumption
- Lock contention on `packed-refs` updates

### Reftable Architecture

Binary block-based storage with:
- **O(log N)** lookups via binary search
- **Atomic transactions** for multiple ref updates
- **Integrated reflog** storage

### Enable Reftable

```bash
# New repository
git init --ref-format=reftable

# Migrate existing repository
git refs migrate --ref-format=reftable
```

### Performance Impact

Benchmarks on large repos (Android source):
- **58%** reduction in ref storage
- Near-instantaneous lookups
- Critical for CI systems with thousands of refs

### When to Use

- Repositories with 10,000+ branches/tags
- CI/CD environments with heavy ref churn
- Monorepos with automated branch creation

---

## Modern Commands: Switch and Restore

Git 2.23+ introduced `switch` and `restore` to replace overloaded `checkout`.

### The Problem with checkout

```bash
git checkout feature    # Switches branch
git checkout file.txt   # Restores file (DANGEROUS: overwrites!)
```

A typo could accidentally overwrite files instead of switching branches.

### git switch: Pure Branch Management

```bash
git switch feature              # Switch to branch
git switch -c new-branch        # Create and switch (replaces checkout -b)
git switch -c feature origin/feature  # Create tracking branch
git switch --detach abc1234     # Detached HEAD (explicit)
git switch -                    # Switch to previous branch
```

**Safety**: Aborts if uncommitted changes would be overwritten.

### git restore: Working Tree Surgery

```bash
git restore file.txt            # Discard changes (from index)
git restore --staged file.txt   # Unstage (replaces reset HEAD)
git restore --source=HEAD~3 file.txt  # Restore from specific commit
git restore --staged --worktree file.txt  # Unstage AND discard
git restore .                   # Restore all files
```

### Command Migration Table

| Old Command | New Command | Purpose |
|-------------|-------------|---------|
| `git checkout branch` | `git switch branch` | Switch branch |
| `git checkout -b new` | `git switch -c new` | Create and switch |
| `git checkout -- file` | `git restore file` | Discard changes |
| `git checkout HEAD~3 -- file` | `git restore --source=HEAD~3 file` | Restore old version |
| `git reset HEAD file` | `git restore --staged file` | Unstage |

### Recommendation

Use `switch` and `restore` in all new scripts and workflows. They're clearer, safer, and the future standard.

---

## SSH Commit Signing

Simpler alternative to GPG signing, using existing SSH keys.

### Why SSH Signing?

- **No GPG setup**: Use your existing SSH keys
- **Hardware support**: Works with YubiKeys via SSH agent
- **Faster verification**: Simpler than GPG
- **Platform support**: GitHub, GitLab verify SSH signatures

### Configuration

```bash
# Enable SSH signing
git config --global gpg.format ssh
git config --global user.signingkey ~/.ssh/id_ed25519.pub
git config --global commit.gpgsign true

# Optional: sign tags too
git config --global tag.gpgsign true
```

### Allowed Signers File

For local verification, create `~/.config/git/allowed_signers`:

```
your@email.com ssh-ed25519 AAAAC3NzaC1lZDI1NTE5AAAAIG...
```

Configure Git to use it:

```bash
git config --global gpg.ssh.allowedSignersFile ~/.config/git/allowed_signers
```

### Verify Signatures

```bash
git log --show-signature
git verify-commit HEAD
```

### Platform Verification

Upload your SSH public key to GitHub/GitLab as a signing key (separate from authentication key). Commits will show "Verified" badge.

---

## Git Maintenance

Scheduled background tasks for repository health and performance.

### The Old Way

```bash
git gc    # Manual, runs everything at once, slow
```

### The New Way

`git maintenance` breaks GC into granular, scheduled tasks:

| Task | Frequency | Purpose |
|------|-----------|---------|
| prefetch | Hourly | Pre-download objects from remotes |
| commit-graph | Hourly | Update graph cache for faster log/merge |
| loose-objects | Daily | Clean up unpacked objects |
| incremental-repack | Daily | Optimize packfiles incrementally |
| gc | Weekly | Full garbage collection |

### Enable Automatic Maintenance

```bash
git maintenance start
```

This:
1. Registers the repo in global config
2. Creates OS-level scheduled tasks (cron/launchd/Task Scheduler)
3. Runs tasks in background without interrupting work

### Manual Tasks

```bash
git maintenance run --task=prefetch    # Run specific task
git maintenance run                     # Run all due tasks
```

### Check Status

```bash
git maintenance unregister    # Stop automatic maintenance
git config --global --get-regexp maintenance
```

### Performance Impact

For large repos:
- `git fetch` becomes near-instant (objects pre-downloaded)
- `git log` stays fast (commit-graph updated)
- No surprise slowdowns from GC

---

## Range-Diff

Compare two sequences of commits - essential for reviewing rebased PRs.

### The Problem

After rebasing a PR, commit hashes change. Normal `git diff` shows the whole PR again, not just what changed.

### range-diff Solution

```bash
git range-diff main..feature@{1} main..feature
# Compares: old feature commits vs new feature commits
```

### Output Symbols

- `=` Commit unchanged
- `!` Commit modified (shows diff-of-diff)
- `<` Commit removed
- `>` Commit added

### Practical Usage

```bash
# Compare before/after rebase
git range-diff @{1}..@{upstream}@{1} @..@{upstream}

# Compare two branches' commits
git range-diff main...feature-v1 main...feature-v2

# See what changed in last force-push
git range-diff origin/feature@{1}..feature@{1} origin/feature..feature
```

### In Code Review

When a PR is rebased:
1. Note the old HEAD: `git rev-parse origin/pr-branch` before update
2. Fetch the update
3. Run range-diff to see only what changed in the rebase

---

## Automated Bisect

Find the commit that introduced a bug automatically.

### Basic Bisect

```bash
git bisect start
git bisect bad HEAD              # Current is broken
git bisect good v1.0.0           # This version worked
# Git checks out middle commit
# Test manually, then:
git bisect good   # or
git bisect bad
# Repeat until found
git bisect reset
```

### Automated Bisect with Script

```bash
# Create test script that exits 0 for good, 1 for bad
cat > test.sh << 'EOF'
#!/bin/bash
npm test -- --grep="login feature"
EOF
chmod +x test.sh

# Run automated bisect
git bisect start HEAD v1.0.0
git bisect run ./test.sh
```

Git will:
1. Checkout middle commit
2. Run script
3. Mark good/bad based on exit code
4. Repeat until pinpointing exact commit

### Exit Codes

- `0`: Good (test passed)
- `1-124, 126, 127`: Bad (test failed)
- `125`: Skip this commit (can't test, e.g., won't compile)

### Example: Find Performance Regression

```bash
cat > perf-test.sh << 'EOF'
#!/bin/bash
TIME=$(./benchmark.sh)
if [ "$TIME" -lt 1000 ]; then
    exit 0  # Good: under 1 second
else
    exit 1  # Bad: too slow
fi
EOF

git bisect start HEAD v2.0.0
git bisect run ./perf-test.sh
```

---

## Merge Queues

Prevent semantic conflicts in trunk-based development.

### The Problem

Two developers branch from `main`:
1. Dev A renames function `foo` to `bar`
2. Dev B calls function `foo` in new code

Both PRs pass CI (tested against old `main`). After both merge, `main` breaks because `foo` no longer exists.

### Merge Queue Solution

Instead of merging directly:

1. **Submit to queue**: PR joins merge queue
2. **Speculative merge**: Queue creates temp branch merging PR onto queue tip
3. **CI runs on merged state**: Tests run as if PR was already merged
4. **Pass = merge**: PR merges to main
5. **Fail = eject**: PR removed, queue re-tests remaining PRs

### Platform Support

- **GitHub**: Merge Queue (Settings → Branch protection → Require merge queue)
- **GitLab**: Merge Trains
- **Graphite**: Built-in stacked PRs with queue

### Benefits

- `main` stays green (all tests pass)
- Catches integration issues before merge
- Enables continuous deployment safely
- Reduces "who broke main" investigations

### When to Use

- Teams with 5+ developers
- Trunk-based development
- Continuous deployment pipelines
- High commit velocity (10+ merges/day)

---

## Pre-Commit Framework

Modern hook management replacing manual `.git/hooks/` scripts.

### The Problem with Native Hooks

- Not shared via clone (`.git/hooks/` is local)
- Manual installation required
- Version conflicts between developers
- Brittle shell scripts

### Pre-Commit Framework Solution

```yaml
# .pre-commit-config.yaml (checked into repo)
repos:
  - repo: https://github.com/pre-commit/pre-commit-hooks
    rev: v4.5.0
    hooks:
      - id: trailing-whitespace
      - id: end-of-file-fixer
      - id: check-yaml
      - id: check-json

  - repo: https://github.com/psf/black
    rev: 24.1.1
    hooks:
      - id: black

  - repo: https://github.com/astral-sh/ruff-pre-commit
    rev: v0.2.1
    hooks:
      - id: ruff
```

### Installation

```bash
pip install pre-commit
pre-commit install          # Install hooks
pre-commit run --all-files  # Run on all files
pre-commit autoupdate       # Update hook versions
```

### Benefits

- **Versioned**: Hook versions pinned in YAML
- **Portable**: Same hooks for all developers
- **Isolated**: Tools run in virtual environments
- **Cached**: Only runs on changed files
- **Language-agnostic**: Python, Node, Ruby, Go, Rust hooks

### Popular Hooks

| Tool | Purpose |
|------|---------|
| black | Python formatting |
| ruff | Fast Python linting |
| prettier | JS/TS/CSS/JSON formatting |
| eslint | JavaScript linting |
| hadolint | Dockerfile linting |
| shellcheck | Shell script linting |
| commitlint | Commit message format |

### Skip Hooks (Emergency)

```bash
git commit --no-verify -m "emergency: bypass hooks"
```

Use sparingly - hooks exist for a reason.
