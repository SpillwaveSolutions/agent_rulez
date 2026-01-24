# Phase 2 Governance Implementation Plan

**Feature ID:** phase2-governance
**Status:** Planned
**Created:** 2026-01-24
**Estimated Duration:** 5.5-9 days

---

## Executive Summary

Phase 2 Governance transforms CCH from a "powerful local hook system" into a **deterministic, auditable AI policy engine** suitable for organizational governance. This upgrade adds policy modes (enforce/warn/audit), rule priority, metadata provenance, and enhanced logging—all while maintaining 100% backward compatibility with v1.0.0 configurations.

### Strategic Value

| Capability | Enterprise Value |
|------------|------------------|
| Policy Modes | Safe rollout of new policies without production impact |
| Rule Priority | Deterministic, predictable policy ordering |
| Metadata | Audit trail, compliance evidence, provenance tracking |
| Enhanced Logging | Dashboards, metrics, security auditing |

---

## Architecture Overview

### High-Level Design

```
┌─────────────────────────────────────────────────────────────┐
│                    CCH Hook Processor                        │
├─────────────────────────────────────────────────────────────┤
│  1. Parse Event (stdin JSON)                                 │
│  2. Load Rules (with priority, mode, metadata)               │
│  3. Sort by Priority (descending)                            │
│  4. Match Rules (AND within, OR across)                      │
│  5. Resolve Conflicts (enforce > warn > audit)               │
│  6. Execute Actions (mode-aware)                             │
│  7. Log Decision (enhanced schema)                           │
│  8. Output Response (stdout JSON)                            │
└─────────────────────────────────────────────────────────────┘
```

### New Data Models

```rust
// Phase 2 Model Additions (models/mod.rs)

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum PolicyMode {
    #[default]
    Enforce,
    Warn,
    Audit,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Confidence {
    High,
    Medium,
    Low,
}

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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum Decision {
    Allowed,
    Blocked,
    Warned,
    Audited,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum TrustLevel {
    Local,
    Verified,
    Untrusted,
}
```

### Rule Struct Extension

```rust
// Extended Rule struct
pub struct Rule {
    // Existing fields (unchanged)
    pub name: String,
    pub event: Option<EventType>,
    pub matchers: Option<Matchers>,
    pub actions: Actions,
    
    // NEW: Phase 2 Governance fields
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mode: Option<PolicyMode>,           // Default: Enforce
    
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub priority: Option<i32>,              // Default: 0
    
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<RuleMetadata>,     // Optional provenance
}
```

---

## Technology Stack

| Component | Technology | Rationale |
|-----------|------------|-----------|
| Language | Rust 2024 edition | Existing stack, memory safety |
| Serialization | serde + serde_yaml | Existing, backward compatible |
| CLI | clap derive | Existing, extensible |
| Testing | cargo test | Standard Rust testing |
| Logging | tracing | Existing structured logging |

### Dependencies (No New Additions)

Phase 2 requires no new dependencies. All features implemented with existing crates:
- `serde` - Enum serialization for PolicyMode, Decision, TrustLevel
- `clap` - Extended CLI for `cch explain rule --json`
- `chrono` - Timestamp handling for activity stats

---

## Implementation Phases

### Phase 2.1: Core Governance (3-4 days)

**Goal:** Add mode, priority, and metadata to rule processing

#### P2.1-T01: Add PolicyMode enum (0.5 day)
**File:** `cch_cli/src/models/mod.rs`

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

**Tests:**
- Parse "enforce", "warn", "audit" (case-insensitive)
- Default to Enforce when absent
- Serialize to lowercase in JSON output

#### P2.1-T02: Add RuleMetadata struct (0.5 day)
**File:** `cch_cli/src/models/mod.rs`

Add Confidence enum and RuleMetadata struct with all optional fields.

**Tests:**
- Parse complete metadata block
- Parse partial metadata (only some fields)
- Skip serialization of None fields

#### P2.1-T03: Extend Rule struct (0.5 day)
**File:** `cch_cli/src/models/mod.rs`

Add `mode`, `priority`, `metadata` fields with `#[serde(default)]`.

**Tests:**
- Existing v1.0 configs parse unchanged (backward compatibility)
- New configs with governance fields parse correctly
- Mixed configs (some rules with, some without) work

#### P2.1-T04: Implement priority-based sorting (0.5 day)
**File:** `cch_cli/src/hooks/processor.rs`

```rust
pub fn sort_rules_by_priority(rules: &mut [Rule]) {
    rules.sort_by(|a, b| {
        let priority_a = a.priority.unwrap_or(0);
        let priority_b = b.priority.unwrap_or(0);
        priority_b.cmp(&priority_a) // Descending: higher first
    });
}
```

**Tests:**
- Higher priority rules run first
- Same priority preserves file order (stable sort)
- Default priority (0) sorts after explicit priorities

#### P2.1-T05: Implement mode-based execution (1 day)
**File:** `cch_cli/src/hooks/actions.rs`

```rust
fn execute_with_mode(rule: &Rule, action: &Action, event: &Event) -> ActionResult {
    let mode = rule.mode.unwrap_or_default();
    
    match mode {
        PolicyMode::Enforce => execute_action_impl(action, event),
        PolicyMode::Warn => {
            if action.is_blocking() {
                ActionResult::Warning(format!(
                    "Rule '{}' would block: {}", 
                    rule.name, 
                    action.block_reason()
                ))
            } else {
                execute_action_impl(action, event)
            }
        }
        PolicyMode::Audit => ActionResult::Audited,
    }
}
```

**Tests:**
- Enforce mode: blocks work, injections work
- Warn mode: blocks become warnings, injections still work
- Audit mode: no action, log only

#### P2.1-T06: Implement conflict resolution (0.5 day)
**File:** `cch_cli/src/hooks/resolver.rs` (new file)

```rust
pub fn resolve_conflicts(matched_rules: &[&Rule]) -> ResolvedOutcome {
    // Separate by mode
    let enforces: Vec<_> = matched_rules.iter()
        .filter(|r| r.mode.unwrap_or_default() == PolicyMode::Enforce)
        .collect();
    
    if !enforces.is_empty() {
        // Enforce mode wins, use highest priority
        let winner = enforces.iter()
            .max_by_key(|r| r.priority.unwrap_or(0))
            .unwrap();
        return ResolvedOutcome::Enforce(winner);
    }
    
    // Similar logic for warn, then audit...
}
```

**Tests:**
- enforce + warn → enforce wins
- enforce + audit → enforce wins
- warn + audit → warn wins
- Multiple enforces → highest priority wins

---

### Phase 2.2: Enhanced Logging (1-2 days)

**Goal:** Include governance fields in log entries

#### P2.2-T01: Add Decision enum (0.25 day)
**File:** `cch_cli/src/models/mod.rs`

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

#### P2.2-T02: Extend LogEntry struct (0.25 day)
**File:** `cch_cli/src/logging/mod.rs`

Add new fields with skip_serializing_if for backward compatibility:
- `mode: Option<PolicyMode>`
- `priority: Option<i32>`
- `decision: Option<Decision>`
- `metadata: Option<RuleMetadata>`

#### P2.2-T03: Update log writer (0.5 day)
**File:** `cch_cli/src/logging/writer.rs`

Populate new fields when creating log entries from rule matches.

**Tests:**
- Log entries include mode when present
- Log entries include decision
- Old log parsers don't break (additive fields)

#### P2.2-T04: Update log querying (0.5 day)
**File:** `cch_cli/src/cli/logs.rs`

Add new filter flags:
- `--mode <enforce|warn|audit>`
- `--decision <allowed|blocked|warned|audited>`

---

### Phase 2.3: CLI Enhancements (1-2 days)

**Goal:** Enhanced rule explanation with governance details

#### P2.3-T01: Enhance `cch explain rule` (0.5 day)
**File:** `cch_cli/src/cli/explain.rs`

Display format:
```
Rule: block-force-push
Event: PreToolUse
Mode: enforce (default)
Priority: 100

Matchers:
  tools: [Bash]
  command_match: "git push.*--force"

Action:
  block: true
  reason: "Force push is prohibited"

Metadata:
  author: security-team
  created_by: infra-skill@1.2.0
  reason: Enforce Git workflow standards
  confidence: high
  last_reviewed: 2025-01-21
  ticket: PLAT-3421
  tags: [security, git, compliance]
```

#### P2.3-T02: Add activity statistics (0.5 day)
**File:** `cch_cli/src/cli/explain.rs`

Parse recent log entries for the rule:
```
Recent Activity:
  Triggered: 14 times
  Blocked: 3 times
  Warned: 2 times
  Audited: 9 times
  Last trigger: 2025-01-20 14:32
```

Add `--no-stats` flag to skip log parsing.

#### P2.3-T03: Add JSON output format (0.5 day)
**File:** `cch_cli/src/cli/explain.rs`

Add `--json` flag for machine-readable output:
```bash
cch explain rule block-force-push --json
```

#### P2.3-T04: Update help text (0.25 day)
Update all CLI help to document governance features.

---

### Phase 2.4: Trust Levels (0.5-1 day)

**Goal:** Informational trust marking for validators

#### P2.4-T01: Add trust field to run action (0.25 day)
**File:** `cch_cli/src/models/actions.rs`

Support both formats:
```yaml
# Simple (existing)
run: .claude/validators/check.py

# Extended (new)
run:
  script: .claude/validators/check.py
  trust: local
```

#### P2.4-T02: Create TrustLevel enum (0.25 day)
**File:** `cch_cli/src/models/mod.rs`

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum TrustLevel {
    Local,
    Verified,
    Untrusted,
}
```

#### P2.4-T03: Log trust levels (0.25 day)
Include in log entries, display in `cch explain rule`.

#### P2.4-T04: Document trust levels (0.25 day)
Update SKILL.md and schema documentation.

---

## Milestone Summary

| Phase | Tasks | Duration | Dependencies |
|-------|-------|----------|--------------|
| P2.1: Core Governance | 6 tasks | 3-4 days | None |
| P2.2: Enhanced Logging | 4 tasks | 1-2 days | P2.1 |
| P2.3: CLI Enhancements | 4 tasks | 1-2 days | P2.2 |
| P2.4: Trust Levels | 4 tasks | 0.5-1 day | P2.1 |

**Total: 5.5-9 days**

### Dependency Graph

```
P2.1-T01 (PolicyMode) ──┬──> P2.1-T03 (Rule struct)
P2.1-T02 (Metadata)  ───┘         │
                                  ▼
                          P2.1-T04 (Sorting)
                                  │
                                  ▼
                          P2.1-T05 (Mode execution)
                                  │
                                  ▼
                          P2.1-T06 (Conflict resolution)
                                  │
                    ┌─────────────┼─────────────┐
                    ▼             ▼             ▼
              P2.2-T01      P2.3-T01      P2.4-T01
              (Decision)   (explain)     (trust)
                    │             │             │
                    ▼             ▼             ▼
              P2.2-T02      P2.3-T02      P2.4-T02
                    │             │             │
                    ▼             ▼             ▼
              P2.2-T03      P2.3-T03      P2.4-T03
                    │             │             │
                    ▼             ▼             ▼
              P2.2-T04      P2.3-T04      P2.4-T04
```

---

## Testing Strategy

### Unit Tests (New)

| Test File | Coverage |
|-----------|----------|
| `models/policy_mode_test.rs` | PolicyMode parsing, defaults |
| `models/metadata_test.rs` | RuleMetadata parsing |
| `models/confidence_test.rs` | Confidence enum parsing |
| `hooks/sorting_test.rs` | Priority sorting |
| `hooks/resolver_test.rs` | Conflict resolution |
| `models/decision_test.rs` | Decision serialization |
| `models/trust_test.rs` | TrustLevel parsing |

### Integration Tests (New)

| Test File | Scenario |
|-----------|----------|
| `tests/mode_enforce.rs` | Enforce mode blocking |
| `tests/mode_warn.rs` | Warn mode warning injection |
| `tests/mode_audit.rs` | Audit mode logging only |
| `tests/priority_ordering.rs` | Priority-based rule ordering |
| `tests/conflict_resolution.rs` | Multi-mode conflict scenarios |
| `tests/enhanced_logging.rs` | Log entry format |
| `tests/explain_rule.rs` | CLI explain command |
| `tests/backward_compat.rs` | v1.0 config compatibility |

### Test Coverage Target

| Category | Target |
|----------|--------|
| New code | > 90% |
| Overall | > 85% |

---

## Risk Assessment

| Risk | Likelihood | Impact | Mitigation |
|------|------------|--------|------------|
| Backward compatibility break | Low | High | All new fields optional with defaults |
| Performance regression | Low | Medium | Benchmark sorting and mode checks |
| Conflict resolution edge cases | Medium | Medium | Comprehensive test matrix |
| Log entry size growth | Low | Low | skip_serializing_if for optional fields |

---

## Quality Gates

### Before Each PR

```bash
cd cch_cli
cargo fmt --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test
```

### Before Phase Completion

- [ ] All unit tests pass
- [ ] All integration tests pass
- [ ] Performance benchmarks within targets
- [ ] Documentation updated
- [ ] CHANGELOG.md updated

### Performance Targets

| Metric | Target |
|--------|--------|
| Priority sorting | < 0.1ms for 100 rules |
| Mode check | < 0.01ms per rule |
| Conflict resolution | < 0.1ms for 10 matched rules |
| Log entry serialization | < 0.1ms per entry |

---

## Deliverables

### Code Artifacts

1. **Models** - PolicyMode, RuleMetadata, Decision, TrustLevel enums
2. **Processor** - Priority sorting, mode-aware execution
3. **Resolver** - Conflict resolution logic
4. **Logging** - Enhanced LogEntry with governance fields
5. **CLI** - Enhanced `cch explain rule` with `--json` and activity stats

### Documentation Artifacts

1. **Updated SKILL.md** - Governance features documentation
2. **Schema changes** - hooks.yaml schema with new fields
3. **Migration guide** - (optional) upgrading from v1.0 to v1.1
4. **CHANGELOG.md** - v1.1.0 release notes

---

## Success Metrics

| Metric | Target |
|--------|--------|
| Backward compatibility | 100% v1.0 configs work unchanged |
| Test coverage (new code) | > 90% |
| Performance overhead | < 0.5ms per event |
| All CI checks | Pass |
| Documentation | Complete for all features |

---

## Open Questions

| Question | Status | Decision |
|----------|--------|----------|
| Warn mode: inject or just log? | Resolved | Inject warning context |
| Default priority? | Resolved | 0 |
| Trust enforcement in v1.1? | Resolved | Informational only |
| Activity stats from logs? | Resolved | Yes, parse recent logs |
| Log rotation handling? | Open | Skip if log file too large (>10MB) |

---

## Next Steps

1. **Create feature branch**: `git checkout -b feature/phase2-governance`
2. **Start with P2.1-T01**: Add PolicyMode enum
3. **Iterate through tasks**: Following dependency graph
4. **Run checks after each task**: Pre-commit validation
5. **Create PR**: When phase complete
6. **Merge after review**: Update CHANGELOG

---

## References

- [Phase 2 PRD](../../../docs/prds/phase2_prd.md)
- [Phase 2 Spec](./spec.md)
- [Phase 2 Tasks](./tasks.md)
- [Constitution](../../constitution.md)
