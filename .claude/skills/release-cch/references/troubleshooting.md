# Release Troubleshooting

## Common Issues

### Pre-flight Check Failures

| Issue | Cause | Solution |
|-------|-------|----------|
| "cargo fmt failed" | Code not formatted | `cd cch_cli && cargo fmt` |
| "clippy warnings" | Lint issues | `cd cch_cli && cargo clippy --fix` |
| "tests failed" | Broken tests | `cd cch_cli && cargo test` to reproduce |
| "not on correct branch" | Wrong branch | `git checkout main` or create release branch |
| "uncommitted changes" | Dirty working dir | Commit or stash changes |

### PR CI Failures

1. **Check which job failed**:
   ```bash
   gh pr checks <PR_NUMBER>
   ```

2. **View logs**: Click the failed check URL in output

3. **Common fixes**:

   **Format failure**:
   ```bash
   cd cch_cli && cargo fmt
   git add -A && git commit -m "style: fix formatting"
   git push
   ```

   **Clippy failure**:
   ```bash
   cd cch_cli && cargo clippy --all-targets --all-features -- -D warnings
   git add -A && git commit -m "fix: address clippy warnings"
   git push
   ```

   **Test failure**:
   ```bash
   cd cch_cli && cargo test
   git add -A && git commit -m "fix: repair broken test"
   git push
   ```

### Tag Push Doesn't Trigger Workflow

1. **Verify tag format**: Must match `v*` pattern
2. **Check workflow trigger** in `.github/workflows/release.yml`
3. **Verify GitHub Actions is enabled**
4. **Check if tag exists on remote**:
   ```bash
   git ls-remote --tags origin | grep v1.0.0
   ```

---

## Recovery Procedures

### Delete and Recreate Tag

```bash
git tag -d v1.0.0
git push origin :refs/tags/v1.0.0
# Fix the issue...
git tag v1.0.0
git push origin v1.0.0
```

### Delete Draft/Failed Release

```bash
gh release list
gh release delete v1.0.0 --yes
```

### Force Re-run Workflow

```bash
gh run list --limit 5
gh run rerun <RUN_ID> --failed
```

---

## Diagnostic Commands

### Check Repository State

```bash
git branch --show-current
git tag -l
git ls-remote --tags origin
git status
git log --oneline -10
```

### Check GitHub State

```bash
gh pr list
gh run list --limit 5
gh run view <RUN_ID>
gh release list
gh release view v1.0.0
```
