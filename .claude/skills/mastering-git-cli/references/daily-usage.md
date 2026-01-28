# Daily Git Usage

Core workflows for repositories, commits, history, and branches.

## Table of Contents

1. [Creating and Cloning Repositories](#creating-and-cloning-repositories)
2. [The Commit Workflow](#the-commit-workflow)
3. [Inspecting History](#inspecting-history)
4. [Branching](#branching)

---

## Creating and Cloning Repositories

### Initialize New Repository

```bash
git init                           # Initialize in current directory
git init my-project                # Create directory and initialize
git init --bare my-project.git     # Bare repo (servers)
```

### Clone Existing Repository

```bash
git clone <url>                              # Standard clone
git clone <url> my-directory                 # Clone to specific directory
git clone git@github.com:user/repo.git       # SSH

# RECOMMENDED for large repos (2025): Blobless partial clone
git clone --filter=blob:none <url>           # Full history, blobs on-demand
# Best balance: fast clone, full git log/blame, minimal disk space

# Shallow clone (CI/throwaway only - not for development!)
git clone --depth 1 <url>                    # Only latest commit
git clone --depth 50 <url>                   # Last 50 commits
# WARNING: Shallow breaks push, blame, log - avoid for development

# Treeless (CI needing commit history)
git clone --filter=tree:0 <url>              # Trees and blobs on-demand

# Scalar (Microsoft's optimization wrapper)
scalar clone <url>                           # Blobless + sparse + maintenance

# Single branch clone
git clone --single-branch --branch main <url>
```

### Unshallowing

```bash
git fetch --unshallow              # Convert shallow to full
git fetch --depth 100              # Get more history
git fetch --deepen 50              # Add 50 more commits
```

---

## The Commit Workflow

### Status

```bash
git status                         # Full status
git status -sb                     # Short format with branch
```

### Staging

```bash
git add <file>                     # Stage specific file
git add .                          # Stage all in current directory
git add -A                         # Stage all in entire repo
git add -p                         # Interactive patch mode
git add -N <file>                  # Intent to add (track without staging)
```

### Unstaging

```bash
git reset HEAD <file>              # Classic
git restore --staged <file>        # Modern (Git 2.23+)
```

### Committing

```bash
git commit -m "message"            # Commit with message
git commit                         # Open editor
git commit -a -m "message"         # Stage tracked files and commit
git commit --amend                 # Modify last commit
git commit --amend --no-edit       # Amend without changing message
git commit --fixup <sha>           # Create fixup commit
git commit --allow-empty -m "msg"  # Empty commit (CI triggers)
```

### Commit Message Format

```
<type>(<scope>): <subject>

<body>

<footer>
```

**Types:** feat, fix, docs, style, refactor, test, chore

**Example:**
```
feat(auth): add OAuth2 integration for Google

Implements Google OAuth2 flow with PKCE for enhanced security.
Includes token refresh handling and session management.

Closes #123
```

**The Seven Rules:**
1. Separate subject from body with blank line
2. Limit subject to 50 characters
3. Capitalize subject line
4. Don't end subject with period
5. Use imperative mood ("Add feature" not "Added feature")
6. Wrap body at 72 characters
7. Explain what and why, not how

---

## Inspecting History

### Git Log

```bash
git log                            # Full log
git log --oneline                  # Compact
git log --graph                    # ASCII graph
git log --graph --oneline --all    # Graph of all branches
```

### Filtering Log

```bash
git log -n 10                      # Last 10 commits
git log --since="2024-01-01"
git log --until="2024-06-30"
git log --author="name"
git log --grep="pattern"           # Search commit messages
git log -S "code"                  # Search for code changes (pickaxe)
git log -G "regex"                 # Search with regex
git log -- path/to/file            # History of specific file
git log --follow -- file           # Follow renames
```

### Formatting Log

```bash
git log --pretty=format:"%h %an %s"
git log --pretty=fuller            # Show author and committer
git log --stat                     # Show file change stats
git log --name-only                # Show changed filenames
```

### Git Show

```bash
git show <sha>                     # Show commit details
git show <sha>:<file>              # Show file at specific commit
git show <sha> --stat              # Commit with stats
```

### Git Diff

```bash
git diff                           # Working dir vs index
git diff --staged                  # Index vs HEAD
git diff HEAD                      # Working dir vs HEAD
git diff <branch1>..<branch2>      # Between branches
git diff <sha1>..<sha2>            # Between commits
git diff --stat                    # Summary statistics
git diff --name-only               # Just filenames
git diff --word-diff               # Word-level diff
```

### Git Blame

```bash
git blame <file>                   # Line-by-line authorship
git blame -L 10,20 <file>          # Lines 10-20 only
git blame -w <file>                # Ignore whitespace
git blame -M <file>                # Detect moved lines
git blame -C <file>                # Detect copied lines
```

---

## Branching

### List Branches

```bash
git branch                         # Local branches
git branch -r                      # Remote-tracking branches
git branch -a                      # All branches
git branch -v                      # With last commit
git branch --merged                # Merged into current
git branch --no-merged             # Not merged
git branch --sort=-committerdate   # Sort by recent activity
```

### Create Branches

```bash
git branch <name>                  # Create at HEAD
git branch <name> <sha>            # Create at specific commit
git checkout -b <name>             # Create and switch (classic)
git switch -c <name>               # Create and switch (modern)
```

### Switch Branches

**Prefer `git switch` (Git 2.23+)** - clearer, safer, less overloaded than checkout.

```bash
git switch <branch>                # Modern (RECOMMENDED)
git switch -                       # Previous branch
git switch --detach <sha>          # Detached HEAD (explicit)

# Legacy (avoid in new scripts)
git checkout <branch>              # Ambiguous with file checkout
git checkout -                     # Previous branch
```

### Delete Branches

```bash
git branch -d <name>               # Delete (safe, must be merged)
git branch -D <name>               # Force delete
git push origin --delete <name>    # Delete remote branch
```

### Rename Branches

```bash
git branch -m <old> <new>          # Rename branch
git branch -m <new>                # Rename current branch
```

### Track Remote Branch

```bash
git branch -u origin/main          # Set upstream for current branch
git branch --set-upstream-to=origin/main feature
git checkout --track origin/feature  # Create local from remote
```
