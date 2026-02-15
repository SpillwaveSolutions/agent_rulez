---
phase: 21-copilot-cli-support-and-copilot-hooks-support
plan: 02
status: complete
started: 2026-02-13
completed: 2026-02-13
---

# Plan 21-02 Summary: Copilot Hook Runner Subcommand

## What Was Done

Added `cch copilot hook` subcommand that reads Copilot hook JSON from stdin and emits a single-line Copilot JSON response.

### Task 1: Implement `cch copilot hook` runner
- Created `cch_cli/src/cli/copilot_hook.rs` mirroring the Gemini hook runner pattern
- Reads stdin JSON, parses via `adapters::copilot::parse_event`, loads project config from event cwd
- Calls `hooks::process_event` and translates response via `adapters::copilot::translate_response`
- On any error, emits a safe Allow response with reason to stdout and logs error to stderr
- Wired `CopilotSubcommand::Hook` in `main.rs` and module export in `cli.rs`

### Task 2: Add copilot hook runner tests
- Created `cch_cli/tests/copilot_hook_runner.rs` with 3 integration tests:
  - `copilot_hook_runner_outputs_allow_json` — valid preToolUse payload returns allow
  - `copilot_hook_runner_denies_blocked_tool` — blocking rule returns deny with reason
  - `copilot_hook_runner_handles_empty_stdin` — empty input returns safe allow

## Files Changed
- `cch_cli/src/cli/copilot_hook.rs` (new)
- `cch_cli/src/cli.rs` (modified — added copilot modules)
- `cch_cli/src/main.rs` (modified — added Copilot subcommand enum and dispatch)
- `cch_cli/tests/copilot_hook_runner.rs` (new)

## Verification
- `cargo test copilot_hook_runner` — 3 tests pass
- `cargo clippy --all-targets --all-features -- -D warnings` — clean
