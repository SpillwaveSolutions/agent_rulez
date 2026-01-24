# CRD: CCH Integration Testing Framework

**Document Type:** Change Request Document (CRD)  
**Status:** Implemented  
**Created:** 2025-01-23  
**Author:** Claude Code Assistant  

---

## 1. Executive Summary

Create an end-to-end integration test suite that validates CCH (Claude Context Hooks) works correctly when invoked by the real Claude CLI. Tests will verify hook triggering, log creation, context injection, and blocking behavior.

---

## 2. Problem Statement

Currently, CCH has unit tests that validate individual components in isolation, but there are no integration tests that verify:

1. CCH hooks fire correctly when Claude CLI invokes tools
2. Log entries are created with proper structure and timing
3. Context injection actually reaches Claude's context
4. Blocking rules prevent dangerous operations end-to-end

---

## 3. Proposed Solution

### 3.1 Architecture

```
test/integration/
├── README.md                    # Documentation
├── run-all.sh                   # Master test runner
├── lib/
│   └── test-helpers.sh          # Shared bash functions
├── use-cases/
│   ├── 01-block-force-push/     # Test blocking dangerous git ops
│   ├── 02-context-injection/    # Test context file injection
│   ├── 03-session-logging/      # Test audit log creation
│   └── 04-permission-explanations/  # Test permission context
└── results/                     # Test outputs (gitignored)
```

### 3.2 Test Framework

- **Language:** Bash scripts (simple, portable, easy to debug)
- **Runner:** `run-all.sh` orchestrates all tests
- **Assertions:** Custom assertion functions in `test-helpers.sh`
- **Invocation:** Real Claude CLI via `claude -p` with `--allowedTools`

### 3.3 Test Cases

| ID | Name | Purpose |
|----|------|---------|
| 01 | block-force-push | Verify CCH blocks `git push --force` |
| 02 | context-injection | Verify CCH injects context for `.cdk.ts` files |
| 03 | session-logging | Verify CCH creates JSON Lines audit logs |
| 04 | permission-explanations | Verify CCH provides context on permission requests |

### 3.4 Test Flow (per test case)

1. **Setup:** Create temp workspace, copy use-case files
2. **Install:** Run `cch install --project` in workspace
3. **Execute:** Invoke `claude -p "<prompt>" --allowedTools <tools>`
4. **Verify:** Check CCH logs for expected behavior
5. **Cleanup:** Remove temp workspace

### 3.5 Taskfile Integration

Add tasks to `Taskfile.yml`:

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

## 4. Technical Details

### 4.1 Prerequisites

- Claude CLI installed and in PATH
- CCH binary built (auto-built by test runner)
- Bash shell

### 4.2 Key Helper Functions

| Function | Purpose |
|----------|---------|
| `start_test "name"` | Begin test, initialize counters |
| `setup_workspace "/path"` | Create temp dir, copy files |
| `install_cch [workspace]` | Install CCH in workspace |
| `run_claude <ws> <prompt> [tools] [turns]` | Invoke Claude CLI |
| `assert_log_contains "pattern" "msg"` | Assert log content |
| `cleanup_workspace` | Remove temp workspace |
| `end_test` | Report pass/fail |

### 4.3 Log Verification

Tests verify CCH logs at `~/.claude/logs/cch.log`:

- **Format:** JSON Lines (one JSON object per line)
- **Required fields:** `timestamp`, `event_type`, `session_id`
- **Timing fields:** `processing_ms`, `rules_evaluated`

### 4.4 Error Handling

- Tests use `set -euo pipefail` for strict error handling
- Timeouts prevent hanging on permission prompts
- Soft assertions allow tests to continue after failures

---

## 5. User Decisions Captured

From the planning conversation:

| Question | Decision |
|----------|----------|
| Test runner framework | Bash scripts (based on existing patterns) |
| Use cases to cover | All: block, inject, logging, permissions |
| Claude invocation method | Real Claude CLI (`claude -p`) |
| Fallback when Claude unavailable | Fail the test suite |

---

## 6. Files Created

| File | Purpose |
|------|---------|
| `test/integration/README.md` | Test suite documentation |
| `test/integration/run-all.sh` | Master test runner |
| `test/integration/lib/test-helpers.sh` | Shared functions |
| `test/integration/use-cases/01-block-force-push/` | Block test |
| `test/integration/use-cases/02-context-injection/` | Injection test |
| `test/integration/use-cases/03-session-logging/` | Logging test |
| `test/integration/use-cases/04-permission-explanations/` | Permission test |
| `test/integration/results/.gitkeep` | Results dir placeholder |
| `Taskfile.yml` | Task runner configuration |

---

## 7. AGENTS.md Updates

Added `using-claude-code-cli` skill to available skills:

```xml
<skill>
<name>using-claude-code-cli</name>
<description>Invoke Claude Code CLI from Python orchestrators and shell scripts...</description>
<location>.opencode/skill/using-claude-code-cli/SKILL.md</location>
</skill>
```

---

## 8. Usage

```bash
# Run all integration tests
task integration-test
# or
./test/integration/run-all.sh

# Run specific test
./test/integration/use-cases/01-block-force-push/test.sh

# Run with debug output
DEBUG=1 ./test/integration/run-all.sh
```

---

## 9. Success Criteria

- [ ] All 4 test cases pass when Claude CLI is available
- [ ] Tests fail gracefully with clear message when Claude CLI missing
- [ ] Test results saved as JSON in `results/` directory
- [ ] Taskfile tasks work correctly (`task itest`)

---

## 10. Future Enhancements

1. Add more test cases for edge cases
2. Add CI/CD GitHub Actions workflow
3. Add performance benchmarking tests
4. Add parallel test execution option
