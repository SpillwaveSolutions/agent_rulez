---
phase: 31-multi-cli-usage-guides
plan: 02
subsystem: docs
tags: [gemini, opencode, guides, dual-fire, plugin]

requires:
  - phase: 30-cli-reference-docs-update
    provides: "Updated CLI commands reference and schema docs"
provides:
  - "Gemini CLI usage guide with dual-fire events documentation"
  - "OpenCode usage guide with plugin setup documentation"
affects: [multi-cli-usage-guides]

tech-stack:
  added: []
  patterns: ["platform-specific usage guides with cross-references to shared docs"]

key-files:
  created:
    - docs/guides/gemini-cli-guide.md
    - docs/guides/opencode-guide.md
  modified: []

key-decisions:
  - "Structured both guides with identical section ordering for consistency"
  - "Dual-fire events given dedicated section in Gemini guide due to platform-specific importance"

patterns-established:
  - "Usage guide pattern: overview, prereqs, quick start, platform-specific section, event mapping, config, verify, troubleshoot, further reading"

requirements-completed: [GUIDE-02, GUIDE-03]

duration: 3min
completed: 2026-03-14
---

# Phase 31 Plan 02: Gemini CLI and OpenCode Usage Guides Summary

**End-to-end usage guides for Gemini CLI (dual-fire events, 283 lines) and OpenCode (plugin setup, 369 lines) covering install through troubleshooting**

## Performance

- **Duration:** 3 min
- **Started:** 2026-03-14T22:54:16Z
- **Completed:** 2026-03-14T22:57:00Z
- **Tasks:** 2
- **Files modified:** 2

## Accomplishments
- Created comprehensive Gemini CLI guide with dedicated dual-fire events section explaining BeforeAgent, AfterTool, and Notification dual-fire scenarios
- Created comprehensive OpenCode guide with plugin setup section covering config, env vars, and settings.json format
- Both guides use `rulez` binary name throughout with no old `cch` references
- Event mapping tables match platform-adapters.md reference

## Task Commits

Each task was committed atomically:

1. **Task 1: Create Gemini CLI usage guide** - `9fb4977` (docs)
2. **Task 2: Create OpenCode usage guide** - `f07d1f2` (docs)

## Files Created/Modified
- `docs/guides/gemini-cli-guide.md` - Complete Gemini CLI usage guide (283 lines)
- `docs/guides/opencode-guide.md` - Complete OpenCode usage guide (369 lines)

## Decisions Made
- Structured both guides with consistent section ordering for user familiarity across platforms
- Gave dual-fire events a dedicated prominent section in Gemini guide since it is the most critical Gemini-specific concept
- Included raw stdin testing example in OpenCode guide for direct hook runner debugging

## Deviations from Plan

None - plan executed exactly as written.

## Issues Encountered
None

## User Setup Required
None - no external service configuration required.

## Next Phase Readiness
- Both guides ready for user consumption
- Cross-reference links point to existing platform-adapters.md and cli-commands.md

---
*Phase: 31-multi-cli-usage-guides*
*Completed: 2026-03-14*
