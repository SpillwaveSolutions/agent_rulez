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
   # Fix reported issues
   git add -A && git commit -m "fix: address clippy warnings"
   git push
   ```

   **Test failure**:
   ```bash
   cd cch_cli && cargo test
   # Find and fix failing test
   git add -A && git commit -m "fix: repair broken test"
   git push
   ```

### Tag Push Doesn't Trigger Workflow

1. **Verify tag format**: Must match `v*` pattern
   ```bash
   git tag -l | grep "^v"
   ```

2. **Check workflow trigger** in `.github/workflows/release.yml`:
   ```yaml
   on:
     push:
       tags:
         - 'v*'
   ```

3. **Verify GitHub Actions is enabled**:
   - Go to repo Settings > Actions > General
   - Ensure "Allow all actions" is selected

4. **Check if tag exists on remote**:
   ```bash
   git ls-remote --tags origin | grep v1.0.0
   ```

### Build Fails for Specific Platform

**Linux aarch64**:
- Usually missing cross-compiler
- CI installs `gcc-aarch64-linux-gnu` automatically
- If local build needed: `sudo apt-get install gcc-aarch64-linux-gnu`

**macOS**:
- Ensure Xcode command line tools installed
- Check target is added: `rustup target add aarch64-apple-darwin`

**Windows**:
- Uses MSVC toolchain
- May need Visual Studio Build Tools

**View full logs**:
```bash
gh run view <RUN_ID> --log
```

### Release Created but Assets Missing

1. **Check build jobs completed**:
   ```bash
   gh run view <RUN_ID>
   ```

2. **Look for upload artifact step**:
   - Check "Upload artifact" step in each build job
   - Check "Create Release" job logs

3. **Verify artifact names**:
   - Must match expected patterns in release workflow

### Version Mismatch

**Symptom**: Tag version doesn't match Cargo.toml

**Solution**:
```bash
# Read current version
.claude/skills/release-cch/scripts/read-version.sh

# Should match your intended tag
# If not, update Cargo.toml and re-run release
```

---

## Recovery Procedures

### Delete and Recreate Tag

```bash
# Delete local tag
git tag -d v1.0.0

# Delete remote tag
git push origin :refs/tags/v1.0.0

# Fix the issue...

# Recreate tag
git tag v1.0.0
git push origin v1.0.0
```

### Delete Draft/Failed Release

```bash
# List releases
gh release list

# Delete specific release
gh release delete v1.0.0 --yes
```

### Rollback Version Bump

If you need to undo a version change:

```bash
git checkout main
git log --oneline -5  # Find the version bump commit

# Revert the commit
git revert <commit-sha>
git push
```

### Force Re-run Workflow

If workflow failed partway:

```bash
# Find the run ID
gh run list --limit 5

# Re-run failed jobs
gh run rerun <RUN_ID> --failed
```

---

## Diagnostic Commands

### Check Repository State

```bash
# Current branch
git branch --show-current

# Local tags
git tag -l

# Remote tags
git ls-remote --tags origin

# Uncommitted changes
git status

# Recent commits
git log --oneline -10
```

### Check GitHub State

```bash
# Open PRs
gh pr list

# Recent workflow runs
gh run list --limit 5

# Specific workflow run
gh run view <RUN_ID>

# Releases
gh release list

# Specific release
gh release view v1.0.0
```

### Check CI Status

```bash
# PR checks
gh pr checks <PR_NUMBER>

# Watch checks
gh pr checks <PR_NUMBER> --watch

# Workflow run details
gh run view <RUN_ID> --log
```

---

## Getting Help

1. **Check this document first** for common issues

2. **Review workflow logs**:
   ```bash
   gh run view <RUN_ID> --log
   ```

3. **Check GitHub Actions UI** for more details:
   ```
   https://github.com/SpillwaveSolutions/code_agent_context_hooks/actions
   ```

4. **Search existing issues**:
   ```bash
   gh issue list --search "release"
   ```
