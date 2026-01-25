# Integration Testing Framework with IQ/OQ/PQ - Tasks

**Feature ID:** integration-testing  
**Status:** In Progress - IQ/OQ/PQ Enhancement  
**Created:** 2025-01-23  
**Updated:** 2025-01-24  
**Source:** [plan.md](./plan.md)

---

## Phase 1: Framework Setup (COMPLETE)

### Task 1.1: Create Directory Structure
**Status:** [x] Complete  
**Complexity:** Low  
**Files:**
- `test/integration/` - Root directory
- `test/integration/lib/` - Shared libraries
- `test/integration/use-cases/` - Test case directories
- `test/integration/results/` - Output directory

### Task 1.2: Create Test Helper Library
**Status:** [x] Complete  
**Complexity:** High  
**Files:** `test/integration/lib/test-helpers.sh`

**Implementation Notes:**
- 445 lines of Bash
- Fixed CCH_BINARY path for workspace builds
- Fixed setup_workspace echo to stderr

### Task 1.3: Create Master Test Runner
**Status:** [x] Complete  
**Complexity:** Medium  
**Files:** `test/integration/run-all.sh`

---

## Phase 2: Integration Test Cases (COMPLETE)

### Task 2.1: Block Force Push Test (OQ-TC-001)
**Status:** [x] Complete  
**Complexity:** Medium  
**Spec:** OQ-005 (Verify block action)  
**Files:**
- `test/integration/use-cases/01-block-force-push/test.sh`
- `test/integration/use-cases/01-block-force-push/.claude/hooks.yaml`

### Task 2.2: Context Injection Test (OQ-TC-002)
**Status:** [x] Complete  
**Complexity:** Medium  
**Spec:** OQ-006 (Verify inject action)  
**Files:**
- `test/integration/use-cases/02-context-injection/test.sh`
- `test/integration/use-cases/02-context-injection/.claude/hooks.yaml`

### Task 2.3: Session Logging Test (OQ-TC-003)
**Status:** [x] Complete  
**Complexity:** Medium  
**Spec:** OQ-010 (Verify JSON audit log format)  
**Files:**
- `test/integration/use-cases/03-session-logging/test.sh`
- `test/integration/use-cases/03-session-logging/.claude/hooks.yaml`

### Task 2.4: Permission Explanations Test (OQ-TC-004)
**Status:** [x] Complete  
**Complexity:** High  
**Spec:** OQ-003 (PermissionRequest events)  
**Files:**
- `test/integration/use-cases/04-permission-explanations/test.sh`
- `test/integration/use-cases/04-permission-explanations/.claude/hooks.yaml`

---

## Phase 3: Installation Qualification (IQ)

### Task 3.1: Create IQ Test Suite
**Status:** [x] Complete  
**Complexity:** Medium  
**Spec:** IQ-001 through IQ-008  
**Files:** `cch_cli/tests/iq_installation.rs`

**Tests Implemented:**
- [x] Binary exists and returns version
- [x] Help command works
- [x] Init creates configuration
- [x] Install registers hooks
- [x] Validate command works
- [x] Uninstall removes hooks
- [x] Logs command works

### Task 3.2: Create IQ Command Tests
**Status:** [x] Complete  
**Complexity:** Medium  
**Files:** `cch_cli/tests/iq_new_commands.rs`

**Tests Implemented:**
- [x] Explain command works
- [x] Logs command shows entries
- [x] Debug mode produces output

### Task 3.3: Add Platform-Specific IQ Tests
**Status:** [ ] Pending  
**Complexity:** High  
**Spec:** IQ-001 through IQ-004  
**Files:** `cch_cli/tests/iq_platform.rs` (new)

**Acceptance Criteria:**
- [ ] macOS ARM64: Code signing verification
- [ ] macOS Intel: x86_64 binary validation
- [ ] Windows: Path separator handling
- [ ] Linux: Multiple distro support

### Task 3.4: Create IQ Evidence Collection Script
**Status:** [ ] Pending  
**Complexity:** Medium  
**Spec:** IQ-009  
**Files:** `scripts/collect-iq-evidence.sh` (new)

**Acceptance Criteria:**
- [ ] Captures installation logs
- [ ] Records environment info
- [ ] Generates markdown report
- [ ] Stores in docs/validation/iq/

---

## Phase 4: Operational Qualification (OQ) - Rust Tests

### Task 4.1: Create OQ Blocking Tests
**Status:** [x] Complete  
**Complexity:** Medium  
**Spec:** OQ-005  
**Files:** `cch_cli/tests/oq_us1_blocking.rs`

**Tests Implemented:**
- [x] Block force push command
- [x] Block hard reset command
- [x] Allow safe commands
- [x] Multiple rules evaluated

### Task 4.2: Create OQ Injection Tests
**Status:** [x] Complete  
**Complexity:** Medium  
**Spec:** OQ-006  
**Files:** `cch_cli/tests/oq_us2_injection.rs`

**Tests Implemented:**
- [x] Inject context for CDK files
- [x] Inject context for Terraform files
- [x] No injection for non-matching files

### Task 4.3: Create OQ Validator Tests
**Status:** [x] Complete  
**Complexity:** Medium  
**Spec:** OQ-008  
**Files:** `cch_cli/tests/oq_us3_validators.rs`

**Tests Implemented:**
- [x] Script execution on matching files
- [x] Block on validator failure
- [x] Timeout handling

### Task 4.4: Create OQ Permission Tests
**Status:** [x] Complete  
**Complexity:** Medium  
**Spec:** OQ-003  
**Files:** `cch_cli/tests/oq_us4_permissions.rs`

**Tests Implemented:**
- [x] PermissionRequest event handling
- [x] Context injection on permission request
- [x] Correct event type routing

### Task 4.5: Create OQ Logging Tests
**Status:** [x] Complete  
**Complexity:** Medium  
**Spec:** OQ-010  
**Files:** `cch_cli/tests/oq_us5_logging.rs`

**Tests Implemented:**
- [x] JSON format verification
- [x] Required fields present
- [x] Timing fields present
- [x] Session ID tracking

### Task 4.6: Add OQ Multi-Condition Tests
**Status:** [ ] Pending  
**Complexity:** Medium  
**Spec:** OQ-009  
**Files:** `cch_cli/tests/oq_us6_complex.rs` (new)

**Acceptance Criteria:**
- [ ] Tool + directory + regex combination
- [ ] Multiple matchers (AND logic)
- [ ] Priority ordering verification

---

## Phase 5: Performance Qualification (PQ)

### Task 5.1: Create PQ Benchmark Suite
**Status:** [x] Complete  
**Complexity:** High  
**Spec:** PQ-001 through PQ-007  
**Files:** `cch_cli/tests/pq_performance.rs`

**Tests Implemented:**
- [x] Cold start version (<15ms target)
- [x] Cold start help (<15ms target)
- [x] Event processing (<50ms target)
- [x] Timing in response verification
- [x] Throughput with 20 rules

**Implementation Notes:**
- Debug builds use 10x threshold multiplier
- Tests pass in both debug and release modes

### Task 5.2: Add Memory Usage Tests
**Status:** [ ] Pending  
**Complexity:** High  
**Spec:** PQ-003  
**Files:** `cch_cli/tests/pq_memory.rs` (new)

**Acceptance Criteria:**
- [ ] Baseline memory measurement
- [ ] Memory under load
- [ ] No memory leaks (valgrind/heaptrack)
- [ ] <10MB RSS target

### Task 5.3: Add Sustained Load Tests
**Status:** [ ] Pending  
**Complexity:** High  
**Spec:** PQ-004, PQ-005  
**Files:** `cch_cli/tests/pq_stress.rs` (new)

**Acceptance Criteria:**
- [ ] 100+ events/second sustained
- [ ] 1-hour stress test
- [ ] No degradation over time
- [ ] Log rotation handling

### Task 5.4: Add Cross-Platform PQ Tests
**Status:** [ ] Pending  
**Complexity:** Medium  
**Spec:** PQ-006  
**Files:** CI workflow configuration

**Acceptance Criteria:**
- [ ] PQ runs on all 4 platforms
- [ ] Performance within 20% variance
- [ ] Platform comparison report

---

## Phase 6: CI/CD Integration

### Task 6.1: Create GitHub Actions IQ Workflow
**Status:** [ ] Pending  
**Complexity:** High  
**Spec:** IQ-010  
**Files:** `.github/workflows/iq-validation.yml` (new)

**Acceptance Criteria:**
- [ ] Runs on macOS ARM64 (macos-14)
- [ ] Runs on macOS Intel (macos-13)
- [ ] Runs on Linux (ubuntu-latest)
- [ ] Runs on Windows (windows-latest)
- [ ] Uploads evidence artifacts

### Task 6.2: Create GitHub Actions OQ Workflow
**Status:** [ ] Pending  
**Complexity:** Medium  
**Files:** `.github/workflows/oq-validation.yml` (new)

**Acceptance Criteria:**
- [ ] Runs OQ Rust tests
- [ ] Runs integration tests (if Claude available)
- [ ] Generates OQ evidence
- [ ] Blocks PR on failure

### Task 6.3: Create GitHub Actions PQ Workflow
**Status:** [ ] Pending  
**Complexity:** Medium  
**Files:** `.github/workflows/pq-validation.yml` (new)

**Acceptance Criteria:**
- [ ] Runs PQ benchmarks on release build
- [ ] Captures timing metrics
- [ ] Generates PQ evidence
- [ ] Warns on performance regression

### Task 6.4: Create Combined Validation Workflow
**Status:** [ ] Pending  
**Complexity:** Medium  
**Files:** `.github/workflows/validation.yml` (new)

**Acceptance Criteria:**
- [ ] Orchestrates IQ → OQ → PQ sequence
- [ ] Generates combined evidence
- [ ] Required for releases

---

## Phase 7: Evidence Collection & Reporting

### Task 7.1: Create Evidence Directory Structure
**Status:** [x] Complete  
**Complexity:** Low  
**Spec:** Evidence Collection  
**Files:**
- `docs/validation/README.md`
- `docs/validation/iq/.gitkeep`
- `docs/validation/oq/.gitkeep`
- `docs/validation/pq/.gitkeep`
- `docs/validation/sign-off/TEMPLATE-validation-report.md`

### Task 7.2: Create Evidence Collection Scripts
**Status:** [ ] Pending  
**Complexity:** Medium  
**Files:**
- `scripts/collect-iq-evidence.sh` (new)
- `scripts/collect-oq-evidence.sh` (new)
- `scripts/collect-pq-evidence.sh` (new)

**Acceptance Criteria:**
- [ ] Captures test output
- [ ] Records timestamps
- [ ] Generates markdown reports
- [ ] Creates JSON artifacts

### Task 7.3: Create Validation Report Generator
**Status:** [ ] Pending  
**Complexity:** Medium  
**Files:** `scripts/generate-validation-report.sh` (new)

**Acceptance Criteria:**
- [ ] Combines IQ/OQ/PQ evidence
- [ ] Generates summary dashboard
- [ ] Includes sign-off section
- [ ] Links to detailed evidence

### Task 7.4: Update Taskfile with Validation Tasks
**Status:** [ ] Pending  
**Complexity:** Low  
**Files:** `Taskfile.yml`

**Tasks to Add:**
- [ ] `iq-test` - Run IQ tests
- [ ] `oq-test` - Run OQ tests
- [ ] `pq-test` - Run PQ tests
- [ ] `validation-all` - Run full suite
- [ ] `validation-report` - Generate report

---

## Phase 8: Gap Resolution

### Task 8.1: Add Strict Assertion Mode
**Status:** [x] Complete  
**Complexity:** Medium  
**Gap:** GAP-001 (Soft assertions) - RESOLVED  
**Files:** `test/integration/lib/test-helpers.sh`, `test/integration/run-all.sh`, `Taskfile.yml`
**PR:** #64 (feature/strict-assertion-mode)

**Acceptance Criteria:**
- [x] STRICT_MODE environment variable
- [x] Fail test on first assertion failure
- [x] Clear distinction between soft and hard failures
- [x] `--strict` command-line flag
- [x] `task integration-test-strict` Taskfile task

### Task 8.2: Add Claude CLI Timeout
**Status:** [ ] Pending  
**Complexity:** Low  
**Gap:** GAP-003 (No timeout)  
**Files:** `test/integration/lib/test-helpers.sh`

**Acceptance Criteria:**
- [ ] 60-second default timeout
- [ ] Configurable via environment
- [ ] Clear timeout error message

### Task 8.3: Update Release Workflow
**Status:** [x] Complete  
**Complexity:** Medium  
**Files:**
- `.opencode/skill/release-cch/scripts/preflight-check.sh`
- `.opencode/skill/release-cch/SKILL.md`

**Implementation Notes:**
- Added integration test requirement (Check 5b)
- Updated PR template with integration test section

---

## Phase 9: Documentation

### Task 9.1: Update Constitution with IQ/OQ/PQ
**Status:** [x] Complete  
**Complexity:** Medium  
**Files:** `.speckit/constitution.md`

**Sections Added:**
- Validation Framework (IQ/OQ/PQ)
- Evidence collection standards
- Sign-off requirements

### Task 9.2: Create Validation Framework README
**Status:** [x] Complete  
**Complexity:** Low  
**Files:** `docs/validation/README.md`

### Task 9.3: Create Sign-Off Template
**Status:** [x] Complete  
**Complexity:** Low  
**Files:** `docs/validation/sign-off/TEMPLATE-validation-report.md`

### Task 9.4: Update Integration Testing Specification
**Status:** [x] Complete  
**Complexity:** Medium  
**Files:** `.speckit/features/integration-testing/specify.md`

**Updates:**
- Added IQ requirements (IQ-001 through IQ-010)
- Added OQ requirements (OQ-001 through OQ-011)
- Added PQ requirements (PQ-001 through PQ-008)
- Added evidence collection section

---

## Summary

| Phase | Total Tasks | Completed | Pending |
|-------|-------------|-----------|---------|
| 1. Framework Setup | 3 | 3 | 0 |
| 2. Integration Test Cases | 4 | 4 | 0 |
| 3. Installation Qualification (IQ) | 4 | 2 | 2 |
| 4. Operational Qualification (OQ) | 6 | 5 | 1 |
| 5. Performance Qualification (PQ) | 4 | 1 | 3 |
| 6. CI/CD Integration | 4 | 0 | 4 |
| 7. Evidence Collection | 4 | 1 | 3 |
| 8. Gap Resolution | 3 | 1 | 2 |
| 9. Documentation | 4 | 4 | 0 |
| **TOTAL** | **36** | **21** | **15** |

**Overall Progress:** 58% Complete (21/36 tasks)

---

## Priority Order for Remaining Tasks

### High Priority (Block Release)
1. Task 6.1: GitHub Actions IQ Workflow
2. Task 6.4: Combined Validation Workflow
3. Task 8.1: Strict Assertion Mode
4. Task 8.2: Claude CLI Timeout

### Medium Priority (Enhance Quality)
5. Task 3.3: Platform-Specific IQ Tests
6. Task 4.6: OQ Multi-Condition Tests
7. Task 5.2: Memory Usage Tests
8. Task 7.2: Evidence Collection Scripts

### Lower Priority (Future Enhancement)
9. Task 5.3: Sustained Load Tests
10. Task 5.4: Cross-Platform PQ Tests
11. Task 7.3: Validation Report Generator

---

## Dependencies

```
Task 6.1 (IQ Workflow)
    └── Task 3.4 (IQ Evidence Script)
    
Task 6.4 (Combined Workflow)
    ├── Task 6.1 (IQ Workflow)
    ├── Task 6.2 (OQ Workflow)
    └── Task 6.3 (PQ Workflow)
    
Task 7.3 (Report Generator)
    └── Task 7.2 (Evidence Scripts)
```
