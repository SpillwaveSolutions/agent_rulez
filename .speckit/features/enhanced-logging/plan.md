# Implementation Plan: CRD-001 Enhanced Logging

**Feature ID:** CRD-001  
**Spec:** `.specify/features/enhanced-logging/spec.md`  
**Branch:** `002-enhanced-logging`  
**Created:** 2026-01-22

---

## Architecture Overview

```
┌─────────────────────────────────────────────────────────────────┐
│                         CCH Binary                               │
├─────────────────────────────────────────────────────────────────┤
│  main.rs                                                         │
│  ├── Parse --debug-logs flag                                     │
│  ├── Check CCH_DEBUG_LOGS env var                               │
│  └── Pass DebugConfig to process_event()                        │
├─────────────────────────────────────────────────────────────────┤
│  hooks.rs                                                        │
│  ├── process_event(event, debug_config) → Response              │
│  ├── Build EventDetails from Event                              │
│  ├── Track RuleEvaluation per rule (debug mode)                 │
│  ├── Build ResponseSummary from Response                        │
│  └── Construct enhanced LogEntry                                │
├─────────────────────────────────────────────────────────────────┤
│  models.rs (Enhanced)                                            │
│  ├── EventDetails enum (Bash, Write, Edit, Read, etc.)          │
│  ├── ResponseSummary struct                                      │
│  ├── RuleEvaluation struct                                       │
│  ├── MatcherResults struct                                       │
│  └── LogEntry (extended with new fields)                        │
├─────────────────────────────────────────────────────────────────┤
│  config.rs (Enhanced)                                            │
│  └── Settings.debug_logs: bool                                   │
└─────────────────────────────────────────────────────────────────┘
```

---

## Technical Decisions

### TD-001: Event Extraction Strategy
**Decision:** Implement `EventDetails::extract()` as a standalone function in a new `extract` module.

**Rationale:**
- Keeps `models.rs` clean (data structures only)
- Allows unit testing extraction logic independently
- Follows single responsibility principle

**Implementation:**
```rust
// cch_cli/src/extract.rs
impl EventDetails {
    pub fn extract(event: &Event) -> Self { ... }
}
```

### TD-002: Debug Mode Configuration
**Decision:** Create a `DebugConfig` struct to encapsulate debug settings from all sources.

**Rationale:**
- Single source of truth for debug state
- Clean API for `process_event()`
- Easy to extend with future debug options

**Implementation:**
```rust
pub struct DebugConfig {
    pub enabled: bool,
    pub include_raw_event: bool,
    pub include_rule_evaluations: bool,
}

impl DebugConfig {
    pub fn from_env_and_config(config: &Config, cli_flag: bool) -> Self {
        let enabled = cli_flag 
            || std::env::var("CCH_DEBUG_LOGS").is_ok()
            || config.settings.debug_logs;
        Self {
            enabled,
            include_raw_event: enabled,
            include_rule_evaluations: enabled,
        }
    }
}
```

### TD-003: Rule Evaluation Tracking
**Decision:** Track matcher results during rule evaluation, not as a separate pass.

**Rationale:**
- Avoids duplicate regex compilation
- Captures actual match state (not re-computed)
- Minimal performance overhead (just struct building)

**Implementation:**
- Modify `matches_rule()` to return `(bool, MatcherResults)` when debug mode enabled
- Store results in `Vec<RuleEvaluation>` during evaluation loop

### TD-004: Backward Compatibility Approach
**Decision:** All new `LogEntry` fields are `Option<T>` with `#[serde(skip_serializing_if = "Option::is_none")]`.

**Rationale:**
- Existing log parsers ignore unknown fields (JSON standard)
- Old logs still parse correctly
- No schema version bump required (but recommend logging schema version)

### TD-005: No New Module for Events
**Decision:** Add extraction logic directly to `models.rs` as `impl EventDetails` block, rather than creating `events/` module.

**Rationale:**
- Simpler structure for this scope
- EventDetails is tightly coupled to Event struct
- Avoids over-engineering for ~100 lines of extraction code

---

## File Changes

### Phase 1: Core Data Structures

#### `cch_cli/src/models.rs`
**Changes:**
1. Add `EventDetails` enum after `LogMetadata` (line ~232)
2. Add `ResponseSummary` struct
3. Add `RuleEvaluation` and `MatcherResults` structs
4. Add `DebugConfig` struct
5. Extend `LogEntry` with 4 new optional fields
6. Add `impl EventDetails` with `extract()` method

**New Code (~150 lines):**
```rust
// === Enhanced Logging Types ===

/// Typed event details for known tools
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "tool_type")]
pub enum EventDetails {
    Bash { command: String },
    Write { file_path: String },
    Edit { file_path: String },
    Read { file_path: String },
    Glob { 
        pattern: Option<String>, 
        path: Option<String> 
    },
    Grep { 
        pattern: Option<String>, 
        path: Option<String> 
    },
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
    Unknown { 
        tool_name: Option<String> 
    },
}

/// Summary of response sent to Claude
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ResponseSummary {
    #[serde(rename = "continue")]
    pub continue_: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub context_length: Option<usize>,
}

/// Per-rule evaluation details (debug mode)
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct RuleEvaluation {
    pub rule_name: String,
    pub matched: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub matcher_results: Option<MatcherResults>,
}

/// Individual matcher results
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

/// Debug mode configuration
#[derive(Debug, Clone, Default)]
pub struct DebugConfig {
    pub enabled: bool,
}

impl DebugConfig {
    pub fn new(cli_flag: bool, config_setting: bool) -> Self {
        let enabled = cli_flag 
            || std::env::var("CCH_DEBUG_LOGS").is_ok()
            || config_setting;
        Self { enabled }
    }
}
```

**LogEntry Extension:**
```rust
pub struct LogEntry {
    // ... existing fields ...
    
    // === New fields ===
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_details: Option<EventDetails>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response: Option<ResponseSummary>,
    
    // === Debug mode fields ===
    #[serde(skip_serializing_if = "Option::is_none")]
    pub raw_event: Option<serde_json::Value>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rule_evaluations: Option<Vec<RuleEvaluation>>,
}
```

---

### Phase 2: Event Extraction Logic

#### `cch_cli/src/models.rs` (continued)
**Add `impl EventDetails` block:**

```rust
impl EventDetails {
    /// Extract typed details from an Event
    pub fn extract(event: &Event) -> Self {
        let tool_name = event.tool_name.as_deref();
        let tool_input = event.tool_input.as_ref();
        
        match tool_name {
            Some("Bash") => {
                let command = tool_input
                    .and_then(|ti| ti.get("command"))
                    .and_then(|c| c.as_str())
                    .unwrap_or("")
                    .to_string();
                EventDetails::Bash { command }
            }
            Some("Write") => {
                let file_path = tool_input
                    .and_then(|ti| ti.get("file_path").or_else(|| ti.get("filePath")))
                    .and_then(|p| p.as_str())
                    .unwrap_or("")
                    .to_string();
                EventDetails::Write { file_path }
            }
            Some("Edit") => {
                let file_path = tool_input
                    .and_then(|ti| ti.get("file_path").or_else(|| ti.get("filePath")))
                    .and_then(|p| p.as_str())
                    .unwrap_or("")
                    .to_string();
                EventDetails::Edit { file_path }
            }
            Some("Read") => {
                let file_path = tool_input
                    .and_then(|ti| ti.get("file_path").or_else(|| ti.get("filePath")))
                    .and_then(|p| p.as_str())
                    .unwrap_or("")
                    .to_string();
                EventDetails::Read { file_path }
            }
            Some("Glob") => {
                let pattern = tool_input
                    .and_then(|ti| ti.get("pattern"))
                    .and_then(|p| p.as_str())
                    .map(String::from);
                let path = tool_input
                    .and_then(|ti| ti.get("path"))
                    .and_then(|p| p.as_str())
                    .map(String::from);
                EventDetails::Glob { pattern, path }
            }
            Some("Grep") => {
                let pattern = tool_input
                    .and_then(|ti| ti.get("pattern"))
                    .and_then(|p| p.as_str())
                    .map(String::from);
                let path = tool_input
                    .and_then(|ti| ti.get("path"))
                    .and_then(|p| p.as_str())
                    .map(String::from);
                EventDetails::Grep { pattern, path }
            }
            None if matches!(event.event_type, EventType::SessionStart | EventType::SessionEnd) => {
                let source = tool_input
                    .and_then(|ti| ti.get("source"))
                    .and_then(|s| s.as_str())
                    .map(String::from);
                let reason = tool_input
                    .and_then(|ti| ti.get("reason"))
                    .and_then(|r| r.as_str())
                    .map(String::from);
                let transcript_path = tool_input
                    .and_then(|ti| ti.get("transcript_path"))
                    .and_then(|t| t.as_str())
                    .map(String::from);
                let cwd = tool_input
                    .and_then(|ti| ti.get("cwd"))
                    .and_then(|c| c.as_str())
                    .map(String::from);
                EventDetails::Session { source, reason, transcript_path, cwd }
            }
            _ => EventDetails::Unknown { 
                tool_name: tool_name.map(String::from) 
            },
        }
    }
}

impl ResponseSummary {
    /// Create from a Response
    pub fn from_response(response: &Response) -> Self {
        Self {
            continue_: response.continue_,
            reason: response.reason.clone(),
            context_length: response.context.as_ref().map(|c| c.len()),
        }
    }
}
```

---

### Phase 3: Hook Integration

#### `cch_cli/src/hooks.rs`
**Changes:**
1. Modify `process_event()` signature to accept `DebugConfig`
2. Build `EventDetails` from event
3. Build `ResponseSummary` from response
4. Conditionally include `raw_event` and `rule_evaluations`
5. Track matcher results when debug mode enabled

**Key Changes:**
```rust
/// Process a hook event and return the appropriate response
pub async fn process_event(event: Event, debug_config: &DebugConfig) -> Result<Response> {
    let start_time = std::time::Instant::now();

    // Load configuration
    let config = Config::load(None)?;

    // Extract event details (always)
    let event_details = EventDetails::extract(&event);

    // Evaluate rules (with optional debug tracking)
    let (matched_rules, response, rule_evaluations) = 
        evaluate_rules(&event, &config, debug_config).await?;

    let processing_time = start_time.elapsed().as_millis() as u64;

    // Build response summary
    let response_summary = ResponseSummary::from_response(&response);

    // Log the event with enhanced fields
    let entry = LogEntry {
        timestamp: event.timestamp,
        event_type: format!("{:?}", event.event_type),
        session_id: event.session_id.clone(),
        tool_name: event.tool_name.clone(),
        rules_matched: matched_rules.into_iter().map(|r| r.name.clone()).collect(),
        outcome: match response.continue_ {
            true if response.context.is_some() => Outcome::Inject,
            true => Outcome::Allow,
            false => Outcome::Block,
        },
        timing: LogTiming {
            processing_ms: processing_time,
            rules_evaluated: config.enabled_rules().len(),
        },
        metadata: Some(LogMetadata {
            injected_files: response
                .context
                .as_ref()
                .map(|_| vec!["injected".to_string()]),
            validator_output: None,
        }),
        // New fields
        event_details: Some(event_details),
        response: Some(response_summary),
        raw_event: if debug_config.enabled {
            Some(serde_json::to_value(&event).unwrap_or_default())
        } else {
            None
        },
        rule_evaluations: if debug_config.enabled {
            Some(rule_evaluations)
        } else {
            None
        },
    };

    // ... rest of function
}
```

---

### Phase 4: Debug Mode Plumbing

#### `cch_cli/src/config.rs`
**Add to Settings struct:**
```rust
pub struct Settings {
    // ... existing fields ...
    
    /// Enable debug logging with full event and rule details
    #[serde(default)]
    pub debug_logs: bool,
}
```

#### `cch_cli/src/main.rs`
**Changes:**
1. Add `--debug-logs` global flag
2. Build `DebugConfig` from all sources
3. Pass to `process_event()`

```rust
#[derive(Parser)]
#[command(name = "cch")]
struct Cli {
    /// Enable debug logging with full event and rule details
    #[arg(long, global = true)]
    debug_logs: bool,
    
    #[command(subcommand)]
    command: Option<Commands>,
}

// In main():
let debug_config = DebugConfig::new(cli.debug_logs, config.settings.debug_logs);
let response = hooks::process_event(event, &debug_config).await?;
```

---

### Phase 5: Testing

#### `cch_cli/src/models.rs` (tests)
**Add unit tests for EventDetails extraction:**
```rust
#[cfg(test)]
mod event_details_tests {
    use super::*;

    #[test]
    fn test_extract_bash_event() {
        let event = Event {
            event_type: EventType::PreToolUse,
            tool_name: Some("Bash".to_string()),
            tool_input: Some(serde_json::json!({
                "command": "git push --force"
            })),
            session_id: "test".to_string(),
            timestamp: Utc::now(),
            user_id: None,
        };
        
        let details = EventDetails::extract(&event);
        assert!(matches!(details, EventDetails::Bash { command } if command == "git push --force"));
    }
    
    #[test]
    fn test_extract_unknown_tool() {
        let event = Event {
            event_type: EventType::PreToolUse,
            tool_name: Some("FutureTool".to_string()),
            tool_input: None,
            session_id: "test".to_string(),
            timestamp: Utc::now(),
            user_id: None,
        };
        
        let details = EventDetails::extract(&event);
        assert!(matches!(details, EventDetails::Unknown { tool_name } if tool_name == Some("FutureTool".to_string())));
    }
    
    // ... tests for each tool type
}
```

#### `cch_cli/tests/oq_enhanced_logging.rs` (new file)
**Integration tests for enhanced logging:**
- Test normal mode log structure
- Test debug mode includes `raw_event`
- Test debug mode includes `rule_evaluations`
- Test `--debug-logs` flag
- Test `CCH_DEBUG_LOGS` environment variable
- Test config `debug_logs: true` setting

---

## Implementation Order

| Phase | Files | Estimated LOC | Priority |
|-------|-------|---------------|----------|
| 1 | `models.rs` (data structures) | +120 | P1 |
| 2 | `models.rs` (extraction impl) | +80 | P1 |
| 3 | `hooks.rs` (integration) | +40, ~20 modified | P1 |
| 4 | `config.rs`, `main.rs` (plumbing) | +15 | P2 |
| 5 | `models.rs` tests, new integration test | +150 | P1 |
| **Total** | | **~400 new LOC** | |

---

## Risk Mitigation

| Risk | Mitigation |
|------|------------|
| Breaking existing log parsers | All new fields are `Option<T>` with `skip_serializing_if` |
| Performance regression from debug tracking | Matcher tracking only builds structs, no extra regex compilation |
| Large log files in debug mode | Document that debug mode should be temporary; recommend log rotation |
| Permission event recursion | Use `Box<EventDetails>` for heap allocation |

---

## Success Verification

After implementation, verify:

1. **Normal Mode:**
   ```bash
   echo '{"event_type":"PreToolUse","tool_name":"Bash","tool_input":{"command":"git status"},...}' | cch
   # Check ~/.claude/logs/cch.log contains event_details and response
   ```

2. **Debug Mode (CLI flag):**
   ```bash
   echo '...' | cch --debug-logs
   # Check log contains raw_event and rule_evaluations
   ```

3. **Debug Mode (env var):**
   ```bash
   CCH_DEBUG_LOGS=1 echo '...' | cch
   # Check log contains raw_event and rule_evaluations
   ```

4. **Backward Compatibility:**
   ```bash
   # Parse old log entries with new code - should work
   # Parse new log entries with old tools - unknown fields ignored
   ```

5. **All Tests Pass:**
   ```bash
   cargo test
   # All 47+ existing tests pass
   # New enhanced logging tests pass
   ```

---

## References

- **Spec:** `.specify/features/enhanced-logging/spec.md`
- **PRD:** `docs/prds/change_request/CRD-001-enhanced-logging.md`
- **Current Models:** `cch_cli/src/models.rs`
- **Current Hooks:** `cch_cli/src/hooks.rs`
