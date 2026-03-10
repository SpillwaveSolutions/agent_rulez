# Phase 15 Plan 01 Summary: CLI --json Flag + Full Event Type Support

**Status:** ✅ Complete
**Completed:** 2026-02-12
**Milestone:** v1.6 RuleZ UI

## What Was Done

### JSON Output Mode
- Added `--json` flag to CLI `debug` subcommand
- JSON output emits structured data: outcome, reason, matchedRules, evaluationTimeMs, evaluations array
- Uses `std::time::Instant` for per-rule timing

### Full Event Type Support
- Expanded `SimEventType` to support all 7 event types: PreToolUse, PostToolUse, UserPromptSubmit, SessionEnd, SessionStart, PreCompact, Notification

## Files Changed
- `rulez/src/main.rs` — CLI arg additions
- `rulez/src/cli/debug.rs` — JSON output mode, full event type support

## Success Criteria Met
- SC1: Debug simulation using real `rulez debug` binary ✅
- SC2: Step-by-step rule evaluation trace ✅
