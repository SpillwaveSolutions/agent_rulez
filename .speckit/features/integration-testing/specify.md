# Integration Testing Framework with IQ/OQ/PQ Validation

**Feature ID:** integration-testing  
**Status:** Implemented - Validation Framework Enhancement Required  
**Created:** 2025-01-23  
**Updated:** 2025-01-24  
**Source:** 
- [CRD-integration-testing.md](../../../docs/prds/change_requests/CRD-integration-testing.md)
- [IQ_OQ_PQ_IntegrationTesting.md](../../../docs/IQ_OQ_PQ_IntegrationTesting.md)  
**Review Date:** 2025-01-24  
**Reviewer:** Claude (SDD Integration)

---

## 1. Overview

Create a comprehensive validation framework for CCH (Claude Context Hooks) that combines end-to-end integration testing with Installation Qualification (IQ), Operational Qualification (OQ), and Performance Qualification (PQ). This framework ensures CCH works correctly across all supported platforms and meets strict engineering standards for AI governance tools.

---

## 2. Problem Statement

CCH is an AI policy enforcement tool where failures can have severe consequences:
- A missed block on a force push could corrupt production repositories
- Failed context injection might lead AI agents to make uninformed, dangerous decisions
- Incomplete audit logs could create compliance gaps or security blind spots

Current testing gaps:
1. Unit tests validate individual components but not end-to-end behavior
2. No cross-platform installation validation (IQ)
3. No operational scenario testing with evidence capture (OQ)
4. Performance benchmarks exist but lack formal qualification process (PQ)
5. Integration tests use soft assertions that rarely fail

---

## 3. Validation Philosophy

**From the IQ/OQ/PQ Guide:**
> "When you build tools designed to enforce governance and oversight in high-stakes environments, those tools must themselves embody the same quality standards they're meant to protect."

This means:
- **Shift left on quality**: Validation from day one, not an afterthought
- **Alignment**: Tools that enforce rules must be built following strict rules
- **Evidence-based trust**: Every validation generates auditable artifacts
- **Cross-platform reliability**: Works everywhere, every time

---

## 4. Requirements

### 4.1 Installation Qualification (IQ) Requirements

| ID | Requirement | Priority |
|----|-------------|----------|
| IQ-001 | Validate installation on macOS Apple Silicon (ARM64) | High |
| IQ-002 | Validate installation on macOS Intel (x86_64) | High |
| IQ-003 | Validate installation on Windows (path handling, registry) | High |
| IQ-004 | Validate installation on Linux (multiple distros) | High |
| IQ-005 | Verify `cch --version` returns correct version | High |
| IQ-006 | Verify `cch init` creates `.claude/hooks.yaml` correctly | High |
| IQ-007 | Verify log directory creation at `~/.claude/logs/` | High |
| IQ-008 | Verify CCH hook registration with Claude settings.json | High |
| IQ-009 | Generate IQ evidence report with timestamps and artifacts | Medium |
| IQ-010 | Automate IQ via CI/CD with platform-specific runners | Medium |

### 4.2 Operational Qualification (OQ) Requirements

| ID | Requirement | Priority |
|----|-------------|----------|
| OQ-001 | Test rule evaluation for `PreToolUse` events | High |
| OQ-002 | Test rule evaluation for `PostToolUse` events | High |
| OQ-003 | Test rule evaluation for `PermissionRequest` events | High |
| OQ-004 | Test rule evaluation for `SessionStart` events | Medium |
| OQ-005 | Verify `block` action prevents tool execution | High |
| OQ-006 | Verify `inject` action adds context to Claude | High |
| OQ-007 | Verify `warn` mode logs but allows operations | High |
| OQ-008 | Verify `run` action executes validator scripts | Medium |
| OQ-009 | Test complex rule conditions (tool + directory + regex) | High |
| OQ-010 | Verify JSON audit log format and required fields | High |
| OQ-011 | Generate OQ evidence with test case results | Medium |

### 4.3 Performance Qualification (PQ) Requirements

| ID | Requirement | Priority |
|----|-------------|----------|
| PQ-001 | Cold start time < 15ms (release build) | High |
| PQ-002 | Event processing < 50ms per event | High |
| PQ-003 | Memory usage < 10MB RSS under normal operation | High |
| PQ-004 | Sustained throughput > 100 events/second | Medium |
| PQ-005 | No memory leaks over 7-day stress test | Medium |
| PQ-006 | Performance consistent across all platforms (+/- 20%) | Medium |
| PQ-007 | Timing fields in responses accurate (processing_ms) | High |
| PQ-008 | Performance benchmarks with evidence capture | Medium |

### 4.4 Integration Testing Requirements

| ID | Requirement | Priority |
|----|-------------|----------|
| FR-001 | Test framework uses Bash scripts for simplicity and portability | High |
| FR-002 | Master test runner (`run-all.sh`) orchestrates all test cases | High |
| FR-003 | Shared helper functions library (`test-helpers.sh`) | High |
| FR-004 | Tests invoke real Claude CLI via `claude -p` with `--allowedTools` | High |
| FR-005 | Each test case has setup, install, execute, verify, cleanup phases | High |
| FR-006 | Test results saved as JSON in `results/` directory | Medium |
| FR-007 | Tests fail gracefully with clear messages when Claude CLI missing | High |
| FR-008 | Quick test mode skips slow tests for rapid feedback | Low |
| FR-009 | Strict assertion mode for CI/CD (fail on any issue) | High |
| FR-010 | Timeout on Claude CLI calls to prevent hanging | High |

---

## 5. Test Cases

### 5.1 IQ Test Cases

| ID | Test Case | Purpose |
|----|-----------|---------|
| IQ-TC-001 | binary-exists | Verify CCH binary is in PATH |
| IQ-TC-002 | version-check | Verify `--version` returns expected format |
| IQ-TC-003 | init-creates-config | Verify `cch init` creates hooks.yaml |
| IQ-TC-004 | install-registers-hook | Verify `cch install` updates settings.json |
| IQ-TC-005 | log-directory-created | Verify log directory is created |
| IQ-TC-006 | validate-passes | Verify `cch validate` passes on valid config |

### 5.2 OQ Test Cases (Integration Tests)

| ID | Test Case | Purpose |
|----|-----------|---------|
| OQ-TC-001 | block-force-push | Verify CCH blocks `git push --force` operations |
| OQ-TC-002 | context-injection | Verify CCH injects context for `.cdk.ts` files |
| OQ-TC-003 | session-logging | Verify CCH creates JSON Lines audit logs |
| OQ-TC-004 | permission-explanations | Verify CCH provides context on permission requests |
| OQ-TC-005 | multi-condition-rule | Verify rules with multiple conditions work |
| OQ-TC-006 | warn-mode-allows | Verify warn mode logs but doesn't block |

### 5.3 PQ Test Cases

| ID | Test Case | Purpose |
|----|-----------|---------|
| PQ-TC-001 | cold-start-version | Measure cold start time with `--version` |
| PQ-TC-002 | cold-start-help | Measure cold start time with `--help` |
| PQ-TC-003 | event-processing | Measure event processing latency |
| PQ-TC-004 | timing-in-response | Verify timing fields in response |
| PQ-TC-005 | throughput-with-rules | Measure throughput with 20+ rules |
| PQ-TC-006 | memory-baseline | Measure memory usage baseline |

---

## 6. User Stories

### US-001: Developer Validates Cross-Platform Installation
**As a** CCH developer  
**I want to** run IQ tests across all platforms via CI/CD  
**So that** I can ensure CCH installs correctly everywhere

**Acceptance Criteria:**
- IQ tests run on macOS (ARM64 + Intel), Windows, and Linux
- Each platform generates IQ evidence report
- CI/CD fails if any IQ check fails

### US-002: Developer Tests Blocking Rules End-to-End
**As a** CCH developer  
**I want to** verify that blocking rules prevent dangerous operations with evidence  
**So that** I can demonstrate CCH works as designed

**Acceptance Criteria:**
- TC-001 verifies `git push --force` is blocked
- CCH logs show the block action and reason
- OQ evidence includes event payloads and log entries

### US-003: Developer Tests Context Injection End-to-End
**As a** CCH developer  
**I want to** verify that context files are injected correctly  
**So that** I can ensure Claude receives the additional context

**Acceptance Criteria:**
- TC-002 verifies context injection for `.cdk.ts` files
- Injected context appears in CCH logs
- Evidence includes before/after comparison

### US-004: Developer Validates Performance Requirements
**As a** CCH developer  
**I want to** run PQ benchmarks with evidence capture  
**So that** I can demonstrate CCH meets <50ms latency requirements

**Acceptance Criteria:**
- PQ tests measure p50, p95, p99 latencies
- Evidence includes benchmark data and statistical analysis
- CI/CD fails if performance exceeds thresholds

### US-005: Release Manager Collects Validation Evidence
**As a** release manager  
**I want to** collect IQ/OQ/PQ evidence for each release  
**So that** I can provide auditable validation reports

**Acceptance Criteria:**
- Evidence stored in `docs/validation/{iq,oq,pq}/{date}/`
- Sign-off template available for approval
- Evidence linked to specific release version

---

## 7. Success Criteria

| ID | Criterion | Metric |
|----|-----------|--------|
| SC-001 | All IQ tests pass on 4 platforms | 100% pass rate |
| SC-002 | All OQ test cases pass | 100% pass rate |
| SC-003 | PQ benchmarks meet requirements | All < 50ms threshold |
| SC-004 | Evidence generated for each phase | Artifacts in docs/validation/ |
| SC-005 | CI/CD integration complete | Tests run on every PR |
| SC-006 | Strict mode catches real failures | No false positives/negatives |

---

## 8. Evidence Collection

### 8.1 IQ Evidence Structure

```
docs/validation/iq/{date}/
├── report.md              # Summary report
├── install.log            # Installation output
├── version.txt            # Version verification
├── config-perms.txt       # File permissions
├── hooks.yaml             # Configuration copy
└── platform-specific/
    ├── codesign.txt       # macOS code signing
    └── registry.txt       # Windows registry
```

### 8.2 OQ Evidence Structure

```
docs/validation/oq/{date}/
├── report.md              # Summary report
├── test-results.json      # All test case results
├── test-cases/
│   ├── OQ-TC-001/
│   │   ├── hooks.yaml     # Test configuration
│   │   ├── event.json     # Input event
│   │   ├── log.json       # CCH log entry
│   │   └── result.json    # Pass/fail with details
│   └── OQ-TC-002/
│       └── ...
└── coverage-summary.md    # Test coverage metrics
```

### 8.3 PQ Evidence Structure

```
docs/validation/pq/{date}/
├── report.md              # Summary report
├── benchmark-results.json # Raw benchmark data
├── latency-analysis.md    # Statistical analysis
├── memory-profile.txt     # Memory usage data
└── platform-comparison.md # Cross-platform comparison
```

---

## 9. Edge Cases & Error Handling

| Edge Case | Expected Behavior |
|-----------|-------------------|
| Claude CLI not installed | Test suite fails with clear error message |
| CCH binary not built | Test runner auto-builds via dependencies |
| Permission prompt appears | Tests timeout after 60 seconds |
| Log directory doesn't exist | Tests create directory as needed |
| Previous test artifacts exist | Tests clean up before and after |
| Claude refuses dangerous command | Test passes with note (expected safe behavior) |
| Platform-specific path issues | Normalize paths in test helpers |

---

## 10. Known Gaps (From Checklist)

| GAP-ID | Description | Impact | Resolution |
|--------|-------------|--------|------------|
| GAP-001 | Soft assertions everywhere | Tests rarely fail even when hooks don't trigger | Add `--strict` mode |
| GAP-002 | No CI/CD workflow | Tests not automated in GitHub Actions | Create workflow file |
| GAP-003 | No timeout on Claude calls | Tests can hang indefinitely | Add 60s timeout |

---

## 11. File Structure

```
test/integration/
├── README.md                    # Documentation
├── run-all.sh                   # Master test runner
├── lib/
│   └── test-helpers.sh          # Shared bash functions
├── use-cases/
│   ├── 01-block-force-push/     # OQ-TC-001
│   ├── 02-context-injection/    # OQ-TC-002
│   ├── 03-session-logging/      # OQ-TC-003
│   └── 04-permission-explanations/  # OQ-TC-004
└── results/                     # Test outputs (gitignored)

cch_cli/tests/
├── iq_installation.rs           # IQ tests in Rust
├── oq_us1_blocking.rs           # OQ blocking tests
├── oq_us2_injection.rs          # OQ injection tests
├── oq_us3_validators.rs         # OQ validator tests
├── oq_us4_permissions.rs        # OQ permission tests
├── oq_us5_logging.rs            # OQ logging tests
└── pq_performance.rs            # PQ benchmark tests

docs/validation/
├── README.md                    # Validation framework docs
├── iq/                          # Installation evidence
├── oq/                          # Operational evidence
├── pq/                          # Performance evidence
└── sign-off/
    └── TEMPLATE-validation-report.md
```

---

## 12. Dependencies

| Dependency | Purpose |
|------------|---------|
| Claude CLI | Tool invocation (`claude -p`) |
| CCH binary | Hook processing |
| Bash shell | Test execution |
| Cargo | Rust test framework for IQ/OQ/PQ |
| GitHub Actions | CI/CD automation |
| jq (optional) | JSON parsing in tests |

---

## 13. Taskfile Integration

```yaml
integration-test:
  desc: Run CCH + Claude CLI integration tests
  aliases: [itest]
  deps: [build]
  cmds:
    - ./test/integration/run-all.sh

integration-test-quick:
  desc: Run quick integration tests (skip slow ones)
  aliases: [itest-quick]
  deps: [build]
  cmds:
    - ./test/integration/run-all.sh --quick

iq-test:
  desc: Run Installation Qualification tests
  cmds:
    - cargo test --release iq_ -- --nocapture

oq-test:
  desc: Run Operational Qualification tests
  cmds:
    - cargo test --release oq_ -- --nocapture

pq-test:
  desc: Run Performance Qualification tests
  cmds:
    - cargo test --release pq_ -- --nocapture

validation-all:
  desc: Run full IQ/OQ/PQ validation suite
  deps: [build]
  cmds:
    - task iq-test
    - task oq-test
    - task pq-test
    - task integration-test
```

---

## 14. Traceability Matrix

| PRD Requirement | Spec Requirement | Test Case |
|-----------------|------------------|-----------|
| Installation works on all platforms | IQ-001 through IQ-004 | IQ-TC-001 through IQ-TC-006 |
| Block dangerous operations | OQ-005 | OQ-TC-001 |
| Inject context files | OQ-006 | OQ-TC-002 |
| Create audit logs | OQ-010 | OQ-TC-003 |
| <50ms latency | PQ-002 | PQ-TC-003 |
| Cross-platform consistency | PQ-006 | All PQ tests on all platforms |

---

## 15. Notes

- Tests use `set -euo pipefail` for strict error handling
- The `using-claude-code-cli` skill was added to AGENTS.md to support this feature
- PQ tests automatically adjust thresholds for debug vs release builds (10x multiplier)
- Evidence collection should be automated via CI/CD artifacts
- Sign-off process required for regulated environments
