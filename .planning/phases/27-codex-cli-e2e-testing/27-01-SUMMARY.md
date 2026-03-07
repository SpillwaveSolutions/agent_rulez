---
phase: 27-codex-cli-e2e-testing
plan: 01
subsystem: testing
tags: [codex, e2e, bash, cli-adapter, openai]

# Dependency graph
requires:
  - phase: 23-claude-code-e2e-testing
    provides: E2E harness framework (run.sh, harness.sh, reporting.sh)
  - phase: 26-opencode-cli-e2e-testing
    provides: OpenCode adapter template pattern
provides:
  - Codex CLI adapter (codex_adapter.sh) with 4 exported functions
  - 4 Codex E2E scenario scripts (install, hook-fire, deny, inject)
  - 3 Codex fixture files (hookfire, deny, inject)
  - run.sh integration for Codex CLI discovery
affects: []

# Tech tracking
tech-stack:
  added: []
  patterns:
    - "Unconditional skip pattern for CLIs without hook support"
    - "Stub setup_hooks for CLIs without hook integration"

key-files:
  created:
    - e2e/lib/codex_adapter.sh
    - e2e/scenarios/codex/01-install.sh
    - e2e/scenarios/codex/02-hook-fire.sh
    - e2e/scenarios/codex/03-deny.sh
    - e2e/scenarios/codex/04-inject.sh
    - e2e/fixtures/codex/hooks-hookfire.yaml
    - e2e/fixtures/codex/hooks-deny.yaml
    - e2e/fixtures/codex/hooks-inject.yaml.template
  modified:
    - e2e/run.sh

key-decisions:
  - "No rulez codex install subcommand -- adapter handles workspace setup directly"
  - "Scenarios 02-04 skip unconditionally with informative messages about missing hook support"
  - "setup_codex_hooks writes .codex/config.toml stub (not hook integration)"
  - "invoke_codex_headless uses codex exec with --ask-for-approval never --json flags"

patterns-established:
  - "Unconditional skip pattern: print descriptive skip messages, return 77"
  - "Stub adapter: setup_hooks creates minimal CLI config without hook integration"

requirements-completed: []

# Metrics
duration: 2min
completed: 2026-03-06
---

# Phase 27 Plan 01: Codex CLI E2E Testing Summary

**Codex CLI adapter with 4 E2E scenarios -- install passes, hook-dependent scenarios skip with clear messages**

## Performance

- **Duration:** 2 min
- **Started:** 2026-03-06T22:58:51Z
- **Completed:** 2026-03-06T23:00:32Z
- **Tasks:** 2
- **Files modified:** 9

## Accomplishments
- Created codex_adapter.sh with check, require, setup_hooks, invoke_headless functions
- Created 4 scenario scripts: 01-install validates adapter detection; 02-04 skip unconditionally
- Created 3 fixture files matching established format (identical to opencode fixtures)
- Integrated codex adapter into run.sh with source line and CODEX_CLI_AVAILABLE check
- Verified end-to-end: `./e2e/run.sh --cli codex` produces 1 PASS, 3 SKIP, 0 FAIL

## Task Commits

Each task was committed atomically:

1. **Task 1: Create Codex adapter, fixtures, and 4 scenario scripts** - `53736ca` (feat)
2. **Task 2: Integrate codex adapter into run.sh** - `8feda81` (feat)

## Files Created/Modified
- `e2e/lib/codex_adapter.sh` - Codex CLI adapter with check, require, setup_hooks, invoke_headless
- `e2e/scenarios/codex/01-install.sh` - Install scenario: adapter detection + workspace config
- `e2e/scenarios/codex/02-hook-fire.sh` - Hook-fire scenario: unconditional skip (no hook support)
- `e2e/scenarios/codex/03-deny.sh` - Deny scenario: unconditional skip (no hook support)
- `e2e/scenarios/codex/04-inject.sh` - Inject scenario: unconditional skip (no hook support)
- `e2e/fixtures/codex/hooks-hookfire.yaml` - Hookfire fixture for future use
- `e2e/fixtures/codex/hooks-deny.yaml` - Deny fixture for future use
- `e2e/fixtures/codex/hooks-inject.yaml.template` - Inject fixture template for future use
- `e2e/run.sh` - Added codex adapter source and CODEX_CLI_AVAILABLE check block

## Decisions Made
- No `rulez codex install` subcommand -- adapter handles workspace setup directly via setup_codex_hooks
- Scenarios 02-04 skip unconditionally with informative "no hook support" messages
- setup_codex_hooks writes `.codex/config.toml` with model and approval_policy settings (stub, not hook integration)
- invoke_codex_headless uses `codex exec "${prompt}" --ask-for-approval never --json` for headless mode

## Deviations from Plan

None - plan executed exactly as written.

## Issues Encountered
None

## User Setup Required
None - no external service configuration required.

## Next Phase Readiness
- Codex E2E testing complete with graceful skip pattern for hook-dependent scenarios
- When Codex CLI adds hook support, enable scenarios 02-04 by replacing unconditional skip with actual hook setup and invocation
- All 5 CLIs now have E2E scenario coverage (Claude Code, Gemini, Copilot, OpenCode, Codex)

## Self-Check: PASSED

All 9 files verified present. Both commit hashes (53736ca, 8feda81) confirmed in git log.

---
*Phase: 27-codex-cli-e2e-testing*
*Completed: 2026-03-06*
