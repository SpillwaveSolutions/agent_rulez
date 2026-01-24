# Phase 2 Governance Quality Checklist

**Feature ID:** phase2-governance
**Generated:** 2026-01-24
**Status:** Pre-Implementation

---

## Pre-Implementation Checklist

### Environment Readiness
- [ ] Rust toolchain up to date (`rustup update`)
- [ ] CCH v1.0.0 codebase checked out
- [ ] All existing tests pass (`cargo test`)
- [ ] Clippy reports no warnings
- [ ] Cargo fmt applied

### Understanding Verification
- [ ] Reviewed spec.md thoroughly
- [ ] Reviewed plan.md for dependencies
- [ ] Understood backward compatibility requirements
- [ ] Reviewed existing Rule struct implementation
- [ ] Reviewed existing LogEntry struct implementation

---

## User Story Acceptance Checklists

### US-GOV-01: Rule Metadata (Provenance)

#### Functional Requirements
- [ ] Rules support optional `metadata` block
- [ ] `author` field parses correctly (String)
- [ ] `created_by` field parses correctly (String)
- [ ] `reason` field parses correctly (String)
- [ ] `confidence` field parses correctly (high/medium/low)
- [ ] `last_reviewed` field parses correctly (String date)
- [ ] `ticket` field parses correctly (String)
- [ ] `tags` field parses correctly (Vec<String>)
- [ ] Metadata is ignored by matcher engine (no runtime impact)
- [ ] Metadata is included in log entries
- [ ] Metadata is displayed by `cch explain rule <name>`

#### Backward Compatibility
- [ ] Existing configs without metadata parse correctly
- [ ] Partial metadata (some fields only) parses correctly
- [ ] Empty metadata block `metadata: {}` handled

#### Edge Cases
- [ ] Very long reason strings (>1000 chars)
- [ ] Special characters in author name
- [ ] Empty tags array `tags: []`
- [ ] Invalid confidence value → clear error message

---

### US-GOV-02: Policy Modes

#### Functional Requirements
- [ ] Rules support optional `mode` field
- [ ] `enforce` mode works (current behavior)
- [ ] `warn` mode: Never blocks, injects warning instead
- [ ] `audit` mode: No injection, no blocking, logs only
- [ ] Default mode is `enforce` when not specified
- [ ] Mode is case-insensitive (`Enforce`, `ENFORCE`, `enforce`)
- [ ] Mode is included in log entries
- [ ] Mode is displayed by `cch explain rule <name>`

#### Mode Behavior Verification
| Test Case | Mode | Expected |
|-----------|------|----------|
| Block action | enforce | Blocks |
| Block action | warn | Injects warning, doesn't block |
| Block action | audit | Logs only, no action |
| Inject action | enforce | Injects |
| Inject action | warn | Injects |
| Inject action | audit | Logs only |
| Run action | enforce | Runs validator |
| Run action | warn | Runs validator |
| Run action | audit | Logs only |

#### Edge Cases
- [ ] Invalid mode value → clear parse error
- [ ] Mode + block_if_match combination works correctly

---

### US-GOV-03: Rule Priority

#### Functional Requirements
- [ ] Rules support optional `priority` field (integer)
- [ ] Higher numbers run first
- [ ] Default priority is 0
- [ ] Rules sorted by: 1) priority (desc), 2) file order (stable)
- [ ] Priority is included in log entries
- [ ] Priority is displayed by `cch explain rule <name>`

#### Sorting Verification
- [ ] Priority 100 runs before priority 50
- [ ] Priority 50 runs before priority 0 (default)
- [ ] Same priority preserves file order
- [ ] Negative priorities allowed and work correctly

#### Edge Cases
- [ ] Very large priority (i32::MAX)
- [ ] Negative priority (-100)
- [ ] All rules same priority → file order preserved
- [ ] Invalid priority (non-integer) → clear parse error

---

### US-GOV-04: Policy Conflict Resolution

#### Functional Requirements
- [ ] Conflict resolution follows explicit rules (not emergent)
- [ ] `enforce` mode wins over `warn` and `audit`
- [ ] Among same modes, higher priority wins
- [ ] Multiple blocks: highest priority block message used
- [ ] Conflict resolution logged for debugging

#### Conflict Resolution Matrix
| Scenario | Expected Winner |
|----------|-----------------|
| enforce(100) + warn(50) | enforce(100) |
| enforce(50) + warn(100) | enforce(50) - mode wins over priority |
| audit(100) + enforce(50) | enforce(50) |
| warn(100) + warn(50) | warn(100) - higher priority |
| audit(100) + audit(50) | audit(100) - higher priority |
| enforce(100) + enforce(50) | enforce(100) - higher priority message |

---

### US-GOV-05: Enhanced `cch explain rule` Command

#### Functional Requirements
- [ ] Command: `cch explain rule <rule-name>`
- [ ] Displays: name correctly
- [ ] Displays: event type correctly
- [ ] Displays: mode (with default indicator)
- [ ] Displays: priority (with default indicator)
- [ ] Displays: matchers configuration
- [ ] Displays: action configuration
- [ ] Displays: full metadata block
- [ ] Displays: recent activity (trigger count, block count, last trigger)
- [ ] Supports `--json` output format
- [ ] Supports `--no-stats` flag

#### Edge Cases
- [ ] Rule not found → clear error message
- [ ] Rule with no metadata → shows "No metadata"
- [ ] No log entries → shows "No recent activity"
- [ ] Very old log entries → handles gracefully
- [ ] Log file missing → graceful degradation

---

### US-GOV-06: Enhanced Logging Schema

#### Functional Requirements
- [ ] Log entries include `mode` field when present
- [ ] Log entries include `priority` field when present
- [ ] Log entries include `metadata` block (if present)
- [ ] Log entries include `decision` field (allowed/blocked/warned/audited)
- [ ] JSON Lines format maintained
- [ ] Backward compatible (new fields are additive)

#### Log Entry Verification
```json
{
  "timestamp": "required",
  "session_id": "required",
  "event": "required",
  "rule_name": "required",
  "mode": "optional - only if rule has mode",
  "priority": "optional - only if rule has priority",
  "decision": "required for matched rules",
  "metadata": "optional - only if rule has metadata"
}
```

#### Backward Compatibility
- [ ] Existing log parsers don't break
- [ ] Optional fields use `skip_serializing_if = "Option::is_none"`
- [ ] Log file format still valid JSON Lines

---

### US-GOV-07: Validator Trust Levels

#### Functional Requirements
- [ ] `run` action supports optional `trust` field
- [ ] Trust levels: `local | verified | untrusted`
- [ ] v1.1: Informational only (no enforcement)
- [ ] Trust level logged in entries
- [ ] Both simple and extended formats work

#### Format Compatibility
```yaml
# Simple format (must still work)
actions:
  run: .claude/validators/check.py

# Extended format (new)
actions:
  run:
    script: .claude/validators/check.py
    trust: local
```

---

## Technical Quality Checklists

### Code Quality (Rust)
- [ ] No unsafe code blocks
- [ ] All new types derive necessary traits (Debug, Clone, Serialize, Deserialize)
- [ ] Error handling with anyhow::Result
- [ ] No unwrap() on Option/Result in production code
- [ ] Proper use of Option<T> for optional fields
- [ ] All public APIs documented with doc comments

### Testing
- [ ] Unit tests for PolicyMode parsing
- [ ] Unit tests for RuleMetadata parsing
- [ ] Unit tests for Confidence enum parsing
- [ ] Unit tests for priority sorting
- [ ] Unit tests for conflict resolution
- [ ] Unit tests for Decision enum
- [ ] Unit tests for TrustLevel enum
- [ ] Integration tests for mode=enforce behavior
- [ ] Integration tests for mode=warn behavior
- [ ] Integration tests for mode=audit behavior
- [ ] Integration tests for enhanced logging
- [ ] Integration tests for `cch explain rule`
- [ ] Backward compatibility tests with v1.0 configs
- [ ] Test coverage > 90% for new code

### Performance
- [ ] Processing overhead < 0.5ms per event
- [ ] Memory overhead < 1KB per rule for metadata
- [ ] Log entry size < 2KB average with full metadata
- [ ] Priority sorting < 0.1ms for 100 rules

### Documentation
- [ ] SKILL.md updated with governance features
- [ ] hooks.yaml schema documented
- [ ] CHANGELOG.md updated
- [ ] CLI help text updated

---

## Pre-Commit Checklist (Per Task)

```bash
cd cch_cli
cargo fmt --check        # Must pass
cargo clippy --all-targets --all-features -- -D warnings  # Must pass
cargo test               # All tests must pass
```

### Code Review
- [ ] Self-review completed
- [ ] Follows existing code patterns
- [ ] No TODO comments without issue reference
- [ ] Error messages are user-friendly

---

## Pre-Merge Checklist (Per Phase)

### Phase 2.1: Core Governance
- [ ] PolicyMode enum implemented and tested
- [ ] RuleMetadata struct implemented and tested
- [ ] Rule struct extended with new fields
- [ ] Priority sorting implemented and tested
- [ ] Mode-based execution implemented and tested
- [ ] Conflict resolution implemented and tested
- [ ] All P2.1 tests pass
- [ ] Backward compatibility verified

### Phase 2.2: Enhanced Logging
- [ ] Decision enum implemented
- [ ] LogEntry extended with new fields
- [ ] Log writer updated
- [ ] Log querying updated with new filters
- [ ] All P2.2 tests pass
- [ ] Log format backward compatible

### Phase 2.3: CLI Enhancements
- [ ] `cch explain rule` enhanced
- [ ] Activity statistics implemented
- [ ] `--json` output format works
- [ ] Help text updated
- [ ] All P2.3 tests pass

### Phase 2.4: Trust Levels
- [ ] TrustLevel enum implemented
- [ ] Run action extended with trust field
- [ ] Trust logged in entries
- [ ] Documentation updated
- [ ] All P2.4 tests pass

---

## Pre-Release Checklist (v1.1.0)

### Functionality
- [ ] All 7 user stories acceptance criteria met
- [ ] All 64+ existing tests still pass
- [ ] All new tests pass
- [ ] Manual testing of each governance feature

### Backward Compatibility
- [ ] v1.0 configs parse without changes
- [ ] v1.0 log parsers work with new logs
- [ ] No breaking changes to CLI interface
- [ ] Defaults preserve v1.0 behavior

### Performance
- [ ] Benchmark: event processing < 10ms (including governance overhead)
- [ ] Benchmark: priority sorting < 0.1ms for 100 rules
- [ ] Memory: no leaks in 24-hour test

### Documentation
- [ ] CHANGELOG.md complete for v1.1.0
- [ ] SKILL.md governance section complete
- [ ] hooks.yaml schema updated
- [ ] Migration notes (if any)

### Release
- [ ] Version bumped in Cargo.toml
- [ ] Git tag created: `v1.1.0`
- [ ] GitHub release with binaries
- [ ] Release notes published

---

## Regression Test Suite

### Critical Paths
1. [ ] v1.0 config → parse → match → execute → log (unchanged behavior)
2. [ ] v1.1 config with mode=enforce → blocks correctly
3. [ ] v1.1 config with mode=warn → warns correctly
4. [ ] v1.1 config with mode=audit → logs only
5. [ ] Priority sorting → higher priority runs first
6. [ ] `cch explain rule` → displays all fields
7. [ ] Log entries → contain all governance fields

### Edge Cases
1. [ ] Mixed v1.0 and v1.1 rules in same config
2. [ ] Rule with all governance fields
3. [ ] Rule with no governance fields
4. [ ] Empty metadata block
5. [ ] Invalid mode value → parse error
6. [ ] Conflict between 10+ matching rules

### Error Scenarios
1. [ ] Invalid mode → clear error with line number
2. [ ] Invalid confidence → clear error with line number
3. [ ] Invalid trust level → clear error with line number
4. [ ] Malformed metadata → clear error with context
