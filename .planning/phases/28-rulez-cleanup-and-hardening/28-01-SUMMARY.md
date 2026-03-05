---
phase: 28-rulez-cleanup-and-hardening
plan: "01"
subsystem: tooling
tags: [rust, regex, security, hooks, config-validation]

# Dependency graph
requires: []
provides:
  - "Fail-closed regex handling at all 5 command_match/block_if_match call sites in hooks.rs and debug.rs"
  - "get_or_compile_regex() promoted to pub(crate) for cross-module use"
  - "Config::validate() catches invalid command_match regex at startup with clear error message"
affects: [28-02, 28-03, 28-04, 28-05]

# Tech tracking
tech-stack:
  added: []
  patterns:
    - "Fail-closed regex: use get_or_compile_regex() + if let Ok / else warn + return false — never bare Regex::new()"
    - "Config validation: validate all regex fields (prompt_match and command_match) in Config::validate()"

key-files:
  created: []
  modified:
    - rulez/src/hooks.rs
    - rulez/src/cli/debug.rs
    - rulez/src/config.rs

key-decisions:
  - "Use if let Ok(regex) = get_or_compile_regex(...) / else { warn; return false } instead of match — clippy prefers if-let for two-arm match"
  - "get_or_compile_regex promoted to pub(crate) so debug.rs can call crate::hooks::get_or_compile_regex without duplicating logic"
  - "Fail-closed on invalid block_if_match: no block triggered, warning logged — not an error return since rule evaluation continues"

patterns-established:
  - "Fail-closed regex pattern: if let Ok(regex) = get_or_compile_regex(pattern, false) { ... } else { tracing::warn!(...); return false; }"
  - "Visibility rule: shared regex/matching helpers in hooks.rs should be pub(crate) to avoid duplication in cli/ submodules"

# Metrics
duration: 12min
completed: 2026-03-05
---

# Phase 28 Plan 01: Fail-Closed Regex Bug Fix Summary

**Invalid command_match regex now fails closed (rule never matches) instead of silently matching all commands; Config::validate() catches bad patterns at startup with a clear error message.**

## Performance

- **Duration:** ~12 min
- **Started:** 2026-03-05T23:23:00Z
- **Completed:** 2026-03-05T23:35:00Z
- **Tasks:** 3 completed
- **Files modified:** 3

## Accomplishments

- Fixed the CRITICAL security bug: invalid `command_match` regex in a rule no longer silently matches all commands — it now fails closed (rule does not match) with a `tracing::warn!` log entry
- Fixed all 5 call sites: 4 in `hooks.rs` (`matches_rule()`, `matches_rule_with_debug()`, `execute_rule_actions()` x2) and 1 in `cli/debug.rs` (`rule_matches_event()`)
- `get_or_compile_regex()` promoted to `pub(crate)` so `debug.rs` uses the shared, cached implementation instead of bare `regex::Regex::new()`
- `Config::validate()` now validates `command_match` regex compilation alongside `prompt_match`, so `rulez validate` catches bad patterns at startup (exit code 1, clear error message)
- Full CI pipeline passes: `cargo fmt`, `cargo clippy -D warnings`, `cargo test` (all tests pass), `cargo llvm-cov`

## Task Commits

Each task was committed atomically:

1. **Task 1: Fix 4 call sites in hooks.rs** - `e62c697` (fix)
2. **Task 2: Fix debug.rs and add command_match validation** - `952ea3b` (fix)
3. **Task 3: CI pipeline verification** - (no code change, verified inline)

## Files Created/Modified

- `rulez/src/hooks.rs` - Replaced all 4 `if let Ok(regex) = Regex::new(pattern)` with `if let Ok(regex) = get_or_compile_regex(pattern, false) { ... } else { warn; return false }`. Promoted `get_or_compile_regex` to `pub(crate)`.
- `rulez/src/cli/debug.rs` - Replaced `if let Ok(re) = regex::Regex::new(cmd_pattern)` with `crate::hooks::get_or_compile_regex` + fail-closed else branch.
- `rulez/src/config.rs` - Added `command_match` regex validation block in `Config::validate()` after `prompt_match` validation.

## Decisions Made

- Used `if let Ok(...) = get_or_compile_regex(...) { ... } else { ... }` form (not `match`) — clippy `single_match_else` lint prefers `if let` for two-arm match expressions.
- `get_or_compile_regex` promoted to `pub(crate)` (not `pub`) — debug.rs is the only external consumer, stays within crate boundary.
- For `block_if_match` invalid regex: log warning and continue (do not return an error Response) — the rule evaluation continues past the `block_if_match` check since no content was matched.

## Deviations from Plan

### Auto-fixed Issues

**1. [Rule 3 - Blocking] Rewrote match arms as if-let due to clippy single_match_else lint**

- **Found during:** Task 2 verification (cargo clippy -D warnings)
- **Issue:** Clippy with `-D warnings` rejected `match get_or_compile_regex(...) { Ok(...) => {...} Err(_) => {...} }` as `single_match_else` — must use `if let / else` form
- **Fix:** Converted all 4 new `match` blocks to `if let Ok(...) = get_or_compile_regex(...) { ... } else { ... }` form before committing Task 2
- **Files modified:** rulez/src/hooks.rs
- **Verification:** `cargo clippy --all-targets --all-features --workspace -- -D warnings` exits 0
- **Committed in:** 952ea3b (Task 2 commit, after reformatting)

---

**Total deviations:** 1 auto-fixed (Rule 3 - blocking clippy error)
**Impact on plan:** Minor syntactic change only — semantics identical. No scope creep.

## Issues Encountered

None beyond the clippy lint deviation above.

## User Setup Required

None - no external service configuration required.

## Next Phase Readiness

- All 5 unsafe `Regex::new()` call sites eliminated from `hooks.rs` and `debug.rs`
- `Config::validate()` now validates both `prompt_match` and `command_match` regex fields
- Full CI pipeline passing (fmt + clippy + test + llvm-cov)
- Ready for Phase 28 Plan 02 (debug run scripts, tool_input eval context, or globset — per roadmap order)

---
*Phase: 28-rulez-cleanup-and-hardening*
*Completed: 2026-03-05*
