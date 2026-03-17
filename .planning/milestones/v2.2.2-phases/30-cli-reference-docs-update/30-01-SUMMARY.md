---
phase: 30-cli-reference-docs-update
plan: 01
subsystem: docs
tags: [cli, reference, mastering-hooks, rulez]

requires:
  - phase: 29-v2-2-1-cleanup-sync-skills-cli-help-and-ui-integration
    provides: "Initial CLI command documentation additions"
provides:
  - "Accurate CLI reference for all 14 rulez commands matching --help output"
  - "Multi-CLI subcommand docs (gemini, copilot, opencode) with hook subcommand"
affects: [30-02, 30-03, mastering-hooks]

tech-stack:
  added: []
  patterns: ["Cross-check docs against binary --help output as canonical source"]

key-files:
  created: []
  modified:
    - mastering-hooks/references/cli-commands.md

key-decisions:
  - "Removed nonexistent 'run' and 'completions' commands from docs"
  - "Removed nonexistent flags (--template, --strict, --dry-run on debug, --project/--user) and replaced with actual flags from --help"
  - "Added 'hook' subcommand documentation for all three multi-CLI platforms"
  - "Removed Shell Completion section (completions command does not exist in current binary)"

patterns-established:
  - "CLI docs must be verified against rulez <cmd> --help before publishing"

requirements-completed: [CLIDOC-01]

duration: 3min
completed: 2026-03-14
---

# Phase 30 Plan 01: CLI Reference Docs Update Summary

**Updated cli-commands.md with accurate flags, descriptions, and examples for all 14 rulez CLI commands verified against --help output**

## Performance

- **Duration:** 3 min
- **Started:** 2026-03-14T21:35:53Z
- **Completed:** 2026-03-14T21:39:12Z
- **Tasks:** 2
- **Files modified:** 1

## Accomplishments
- Fixed global options section (was listing per-command flags as global; now shows only --debug-logs, --help, --version)
- Corrected 10+ incorrect flag names across init, install, uninstall, validate, debug, repl, logs commands
- Added hook subcommand docs for gemini, copilot, and opencode platforms
- Removed two nonexistent command sections (run, completions)
- Added command index table listing all 14 commands

## Task Commits

Each task was committed atomically:

1. **Task 1: Gather current CLI help output** - research only (no file changes)
2. **Task 2: Update cli-commands.md** - `c0c5e7f` (docs)

**Plan metadata:** TBD (docs: complete plan)

## Files Created/Modified
- `mastering-hooks/references/cli-commands.md` - Complete CLI reference updated to match v2.2.1 binary output

## Decisions Made
- Removed `rulez run` section -- command does not exist in current binary
- Removed Shell Completion section -- `rulez completions` command does not exist
- Replaced incorrect --project/--user flags with actual --global/-g flag on install/uninstall
- Replaced --tail with --limit/-l and --event/--rule/--status with --mode/--decision on logs command
- Added --with-examples flag to init (was missing), removed nonexistent --template flag

## Deviations from Plan

None - plan executed exactly as written.

## Issues Encountered
None.

## User Setup Required
None - no external service configuration required.

## Next Phase Readiness
- CLI reference docs are complete and accurate
- Ready for Phase 30 Plan 02 (schema and quick-reference updates)

---
*Phase: 30-cli-reference-docs-update*
*Completed: 2026-03-14*
