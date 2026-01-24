# Phase 2 Governance Implementation Tasks

**Feature ID:** phase2-governance
**Status:** Ready for Implementation
**Total Estimated Days:** 5.5-9 days

---

## Phase 2.1: Core Governance (3-4 days)

### P2.1-T01: Add PolicyMode enum
- [ ] Create `PolicyMode` enum in `models/mod.rs`
- [ ] Values: `Enforce`, `Warn`, `Audit`
- [ ] Implement `Default` trait (default = Enforce)
- [ ] Implement `Deserialize` for YAML parsing (case-insensitive)
- [ ] Implement `Serialize` for JSON output
- [ ] Add unit tests for parsing

**Code:**
```rust
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum PolicyMode {
    #[default]
    Enforce,
    Warn,
    Audit,
}
```

---

### P2.1-T02: Add RuleMetadata struct
- [ ] Create `RuleMetadata` struct in `models/mod.rs`
- [ ] Fields: `author`, `created_by`, `reason`, `confidence`, `last_reviewed`, `ticket`, `tags`
- [ ] All fields are `Option<T>`
- [ ] Create `Confidence` enum: `High`, `Medium`, `Low`
- [ ] Implement `Deserialize` and `Serialize`
- [ ] Add unit tests

**Code:**
```rust
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct RuleMetadata {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub author: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_by: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub confidence: Option<Confidence>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_reviewed: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ticket: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Confidence {
    High,
    Medium,
    Low,
}
```

---

### P2.1-T03: Extend Rule struct
- [ ] Add `mode: Option<PolicyMode>` field to `Rule`
- [ ] Add `priority: Option<i32>` field to `Rule`
- [ ] Add `metadata: Option<RuleMetadata>` field to `Rule`
- [ ] Use `#[serde(default)]` for backward compatibility
- [ ] Update existing tests to verify backward compatibility
- [ ] Add new tests for parsing rules with governance fields

---

### P2.1-T04: Implement priority-based rule sorting
- [ ] Create function `sort_rules_by_priority(rules: &mut Vec<Rule>)`
- [ ] Sort by priority descending (higher first)
- [ ] Stable sort to preserve file order for same priority
- [ ] Default priority = 0 for rules without explicit priority
- [ ] Call sorting before rule matching in hook processor
- [ ] Add unit tests for sorting behavior

**Code:**
```rust
pub fn sort_rules_by_priority(rules: &mut [Rule]) {
    rules.sort_by(|a, b| {
        let priority_a = a.priority.unwrap_or(0);
        let priority_b = b.priority.unwrap_or(0);
        priority_b.cmp(&priority_a) // Descending order
    });
}
```

---

### P2.1-T05: Implement mode-based action execution
- [ ] Update `execute_action` to check rule mode
- [ ] `Enforce`: Current behavior (block/inject/run)
- [ ] `Warn`: Never block, inject warning message instead
- [ ] `Audit`: Skip action, log only
- [ ] Create warning context injection for warn mode
- [ ] Add integration tests for each mode

**Mode Execution Logic:**
```rust
fn execute_action(rule: &Rule, action: &Action, event: &Event) -> ActionResult {
    let mode = rule.mode.unwrap_or_default();
    
    match mode {
        PolicyMode::Enforce => {
            // Normal execution
            execute_action_impl(action, event)
        }
        PolicyMode::Warn => {
            // Never block, inject warning instead
            if action.is_block() {
                ActionResult::Warning(format!("Rule '{}' would block: {}", rule.name, action.reason()))
            } else {
                execute_action_impl(action, event)
            }
        }
        PolicyMode::Audit => {
            // Log only, no execution
            ActionResult::Audited
        }
    }
}
```

---

### P2.1-T06: Implement conflict resolution
- [ ] Create `resolve_conflicts(matched_rules: Vec<&Rule>) -> ResolvedOutcome`
- [ ] Enforce mode always wins over warn/audit
- [ ] Among same modes, highest priority wins
- [ ] For multiple blocks, use highest priority block message
- [ ] Log conflict resolution decisions
- [ ] Add unit tests for all conflict scenarios

**Conflict Resolution Table Tests:**
```rust
#[test]
fn test_enforce_wins_over_warn() { ... }

#[test]
fn test_enforce_wins_over_audit() { ... }

#[test]
fn test_higher_priority_wins() { ... }

#[test]
fn test_multiple_enforces_highest_priority_message() { ... }
```

---

## Phase 2.2: Enhanced Logging (1-2 days)

### P2.2-T01: Add Decision enum
- [ ] Create `Decision` enum in `models/mod.rs`
- [ ] Values: `Allowed`, `Blocked`, `Warned`, `Audited`
- [ ] Implement `Serialize` for JSON output
- [ ] Add to log entries

**Code:**
```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum Decision {
    Allowed,
    Blocked,
    Warned,
    Audited,
}
```

---

### P2.2-T02: Extend LogEntry struct
- [ ] Add `mode: Option<PolicyMode>` field
- [ ] Add `priority: Option<i32>` field
- [ ] Add `decision: Option<Decision>` field
- [ ] Add `metadata: Option<RuleMetadata>` field
- [ ] Use `#[serde(skip_serializing_if = "Option::is_none")]` for all new fields
- [ ] Verify existing log parsing still works

---

### P2.2-T03: Update log writer
- [ ] Populate new fields when writing log entries
- [ ] Include mode from matched rule
- [ ] Include priority from matched rule
- [ ] Include decision from action result
- [ ] Include metadata if present
- [ ] Add integration tests for log format

---

### P2.2-T04: Update log querying
- [ ] Extend `cch logs` to filter by mode
- [ ] Extend `cch logs` to filter by decision
- [ ] Add `--mode <mode>` flag
- [ ] Add `--decision <decision>` flag
- [ ] Update help text

---

## Phase 2.3: CLI Enhancements (1-2 days)

### P2.3-T01: Enhance `cch explain rule` command
- [ ] Display mode (with default indicator)
- [ ] Display priority (with default indicator)
- [ ] Display full metadata block
- [ ] Format output for readability
- [ ] Add `--json` flag for structured output

**Output Format:**
```
Rule: <name>
Event: <event_type>
Mode: <mode> (default: enforce)
Priority: <priority> (default: 0)

Matchers:
  tools: [...]
  extensions: [...]
  ...

Action:
  <action_type>: <action_config>

Metadata:
  author: <author>
  created_by: <created_by>
  reason: <reason>
  ...
```

---

### P2.3-T02: Add activity statistics
- [ ] Parse recent log entries for the rule
- [ ] Count total triggers
- [ ] Count blocks/warns/audits
- [ ] Find last trigger timestamp
- [ ] Display in `cch explain rule` output
- [ ] Add `--no-stats` flag to skip log parsing

**Activity Section:**
```
Recent Activity:
  Triggered: 14 times
  Blocked: 3 times
  Warned: 2 times
  Audited: 9 times
  Last trigger: 2025-01-20 14:32
```

---

### P2.3-T03: Add `cch explain rule --json`
- [ ] Output complete rule as JSON
- [ ] Include metadata
- [ ] Include activity stats
- [ ] Machine-parseable format

---

### P2.3-T04: Update help text
- [ ] Document `mode` field in help
- [ ] Document `priority` field in help
- [ ] Document `metadata` field in help
- [ ] Update examples with governance features

---

## Phase 2.4: Trust Levels (0.5-1 day)

### P2.4-T01: Add trust field to run action
- [ ] Extend `run` action to support object format
- [ ] Add optional `trust` field: `local | verified | untrusted`
- [ ] Maintain backward compatibility with string format
- [ ] Parse both formats correctly

**YAML Formats:**
```yaml
# Simple format (existing)
actions:
  run: .claude/validators/check.py

# Extended format (new)
actions:
  run:
    script: .claude/validators/check.py
    trust: local
```

---

### P2.4-T02: Create TrustLevel enum
- [ ] Values: `Local`, `Verified`, `Untrusted`
- [ ] Implement parsing
- [ ] Default: None (unspecified)

---

### P2.4-T03: Log trust levels
- [ ] Include trust level in log entries when present
- [ ] Display in `cch explain rule` output
- [ ] No enforcement (informational only in v1.1)

---

### P2.4-T04: Document trust levels
- [ ] Update hooks.yaml schema documentation
- [ ] Add examples in SKILL.md
- [ ] Note: Enforcement planned for future version

---

## Definition of Done (per task)

- [ ] Code complete and compiles
- [ ] Unit tests written and passing
- [ ] Integration tests for user-facing behavior
- [ ] Backward compatibility verified
- [ ] Documentation updated
- [ ] Pre-commit checks pass:
  ```bash
  cd cch_cli && cargo fmt && cargo clippy --all-targets --all-features -- -D warnings && cargo test
  ```

---

## Test Coverage Requirements

### Unit Tests
- [ ] PolicyMode parsing (case-insensitive)
- [ ] RuleMetadata parsing (all optional fields)
- [ ] Confidence enum parsing
- [ ] Priority sorting
- [ ] Conflict resolution logic
- [ ] Decision enum serialization

### Integration Tests
- [ ] Rule with mode=enforce blocks correctly
- [ ] Rule with mode=warn injects warning, doesn't block
- [ ] Rule with mode=audit logs only
- [ ] Priority sorting affects rule order
- [ ] Conflict resolution with mixed modes
- [ ] Enhanced log entries contain new fields
- [ ] `cch explain rule` displays all fields
- [ ] Backward compatibility with v1.0 configs

---

## Notes

### Backward Compatibility Strategy
- All new fields use `Option<T>`
- All new fields use `#[serde(skip_serializing_if = "Option::is_none")]`
- Default values preserve v1.0 behavior
- Existing configs parse without changes
- Existing log parsers ignore new fields

### Performance Considerations
- Priority sorting is O(n log n), negligible for typical rule counts (<100)
- Metadata adds minimal memory overhead
- Mode checking is O(1)
- Log entry size increase is bounded (<2KB per entry)
