# Release Process

## Overview

CCH releases follow a structured process ensuring quality and traceability:

1. **Development** on `develop` branch (Fast CI)
2. **Validation** via PR to `main` (Full IQ/OQ/PQ)
3. **Release** tag from `main`
4. **Deployment** via GitHub Releases

---

## Pre-Release Checklist

Before creating a release PR:

- [ ] All planned features merged to `develop`
- [ ] All tests passing on `develop`
- [ ] Version updated in `cch_cli/Cargo.toml`
- [ ] CHANGELOG updated
- [ ] Documentation updated

---

## Release Workflow

### Step 1: Prepare Release

```bash
# Ensure develop is clean
git checkout develop
git pull origin develop

# Verify all tests pass
cd cch_cli && cargo test
cd ..

# Update version if needed
# Edit cch_cli/Cargo.toml
```

### Step 2: Create Release PR

```bash
# Create PR from develop to main
gh pr create \
  --base main \
  --head develop \
  --title "Release: v1.x.x" \
  --body "## Release v1.x.x

### Changes
- Feature A
- Feature B
- Bug fix C

### Validation
Full IQ/OQ/PQ validation will run automatically."
```

### Step 3: Wait for Validation

The PR triggers Full Validation (~10-15 minutes):

| Phase | What Runs |
|-------|-----------|
| IQ | 4-platform installation tests |
| OQ | All operational test suites |
| PQ | Performance and memory tests |
| Report | Validation summary generated |

**All phases must pass before merge.**

### Step 4: Review Evidence

Download validation artifacts from the GitHub Actions run:

1. Go to Actions tab
2. Find the validation workflow run
3. Download artifacts:
   - `iq-evidence-*` (per platform)
   - `oq-evidence`
   - `pq-evidence`
   - `validation-report`

### Step 5: Merge and Tag

```bash
# After PR approval and validation passes
# Merge via GitHub UI

# Pull the merged main
git checkout main
git pull origin main

# Create annotated tag
git tag -a v1.x.x -m "Release v1.x.x

Changes:
- Feature A
- Feature B
- Bug fix C"

# Push tag
git push origin v1.x.x
```

### Step 6: Create GitHub Release

```bash
gh release create v1.x.x \
  --title "CCH v1.x.x" \
  --notes "## What's New

### Features
- Feature A
- Feature B

### Bug Fixes
- Bug fix C

### Validation
- IQ: Passed on macOS (ARM64, Intel), Linux, Windows
- OQ: All test suites passed
- PQ: Performance requirements met"
```

---

## Hotfix Release

For critical fixes that can't wait for normal release cycle:

### Step 1: Create Hotfix

```bash
git checkout main
git pull origin main
git checkout -b hotfix/critical-issue
```

### Step 2: Implement Fix

```bash
# Minimal changes only
git add .
git commit -m "fix: critical security issue"
```

### Step 3: Create PR to Main

```bash
git push -u origin hotfix/critical-issue
gh pr create \
  --base main \
  --title "hotfix: critical security issue" \
  --body "## Hotfix

### Issue
Description of the critical issue.

### Fix
Description of the fix.

### Testing
- [ ] Verified fix locally
- [ ] Full validation will run"
```

### Step 4: After Merge, Backport

```bash
# After hotfix merged to main
git checkout develop
git pull origin develop
git cherry-pick <hotfix-commit-hash>
git push origin develop
```

---

## Version Numbering

CCH follows [Semantic Versioning](https://semver.org/):

| Version | When to Increment |
|---------|-------------------|
| MAJOR (1.x.x) | Breaking changes |
| MINOR (x.1.x) | New features, backward compatible |
| PATCH (x.x.1) | Bug fixes, backward compatible |

---

## Evidence Retention

Validation evidence is retained per release:

| Release Type | Retention |
|--------------|-----------|
| Major | Indefinite |
| Minor | 2 years minimum |
| Patch | 1 year minimum |

Store evidence in `docs/validation/sign-off/v{version}/`.

---

## Rollback Procedure

If a release has critical issues:

```bash
# Identify last good release
git log --oneline --tags

# Create hotfix from last good release
git checkout v1.x.x  # last good version
git checkout -b hotfix/rollback-issue

# Cherry-pick fix or revert problematic commit
git revert <bad-commit>

# Follow hotfix process above
```

---

## Automation

### Taskfile Commands

```bash
# Collect all validation evidence
task collect-all

# Generate validation report
task validation-report
```

### GitHub Actions

- **Release tag push** triggers release workflow
- **Binaries** automatically built and attached to release
- **Evidence** available as workflow artifacts
