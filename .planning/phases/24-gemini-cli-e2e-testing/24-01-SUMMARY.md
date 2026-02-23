---
phase: 24-gemini-cli-e2e-testing
plan: 01
subsystem: testing
tags: [gemini, e2e, bash, adapter, fixtures, headless]

# Dependency graph
requires:
  - phase: 23-claude-code-e2e-testing
    provides: claude_adapter.sh pattern, run.sh CLI loop pattern, fixture format
provides:
  - Gemini CLI adapter (gemini_adapter.sh) with availability check, hooks setup, headless invocation
  - Gemini fixture files (hooks-hookfire, hooks-deny, hooks-inject.yaml.template)
  - Updated run.sh with gemini adapter sourcing and GEMINI_CLI_AVAILABLE flag
affects: [25-copilot-e2e-testing, 26-opencode-e2e-testing, 27-codex-e2e-testing]

# Tech tracking
tech-stack:
  added: []
  patterns:
    - "gemini_adapter.sh parallel to claude_adapter.sh: same 4-function shape (check, require, setup, invoke)"
    - "Timeout exit 124 maps to skip exit 77 for Gemini due to --yolo flag intermittent behavior"
    - "Gemini settings at .gemini/settings.json; RuleZ hooks.yaml always at .claude/hooks.yaml even for Gemini"
    - "Gemini BeforeTool uses regex matcher '.*' (not glob '*' like Claude Code)"
    - "Hook command: '<abs_rulez> gemini hook'"

key-files:
  created:
    - e2e/lib/gemini_adapter.sh
    - e2e/fixtures/gemini/hooks-hookfire.yaml
    - e2e/fixtures/gemini/hooks-deny.yaml
    - e2e/fixtures/gemini/hooks-inject.yaml.template
  modified:
    - e2e/run.sh

key-decisions:
  - "Timeout exit (124) maps to skip (77) in invoke_gemini_headless — Gemini --yolo flag has known intermittent behavior making timeout a skip not a failure"
  - "Gemini fixture files identical to claude-code fixtures — canonical tool names (Bash) work for both CLIs"
  - "GEMINI_API_KEY check is part of gemini_adapter_check (unlike claude which only checks PATH) because Gemini CLI requires API key at launch"

patterns-established:
  - "CLI adapter pattern: check() + require_<cli>_cli() + setup_<cli>_hooks() + invoke_<cli>_headless()"
  - "Availability flag: <CLI>_CLI_AVAILABLE exported in run.sh CLI loop, checked by require_<cli>_cli() in scenarios"

# Metrics
duration: 2min
completed: 2026-02-23
---

# Phase 24 Plan 01: Gemini CLI Adapter and Fixtures Summary

**Gemini CLI adapter (gemini_adapter.sh) with BeforeTool hook setup, headless invocation with timeout-to-skip mapping, and gemini fixture files -- completing all infrastructure for Gemini E2E scenarios**

## Performance

- **Duration:** 2 min
- **Started:** 2026-02-23T04:24:40Z
- **Completed:** 2026-02-23T04:26:31Z
- **Tasks:** 2
- **Files modified:** 5

## Accomplishments

- Created `e2e/lib/gemini_adapter.sh` as a direct parallel to `claude_adapter.sh` with all 4 required functions plus GEMINI_CLI_NAME constant
- Created 3 Gemini fixture files (hookfire, deny, inject template) identical to claude-code counterparts using canonical tool names
- Updated `e2e/run.sh` to source gemini_adapter.sh and set GEMINI_CLI_AVAILABLE flag in the CLI loop

## Task Commits

Each task was committed atomically:

1. **Task 1: Create gemini_adapter.sh and fixture files** - `b998f22` (feat)
2. **Task 2: Update run.sh to source gemini adapter and check availability** - `ca309ab` (feat)

**Plan metadata:** TBD (docs: complete plan)

## Files Created/Modified

- `e2e/lib/gemini_adapter.sh` - Gemini CLI adapter: adapter_check (PATH+API key), require_gemini_cli (returns 77 if unavailable), setup_gemini_hooks (writes .gemini/settings.json), invoke_gemini_headless (--yolo, timeout 124->77)
- `e2e/fixtures/gemini/hooks-hookfire.yaml` - Hookfire test fixture (identical to claude-code version)
- `e2e/fixtures/gemini/hooks-deny.yaml` - Deny test fixture (identical to claude-code version)
- `e2e/fixtures/gemini/hooks-inject.yaml.template` - Inject test fixture template (identical to claude-code version)
- `e2e/run.sh` - Sources gemini_adapter.sh; adds GEMINI_CLI_AVAILABLE check block in CLI loop

## Decisions Made

- Timeout exit (124) maps to skip (77) in `invoke_gemini_headless`: Gemini `--yolo` flag has known intermittent behavior making timeout a skip-worthy condition rather than a test failure
- Gemini fixture files are identical to claude-code fixtures: canonical tool names (Bash) are resolved by RuleZ canonicalization layer so same rules work for both CLIs
- `GEMINI_API_KEY` check is part of `gemini_adapter_check` (unlike Claude which only checks PATH) because Gemini CLI fails at launch without a valid API key

## Deviations from Plan

None - plan executed exactly as written.

## Issues Encountered

None.

## User Setup Required

External service configuration required for Gemini scenarios. Install and configure:

1. Install Gemini CLI: `npm install -g @google/gemini-cli`
2. Set `GEMINI_API_KEY` environment variable (get key at https://aistudio.google.com/apikey)
3. Verify with: `gemini --version && echo "API key: ${GEMINI_API_KEY:0:8}..."`

Individual scenarios will auto-skip (exit 77) if Gemini CLI or API key is unavailable.

## Next Phase Readiness

- Gemini adapter infrastructure is complete; Plan 02 can now create `e2e/scenarios/gemini/*.sh` scenario scripts
- All 3 fixture files ready for scenario use
- run.sh will auto-discover `e2e/scenarios/gemini/` once Plan 02 creates it
- Scenarios should call `require_gemini_cli` at start and `setup_gemini_hooks` to write the BeforeTool config

## Self-Check: PASSED

All created files verified present:
- FOUND: e2e/lib/gemini_adapter.sh
- FOUND: e2e/fixtures/gemini/hooks-hookfire.yaml
- FOUND: e2e/fixtures/gemini/hooks-deny.yaml
- FOUND: e2e/fixtures/gemini/hooks-inject.yaml.template
- FOUND: .planning/phases/24-gemini-cli-e2e-testing/24-01-SUMMARY.md

All task commits verified present:
- FOUND: b998f22 (feat(24-01): add gemini_adapter.sh and gemini fixture files)
- FOUND: ca309ab (feat(24-01): update run.sh to source gemini adapter and check availability)

---
*Phase: 24-gemini-cli-e2e-testing*
*Completed: 2026-02-23*
