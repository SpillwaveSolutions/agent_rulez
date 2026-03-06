---
phase: 28-rulez-cleanup-and-hardening
plan: "04"
subsystem: tooling
tags: [rust, debug, cli, json-trace, run-scripts]

requires:
  - phase: 28-01
    provides: "get_or_compile_regex fail-closed pattern used by debug.rs"
provides:
  - "JsonRuleEvaluation.script_output field for run script action results in debug JSON trace"
  - "Human-readable debug mode note when run scripts are exercised"
affects: [debug-command, rulez-ui-debug-simulator]

tech-stack:
  added: []
  patterns: ["approximate per-rule attribution from merged process_event response"]

key-files:
  created: []
  modified:
    - rulez/src/cli/debug.rs

key-decisions:
  - "Per-rule script_output is approximate — process_event() merges all matched rules into one response, so individual script output cannot be attributed per-rule"
  - "Enrichment shows block reason or inject char count, not raw script stdout"

patterns-established:
  - "Post-process_event enrichment: run process_event first, then annotate per-rule evaluations with response data"

duration: 5min
completed: 2026-03-05
---

# Phase 28 Plan 04: Debug JSON Trace Run Script Output Summary

**Enriched rulez debug --json trace with script_output field showing action results (block reason / inject size) for matched rules with run scripts**

## Performance

- **Duration:** 5 min
- **Started:** 2026-03-05T23:33:35Z
- **Completed:** 2026-03-05T23:38:16Z
- **Tasks:** 2
- **Files modified:** 1

## Accomplishments
- Added `script_output: Option<String>` field to `JsonRuleEvaluation` struct
- Enriched matched rule evaluations with action results from `process_event()` response
- Added human-readable mode note when config contains rules with `run:` scripts
- Full CI pipeline passes (fmt, clippy, test, llvm-cov)

## Task Commits

Each task was committed atomically:

1. **Task 1: Enrich JSON trace with run script output** - `65a5da2` (feat)
2. **Task 2: Run full CI pipeline** - verification only, no commit needed

**Plan metadata:** (pending)

## Files Created/Modified
- `rulez/src/cli/debug.rs` - Added script_output field, post-process_event enrichment loop, human-readable run script note

## Decisions Made
- Per-rule script_output is approximate since process_event() merges all matched rules into one response -- commented clearly in code
- Shows block reason or inject char count rather than raw script stdout (which is not available per-rule)

## Deviations from Plan

None - plan executed exactly as written.

## Issues Encountered
- Pre-existing flaky test `test_build_eval_context_tool_input_number_field` failed on first llvm-cov run but passed on second run and on normal cargo test -- no action needed (not related to this plan's changes)

## User Setup Required

None - no external service configuration required.

## Next Phase Readiness
- Debug JSON trace now includes script output info for matched rules
- Ready for remaining Phase 28 plans (globset, caching, parallel eval, log worker)

---
*Phase: 28-rulez-cleanup-and-hardening*
*Completed: 2026-03-05*
