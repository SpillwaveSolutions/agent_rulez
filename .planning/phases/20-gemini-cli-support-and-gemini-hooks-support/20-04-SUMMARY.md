---
phase: 20-gemini-cli-support-and-gemini-hooks-support
plan: 04
subsystem: cli
tags: [gemini, cli, hooks, diagnostics]

# Dependency graph
requires:
  - phase: 20-03
    provides: Gemini hook runner + Gemini CLI diagnostics baseline
provides:
  - Gemini hook settings install command with merge/print support
  - Diagnostics warnings for outdated cch Gemini hook commands
  - Updated Gemini CLI hooks documentation and troubleshooting
affects: [phase-21-copilot-cli-support, gemini-hooks]

# Tech tracking
tech-stack:
  added: []
  patterns: ["Gemini settings.json merge preserves non-cch hooks while refreshing cch entries"]

key-files:
  created:
    - cch_cli/src/cli/gemini_install.rs
    - cch_cli/tests/gemini_install.rs
  modified:
    - cch_cli/src/cli.rs
    - cch_cli/src/main.rs
    - cch_cli/src/cli/gemini_doctor.rs
    - cch_cli/tests/gemini_doctor.rs
    - docs/GEMINI_CLI_HOOKS.md

key-decisions:
  - "None - followed plan as specified"

patterns-established:
  - "Gemini installer removes cch hook entries before adding the cch gemini hook runner"

# Metrics
duration: 0 min
completed: 2026-02-12
---

# Phase 20 Plan 04: Gemini Install + Diagnostics Summary

**Gemini hook settings installer writes merged settings.json entries that call `cch gemini hook` and flags outdated hook commands in diagnostics.**

## Performance

- **Duration:** 0 min
- **Started:** 2026-02-12T20:45:51Z
- **Completed:** 2026-02-12T20:46:08Z
- **Tasks:** 2
- **Files modified:** 7

## Accomplishments
- Added `cch gemini install` with scope selection and JSON snippet output
- Ensured Gemini hook settings merge keeps non-cch hooks while replacing cch entries
- Improved `cch gemini doctor` and docs to warn on outdated binary usage

## Task Commits

Each task was committed atomically:

1. **Task 1: Implement Gemini install/generate command with tests** - `d51036b` (feat)
2. **Task 2: Update diagnostics and docs for outdated binary remediation** - `dcbc4ff` (feat)

**Plan metadata:** `TBD` (docs: complete plan)

## Files Created/Modified
- `cch_cli/src/cli/gemini_install.rs` - Generates and merges Gemini hook settings with cch runner commands
- `cch_cli/tests/gemini_install.rs` - Validates merge behavior and print-only output
- `cch_cli/src/cli/gemini_doctor.rs` - Flags outdated cch hook commands without `gemini hook`
- `docs/GEMINI_CLI_HOOKS.md` - Install instructions and troubleshooting for outdated binaries

## Decisions Made
None - followed plan as specified.

## Deviations from Plan

None - plan executed exactly as written.

## Issues Encountered
None.

## User Setup Required

None - no external service configuration required.

## Next Phase Readiness
Phase 20 complete, ready for Phase 21 planning.

---
*Phase: 20-gemini-cli-support-and-gemini-hooks-support*
*Completed: 2026-02-12*

## Self-Check: PASSED
