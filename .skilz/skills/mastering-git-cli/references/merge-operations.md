# Merge Operations

Complete guide to merging, rebasing, cherry-picking, and conflict resolution.

## Table of Contents

1. [Merge Fundamentals](#merge-fundamentals)
2. [Merge Strategies](#merge-strategies)
3. [Strategy Options](#strategy-options)
4. [Fast-Forward vs True Merge](#fast-forward-vs-true-merge)
5. [Three-Way Merge](#three-way-merge)
6. [Conflict Resolution](#conflict-resolution)
7. [Rerere](#rerere-reuse-recorded-resolution)
8. [Cherry-Pick vs Merge vs Rebase](#cherry-pick-vs-merge-vs-rebase)
9. [Easy Buttons](#easy-buttons)

---

## Merge Fundamentals

### What Happens During a Merge

1. Git finds the **merge base** (common ancestor)
2. Git computes two diffs: base→ours and base→theirs
3. Git applies both sets of changes
4. Non-overlapping changes: automatic merge
5. Overlapping changes: conflict, manual resolution

```
       A---B---C  (main, ours)
      /
     O           (merge base)
      \
       D---E---F  (feature, theirs)
```

### Merge vs Rebase

**Merge preserves topology:**
```
       A---B---C---M  (main)
      /           /
     O---D---E---F    (feature merged)
```

**Rebase linearizes:**
```
     O---A---B---C---D'---E'---F'  (feature rebased onto main)
```

| Factor | Merge | Rebase |
|--------|-------|--------|
| History | Preserves actual order | Linear, "clean" |
| Shared branches | Safe | DANGEROUS |
| Debugging (bisect) | Accurate | May mislead |
| Conflicts | Resolve once | May resolve repeatedly |

**Golden rule:** Never rebase commits that have been pushed and shared.

---

## Merge Strategies

The `-s` flag selects the strategy. The `-X` flag sets strategy options.

### Strategy: `ort` (Default, Git 2.34+)

Modern default. Faster, better rename detection than `recursive`.

```bash
git merge feature                  # Uses ort
git merge -s ort feature           # Explicit
```

### Strategy: `recursive` (Legacy Default)

Previous default. Still works, `ort` preferred.

```bash
git merge -s recursive feature
```

### Strategy: `ours`

**IGNORES EVERYTHING** from the other branch. Records merge but keeps our tree.

```bash
git merge -s ours obsolete-branch
```

**Use cases:**
- Officially rejecting a branch
- Superseding with a rewrite
- Marking as "considered and rejected"

### Strategy: `octopus`

Merges 3+ branches simultaneously. Only for clean merges (no conflicts).

```bash
git merge feature-a feature-b feature-c
```

**Limitations:**
- Cannot resolve conflicts (aborts)
- Fall back to sequential merges if needed

### Strategy: `subtree`

Merges a subdirectory of another repository.

```bash
git remote add external https://github.com/org/lib.git
git fetch external
git merge -s subtree --allow-unrelated-histories external/main

# Later updates
git fetch external
git merge -s subtree external/main
```

### Strategy: `resolve`

Simple three-way merge. Rarely needed.

```bash
git merge -s resolve feature
```

---

## Strategy Options

The `-X` flag modifies how strategies handle specific situations.

### CRITICAL: `-X ours` vs `-s ours`

```bash
# -X ours: Prefer our changes ONLY IN CONFLICTS
git merge -X ours feature
# ↑ Still merges non-conflicting changes from feature

# -s ours: IGNORE EVERYTHING from other branch
git merge -s ours feature
# ↑ Keeps our tree exactly, just records merge
```

### Conflict Resolution Preferences

```bash
git merge -X ours feature      # Auto-resolve conflicts: prefer ours
git merge -X theirs feature    # Auto-resolve conflicts: prefer theirs
```

### Rename Detection

```bash
git merge -X rename-threshold=25 feature   # More aggressive (25% similarity)
git merge -X rename-threshold=75 feature   # More conservative (75% similarity)
git merge -X no-renormalize feature        # Skip file normalization
```

### Whitespace Handling

```bash
git merge -X ignore-space-change feature   # Ignore amount of whitespace
git merge -X ignore-all-space feature      # Ignore all whitespace
git merge -X ignore-space-at-eol feature   # Ignore trailing whitespace
```

### Diff Algorithm

```bash
git merge -X diff-algorithm=patience feature   # Better for moved blocks
git merge -X diff-algorithm=histogram feature  # Best general-purpose
git merge -X diff-algorithm=minimal feature    # Smallest diff (slow)
git merge -X diff-algorithm=myers feature      # Classic algorithm
```

---

## Fast-Forward vs True Merge

### Fast-Forward

When current branch is ancestor of target:

```bash
# Before
main: A---B
            \
feature:     C---D---E

# After fast-forward
main: A---B---C---D---E  (no merge commit)
```

```bash
git merge feature              # Fast-forwards if possible
git merge --ff-only feature    # Fail if can't fast-forward
```

### True Merge (No Fast-Forward)

Force merge commit:

```bash
git merge --no-ff feature
```

```
# Result
main: A---B-----------M
            \       /
feature:     C---D---E
```

**Use `--no-ff` when:**
- Preserving branch topology
- Feature branches should be visible
- Team policy requires merge commits

---

## Three-Way Merge

### Mechanics

```
   BASE  ----+---- MERGE
     |              |
     ↓              ↓
  (common       (result)
  ancestor)
     ↑              ↑
  OURS          THEIRS
```

1. Find merge base (common ancestor)
2. Diff base→ours (what we changed)
3. Diff base→theirs (what they changed)
4. Combine diffs, detect overlaps

### Conflict Markers

**Default:**
```
<<<<<<< HEAD
our version
=======
their version
>>>>>>> feature
```

**diff3 style** (shows base):
```
<<<<<<< HEAD
our version
||||||| merged common ancestor
original version
=======
their version
>>>>>>> feature
```

**zdiff3** (Git 2.35+, omits common parts):
```bash
git config --global merge.conflictStyle zdiff3
```

---

## Conflict Resolution

### Basic Workflow

```bash
git merge feature                          # CONFLICT

git status                                 # See conflicted files

# Option 1: Edit manually
vim conflicted-file.txt                    # Remove markers, fix code
git add conflicted-file.txt

# Option 2: Choose entire file
git checkout --ours conflicted-file.txt   # Keep our version
git checkout --theirs conflicted-file.txt # Take their version
git add conflicted-file.txt

# Option 3: Use merge tool
git mergetool

git commit                                 # Complete merge
```

### See All Versions

```bash
git show :1:file.txt    # Base (common ancestor)
git show :2:file.txt    # Ours (HEAD)
git show :3:file.txt    # Theirs (feature)
```

### Restore Conflict Markers

```bash
git checkout -m file.txt
```

### Abort Merge

```bash
git merge --abort
git reset --hard HEAD   # Alternative
```

---

## Rerere: Reuse Recorded Resolution

Remembers how you resolved conflicts and replays automatically.

### Enable

```bash
git config --global rerere.enabled true
```

### How It Works

1. First conflict: resolve manually
2. Git records resolution
3. Same conflict later: Git auto-applies

### Commands

```bash
git rerere status          # See recorded resolutions
git rerere diff            # Show what rerere would apply
git rerere forget <file>   # Forget specific resolution
rm -rf .git/rr-cache       # Clear all
```

---

## Cherry-Pick vs Merge vs Rebase

### Decision Tree

```
Need to move commits?
│
├─ All commits from branch → MERGE or REBASE
│  ├─ Shared branch → MERGE
│  └─ Local only → REBASE or MERGE
│
└─ Specific commits only → CHERRY-PICK
```

### Cherry-Pick

```bash
git cherry-pick abc1234                # Single commit
git cherry-pick abc1234 def5678        # Multiple
git cherry-pick abc1234..def5678       # Range (exclusive of first)
git cherry-pick abc1234^..def5678      # Range (inclusive)

git cherry-pick -n abc1234             # Stage only, don't commit
git cherry-pick -x abc1234             # Add "cherry picked from" note
git cherry-pick --abort                # Cancel
git cherry-pick --continue             # Continue after conflict
```

**Risks:**
- Creates duplicate commits (different SHAs)
- Confusion if later merging original branch
- Doesn't track relationship to original

### When to Use Each

| Scenario | Method |
|----------|--------|
| Integrate feature branch | Merge or Rebase |
| Hotfix from main to release | Cherry-pick |
| Backport fix to old version | Cherry-pick |
| Update feature with main | Rebase (if not shared) or Merge |
| Try one commit locally | Cherry-pick -n |

---

## Easy Buttons

### Integrate Feature into Main

```bash
git checkout main
git merge feature                # Fast-forward or merge commit

# Always create merge commit
git merge --no-ff feature
```

### Update Feature with Latest Main

```bash
# Safe for shared branches
git checkout feature
git merge main

# Only if branch not shared
git checkout feature
git rebase main
```

### Redo a Merge

```bash
# If not pushed
git reset --hard HEAD~1
git merge feature                # Try again

# If pushed
git revert -m 1 <merge-commit>
```

### Take Their Version Entirely

```bash
# Specific files
git checkout --theirs path/to/file
git add path/to/file

# All conflicts
git checkout --theirs .
git add .
git commit
```

### Take Our Version Entirely

```bash
git checkout --ours .
git add .
git commit
```

### Abort Wrong Merge

```bash
git merge --abort
```

### Fix Pushed Broken Merge

```bash
git revert -m 1 <merge-sha>
git push
```

### Merge Multiple Branches

```bash
git merge feature-a feature-b feature-c   # Octopus (no conflicts)
```

### Ignore Their Changes Entirely

```bash
git merge -s ours obsolete-branch         # Records merge, keeps our tree
```

### Prefer Their Changes in Conflicts Only

```bash
git merge -X theirs feature               # Merges all, prefers theirs in conflicts
```
