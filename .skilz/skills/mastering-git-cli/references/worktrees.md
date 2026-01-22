# Git Worktrees

Multiple working directories for parallel development and agent workflows.

## Table of Contents

1. [Understanding Worktrees](#understanding-worktrees)
2. [Worktree Commands](#worktree-commands)
3. [Workflow Patterns](#workflow-patterns)
4. [Multi-Agent Orchestration](#multi-agent-orchestration)
5. [Troubleshooting](#troubleshooting)

---

## Understanding Worktrees

### What Are Worktrees?

A worktree is an additional working directory linked to your repository. Unlike branches (which are just pointers), worktrees give you multiple physical checkouts of different branches simultaneously.

**Key characteristics:**
- Each worktree has its own working directory and index
- All worktrees share the same `.git` directory (object store, refs, config)
- A branch can only be checked out in ONE worktree at a time
- The main worktree is where you ran `git init` or `git clone`

### When to Use Worktrees

| Use Case | Worktrees | Branches |
|----------|-----------|----------|
| Quick context switch | ✓ Best | Requires stash/commit |
| Parallel compilation/testing | ✓ Best | Not possible |
| Comparing implementations | ✓ Best | Requires switching |
| Long-running tasks in background | ✓ Best | Blocks main work |
| Code review while developing | ✓ Best | Interrupts flow |
| Simple feature work | Overhead | ✓ Best |
| Agent/AI parallel workflows | ✓ Essential | Not feasible |

### Worktree vs Stash vs Multiple Clones

| Approach | Disk Space | Shared Objects | Speed | Independence |
|----------|------------|----------------|-------|--------------|
| Worktrees | Low | Yes | Fast | Partial |
| Stash | None | Yes | Fast | None |
| Multiple Clones | High | No | Slow | Full |

---

## Worktree Commands

### Create Worktrees

```bash
# With existing branch
git worktree add ../hotfix hotfix-branch

# With new branch
git worktree add -b feature-x ../feature-x main

# Detached at specific commit
git worktree add --detach ../experiment abc1234

# Tracking remote branch
git worktree add ../upstream-main origin/main
```

### List Worktrees

```bash
git worktree list                  # Human-readable
git worktree list --porcelain      # Machine-readable
```

**Output example:**
```
/path/to/main           abc1234 [main]
/path/to/feature        def5678 [feature-x]
/path/to/hotfix         789abc0 [hotfix-123]
```

### Move Worktrees

```bash
git worktree move ../feature ../features/feature-x
```

### Remove Worktrees

```bash
git worktree remove ../feature           # Safe (checks for changes)
git worktree remove --force ../feature   # Force (discards changes)
```

### Prune Stale Metadata

```bash
git worktree prune
```

### Lock/Unlock Worktrees

Prevent accidental removal:

```bash
git worktree lock ../feature --reason "On external drive"
git worktree unlock ../feature
```

### Repair Worktree References

```bash
git worktree repair                      # Fix all
git worktree repair /path/to/worktree    # Fix specific
```

---

## Workflow Patterns

### Pattern 1: Interrupt-Driven Development

Drop everything for a hotfix without losing work:

```bash
# You're mid-feature, urgent bug reported
git worktree add -b hotfix-critical ../hotfix main

cd ../hotfix
# ... fix the bug ...
git add -A && git commit -m "fix: critical bug"
git push origin hotfix-critical

cd ../main
# All your uncommitted changes are still here!
```

### Pattern 2: Parallel Review

Review a PR while continuing your work:

```bash
git fetch origin pull/123/head:pr-123
git worktree add ../review-pr-123 pr-123

# In another terminal
cd ../review-pr-123
# Run tests, inspect code

# When done
git worktree remove ../review-pr-123
git branch -D pr-123
```

### Pattern 3: Parallel Builds

Build multiple versions simultaneously:

```bash
git worktree add ../v2-test v2.0
git worktree add ../v3-test v3.0
git worktree add ../main-test main

# Run in parallel
(cd ../v2-test && make test) &
(cd ../v3-test && make test) &
(cd ../main-test && make test) &
wait
```

### Pattern 4: A/B Implementation

Compare two approaches:

```bash
git worktree add -b approach-a ../approach-a main
git worktree add -b approach-b ../approach-b main

cd ../approach-a && # implement A
cd ../approach-b && # implement B

# Compare, benchmark, choose winner
```

---

## Multi-Agent Orchestration

For parallel AI agent development with isolated workspaces.

### Architecture: Hub and Spoke

```
repository/
├── main/                    # Hub: coordination
│   └── .git/               # Shared Git database
├── agent-1/                # Spoke: Agent 1 workspace
├── agent-2/                # Spoke: Agent 2 workspace
├── agent-3/                # Spoke: Agent 3 workspace
└── integration/            # Merge testing
```

### Setup Script

Use `scripts/setup-agent-worktrees.sh`:

```bash
#!/bin/bash
REPO_ROOT=$(pwd)
NUM_AGENTS=${1:-3}
BASE_BRANCH=${2:-main}

for i in $(seq 1 $NUM_AGENTS); do
    BRANCH_NAME="agent-$i-work"
    WORKTREE_PATH="../agent-$i"
    git worktree add -b "$BRANCH_NAME" "$WORKTREE_PATH" "$BASE_BRANCH"
    echo "Created: $WORKTREE_PATH on $BRANCH_NAME"
done

git worktree add -b "integration" "../integration" "$BASE_BRANCH"
git worktree list
```

### Coordination Patterns

**Pattern 1: Independent Tasks**

Each agent works on separate files:

```bash
# Agent 1: auth module
cd ../agent-1
git add -A && git commit -m "feat(auth): implement OAuth"

# Agent 2: database module
cd ../agent-2
git add -A && git commit -m "feat(db): add pooling"

# Integration
cd ../integration
git merge agent-1-work --no-edit
git merge agent-2-work --no-edit
```

**Pattern 2: Sequential Dependencies**

Agent 2 depends on Agent 1:

```bash
# Agent 1
cd ../agent-1
git add -A && git commit -m "feat: define interfaces"
git push origin agent-1-work

# Agent 2
cd ../agent-2
git fetch origin agent-1-work
git merge origin/agent-1-work --no-edit
# ... implement interfaces ...
```

**Pattern 3: Competitive/Exploratory**

Multiple agents try different solutions:

```bash
# Agent 1: Approach A
cd ../agent-1 && # implement solution A
git commit -m "experiment: solution A"

# Agent 2: Approach B
cd ../agent-2 && # implement solution B
git commit -m "experiment: solution B"

# Evaluate and choose winner
cd ../main
```

### Sync Commands

```bash
# Sync agent with latest main
cd ../agent-1
git fetch origin main
git rebase origin/main  # or merge

# Push agent work
git push origin agent-1-work

# Pull integration results
git fetch origin integration
git merge origin/integration
```

### Cleanup Script

Use `scripts/cleanup-agent-worktrees.sh`:

```bash
#!/bin/bash
for wt in $(git worktree list --porcelain | grep "^worktree" | grep "agent-" | cut -d' ' -f2); do
    git worktree remove --force "$wt"
done
git worktree remove --force ../integration 2>/dev/null
git worktree prune

if [[ "$1" == "--delete-branches" ]]; then
    for branch in $(git branch | grep "agent-.*-work"); do
        git branch -D "$branch"
    done
    git branch -D integration 2>/dev/null
fi
```

### Best Practices

1. **Clear naming:** `agent-<id>-<task>` or `agent-<id>-work`
2. **Frequent commits:** Better merge granularity
3. **Atomic tasks:** Minimize file overlap
4. **Integration worktree:** Dedicated merge testing
5. **Lock critical worktrees:** Prevent accidental removal
6. **Communication via commits:** Use messages to communicate state
7. **Regular sync points:** Periodically sync with integration

---

## Troubleshooting

### Branch Already Checked Out

```
fatal: 'feature-x' is already checked out at '/path/to/other'
```

**Solution:** Use different branch or create copy:
```bash
git worktree add -b feature-x-copy ../feature-copy feature-x
```

### Stale References After Moving

```bash
git worktree repair
```

### Can't Delete Branch Checked Out Elsewhere

```bash
git worktree remove ../feature  # First remove worktree
git branch -d feature-x         # Then delete branch
```

### Submodules in Worktrees

After creating worktree, initialize submodules:
```bash
cd ../new-worktree
git submodule update --init --recursive
```

### Worktree Limits

- No hard limit on number
- Each requires disk space for working directory
- Too many can slow `git worktree list`
- Some GUIs don't handle multiple worktrees well
