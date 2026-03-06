---
phase: 25-copilot-cli-e2e-testing
plan: 01
subsystem: testing
tags: [bash, e2e, copilot, hooks, shell-adapter]

# Dependency graph
requires:
  - phase: 24-gemini-cli-e2e-testing
    provides: "gemini_adapter.sh pattern and fixture structure this mirrors"
  - phase: 21-copilot-cli-support-and-copilot-hooks-support
    provides: "Copilot hook format (.github/hooks/rulez.json, preToolUse, bash/powershell fields)"
provides:
  - "e2e/lib/copilot_adapter.sh with 4 functions: copilot_adapter_check, require_copilot_cli, setup_copilot_hooks, invoke_copilot_headless"
  - "e2e/fixtures/copilot/ with hooks-hookfire.yaml, hooks-deny.yaml, hooks-inject.yaml.template"
  - "e2e/run.sh updated to source copilot adapter and set COPILOT_CLI_AVAILABLE"
affects: [25-02-copilot-e2e-scenarios]

# Tech tracking
tech-stack:
  added: []
  patterns:
    - "CLI adapter pattern: copilot_adapter.sh mirrors gemini_adapter.sh structure"
    - "Copilot hook format: .github/hooks/rulez.json with preToolUse/bash/powershell/timeoutSec"
    - "OAuth-only check: copilot_adapter_check omits API key check (Copilot uses OAuth login)"

key-files:
  created:
    - e2e/lib/copilot_adapter.sh
    - e2e/fixtures/copilot/hooks-hookfire.yaml
    - e2e/fixtures/copilot/hooks-deny.yaml
    - e2e/fixtures/copilot/hooks-inject.yaml.template
  modified:
    - e2e/run.sh

key-decisions:
  - "copilot_adapter_check checks PATH only — no API key env var (Copilot uses OAuth login, unlike Gemini)"
  - "setup_copilot_hooks writes .github/hooks/rulez.json (not .gemini/settings.json)"
  - "Copilot hook uses preToolUse + bash/powershell fields + timeoutSec (vs Gemini BeforeTool + command + timeout ms)"
  - "invoke_copilot_headless uses --allow-all-tools (not --yolo --output-format json like Gemini)"
  - "Fixture YAML files are identical to Gemini fixtures — canonical tool names work across CLIs via RuleZ canonicalization"

patterns-established:
  - "CLI adapter pattern: each CLI has its own adapter.sh with check/require/setup_hooks/invoke_headless functions"
  - "Availability check in run.sh: per-CLI block sets CLI_AVAILABLE=1/0 and exports for scenario use"

# Metrics
duration: 2min
completed: 2026-02-23
---

# Phase 25 Plan 01: Copilot CLI E2E Adapter and Fixtures Summary

**Copilot CLI E2E adapter (copilot_adapter.sh) with OAuth-only auth check, .github/hooks/rulez.json setup using preToolUse format, and 3 fixture YAML files mirroring gemini fixture structure**

## Performance

- **Duration:** 2 min
- **Started:** 2026-02-23T20:52:59Z
- **Completed:** 2026-02-23T20:54:53Z
- **Tasks:** 2
- **Files modified:** 5

## Accomplishments
- Created `e2e/lib/copilot_adapter.sh` with 4 adapter functions mirroring gemini_adapter.sh pattern
- copilot_adapter_check does not check for any API key (Copilot uses OAuth login, unlike Gemini)
- setup_copilot_hooks writes `.github/hooks/rulez.json` using Copilot-specific format: preToolUse, bash/powershell fields, timeoutSec in seconds, version: 1 at top level
- Created 3 fixture files under `e2e/fixtures/copilot/` with identical content to gemini fixtures
- Updated `e2e/run.sh` to source copilot adapter and add COPILOT_CLI_AVAILABLE check block

## Task Commits

Each task was committed atomically:

1. **Task 1: Create copilot_adapter.sh and fixture files** - `043202c` (feat)
2. **Task 2: Update run.sh to source copilot adapter and check availability** - `709fae3` (feat)

**Plan metadata:** (docs commit below)

## Files Created/Modified
- `e2e/lib/copilot_adapter.sh` - Copilot CLI adapter with 4 functions: copilot_adapter_check, require_copilot_cli, setup_copilot_hooks, invoke_copilot_headless
- `e2e/fixtures/copilot/hooks-hookfire.yaml` - Hookfire fixture for scenario 02 (identical to gemini counterpart)
- `e2e/fixtures/copilot/hooks-deny.yaml` - Deny fixture for scenario 03 (identical to gemini counterpart)
- `e2e/fixtures/copilot/hooks-inject.yaml.template` - Inject template for scenario 04 with __WORKSPACE__ placeholder
- `e2e/run.sh` - Added copilot_adapter.sh source and COPILOT_CLI_AVAILABLE check block

## Decisions Made
- copilot_adapter_check checks PATH only — no API key (Copilot uses OAuth login, unlike Gemini which requires GEMINI_API_KEY)
- setup_copilot_hooks writes `.github/hooks/rulez.json` (not `.gemini/settings.json`)
- Copilot hook format uses preToolUse + bash/powershell fields + timeoutSec in seconds (vs Gemini BeforeTool + command field + timeout in ms)
- invoke_copilot_headless runs `copilot -p --allow-all-tools` (not `--yolo --output-format json` like Gemini)
- Fixture YAML content is identical to Gemini fixtures — canonical tool names (Bash) work for Copilot via RuleZ canonicalization

## Deviations from Plan

None - plan executed exactly as written.

## Issues Encountered
None.

## User Setup Required
None - no external service configuration required.

## Next Phase Readiness
- copilot_adapter.sh, fixture files, and run.sh integration are complete
- Plan 02 (Copilot E2E scenario scripts) can proceed immediately
- Copilot CLI scenarios will be auto-skipped when copilot is not installed (COPILOT_CLI_AVAILABLE=0)

---
*Phase: 25-copilot-cli-e2e-testing*
*Completed: 2026-02-23*
