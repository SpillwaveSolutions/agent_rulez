---
phase: 21-copilot-cli-support-and-copilot-hooks-support
plan: 03
status: complete
started: 2026-02-13
completed: 2026-02-13
---

# Plan 21-03 Summary: Copilot Hook Install + Doctor + Docs

## What Was Done

Added `cch copilot install` and `cch copilot doctor` subcommands plus hook wrapper scripts and documentation.

### Task 1: Add Copilot hook install command + scripts
- Created `cch_cli/src/cli/copilot_install.rs` — installs `.github/hooks/rulez.json` with preToolUse and postToolUse entries
- Supports `--print` flag for dry-run JSON output and `--binary` for explicit binary path
- Merges with existing hook files (removes old cch entries, adds new)
- Creates wrapper scripts `scripts/copilot/rulez-pretool.sh` (bash) and `scripts/copilot/rulez-pretool.ps1` (PowerShell)

### Task 2: Add Copilot hook diagnostics + docs
- Created `cch_cli/src/cli/copilot_doctor.rs` — scans `.github/hooks/*.json` for cch hook entries
- Reports per-file status: Installed/Missing/Misconfigured/Error
- Detects outdated cch hooks (references cch but not `copilot hook`) with remediation hint
- Supports `--json` output for machine parsing
- Created `docs/COPILOT_CLI_HOOKS.md` — install steps, doctor usage, troubleshooting, response format

### Tests
- `cch_cli/tests/copilot_install.rs` — 3 tests (creates hooks, merges existing, print mode)
- `cch_cli/tests/copilot_doctor.rs` — 4 tests (installed, missing dir, misconfigured, outdated)

## Files Changed
- `cch_cli/src/cli/copilot_install.rs` (new)
- `cch_cli/src/cli/copilot_doctor.rs` (new)
- `cch_cli/src/cli.rs` (modified)
- `cch_cli/src/main.rs` (modified)
- `cch_cli/tests/copilot_install.rs` (new)
- `cch_cli/tests/copilot_doctor.rs` (new)
- `scripts/copilot/rulez-pretool.sh` (new)
- `scripts/copilot/rulez-pretool.ps1` (new)
- `docs/COPILOT_CLI_HOOKS.md` (new)

## Verification
- `cargo test copilot_install copilot_doctor` — 7 tests pass
- `cargo clippy --all-targets --all-features -- -D warnings` — clean
- Full test suite: 155 tests pass
