# Parallel Development with Git Worktrees - Manual Execution Guide

**Date:** 2025-12-31
**Estimated Time:** 10 minutes
**Difficulty:** Easy
**Corresponding Code Agent Instructions:** `instructions/instruct-20251231_160000-parallel-worktrees.md`

---

## Prerequisites

Before starting, ensure:

- [ ] Git 2.15+ installed (worktrees were improved in this version)
- [ ] You're in a git repository
- [ ] All changes are committed or stashed (clean working directory)
- [ ] You know which branches you want to work on

### Verify Prerequisites

```bash
# Check git version
git --version
# Expected: git version 2.15.0 or higher

# Check you're in a git repo
git rev-parse --is-inside-work-tree
# Expected: true

# Check for clean working directory
git status --porcelain
# Expected: (empty output means clean)
```

---

## Overview

### What This Accomplishes
Git worktrees let you have multiple branches checked out simultaneously in different directories. This is useful when you need to:
- Work on two features in parallel
- Compare implementations side-by-side
- Keep a reference copy while making changes
- Run tests on one branch while coding on another

### What You'll Do
1. Create worktree directories for each parallel task
2. Set up environment in each worktree
3. Verify everything is working
4. (Later) Clean up when done

---

## Step 1: Identify Your Tasks

### Why This Step
Before creating worktrees, you need to know what branches you'll work on. Each worktree needs a unique branch name.

### Plan Your Worktrees

For this example, we'll create two parallel tasks:
- **Task A:** JSON import feature → branch `feature/json-import`
- **Task B:** Embeddings system → branch `feature/embeddings`

```bash
# Set variables for clarity (adjust for your tasks)
TASK_A_BRANCH="feature/json-import"
TASK_B_BRANCH="feature/embeddings"
PROJECT_ROOT=$(pwd)

echo "Creating worktrees from: $PROJECT_ROOT"
echo "Task A branch: $TASK_A_BRANCH"
echo "Task B branch: $TASK_B_BRANCH"
```

### Expected Output

```
Creating worktrees from: /Users/you/projects/myapp
Task A branch: feature/json-import
Task B branch: feature/embeddings
```

---

## Step 2: Create Worktree Directories

### Why This Step
A worktree is a linked working directory that shares the same `.git` data but has its own branch checked out. We create them in a `.worktrees/` subdirectory to keep them organized.

### Commands

```bash
# Create directory for worktrees (if it doesn't exist)
mkdir -p .worktrees

# Create worktree for Task A
git worktree add .worktrees/task-a -b $TASK_A_BRANCH main

# Create worktree for Task B
git worktree add .worktrees/task-b -b $TASK_B_BRANCH main
```

### Expected Output

```
Preparing worktree (new branch 'feature/json-import')
HEAD is now at abc1234 Latest commit on main
Preparing worktree (new branch 'feature/embeddings')
HEAD is now at abc1234 Latest commit on main
```

### Verification

```bash
# List all worktrees
git worktree list
```

Expected:
```
/Users/you/projects/myapp                     abc1234 [main]
/Users/you/projects/myapp/.worktrees/task-a   abc1234 [feature/json-import]
/Users/you/projects/myapp/.worktrees/task-b   abc1234 [feature/embeddings]
```

### If This Fails

| Error | Cause | Solution |
|-------|-------|----------|
| `fatal: 'feature/json-import' is already checked out` | Branch already in a worktree | Use `git worktree list` to find where, then remove it |
| `fatal: invalid reference: main` | Branch 'main' doesn't exist | Use your default branch: `git symbolic-ref --short HEAD` |
| `fatal: 'feature/json-import' already exists` | Branch exists from previous work | Either use it (`git worktree add .worktrees/task-a feature/json-import`) or delete it first |

---

## Step 3: Copy Environment Files

### Why This Step
Worktrees share git data but NOT working files. Your `.env`, config files, and `node_modules` won't be present in new worktrees. You need to copy or recreate them.

### Commands

```bash
# Copy environment files to each worktree
cp .env .worktrees/task-a/ 2>/dev/null || echo "No .env to copy"
cp .env .worktrees/task-b/ 2>/dev/null || echo "No .env to copy"

# If using Node.js, install dependencies in each
# (Run these if applicable to your project)
# cd .worktrees/task-a && npm install && cd -
# cd .worktrees/task-b && npm install && cd -
```

### Expected Output

```
(no output if successful, or "No .env to copy" if you don't have one)
```

### Verification

```bash
# Confirm env files were copied
ls -la .worktrees/task-a/.env .worktrees/task-b/.env 2>/dev/null || echo "No .env files needed"
```

### If This Fails

| Error | Cause | Solution |
|-------|-------|----------|
| `No such file or directory` | Source file doesn't exist | Create `.env` first or skip if not needed |
| Permission issues | Directory not writable | Check `ls -la .worktrees/` |

---

## Step 4: Open Worktrees in Separate Terminals

### Why This Step
The power of worktrees is working in parallel. Open each worktree in its own terminal (or IDE window) so you can switch between tasks without git operations.

### Commands

```bash
# Option A: Open in new terminal tabs (macOS)
open -a Terminal .worktrees/task-a
open -a Terminal .worktrees/task-b

# Option B: Print paths to open manually
echo "Task A: cd $PROJECT_ROOT/.worktrees/task-a"
echo "Task B: cd $PROJECT_ROOT/.worktrees/task-b"
```

### Expected Output

```
Task A: cd /Users/you/projects/myapp/.worktrees/task-a
Task B: cd /Users/you/projects/myapp/.worktrees/task-b
```

### Verification

In each terminal:
```bash
# Confirm you're in the right worktree
pwd
git branch --show-current
```

Expected for Task A:
```
/Users/you/projects/myapp/.worktrees/task-a
feature/json-import
```

---

## Final Verification

Confirm your worktree setup is complete:

```bash
# From the main project directory
git worktree list --porcelain | grep -E "^worktree|^branch"
```

### Expected Final State

| Worktree | Branch | Status |
|----------|--------|--------|
| Main project | main | Active |
| .worktrees/task-a | feature/json-import | Ready |
| .worktrees/task-b | feature/embeddings | Ready |

---

## Working in Worktrees

Now you can work in each worktree independently:

```bash
# In Task A terminal
cd .worktrees/task-a
git add . && git commit -m "feat: add JSON import"
git push -u origin feature/json-import

# In Task B terminal (parallel, no conflicts)
cd .worktrees/task-b
git add . && git commit -m "feat: add embeddings"
git push -u origin feature/embeddings
```

**Key benefit:** No branch switching, no stashing, no merge conflicts between your parallel work.

---

## Cleanup (When Done)

When you're finished with a task and have merged the branch:

```bash
# Remove a worktree (from main project directory)
git worktree remove .worktrees/task-a

# Delete the branch if merged
git branch -d feature/json-import

# List remaining worktrees
git worktree list
```

### If Cleanup Fails

| Error | Cause | Solution |
|-------|-------|----------|
| `contains modified or untracked files` | Uncommitted changes | Commit or `git worktree remove --force` |
| `is not a working tree` | Already removed | Check with `git worktree list` |

---

## Summary

| Step | What You Did | Time |
|------|--------------|------|
| 1 | Identified tasks and branches | 1 min |
| 2 | Created worktree directories | 2 min |
| 3 | Copied environment files | 1 min |
| 4 | Opened in separate terminals | 1 min |
| **Total** | | **5 min** |

---

## Next Steps

After setup is complete:
- [ ] Start working on Task A in its terminal
- [ ] Start working on Task B in its terminal
- [ ] Create PRs for each when ready
- [ ] Clean up worktrees after merging

---

## Tips

1. **Add `.worktrees/` to `.gitignore`** - Prevents accidentally committing worktree directories
2. **Use descriptive worktree names** - `task-a` is okay, but `json-import` is clearer
3. **Don't nest worktrees** - Keep them flat in `.worktrees/`
4. **Run tests in each** - Worktrees are isolated, so tests won't interfere
