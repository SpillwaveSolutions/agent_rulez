---
phase: 23-claude-code-cli-e2e-testing
plan: "01"
subsystem: testing
tags: [bash, e2e, junit, harness, shell, taskfile]

# Dependency graph
requires: []
provides:
  - "e2e/lib/harness.sh: workspace isolation, assertions, timing, scenario runner"
  - "e2e/lib/reporting.sh: JUnit XML, ASCII table, Markdown summary generation"
  - "e2e/run.sh: main entry point discovering and running CLI scenario directories"
  - "e2e/.gitignore: excludes .runs/ from version control"
  - "task e2e: Taskfile entry point (deps: build-cli)"
affects:
  - 23-claude-code-cli-e2e-testing
  - 24-gemini-cli-e2e-testing
  - 25-copilot-cli-e2e-testing
  - 26-opencode-cli-e2e-testing
  - 27-codex-cli-e2e-testing

# Tech tracking
tech-stack:
  added:
    - "bash harness scripts (pure shell, no Node/Python dependency)"
    - "JUnit XML via bash heredoc pattern"
    - "printf ASCII table rendering"
    - "GITHUB_STEP_SUMMARY markdown integration"
  patterns:
    - "Workspace isolation via project-level .claude/settings.json in temp dir"
    - "Log snapshot + tail-n-plus approach for audit log assertion isolation"
    - "Scenario function naming: scenario_<name> sourced from numbered scripts"
    - "run_scenario orchestrator pattern: setup -> timer -> func -> record -> cleanup"

key-files:
  created:
    - "e2e/lib/harness.sh"
    - "e2e/lib/reporting.sh"
    - "e2e/run.sh"
    - "e2e/.gitignore"
  modified:
    - "Taskfile.yml"

key-decisions:
  - "Pure bash harness with no Node/Python dependencies (locked decision from CONTEXT.md)"
  - "Workspace isolation via project-level .claude/settings.json (not CLAUDE_CONFIG_DIR which doesn't exist)"
  - "Log assertion uses line-count snapshot before scenario, tails new lines after (avoids global log contamination)"
  - "Scenario function naming convention: scenario_<name> with dashes-to-underscores translation"
  - "run.sh discovers scenarios dynamically from e2e/scenarios/*/  subdirectories"
  - "task e2e depends on build-cli to ensure fresh binary before running tests"

patterns-established:
  - "harness_init: sets E2E_ROOT, RUN_ID, RUN_DIR, RULEZ_BINARY globals"
  - "setup_workspace(cli, scenario): creates .runs/<run-id>/<cli>/<scenario>/.claude/, returns path, snapshots log"
  - "assert_*: prints PASS/FAIL, increments TOTAL_PASS/TOTAL_FAIL, returns 0/1"
  - "record_result(cli, scenario, status, elapsed, msg): stores in RESULTS[], appends JUnit XML element"
  - "print_results_table: ASCII CLI x scenario matrix via printf fixed-width"

# Metrics
duration: 5min
completed: "2026-02-23"
---

# Phase 23 Plan 01: E2E Harness Framework Summary

**Bash E2E test harness with workspace isolation, JUnit XML, ASCII table reporting, and `task e2e` Taskfile entry point — reusable foundation for all 5 CLI testing phases**

## Performance

- **Duration:** ~5 min
- **Started:** 2026-02-23T03:31:43Z
- **Completed:** 2026-02-23T03:36:00Z
- **Tasks:** 2
- **Files modified:** 5

## Accomplishments

- Created `e2e/lib/harness.sh` with workspace isolation, 4 assertion helpers, timing, and scenario runner
- Created `e2e/lib/reporting.sh` with JUnit XML generation, ASCII table, and Markdown summary with GitHub Actions step summary support
- Created `e2e/run.sh` executable entry point that discovers CLI scenario directories dynamically and runs all scenarios
- Added `task e2e` to Taskfile.yml (depends on build-cli, with precondition check)
- Established `.runs/` gitignore for runtime artifacts

## Task Commits

Each task was committed atomically:

1. **Task 1: Create harness core library and workspace isolation** - `adb0c86` (feat)
2. **Task 2: Create reporting library, main entry point, and Taskfile integration** - `c834112` (feat)

**Plan metadata:** (docs commit follows)

## Files Created/Modified

- `e2e/lib/harness.sh` - Core harness: harness_init, setup_workspace, cleanup_workspace, assert_*, timer_*, run_scenario
- `e2e/lib/reporting.sh` - Reporting: reporting_init, record_result, write_junit_xml, print_results_table, write_markdown_summary
- `e2e/run.sh` - Main entry: discovers e2e/scenarios/*/**.sh, sources each, invokes run_scenario, outputs JUnit + ASCII + Markdown
- `e2e/.gitignore` - Excludes .runs/ directory
- `Taskfile.yml` - Added e2e task (deps: build-cli, runs e2e/run.sh)

## Decisions Made

- **Pure bash**: No Node/Python dependency per CONTEXT.md locked decision. JUnit XML via heredoc pattern.
- **Workspace isolation**: Project-level `.claude/settings.json` in isolated run dir (CLAUDE_CONFIG_DIR does not exist).
- **Log snapshot approach**: `WORKSPACE_LOG_SNAPSHOT` captures `wc -l rulez.log` before each scenario; assertions use `tail -n +<snapshot+1>` to read only new entries, avoiding global log contamination.
- **Dynamic scenario discovery**: `run.sh` discovers `e2e/scenarios/<cli>/*.sh` files; no hardcoded CLI list in the runner.
- **Scenario function naming**: `scenario_<name>` with dashes replaced by underscores; sourced from numbered scripts (01-install.sh -> scenario_install).
- **task e2e depends on build-cli**: Ensures fresh rulez binary before running E2E tests.

## Deviations from Plan

None - plan executed exactly as written.

## Issues Encountered

None.

## User Setup Required

None - no external service configuration required.

## Next Phase Readiness

- Harness framework complete; ready for Phase 23 Plans 02+ (Claude Code CLI scenarios)
- `e2e/scenarios/claude-code/` directory needs scenario scripts (01-install.sh, 02-hook-fire.sh, etc.)
- `e2e/fixtures/claude-code/` directory needs YAML fixture files for deny/inject scenarios
- All assertion and reporting functions are in place; next plans add CLI-specific scenario implementations

---
*Phase: 23-claude-code-cli-e2e-testing*
*Completed: 2026-02-23*
