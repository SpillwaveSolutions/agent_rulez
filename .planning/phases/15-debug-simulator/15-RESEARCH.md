# Phase 15: Debug Simulator — Research

## Goal
Real binary integration with step-by-step rule evaluation traces, save/load test cases.

## Success Criteria
1. User can run debug simulation using the real `rulez debug` binary (not mock data)
2. User sees step-by-step rule evaluation trace showing which rules matched and why
3. User can save debug test cases (event + expected result) for reuse
4. User can load and replay saved test cases from previous sessions

## What Already Exists

### UI Components (Complete)
- `DebugSimulator.tsx` — Main wrapper: form → runDebug → ResultView + EvaluationTrace
- `EventForm.tsx` — Dropdown for 7 event types, text inputs for tool/command/path
- `ResultView.tsx` — Outcome badge (Allow/Block/Inject), reason, matched rules list
- `EvaluationTrace.tsx` — Accordion showing per-rule evaluation details
- `RightPanel.tsx` — Renders `<DebugSimulator />` in "Simulator" tab (320px)

### Types (Complete)
- `DebugParams`: `{ eventType, tool?, command?, path? }`
- `DebugResult`: `{ outcome, reason?, matchedRules[], evaluationTimeMs, evaluations[] }`
- `RuleEvaluation`: `{ ruleName, matched, timeMs, details?, pattern?, input? }`
- `EventType`: union of 7 string literals

### Tauri Backend
- `run_debug` command in `rulez_ui/src-tauri/src/commands/debug.rs`
- Passes `--json` flag to CLI binary, parses stdout as `DebugResult`
- Binary path resolved from settings store → PATH env fallback

### CLI Debug Command
- `cch_cli/src/cli/debug.rs` — `run()` function
- `SimEventType` enum: only 4 variants (PreToolUse, PostToolUse, SessionStart, PermissionRequest)
- Model `EventType` enum: all 7 variants (includes UserPromptSubmit, SessionEnd, PreCompact)
- No `--json` flag accepted — output is human-readable mixed text + pretty JSON blocks
- Tauri backend passes `--json` which clap silently ignores → `serde_json::from_str()` fails

### Browser Mock
- `mockRunDebug()` in `lib/tauri.ts` — returns hardcoded 2-rule evaluation after 100ms delay
- `window.__rulezMockDebugResponse` in test utils — not wired into app

### E2E Tests
- `simulator.spec.ts`: 10 tests, tests 1-7 work with mock, tests 8-9 (save/load) reference buttons that don't exist

## Critical Gaps

### Gap 1: CLI `--json` flag missing
The CLI `debug` subcommand does not accept `--json`. Clap silently ignores unknown flags.
The output is mixed human-readable text + JSON blocks — unparseable by `serde_json::from_str()`.

**Fix:** Add `--json` flag to clap args. When present, output a single JSON object matching `DebugResult`.

### Gap 2: SimEventType only supports 4 of 7 event types
`SimEventType` in `debug.rs` only has: PreToolUse, PostToolUse, SessionStart, PermissionRequest.
Missing: UserPromptSubmit, SessionEnd, PreCompact.
The model `EventType` already supports all 7.

**Fix:** Add the 3 missing variants to `SimEventType` and its `from_str()` method.

### Gap 3: Save/load test cases not implemented
SC-3 and SC-4 require persistent test cases. No UI, no store, no persistence.

**Fix:** Add test case store (Zustand + localStorage), save/load buttons in DebugSimulator.

### Gap 4: Mock injection unused in E2E
`window.__rulezMockDebugResponse` exists but `mockRunDebug()` doesn't check it.

**Fix:** Wire `mockRunDebug()` to check `window.__rulezMockDebugResponse` first.

## Plan Structure

- **Plan 01**: CLI `--json` flag + full event type support
- **Plan 02**: Save/load test cases UI + persistence
- **Plan 03**: Integration wiring + E2E test fixes
