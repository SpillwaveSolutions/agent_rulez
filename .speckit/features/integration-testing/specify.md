# Integration Testing Framework Specification

**Feature ID:** integration-testing  
**Status:** Implemented - Review Required  
**Created:** 2025-01-23  
**Source:** [CRD-integration-testing.md](../../../docs/prds/change_requests/CRD-integration-testing.md)  
**Review Date:** 2025-01-23  
**Reviewer:** Claude (SDD Integration)

---

## 1. Overview

Create an end-to-end integration test suite that validates CCH (Claude Context Hooks) works correctly when invoked by the real Claude CLI. Tests verify hook triggering, log creation, context injection, and blocking behavior.

---

## 2. Problem Statement

CCH has unit tests that validate individual components in isolation, but there are no integration tests that verify:

1. CCH hooks fire correctly when Claude CLI invokes tools
2. Log entries are created with proper structure and timing
3. Context injection actually reaches Claude's context
4. Blocking rules prevent dangerous operations end-to-end

---

## 3. Requirements

### 3.1 Functional Requirements

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

### 3.2 Test Cases

| ID | Test Case | Purpose |
|----|-----------|---------|
| TC-001 | block-force-push | Verify CCH blocks `git push --force` operations |
| TC-002 | context-injection | Verify CCH injects context for `.cdk.ts` files |
| TC-003 | session-logging | Verify CCH creates JSON Lines audit logs |
| TC-004 | permission-explanations | Verify CCH provides context on permission requests |

### 3.3 Helper Functions

| Function | Purpose |
|----------|---------|
| `start_test "name"` | Begin test, initialize counters |
| `setup_workspace "/path"` | Create temp directory, copy files |
| `install_cch [workspace]` | Install CCH in workspace |
| `run_claude <ws> <prompt> [tools] [turns]` | Invoke Claude CLI |
| `assert_log_contains "pattern" "msg"` | Assert log content matches |
| `cleanup_workspace` | Remove temp workspace |
| `end_test` | Report pass/fail status |

---

## 4. User Stories

### US-001: Developer Verifies Hook Behavior
**As a** CCH developer  
**I want to** run integration tests that validate hook firing  
**So that** I can ensure CCH works correctly with the real Claude CLI

**Acceptance Criteria:**
- Tests can be run with a single command (`task integration-test` or `./test/integration/run-all.sh`)
- Tests use real Claude CLI invocations
- Clear pass/fail reporting for each test case

### US-002: Developer Tests Blocking Rules
**As a** CCH developer  
**I want to** verify that blocking rules actually prevent dangerous operations  
**So that** I can ensure safety mechanisms work end-to-end

**Acceptance Criteria:**
- TC-001 verifies `git push --force` is blocked
- CCH logs show the block action and reason
- Claude receives the rejection message

### US-003: Developer Tests Context Injection
**As a** CCH developer  
**I want to** verify that context files are injected correctly  
**So that** I can ensure Claude receives the additional context

**Acceptance Criteria:**
- TC-002 verifies context injection for `.cdk.ts` files
- Injected context appears in CCH logs
- Context is sent to Claude before tool execution

### US-004: Developer Tests Audit Logging
**As a** CCH developer  
**I want to** verify that session logging creates proper audit trails  
**So that** I can ensure compliance and debugging capabilities

**Acceptance Criteria:**
- TC-003 verifies JSON Lines format in logs
- Required fields present: `timestamp`, `event_type`, `session_id`
- Timing fields present: `processing_ms`, `rules_evaluated`

---

## 5. Success Criteria

| ID | Criterion | Metric |
|----|-----------|--------|
| SC-001 | All 4 test cases pass when Claude CLI available | 100% pass rate |
| SC-002 | Clear error message when Claude CLI missing | Error includes setup instructions |
| SC-003 | Test results saved to `results/` directory | JSON format with pass/fail status |
| SC-004 | Taskfile integration works correctly | `task itest` runs all tests |
| SC-005 | Tests complete in reasonable time | < 5 minutes for full suite |
| SC-006 | No hanging on permission prompts | Tests use timeouts |

---

## 6. Edge Cases & Error Handling

| Edge Case | Expected Behavior |
|-----------|-------------------|
| Claude CLI not installed | Test suite fails with clear error message |
| CCH binary not built | Test runner auto-builds via dependencies |
| Permission prompt appears | Tests timeout after configured duration |
| Log directory doesn't exist | Tests create directory as needed |
| Previous test artifacts exist | Tests clean up before and after |

---

## 7. File Structure

```
test/integration/
├── README.md                    # Documentation
├── run-all.sh                   # Master test runner
├── lib/
│   └── test-helpers.sh          # Shared bash functions
├── use-cases/
│   ├── 01-block-force-push/     # Test blocking dangerous git ops
│   │   ├── README.md
│   │   ├── test.sh
│   │   └── .claude/hooks.yaml
│   ├── 02-context-injection/    # Test context file injection
│   │   ├── README.md
│   │   ├── test.sh
│   │   ├── sample.cdk.ts
│   │   └── .claude/
│   │       ├── hooks.yaml
│   │       └── context/cdk-best-practices.md
│   ├── 03-session-logging/      # Test audit log creation
│   │   ├── README.md
│   │   ├── test.sh
│   │   └── .claude/hooks.yaml
│   └── 04-permission-explanations/  # Test permission context
│       ├── README.md
│       ├── test.sh
│       └── .claude/
│           ├── hooks.yaml
│           └── context/
│               ├── bash-permission-context.md
│               └── write-permission-context.md
└── results/                     # Test outputs (gitignored)
    └── .gitkeep
```

---

## 8. Taskfile Integration

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
```

---

## 9. Dependencies

| Dependency | Purpose |
|------------|---------|
| Claude CLI | Tool invocation (`claude -p`) |
| CCH binary | Hook processing |
| Bash shell | Test execution |
| jq (optional) | JSON parsing in tests |

---

## 10. Notes

- Tests use `set -euo pipefail` for strict error handling
- Soft assertions allow tests to continue after failures for complete reporting
- The `using-claude-code-cli` skill was added to AGENTS.md to support this feature
