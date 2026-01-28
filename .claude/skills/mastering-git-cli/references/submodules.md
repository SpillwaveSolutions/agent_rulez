# Git Submodules

Managing multiple repositories and multi-repo architectures.

## Table of Contents

1. [Submodule Fundamentals](#submodule-fundamentals)
2. [Submodule Operations](#submodule-operations)
3. [Multi-Repo Project Management](#multi-repo-project-management)
4. [Best Practices](#best-practices)
5. [Troubleshooting](#troubleshooting)
6. [Alternatives](#alternatives)

---

## Submodule Fundamentals

### What Are Submodules?

Submodules let you include one Git repository inside another, at a specific commit.

**Parent repo stores:**
- Reference to submodule's repository URL
- Reference to a specific commit SHA
- Path where submodule should live

**Key characteristics:**
- Parent tracks a specific commit, not a branch
- Submodule is a full Git repo (its own `.git`)
- Changes in submodule must be committed separately
- Parent must be updated to point to new commits

### When to Use Submodules

**Good use cases:**
- Shared libraries across multiple projects
- Third-party dependencies you want to modify
- Monorepo-style organization with independent versioning
- Separating concerns with different release cycles

**Consider alternatives:**
- Package managers (npm, pip, maven) — for true dependencies
- Monorepo — if tight coupling desired
- Subtree merge — for simpler, merged history
- Git LFS — for large binary files (not repos)

---

## Submodule Operations

### Adding a Submodule

```bash
git submodule add https://github.com/org/library.git libs/library
git submodule add -b main https://github.com/org/library.git libs/library

# Creates:
# - .gitmodules file (tracked configuration)
# - .git/config entries (local configuration)
# - Special entry in tree (commit pointer)
```

**.gitmodules file:**
```ini
[submodule "libs/library"]
    path = libs/library
    url = https://github.com/org/library.git
    branch = main
```

### Cloning with Submodules

```bash
# Option 1: Clone then init
git clone https://github.com/org/parent.git
cd parent
git submodule init
git submodule update

# Option 2: All in one
git clone --recurse-submodules https://github.com/org/parent.git

# Option 3: Forgot --recurse-submodules
git submodule update --init

# Nested submodules
git submodule update --init --recursive
```

### Updating Submodules

```bash
# Update to commit recorded in parent
git submodule update

# Update to latest on tracked branch
git submodule update --remote

# Update specific submodule
git submodule update --remote libs/library

# Update all, including nested
git submodule update --init --recursive

# Merge instead of checkout (preserve local changes)
git submodule update --remote --merge

# Rebase instead of checkout
git submodule update --remote --rebase
```

### Making Changes in Submodules

```bash
cd libs/library

# IMPORTANT: Checkout a branch first!
git checkout main

# Make changes
vim file.txt
git add file.txt
git commit -m "Update library"
git push origin main

# Return to parent
cd ../..

# Parent now shows change
git status
# modified: libs/library (new commits)

# Commit submodule update in parent
git add libs/library
git commit -m "Update library submodule"
git push
```

### Syncing Submodule URLs

If submodule URL changes:

```bash
vim .gitmodules                          # Update URL
git submodule sync                       # Sync to .git/config
git submodule update --init --recursive  # Update from new URL
```

### Removing a Submodule

```bash
git rm libs/library
git commit -m "Remove library submodule"

# Manual cleanup if needed
rm -rf .git/modules/libs/library
```

### Submodule Status

```bash
git submodule status

# Output:
#  abc1234 libs/library (v1.0)      # Normal
# +def5678 libs/library (v1.1)      # Different commit checked out
# -abc1234 libs/library             # Not initialized

git submodule summary                    # Detailed status

# Run command in each submodule
git submodule foreach 'git status'
git submodule foreach 'git fetch --all'
```

---

## Multi-Repo Project Management

### Project Structure

```
parent-project/
├── .gitmodules
├── .git/
├── src/
│   └── main.py
├── libs/
│   ├── auth/          (submodule)
│   ├── database/      (submodule)
│   └── common/        (submodule)
└── docs/
    └── api/           (submodule)
```

### Setup Workflow

```bash
mkdir parent-project && cd parent-project
git init

git submodule add git@github.com:org/auth.git libs/auth
git submodule add git@github.com:org/database.git libs/database
git submodule add git@github.com:org/common.git libs/common
git submodule add git@github.com:org/api-docs.git docs/api

git add .
git commit -m "Initial structure with submodules"
```

### Team Member Clone

```bash
git clone --recurse-submodules git@github.com:org/parent-project.git

# Or if already cloned
git submodule update --init --recursive
```

### Update All to Latest

```bash
git submodule foreach 'git fetch origin'
git submodule update --remote
git diff --submodule
git add .
git commit -m "Update all submodules"
```

### Pin to Specific Version

```bash
cd libs/auth
git checkout v2.1.0
cd ../..
git add libs/auth
git commit -m "Pin auth to v2.1.0"
```

### Branch Tracking

```ini
# In .gitmodules
[submodule "libs/auth"]
    path = libs/auth
    url = git@github.com:org/auth.git
    branch = develop
```

```bash
git submodule update --remote libs/auth
```

---

## Best Practices

### Commit Ordering

Always commit submodule changes before parent:

```bash
# CORRECT
cd libs/auth
git add . && git commit -m "Fix auth bug"
git push origin main

cd ../..
git add libs/auth
git commit -m "Update auth submodule"
git push

# WRONG: Push parent before submodule
# Other team members get errors!
```

### CI/CD Configuration

```yaml
# GitHub Actions
jobs:
  build:
    steps:
      - uses: actions/checkout@v4
        with:
          submodules: recursive

      # For private submodules
      - name: Setup SSH
        uses: webfactory/ssh-agent@v0.8.0
        with:
          ssh-private-key: ${{ secrets.SUBMODULE_SSH_KEY }}
```

### Helpful Configurations

```bash
git config --global status.submoduleSummary true
git config --global diff.submodule log
git config --global submodule.recurse true
git config --global alias.pullall '!git pull && git submodule update --init --recursive'
```

### Common Patterns

**Shared libs with stable/dev:**
```
libs/
├── lib-stable/      (tracks: stable branch)
└── lib-dev/         (tracks: develop branch)
```

**Documentation as submodule:**
```
project/
├── src/
└── docs/            (submodule)
```

**Environment configs:**
```
config/
├── dev/             (submodule to dev-config)
└── prod/            (submodule to prod-config)
```

---

## Troubleshooting

### Detached HEAD

**Symptom:** Changes disappear after `git submodule update`

**Cause:** Submodules checkout commits, not branches

**Solution:**
```bash
cd libs/library
git checkout main    # Before making changes
```

### "Reference is not a tree"

**Symptom:** `git submodule update` fails

**Cause:** Parent references unpushed commit

**Solution:**
```bash
cd libs/library
git push origin main
cd ../..
git submodule update
```

### Merge Conflicts in Submodule Pointer

**Symptom:** Conflict shows different commit SHAs

**Solutions:**
```bash
# Take theirs
git checkout --theirs libs/library
git add libs/library

# Take ours
git checkout --ours libs/library
git add libs/library

# Choose specific commit
cd libs/library
git checkout <sha>
cd ../..
git add libs/library
```

### Private Submodule Clone Fails

```bash
ssh-add ~/.ssh/id_rsa
git clone --recurse-submodules git@github.com:org/parent.git
```

### Change Submodule URL

```bash
vim .gitmodules
git submodule sync --recursive
git submodule update --init --recursive
```

---

## Alternatives

### Subtree Merge

Include external repo directly (not separate repo):

```bash
git remote add library https://github.com/org/library.git
git fetch library
git merge -s subtree --allow-unrelated-histories library/main

# Updates
git fetch library
git merge -s subtree library/main
```

**Pros:** Simpler for contributors  
**Cons:** Harder to contribute back

### Git Subtree Command

```bash
# Add
git subtree add --prefix=libs/library https://github.com/org/library.git main --squash

# Update
git subtree pull --prefix=libs/library https://github.com/org/library.git main --squash

# Contribute back
git subtree push --prefix=libs/library https://github.com/org/library.git feature
```

**Pros:** No initialization needed  
**Cons:** Verbose commands

### Comparison

| Feature | Submodules | Subtree | Package Managers |
|---------|------------|---------|------------------|
| Separate history | Yes | Mixed | Yes |
| Easy clone | Requires init | Yes | Requires install |
| Easy update | Medium | Easy | Very easy |
| Contribute back | Easy | Medium | Hard |
| Specific version | Easy | Hard | Easy |
| Works offline | After init | Yes | After install |
| CI complexity | Medium | Low | Low |
