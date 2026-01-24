---
description: Execute CCH release workflow - prepare, execute, verify, or hotfix releases
---

## User Input

```text
$ARGUMENTS
```

## CCH Release Workflow

This command orchestrates the CCH release process using the `release-cch` skill.

### Quick Reference

| Phase | Command | Description |
|-------|---------|-------------|
| Prepare | `/cch-release prepare` | Create branch, changelog, PR |
| Execute | `/cch-release execute` | Merge PR, create tag |
| Verify | `/cch-release verify` | Check release status |
| Hotfix | `/cch-release hotfix v1.0.0` | Patch from existing tag |
| Full | `/cch-release` | Interactive full workflow |

### Workflow

1. **Load the release-cch skill**: Read `.opencode/skill/release-cch/SKILL.md` for detailed instructions.

2. **Read version** from `Cargo.toml` (single source of truth):
   ```bash
   .opencode/skill/release-cch/scripts/read-version.sh
   ```

3. **Parse arguments** and execute the appropriate phase:

   **If `$ARGUMENTS` is empty** (interactive mode):
   - Ask user which phase to execute
   - Guide through each step with confirmations

   **If `$ARGUMENTS` is `prepare`**:
   - Verify version is updated in `Cargo.toml`
   - Run preflight checks: `.opencode/skill/release-cch/scripts/preflight-check.sh`
   - Create release branch: `git checkout -b release/v${VERSION}`
   - Generate changelog: `.opencode/skill/release-cch/scripts/generate-changelog.sh`
   - Commit and push release branch
   - Create PR with release checklist

   **If `$ARGUMENTS` is `execute`**:
   - Verify PR is merged
   - Sync main: `git checkout main && git pull`
   - Create tag: `git tag v${VERSION}`
   - Push tag: `git push origin v${VERSION}`
   - This triggers the release workflow

   **If `$ARGUMENTS` is `verify`**:
   - Run verification: `.opencode/skill/release-cch/scripts/verify-release.sh`
   - Check workflow status
   - Verify release assets

   **If `$ARGUMENTS` starts with `hotfix`**:
   - Extract base tag from arguments (e.g., `hotfix v1.0.0`)
   - Checkout the base tag
   - Create hotfix branch
   - Guide through hotfix workflow (see SKILL.md Phase 4)

### Version Management

**IMPORTANT**: The version is read from `Cargo.toml` at the workspace root:

```toml
[workspace.package]
version = "X.Y.Z"
```

Before running `/cch-release prepare`:
1. Decide on the new version (follow semver)
2. Update the version in `Cargo.toml`
3. Then run the prepare phase

### Pre-release Checklist

Before any release, the preflight script verifies:

- [ ] Clean working directory (or only release files modified)
- [ ] On correct branch (main, release/*, or hotfix/*)
- [ ] `cargo fmt --check` passes
- [ ] `cargo clippy` has no warnings
- [ ] All tests pass
- [ ] CHANGELOG.md exists

### CI Checks (15 total)

The release PR must pass all checks:

| Category | Checks |
|----------|--------|
| Quality | Format, Clippy, Unit Tests, Code Coverage |
| Integration | 6 user story test jobs |
| Build | 5 cross-platform builds |
| Meta | CI Success |

### Release Assets

After tagging, the workflow builds and uploads:

- `cch-linux-x86_64.tar.gz`
- `cch-linux-aarch64.tar.gz`
- `cch-macos-x86_64.tar.gz`
- `cch-macos-aarch64.tar.gz`
- `cch-windows-x86_64.exe.zip`
- `checksums.txt`

### Troubleshooting

If something goes wrong, see:
- `.opencode/skill/release-cch/references/troubleshooting.md`
- Or run `/cch-release verify` to diagnose

### Examples

```bash
# Full interactive release
/cch-release

# Just prepare (create branch, changelog, PR)
/cch-release prepare

# Execute after PR is merged (tag and push)
/cch-release execute

# Verify release completed
/cch-release verify

# Create hotfix from v1.0.0
/cch-release hotfix v1.0.0
```
