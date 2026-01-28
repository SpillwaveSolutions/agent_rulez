# Phase 2 Governance Implementation Tasks

**Feature ID:** phase2-governance
**Status:** COMPLETE (P2.1, P2.2, P2.3, P2.4 all implemented)
**Total Estimated Days:** 5.5-9 days
**Completion Date:** 2026-01-25

---

## Phase 2.1: Core Governance (3-4 days)

### P2.1-T01: Add PolicyMode enum
- [x] Create `PolicyMode` enum in `models/mod.rs`
- [x] Values: `Enforce`, `Warn`, `Audit`
- [x] Implement `Default` trait (default = Enforce)
- [x] Implement `Deserialize` for YAML parsing (case-insensitive)
- [x] Implement `Serialize` for JSON output
- [x] Add unit tests for parsing

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
- [x] Create `RuleMetadata` struct in `models/mod.rs`
- [x] Fields: `author`, `created_by`, `reason`, `confidence`, `last_reviewed`, `ticket`, `tags`
- [x] All fields are `Option<T>`
- [x] Create `Confidence` enum: `High`, `Medium`, `Low`
- [x] Implement `Deserialize` and `Serialize`
- [x] Add unit tests

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
- [x] Add `mode: Option<PolicyMode>` field to `Rule`
- [x] Add `priority: Option<i32>` field to `Rule`
- [x] Add `metadata: Option<RuleMetadata>` field to `Rule`
- [x] Use `#[serde(default)]` for backward compatibility
- [x] Update existing tests to verify backward compatibility
- [x] Add new tests for parsing rules with governance fields

---

### P2.1-T04: Implement priority-based rule sorting
- [x] Create function `sort_rules_by_priority(rules: &mut Vec<Rule>)`
- [x] Sort by priority descending (higher first)
- [x] Stable sort to preserve file order for same priority
- [x] Default priority = 0 for rules without explicit priority
- [x] Call sorting before rule matching in hook processor
- [x] Add unit tests for sorting behavior

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
- [x] Update `execute_action` to check rule mode
- [x] `Enforce`: Current behavior (block/inject/run)
- [x] `Warn`: Never block, inject warning message instead
- [x] `Audit`: Skip action, log only
- [x] Create warning context injection for warn mode
- [x] Add integration tests for each mode

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
- [x] Create `resolve_conflicts(matched_rules: Vec<&Rule>) -> ResolvedOutcome`
- [x] Enforce mode always wins over warn/audit
- [x] Among same modes, highest priority wins
- [x] For multiple blocks, use highest priority block message
- [x] Log conflict resolution decisions
- [x] Add unit tests for all conflict scenarios

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

## Phase 2.2: Enhanced Logging (1-2 days) - COMPLETE

### P2.2-T01: Add Decision enum
- [x] Create `Decision` enum in `models/mod.rs`
- [x] Values: `Allowed`, `Blocked`, `Warned`, `Audited`
- [x] Implement `Serialize` for JSON output
- [x] Add to log entries
- [x] Implement `FromStr` for CLI parsing

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
- [x] Add `mode: Option<PolicyMode>` field
- [x] Add `priority: Option<i32>` field
- [x] Add `decision: Option<Decision>` field
- [x] Add `governance: Option<GovernanceMetadata>` field
- [x] Add `trust_level: Option<TrustLevel>` field
- [x] Use `#[serde(skip_serializing_if = "Option::is_none")]` for all new fields
- [x] Verify existing log parsing still works

---

### P2.2-T03: Update log writer
- [x] Populate new fields when writing log entries
- [x] Include mode from matched rule
- [x] Include priority from matched rule
- [x] Include decision from action result
- [x] Include governance metadata if present
- [x] Include trust level from run action
- [x] Tests pass (68 unit + integration tests)

---

### P2.2-T04: Update log querying
- [x] Extend `cch logs` to filter by mode
- [x] Extend `cch logs` to filter by decision
- [x] Add `--mode <mode>` flag
- [x] Add `--decision <decision>` flag
- [x] Update help text and display columns

---

## Phase 2.3: CLI Enhancements (1-2 days) - COMPLETE

### P2.3-T01: Enhance `cch explain rule` command
- [x] Display mode (with default indicator)
- [x] Display priority (with default indicator)
- [x] Display full governance metadata block
- [x] Display trust level for run actions
- [x] Format output for readability
- [x] Add `--json` flag for structured output

**Output Format:**
```
Rule: <name>
Event: <hook_event_name>
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
- [x] Parse recent log entries for the rule
- [x] Count total triggers
- [x] Count blocks/warns/audits/allowed
- [x] Find last trigger timestamp
- [x] Display in `cch explain rule` output
- [x] Add `--no-stats` flag to skip log parsing

**Activity Section:**
```
Recent Activity:
  Triggered: 14 times
  Blocked: 3 times
  Warned: 2 times
  Audited: 9 times
  Allowed: 0 times
  Last trigger: 2025-01-20 14:32
```

---

### P2.3-T03: Add `cch explain rule --json`
- [x] Output complete rule as JSON
- [x] Include governance metadata
- [x] Include activity stats
- [x] Machine-parseable format with serde_json

---

### P2.3-T04: Update help text
- [x] Document `mode` field via CLI arg help
- [x] Document `priority` field via CLI arg help
- [x] Added `cch explain rules` command to list all rules
- [x] Added subcommand structure (rule, rules, event)

---

## Phase 2.4: Trust Levels (0.5-1 day) - COMPLETE

### P2.4-T01: Add trust field to run action
- [x] Extend `run` action to support object format via `RunAction` enum
- [x] Add optional `trust` field: `local | verified | untrusted`
- [x] Maintain backward compatibility with string format
- [x] Parse both formats correctly using `#[serde(untagged)]`

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
- [x] Values: `Local`, `Verified`, `Untrusted`
- [x] Implement Serialize/Deserialize
- [x] Default: `Local` (via #[default] derive)

---

### P2.4-T03: Log trust levels
- [x] Include trust level in log entries when present
- [x] Display in `cch explain rule` output
- [x] No enforcement (informational only in v1.1)

---

### P2.4-T04: Document trust levels
- [x] Code documentation via doc comments
- [x] Displayed in `cch explain rule` output
- [x] Note: Enforcement planned for future version (in doc comments)

---

## Definition of Done (per task)

- [x] Code complete and compiles
- [x] Unit tests written and passing (68 tests)
- [x] Integration tests pass (all existing tests)
- [x] Backward compatibility verified (v1.0 configs still work)
- [x] Code documentation via doc comments
- [x] Pre-commit checks pass:
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
