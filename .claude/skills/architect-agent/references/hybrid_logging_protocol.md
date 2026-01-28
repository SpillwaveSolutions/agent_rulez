# Hybrid Logging Protocol

**Version:** 2.0
**Last Updated:** 2025-01-15
**Replaces:** `logging_protocol.md` (v1.0)
**Purpose:** Reduce token consumption while maintaining comprehensive audit trails through automated hooks and lightweight manual logging

---

## Overview

The Hybrid Logging Protocol combines **automated hook-based logging** for mechanical operations with **lightweight manual logging** for high-value contextual information (decisions, rationale, investigations).

### Key Benefits

- **60-70% Token Reduction** - Hooks capture commands automatically (zero tokens)
- **Maintains Audit Trail** - All operations logged for grading/debugging
- **Preserves Context** - Critical decision-making rationale still captured
- **Faster Execution** - No permission prompts, no manual `echo` + `tee` commands
- **Slash Commands Remain** - `/log-start`, `/log-checkpoint`, `/log-complete` for session management

---

## Architecture

### Two-Layer Logging System

**Layer 1: Automated (Hooks)**
- All Bash commands
- All file operations (Read, Write, Edit)
- Tool execution timestamps
- Command outputs and exit codes
- **Token cost: 0**

**Layer 2: Manual (Lightweight Scripts)**
- Decisions and rationale
- Error investigations
- Verification steps
- Plan deviations
- Milestones
- **Token cost: ~10-15 tokens per entry**

---

## Setup Requirements

### ⚠️ CRITICAL: settings.json NOT hooks.json

**Claude Code ONLY reads hooks from `.claude/settings.json`**

❌ **WRONG:** `.claude/hooks.json` - Claude Code does NOT read this file
✅ **CORRECT:** `.claude/settings.json` - Claude Code ONLY reads hooks from here

This is THE critical requirement for hooks to work. See [hook_configuration_critical.md](./hook_configuration_critical.md) for complete details.

---

### 1. Hook Configuration

**File: `.claude/settings.json`** (NOT hooks.json!)
```json
{
  "hooks": {
    "user-prompt-submit-hook": {
      "description": "Auto-log all tool executions to active log file",
      "command": "bash -c '[ -f debugging/current_log_file.txt ] && LOG_FILE=$(cat debugging/current_log_file.txt) && [ -n \"$LOG_FILE\" ] && echo \"[$(date +%H:%M:%S)] User message received\" >> \"$LOG_FILE\" || true'"
    },
    "tool-call-hook": {
      "description": "Log tool calls before execution",
      "command": "bash -c '[ -f debugging/current_log_file.txt ] && LOG_FILE=$(cat debugging/current_log_file.txt) && [ -n \"$LOG_FILE\" ] && echo -e \"\\n---\\n[$(date +%H:%M:%S)] TOOL: $TOOL_NAME\\nPARAMS: $TOOL_PARAMS\" >> \"$LOG_FILE\" || true'"
    },
    "tool-result-hook": {
      "description": "Log tool results after execution",
      "command": "bash -c '[ -f debugging/current_log_file.txt ] && LOG_FILE=$(cat debugging/current_log_file.txt) && [ -n \"$LOG_FILE\" ] && echo -e \"[$(date +%H:%M:%S)] RESULT: $([ \"$TOOL_EXIT_CODE\" = \"0\" ] && echo \"✅ Success\" || echo \"❌ Failed (exit $TOOL_EXIT_CODE)\")\\nOUTPUT:\\n$TOOL_OUTPUT\\n---\" >> \"$LOG_FILE\" || true'"
    }
  }
}
```

### 2. Decision Logging Script

Copy script to code agent workspace:

```bash
cp ~/.claude/skills/architect-agent/templates/debugging/scripts/log-decision.sh \
   ~/clients/project/src/project-name/debugging/scripts/log-decision.sh

chmod +x ~/clients/project/src/project-name/debugging/scripts/log-decision.sh
```

### 3. Permissions

Add to code agent `.claude/settings.local.json`:

```json
{
  "permissions": {
    "allow": [
      "Bash(./debugging/scripts/log-decision.sh:*)",
      "Bash(debugging/scripts/log-decision.sh:*)"
    ]
  }
}
```

---

## Usage

### Session Management (Slash Commands)

**Start New Log File** (per instruction)
```bash
/log-start
```

Creates: `debugging/logs/log-YYYY_MM_DD-HH_MM-DESCRIPTION.md`

**Checkpoint** (manual milestones, reduced frequency)
```bash
/log-checkpoint
```

Use for major milestones only (not every 15 minutes like v1.0)

**Complete Session**
```bash
/log-complete
```

Finalizes log with summary

### Automated Logging (Hooks)

**Hooks automatically capture:**

Every time you run a command like:
```bash
task test
```

Hook logs:
```
---
[14:23:45] TOOL: Bash
PARAMS: command="task test"
[14:23:47] RESULT: ✅ Success
OUTPUT:
test_database.py::test_connection PASSED
test_api.py::test_endpoints PASSED
47 passed in 2.34s
---
```

**You don't need to manually log this anymore!** No `echo` + `tee` required.

### Manual Decision Logging

**When to use `log-decision.sh`:**

Use for high-value contextual information that hooks cannot capture:

#### 1. Decisions
```bash
./debugging/scripts/log-decision.sh decision \
  "Using async approach instead of sync because API calls will be concurrent"
```

#### 2. Rationale
```bash
./debugging/scripts/log-decision.sh rationale \
  "Chose PostgreSQL over SQLite for production - need ACID compliance and concurrent writes"
```

#### 3. Investigations
```bash
./debugging/scripts/log-decision.sh investigation \
  "Error traced to missing environment variable. Root cause: .env file not loaded in test environment"
```

#### 4. Verifications
```bash
./debugging/scripts/log-decision.sh verification \
  "Confirmed all migrations applied by checking database schema version table"
```

#### 5. Deviations
```bash
./debugging/scripts/log-decision.sh deviation \
  "Instruction said use JWT, but OAuth2 already configured. Keeping OAuth2 to avoid breaking existing auth"
```

#### 6. Milestones
```bash
./debugging/scripts/log-decision.sh milestone \
  "All unit tests passing (47/47). Coverage at 94%. Ready for integration tests"
```

---

## Golden Rule (Updated)

**v1.0 Rule:** Log → Act → Log Result
**v2.0 Rule:** Hooks Log → You Explain Why → Hooks Log Result

### Workflow Comparison

**OLD (v1.0) - Manual Everything:**
```bash
echo "[14:23:45] Running unit tests" | tee -a "$LOG_FILE"
echo "Decision: Testing database layer first" | tee -a "$LOG_FILE"
task test 2>&1 | tee -a "$LOG_FILE"
echo "[14:23:47] Result: All tests passed" | tee -a "$LOG_FILE"
```
**Token cost:** ~100 tokens

**NEW (v2.0) - Hybrid:**
```bash
# Hook automatically logs this:
# [14:23:45] TOOL: Bash, PARAMS: task test

# You only log the "why":
./debugging/scripts/log-decision.sh decision \
  "Testing database layer first - fastest feedback loop"

# Run command (hook captures output automatically)
task test

# Hook automatically logs:
# [14:23:47] RESULT: ✅ Success, OUTPUT: [test results]
```
**Token cost:** ~15 tokens

**Savings:** 85 tokens per command cycle (85% reduction)

---

## What to Log Manually

### ALWAYS Log (High Value)

✅ **Decisions** - Why you chose approach A over B
✅ **Rationale** - Reasoning behind technical choices
✅ **Investigations** - Root cause analysis of errors
✅ **Verifications** - How you confirmed something worked
✅ **Deviations** - When you diverge from instructions (with justification)
✅ **Milestones** - Major progress markers (tests passing, coverage met, etc.)

### NEVER Log Manually (Hooks Handle It)

❌ Command execution timestamps
❌ Command outputs
❌ Exit codes
❌ File paths being read/written
❌ "Running X command" announcements

---

## Log Entry Format

### Automated (Hooks)

```markdown
---
[HH:MM:SS] TOOL: Bash
PARAMS: command="task test"
[HH:MM:SS] RESULT: ✅ Success
OUTPUT:
[command output here]
---

---
[HH:MM:SS] TOOL: Edit
PARAMS: file_path="src/database.py", old_string="...", new_string="..."
[HH:MM:SS] RESULT: ✅ Success
OUTPUT:
File edited successfully
---
```

### Manual (Decision Logging)

```markdown
[HH:MM:SS] 🎯 DECISION: Using async approach for API calls

[HH:MM:SS] 💭 RATIONALE: API calls will be concurrent, async reduces latency by 60%

[HH:MM:SS] 🔍 INVESTIGATION: Error traced to missing DB_HOST env var

[HH:MM:SS] ✓ VERIFICATION: Confirmed migrations applied by checking schema_version table

[HH:MM:SS] ⚠️  DEVIATION: Keeping OAuth2 instead of JWT - OAuth2 already configured and tested

[HH:MM:SS] 🏁 MILESTONE: All 47 unit tests passing, coverage 94%, ready for integration tests
---
```

---

## Session Checkpoints

### Start Checkpoint (Mandatory)

Slash command `/log-start` creates this automatically:

```markdown
[HH:MM:SS] Session Started
**Goal:** Implement contact management API endpoints
**Success Criteria:**
- [ ] All CRUD endpoints functional
- [ ] Tests passing
- [ ] Coverage >90%
**Baseline:**
- Tests: 0/0 (no tests yet)
- Coverage: 0%
---
```

### Progress Checkpoints (Reduced Frequency)

**OLD:** Every 15 minutes
**NEW:** Only at major milestones (25%, 50%, 75%, 100%)

Use slash command `/log-checkpoint` or script:
```bash
./debugging/scripts/log-decision.sh milestone \
  "Database layer complete (25%). All models created, migrations working, tests passing"
```

### Final Checkpoint (Mandatory)

Slash command `/log-complete` creates this automatically:

```markdown
[HH:MM:SS] 🏁 Final Summary
**All Success Criteria Met:**
- [x] CRUD endpoints functional - Evidence: All integration tests passing
- [x] Tests passing - Evidence: 47/47 unit + 12/12 integration
- [x] Coverage >90% - Evidence: 94% coverage report
**Test Results:**
- Unit tests: 47/47 passing ✅
- Integration tests: 12/12 passing ✅
- Coverage: 94% ✅
**Files Modified:**
- src/api/endpoints.py - Added CRUD operations
- tests/test_api.py - Full test coverage
**Total Duration:** 2 hours 15 minutes
**Status:** ✅ COMPLETE
---
```

---

## Token Consumption Comparison

### Typical 30-Command Session

**OLD Protocol (v1.0):**
| Operation | Token Cost | Count | Total |
|-----------|-----------|-------|-------|
| Manual command logging | 80 | 30 | 2,400 |
| Checkpoints (every 15 min) | 50 | 8 | 400 |
| Final summary | 100 | 1 | 100 |
| **TOTAL** | | | **2,900** |

**NEW Protocol (v2.0):**
| Operation | Token Cost | Count | Total |
|-----------|-----------|-------|-------|
| Hooks (automatic) | 0 | 30 | 0 |
| Decision logging | 15 | 20 | 300 |
| Milestone checkpoints | 50 | 4 | 200 |
| Final summary | 100 | 1 | 100 |
| **TOTAL** | | | **600** |

**Savings:** 2,300 tokens (79% reduction)

---

## Migration from v1.0

### For Existing Code Agent Workspaces

1. **Install hooks:**
   ```bash
   # Hooks must be in settings.json, not hooks.json!
   # Configure hooks in .claude/settings.json
   ```

2. **Install decision script:**
   ```bash
   mkdir -p debugging/scripts
   cp ~/.claude/skills/architect-agent/templates/debugging/scripts/log-decision.sh \
      debugging/scripts/
   chmod +x debugging/scripts/log-decision.sh
   ```

3. **Update permissions:**
   Add to `.claude/settings.local.json`:
   ```json
   {
     "permissions": {
       "allow": [
         "Bash(./debugging/scripts/log-decision.sh:*)",
         "Bash(debugging/scripts/log-decision.sh:*)"
       ]
     }
   }
   ```

4. **Update AGENTS.md:**
   Remove manual `echo` + `tee` instructions, reference this protocol instead

5. **Keep slash commands:**
   `/log-start`, `/log-checkpoint`, `/log-complete` remain unchanged

---

## Grading Impact

### Updated Rubric (Logging & Traceability = 10 points)

| Score | Criteria |
|-------|----------|
| 10 | Perfect logging: Hooks capturing all commands, manual logging covers all decisions/rationale/investigations |
| 8 | Good logging: Hooks working, most key decisions logged |
| 6 | Adequate logging: Some manual entries missing, but hooks operational |
| 4 | Poor logging: Hooks not configured or many decisions not logged |
| 0-3 | No logging or batch logs at end |

**Key Changes from v1.0:**
- No longer required to manually log command execution (hooks do it)
- Still required to log decisions, rationale, investigations (manual)
- Deduction for not using hooks: -3 points
- Deduction for missing decision logs: -1 point per occurrence

**Critical Rule (unchanged):** No logs = Maximum grade of C+ (78%), even if work is perfect.

---

## Troubleshooting

### Hooks Not Logging

**Symptom:** No automated entries in log file

**Check:**
1. Hooks in settings.json: `grep "PostToolUse" .claude/settings.json`
2. No hooks.json exists: `ls .claude/hooks.json 2>&1 | grep "No such file"`
3. Log file active: `cat debugging/current_log_file.txt`
4. Hook syntax valid: `python3 -m json.tool .claude/settings.json`
5. Hook logger executable: `ls -la .claude/hook-logger.py`
6. Restart Claude Code session

### Decision Script Permission Denied

**Symptom:** "Permission denied" when running `log-decision.sh`

**Fix:**
1. Make executable: `chmod +x debugging/scripts/log-decision.sh`
2. Add to permissions (see Setup section above)
3. Reload Claude Code

### Log File Not Found

**Symptom:** "No active log session" error

**Fix:**
```bash
# Start new session
/log-start

# Verify active log
cat debugging/current_log_file.txt
```

---

## Examples

### Complete Session Example

```bash
# 1. Start session
/log-start

# 2. Hooks automatically log all commands from here

# 3. Add context for first major decision
./debugging/scripts/log-decision.sh decision \
  "Using FastAPI instead of Flask - better async support for concurrent API calls"

# 4. Run commands (hooks log automatically)
task test

# 5. Investigate error (manual context)
./debugging/scripts/log-decision.sh investigation \
  "Import error traced to missing pydantic package. Root cause: not in requirements.txt"

# 6. Fix and verify (hooks log commands, you log verification)
pip install pydantic
./debugging/scripts/log-decision.sh verification \
  "Verified pydantic installed and tests now passing"

# 7. Milestone (manual)
./debugging/scripts/log-decision.sh milestone \
  "API endpoints complete - 12/12 tests passing, ready for integration tests"

# 8. Complete session
/log-complete
```

### Log Output

```markdown
[14:23:00] Session Started
**Goal:** Implement FastAPI endpoints
**Success Criteria:**
- [ ] All endpoints functional
- [ ] Tests passing
---

[14:23:05] 🎯 DECISION: Using FastAPI instead of Flask - better async support for concurrent API calls

---
[14:23:10] TOOL: Bash
PARAMS: command="task test"
[14:23:12] RESULT: ❌ Failed (exit 1)
OUTPUT:
ImportError: No module named 'pydantic'
---

[14:23:15] 🔍 INVESTIGATION: Import error traced to missing pydantic package. Root cause: not in requirements.txt

---
[14:23:20] TOOL: Bash
PARAMS: command="pip install pydantic"
[14:23:25] RESULT: ✅ Success
OUTPUT:
Successfully installed pydantic-2.5.0
---

[14:23:30] ✓ VERIFICATION: Verified pydantic installed and tests now passing

---
[14:23:35] TOOL: Bash
PARAMS: command="task test"
[14:23:40] RESULT: ✅ Success
OUTPUT:
test_endpoints.py::test_create PASSED
test_endpoints.py::test_read PASSED
12 passed in 1.23s
---

[14:23:45] 🏁 MILESTONE: API endpoints complete - 12/12 tests passing, ready for integration tests
---

[14:30:00] 🏁 Final Summary
**All Success Criteria Met:**
- [x] All endpoints functional
- [x] Tests passing
**Status:** ✅ COMPLETE
---
```

**Token Usage:**
- Hooks (automatic): 0 tokens
- 4 manual entries × 15 tokens = 60 tokens
- Total: ~60 tokens (vs ~500 tokens in v1.0)

---

## References

- [Logging Protocol v1.0 (Deprecated)](./logging_protocol.md)
- [Permissions Setup Protocol](./permissions_setup_protocol.md)
- [Grading Workflow](./instruction_grading_workflow.md)
- [Claude Code Hooks Documentation](https://code.claude.com/docs/en/configuration/hooks)

---

**Version History:**
- **v2.0 (2025-01-15):** Introduced hybrid approach with hooks + lightweight manual logging
- **v1.0 (2025-11-13):** Original fully-manual protocol with `echo` + `tee`
