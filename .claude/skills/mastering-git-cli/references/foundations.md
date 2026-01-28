# Git Foundations

Git internals, configuration, and mental model.

## Table of Contents

1. [The Object Model](#the-object-model)
2. [The DAG](#the-dag-directed-acyclic-graph)
3. [The Three Trees](#the-three-trees)
4. [References](#references-refs)
5. [Configuration](#configuration)
6. [Command Categories](#command-categories)

---

## The Object Model

Git is a content-addressable filesystem. Everything is stored as objects identified by SHA hashes.

### Four Object Types

| Object | Purpose | Contains |
|--------|---------|----------|
| **blob** | File contents | Raw file data (no filename, no metadata) |
| **tree** | Directory | List of blobs and other trees with names and modes |
| **commit** | Snapshot | Tree pointer, parent pointer(s), author, committer, message |
| **tag** | Named reference | Points to a commit with optional signature and message |

**Key insight:** A blob stores content only. The same file content in different locations or with different names is the same blob.

### Inspect Objects

```bash
git cat-file -t <sha>    # Show object type
git cat-file -p <sha>    # Pretty-print object
git cat-file -s <sha>    # Show object size
```

---

## The DAG (Directed Acyclic Graph)

Commits form a DAG where each commit points to its parent(s):

```
A---B---C---D  (main)
     \
      E---F---G  (feature)
```

- **Linear history:** Each commit has exactly one parent
- **Merge commits:** Have two (or more) parents
- **Initial commit:** Has no parent
- **Acyclic:** You can never follow parents back to yourself

---

## The Three Trees

Git operates on three "trees":

| Tree | Location | Purpose |
|------|----------|---------|
| **Working Directory** | Your filesystem | Where you edit files |
| **Index (Staging Area)** | `.git/index` | What will go into next commit |
| **HEAD** | `.git/HEAD` | Last commit on current branch |

### Command Effects on Trees

| Command | Working Dir | Index | HEAD |
|---------|-------------|-------|------|
| `checkout <branch>` | ✓ | ✓ | ✓ |
| `checkout -- <file>` | ✓ | | |
| `add <file>` | | ✓ | |
| `commit` | | | ✓ |
| `reset --soft` | | | ✓ |
| `reset --mixed` | | ✓ | ✓ |
| `reset --hard` | ✓ | ✓ | ✓ |

---

## References (Refs)

Refs are human-readable pointers to commits:

```bash
.git/refs/heads/main          # Local branch
.git/refs/remotes/origin/main # Remote-tracking branch
.git/refs/tags/v1.0.0         # Tag
.git/HEAD                      # Current branch pointer
```

### HEAD Explained

- Usually points to a branch: `ref: refs/heads/main`
- In detached state: points directly to a commit SHA
- `HEAD~1` = first parent, `HEAD~2` = grandparent
- `HEAD^1` = first parent, `HEAD^2` = second parent (for merges)

### Inspect References

```bash
git rev-parse HEAD           # What SHA does HEAD point to?
git symbolic-ref HEAD        # What branch does HEAD point to?
git show-ref                 # List all refs
```

---

## Configuration

### Config Scopes

```bash
git config --system   # /etc/gitconfig (all users)
git config --global   # ~/.gitconfig (current user)
git config --local    # .git/config (current repo)
git config --worktree # .git/config.worktree (current worktree)
```

Later scopes override earlier ones.

### Find Config Source

```bash
git config --show-origin --get user.email
git config --list --show-origin
```

### 2025 Recommended Defaults

```bash
# Identity
git config --global user.name "Your Name"
git config --global user.email "you@example.com"

# Modern defaults
git config --global init.defaultBranch main
git config --global pull.rebase true
git config --global fetch.prune true
git config --global push.default current
git config --global push.autoSetupRemote true
git config --global rebase.autoStash true
git config --global rerere.enabled true
git config --global merge.conflictStyle zdiff3
git config --global diff.algorithm histogram
git config --global branch.sort -committerdate

# Performance (large repos)
git config --global feature.manyFiles true
git config --global core.fsmonitor true
git config --global core.untrackedCache true

# Safety
git config --global transfer.fsckObjects true
git config --global fetch.fsckObjects true
```

### Useful Aliases

```bash
git config --global alias.st 'status -sb'
git config --global alias.co 'checkout'
git config --global alias.br 'branch'
git config --global alias.ci 'commit'
git config --global alias.unstage 'reset HEAD --'
git config --global alias.last 'log -1 HEAD'
git config --global alias.lg "log --graph --pretty=format:'%Cred%h%Creset -%C(yellow)%d%Creset %s %Cgreen(%cr) %C(bold blue)<%an>%Creset' --abbrev-commit"
git config --global alias.branches 'branch -a --sort=-committerdate'
git config --global alias.wt 'worktree'
```

### Full Recommended .gitconfig

```ini
[user]
    name = Your Name
    email = you@example.com

[core]
    editor = vim
    autocrlf = input
    fsmonitor = true
    untrackedCache = true

[init]
    defaultBranch = main

[pull]
    rebase = true

[push]
    default = current
    autoSetupRemote = true

[fetch]
    prune = true

[rebase]
    autoStash = true
    autoSquash = true

[merge]
    conflictStyle = zdiff3

[diff]
    algorithm = histogram
    colorMoved = default
    submodule = log

[rerere]
    enabled = true

[branch]
    sort = -committerdate

[status]
    submoduleSummary = true

[submodule]
    recurse = true

[alias]
    st = status -sb
    co = checkout
    br = branch
    ci = commit
    lg = log --graph --oneline --all
    unstage = reset HEAD --
    last = log -1 HEAD
    wt = worktree
```

---

## Command Categories

### Porcelain (User-Facing)

High-level commands for daily use:
- `add`, `commit`, `push`, `pull`
- `merge`, `rebase`, `cherry-pick`
- `checkout`, `switch`, `branch`
- `log`, `diff`, `status`, `show`
- `stash`, `tag`, `remote`

### Plumbing (Low-Level)

Building blocks for scripts and debugging:
- `cat-file` — examine objects
- `hash-object` — create objects
- `ls-tree` — list tree contents
- `rev-parse` — parse revisions
- `update-ref` — update references
- `write-tree` — create tree from index

### Using Plumbing for Debugging

```bash
# What commit does HEAD point to?
git rev-parse HEAD

# What tree does that commit contain?
git cat-file -p HEAD

# List contents of that tree
git ls-tree HEAD

# Show object type
git cat-file -t abc1234
```
