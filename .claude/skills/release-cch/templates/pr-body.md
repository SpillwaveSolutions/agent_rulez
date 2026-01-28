## Summary

Prepare for the v${VERSION} release of Claude Context Hooks (CCH).

## Changes

- Update version to ${VERSION} in Cargo.toml
- Add CHANGELOG.md entry for v${VERSION}

## Pre-release Checklist

- [ ] Version updated in `Cargo.toml`
- [ ] CHANGELOG.md updated with release notes
- [ ] All tests passing locally
- [ ] Clippy has no warnings
- [ ] Format check passes

## Release Checklist (After PR Merge)

1. Checkout main:
   ```bash
   git checkout main && git pull
   ```

2. Create tag:
   ```bash
   git tag v${VERSION}
   ```

3. Push tag (triggers release workflow):
   ```bash
   git push origin v${VERSION}
   ```

4. Verify release:
   ```bash
   .claude/skills/release-cch/scripts/verify-release.sh
   ```

## Build Targets

This release will build cross-platform binaries for:

| Platform | Target |
|----------|--------|
| Linux x86_64 | x86_64-unknown-linux-gnu |
| Linux ARM64 | aarch64-unknown-linux-gnu |
| macOS Intel | x86_64-apple-darwin |
| macOS Apple Silicon | aarch64-apple-darwin |
| Windows | x86_64-pc-windows-msvc |
