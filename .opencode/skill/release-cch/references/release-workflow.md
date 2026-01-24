# CCH Release Workflow

## Overview Diagram

```
┌─────────────────────────────────────────────────────────────────────┐
│                        PHASE 1: PREPARE                             │
├─────────────────────────────────────────────────────────────────────┤
│                                                                     │
│  1. Update version in Cargo.toml (manual)                           │
│                          │                                          │
│                          ▼                                          │
│  2. git checkout -b release/vX.Y.Z                                  │
│                          │                                          │
│                          ▼                                          │
│  3. Run preflight-check.sh ─────────────────────┐                   │
│                          │                      │                   │
│                          ▼                      ▼                   │
│                    [All pass?] ──No──► Fix issues, retry            │
│                          │                                          │
│                         Yes                                         │
│                          │                                          │
│                          ▼                                          │
│  4. Generate/edit CHANGELOG.md                                      │
│                          │                                          │
│                          ▼                                          │
│  5. git commit -m "chore: prepare vX.Y.Z release"                   │
│                          │                                          │
│                          ▼                                          │
│  6. git push -u origin release/vX.Y.Z                               │
│                          │                                          │
│                          ▼                                          │
│  7. gh pr create                                                    │
│                          │                                          │
│                          ▼                                          │
│  8. Wait for CI (15 checks) ────────────────────┐                   │
│                          │                      │                   │
│                          ▼                      ▼                   │
│                   [All green?] ──No──► Fix issues, push again       │
│                          │                                          │
│                         Yes                                         │
│                          │                                          │
└──────────────────────────┼──────────────────────────────────────────┘
                           │
                           ▼
┌─────────────────────────────────────────────────────────────────────┐
│                        PHASE 2: EXECUTE                             │
├─────────────────────────────────────────────────────────────────────┤
│                                                                     │
│  1. gh pr merge --merge --delete-branch                             │
│                          │                                          │
│                          ▼                                          │
│  2. git checkout main && git pull                                   │
│                          │                                          │
│                          ▼                                          │
│  3. git tag vX.Y.Z                                                  │
│                          │                                          │
│                          ▼                                          │
│  4. git push origin vX.Y.Z ───────────► TRIGGERS RELEASE WORKFLOW   │
│                          │                                          │
└──────────────────────────┼──────────────────────────────────────────┘
                           │
                           ▼
┌─────────────────────────────────────────────────────────────────────┐
│                        PHASE 3: VERIFY                              │
├─────────────────────────────────────────────────────────────────────┤
│                                                                     │
│  1. gh run list / gh run view <RUN_ID>                              │
│                          │                                          │
│                          ▼                                          │
│  2. Wait for 5 build jobs + 1 release job                           │
│                          │                                          │
│        ┌─────────────────┼─────────────────┐                        │
│        │                 │                 │                        │
│        ▼                 ▼                 ▼                        │
│   Linux x86_64    macOS x86_64    Windows x86_64                    │
│   Linux aarch64   macOS aarch64                                     │
│        │                 │                 │                        │
│        └─────────────────┼─────────────────┘                        │
│                          │                                          │
│                          ▼                                          │
│  3. Create Release job (uploads artifacts)                          │
│                          │                                          │
│                          ▼                                          │
│  4. gh release view vX.Y.Z                                          │
│                          │                                          │
│                          ▼                                          │
│  5. Verify 6 assets uploaded                                        │
│     - cch-linux-x86_64.tar.gz                                       │
│     - cch-linux-aarch64.tar.gz                                      │
│     - cch-macos-x86_64.tar.gz                                       │
│     - cch-macos-aarch64.tar.gz                                      │
│     - cch-windows-x86_64.exe.zip                                    │
│     - checksums.txt                                                 │
│                                                                     │
└─────────────────────────────────────────────────────────────────────┘
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
| ubuntu-latest | x86_64-unknown-linux-gnu | cch-linux-x86_64.tar.gz |
| ubuntu-latest | aarch64-unknown-linux-gnu | cch-linux-aarch64.tar.gz |
| macos-latest | x86_64-apple-darwin | cch-macos-x86_64.tar.gz |
| macos-latest | aarch64-apple-darwin | cch-macos-aarch64.tar.gz |
| windows-latest | x86_64-pc-windows-msvc | cch-windows-x86_64.exe.zip |

### Create Release Job

After all builds complete:

1. Download all artifacts
2. Generate checksums: `sha256sum *.tar.gz *.zip > checksums.txt`
3. Create GitHub release with `softprops/action-gh-release`
4. Upload all assets

## Version Flow

```
Cargo.toml                    Git Tags                    GitHub Release
    │                             │                             │
    ▼                             ▼                             ▼
version = "1.0.0"  ───────►  v1.0.0  ────────────────►  Release v1.0.0
    │                             │                        │
    │                             │                        ├─ Assets
    │                             │                        ├─ Release notes
    │                             │                        └─ Checksums
    │                             │
    ▼                             ▼
version = "1.1.0"  ───────►  v1.1.0  ────────────────►  Release v1.1.0
```

## Timing Expectations

| Phase | Typical Duration |
|-------|-----------------|
| Prepare (manual) | 5-10 minutes |
| CI checks | 2-3 minutes |
| Review/Merge PR | Variable |
| Tag push to release | 3-5 minutes |
| **Total** | ~15-20 minutes (excluding review) |
