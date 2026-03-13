# RuleZ Release Workflow

## Overview Diagram

```
+---------------------------------------------------------------------+
|                        PHASE 1: PREPARE                             |
+---------------------------------------------------------------------+
|                                                                     |
|  1. Update version in Cargo.toml (manual)                           |
|                          |                                          |
|                          v                                          |
|  2. git checkout -b release/vX.Y.Z                                  |
|                          |                                          |
|                          v                                          |
|  3. Run preflight-check.sh ---------------------+                   |
|                          |                      |                   |
|                          v                      v                   |
|                    [All pass?] --No--> Fix issues, retry            |
|                          |                                          |
|                         Yes                                         |
|                          |                                          |
|                          v                                          |
|  4. Generate/edit CHANGELOG.md                                      |
|                          |                                          |
|                          v                                          |
|  5. git commit -m "chore: prepare vX.Y.Z release"                   |
|                          |                                          |
|                          v                                          |
|  6. git push -u origin release/vX.Y.Z                               |
|                          |                                          |
|                          v                                          |
|  7. gh pr create                                                    |
|                          |                                          |
|                          v                                          |
|  8. Wait for CI (15 checks) ----------------------+                 |
|                          |                        |                 |
|                          v                        v                 |
|                   [All green?] --No--> Fix issues, push again       |
|                          |                                          |
|                         Yes                                         |
|                          |                                          |
+---------------------------------------------------------------------+
                           |
                           v
+---------------------------------------------------------------------+
|                        PHASE 2: EXECUTE                             |
+---------------------------------------------------------------------+
|                                                                     |
|  1. gh pr merge --merge --delete-branch                             |
|                          |                                          |
|                          v                                          |
|  2. git checkout main && git pull                                   |
|                          |                                          |
|                          v                                          |
|  3. git tag vX.Y.Z                                                  |
|                          |                                          |
|                          v                                          |
|  4. git push origin vX.Y.Z -----------> TRIGGERS RELEASE WORKFLOW   |
|                          |                                          |
+---------------------------------------------------------------------+
                           |
                           v
+---------------------------------------------------------------------+
|                        PHASE 3: VERIFY                              |
+---------------------------------------------------------------------+
|                                                                     |
|  1. gh run list / gh run view <RUN_ID>                              |
|                          |                                          |
|                          v                                          |
|  2. Wait for 5 build jobs + 1 release job                           |
|                          |                                          |
|        +-----------------+-----------------+                        |
|        |                 |                 |                        |
|        v                 v                 v                        |
|   Linux x86_64    macOS x86_64    Windows x86_64                    |
|   Linux aarch64   macOS aarch64                                     |
|        |                 |                 |                        |
|        +-----------------+-----------------+                        |
|                          |                                          |
|                          v                                          |
|  3. Create Release job (uploads artifacts)                          |
|                          |                                          |
|                          v                                          |
|  4. gh release view vX.Y.Z                                          |
|                          |                                          |
|                          v                                          |
|  5. Verify 6 assets uploaded                                        |
|     - rulez-linux-x86_64.tar.gz                                     |
|     - rulez-linux-aarch64.tar.gz                                    |
|     - rulez-macos-x86_64.tar.gz                                     |
|     - rulez-macos-aarch64.tar.gz                                    |
|     - rulez-windows-x86_64.exe.zip                                  |
|     - checksums.txt                                                 |
|                                                                     |
+---------------------------------------------------------------------+
```

## CI Checks Detail (15 total)

| # | Check | Description | Time |
|---|-------|-------------|------|
| 1 | Format | `cargo fmt --check` | ~15s |
| 2 | Clippy | `cargo clippy -- -D warnings` | ~25s |
| 3 | Unit Tests | Core unit tests | ~30s |
| 4 | Code Coverage | Coverage report generation | ~55s |
| 5-10 | Integration Tests | One per user story (6 jobs) | ~30s each |
| 11-15 | Build Release | Cross-platform builds (5 jobs) | ~1-2m each |
| 16 | CI Success | Meta-check (all above pass) | ~5s |

## Release Workflow Jobs

The `.github/workflows/release.yml` runs:

### Build Matrix (5 parallel jobs)

| OS | Target | Output |
|----|--------|--------|
| ubuntu-latest | x86_64-unknown-linux-gnu | rulez-linux-x86_64.tar.gz |
| ubuntu-latest | aarch64-unknown-linux-gnu | rulez-linux-aarch64.tar.gz |
| macos-latest | x86_64-apple-darwin | rulez-macos-x86_64.tar.gz |
| macos-latest | aarch64-apple-darwin | rulez-macos-aarch64.tar.gz |
| windows-latest | x86_64-pc-windows-msvc | rulez-windows-x86_64.exe.zip |

### Create Release Job

After all builds complete:

1. Download all artifacts
2. Generate checksums: `sha256sum *.tar.gz *.zip > checksums.txt`
3. Create GitHub release with `softprops/action-gh-release`
4. Upload all assets

## Version Flow

```
Cargo.toml                    Git Tags                    GitHub Release
    |                             |                             |
    v                             v                             v
version = "1.0.0"  ------->  v1.0.0  ---------------->  Release v1.0.0
    |                             |                        |
    |                             |                        +- Assets
    |                             |                        +- Release notes
    |                             |                        +- Checksums
    |                             |
    v                             v
version = "1.1.0"  ------->  v1.1.0  ---------------->  Release v1.1.0
```

## Timing Expectations

| Phase | Typical Duration |
|-------|-----------------|
| Prepare (manual) | 5-10 minutes |
| CI checks | 2-3 minutes |
| Review/Merge PR | Variable |
| Tag push to release | 3-5 minutes |
| **Total** | ~15-20 minutes (excluding review) |
