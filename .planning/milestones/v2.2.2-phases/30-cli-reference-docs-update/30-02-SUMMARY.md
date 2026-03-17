---
phase: 30-cli-reference-docs-update
plan: 02
subsystem: docs
tags: [hooks-yaml, schema, quick-reference, parallel-eval, globset, logging]

requires:
  - phase: 28-cleanup-hardening
    provides: "Engine features (parallel eval, config cache, globset, tool_input, regex fail-closed)"
  - phase: 29-v2-2-1-cleanup-sync-skills-cli-help-and-ui-integration
    provides: "External logging backends (OTLP, Datadog, Splunk)"
provides:
  - "Complete hooks-yaml-schema.md with v2.0-v2.2.1 engine features documented"
  - "Updated quick-reference.md with all 22 CLI commands, action types, exit codes"
affects: [mastering-hooks-skill, user-facing-docs]

tech-stack:
  added: []
  patterns: ["Engine behavior docs section in schema reference"]

key-files:
  created: []
  modified:
    - mastering-hooks/references/hooks-yaml-schema.md
    - mastering-hooks/references/quick-reference.md

key-decisions:
  - "Documented external logging in schema reference even though it lives in settings, not hooks.yaml — users need to know about it"
  - "Expanded CLI commands to 22 entries including all multi-CLI subcommands (gemini/copilot/opencode install/hook/doctor)"

patterns-established:
  - "Engine Behavior section in schema docs: internal engine details documented separately from user-facing schema fields"

requirements-completed: [CLIDOC-02, CLIDOC-03]

duration: 2min
completed: 2026-03-14
---

# Phase 30 Plan 02: Schema & Quick Reference Docs Summary

**hooks-yaml-schema.md updated with 6 engine features (parallel eval, config cache, globset, fail-closed, tool_input, external logging) and quick-reference.md expanded to 22 CLI commands with exit codes**

## Performance

- **Duration:** 2 min
- **Started:** 2026-03-14T21:35:55Z
- **Completed:** 2026-03-14T21:38:00Z
- **Tasks:** 2
- **Files modified:** 2

## Accomplishments
- hooks-yaml-schema.md now documents parallel rule evaluation (PARALLEL_THRESHOLD=10), config caching with mtime invalidation, globset matching, regex fail-closed behavior, tool_input eval context fields, and external logging backends (OTLP, Datadog, Splunk)
- quick-reference.md now lists all 22 CLI commands including test, lint, upgrade, and multi-CLI subcommands, plus inject_inline/inject_command action types, global options, and exit codes

## Task Commits

Each task was committed atomically:

1. **Task 1: Update hooks-yaml-schema.md with v2.0-v2.2.1 engine features** - `1c86551` (docs)
2. **Task 2: Update quick-reference.md with current commands, events, actions, and matchers** - `10486e2` (docs)

## Files Created/Modified
- `mastering-hooks/references/hooks-yaml-schema.md` - Added Engine Behavior section with 6 feature subsections (102 lines added)
- `mastering-hooks/references/quick-reference.md` - Expanded CLI commands table, added action types, global options, exit codes

## Decisions Made
- Documented external logging in schema reference even though it lives in settings config, not hooks.yaml -- users need a single reference for all engine capabilities
- Listed all multi-CLI subcommands individually (gemini/copilot/opencode install/hook/doctor) for discoverability

## Deviations from Plan

None - plan executed exactly as written.

## Issues Encountered
None

## User Setup Required
None - no external service configuration required.

## Next Phase Readiness
- Schema and quick-reference docs are current through v2.2.1
- Ready for remaining Phase 30 plans

---
*Phase: 30-cli-reference-docs-update*
*Completed: 2026-03-14*
