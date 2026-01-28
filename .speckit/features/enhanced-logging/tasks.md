# Tasks: CRD-001 Enhanced Logging

**Feature ID:** CRD-001  
**Spec:** `.speckit/features/enhanced-logging/spec.md`  
**Plan:** `.speckit/features/enhanced-logging/plan.md`  
**Status:** Complete  
**Implemented:** 2026-01-22 (commit b9faa44)

---

## Phase 1: Core Data Structures (models.rs)

- [x] Add `EventDetails` enum with tagged variants
  - [x] Bash variant with `command` field
  - [x] Write variant with `file_path` field
  - [x] Edit variant with `file_path` field
  - [x] Read variant with `file_path` field
  - [x] Glob variant with `pattern` and `path` fields
  - [x] Grep variant with `pattern` and `path` fields
  - [x] Session variant with `source`, `reason`, `transcript_path`, `cwd` fields
  - [x] Permission variant with `permission_mode` and boxed `tool_details`
  - [x] Unknown variant with `tool_name` field
- [x] Add `ResponseSummary` struct with `continue` (serde-renamed from `continue_`), `reason`, `context_length`
- [x] Add `RuleEvaluation` struct with `rule_name`, `matched`, `matcher_results`
- [x] Add `MatcherResults` struct with individual matcher result fields
- [x] Add `DebugConfig` struct with `enabled` flag
- [x] Extend `LogEntry` with 4 new optional fields
  - [x] `event_details: Option<EventDetails>`
  - [x] `response: Option<ResponseSummary>`
  - [x] `raw_event: Option<serde_json::Value>` (debug mode only)
  - [x] `rule_evaluations: Option<Vec<RuleEvaluation>>` (debug mode only)
- [x] Add `#[serde(skip_serializing_if = "Option::is_none")]` for backward compatibility

## Phase 2: Event Extraction Logic (models.rs)

- [x] Implement `EventDetails::extract()` method
- [x] Handle Bash events (extract `command` from tool_input)
- [x] Handle Write events (extract `file_path` or `filePath` from tool_input)
- [x] Handle Edit events (extract `file_path` or `filePath` from tool_input)
- [x] Handle Read events (extract `file_path` or `filePath` from tool_input)
- [x] Handle Glob events (extract `pattern` and `path` from tool_input)
- [x] Handle Grep events (extract `pattern` and `path` from tool_input)
- [x] Handle Session events (SessionStart/SessionEnd with source, reason, etc.)
- [x] Implement Unknown fallback for unrecognized tools
- [x] Implement `ResponseSummary::from_response()` method
- [x] Implement `DebugConfig::new()` constructor

## Phase 3: Hook Integration (hooks.rs)

- [x] Modify `process_event()` signature to accept `&DebugConfig`
- [x] Build `EventDetails` from incoming event using `EventDetails::extract()`
- [x] Build `ResponseSummary` from outgoing response using `ResponseSummary::from_response()`
- [x] Conditionally include `raw_event` when debug mode enabled
- [x] Conditionally include `rule_evaluations` when debug mode enabled
- [x] Modify `evaluate_rules()` to return `Vec<RuleEvaluation>`
- [x] Implement `matches_rule_with_debug()` function for debug tracking
- [x] Track individual matcher results (`tools_matched`, `extensions_matched`, etc.)
- [x] Update `LogEntry` construction with all new fields

## Phase 4: Debug Mode Plumbing

### config.rs
- [x] Add `debug_logs: bool` field to `Settings` struct
- [x] Add `default_debug_logs()` function returning `false`
- [x] Update `Settings::default()` to include `debug_logs`

### main.rs
- [x] Add `--debug-logs` global CLI flag using clap
- [x] Load config to access `settings.debug_logs`
- [x] Build `DebugConfig` from CLI flag and config setting
- [x] Pass `DebugConfig` to `process_event()`

### Environment Variable
- [x] Support `CCH_DEBUG_LOGS` environment variable in `DebugConfig::new()`

## Phase 5: Testing

### Unit Tests (models.rs)
- [x] Test `EventDetails::extract()` for Bash events
- [x] Test `EventDetails::extract()` for Write events (both `file_path` and `filePath`)
- [x] Test `EventDetails::extract()` for Edit events
- [x] Test `EventDetails::extract()` for Read events
- [x] Test `EventDetails::extract()` for Glob events
- [x] Test `EventDetails::extract()` for Grep events
- [x] Test `EventDetails::extract()` for Session events
- [x] Test `EventDetails::extract()` for Unknown tools
- [x] Test `ResponseSummary::from_response()`
- [x] Test `DebugConfig::new()` with various flag combinations

### Integration Tests
- [ ] Test normal mode log structure contains `event_details` and `response`
- [ ] Test debug mode log contains `raw_event` and `rule_evaluations`
- [ ] Test `--debug-logs` CLI flag enables debug mode
- [ ] Test `CCH_DEBUG_LOGS=1` environment variable enables debug mode
- [ ] Test `settings.debug_logs: true` config enables debug mode

## Phase 6: Documentation (Deferred)

- [ ] Update `docs/USER_GUIDE_CLI.md` with new log format
- [ ] Update `docs/USER_GUIDE_CLI.md` with `--debug-logs` flag documentation
- [ ] Update spec acceptance criteria for US5 (log troubleshooting)

---

## Summary

| Phase | Status | Tasks | Completed |
|-------|--------|-------|-----------|
| Phase 1: Data Structures | Complete | 20 | 20 |
| Phase 2: Event Extraction | Complete | 11 | 11 |
| Phase 3: Hook Integration | Complete | 9 | 9 |
| Phase 4: Debug Mode Plumbing | Complete | 7 | 7 |
| Phase 5: Testing | Partial | 15 | 10 |
| Phase 6: Documentation | Deferred | 3 | 0 |
| **Total** | **87%** | **65** | **57** |

---

## Notes

This tasks.md was backfilled after implementation to realign with SDD workflow.

**Implementation Commit:** `b9faa44 feat(crd-001): Implement enhanced logging with EventDetails, ResponseSummary, and debug mode`

**Key Implementation Details:**
- All new structs added to `models.rs` (lines 460-689)
- LogEntry extended with 4 new fields (lines 411-426)
- `matches_rule_with_debug()` added to `hooks.rs` (lines 198-296)
- `process_event()` now accepts `&DebugConfig` parameter
- Debug mode supported via CLI flag, environment variable, and config setting
