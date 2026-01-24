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
# Fetch all tags
git fetch --tags

# List available tags
git tag -l

# Checkout the tag you want to patch
git checkout v1.0.0

# Create hotfix branch
git checkout -b hotfix/v1.0.1
```

### 2. Apply the Fix

Make the minimal fix needed. Keep changes focused on the issue.

```bash
# Edit the necessary files
# ...

# Run all checks
cd cch_cli
cargo fmt
cargo clippy --all-targets --all-features -- -D warnings
cargo test
```

### 3. Update Version

Edit `Cargo.toml` at workspace root:

```toml
[workspace.package]
version = "1.0.1"  # Increment patch version
```

### 4. Update Changelog

Add entry at the top of `CHANGELOG.md`:

```markdown
## [1.0.1] - YYYY-MM-DD

### Fixed

- Description of the critical fix
```

### 5. Commit and Push

```bash
git add -A
git commit -m "fix: <description of hotfix>

Hotfix for v1.0.0 addressing <issue description>.
Fixes #<issue-number> (if applicable)"

git push -u origin hotfix/v1.0.1
```

### 6. Create PR

```bash
gh pr create \
  --title "fix: hotfix v1.0.1" \
  --body "## Hotfix Release

**Base Version**: v1.0.0
**Hotfix Version**: v1.0.1

### Issue
<Description of the critical bug>

### Fix
<Description of the fix>

### Testing
- [ ] Local tests pass
- [ ] Fix verified manually

### Release Steps After Merge
\`\`\`bash
git checkout main && git pull
git tag v1.0.1
git push origin v1.0.1
\`\`\`"
```

### 7. Wait for CI and Merge

```bash
# Watch CI
gh pr checks <PR_NUMBER> --watch

# Merge when green
gh pr merge <PR_NUMBER> --merge --delete-branch
```

### 8. Tag and Release

```bash
git checkout main
git pull
git tag v1.0.1
git push origin v1.0.1
```

### 9. Verify

```bash
.opencode/skill/release-cch/scripts/verify-release.sh 1.0.1
```

## Important Notes

### DO

- Keep hotfixes minimal and focused
- Increment only the patch version
- Test thoroughly before releasing
- Document the fix clearly in changelog

### DON'T

- Include unrelated changes
- Skip CI checks
- Forget to update the version
- Rush without proper testing

## Versioning Example

```
v1.0.0  (initial release)
   │
   ├── Bug found in production
   │
   ▼
v1.0.1  (hotfix for critical bug)
   │
   ├── Another bug found
   │
   ▼
v1.0.2  (another hotfix)

Meanwhile, main branch continues:
v1.0.0 ──► development ──► v1.1.0 (includes v1.0.1, v1.0.2 fixes)
```

## Cherry-picking (Advanced)

If you maintain long-lived release branches, you may need to cherry-pick:

```bash
# After hotfix is merged to main
git checkout release/v1.0
git cherry-pick <hotfix-commit-sha>
git push
```
