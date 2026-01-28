# CRD-001: Enhanced Logging for Debugging

**Feature ID:** CRD-001  
**Status:** Draft  
**Priority:** P1  
**Created:** 2026-01-22  
**Source PRD:** `docs/prds/change_request/CRD-001-enhanced-logging.md`

---

## Overview

Enhance CCH logging to capture detailed event context, response summaries, and per-rule evaluation details for debugging and compliance purposes.

---

## User Stories

### US1: Event Context Capture
**As a** developer debugging rule behavior  
**I want** logs to include extracted event details (command, file_path, pattern)  
**So that** I can understand what triggered or didn't trigger rules

**Acceptance Criteria:**
- [ ] Bash events log the `command` field
- [ ] Write/Edit/Read events log the `file_path` field
- [ ] Glob/Grep events log `pattern` and `path` fields
- [ ] Session events log `source`, `reason`, `transcript_path`, `cwd`
- [ ] Permission events wrap underlying tool details
- [ ] Unknown events include `tool_name` with graceful fallback

### US2: Response Summary Logging
**As a** developer analyzing CCH decisions  
**I want** logs to include response summaries  
**So that** I can correlate logs with actual Claude behavior

**Acceptance Criteria:**
- [ ] Log entries include `continue` boolean (serialized via `#[serde(rename = "continue")]`)
- [ ] Blocked responses include `reason` field
- [ ] Injection responses include `context_length` (not full content)
- [ ] Response summary is always present on processed events

### US3: Debug Mode for Verbose Logging
**As a** developer troubleshooting complex issues  
**I want** a debug mode that captures full event and rule details  
**So that** I can perform deep root cause analysis

**Acceptance Criteria:**
- [ ] Debug mode activatable via `--debug-logs` CLI flag
- [ ] Debug mode activatable via `CCH_DEBUG_LOGS=1` environment variable
- [ ] Debug mode activatable via `settings.debug_logs: true` in config
- [ ] Debug mode includes full `raw_event` JSON
- [ ] Debug mode includes `rule_evaluations` with matcher results

### US4: Backward Compatibility
**As a** user with existing log parsing tools  
**I want** existing log fields preserved  
**So that** my tooling continues to work

**Acceptance Criteria:**
- [ ] All existing `LogEntry` fields preserved
- [ ] New fields are `Option<T>` with `skip_serializing_if`
- [ ] Existing log parsers ignore unknown fields gracefully
- [ ] No breaking changes to log schema

---

## Technical Requirements

### New Data Structures

#### EventDetails Enum
Typed extraction for known tools with `Unknown` fallback:

```rust
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "tool_type")]
pub enum EventDetails {
    Bash { command: String },
    Write { file_path: String },
    Edit { file_path: String },
    Read { file_path: String },
    Glob { pattern: Option<String>, path: Option<String> },
    Grep { pattern: Option<String>, path: Option<String> },
    Session {
        source: Option<String>,
        reason: Option<String>,
        transcript_path: Option<String>,
        cwd: Option<String>,
    },
    Permission {
        permission_mode: Option<String>,
        tool_details: Box<EventDetails>,
    },
    Unknown { tool_name: Option<String> },
}
```

#### ResponseSummary Struct
```rust
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ResponseSummary {
    #[serde(rename = "continue")]
    pub continue_: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub context_length: Option<usize>,
}
```

#### RuleEvaluation Struct (Debug Mode)
```rust
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct RuleEvaluation {
    pub rule_name: String,
    pub matched: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub matcher_results: Option<MatcherResults>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct MatcherResults {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tools_matched: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extensions_matched: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub directories_matched: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub command_match_matched: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operations_matched: Option<bool>,
}
```

#### Enhanced LogEntry
```rust
pub struct LogEntry {
    // === Existing fields (preserved) ===
    pub timestamp: DateTime<Utc>,
    pub hook_event_name: String,  // Note: aliased from event_type for backward compat
    pub session_id: String,
    pub tool_name: Option<String>,
    pub rules_matched: Vec<String>,
    pub outcome: Outcome,
    pub timing: LogTiming,
    pub metadata: Option<LogMetadata>,
    
    // === New fields ===
    pub event_details: Option<EventDetails>,
    pub response: Option<ResponseSummary>,
    
    // === Debug mode fields ===
    pub raw_event: Option<serde_json::Value>,
    pub rule_evaluations: Option<Vec<RuleEvaluation>>,
}
```

---

## Files to Modify

| File | Changes | Priority |
|------|---------|----------|
| `cch_cli/src/models.rs` | Add `EventDetails`, `ResponseSummary`, `RuleEvaluation`, `MatcherResults`; extend `LogEntry` | P1 |
| `cch_cli/src/events/mod.rs` | Add `pub mod extract;` | P1 |
| `cch_cli/src/events/extract.rs` | **New file** - `EventDetails::from_event()` implementation | P1 |
| `cch_cli/src/hooks.rs` | Update `process_event()` to build enhanced log entries | P1 |
| `cch_cli/src/config.rs` | Add `debug_logs: bool` to `Settings` struct | P2 |
| `cch_cli/src/cli/args.rs` | Add `--debug-logs` flag to event commands | P2 |
| `cch_cli/src/main.rs` | Pass debug mode flag through to `process_event()` | P2 |
| `docs/USER_GUIDE_CLI.md` | Update log format documentation | P3 |
| `specs/001-cch-binary-v1/spec.md` | Update US5 acceptance scenarios | P3 |

---

## Implementation Phases

### Phase 1: Core Structures (models.rs)
- Add all new structs/enums
- Keep existing fields for backward compatibility
- All new fields are `Option<T>` with `skip_serializing_if`

### Phase 2: Event Extraction (events/extract.rs)
- Implement `EventDetails::from_event()`
- Handle each tool type
- Implement `Unknown` fallback

### Phase 3: Hook Integration (hooks.rs)
- Modify `process_event()` signature to accept debug flag
- Build `EventDetails` from event
- Build `ResponseSummary` from response
- Conditionally include `raw_event` and `rule_evaluations`

### Phase 4: Debug Mode Plumbing (config, args, main)
- Add config setting
- Add CLI flag
- Check environment variable
- Thread debug flag through

### Phase 5: Documentation (USER_GUIDE_CLI.md, spec.md)
- Update log format examples
- Document `--debug-logs` flag
- Update spec acceptance criteria

---

## Success Criteria

| ID | Criteria | Validation |
|----|----------|------------|
| SC-001 | Normal mode logs include `event_details` with extracted fields for all known tools | Unit tests for each tool type |
| SC-002 | Normal mode logs include `response` summary with continue/block status | Integration test |
| SC-003 | Debug mode logs include `raw_event` with full event JSON | Integration test with `--debug-logs` |
| SC-004 | Debug mode logs include `rule_evaluations` with matcher results | Integration test |
| SC-005 | Unknown tools/events fall back gracefully with `Unknown` type | Unit test |
| SC-006 | Debug mode activatable via CLI flag, env var, or config | Manual verification |
| SC-007 | All existing log fields preserved (backward compatible) | Schema comparison |
| SC-008 | All tests pass, including new logging tests | CI pipeline |

---

## Testing Strategy

### Unit Tests
- `EventDetails::from_event()` for each tool type (Bash, Write, Edit, Read, Glob, Grep, Session, Permission, Unknown)
- Serialization/deserialization round-trip
- Unknown tool fallback behavior

### Integration Tests
- Normal mode produces expected log structure
- Debug mode includes `raw_event` and `rule_evaluations`
- Session events extract correctly
- Permission events wrap underlying tool

### Manual Testing
- `echo '{"tool_name":"Bash",...}' | cch --debug-logs`
- Verify log file contains expected fields
- `cch logs` query shows new fields

---

## Example Outputs

### Normal Mode Log Entry
```json
{
  "timestamp": "2026-01-22T14:32:11Z",
  "hook_event_name": "PreToolUse",
  "session_id": "abc123",
  "tool_name": "Bash",
  "event_details": {
    "tool_type": "Bash",
    "command": "git push --force origin main"
  },
  "rules_matched": ["block-force-push"],
  "outcome": "block",
  "response": {
    "continue": false,
    "reason": "Blocked by rule 'block-force-push': Force push is not allowed"
  },
  "timing": {
    "processing_ms": 2,
    "rules_evaluated": 12
  }
}
```

### Debug Mode Log Entry
```json
{
  "timestamp": "2026-01-22T14:32:11Z",
  "hook_event_name": "PreToolUse",
  "session_id": "abc123",
  "tool_name": "Bash",
  "event_details": {
    "tool_type": "Bash",
    "command": "git push --force origin main"
  },
  "rules_matched": ["block-force-push"],
  "outcome": "block",
  "response": {
    "continue": false,
    "reason": "Blocked by rule 'block-force-push'"
  },
  "timing": {
    "processing_ms": 2,
    "rules_evaluated": 12
  },
  "raw_event": { "...full event JSON..." },
  "rule_evaluations": [
    {
      "rule_name": "block-force-push",
      "matched": true,
      "matcher_results": {
        "tools_matched": true,
        "command_match_matched": true
      }
    }
  ]
}
```

---

## References

- **Source PRD:** `docs/prds/change_request/CRD-001-enhanced-logging.md`
- **Original Spec:** `specs/001-cch-binary-v1/spec.md` - User Story 5
- **Claude Code Hooks Docs:** https://docs.anthropic.com/en/docs/claude-code/hooks
- **Current Models:** `cch_cli/src/models.rs` - Lines 175-232
