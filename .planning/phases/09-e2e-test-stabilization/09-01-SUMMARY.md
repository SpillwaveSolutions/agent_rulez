---
phase: 09-e2e-test-stabilization
plan: 01
subsystem: test-infrastructure
tags: [e2e-tests, cross-platform, macos-symlinks, path-resolution]

dependency_graph:
  requires: []
  provides:
    - canonicalize_path() helper in tests/common/mod.rs
    - Canonical path handling in E2E test event setup
  affects:
    - rulez/tests/e2e_git_push_block.rs

tech_stack:
  added: []
  patterns:
    - Cross-platform path canonicalization with fallback
    - Symlink resolution for tempdir paths in test fixtures

key_files:
  created: []
  modified:
    - rulez/tests/common/mod.rs — Added canonicalize_path() helper
    - rulez/tests/e2e_git_push_block.rs — Updated E2E tests to use canonical paths

decisions:
  - decision: Use canonicalize_path() wrapper with fallback instead of raw fs::canonicalize()
    rationale: Provides graceful degradation when path doesn't exist yet
    alternatives: [Could use expect() to fail fast, but fallback is safer for test setup]
  - decision: Apply canonicalization at event JSON creation time
    rationale: Ensures event cwd matches what binary sees after internal canonicalization
    alternatives: [Could canonicalize in binary, but that's already done - this ensures test consistency]

metrics:
  duration_minutes: 4
  tasks_completed: 2
  files_modified: 2
  tests_added: 0
  tests_modified: 8
  completed_date: 2026-02-10
---

# Phase 9 Plan 1: E2E Path Canonicalization Summary

**One-liner:** Added canonicalize_path() helper to resolve macOS /var symlinks in E2E test event JSON, fixing tempdir path mismatches.

## What Was Built

### Core Implementation

**canonicalize_path() Helper Function**
- Added to `rulez/tests/common/mod.rs`
- Wraps `fs::canonicalize()` with fallback to original path
- Resolves macOS symlink: `/var/folders/...` → `/private/var/folders/...`
- Graceful degradation when path doesn't exist yet
- Used by all E2E tests for consistent path handling

**E2E Test Updates**
- Updated `setup_claude_code_event()` to canonicalize tempdir paths before embedding in event JSON
- Updated `test_e2e_no_config_allows_all()` to use canonical paths for standalone tempdir
- All 8 E2E tests now use canonical paths in cwd field

### Problem Solved

On macOS, `tempfile::tempdir()` returns paths like `/var/folders/...` but `/var` is a symlink to `/private/var`. When the binary internally canonicalizes the cwd from the event JSON, it resolves to `/private/var/folders/...`, causing path mismatches in config loading and test assertions.

**Before:** Event JSON had `/var/...` but binary saw `/private/var/...` → potential config loading failures
**After:** Event JSON has `/private/var/...` matching what binary sees → consistent path resolution

## Testing & Verification

**Test Results:**
- All 8 E2E tests pass with canonical paths
- Full test suite: 258 unit tests + 250+ integration tests pass
- `cargo llvm-cov` passes (catches pipe/process bugs)
- `cargo clippy` passes with `-D warnings`
- `cargo fmt --all --check` passes

**E2E Tests Verified:**
1. `test_e2e_git_push_blocked_exit_code_2` — Git push blocked with exit code 2
2. `test_e2e_cwd_based_config_loading_exit_code_2` — CWD-based config loading works
3. `test_e2e_git_status_allowed_exit_code_0` — Git status allowed
4. `test_e2e_git_push_variants_exit_code_2` — All push variants blocked
5. `test_e2e_non_push_git_commands_exit_code_0` — Non-push commands allowed
6. `test_e2e_output_format_claude_code_protocol` — Output format correct
7. `test_e2e_no_config_allows_all` — No config = fail-open
8. `test_e2e_cwd_git_push_variants_from_wrong_dir` — CWD variants from wrong dir

## Deviations from Plan

### Auto-fixed Issues

**1. [Rule 1 - Bug] Cargo fmt formatting violations**
- **Found during:** Final verification step
- **Issue:** canonicalize_path() and use statement didn't follow rustfmt conventions (multi-line unwrap_or_else, single-line imports)
- **Fix:** Applied `cargo fmt --all` to fix formatting
- **Files modified:** rulez/tests/common/mod.rs, rulez/tests/e2e_git_push_block.rs
- **Commit:** 765baad

## Implementation Notes

### Path Canonicalization Strategy

The helper uses a simple pattern:
```rust
fs::canonicalize(path.as_ref()).unwrap_or_else(|_| path.as_ref().to_path_buf())
```

This provides:
- **Success case:** Symlinks resolved to canonical paths
- **Failure case:** Original path returned (e.g., path doesn't exist yet)
- **No panic:** Graceful degradation for edge cases

### Cross-Platform Behavior

- **macOS:** Resolves `/var` → `/private/var` symlink
- **Linux:** Typically no-op (tempdir already canonical)
- **Windows:** Resolves junction points if present
- **All platforms:** Fallback ensures tests don't break if canonicalization fails

## Requirements Traceability

| Requirement | Implementation | Verification |
|-------------|----------------|--------------|
| REQ-E2E-05: Canonical paths in E2E tests | canonicalize_path() in setup_claude_code_event() | All 8 E2E tests pass |
| REQ-E2E-05: Symlink resolution | fs::canonicalize() wrapper | macOS /var → /private/var resolved |
| REQ-E2E-05: Graceful fallback | unwrap_or_else() with to_path_buf() | Tests don't panic on missing paths |
| REQ-E2E-03: No broken pipe issues | Already satisfied (tests use .output() not .spawn()) | cargo llvm-cov passes |

## Self-Check: PASSED

**Created files verified:**
- N/A (no new files created)

**Modified files verified:**
```bash
$ [ -f "rulez/tests/common/mod.rs" ] && echo "FOUND: rulez/tests/common/mod.rs"
FOUND: rulez/tests/common/mod.rs

$ [ -f "rulez/tests/e2e_git_push_block.rs" ] && echo "FOUND: rulez/tests/e2e_git_push_block.rs"
FOUND: rulez/tests/e2e_git_push_block.rs
```

**Commits verified:**
```bash
$ git log --oneline --grep="09-01" -3
765baad fix(09-01): apply cargo fmt to canonical path implementation
4122776 test(09-01): use canonical paths in E2E event JSON setup
8ac14a5 test(09-01): add canonicalize_path() helper for cross-platform path resolution
```

**Function existence verified:**
```bash
$ grep -n "pub fn canonicalize_path" rulez/tests/common/mod.rs
116:pub fn canonicalize_path<P: AsRef<Path>>(path: P) -> PathBuf {
```

**Usage verified:**
```bash
$ grep -n "canonicalize_path" rulez/tests/e2e_git_push_block.rs
25:use common::{
26:    CchResponse, TestEvidence, Timer, canonicalize_path, evidence_dir, fixture_path, setup_test_env,
33:    let canonical_path = canonicalize_path(temp_dir.path());
353:    let cwd = canonicalize_path(empty_dir.path())
```

All verification checks passed.

## Commits

| Order | Hash | Message |
|-------|------|---------|
| 1 | 8ac14a5 | test(09-01): add canonicalize_path() helper for cross-platform path resolution |
| 2 | 4122776 | test(09-01): use canonical paths in E2E event JSON setup |
| 3 | 765baad | fix(09-01): apply cargo fmt to canonical path implementation |

## Next Steps

Proceed to Phase 9 Plan 2 (E2E Cross-Platform Matrix) to:
1. Add GitHub Actions matrix for Linux + macOS E2E tests
2. Verify canonical path handling works consistently across platforms
3. Document platform-specific behaviors in CI logs
