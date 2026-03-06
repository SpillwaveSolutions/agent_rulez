---
phase: 28-rulez-cleanup-and-hardening
plan: "06"
subsystem: rulez-core
tags: [cli, upgrade, self-update, github-releases]
dependency_graph:
  requires: []
  provides: [rulez-upgrade-subcommand]
  affects: [rulez/src/main.rs, rulez/src/cli/upgrade.rs, rulez/Cargo.toml]
tech_stack:
  added: [self_update@0.40]
  patterns: [github-releases-api, binary-self-replace, semver-comparison]
key_files:
  created:
    - rulez/src/cli/upgrade.rs
  modified:
    - rulez/Cargo.toml
    - rulez/src/cli.rs
    - rulez/src/main.rs
decisions:
  - GitHub owner/repo values are placeholders (SpillwaveSolutions/agent_rulez) — must be updated when repo is made public with releases
  - Using self_update crate (industry standard for Rust binary self-upgrade) over manual reqwest approach
  - --check flag prints version info and exits 0/1 without installing — safe for use in CI/automation
metrics:
  duration: 5 min
  completed: 2026-03-05
  tasks_completed: 2
  files_modified: 4
---

# Phase 28 Plan 06: Upgrade Subcommand Summary

## One-liner

`rulez upgrade` subcommand added using the self_update crate for GitHub releases-based binary self-upgrade with `--check` flag for version comparison only.

## What Was Built

Added `rulez upgrade` and `rulez upgrade --check` subcommands to the RuleZ CLI. The upgrade command uses the `self_update` crate (0.40) to check for newer releases on GitHub and optionally download and install the latest binary.

- `rulez upgrade --check` — prints current version, checks GitHub releases, prints latest version and whether an upgrade is available. Exits 0 if already current, exits cleanly with message if no releases found.
- `rulez upgrade` — performs the check and then downloads/installs if a newer version is available.

The GitHub owner/repo values (`SpillwaveSolutions`/`agent_rulez`) are placeholders — they must match the actual public GitHub release repo. If the repo is private or has no releases, `--check` prints "No releases found" and exits cleanly (no panic, no error).

## Tasks Completed

| Task | Name | Commit | Files |
|------|------|--------|-------|
| 1 | Add self_update dependency and create upgrade.rs | 9fc4099 | rulez/Cargo.toml, rulez/src/cli/upgrade.rs, rulez/src/cli.rs |
| 2 | Wire upgrade subcommand into main.rs and run full CI | aa6ede3 | rulez/src/main.rs, rulez/src/cli/upgrade.rs (fmt fix) |

## Verification Results

- `./target/debug/rulez --help | grep -i upgrade` — "upgrade    Check for and install newer rulez binary releases"
- `./target/debug/rulez upgrade --help` — shows `--check` flag
- `cargo fmt --all --check` — passes
- `cargo clippy --all-targets --all-features --workspace -- -D warnings` — passes (0 warnings)
- `cargo test --tests --all-features --workspace` — all tests pass
- `cargo llvm-cov --all-features --workspace --no-report` — passes

## Deviations from Plan

### Auto-fixed Issues

**1. [Rule 1 - Bug] Formatting issue in upgrade.rs**
- **Found during:** Task 2 (cargo fmt --all --check)
- **Issue:** `println!("Upgrade available: {} -> {}", current_version, latest_version)` exceeded line width limit
- **Fix:** Applied `cargo fmt --all` to reformat to multi-line println! macro
- **Files modified:** rulez/src/cli/upgrade.rs
- **Commit:** aa6ede3 (included in Task 2 commit)

## Self-Check: PASSED

Files verified:
- FOUND: rulez/src/cli/upgrade.rs
- FOUND: 9fc4099 (Task 1 commit)
- FOUND: aa6ede3 (Task 2 commit)

All CI steps verified passing at completion time.
