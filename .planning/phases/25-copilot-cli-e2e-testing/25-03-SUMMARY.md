---
phase: 25-copilot-cli-e2e-testing
plan: 03
subsystem: testing
tags: [bash, e2e, copilot, hooks, shell-auth, gap-closure]

# Dependency graph
requires:
  - phase: 25-copilot-cli-e2e-testing-plan-02
    provides: "4 Copilot E2E scenario scripts (01-install/02-hook-fire/03-deny/04-inject)"
provides:
  - "e2e/lib/copilot_adapter.sh: copilot_adapter_check with two-stage GitHub OAuth verification"
  - "Auth gap closed: unauthenticated Copilot installs now cause COPILOT_CLI_AVAILABLE=0 and scenarios 02-04 skip (exit 77)"
affects: [26-opencode-cli-e2e-testing, 27-codex-cli-e2e-testing]

# Tech tracking
tech-stack:
  added: []
  patterns:
    - "Two-stage auth check: gh auth status (preferred) with copilot probe fallback (no gh CLI)"
    - "Auth gate pattern: adapter_check returns 1 on unauthenticated => AVAILABLE=0 => require_* returns 77 => scenario skip"

key-files:
  created: []
  modified:
    - e2e/lib/copilot_adapter.sh

key-decisions:
  - "Stage 1: gh auth status (exit 0 = OAuth active) used when gh CLI is in PATH — reliable and fast"
  - "Stage 2: timeout 5 copilot probe used when gh absent — exit 1 = unauthenticated, exit 124/0 = authenticated (or slow)"
  - "auth check inserted after PATH check and version print, before returning 0 — no changes to require_copilot_cli, setup_copilot_hooks, or invoke_copilot_headless"

patterns-established:
  - "Adapter auth check order: PATH check => version print => auth check => success print"
  - "Analogous to gemini_adapter_check GEMINI_API_KEY check — Copilot OAuth is the equivalent auth gate"

# Metrics
duration: 1min
completed: 2026-02-23
---

# Phase 25 Plan 03: Copilot Auth Gap Closure Summary

**copilot_adapter_check now verifies GitHub OAuth state (gh auth status / probe fallback) so unauthenticated Copilot installs set COPILOT_CLI_AVAILABLE=0 and scenarios 02-04 skip cleanly instead of failing**

## Performance

- **Duration:** 1 min
- **Started:** 2026-02-23T22:52:10Z
- **Completed:** 2026-02-23T22:53:30Z
- **Tasks:** 1
- **Files modified:** 1

## Accomplishments
- Added two-stage authentication check to `copilot_adapter_check` in `e2e/lib/copilot_adapter.sh`
- Stage 1: `gh auth status` when `gh` CLI is present (reliable GitHub OAuth check)
- Stage 2: `timeout 5 copilot -p "ping" --allow-all-tools` probe when `gh` absent (exit 1 = unauthenticated, exit 0/124 = authenticated)
- Unauthenticated path: prints clear error with "Run: copilot auth login" hint, returns 1
- All other adapter functions (`require_copilot_cli`, `setup_copilot_hooks`, `invoke_copilot_headless`) unchanged

## Task Commits

Each task was committed atomically:

1. **Task 1: Add auth check to copilot_adapter_check** - `3fb6750` (fix)

**Plan metadata:** (docs commit below)

## Files Created/Modified
- `e2e/lib/copilot_adapter.sh` - copilot_adapter_check now checks GitHub OAuth state after PATH/version check; updated function header comment to reflect auth verification

## Decisions Made
- `gh auth status` used as Stage 1 (preferred) — same OAuth used by Copilot CLI, zero-cost check, reliable
- Probe fallback (`timeout 5 copilot -p "ping"`) used when `gh` absent — exit 1 maps to unauthenticated, exit 124 (timeout) maps to authenticated-but-slow (continue)
- Only `copilot_adapter_check` modified — other functions unchanged per plan requirement

## Deviations from Plan

None - plan executed exactly as written.

## Issues Encountered
- None. On this machine `gh auth status` returns 0 (user is authenticated), so `copilot_adapter_check` returns 0 correctly. The unauthenticated path was code-reviewed for correctness (non-zero `gh auth status` => return 1 with error).

## User Setup Required
None - no external service configuration required.

## Next Phase Readiness
- Auth gap in Phase 25 fully closed
- `copilot_adapter_check` now handles unauthenticated machines gracefully: scenarios 02-04 skip instead of fail
- Phase 26 (OpenCode CLI E2E Testing) and Phase 27 (Codex CLI E2E Testing) can proceed with same adapter + auth check pattern

---
*Phase: 25-copilot-cli-e2e-testing*
*Completed: 2026-02-23*

## Self-Check: PASSED

- e2e/lib/copilot_adapter.sh: FOUND
- .planning/phases/25-copilot-cli-e2e-testing/25-03-SUMMARY.md: FOUND
- Commit 3fb6750 (task 1): FOUND
