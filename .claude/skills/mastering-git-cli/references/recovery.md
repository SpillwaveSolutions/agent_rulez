# Recovery and Troubleshooting

Reflog, recovery procedures, common errors, and command reference.

## Table of Contents

1. [The Reflog](#the-reflog)
2. [Recovering Lost Work](#recovering-lost-work)
3. [Common Errors](#common-errors)
4. [Command Cheat Sheet](#command-cheat-sheet)
5. [Glossary](#glossary)

---

## The Reflog

The reflog records every change to HEAD and branch tips. Your safety net.

### View Reflog

```bash
git reflog                        # HEAD reflog
git reflog show main              # Specific branch

# Output:
# abc1234 HEAD@{0}: commit: Latest commit
# def5678 HEAD@{1}: checkout: moving from feature to main
# 789abcd HEAD@{2}: commit: Previous commit
```

### Recover Deleted Branch

```bash
git branch -D feature             # Oops!
git reflog                        # Find the commit
git checkout -b feature abc1234   # Recreate
```

### Recover from Bad Reset

```bash
git reset --hard HEAD~5           # Oops, too far!
git reflog                        # Find where you were
git reset --hard HEAD@{1}         # Go back
```

### Recover Lost Commits

```bash
git reflog
# Find the SHA
git cherry-pick abc1234
# Or create branch
git branch recovered abc1234
```

### Reflog Expiration

```bash
# Default: 30 days unreachable, 90 days reachable

git config gc.reflogExpireUnreachable "60 days"

# Manual cleanup
git reflog expire --expire=now --all
git gc --prune=now
```

---

## Recovering Lost Work

### Dangling Commits

```bash
git fsck --lost-found             # Find dangling
git show <sha>                    # Inspect
git cherry-pick <sha>             # Recover
git branch recovered <sha>
```

### Recovering Stashed Changes

```bash
# After accidental stash clear
git fsck --unreachable | grep commit
git show <sha>
```

### Recovering Deleted Files

```bash
# Find when deleted
git log --diff-filter=D --summary -- path/to/file

# Restore from before deletion
git checkout <sha>~1 -- path/to/file
```

---

## Common Errors

### "Detached HEAD"

Not an error, just a state.

```bash
git status                        # See where you are
git checkout -b new-branch        # Create branch here
git checkout main                 # Return to branch
```

### "Your branch is behind"

```bash
git pull
git pull --rebase
```

### "Refusing to merge unrelated histories"

```bash
git merge other-branch --allow-unrelated-histories
```

### "Cannot pull with rebase: You have unstaged changes"

```bash
# Option 1
git stash
git pull --rebase
git stash pop

# Option 2
git commit -am "WIP"
git pull --rebase
```

### "Not possible to fast-forward"

```bash
git pull --no-rebase              # Merge instead
git pull --rebase                 # Rebase instead
```

### "Permission denied (publickey)"

```bash
ssh -T git@github.com             # Test SSH
ssh-add ~/.ssh/id_rsa             # Add key
git config --global credential.helper cache
```

### "fatal: refusing to merge unrelated histories"

```bash
git pull origin main --allow-unrelated-histories
```

### "error: pathspec 'file' did not match any file(s)"

File doesn't exist or isn't tracked:
```bash
git status
git add <file>
```

### "error: Your local changes would be overwritten"

```bash
git stash
git checkout <branch>
git stash pop
```

---

## Command Cheat Sheet

### Daily Commands

```bash
# Status and diff
git status          git status -sb
git diff            git diff --staged

# Stage and commit
git add <file>      git add -A
git add -p          git commit -m "msg"
git commit --amend

# Branch
git branch          git branch <n>
git checkout <br>   git switch <br>
git checkout -b <n> git switch -c <n>
git branch -d <n>   git branch -D <n>

# Remote
git fetch           git pull
git push            git push -u origin <br>
```

### History

```bash
git log --oneline --graph
git log -p -- <file>
git show <sha>
git blame <file>
git reflog
git diff <a>..<b>
```

### Merge and Rebase

```bash
git merge <branch>
git merge --no-ff <branch>
git merge --abort

git rebase <branch>
git rebase -i HEAD~n
git rebase --abort
git rebase --continue

git cherry-pick <sha>
```

### Undo

```bash
git restore <file>
git restore --staged <file>
git reset HEAD~1
git reset --hard HEAD~1
git revert <sha>
```

### Worktrees

```bash
git worktree add <path> <branch>
git worktree list
git worktree remove <path>
git worktree prune
```

### Submodules

```bash
git submodule add <url> <path>
git submodule update --init --recursive
git submodule update --remote
git submodule foreach '<cmd>'
```

### Stash

```bash
git stash           git stash pop
git stash list      git stash apply
git stash push -m   git stash drop
```

### Tags

```bash
git tag             git tag -a v1.0 -m "msg"
git push --tags     git tag -d <tag>
```

---

## Glossary

| Term | Definition |
|------|------------|
| **blob** | Git object containing file contents |
| **branch** | Movable pointer to a commit |
| **checkout** | Switch branches or restore files |
| **commit** | Snapshot of repository at a point in time |
| **DAG** | Directed Acyclic Graph, structure of Git history |
| **detached HEAD** | HEAD points to commit, not branch |
| **fast-forward** | Merge where target is direct descendant |
| **fetch** | Download objects/refs from remote without merging |
| **HEAD** | Pointer to current commit/branch |
| **index** | Staging area; what goes into next commit |
| **merge base** | Common ancestor for three-way merge |
| **object** | Blob, tree, commit, or tag in Git database |
| **origin** | Conventional name for primary remote |
| **pull** | Fetch + merge (or rebase) |
| **push** | Upload local commits to remote |
| **rebase** | Replay commits onto different base |
| **ref** | Human-readable name pointing to SHA |
| **reflog** | Log of all ref changes |
| **remote** | Repository hosted elsewhere |
| **remote-tracking branch** | Local ref to remote branch state |
| **rerere** | Reuse Recorded Resolution for conflicts |
| **SHA** | Hash identifying Git objects |
| **stash** | Temporary storage for uncommitted changes |
| **submodule** | Repository embedded in another |
| **tag** | Permanent pointer to specific commit |
| **tree** | Git object representing directory structure |
| **upstream** | Remote branch that local tracks |
| **worktree** | Additional working directory for same repo |
