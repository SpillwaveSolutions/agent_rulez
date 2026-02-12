---
phase: 20-gemini-cli-support-and-gemini-hooks-support
plan: 05
subsystem: cli
tags: [gemini, hooks, diagnostics]

# Dependency graph
requires:
  - phase: 20-04
    provides: Gemini hook install + diagnostics baseline
provides:
  - Gemini doctor warnings include docs and install remediation for outdated hooks
affects: [phase-21-copilot-cli-support, gemini-hooks]

# Tech tracking
tech-stack:
  added: []
  patterns: ["Diagnostics details include docs reference and install guidance for outdated hook commands"]

key-files:
  created: []
  modified:
    - cch_cli/src/cli/gemini_doctor.rs
    - cch_cli/tests/gemini_doctor.rs

key-decisions:
  - "None - followed plan as specified"

patterns-established:
  - "Outdated hook diagnostics include a direct docs link and install hint"

# Metrics
duration: 4 min
completed: 2026-02-12
---

# Phase 20 Plan 05: Gemini Doctor Remediation Hints Summary

**Gemini doctor warnings now include docs and install guidance when outdated cch hook commands are detected.**

## Performance

- **Duration:** 4 min
- **Started:** 2026-02-12T22:22:18Z
- **Completed:** 2026-02-12T22:27:06Z
- **Tasks:** 2
- **Files modified:** 2

## Accomplishments
- Added docs/remediation hints to outdated hook diagnostics for scopes and hook files
- Extended Gemini doctor tests to assert the docs reference and install guidance

## Task Commits

Each task was committed atomically:

1. **Task 1: Add docs/remediation hint to outdated cch diagnostics** - `1d95add` (fix)
2. **Task 2: Extend Gemini doctor tests for outdated guidance** - `dd4faab` (test)

**Plan metadata:** `TBD` (docs: complete plan)

## Files Created/Modified
- `cch_cli/src/cli/gemini_doctor.rs` - Appends docs and install guidance to outdated hook details
- `cch_cli/tests/gemini_doctor.rs` - Asserts outdated details include docs reference and install hint

## Decisions Made
None - followed plan as specified.

## Deviations from Plan

### Auto-fixed Issues

**1. [Rule 3 - Blocking] Installed Rust toolchain to run tests**
- **Found during:** Task 1 (Add docs/remediation hint to outdated cch diagnostics)
- **Issue:** `cargo test -p cch gemini_doctor` failed because no default Rust toolchain was configured
- **Fix:** Ran `rustup default stable` to install and select the stable toolchain
- **Files modified:** None
- **Verification:** `cargo test -p cch gemini_doctor`
- **Committed in:** 1d95add

---

**Total deviations:** 1 auto-fixed (1 blocking)
**Impact on plan:** Required for verification only; no scope change.

## Issues Encountered
- Initial test run timed out during toolchain install; reran with longer timeout.

## User Setup Required

None - no external service configuration required.

## Next Phase Readiness
Phase 20 complete, ready for Phase 21 planning.

---
*Phase: 20-gemini-cli-support-and-gemini-hooks-support*
*Completed: 2026-02-12*

## Self-Check: PASSED
