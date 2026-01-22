# OpenCode Integration Test Suite

**Purpose:** Verify OpenCode hooks and logging work correctly in architect→code agent workflow
**Test Coverage:** 4 scenarios covering basic logging, decision logging, research orchestration, and full workflow
**Estimated Time:** 10-15 minutes

---

## Test Environment

These tests create temporary workspaces and verify:
- TypeScript plugin logs automatically
- Bash wrapper scripts log correctly
- Manual decision logging works
- Session management functions properly
- Cross-workspace access works
- Log format matches Claude Code format

---

## Prerequisites

- **OpenCode installed:** `opencode --version` should work
- **Architect-agent skill:** Available at `~/.claude/skills/architect-agent`
- **Bash:** Tests use bash scripts
- **jq (optional):** For JSON validation

---

## Quick Start

```bash
# Run all tests
cd ~/.claude/skills/architect-agent/docs/testing/opencode-integration-test
./run-all-tests.sh

# Or run individual tests
./test-scenarios/01-basic-logging.sh
./test-scenarios/02-decision-logging.sh
./test-scenarios/03-get-unstuck.sh
./test-scenarios/04-full-workflow.sh
```

---

## Test Scenarios

### 01: Basic Logging Test

**File:** `test-scenarios/01-basic-logging.sh`

**Tests:**
- Log session can be started
- Active log file is created and tracked
- Session can be completed
- Log file has correct format

**Expected Duration:** 30 seconds

**Success Criteria:**
- ✅ Log file created in `debugging/logs/`
- ✅ `debugging/current_log_file.txt` points to active log
- ✅ Log contains session header
- ✅ Session completes and clears active log

---

### 02: Decision Logging Test

**File:** `test-scenarios/02-decision-logging.sh`

**Tests:**
- `log-decision.sh` script works
- All decision types log correctly (decision, rationale, investigation, verification, deviation, milestone)
- Timestamps are in correct format `[HH:MM:SS]`
- Emoji markers present

**Expected Duration:** 1 minute

**Success Criteria:**
- ✅ All 6 decision types logged
- ✅ Timestamps match `[HH:MM:SS]` format
- ✅ Emoji markers present: 🎯 💭 🔍 ✓ ⚠️ 🏁
- ✅ Log entries readable and well-formatted

---

### 03: Get Unstuck Test

**File:** `test-scenarios/03-get-unstuck.sh`

**Tests:**
- `get-unstuck.sh` script works
- Research orchestration functions
- Logs decisions and findings

**Expected Duration:** 2 minutes (actual research time varies)

**Success Criteria:**
- ✅ Script executes without errors
- ✅ Research plan created
- ✅ Findings logged with `log-decision.sh`
- ✅ Multiple research channels attempted

**Note:** This test may require network access for actual research. Can be run in mock mode.

---

### 04: Full Workflow Test

**File:** `test-scenarios/04-full-workflow.sh`

**Tests:**
- Complete architect→code agent workflow
- Session start → decision logging → automated command logging → session complete
- Both plugin and wrapper approaches
- Cross-workspace access

**Expected Duration:** 3 minutes

**Success Criteria:**
- ✅ Full session lifecycle works
- ✅ Manual decisions logged
- ✅ Automated commands logged (plugin or wrapper)
- ✅ Log format matches expected output
- ✅ Architect can read code agent logs

---

## Test Workspace Structure

Tests create temporary workspaces:

```
/tmp/opencode-test-XXXXXX/
├── architect-workspace/
│   ├── instructions/
│   │   └── test-instruction.md
│   └── human/
│       └── test-summary.md
└── code-agent-workspace/
    ├── .opencode/
    │   ├── opencode.json
    │   └── plugins/
    │       └── logger/
    │           └── index.ts
    ├── debugging/
    │   ├── current_log_file.txt
    │   ├── logs/
    │   ├── scripts/
    │   │   ├── log-start.sh
    │   │   ├── log-complete.sh
    │   │   ├── log-decision.sh
    │   │   └── get-unstuck.sh
    │   └── wrapper-scripts/
    │       ├── run-with-logging.sh
    │       ├── log-tool-call.sh
    │       └── log-tool-result.sh
    └── test-files/
        └── sample.txt
```

---

## Running Tests

### Run All Tests

```bash
./run-all-tests.sh
```

**Output:**
```
🧪 OpenCode Integration Test Suite
=====================================

Test 01: Basic Logging................ ✅ PASS
Test 02: Decision Logging............. ✅ PASS
Test 03: Get Unstuck.................. ✅ PASS
Test 04: Full Workflow................ ✅ PASS

=====================================
Results: 4/4 tests passed
Duration: 6m 32s
```

### Run Individual Test

```bash
./test-scenarios/01-basic-logging.sh
```

**Output:**
```
🧪 Test 01: Basic Logging
========================

Setting up test workspace...
Testing log session start...
Testing log session complete...
Verifying log format...

✅ All checks passed
```

### Run with Verbose Output

```bash
VERBOSE=1 ./test-scenarios/01-basic-logging.sh
```

Shows detailed output of each step.

---

## Verifying Results

### Manual Verification

After running tests, check:

```bash
# List generated log files
ls -lt /tmp/opencode-test-*/code-agent-workspace/debugging/logs/

# View a log file
cat /tmp/opencode-test-*/code-agent-workspace/debugging/logs/log-*.md

# Compare to expected output
diff <expected-output-file> <actual-log-file>
```

### Expected Log Format

See `expected-outputs/` directory for reference log files:

```
expected-outputs/
├── basic-log-sample.md
├── decision-log-sample.md
├── get-unstuck-log-sample.md
└── full-workflow-log.md
```

---

## Troubleshooting

### Test Fails: "OpenCode not found"

**Cause:** OpenCode not installed or not in PATH

**Fix:**
```bash
# Verify OpenCode installation
which opencode

# Install OpenCode if needed
# (follow OpenCode installation instructions)
```

### Test Fails: "Permission denied"

**Cause:** Test scripts not executable

**Fix:**
```bash
chmod +x test-scenarios/*.sh
chmod +x run-all-tests.sh
```

### Test Fails: "Plugin not working"

**Cause:** OpenCode version doesn't support plugins

**Fix:**
- Update OpenCode to latest version
- Or: Tests will fallback to wrapper scripts automatically

### Test Fails: "Log format mismatch"

**Cause:** Timestamps or format differ from expected

**Fix:**
- Check if timezone affects timestamp format
- Verify log-decision.sh copied correctly
- Compare actual vs expected manually

---

## Cleanup

Tests create temporary workspaces in `/tmp/`. To clean up:

```bash
# Remove all test workspaces
rm -rf /tmp/opencode-test-*

# Or let OS clean up /tmp/ automatically (recommended)
```

---

## CI/CD Integration

To run tests in CI/CD pipeline:

```bash
#!/bin/bash
# .github/workflows/test-opencode.yml

- name: Run OpenCode Integration Tests
  run: |
    cd docs/testing/opencode-integration-test
    ./run-all-tests.sh
    exit $?
```

---

## Test Development

### Adding New Tests

1. Create test script: `test-scenarios/05-my-test.sh`
2. Follow template from existing tests
3. Add to `run-all-tests.sh`
4. Create expected output: `expected-outputs/my-test-sample.md`
5. Document in this README

### Test Script Template

```bash
#!/bin/bash
# Test NN: Description

set -euo pipefail

source ../test-lib.sh  # Common test functions

# Setup
setup_test_workspace

# Test steps
log "Testing feature X..."
# ... test code ...

# Verify
verify_log_format
verify_feature_specific_thing

# Cleanup
cleanup_test_workspace

# Report
report_success
```

---

## Related Documentation

- [OpenCode Logging Protocol](../../../references/opencode_logging_protocol.md)
- [OpenCode Setup Guide](../../../references/opencode_setup_guide.md)
- [Test Results](./TEST_RESULTS.md) - Latest test run results

---

**Version:** 1.0
**Last Updated:** 2025-01-17
