---
phase: 29-v2-2-1-cleanup-sync-skills-cli-help-and-ui-integration
plan: 02
subsystem: docs, ui
tags: [cli-reference, mastering-hooks, tauri, react, zustand, config-diff]

# Dependency graph
requires:
  - phase: 29-v2-2-1-cleanup-sync-skills-cli-help-and-ui-integration
    provides: "Phase 34 ConfigDiffView component, v2.2 CLI commands (test, lint, upgrade, platform install/doctor)"
provides:
  - "Complete CLI command reference in mastering-hooks skill (17+ commands)"
  - "ConfigDiffView accessible via Diff button in UI header"
affects: [mastering-hooks, rulez-ui]

# Tech tracking
tech-stack:
  added: []
  patterns: ["View switcher pattern extended with diff view"]

key-files:
  created: []
  modified:
    - mastering-hooks/references/cli-commands.md
    - rulez-ui/src/stores/uiStore.ts
    - rulez-ui/src/components/layout/Header.tsx
    - rulez-ui/src/components/layout/MainContent.tsx

key-decisions:
  - "Grouped platform commands under Multi-Platform Commands heading for clarity"
  - "Included doctor commands alongside install commands (9 total new entries, not just 7)"

patterns-established:
  - "View routing: add to MainView type, add button in Header, add conditional in MainContent"

requirements-completed: [CLEANUP-03, CLEANUP-04]

# Metrics
duration: 5min
completed: 2026-03-12
---

# Phase 29 Plan 02: CLI Docs and UI Diff View Summary

**Added 9 missing CLI commands to mastering-hooks reference and wired ConfigDiffView into UI navigation via Diff button**

## Performance

- **Duration:** 5 min
- **Started:** 2026-03-12T23:51:32Z
- **Completed:** 2026-03-12T23:56:32Z
- **Tasks:** 2
- **Files modified:** 4

## Accomplishments
- Documented rulez test, lint, upgrade commands with accurate flags from Rust source
- Documented gemini/copilot/opencode install and doctor commands (6 platform commands)
- Wired existing ConfigDiffView into UI with Editor | Logs | Diff view switcher
- TypeScript compiles cleanly with all changes

## Task Commits

Each task was committed atomically:

1. **Task 1: Add missing CLI commands to mastering-hooks cli-commands.md** - `c750400` (docs)
2. **Task 2: Wire ConfigDiffView into UI navigation** - `dfd6e32` (feat)

## Files Created/Modified
- `mastering-hooks/references/cli-commands.md` - Added 9 new command entries (test, lint, upgrade, gemini install/doctor, copilot install/doctor, opencode install/doctor)
- `rulez-ui/src/stores/uiStore.ts` - Extended MainView type with "diff"
- `rulez-ui/src/components/layout/Header.tsx` - Added Diff button to view switcher
- `rulez-ui/src/components/layout/MainContent.tsx` - Added diff view routing to ConfigDiffView

## Decisions Made
- Grouped platform commands under "Multi-Platform Commands" heading for clarity
- Included doctor commands alongside install commands (9 total new entries vs 7 planned) -- doctor commands were in the CLI source and logically belong with install docs

## Deviations from Plan

None - plan executed exactly as written.

## Issues Encountered
None

## User Setup Required
None - no external service configuration required.

## Next Phase Readiness
- CLI documentation now covers all rulez commands
- ConfigDiffView is accessible in the UI
- Ready for next plan in phase

---
*Phase: 29-v2-2-1-cleanup-sync-skills-cli-help-and-ui-integration*
*Completed: 2026-03-12*
