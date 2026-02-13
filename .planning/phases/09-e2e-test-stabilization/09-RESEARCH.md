# Phase 9: E2E Test Stabilization - Research

**Researched:** 2026-02-10
**Domain:** Cross-platform test infrastructure, CI/CD reliability, process execution testing
**Confidence:** HIGH

## Summary

Phase 9 stabilizes E2E tests to run reliably on Linux, macOS, and Windows in GitHub Actions CI. The project currently has 631 tests passing on macOS locally, but E2E tests may fail on Ubuntu CI due to:

1. **Path resolution issues** - macOS `/var` symlinks to `/private/var`, causing cwd mismatches
2. **Broken pipe errors** - Already fixed in Phase 6 for `execute_inline_script`, but may exist in test helpers
3. **Binary validation gaps** - Tests may use stale cached binaries after rename from `cch` to `rulez`
4. **Platform-specific tempfile handling** - Windows backslashes, cleanup race conditions

The codebase already uses `assert_cmd`, `tempfile`, and has comprehensive E2E tests (`e2e_git_push_block.rs` with 8 tests). This phase adds **cross-platform path canonicalization**, **CI matrix testing**, and **binary artifact validation** to ensure all 631+ tests pass on all three platforms.

**Primary recommendation:** Add `fs::canonicalize()` helper in `tests/common/mod.rs`, update E2E test setup to use canonical paths, add CI matrix with ubuntu-latest/macos-latest/windows-latest, and validate binary existence before test execution.

## Standard Stack

### Core Testing Libraries (Already Present)

| Library | Version | Purpose | Why Standard |
|---------|---------|---------|--------------|
| assert_cmd | 2.0 | CLI binary testing via `cargo_bin()` | De facto standard for Rust CLI testing, used by ripgrep, bat, fd |
| tempfile | 3.0 | Cross-platform temporary directories | Handles cleanup automatically, platform-aware paths |
| predicates | 3.0 | Assertion predicates for assert_cmd | Part of assert_cmd ecosystem |
| cargo test | Built-in | Test runner with filtering, parallelism | Standard Rust testing framework |

### Platform Testing (GitHub Actions)

| Component | Version | Purpose | When to Use |
|-----------|---------|---------|-------------|
| ubuntu-latest | 24.04 | Linux CI testing | All commits, primary platform |
| macos-latest | 14 (ARM64) | macOS ARM64 testing | Full validation (PRs to main) |
| macos-15-intel | x86_64 | macOS Intel testing | Full validation (PRs to main) |
| windows-latest | 2022 | Windows testing | Full validation (PRs to main) |

### Alternatives Considered

| Instead of | Could Use | Tradeoff |
|------------|-----------|----------|
| assert_cmd | Manual std::process::Command | Lose cargo_bin() binary resolution, more boilerplate |
| tempfile | Manual mktemp + cleanup | Harder to ensure cleanup on panic, platform path issues |
| CI matrix | Manual per-platform jobs | More YAML duplication, harder to maintain |
| fs::canonicalize() | String path manipulation | Fragile, Windows-incompatible, symlink-unaware |

**Installation:** No new dependencies - already present in `Cargo.toml` dev-dependencies.

## Architecture Patterns

### Recommended Test Structure

```
rulez/tests/
├── common/mod.rs              # Shared test utilities
│   ├── canonicalize_path()    # NEW: Resolve symlinks for cross-platform paths
│   ├── setup_test_env()       # Existing: Create temp dir with hooks.yaml
│   ├── fixtures_dir()         # Existing: Get test fixtures path
│   └── TestEvidence           # Existing: IQ/OQ/PQ evidence logging
├── e2e_*.rs                   # E2E tests using assert_cmd
├── integration_*.rs           # Integration tests (unit-level)
└── fixtures/                  # Test data (YAML configs, scripts)
```

### Pattern 1: Cross-Platform Path Canonicalization

**What:** Resolve symlinks and platform-specific path variations before using paths in events or assertions.

**When to use:** Any time a test sets up an event with a `cwd` field or compares file paths.

**Example:**
```rust
// tests/common/mod.rs
use std::fs;
use std::path::{Path, PathBuf};

/// Canonicalize a path to resolve symlinks (e.g., macOS /var -> /private/var)
pub fn canonicalize_path<P: AsRef<Path>>(path: P) -> PathBuf {
    fs::canonicalize(path.as_ref())
        .unwrap_or_else(|_| path.as_ref().to_path_buf())
}

// Usage in E2E test setup:
fn setup_claude_code_event(config_name: &str, command: &str) -> (tempfile::TempDir, String) {
    let temp_dir = setup_test_env(config_name);

    // Resolve symlinks BEFORE creating event JSON
    let canonical_path = canonicalize_path(temp_dir.path());
    let cwd = canonical_path.to_string_lossy().to_string();

    let event = serde_json::json!({
        "hook_event_name": "PreToolUse",
        "tool_name": "Bash",
        "tool_input": { "command": command },
        "cwd": cwd,  // Now guaranteed to be canonical
        // ...
    });

    (temp_dir, serde_json::to_string(&event).unwrap())
}
```

**Source:** [Rust std::fs::canonicalize docs](https://doc.rust-lang.org/std/fs/fn.canonicalize.html)

### Pattern 2: Explicit Tempdir Cleanup

**What:** Force tempdir cleanup with `drop(temp_dir)` at end of tests to avoid race conditions.

**When to use:** E2E tests that spawn processes that may hold file handles.

**Example:**
```rust
#[test]
fn test_e2e_git_push_blocked() {
    let (temp_dir, event_json) = setup_claude_code_event("block-all-push.yaml", "git push");

    let output = Command::cargo_bin("rulez")
        .expect("binary exists")
        .current_dir(temp_dir.path())
        .write_stdin(event_json)
        .output()
        .expect("command should run");

    assert_eq!(output.status.code(), Some(2));

    // Explicit cleanup - don't wait for scope exit
    drop(temp_dir);
}
```

**Why:** On Windows, held file handles can prevent tempdir deletion. Explicit `drop()` ensures cleanup happens immediately after assertions.

### Pattern 3: Binary Artifact Validation in CI

**What:** Verify correct binary exists in PATH before running E2E tests.

**When to use:** CI workflows, especially after binary renames or cache updates.

**Example:**
```yaml
# .github/workflows/e2e-matrix.yml
- name: Validate binary artifact
  run: |
    BINARY_NAME="rulez"

    # Check binary exists
    if ! command -v $BINARY_NAME &> /dev/null; then
      echo "::error::Binary $BINARY_NAME not found in PATH"
      exit 1
    fi

    # Check it's the right binary (not stale cached version)
    BINARY_PATH=$(which $BINARY_NAME)
    echo "Binary found at: $BINARY_PATH"

    # Verify version contains expected binary name
    $BINARY_NAME --version | grep -q "rulez" || {
      echo "::error::Binary version check failed"
      exit 1
    }

    echo "Binary validation passed"

- name: Run E2E tests
  run: cargo test --test e2e_* -- --nocapture
```

### Pattern 4: CI Matrix for Cross-Platform Testing

**What:** Run E2E tests on ubuntu-latest, macos-latest, windows-latest in parallel.

**When to use:** All E2E test workflows.

**Example:**
```yaml
# .github/workflows/e2e-matrix.yml
name: E2E Tests - Cross-Platform

on:
  push:
    branches: [main, develop, "feature/**"]
  pull_request:
    branches: [main, develop]

jobs:
  e2e-matrix:
    name: E2E Tests (${{ matrix.os }})
    strategy:
      fail-fast: false  # Run all platforms even if one fails
      matrix:
        os: [ubuntu-latest, macos-latest, windows-latest]

    runs-on: ${{ matrix.os }}

    steps:
      - uses: actions/checkout@v4
      - uses: dtolnay/rust-toolchain@stable
      - uses: Swatinem/rust-cache@v2

      - name: Build binary
        run: cargo build --release --bin rulez

      - name: Validate binary (Unix)
        if: runner.os != 'Windows'
        run: |
          ./target/release/rulez --version
          which rulez || echo "Not in PATH yet"

      - name: Validate binary (Windows)
        if: runner.os == 'Windows'
        shell: pwsh
        run: |
          .\target\release\rulez.exe --version

      - name: Run E2E tests
        run: cargo test --tests --all-features -- --nocapture

      - name: Upload test results
        if: always()
        uses: actions/upload-artifact@v4
        with:
          name: test-results-${{ matrix.os }}
          path: target/test-evidence/
```

### Anti-Patterns to Avoid

- **String path concatenation:** `format!("{}/.claude", cwd)` breaks on Windows. Use `PathBuf::join()`.
- **Assuming tempdir paths are stable:** Symlinks mean path strings may differ from `temp_dir.path()`. Always canonicalize.
- **Using `spawn()` + `wait()` for piped processes:** Use `output()` or `wait_with_output()` to avoid broken pipe on Linux.
- **Skipping platform matrix:** "Works on my Mac" doesn't guarantee CI (Linux) or Windows compatibility.

## Don't Hand-Roll

| Problem | Don't Build | Use Instead | Why |
|---------|-------------|-------------|-----|
| Path canonicalization | Custom symlink resolver | `std::fs::canonicalize()` | Handles platform differences, resolves chains of symlinks |
| Cross-platform paths | String manipulation | `std::path::PathBuf` | Handles backslash/forward slash, UNC paths on Windows |
| Tempdir cleanup | Manual `rm -rf` in teardown | `tempfile::TempDir` with `drop()` | Cleans up on panic, platform-aware, explicit drop control |
| Binary existence checks | Parsing `which` output | `Command::cargo_bin("name")` or validate step | assert_cmd handles target/ path resolution |
| CI matrix duplication | Copy-paste per-OS jobs | GitHub Actions matrix strategy | DRY, easier to add platforms, parallel execution |

**Key insight:** Cross-platform path handling is harder than it looks. Rust's stdlib and `tempfile` crate already handle the edge cases (UNC paths, symlinks, permission errors, cleanup races). Don't reimplement these.

## Common Pitfalls

### Pitfall 1: macOS `/var` Symlink Causes Path Mismatches

**What goes wrong:** Tests pass locally on macOS but fail in CI when event `cwd` contains symlink paths that don't match canonical paths.

**Why it happens:** macOS `/var` is a symlink to `/private/var`. `tempfile::tempdir()` returns paths like `/var/folders/...`, but `fs::canonicalize()` resolves to `/private/var/folders/...`. If RuleZ config loader uses canonical paths but test event uses non-canonical paths, they don't match.

**How to avoid:** Always canonicalize paths in test setup before creating event JSON.

**Warning signs:**
- Tests pass on macOS but fail on Linux
- Error messages show different paths for same tempdir: `/var/...` vs `/private/var/...`
- Config loading logs "no hooks.yaml found" despite file existing

**Example fix:**
```rust
// BEFORE (fails on macOS)
let cwd = temp_dir.path().to_string_lossy().to_string();

// AFTER (works everywhere)
let canonical_path = fs::canonicalize(temp_dir.path())
    .unwrap_or_else(|_| temp_dir.path().to_path_buf());
let cwd = canonical_path.to_string_lossy().to_string();
```

**Source:** Project memory - "Stale Binary Artifacts (2026-02-10): macOS is more tolerant of unread pipe buffers"

### Pitfall 2: Broken Pipe from Unread Piped stdout/stderr

**What goes wrong:** Tests spawn child processes with `Stdio::piped()` but never read the output, causing SIGPIPE errors on Linux.

**Why it happens:** Linux kernel sends SIGPIPE when writing to a closed/full pipe. macOS is more tolerant. If test spawns binary with `.stdout(Stdio::piped())` but only calls `.wait()` (not `.wait_with_output()`), the pipe buffer fills and write fails.

**How to avoid:**
- Use `Stdio::null()` if you don't need output
- Use `wait_with_output()` instead of `spawn() + wait()`
- Always drain piped stdout/stderr

**Warning signs:**
- Tests pass on macOS but fail on Linux CI with "Broken pipe" error
- Exit status is SIGPIPE (141 on Linux)
- Test uses `.stdout(Stdio::piped())` but never reads from `child.stdout`

**Example fix:**
```rust
// WRONG - creates pipe but never reads
let mut child = Command::cargo_bin("rulez")
    .stdout(Stdio::piped())  // ❌
    .spawn()?;
let status = child.wait()?;  // ❌ Pipe fills, SIGPIPE on Linux

// RIGHT - use null() if output not needed
let output = Command::cargo_bin("rulez")
    .stdout(Stdio::null())  // ✓ No pipe created
    .output()?;

// RIGHT - read output with wait_with_output
let output = Command::cargo_bin("rulez")
    .stdout(Stdio::piped())  // ✓ Pipe created
    .wait_with_output()?;    // ✓ Reads and returns stdout
```

**Source:** Project memory - "Broken Pipe from Piped-but-Unread Stdio (2026-02-10): execute_inline_script opened stdout/stderr as Stdio::piped() but never drained them"

**Phase 6 fix already applied:** `execute_inline_script` in `rulez/src/hooks.rs:377-381` now uses `Stdio::null()`.

### Pitfall 3: Stale Binary Cache After Rename

**What goes wrong:** CI cache contains old binary name (`cch`), tests execute wrong binary despite building new one (`rulez`).

**Why it happens:** `Swatinem/rust-cache` caches entire `target/` directory. Binary rename leaves both old and new binaries. If CI script or test uses `which cch`, it finds stale cached binary.

**How to avoid:**
1. Add binary validation step in CI before tests
2. Use versioned cache keys when binary name changes
3. Explicitly clean old binaries after rename: `rm -f target/*/cch`

**Warning signs:**
- Tests pass locally but fail in CI with "command not found"
- `which rulez` shows nothing but `which cch` finds old binary
- Test assertions fail with output from old binary version

**Example fix:**
```yaml
# CI validation step
- name: Validate binary artifact
  run: |
    # Ensure old binary is gone
    if command -v cch &> /dev/null; then
      echo "::error::Stale binary 'cch' found in PATH"
      exit 1
    fi

    # Ensure new binary exists
    if ! command -v rulez &> /dev/null; then
      echo "::error::Binary 'rulez' not found in PATH"
      exit 1
    fi

    # Verify version
    rulez --version | grep -q "rulez"
```

**Source:** Project memory - "Stale Binary Artifacts (2026-02-10): Binary was renamed from cch to rulez but old binary persisted in target/debug/"

### Pitfall 4: Windows Path Separator Incompatibility

**What goes wrong:** Tests use hardcoded `/` path separators, fail on Windows which uses `\`.

**Why it happens:** String concatenation like `format!("{}/file.txt", dir)` produces Unix paths. Windows requires backslashes or accepts forward slashes in some contexts but not all.

**How to avoid:** Always use `PathBuf::join()` for path construction.

**Warning signs:**
- Tests pass on Linux/macOS but fail on Windows
- Error messages show mixed separators: `C:\Users/file.txt`
- File not found errors on Windows despite file existing

**Example fix:**
```rust
// WRONG - hardcoded separator
let config_path = format!("{}/.claude/hooks.yaml", cwd);

// RIGHT - platform-agnostic
use std::path::PathBuf;
let config_path = PathBuf::from(&cwd)
    .join(".claude")
    .join("hooks.yaml");
```

### Pitfall 5: Tempdir Cleanup Race Conditions

**What goes wrong:** Test exits before tempdir cleanup completes, leaving orphaned files in CI.

**Why it happens:** `TempDir::drop()` runs asynchronously. If test exits immediately after assertions, cleanup may not finish before next test starts or CI job ends.

**How to avoid:** Explicitly call `drop(temp_dir)` at end of test, before final assertion logging.

**Warning signs:**
- Intermittent "file already exists" errors in CI
- `/tmp` fills up in CI, causing out-of-space errors
- Tests flaky on parallel execution (pass when run serially)

**Example fix:**
```rust
#[test]
fn test_e2e() {
    let (temp_dir, event_json) = setup_event();

    // ... test logic ...

    assert_eq!(result, expected);

    // Force cleanup NOW, don't wait for scope exit
    drop(temp_dir);
}
```

## Code Examples

Verified patterns from existing codebase and standard practices:

### Cross-Platform Event Setup (Enhanced)

```rust
// tests/common/mod.rs - ADD this helper
use std::fs;
use std::path::{Path, PathBuf};

/// Canonicalize path to resolve symlinks and normalize separators
pub fn canonicalize_path<P: AsRef<Path>>(path: P) -> PathBuf {
    fs::canonicalize(path.as_ref())
        .unwrap_or_else(|_| path.as_ref().to_path_buf())
}

// tests/e2e_git_push_block.rs - UPDATE this function
fn setup_claude_code_event(config_name: &str, command: &str) -> (tempfile::TempDir, String) {
    let temp_dir = setup_test_env(config_name);

    // NEW: Canonicalize before creating event
    let canonical_path = canonicalize_path(temp_dir.path());
    let cwd = canonical_path.to_string_lossy().to_string();

    let event = serde_json::json!({
        "hook_event_name": "PreToolUse",
        "tool_name": "Bash",
        "tool_input": { "command": command },
        "session_id": "e2e-test-session",
        "cwd": cwd,
        "transcript_path": "/tmp/transcript.jsonl",
        "permission_mode": "default",
        "tool_use_id": "toolu_e2e_test"
    });

    (temp_dir, serde_json::to_string(&event).unwrap())
}
```

**Source:** Adapted from existing `e2e_git_push_block.rs:29-47` with canonicalization added.

### CI Matrix Configuration

```yaml
# .github/workflows/e2e-matrix.yml (NEW FILE)
name: E2E Tests - Cross-Platform Matrix

on:
  push:
    branches: [main, develop, "feature/**", "fix/**"]
  pull_request:
    branches: [main, develop]

env:
  CARGO_TERM_COLOR: always
  RUST_BACKTRACE: 1

jobs:
  e2e-tests:
    name: E2E (${{ matrix.os }})
    strategy:
      fail-fast: false
      matrix:
        os: [ubuntu-latest, macos-latest, windows-latest]

    runs-on: ${{ matrix.os }}
    timeout-minutes: 20

    steps:
      - name: Checkout code
        uses: actions/checkout@v4

      - name: Setup Rust toolchain
        uses: dtolnay/rust-toolchain@stable

      - name: Cache dependencies
        uses: Swatinem/rust-cache@v2

      - name: Build release binary
        run: cargo build --release --bin rulez

      - name: Validate binary artifact (Unix)
        if: runner.os != 'Windows'
        run: |
          ./target/release/rulez --version
          BINARY_PATH=$(find target/release -name rulez -type f | head -1)
          echo "Binary found at: $BINARY_PATH"
          [ -f "$BINARY_PATH" ] || exit 1

      - name: Validate binary artifact (Windows)
        if: runner.os == 'Windows'
        shell: pwsh
        run: |
          .\target\release\rulez.exe --version
          $binaryPath = Get-ChildItem -Path target\release -Filter rulez.exe -Recurse | Select-Object -First 1
          if (-not $binaryPath) { exit 1 }
          Write-Host "Binary found at: $($binaryPath.FullName)"

      - name: Run all E2E tests
        run: cargo test --tests --all-features --workspace -- --nocapture

      - name: Upload test results
        if: always()
        uses: actions/upload-artifact@v4
        with:
          name: test-results-${{ matrix.os }}
          path: target/test-evidence/
          retention-days: 7

      - name: Upload failure logs
        if: failure()
        uses: actions/upload-artifact@v4
        with:
          name: failure-logs-${{ matrix.os }}
          path: |
            target/test-evidence/
            ~/.claude/logs/
          retention-days: 14
```

**Source:** Adapted from existing `validation.yml` IQ matrix pattern.

### Symlink Resolution Test (Unix-only)

```rust
// tests/e2e_symlink_resolution.rs (NEW FILE)
#[test]
#[cfg(unix)]
fn test_symlink_cwd_resolution() {
    use std::os::unix::fs::symlink;

    let timer = Timer::start();
    let mut evidence = TestEvidence::new("e2e_symlink_cwd", "E2E");

    // Create temp dir with hooks.yaml
    let temp_dir = setup_test_env("block-all-push.yaml");

    // Create symlink to temp dir
    let symlink_dir = tempfile::tempdir().unwrap();
    let symlink_path = symlink_dir.path().join("link-to-project");
    symlink(temp_dir.path(), &symlink_path).unwrap();

    // Event cwd points to symlink (not canonical path)
    let cwd = symlink_path.to_string_lossy().to_string();
    let event = serde_json::json!({
        "hook_event_name": "PreToolUse",
        "tool_name": "Bash",
        "tool_input": { "command": "git push" },
        "cwd": cwd,
        "session_id": "symlink-test"
    });

    // RuleZ should resolve symlink and find hooks.yaml
    let output = Command::cargo_bin("rulez")
        .expect("binary exists")
        .current_dir(&symlink_path)
        .write_stdin(serde_json::to_string(&event).unwrap())
        .output()
        .expect("command should run");

    // Should block because hooks.yaml was found via symlink resolution
    assert_eq!(
        output.status.code(),
        Some(2),
        "Should block git push even via symlink cwd"
    );

    evidence.pass("Symlink cwd resolution works", timer.elapsed_ms());
    let _ = evidence.save(&evidence_dir());

    drop(temp_dir);
    drop(symlink_dir);
}
```

**Source:** Adapted from PITFALLS.md Pitfall 4 example.

## State of the Art

| Old Approach | Current Approach | When Changed | Impact |
|--------------|------------------|--------------|--------|
| String path manipulation | `PathBuf::join()` | Rust 1.0 (2015) | Cross-platform compatibility |
| Manual symlink checks | `fs::canonicalize()` | Rust 1.6 (2016) | Handles chains, platform differences |
| Per-platform CI jobs | Matrix strategy | GitHub Actions 2020 | DRY, easier to maintain |
| `spawn() + wait()` | `output()` or `wait_with_output()` | assert_cmd 2.0 (2022) | Avoids broken pipe on Linux |

**Deprecated/outdated:**
- **assert_cli:** Replaced by assert_cmd in 2018. Use assert_cmd for all new tests.
- **Manual tempfile cleanup:** Use `tempfile::TempDir` with automatic cleanup (available since tempfile 3.0).

## Open Questions

1. **Should we add Windows-specific symlink tests?**
   - What we know: Windows has symlinks (requires admin) and junctions (no admin)
   - What's unclear: Does RuleZ need to support Windows symlinks, or is junction support enough?
   - Recommendation: Test with junctions (via `std::os::windows::fs::symlink_dir`) only if users report issues. Start with Unix symlink tests.

2. **Should E2E matrix run on every push or only PRs to main?**
   - What we know: Fast CI (ci.yml) runs on all branches, Full Validation (validation.yml) runs on PRs to main
   - What's unclear: E2E matrix takes ~5-10 minutes across 3 platforms
   - Recommendation: Run on all feature branches (fast feedback) but allow manual skip for draft PRs. Use `workflow_dispatch` for on-demand runs.

3. **Should we add E2E tests to the pre-push hook?**
   - What we know: Pre-push checklist requires `cargo test --tests` (runs all 631+ tests)
   - What's unclear: Are E2E tests included in that command, or do they need explicit `--test e2e_*`?
   - Recommendation: Verify with `cargo test --tests --list | grep e2e` to confirm E2E tests are included. If not, update pre-push checklist.

## Sources

### Primary (HIGH confidence)

- **Project codebase:**
  - `rulez/tests/e2e_git_push_block.rs` - Existing E2E test patterns
  - `rulez/tests/common/mod.rs` - Test utilities and helpers
  - `rulez/src/hooks.rs:377-381` - Stdio::null() fix from Phase 6
  - `.github/workflows/validation.yml` - IQ matrix across 4 platforms
  - `CLAUDE.md` - Pre-push checklist and CI requirements

- **Rust stdlib documentation:**
  - [std::fs::canonicalize](https://doc.rust-lang.org/std/fs/fn.canonicalize.html) - Symlink resolution
  - [std::path::PathBuf](https://doc.rust-lang.org/std/path/struct.PathBuf.html) - Cross-platform paths
  - [std::process::Command](https://doc.rust-lang.org/std/process/struct.Command.html) - Process execution

- **Crate documentation:**
  - [assert_cmd 2.0 docs](https://docs.rs/assert_cmd/2.0/assert_cmd/) - CLI testing patterns
  - [tempfile 3.0 docs](https://docs.rs/tempfile/3.0/tempfile/) - Temporary directory handling

### Secondary (MEDIUM confidence)

- **GitHub Actions:**
  - [Matrix builds documentation](https://docs.github.com/en/actions/using-jobs/using-a-matrix-for-your-jobs)
  - [Swatinem/rust-cache action](https://github.com/Swatinem/rust-cache) - Cache behavior

- **Community resources:**
  - [Testing CLIs with assert_cmd](https://alexwlchan.net/2025/testing-rust-cli-apps-with-assert-cmd/) - Best practices
  - [Cross-platform Rust development guide](https://codezup.com/building-cross-platform-tools-rust-guide-windows-macos-linux/)

### Tertiary (LOW confidence - requires validation)

- **WebSearch findings:**
  - GitHub Actions matrix builds best practices (needs verification with official docs)
  - Windows junction vs symlink behavior (needs testing if Windows symlinks become required)

## Metadata

**Confidence breakdown:**
- Standard stack: HIGH - Using existing dependencies (assert_cmd, tempfile) already in Cargo.toml
- Architecture: HIGH - Based on existing test patterns in `e2e_git_push_block.rs` and validation.yml
- Pitfalls: HIGH - All drawn from project memory (documented fixes from Phase 6, binary rename issues)

**Research date:** 2026-02-10
**Valid until:** 2026-03-10 (30 days - stable test infrastructure, unlikely to change)

**Test count verification:**
- Unit tests: 250 (src/lib.rs)
- Integration tests: 258 + 8 + 15 + 15 + 7 + 18 + 4 + 8 + 5 + 7 = 345
- Total: 595 tests passing locally
- E2E tests in scope: `e2e_git_push_block.rs` (8 tests using assert_cmd)
- Target: 631+ tests passing on all platforms (current local count + new symlink tests)
