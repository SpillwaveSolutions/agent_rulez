---
phase: 24-gemini-cli-e2e-testing
plan: 02
subsystem: testing
tags: [gemini, e2e, bash, scenarios, headless, skip-77]

# Dependency graph
requires:
  - phase: 24-01
    provides: gemini_adapter.sh, gemini fixture files, run.sh gemini integration
provides:
  - Gemini E2E scenario 01-install (structural, no CLI needed)
  - Gemini E2E scenario 02-hook-fire (BeforeTool hook fires, audit log assertion)
  - Gemini E2E scenario 03-deny (deny rule blocks tool call, audit log assertion)
  - Gemini E2E scenario 04-inject (inject_command creates marker file)
affects: [25-copilot-e2e-testing, 26-opencode-e2e-testing, 27-codex-e2e-testing]

# Tech tracking
tech-stack:
  added: []
  patterns:
    - "All 4 gemini scenarios mirror claude-code counterparts with gemini_adapter.sh functions"
    - "Scenarios 02-04 call require_gemini_cli at top; return 77 if GEMINI_CLI_AVAILABLE=0"
    - "Scenario 01 (install) is purely structural — no gemini CLI invocation, always runs"
    - "RuleZ policy config always at .claude/hooks.yaml; gemini hook config at .gemini/settings.json"
    - "Log snapshot refreshed after setup_gemini_hooks to avoid counting hook-setup log writes"

key-files:
  created:
    - e2e/scenarios/gemini/01-install.sh
    - e2e/scenarios/gemini/02-hook-fire.sh
    - e2e/scenarios/gemini/03-deny.sh
    - e2e/scenarios/gemini/04-inject.sh
  modified: []

key-decisions:
  - "Install scenario (01) uses --scope project --binary flags: required to scope to workspace and locate binary"
  - "Scenarios 02-04 all call mkdir -p .claude before cp hooks.yaml: ensures dir exists even in fresh workspaces"
  - "All scenarios follow exact claude-code counterpart structure for consistency and maintainability"

# Metrics
duration: 2min
completed: 2026-02-23
---

# Phase 24 Plan 02: Gemini E2E Scenarios Summary

**4 Gemini E2E scenario scripts (install, hook-fire, deny, inject) mirroring claude-code scenarios — all use gemini_adapter.sh functions, assert via audit log, and skip gracefully (exit 77) when Gemini CLI unavailable**

## Performance

- **Duration:** 2 min
- **Started:** 2026-02-23T04:28:52Z
- **Completed:** 2026-02-23T04:30:22Z
- **Tasks:** 2
- **Files modified:** 4

## Accomplishments

- Created `e2e/scenarios/gemini/01-install.sh`: structural test for `rulez gemini install --scope project`; asserts `.gemini/settings.json` created with `BeforeTool` and `command` fields; no gemini CLI required
- Created `e2e/scenarios/gemini/02-hook-fire.sh`: headless test; skips (77) if gemini unavailable; asserts audit log contains hookfire rule name after BeforeTool hook fires
- Created `e2e/scenarios/gemini/03-deny.sh`: headless test; skips (77) if gemini unavailable; asserts audit log contains deny rule name and block action
- Created `e2e/scenarios/gemini/04-inject.sh`: headless test; skips (77) if gemini unavailable; uses sed to substitute `__WORKSPACE__` in inject template; asserts marker file created and audit log contains inject rule name

## Task Commits

Each task was committed atomically:

1. **Task 1: Create install and hook-fire scenarios** - `2d67d83` (feat)
2. **Task 2: Create deny and inject scenarios** - `ad18973` (feat)

**Plan metadata:** TBD (docs: complete plan)

## Files Created/Modified

- `e2e/scenarios/gemini/01-install.sh` - Install scenario: `rulez gemini install --scope project --binary`; asserts `.gemini/settings.json` with `BeforeTool` + `command`; no gemini CLI needed
- `e2e/scenarios/gemini/02-hook-fire.sh` - Hook-fire scenario: `setup_gemini_hooks` + hookfire fixture at `.claude/hooks.yaml`; invokes `invoke_gemini_headless`; asserts `e2e-hookfire-log` in audit log
- `e2e/scenarios/gemini/03-deny.sh` - Deny scenario: deny fixture at `.claude/hooks.yaml`; invokes gemini with force push prompt; asserts `e2e-deny-force-push` + `block` in audit log
- `e2e/scenarios/gemini/04-inject.sh` - Inject scenario: sed-substituted inject template at `.claude/hooks.yaml`; asserts `e2e-inject-fired.marker` file and `e2e-inject-marker` in audit log

## Decisions Made

- Install scenario (01) uses `--scope project --binary` flags: `--scope project` scopes to workspace `.gemini/settings.json`, `--binary` ensures correct rulez path in command field
- Scenarios 02-04 all call `mkdir -p .claude` before copying hooks.yaml: ensures directory exists even in fresh workspaces created by test harness
- All 4 scenarios follow exact claude-code counterpart structure for consistency and maintainability across CLIs

## Deviations from Plan

None - plan executed exactly as written.

## Issues Encountered

None.

## User Setup Required

Gemini scenarios 02-04 require:
1. Install Gemini CLI: `npm install -g @google/gemini-cli`
2. Set `GEMINI_API_KEY` environment variable (get key at https://aistudio.google.com/apikey)
3. Scenario 01 (install) runs regardless of Gemini CLI availability

Individual scenarios will auto-skip (exit 77) if Gemini CLI or API key is unavailable.

## Next Phase Readiness

- All 4 Gemini E2E scenarios are complete; `run.sh --cli gemini` will discover them via `e2e/scenarios/gemini/*.sh` glob
- Phase 24 is now fully complete (both plans done)
- Phase 25 (Copilot E2E testing) can follow the same adapter + scenarios pattern

## Self-Check: PASSED

All created files verified present:
- FOUND: e2e/scenarios/gemini/01-install.sh
- FOUND: e2e/scenarios/gemini/02-hook-fire.sh
- FOUND: e2e/scenarios/gemini/03-deny.sh
- FOUND: e2e/scenarios/gemini/04-inject.sh

All task commits verified present:
- FOUND: 2d67d83 (feat(24-02): add gemini scenarios 01-install and 02-hook-fire)
- FOUND: ad18973 (feat(24-02): add gemini scenarios 03-deny and 04-inject)

---
*Phase: 24-gemini-cli-e2e-testing*
*Completed: 2026-02-23*
