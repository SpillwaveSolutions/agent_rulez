# CCH Hotfix Workflow

## When to Use

Use a hotfix workflow when:

- Critical bug found in production release
- Security vulnerability discovered
- Urgent patch needed without including unreleased features

## Hotfix vs Regular Release

| Aspect | Regular Release | Hotfix |
|--------|----------------|--------|
| Branch from | `main` | Existing tag (e.g., `v1.0.0`) |
| Branch name | `release/vX.Y.Z` | `hotfix/vX.Y.Z` |
| Version bump | Any (major/minor/patch) | Patch only |
| Scope | Full feature set | Minimal fix |

## Hotfix Diagram

```
                    main branch
                         │
    v1.0.0 ──────────────┼──────────────────────── v1.1.0 (future)
       │                 │
       │                 │
       ▼                 │
  ┌─────────┐            │
  │ Hotfix  │            │
  │ Branch  │            │
  └────┬────┘            │
       │                 │
       ▼                 │
  hotfix/v1.0.1          │
       │                 │
       ├── Fix bug       │
       ├── Update version│
       ├── Update changelog
       │                 │
       ▼                 │
   Create PR ────────────┤
       │                 │
       ▼                 │
   Merge to main ────────┤
       │                 │
       ▼                 │
   git tag v1.0.1        │
       │                 │
       ▼                 │
   Release workflow      │
       │                 │
       ▼                 │
   v1.0.1 released       │
```

## Step-by-Step

### 1. Create Hotfix Branch from Tag

```bash
git fetch --tags
git checkout v1.0.0
git checkout -b hotfix/v1.0.1
```

### 2. Apply the Fix

```bash
cd cch_cli
cargo fmt
cargo clippy --all-targets --all-features -- -D warnings
cargo test
```

### 3. Update Version

```toml
[workspace.package]
version = "1.0.1"
```

### 4. Update Changelog

```markdown
## [1.0.1] - YYYY-MM-DD

### Fixed

- Description of the hotfix
```

### 5. Commit and Push

```bash
git add -A
git commit -m "fix: <description of hotfix>"
git push -u origin hotfix/v1.0.1
```

### 6. Create PR and Merge

```bash
gh pr create --title "fix: hotfix v1.0.1" --body "..."
gh pr merge <PR_NUMBER> --merge --delete-branch
```

### 7. Tag and Release

```bash
git checkout main && git pull
git tag v1.0.1
git push origin v1.0.1
```

### 8. Verify

```bash
.claude/skills/release-cch/scripts/verify-release.sh 1.0.1
```
