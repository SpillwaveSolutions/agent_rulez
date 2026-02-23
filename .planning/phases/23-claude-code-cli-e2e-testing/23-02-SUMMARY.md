---
phase: 23-claude-code-cli-e2e-testing
plan: "02"
subsystem: testing
tags: [bash, e2e, claude-code, scenarios, fixtures, headless, audit-log, shell]

# Dependency graph
requires:
  - phase: 23-01
    provides: "e2e/lib/harness.sh, e2e/lib/reporting.sh, e2e/run.sh — harness framework with assertions, workspace isolation, reporting"
provides:
  - "e2e/lib/claude_adapter.sh: headless claude invocation helper and workspace settings.json generator"
  - "e2e/fixtures/claude-code/hooks-hookfire.yaml: passthrough rule fixture for log-fire scenario"
  - "e2e/fixtures/claude-code/hooks-deny.yaml: deny rule fixture blocking git force push"
  - "e2e/fixtures/claude-code/hooks-inject.yaml.template: inject_command fixture with __WORKSPACE__ placeholder"
  - "e2e/scenarios/claude-code/01-install.sh: install scenario verifying settings.json structure"
  - "e2e/scenarios/claude-code/02-hook-fire.sh: hook-fire scenario verifying PreToolUse audit log entry"
  - "e2e/scenarios/claude-code/03-deny.sh: deny scenario verifying block rule recorded in audit log"
  - "e2e/scenarios/claude-code/04-inject.sh: inject scenario verifying inject_command creates marker file"
affects:
  - 24-gemini-cli-e2e-testing
  - 25-copilot-cli-e2e-testing
  - 26-opencode-cli-e2e-testing
  - 27-codex-cli-e2e-testing

# Tech tracking
tech-stack:
  added:
    - "claude_adapter.sh: bash adapter for Claude Code CLI headless invocation"
    - "YAML fixture files for hooks: hookfire, deny (with command_match), inject (with inject_command + __WORKSPACE__)"
    - "sed-based template substitution pattern for workspace-specific paths in YAML"
  patterns:
    - "claude_adapter_check: guard function that checks claude in PATH, skips all CLI scenarios if missing"
    - "invoke_claude_headless: runs claude -p with --dangerously-skip-permissions --output-format json --max-turns 1 --allowedTools Bash --model claude-haiku-3-5"
    - "setup_claude_hooks: writes minimal .claude/settings.json with PreToolUse hook pointing at rulez binary"
    - "Audit log snapshot refresh: each scenario that invokes claude refreshes WORKSPACE_LOG_SNAPSHOT immediately before the claude invocation"
    - "Deterministic proof via audit log (not claude output): all 3 claude-invoking scenarios verify via assert_log_contains, not claude's stdout"
    - "Marker file proof for inject: assert_file_exists on workspace/e2e-inject-fired.marker (written by inject_command)"

key-files:
  created:
    - "e2e/lib/claude_adapter.sh"
    - "e2e/fixtures/claude-code/hooks-hookfire.yaml"
    - "e2e/fixtures/claude-code/hooks-deny.yaml"
    - "e2e/fixtures/claude-code/hooks-inject.yaml.template"
    - "e2e/scenarios/claude-code/01-install.sh"
    - "e2e/scenarios/claude-code/02-hook-fire.sh"
    - "e2e/scenarios/claude-code/03-deny.sh"
    - "e2e/scenarios/claude-code/04-inject.sh"
  modified:
    - "e2e/run.sh"

key-decisions:
  - "Claude adapter sourced in run.sh globally so CLAUDE_CLI_NAME and adapter functions are available to all claude-code scenarios"
  - "claude_adapter_check called in run.sh per-CLI loop before claude-code scenarios; all scenarios skipped with SKIP status if claude not found"
  - "Log snapshot refreshed inside each invoke-claude scenario (after setup_claude_hooks/fixture copy) to avoid counting hook-setup log writes as scenario entries"
  - "install scenario (01) does NOT invoke claude CLI — structural assertion only, so runs even without claude in PATH (guard is per-CLI, not per-scenario)"
  - "deny scenario asserts via audit log (not claude exit code) — Claude may return 0 even when hook denies, denial is recorded in rulez audit log"

patterns-established:
  - "scenario_<name> function sourced from numbered scripts (01-install.sh -> scenario_install), no set -e — collect all assertion failures"
  - "fixture + template pattern: hooks-inject.yaml.template uses __WORKSPACE__ placeholder replaced at test-setup time via sed"
  - "Per-CLI availability guard in run.sh loop: check CLI before sourcing/running any scenario scripts for that CLI"

# Metrics
duration: 5min
completed: "2026-02-23"
---

# Phase 23 Plan 02: Claude Code CLI Scenarios Summary

**Claude Code adapter library, 3 YAML hook fixtures, and 4 E2E scenarios (install/hook-fire/deny/inject) verifying RuleZ integration with headless claude -p invocation via audit log and marker file proof**

## Performance

- **Duration:** ~5 min
- **Started:** 2026-02-23T03:36:22Z
- **Completed:** 2026-02-23T03:41:00Z
- **Tasks:** 2
- **Files modified:** 9

## Accomplishments

- Created `e2e/lib/claude_adapter.sh` with `claude_adapter_check`, `setup_claude_hooks`, and `invoke_claude_headless` functions
- Created 3 YAML fixture files: hookfire (passthrough), deny (git force push block), inject (inject_command with __WORKSPACE__ template)
- Created 4 scenario scripts: 01-install (structural config assertion, no claude CLI), 02-hook-fire (audit log), 03-deny (block in audit log), 04-inject (marker file + audit log)
- Updated `e2e/run.sh` to source `claude_adapter.sh` globally and add per-CLI availability check that skips claude-code scenarios with SKIP status if claude not found

## Task Commits

Each task was committed atomically:

1. **Task 1: Create Claude Code adapter library and fixture files** - `523a514` (feat)
2. **Task 2: Create all 4 Claude Code E2E scenario scripts** - `bc38d71` (feat)

**Plan metadata:** (docs commit follows)

## Files Created/Modified

- `e2e/lib/claude_adapter.sh` - Adapter: claude_adapter_check, setup_claude_hooks, invoke_claude_headless, CLAUDE_CLI_NAME constant
- `e2e/fixtures/claude-code/hooks-hookfire.yaml` - Passthrough rule fixture (e2e-hookfire-log, block: false)
- `e2e/fixtures/claude-code/hooks-deny.yaml` - Deny rule fixture (e2e-deny-force-push, block: true, command_match: git push --force)
- `e2e/fixtures/claude-code/hooks-inject.yaml.template` - Inject rule template (e2e-inject-marker, inject_command with __WORKSPACE__)
- `e2e/scenarios/claude-code/01-install.sh` - scenario_install: rulez install + settings.json structural assertions
- `e2e/scenarios/claude-code/02-hook-fire.sh` - scenario_hook_fire: invoke_claude_headless + assert_log_contains e2e-hookfire-log
- `e2e/scenarios/claude-code/03-deny.sh` - scenario_deny: invoke_claude_headless + assert_log_contains e2e-deny-force-push + block
- `e2e/scenarios/claude-code/04-inject.sh` - scenario_inject: sed template sub + invoke_claude_headless + assert_file_exists marker + assert_log_contains
- `e2e/run.sh` - Added: source claude_adapter.sh; claude_adapter_check guard for claude-code CLI loop

## Decisions Made

- **Audit log as proof**: All 3 claude-invoking scenarios verify via `assert_log_contains` rather than parsing claude's stdout. This is deterministic — claude's text output is LLM-generated and unpredictable; the rulez audit log is structured and deterministic.
- **install runs without claude CLI**: The 01-install scenario only calls `rulez install` and asserts settings.json — no claude invocation. The per-CLI availability guard in run.sh skips scenarios at the CLI loop level, but install doesn't actually need claude. This is acceptable behavior (install will run if rulez exists, even if claude doesn't).
- **Snapshot refresh in each claude scenario**: After `setup_claude_hooks` and fixture copy, each scenario refreshes `WORKSPACE_LOG_SNAPSHOT` before invoking claude to avoid counting any setup-phase log writes as scenario entries.
- **No `set -e` in scenario scripts**: Scenarios collect all assertion failures rather than short-circuiting on first failure, matching the harness pattern from Plan 01.

## Deviations from Plan

None - plan executed exactly as written.

## Issues Encountered

None.

## User Setup Required

None - no external service configuration required. Note: running `task e2e` requires `claude` CLI in PATH and `ANTHROPIC_API_KEY` set. If claude is not found, all claude-code scenarios are skipped with SKIP status (not failed).

## Next Phase Readiness

- Phase 23 Plan 02 complete; all 4 Claude Code E2E scenarios ready
- `task e2e --cli claude-code` will run all 4 scenarios end-to-end (requires claude CLI + ANTHROPIC_API_KEY)
- Harness framework + claude adapter established as pattern for Phases 24-27 (Gemini, Copilot, OpenCode, Codex)
- Each subsequent phase needs: adapter lib, fixtures, and 4 scenario scripts following the same pattern

---
*Phase: 23-claude-code-cli-e2e-testing*
*Completed: 2026-02-23*

## Self-Check: PASSED

All files found. All commits found.
- FOUND: e2e/lib/claude_adapter.sh
- FOUND: e2e/fixtures/claude-code/hooks-hookfire.yaml
- FOUND: e2e/fixtures/claude-code/hooks-deny.yaml
- FOUND: e2e/fixtures/claude-code/hooks-inject.yaml.template
- FOUND: e2e/scenarios/claude-code/01-install.sh
- FOUND: e2e/scenarios/claude-code/02-hook-fire.sh
- FOUND: e2e/scenarios/claude-code/03-deny.sh
- FOUND: e2e/scenarios/claude-code/04-inject.sh
- FOUND: 523a514 (Task 1 commit)
- FOUND: bc38d71 (Task 2 commit)
