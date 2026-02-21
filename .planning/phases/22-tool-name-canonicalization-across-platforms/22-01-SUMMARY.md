# Phase 22 Plan 01 Summary

## What was done
- Fixed Rust ownership bug (E0382) in gemini.rs and copilot.rs by refactoring from `merge_tool_input()` helper to Map-first inline pattern (matching opencode.rs)
- Corrected Gemini CLI tool name mappings: `run_shell_command`/`execute_code` → Bash, `glob` → Glob, `search_file_content`/`grep_search` → Grep
- Removed unverified `run_agent` → Task mapping from Gemini adapter
- Added `webfetch` as alias alongside `fetch` for OpenCode's WebFetch mapping
- Removed now-unused `merge_tool_input()` functions from both gemini.rs and copilot.rs
- Updated all 3 adapter test files with canonical name assertions and platform_tool_name preservation checks
- Added new tests: unknown tool pass-through, no-tool-name events, specific canonicalization tests

## Files modified
- `rulez/src/adapters/gemini.rs` — Map-first pattern, corrected mappings, removed merge_tool_input
- `rulez/src/adapters/copilot.rs` — Map-first pattern, removed merge_tool_input
- `rulez/src/adapters/opencode.rs` — Added webfetch alias
- `rulez/tests/gemini_adapter.rs` — 9 tests (4 new)
- `rulez/tests/copilot_adapter.rs` — 5 tests (2 new)
- `rulez/tests/opencode_payload_tests.rs` — 5 tests (3 new)

## Verification
- `cargo fmt --all --check` — pass
- `cargo clippy --all-targets --all-features --workspace -- -D warnings` — pass
- `cargo test --tests --all-features --workspace` — all tests pass (19 adapter tests)
