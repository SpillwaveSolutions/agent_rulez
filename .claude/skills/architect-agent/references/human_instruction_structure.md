# Human Instruction Structure Template

## Purpose

Human instructions (`human/human-*.md`) must be **executable documentation** - detailed enough for a human to complete the task manually without code agents.

This is NOT a summary. It's a step-by-step guide that any team member can follow.

## Why Humans Need More Than Summaries

| What Code Agents Have | What Humans Need |
|----------------------|------------------|
| Tool access (grep, read, edit) | Copy-pasteable commands |
| Pattern recognition | Clear explanations of *why* |
| Error recovery | Troubleshooting tables |
| Implicit knowledge | Prerequisites spelled out |
| Infinite patience | Time estimates |

## Key Difference: Summaries vs. Executable Docs

| Aspect | Summary (BAD) | Executable (GOOD) |
|--------|---------------|-------------------|
| Commands | "Create worktrees" | `git worktree add .worktrees/feature -b feature-branch main` |
| Verification | "Check it worked" | Show expected output with `git worktree list` |
| Errors | Not mentioned | Table of common errors + solutions |
| Context | Assumed | Prerequisites clearly listed |

## Human Instruction Template

```markdown
# [Task Name] - Manual Execution Guide

**Date:** YYYY-MM-DD
**Estimated Time:** X minutes
**Difficulty:** Easy | Medium | Advanced
**Corresponding Code Agent Instructions:** `instructions/instruct-TIMESTAMP-description.md`

---

## Prerequisites

Before starting, ensure:

- [ ] [Tool/service] is running
- [ ] You have access to [resource]
- [ ] Environment variable `X` is set

### Environment Setup

```bash
# Set required environment variables
export DATABASE_URL="postgresql://user:pass@localhost:5432/db"
export API_KEY="${API_KEY:-your-key-here}"

# Verify setup
echo "Database: $DATABASE_URL"
```

---

## Overview

### What This Accomplishes
[2-3 sentences explaining the goal and why it matters]

### What You'll Do
1. [High-level step 1]
2. [High-level step 2]
3. [High-level step 3]

---

## Step 1: [Descriptive Step Name]

### Why This Step
[1-2 sentences explaining why this step is necessary - humans benefit from understanding purpose]

### Commands

```bash
cd /path/to/directory
command-to-run --with --options
```

### Expected Output

```
[Show what success looks like]
Processing...
Complete: 100 items processed
```

### Verification

```bash
# Confirm the step succeeded
verification-command
# Expected: [what you should see]
```

### If This Fails

| Error | Cause | Solution |
|-------|-------|----------|
| `connection refused` | Service not running | Start the service: `docker compose up -d` |
| `permission denied` | Wrong user/permissions | Check ownership: `ls -la filename` |
| `command not found` | Missing dependency | Install: `brew install package` |

---

## Step 2: [Next Step Name]

### Why This Step
[Purpose explanation]

### Commands
[...]

### Expected Output
[...]

### Verification
[...]

### If This Fails
[...]

---

## Final Verification

After completing all steps, verify the entire task succeeded:

```bash
# Comprehensive verification command
final-check-command
```

### Expected Final State

| Metric | Expected Value |
|--------|---------------|
| [Metric 1] | [Value] |
| [Metric 2] | [Value] |

---

## Rollback Procedure

If something goes wrong and you need to undo changes:

```bash
# Restore to previous state
rollback-command

# Or reset completely
reset-command
```

---

## Summary

| Step | What You Did | Time |
|------|--------------|------|
| 1 | [Brief description] | X min |
| 2 | [Brief description] | X min |
| 3 | [Brief description] | X min |
| **Total** | | **Y min** |

---

## Next Steps

After completing this task:
- [ ] Notify [person/team] of completion
- [ ] Run [follow-up task] if applicable
- [ ] Update [documentation] with any issues encountered
```

---

## Examples

### BAD: Vague Summary

```markdown
## Execution Flow
1. Create worktrees for each task
2. Spawn background agents
3. Monitor status
4. Merge branches
```

**Problems:**
- What commands create worktrees?
- What does "spawn" mean to a human?
- How do you monitor status?
- No error handling if something fails

### GOOD: Executable Instructions

```markdown
## Step 1: Create Git Worktrees

### Why This Step
Worktrees let you work on multiple branches simultaneously without switching or stashing. Each worktree is an isolated directory with its own branch.

### Commands

```bash
cd /Users/you/projects/myproject

# Create worktree for feature A
git worktree add .worktrees/feature-a -b feature-a main

# Create worktree for feature B
git worktree add .worktrees/feature-b -b feature-b main

# Copy environment config to each
cp .env .worktrees/feature-a/
cp .env .worktrees/feature-b/
```

### Expected Output

```
Preparing worktree (new branch 'feature-a')
HEAD is now at abc1234 Previous commit message
Preparing worktree (new branch 'feature-b')
HEAD is now at abc1234 Previous commit message
```

### Verification

```bash
git worktree list
```

Expected:
```
/Users/you/projects/myproject                    abc1234 [main]
/Users/you/projects/myproject/.worktrees/feature-a  abc1234 [feature-a]
/Users/you/projects/myproject/.worktrees/feature-b  abc1234 [feature-b]
```

### If This Fails

| Error | Cause | Solution |
|-------|-------|----------|
| `fatal: not a git repository` | Wrong directory | `cd` to project root first |
| `'feature-a' already exists` | Branch exists | Use different name or `git worktree remove` first |
| `is already checked out` | Worktree exists | `git worktree list` to check existing worktrees |
```

---

## Checklist for Human Instructions

Before finalizing human instructions, verify:

- [ ] **Prerequisites** - All requirements listed with verification commands
- [ ] **Every step has commands** - Copy-pasteable, not descriptions
- [ ] **Expected output shown** - User knows what success looks like
- [ ] **Verification provided** - How to confirm each step worked
- [ ] **Troubleshooting table** - At least 2-3 common errors per step
- [ ] **Why explanations** - Purpose of each step explained
- [ ] **Time estimates** - Realistic expectations set
- [ ] **Rollback procedure** - How to undo if needed

---

## Loading This Template

This template should be loaded when the architect agent:
1. Is creating instructions for any task
2. Needs to create a human-executable version alongside code agent instructions

**Load alongside:** `references/instruction_structure.md`, `references/file_naming.md`
