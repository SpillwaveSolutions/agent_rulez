# CRD-001: Enhanced Logging for Debugging

**Status:** Proposed
**Created:** 2026-01-22
**Author:** Claude (with user collaboration)
**Branch:** `001-cch-binary-v1`
**Base Commit:** `fba3dd069347f21cb9301e9c799ce72919fffc46`
**Commit Date:** 2026-01-22 11:36:43 -0600
**Commit Message:** "fix: resolve CI failures (clippy + rustfmt)"

---

## 1. Problem Statement

### Current State

The current `LogEntry` structure in `cch_cli/src/models.rs` (lines 175-201) captures only minimal information:

```rust
pub struct LogEntry {
    pub timestamp: DateTime<Utc>,
    pub event_type: String,
    pub session_id: String,
    pub tool_name: Option<String>,
    pub rules_matched: Vec<String>,
    pub outcome: Outcome,
    pub timing: LogTiming,
    pub metadata: Option<LogMetadata>,
}
```

### Pain Points

1. **Insufficient Context for Debugging**: When a rule fires or doesn't fire unexpectedly, there's no record of what the actual event input was. For example:

   - What command was Claude trying to run?
   - What file path was being edited?
   - What content triggered a block?

2. **No Response Tracking**: The log doesn't record what response was sent back to Claude, making it impossible to correlate logs with actual behavior.

3. **No Rule Evaluation Visibility**: When troubleshooting, users can't see which matchers passed or failed for each rule.

4. **Future-Proofing Gap**: If Claude Code adds new event types or tool inputs, the logging system has no fallback mechanism.

### User Guide Reference

The User Guide (`docs/USER_GUIDE_CLI.md`, lines 222-254) documents logs as:

```json
{
  "timestamp": "2025-01-21T14:32:11Z",
  "event": "PreToolUse",
  "rule_name": "block-force-push",
  "mode": "enforce",
  "decision": "blocked",
  "tool": "Bash"
}
```

This format lacks the detail needed for:

- Root cause analysis
- Policy debugging
- Compliance evidence with full context

---

## 2. Decisions Made

The following decisions were made through collaborative discussion:

### Decision 1: Hybrid Approach for Event Capture

**Choice:** Use typed event details (Option B) for known tools, with fallback to full raw event (Option A) for unknown events or debug mode.

**Rationale:**

- Typed extraction keeps logs clean and consistent for 95% of use cases
- Full raw event fallback ensures we handle new/unknown Claude events gracefully
- Debug mode provides maximum detail when needed

### Decision 2: Extracted Fields by Tool Type

| Tool Type | Extracted Fields |
| --- | --- |
| **Bash** | `command` only |
| **Write** | `file_path` only |
| **Edit** | `file_path` only |
| **Read** | `file_path` only |
| **Glob** | `pattern`, `path` |
| **Grep** | `pattern`, `path` |
| **Session events** | `source`/`reason`, `transcript_path`, `cwd` |
| **PermissionRequest** | `permission_mode` + underlying tool fields |
| **Unknown** | `tool_name` (with `raw_event` in debug mode only) |

**Rationale:** These are the minimum fields needed to understand why rules matched or didn't match, without bloating logs with full content (which could be megabytes for Write operations).

### Decision 3: Response Summary Capture

**Choice:** Log response summary fields: `continue_`, `reason` (if blocked), `context_length` (not full content).

**Rationale:** Full injected context can be very large. The length indicates whether injection occurred, and the reason provides block explanations.

### Decision 4: Debug Mode Activation

**Choice:** Support all three activation methods:

1. CLI flag: `--debug-logs`
2. Environment variable: `CCH_DEBUG_LOGS=1`
3. Config setting: `settings.debug_logs: true`

**Rationale:** Maximum flexibility for different use cases:

- CLI flag for one-off debugging
- Environment variable for CI/CD or scripted testing
- Config setting for persistent verbose logging during development

### Decision 5: Rule Evaluation Details

**Choice:** Include per-rule matcher results (`tools_matched`, `extensions_matched`, etc.) in debug mode only.

**Rationale:** This data is essential for understanding "why didn't my rule fire?" but adds significant log volume. Debug mode is the appropriate place.

### Decision 6: Field Naming

**Choice:** Use `raw_event` for the full event JSON field.

**Rationale:** Clear naming indicates this is the unprocessed input, distinguishing it from the extracted `event_details`.

### Decision 7: Unknown Events Handling

**Choice:** Unknown events include `raw_event` only in debug mode (not always).

**Rationale:** Consistent behavior across all event types. Users who need to debug unknown events should enable debug mode.

### Decision 8: Future Feature - Content Length

**Choice:** Add `content_length` field to Edit/Write event details in a future iteration.

**Rationale:** Provides size visibility without including actual content. Useful for understanding large file operations. Specified now for future implementation.

### Decision 9: Permission Event Nesting (Not Flattening)

**Choice:** Use explicit nesting with `tool_details: Box<EventDetails>` instead of `#[serde(flatten)]`.

**Rationale:** The `#[serde(flatten)]` attribute conflicts with `#[serde(tag = "tool_type")]` on the parent enum. Flattening would create duplicate `tool_type` keys in the JSON output, which is invalid. Explicit nesting with `tool_details` provides a clear, valid JSON structure:

```json
{
  "tool_type": "Permission",
  "permission_mode": "default",
  "tool_details": {
    "tool_type": "Bash",
    "command": "git push --force"
  }
}
```

Additionally, `Box<EventDetails>` is required because `Permission` contains another `EventDetails`, creating a recursive type. Without `Box`, the compiler cannot determine the size at compile time. `Box` allocates the inner value on the heap, providing a fixed-size pointer (8 bytes) on the stack.

---

## 3. Proposed Solution

### New Data Structures

#### 3.1 EventDetails Enum

Typed extraction for known tools with `Unknown` fallback:

```rust
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "tool_type")]
pub enum EventDetails {
    Bash { 
        command: String,
    },
    Write { 
        file_path: String,
        // Future: content_length: Option<usize>,
    },
    Edit { 
        file_path: String,
        // Future: content_length: Option<usize>,
    },
    Read { 
        file_path: String,
    },
    Glob { 
        pattern: Option<String>, 
        path: Option<String>,
    },
    Grep { 
        pattern: Option<String>, 
        path: Option<String>,
    },
    Session {
        source: Option<String>,
        reason: Option<String>,
        transcript_path: Option<String>,
        cwd: Option<String>,
    },
    Permission {
        permission_mode: Option<String>,
        tool_details: Box<EventDetails>,  // Nested, not flattened
    },
    Unknown { 
        tool_name: Option<String>,
    },
}
```

#### 3.2 ResponseSummary Struct

```rust
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ResponseSummary {
    pub continue_: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub context_length: Option<usize>,
}
```

#### 3.3 RuleEvaluation Struct (Debug Mode)

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

#### 3.4 Enhanced LogEntry

```rust
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct LogEntry {
    // === Existing fields (preserved for compatibility) ===
    pub timestamp: DateTime<Utc>,
    pub event_type: String,
    pub session_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool_name: Option<String>,
    pub rules_matched: Vec<String>,
    pub outcome: Outcome,
    pub timing: LogTiming,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<LogMetadata>,
    
    // === New fields ===
    
    /// Typed event details for known tools
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_details: Option<EventDetails>,
    
    /// Summary of response sent to Claude
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response: Option<ResponseSummary>,
    
    // === Debug mode fields ===
    
    /// Full raw event JSON (debug mode only)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub raw_event: Option<serde_json::Value>,
    
    /// Per-rule evaluation details (debug mode only)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rule_evaluations: Option<Vec<RuleEvaluation>>,
}
```

---

## 4. Implementation Notes

### 4.1 Files to Modify

| File | Changes | Priority |
| --- | --- | --- |
| `cch_cli/src/models.rs` | Add `EventDetails`, `ResponseSummary`, `RuleEvaluation`, `MatcherResults`; extend `LogEntry` | P1 |
| `cch_cli/src/events/mod.rs` | Add `pub mod extract;` | P1 |
| `cch_cli/src/events/extract.rs` | **New file** - `EventDetails::from_event()` implementation | P1 |
| `cch_cli/src/hooks.rs` | Update `process_event()` to build enhanced log entries | P1 |
| `cch_cli/src/config.rs` | Add `debug_logs: bool` to `Settings` struct | P2 |
| `cch_cli/src/cli/args.rs` | Add `--debug-logs` flag to event commands | P2 |
| `cch_cli/src/main.rs` | Pass debug mode flag through to `process_event()` | P2 |
| `docs/USER_GUIDE_CLI.md` | Update log format documentation (lines 222-254) | P3 |
| `specs/001-cch-binary-v1/spec.md` | Update US5 acceptance scenarios | P3 |

### 4.2 Implementation Order

1. **Phase 1: Core Structures** (models.rs)

   - Add all new structs/enums
   - Keep existing fields for backward compatibility
   - All new fields are `Option<T>` with `skip_serializing_if`

2. **Phase 2: Event Extraction** (events/extract.rs)

   - Implement `EventDetails::from_event()`
   - Handle each tool type
   - Implement `Unknown` fallback

3. **Phase 3: Hook Integration** (hooks.rs)

   - Modify `process_event()` signature to accept debug flag
   - Build `EventDetails` from event
   - Build `ResponseSummary` from response
   - Conditionally include `raw_event` and `rule_evaluations`

4. **Phase 4: Debug Mode Plumbing** (config, args, main)

   - Add config setting
   - Add CLI flag
   - Check environment variable
   - Thread debug flag through

5. **Phase 5: Documentation** (USER_GUIDE_CLI.md, spec.md)

   - Update log format examples
   - Document `--debug-logs` flag
   - Update spec acceptance criteria

### 4.3 Testing Strategy

1. **Unit Tests** (in models.rs or extract.rs):

   - `EventDetails::from_event()` for each tool type
   - Serialization/deserialization round-trip
   - Unknown tool fallback behavior

2. **Integration Tests** (in tests/oq/us5_logging_test.rs):

   - Normal mode produces expected log structure
   - Debug mode includes `raw_event` and `rule_evaluations`
   - Session events extract correctly
   - Permission events wrap underlying tool

3. **Manual Testing**:

   - `echo '{"tool_name":"Bash",...}' | cch pre-tool-use --debug-logs`
   - Verify log file contains expected fields
   - `cch logs` query shows new fields

### 4.4 Backward Compatibility

- All existing `LogEntry` fields preserved
- New fields are `Option<T>` with `skip_serializing_if = "Option::is_none"`
- Existing log parsing tools will ignore unknown fields
- No breaking changes to log schema version

### 4.5 Log Schema Version

Consider bumping `log_schema_version` in version metadata (from `cch --version --json`) to indicate enhanced logging is available. Suggested: `log_schema_version: 2`.

### 4.6 Log Size Considerations

**Warning:** Debug mode significantly increases log file size due to:

- Full `raw_event` JSON (200-500+ bytes per event)
- `rule_evaluations` array (50-200 bytes per rule evaluated)

Recommend documenting log rotation best practices:

- Default log location: `~/.claude/logs/cch.log`
- Consider `logrotate` or similar for production use
- Debug mode should be temporary, not permanent

---

## 5. Other Enhancements

### 5.1 Content Length for Edit/Write (v2)

Add `content_length` field to provide size visibility without full content:

```rust
Write { 
    file_path: String,
    content_length: Option<usize>,  // Bytes of content being written
},
Edit { 
    file_path: String,
    old_string_length: Option<usize>,
    new_string_length: Option<usize>,
},
```

**Use Cases:**

- Identify unexpectedly large file operations
- Debug content-based blocking rules
- Compliance logging for data size

### 5.2 Log Compression (v2)

For high-volume environments, consider optional gzip compression of log files. 

Investigae other frameworks that provide this support use a Perplexity MCP Search to research this.  

### 5.3 Structured Log Queries (v2)

Enhance `cch logs` command with filtering by:

- `--tool <name>` - Filter by tool type
- `--outcome <allow|block|inject>` - Filter by decision
- `--rule <name>` - Filter by matching rule
- `--since <duration>` - Time-based filtering

---

## 6. Example Outputs

### Normal Mode Log Entry

```json
{
  "timestamp": "2026-01-22T14:32:11Z",
  "event_type": "PreToolUse",
  "session_id": "abc123",
  "tool_name": "Bash",
  "event_details": {
    "tool_type": "Bash",
    "command": "git push --force origin main"
  },
  "rules_matched": ["block-force-push"],
  "outcome": "block",
  "response": {
    "continue_": false,
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
  "event_type": "PreToolUse",
  "session_id": "abc123",
  "tool_name": "Bash",
  "event_details": {
    "tool_type": "Bash",
    "command": "git push --force origin main"
  },
  "rules_matched": ["block-force-push"],
  "outcome": "block",
  "response": {
    "continue_": false,
    "reason": "Blocked by rule 'block-force-push': Force push is not allowed"
  },
  "timing": {
    "processing_ms": 2,
    "rules_evaluated": 12
  },
  "raw_event": {
    "event_type": "PreToolUse",
    "tool_name": "Bash",
    "tool_input": {
      "command": "git push --force origin main",
      "description": "Push changes to remote"
    },
    "session_id": "abc123",
    "timestamp": "2026-01-22T14:32:11Z",
    "transcript_path": "/Users/.../.claude/projects/.../abc123.jsonl",
    "cwd": "/Users/project",
    "permission_mode": "default"
  },
  "rule_evaluations": [
    {
      "rule_name": "block-force-push",
      "matched": true,
      "matcher_results": {
        "tools_matched": true,
        "command_match_matched": true
      }
    },
    {
      "rule_name": "inject-git-context",
      "matched": false,
      "matcher_results": {
        "tools_matched": true,
        "command_match_matched": false
      }
    }
  ]
}
```

### Session Start Log Entry

```json
{
  "timestamp": "2026-01-22T14:30:00Z",
  "event_type": "SessionStart",
  "session_id": "abc123",
  "event_details": {
    "tool_type": "Session",
    "source": "startup",
    "transcript_path": "/Users/.../.claude/projects/.../abc123.jsonl",
    "cwd": "/Users/project"
  },
  "rules_matched": ["load-project-context"],
  "outcome": "inject",
  "response": {
    "continue_": true,
    "context_length": 2048
  },
  "timing": {
    "processing_ms": 5,
    "rules_evaluated": 3
  }
}
```

### Unknown Event Log Entry (Debug Mode)

```json
{
  "timestamp": "2026-01-22T14:32:11Z",
  "event_type": "NewFutureEvent",
  "session_id": "abc123",
  "tool_name": "NewTool",
  "event_details": {
    "tool_type": "Unknown",
    "tool_name": "NewTool"
  },
  "rules_matched": [],
  "outcome": "allow",
  "response": {
    "continue_": true
  },
  "timing": {
    "processing_ms": 1,
    "rules_evaluated": 12
  },
  "raw_event": {
    "event_type": "NewFutureEvent",
    "tool_name": "NewTool",
    "tool_input": { "new_field": "value" },
    "session_id": "abc123"
  }
}
```

---

## 7. Success Criteria

| ID | Criteria | Validation |
| --- | --- | --- |
| SC-001 | Normal mode logs include `event_details` with extracted fields for all known tools | Unit tests for each tool type |
| SC-002 | Normal mode logs include `response` summary with continue/block status | Integration test |
| SC-003 | Debug mode logs include `raw_event` with full event JSON | Integration test with `--debug-logs` |
| SC-004 | Debug mode logs include `rule_evaluations` with matcher results | Integration test |
| SC-005 | Unknown tools/events fall back gracefully with `Unknown` type | Unit test |
| SC-006 | Debug mode activatable via CLI flag, env var, or config | Manual verification |
| SC-007 | All existing log fields preserved (backward compatible) | Schema comparison |
| SC-008 | All tests pass, including new logging tests | CI pipeline |

---

## 8. References

- **Spec**: `specs/001-cch-binary-v1/spec.md` - User Story 5 (Logging)
- **PRD**: `docs/prds/cch_cli_prd.md` - Section 3.6 (Output Format)
- **User Guide**: `docs/USER_GUIDE_CLI.md` - Section 4 (Observability)
- **Claude Code Hooks Docs**: https://docs.anthropic.com/en/docs/claude-code/hooks
- **Current Models**: `cch_cli/src/models.rs` - Lines 175-232

---

## 9. Appendix: Claude Code Hook Input Schemas

Reference schemas from official Claude Code documentation for implementation:

### Bash Tool Input

```json
{
  "tool_name": "Bash",
  "tool_input": {
    "command": "string",
    "description": "string (optional)",
    "timeout": "number (optional)",
    "run_in_background": "boolean (optional)"
  }
}
```

### Write Tool Input

```json
{
  "tool_name": "Write",
  "tool_input": {
    "file_path": "string",
    "content": "string"
  }
}
```

### Edit Tool Input

```json
{
  "tool_name": "Edit",
  "tool_input": {
    "file_path": "string",
    "old_string": "string",
    "new_string": "string",
    "replace_all": "boolean (optional)"
  }
}
```

### Read Tool Input

```json
{
  "tool_name": "Read",
  "tool_input": {
    "file_path": "string",
    "offset": "number (optional)",
    "limit": "number (optional)"
  }
}
```

### Common Fields (All Events)

```json
{
  "session_id": "string",
  "transcript_path": "string",
  "cwd": "string",
  "permission_mode": "string (default|plan|acceptEdits|dontAsk|bypassPermissions)",
  "hook_event_name": "string"
}
```

### SessionStart Input

```json
{
  "hook_event_name": "SessionStart",
  "source": "startup | resume | clear | compact"
}
```

### SessionEnd Input

```json
{
  "hook_event_name": "SessionEnd",
  "reason": "clear | logout | prompt_input_exit | other"
}
```