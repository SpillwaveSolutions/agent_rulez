# Phase 2 Governance Feature Specification

**Feature ID:** phase2-governance
**Status:** Specified
**Created:** 2026-01-24
**Version:** 1.1
**Source:** docs/prds/phase2_prd.md

---

## Overview

Phase 2 Governance introduces a policy governance layer to CCH, enhancing explainability, auditability, gradual rollout capabilities, and enterprise readiness. This upgrade evolves CCH from "a powerful local hook system" into "a deterministic, auditable AI policy engine."

### Design Philosophy

**LLMs do not enforce policy. LLMs are subject to policy.**

- CCH is the policy authority
- Skills are policy authors
- Claude is policy-constrained execution

### Strategic Positioning

Comparable to:
- **OPA** (but human-readable)
- **Terraform Sentinel** (but local)
- **Kubernetes admission controllers** (but for agents)

### Backward Compatibility

All new features are **optional**. Existing configurations continue to work unchanged.

---

## User Stories

### US-GOV-01: Rule Metadata (Provenance)
**As a** policy administrator
**I want to** attach metadata to rules documenting their origin and purpose
**So that** I can audit and explain why rules exist

**Acceptance Criteria:**
- [ ] Rules support optional `metadata` block
- [ ] Metadata fields: `author`, `created_by`, `reason`, `confidence`, `last_reviewed`, `ticket`, `tags`
- [ ] Metadata is ignored by matcher engine (no runtime impact)
- [ ] Metadata is included in log entries
- [ ] Metadata is displayed by `cch explain rule <name>`

**Example Configuration:**
```yaml
rules:
  - name: block-force-push
    metadata:
      author: "security-team"
      created_by: "aws-cdk-skill@1.2.0"
      reason: "Enforce infrastructure coding standards"
      confidence: high
      last_reviewed: 2025-01-21
      ticket: "PLAT-3421"
      tags: [security, infra, compliance]
    matchers:
      tools: ["Bash"]
      command_match: "git push.*--force"
    actions:
      block: true
```

---

### US-GOV-02: Policy Modes
**As a** policy administrator
**I want to** set rules to enforce, warn, or audit mode
**So that** I can gradually roll out policies and test them safely

**Acceptance Criteria:**
- [ ] Rules support optional `mode` field: `enforce | warn | audit`
- [ ] Default mode is `enforce` (current behavior)
- [ ] `enforce`: Normal blocking behavior
- [ ] `warn`: Never blocks, injects warning context instead
- [ ] `audit`: No injection, no blocking, logs only
- [ ] Mode is included in log entries
- [ ] Mode is displayed by `cch explain rule <name>`

**Mode Behavior Matrix:**

| Mode | Blocks? | Injects? | Logs? |
|------|---------|----------|-------|
| enforce | Yes | Yes | Yes |
| warn | No | Warning only | Yes |
| audit | No | No | Yes |

**Use Cases:**

| Scenario | Mode |
|----------|------|
| Rollout new guardrails | `audit` |
| Soft cultural rules | `warn` |
| Security policy | `enforce` |
| Observability only | `audit` |

---

### US-GOV-03: Rule Priority
**As a** policy administrator
**I want to** define explicit rule priorities
**So that** I have deterministic control over policy ordering

**Acceptance Criteria:**
- [ ] Rules support optional `priority` field (integer)
- [ ] Higher numbers run first
- [ ] Default priority is 0
- [ ] Rules sorted by: 1) priority (desc), 2) file order (stable)
- [ ] Priority is included in log entries
- [ ] Priority is displayed by `cch explain rule <name>`

**Example:**
```yaml
rules:
  - name: security-block
    priority: 100    # Runs first
    actions:
      block: true

  - name: inject-context
    priority: 50     # Runs second
    actions:
      inject: ".claude/context.md"

  - name: log-everything
    priority: 0      # Runs last (default)
    mode: audit
```

---

### US-GOV-04: Policy Conflict Resolution
**As a** policy administrator
**I want** deterministic conflict resolution between rules
**So that** I can predict policy outcomes

**Acceptance Criteria:**
- [ ] Conflict resolution follows explicit rules (not emergent)
- [ ] `enforce` mode wins over `warn` and `audit`
- [ ] Among same modes, higher priority wins
- [ ] Multiple blocks: highest priority block message used
- [ ] Conflict resolution logged for debugging

**Conflict Resolution Table:**

| Situation | Outcome |
|-----------|---------|
| enforce + warn | enforce wins |
| audit + enforce | enforce wins |
| warn only | inject warning |
| audit only | log only |
| multiple enforce | highest priority wins |

---

### US-GOV-05: Enhanced `cch explain rule` Command
**As a** developer
**I want to** see complete rule details including metadata and activity
**So that** I can understand and debug policy behavior

**Acceptance Criteria:**
- [ ] Command: `cch explain rule <rule-name>`
- [ ] Displays: name, event, mode, priority
- [ ] Displays: matchers configuration
- [ ] Displays: action configuration
- [ ] Displays: full metadata block
- [ ] Displays: recent activity (trigger count, block count, last trigger)
- [ ] Supports `--json` output format

**Example Output:**
```
Rule: no-console-log
Event: PreToolUse
Mode: enforce
Priority: 100

Matchers:
  tools: [Edit, Write]
  extensions: [.ts, .js]

Action:
  run: .claude/validators/no-console-log.py

Metadata:
  author: cch-skill
  created_by: react-skill@2.1.0
  reason: Enforce CLAUDE.md rule
  confidence: high
  last_reviewed: 2025-01-21

Recent Activity:
  Triggered 14 times
  Blocked 3 times
  Last trigger: 2025-01-20 14:32
```

---

### US-GOV-06: Enhanced Logging Schema
**As a** security auditor
**I want** comprehensive log entries with governance metadata
**So that** I can build dashboards and generate compliance evidence

**Acceptance Criteria:**
- [ ] Log entries include `mode` field
- [ ] Log entries include `priority` field
- [ ] Log entries include `metadata` block (if present)
- [ ] Log entries include `decision` field (allowed, blocked, warned, audited)
- [ ] JSON Lines format maintained
- [ ] Backward compatible (new fields are additive)

**Enhanced Log Entry Example:**
```json
{
  "timestamp": "2025-01-21T14:32:11Z",
  "session_id": "abc123",
  "event": "PreToolUse",
  "rule_name": "no-console-log",
  "mode": "enforce",
  "priority": 100,
  "decision": "blocked",
  "metadata": {
    "author": "cch-skill",
    "created_by": "react-skill@2.1.0",
    "reason": "CLAUDE.md enforcement"
  }
}
```

---

### US-GOV-07: Validator Trust Levels (Informational)
**As a** security-conscious user
**I want** to mark validators with trust levels
**So that** I can track the provenance of external scripts

**Acceptance Criteria:**
- [ ] `run` action supports optional `trust` field
- [ ] Trust levels: `local | verified | untrusted`
- [ ] v1.1: Informational only (no enforcement)
- [ ] Trust level logged in entries
- [ ] Prepares for future sandboxing/signing

**Example:**
```yaml
actions:
  run:
    script: .claude/validators/check.py
    trust: local
```

**Trust Level Semantics:**

| Trust | Meaning |
|-------|---------|
| local | User-authored script |
| verified | Signed skill package |
| untrusted | External source |

---

## Technical Architecture

### Model Changes

**Rule struct additions:**
```rust
pub struct Rule {
    // Existing fields...
    pub name: String,
    pub matchers: Matchers,
    pub actions: Actions,
    
    // New Phase 2 fields
    pub mode: Option<PolicyMode>,      // enforce | warn | audit
    pub priority: Option<i32>,          // Default: 0
    pub metadata: Option<RuleMetadata>,
}

#[derive(Default)]
pub enum PolicyMode {
    #[default]
    Enforce,
    Warn,
    Audit,
}

pub struct RuleMetadata {
    pub author: Option<String>,
    pub created_by: Option<String>,
    pub reason: Option<String>,
    pub confidence: Option<Confidence>,
    pub last_reviewed: Option<String>,
    pub ticket: Option<String>,
    pub tags: Option<Vec<String>>,
}

pub enum Confidence {
    High,
    Medium,
    Low,
}
```

**LogEntry struct additions:**
```rust
pub struct LogEntry {
    // Existing fields...
    
    // New Phase 2 fields
    pub mode: Option<PolicyMode>,
    pub priority: Option<i32>,
    pub decision: Option<Decision>,
    pub metadata: Option<RuleMetadata>,
}

pub enum Decision {
    Allowed,
    Blocked,
    Warned,
    Audited,
}
```

### Processing Changes

1. **Rule Sorting**: Before matching, sort rules by priority (desc), then file order
2. **Mode Handling**: After match, apply mode-specific behavior
3. **Conflict Resolution**: When multiple rules match, apply resolution logic
4. **Enhanced Logging**: Include all new fields in log entries

### CLI Changes

**Updated `cch explain rule` command:**
- Add `--json` flag for structured output
- Include metadata in output
- Include activity statistics from log analysis

---

## Performance Requirements

| Metric | Target | Notes |
|--------|--------|-------|
| Processing overhead | < 0.5ms | For mode/priority/metadata handling |
| Memory overhead | < 1KB per rule | For metadata storage |
| Log entry size | < 2KB average | With full metadata |

---

## Quality Gates

### Pre-Commit Checks
```bash
cd cch_cli
cargo fmt --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test
```

### Test Coverage
- Unit tests for PolicyMode parsing
- Unit tests for priority sorting
- Unit tests for conflict resolution
- Integration tests for each mode behavior
- Integration tests for enhanced logging
- Integration tests for `cch explain rule`

---

## Backward Compatibility

| Feature | Required? | Default |
|---------|-----------|---------|
| metadata | Optional | None |
| mode | Optional | `enforce` |
| priority | Optional | 0 |
| trust | Optional | None |

**Existing configs work unchanged.** All new features use Option<T> with sensible defaults.

---

## Implementation Phases

### Phase 2.1: Core Governance (3-4 days)
- [ ] Add PolicyMode enum and parsing
- [ ] Add priority field and sorting
- [ ] Add RuleMetadata struct and parsing
- [ ] Update rule matching to respect priority order
- [ ] Update action execution to respect mode

### Phase 2.2: Enhanced Logging (1-2 days)
- [ ] Extend LogEntry with new fields
- [ ] Update log writer to include metadata
- [ ] Add Decision enum
- [ ] Maintain backward compatibility

### Phase 2.3: CLI Enhancements (1-2 days)
- [ ] Update `cch explain rule` command
- [ ] Add activity statistics
- [ ] Add `--json` output format
- [ ] Update help text

### Phase 2.4: Trust Levels (0.5-1 day)
- [ ] Add trust field to run action
- [ ] Parse and log trust levels
- [ ] No enforcement (informational only)

**Total Estimated: 5.5-9 days**

---

## Future Roadmap (Not in Phase 2)

### Policy Packs (Phase 3+)
```yaml
imports:
  - source: "github.com/org/cch-baselines"
    version: ">=1.2.0"
```

### Sandboxing (Phase 3+)
- Enforce trust levels
- Sandbox untrusted validators
- Signature verification

### Policy Distribution (Phase 3+)
- Versioned policy repositories
- Central policy management
- Team sharing

---

## Success Metrics

| Metric | Target |
|--------|--------|
| Backward compatibility | 100% existing configs work |
| Performance overhead | < 0.5ms per event |
| Test coverage | > 90% for new code |
| Documentation | Complete for all new features |

---

## Open Questions

| Question | Status | Decision |
|----------|--------|----------|
| Should warn mode inject or just log? | Resolved | Inject warning context |
| Default priority value? | Resolved | 0 |
| Trust level enforcement in v1.1? | Resolved | Informational only |
| Activity stats from log analysis? | Open | Yes, parse recent logs |
