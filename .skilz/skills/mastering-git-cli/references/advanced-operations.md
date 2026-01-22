# Advanced Git Operations

Reset, revert, restore, rebase, stash, tags, and hooks.

## Table of Contents

1. [Reset, Revert, and Restore](#reset-revert-and-restore)
2. [Interactive Rebase](#interactive-rebase)
3. [Stashing](#stashing)
4. [Tags and Releases](#tags-and-releases)
5. [Git Hooks](#git-hooks)

---

## Reset, Revert, and Restore

### Reset

Moves HEAD and optionally modifies index/working directory:

```bash
# Soft: Move HEAD only (staging + working dir unchanged)
git reset --soft HEAD~1
# Use: Uncommit, keep changes staged

# Mixed (default): Move HEAD + reset index
git reset HEAD~1
git reset --mixed HEAD~1
# Use: Uncommit + unstage, keep changes in working dir

# Hard: Move HEAD + reset index + reset working dir
git reset --hard HEAD~1
# Use: Completely undo, discard everything
```

### Revert

Creates new commit that undoes changes (safe for shared branches):

```bash
git revert abc1234                # Single commit

# Revert merge (specify parent to keep)
git revert -m 1 <merge-sha>       # Keep first parent (main)
git revert -m 2 <merge-sha>       # Keep second parent (feature)

git revert abc1234..def5678       # Multiple commits
git revert -n abc1234             # Stage only, don't commit
```

### Restore (Git 2.23+)

Modern file restoration:

```bash
git restore <file>                       # Restore from index
git restore --source=HEAD~1 <file>       # Restore from commit
git restore --staged <file>              # Unstage file
git restore --staged --worktree <file>   # Restore both
git restore src/                         # Restore directory
```

### Decision Guide

| Goal | Command |
|------|---------|
| Undo last commit, keep staged | `git reset --soft HEAD~1` |
| Undo last commit, keep unstaged | `git reset HEAD~1` |
| Undo last commit, discard all | `git reset --hard HEAD~1` |
| Undo pushed commit | `git revert <sha>` |
| Discard file changes | `git restore <file>` |
| Unstage file | `git restore --staged <file>` |
| Get old file version | `git restore --source=<sha> <file>` |

---

## Interactive Rebase

### Basic Usage

```bash
git rebase -i HEAD~5              # Last 5 commits
git rebase -i main                # Onto main
git rebase -i --root              # Entire history
```

### Commands

```
pick abc1234 First commit         # Keep as-is
reword def5678 Second commit      # Keep, edit message
edit 789abcd Third commit         # Stop to amend
squash 1234567 Fourth commit      # Combine with previous
fixup 89abcde Fifth commit        # Combine, discard message
drop fedcba1 Sixth commit         # Remove entirely
```

### Common Operations

**Squash commits:**
```
pick abc1234 Main feature
squash def5678 Fix typo
squash 789abcd Another fix
```

**Reorder commits:**
```
pick 789abcd Third commit    # Move up
pick abc1234 First commit
pick def5678 Second commit
```

**Split a commit:**
```
edit abc1234 Big commit
# Git stops here
git reset HEAD~1
git add file1.txt && git commit -m "First part"
git add file2.txt && git commit -m "Second part"
git rebase --continue
```

### Autosquash Workflow

```bash
# During development
git commit --fixup=abc1234        # Creates: fixup! Original message
git commit --squash=abc1234       # Creates: squash! Original message

# Later
git rebase -i --autosquash main   # Auto-positions fixups
```

### Conflict Resolution

```bash
# Fix conflict
git add resolved-file.txt
git rebase --continue

# Skip commit
git rebase --skip

# Abort
git rebase --abort
```

---

## Stashing

### Basic Operations

```bash
git stash                         # Stash working changes
git stash push                    # Same
git stash push -m "Description"   # With message

git stash -u                      # Include untracked
git stash --include-untracked
git stash -a                      # Include ignored too
git stash --all

git stash list                    # List stashes
# stash@{0}: WIP on main: abc1234 Last commit
# stash@{1}: On feature: def5678 Previous

git stash apply                   # Apply, keep in list
git stash apply stash@{1}         # Apply specific
git stash pop                     # Apply and remove
git stash pop stash@{1}           # Apply specific, remove

git stash drop                    # Drop most recent
git stash drop stash@{1}          # Drop specific
git stash clear                   # Drop all
```

### Advanced

```bash
git stash branch feature stash@{0}   # Create branch from stash
git stash push -m "msg" -- file.txt  # Stash specific files
git stash push -p                    # Interactive (select hunks)

git stash show                       # Stat summary
git stash show -p                    # Full diff
git stash show stash@{1}             # Specific stash
```

### Patterns

**Quick context switch:**
```bash
git stash
git checkout other-branch
# work
git checkout original-branch
git stash pop
```

**Test clean state:**
```bash
git stash
# run tests
git stash pop
```

**Partial commit:**
```bash
git stash -p                      # Stash what you don't want
git commit -m "Selected changes"
git stash pop
```

---

## Tags and Releases

### Tag Types

**Lightweight:** Simple pointer
```bash
git tag v1.0.0
git tag v1.0.0 abc1234
```

**Annotated:** Full object with metadata
```bash
git tag -a v1.0.0 -m "Release 1.0.0"
git tag -a v1.0.0 abc1234 -m "Message"
```

### Operations

```bash
git tag                           # List tags
git tag -l "v1.*"                 # Filter
git tag -n                        # Show messages

git show v1.0.0                   # Tag details

# Push
git push origin v1.0.0            # Single
git push origin --tags            # All tags
git push --follow-tags            # Commits + annotated tags

# Delete
git tag -d v1.0.0                 # Local
git push origin --delete v1.0.0   # Remote

# Checkout (detached HEAD)
git checkout v1.0.0

# Create branch from tag
git checkout -b hotfix-1.0.1 v1.0.0
```

### Signed Tags

```bash
git config --global user.signingkey YOUR_KEY_ID
git tag -s v1.0.0 -m "Signed release"
git tag -v v1.0.0                 # Verify
```

---

## Git Hooks

### Locations

- Client-side: `.git/hooks/` (not shared via clone)
- Server-side: On Git server

### Common Client Hooks

```
.git/hooks/
├── pre-commit           # Before commit message editor
├── prepare-commit-msg   # Modify default message
├── commit-msg           # Validate message
├── post-commit          # After commit
├── pre-push             # Before push
├── pre-rebase           # Before rebase
└── post-checkout        # After checkout
```

### Example: Pre-commit

```bash
#!/bin/bash
# .git/hooks/pre-commit

npm run lint
if [ $? -ne 0 ]; then
    echo "Lint failed"
    exit 1
fi

if git diff --cached | grep -E "console\.log|debugger"; then
    echo "Found debug statements"
    exit 1
fi
```

### Sharing Hooks

```bash
# Option 1: Hooks directory in repo
mkdir .githooks
git config core.hooksPath .githooks

# Option 2: Hook managers
# husky (Node.js), pre-commit (Python), lefthook (Go)
```
