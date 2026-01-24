# Integration Testing Framework - Technical Plan

**Feature ID:** integration-testing  
**Status:** Implemented  
**Created:** 2025-01-23  
**Source:** [specify.md](./specify.md)

---

## 1. Architecture Overview

```
test/integration/
├── run-all.sh                    # Master orchestrator
├── lib/
│   └── test-helpers.sh           # Shared bash library (445 LOC)
├── use-cases/
│   ├── 01-block-force-push/      # Blocking test
│   ├── 02-context-injection/     # Context injection test
│   ├── 03-session-logging/       # Audit logging test
│   └── 04-permission-explanations/  # Permission context test
└── results/                      # JSON test outputs
```

---

## 2. Technology Stack

| Component | Technology | Rationale |
|-----------|------------|-----------|
| Test Runner | Bash | Simple, portable, no external dependencies |
| Assertions | Custom functions | Tailored to CCH log verification |
| Claude Invocation | `claude -p` CLI | Real end-to-end validation |
| Log Format | JSON Lines | One JSON object per line for easy parsing |
| Result Storage | JSON files | Machine-readable, CI/CD compatible |
| Task Runner | Taskfile | Consistent with project conventions |

---

## 3. Key Components

### 3.1 Master Test Runner (`run-all.sh`)

**Purpose:** Orchestrate all test cases and aggregate results

**Capabilities:**
- Parse command-line arguments (`--quick`, `--test`)
- Auto-discover test cases in `use-cases/` directory
- Skip slow tests in quick mode (`.slow` marker file)
- Aggregate pass/fail counts
- Exit with appropriate code for CI/CD

**Implementation:** 106 lines of Bash

### 3.2 Test Helper Library (`lib/test-helpers.sh`)

**Purpose:** Shared functions for test setup, execution, and assertions

**Function Categories:**

| Category | Functions |
|----------|-----------|
| Prerequisites | `check_prerequisites`, `build_cch` |
| Setup/Teardown | `start_test`, `setup_workspace`, `install_cch`, `cleanup_workspace` |
| Log Management | `clear_cch_logs`, `get_log_line_count`, `get_new_log_entries`, `log_contains`, `log_contains_since` |
| Claude CLI | `run_claude` (captures stdout/stderr) |
| Assertions | `assert_true`, `assert_log_contains`, `assert_log_contains_since`, `assert_claude_output_contains`, `assert_success`, `assert_file_exists` |
| Results | `end_test`, `save_result` |
| Utilities | `section`, `debug`, `wait_for` |

**Implementation:** 445 lines of Bash

### 3.3 Test Case Structure

Each test case follows a consistent structure:

```bash
#!/bin/bash
set -euo pipefail

# Source helper library
source "$SCRIPT_DIR/../../lib/test-helpers.sh"

# Lifecycle
start_test "test-name"
check_prerequisites

# Setup
WORKSPACE=$(setup_workspace "$SCRIPT_DIR")
install_cch "$WORKSPACE"

# Execute
run_claude "$WORKSPACE" "<prompt>" "<tools>" <max_turns>

# Verify
assert_log_contains "<pattern>" "<message>"

# Cleanup
cleanup_workspace
end_test
```

---

## 4. Test Cases - Technical Details

### TC-001: Block Force Push

**Hooks Configuration:**
```yaml
rules:
  - name: block-force-push
    matchers:
      tools: ["Bash"]
      command_match: "git push.*--force|git push.*-f"
    actions:
      block: true
```

**Verification Strategy:**
- Check CCH log for `block-force-push` rule match
- Verify `"Block"` outcome in log
- Handle case where Claude refuses before CCH intercepts

### TC-002: Context Injection

**Hooks Configuration:**
```yaml
rules:
  - name: cdk-context-injection
    matchers:
      tools: ["Read"]
      file_match: ".*\\.cdk\\.ts$"
    actions:
      inject_context: ".claude/context/cdk-best-practices.md"
```

**Verification Strategy:**
- Check for `cdk-context-injection` rule in logs
- Verify `injected_files` field present
- Negative test: non-CDK file should not trigger injection

### TC-003: Session Logging

**Verification Strategy:**
- Clear logs before test
- Run Claude command
- Verify log file exists
- Validate JSON Lines format (each line parseable as JSON)
- Check for required fields: `timestamp`, `event_type`, `session_id`
- Check for timing fields: `processing_ms` or `duration`

### TC-004: Permission Explanations

**Hooks Configuration:**
```yaml
rules:
  - name: explain-write-permissions
    event_types: ["PermissionRequest"]
    matchers:
      tools: ["Write"]
    actions:
      inject_context: ".claude/context/write-permission-context.md"
```

**Verification Strategy:**
- Run Claude with limited `--allowedTools` to trigger permission request
- Use timeout to prevent hanging
- Check for `PermissionRequest` event in logs
- Verify permission explanation rules matched

---

## 5. Taskfile Integration

```yaml
integration-test:
  desc: Run CCH + Claude CLI integration tests
  aliases: [itest]
  deps: [build]  # Auto-build CCH before running
  cmds:
    - ./{{.INTEGRATION_DIR}}/run-all.sh
  preconditions:
    - sh: command -v claude
      msg: "Claude CLI not found. Install it first."

integration-test-quick:
  desc: Run quick integration tests (skip slow ones)
  aliases: [itest-quick]
  deps: [build]
  cmds:
    - ./{{.INTEGRATION_DIR}}/run-all.sh --quick

integration-test-single:
  desc: Run a single integration test
  deps: [build]
  cmds:
    - ./{{.INTEGRATION_DIR}}/run-all.sh --test {{.TEST_NAME}}

test-all:
  desc: Run all tests (unit + integration)
  cmds:
    - task: test
    - task: integration-test
```

---

## 6. Error Handling Strategy

| Scenario | Handling |
|----------|----------|
| Claude CLI missing | Fail fast with clear installation instructions |
| CCH binary missing | Auto-build via `cargo build --release` |
| Permission prompt hang | Timeout after configurable duration |
| Test assertion failure | Soft failure - continue to next assertion |
| Workspace cleanup | Always runs, even on failure |
| Log directory missing | Auto-create as needed |

---

## 7. Result Artifacts

### Test Result JSON
```json
{
  "test_name": "01-block-force-push",
  "status": "PASS",
  "assertions_passed": 5,
  "assertions_failed": 0,
  "timestamp": "2025-01-23T10:30:00Z",
  "duration_seconds": 15
}
```

### Console Output
- Color-coded pass/fail indicators
- Assertion-level detail
- Summary with totals
- Exit code: 0 (all pass) or 1 (any fail)

---

## 8. Dependencies

| Dependency | Type | Required |
|------------|------|----------|
| Claude CLI | External | Yes |
| CCH Binary | Internal | Yes (auto-built) |
| Bash | System | Yes |
| Python3 | System | Optional (JSON validation) |
| jq | External | Optional (log inspection) |

---

## 9. Performance Considerations

| Aspect | Design Decision |
|--------|-----------------|
| Test Isolation | Each test uses temp workspace, cleaned after |
| Parallel Execution | Not supported (Claude CLI limitation) |
| Quick Mode | Skip slow tests via `.slow` marker file |
| Timeout | Prevent hanging on permission prompts |
| Log Tracking | Record position before test, check only new entries |

---

## 10. Future Enhancements (Planned)

1. **Parallel Test Execution** - Use process pools when Claude supports it
2. **CI/CD GitHub Actions** - Workflow for automated testing
3. **Performance Benchmarks** - Track CCH processing times over releases
4. **Coverage Reports** - Map test cases to CCH code paths
5. **Snapshot Testing** - Compare log output against golden files
