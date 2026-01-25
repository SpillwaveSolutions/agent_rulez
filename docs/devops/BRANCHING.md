# Branching Strategy

## Overview

CCH uses a two-branch model optimized for rapid development with production stability:

```
main (protected)          <- Production-ready, fully validated
  ^
  |
develop (default)         <- Integration branch, fast CI
  ^
  |
feature/* | fix/*         <- Short-lived working branches
```

## Branch Descriptions

### `main` - Production Branch
- **Purpose:** Production-ready code only
- **Protection:** Full IQ/OQ/PQ validation required
- **Who merges:** Via PR from `develop` after full validation
- **Direct commits:** NEVER allowed

### `develop` - Integration Branch
- **Purpose:** Integration of completed features
- **Protection:** Fast CI required
- **Default branch:** Yes (clone targets this)
- **Who merges:** Via PR from feature branches
- **Direct commits:** NEVER allowed

### `feature/*` - Feature Branches
- **Purpose:** Active development work
- **Naming:** `feature/<descriptive-name>`
- **Created from:** `develop`
- **Merged to:** `develop`
- **Lifetime:** Short-lived (days, not weeks)

### `fix/*` - Bug Fix Branches
- **Purpose:** Bug fixes for develop
- **Naming:** `fix/<issue-or-description>`
- **Created from:** `develop`
- **Merged to:** `develop`

### `hotfix/*` - Emergency Fixes
- **Purpose:** Critical fixes that must go directly to production
- **Naming:** `hotfix/<issue>`
- **Created from:** `main`
- **Merged to:** `main`, then backported to `develop`
- **Requires:** Full validation before merge

### `release/*` - Release Candidates
- **Purpose:** Preparing a release
- **Naming:** `release/v<version>`
- **Created from:** `develop`
- **Merged to:** `main` and `develop`

---

## Workflows

### Daily Development Workflow

```bash
# 1. Start from develop
git checkout develop
git pull origin develop

# 2. Create feature branch
git checkout -b feature/my-new-feature

# 3. Make changes, commit frequently
git add .
git commit -m "feat: add new capability"

# 4. Run pre-commit checks
cd cch_cli && cargo fmt && cargo clippy --all-targets --all-features -- -D warnings && cargo test

# 5. Push and create PR
git push -u origin feature/my-new-feature
gh pr create --base develop --title "feat: add new capability"

# 6. After PR approval and merge, clean up
git checkout develop
git pull origin develop
git branch -d feature/my-new-feature
```

### Release Workflow

```bash
# 1. Ensure develop is stable
git checkout develop
git pull origin develop

# 2. Create PR to main
gh pr create --base main --head develop --title "Release: merge develop to main"

# 3. Wait for full validation (~10-15 min)
# - IQ runs on 4 platforms
# - OQ runs all test suites
# - PQ runs benchmarks
# - Evidence is collected

# 4. After validation passes, merge PR

# 5. Tag the release
git checkout main
git pull origin main
git tag -a v1.x.x -m "Release v1.x.x"
git push origin v1.x.x
```

### Hotfix Workflow

```bash
# 1. Create hotfix from main
git checkout main
git pull origin main
git checkout -b hotfix/critical-issue

# 2. Implement minimal fix
git add .
git commit -m "fix: critical security issue"

# 3. Create PR to main (triggers full validation)
git push -u origin hotfix/critical-issue
gh pr create --base main --title "hotfix: critical security issue"

# 4. After merge to main, backport to develop
git checkout develop
git pull origin develop
git cherry-pick <commit-hash>
git push origin develop
```

---

## CI Integration

| Branch Target | CI Workflow | Duration | Blocking |
|---------------|-------------|----------|----------|
| PR to `develop` | Fast CI | ~2-3 min | Yes |
| PR to `main` | Full Validation | ~10-15 min | Yes |
| Push to `feature/*` | Fast CI | ~2-3 min | No |

See [CI_TIERS.md](CI_TIERS.md) for detailed CI configuration.

---

## Best Practices

### Do
- Keep feature branches short-lived (< 1 week)
- Rebase feature branches on develop before PR
- Write descriptive PR titles following conventional commits
- Delete branches after merge

### Don't
- Commit directly to `main` or `develop`
- Let feature branches diverge significantly
- Merge without CI passing
- Force push to shared branches

---

## Quick Reference

| Task | Command |
|------|---------|
| Start new feature | `git checkout develop && git pull && git checkout -b feature/name` |
| Create PR to develop | `gh pr create --base develop` |
| Create PR to main | `gh pr create --base main --head develop` |
| Delete local branch | `git branch -d feature/name` |
| Delete remote branch | `git push origin --delete feature/name` |
