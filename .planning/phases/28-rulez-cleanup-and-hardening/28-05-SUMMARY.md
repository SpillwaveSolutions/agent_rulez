---
phase: 28-rulez-cleanup-and-hardening
plan: "05"
subsystem: tooling
tags: [globset, glob, directory-matching, rust]

requires:
  - phase: 28-03
    provides: "config cache and hooks.rs build_eval_context changes"
  - phase: 28-04
    provides: "debug.rs with script_output field in JsonRuleEvaluation"
provides:
  - "globset-based directory matching in hooks.rs and debug.rs"
  - "pub(crate) build_glob_set() helper for reuse across modules"
  - "false-positive elimination for directory glob patterns"
affects: [hooks, debug, directory-matching]

tech-stack:
  added: [globset 0.4]
  patterns: [GlobSet compiled from pattern list, OR-semantics matching]

key-files:
  created: []
  modified:
    - rulez/Cargo.toml
    - rulez/src/hooks.rs
    - rulez/src/cli/debug.rs

key-decisions:
  - "build_glob_set() auto-appends /** suffix for bare directory names (e.g., src/ becomes src/** too) to match files recursively"
  - "Invalid glob patterns logged as warnings and skipped (fail-open per pattern, fail-closed overall via empty GlobSet)"
  - "GlobSet compiled per evaluation call -- config cache from 28-03 ensures config itself is cached"

patterns-established:
  - "Directory matching via globset::GlobSet instead of String::contains()"
  - "build_glob_set() as shared pub(crate) helper for hooks.rs and debug.rs"

duration: 5min
completed: 2026-03-05
---

# Phase 28 Plan 05: Globset Directory Matching Summary

**Replace contains() directory matching hack with globset crate -- eliminates false positives like src/ matching /other/src/foo.rs**

## Performance

- **Duration:** 5 min
- **Started:** 2026-03-05T23:41:03Z
- **Completed:** 2026-03-05T23:45:33Z
- **Tasks:** 2
- **Files modified:** 3

## Accomplishments
- Replaced `contains(trim_end_matches())` hack with proper `globset::GlobSet` matching in both `hooks.rs` and `debug.rs`
- Added `build_glob_set()` pub(crate) helper that handles bare directory names, wildcard patterns, and invalid patterns gracefully
- Added 3 unit tests: false-positive prevention, wildcard patterns, empty pattern set
- Full CI pipeline passes (fmt, clippy, test, llvm-cov)

## Task Commits

Each task was committed atomically:

1. **Task 1: Add globset dependency and replace directory matching in hooks.rs** - `9167d98` (feat)
2. **Task 2: Replace directory matching in debug.rs and run full CI** - `521b460` (feat)

## Files Created/Modified
- `rulez/Cargo.toml` - Added globset = "0.4" dependency
- `rulez/src/hooks.rs` - Added build_glob_set() helper, replaced contains() in matches_rule() and matches_rule_with_debug(), added 3 glob tests
- `rulez/src/cli/debug.rs` - Replaced contains() with crate::hooks::build_glob_set() in rule_matches_event()

## Decisions Made
- build_glob_set() auto-appends `/**` suffix for bare directory names so `src/` matches `src/main.rs` and `src/lib/utils.rs`
- Invalid glob patterns are logged as warnings and skipped (individual fail-open, overall fail-closed via empty GlobSet)
- GlobSet compiled per evaluation call; config-level caching handled by 28-03's mtime cache

## Deviations from Plan

None - plan executed exactly as written.

## Issues Encountered
None

## User Setup Required
None - no external service configuration required.

## Next Phase Readiness
- Directory matching now uses proper glob semantics across both live evaluation and debug CLI
- Ready for Phase 28 Plan 08 (remaining cleanup tasks)

---
*Phase: 28-rulez-cleanup-and-hardening*
*Completed: 2026-03-05*
