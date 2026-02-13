---
phase: 18
plan: 01
subsystem: opencode
tags: [opencode, adapter, payload, events]

# Dependency graph
requires:
  - phase: 17
    provides: E2E test baseline
provides:
  - OpenCode event capture and RuleZ payload mapping
  - cch opencode install command
  - cch opencode hook command
affects: [18-02, 18-03]

# Tech tracking
tech-stack:
  added: [OpenCode adapter]
  patterns: [Adapter-based event mapping]

key-files:
  created:
    - cch_cli/src/adapters/opencode.rs
    - cch_cli/src/cli/opencode_hook.rs
    - cch_cli/src/cli/opencode_install.rs
    - cch_cli/tests/opencode_payload_tests.rs
  modified:
    - cch_cli/src/adapters/mod.rs
    - cch_cli/src/cli.rs
    - cch_cli/src/main.rs

key-decisions:
  - "Integrated OpenCode into cch_cli crate instead of a new rulez_plugin crate to maintain consistency with existing Gemini/Copilot adapters."
  - "Mapped OpenCode lifecycle events (file.edited, tool.execute.before/after, session.updated) to internal RuleZ event types."

patterns-established:
  - "Standardized hook installation pattern for OpenCode similar to Gemini/Copilot."

# Metrics
duration: 45 min
completed: 2026-02-12
---

# Phase 18 Plan 01: OpenCode Event Capture + RuleZ Payload Mapping Summary

**OpenCode event capture and RuleZ payload mapping with installation and hook runner commands.**

## Performance

- **Duration:** 45 min
- **Tasks:** 3
- **Files created:** 4
- **Files modified:** 3

## Accomplishments
- Implemented OpenCode adapter for parsing and mapping events to RuleZ internal model.
- Created `opencode hook` command to process events from stdin.
- Created `opencode install` command for registering hooks in `.opencode/settings.json`.
- Added comprehensive unit tests for payload mapping.
- Verified compilation and passing of all tests.

## Task Commits

1. **Task 1: Add OpenCode event hook wiring**
2. **Task 2: Build RuleZ payload mapper**
3. **Task 3: Create event dispatch stub**

## Files Created/Modified
- `cch_cli/src/adapters/opencode.rs` - OpenCode adapter logic.
- `cch_cli/src/cli/opencode_hook.rs` - Hook runner command.
- `cch_cli/src/cli/opencode_install.rs` - Installation command.
- `cch_cli/tests/opencode_payload_tests.rs` - Unit tests.
- `cch_cli/src/adapters/mod.rs` - Adapter registration.
- `cch_cli/src/cli.rs` - CLI module registration.
- `cch_cli/src/main.rs` - Subcommand registration and routing.

## Decisions Made
- Chose to keep all OpenCode logic within the `cch_cli` crate for consistency with other platform adapters (Gemini, Copilot).

## Deviations from Plan
- Used `cch_cli` directory instead of `rulez_plugin` directory as it is the actual crate name in the repository.

## Next Steps
Proceed to Plan 18-02 (Policy Enforcement + Tool Registration).
