# OpenCode Hybrid Logging Protocol

**Version:** 2.0 (OpenCode Edition)
**Last Updated:** 2025-01-17
**Based On:** Hybrid Logging Protocol v2.0 for Claude Code
**Purpose:** Reduce token consumption while maintaining comprehensive audit trails through automated plugins/wrappers and lightweight manual logging

---

## Overview

The OpenCode Hybrid Logging Protocol adapts the proven hybrid approach for OpenCode environments, combining **automated logging** (via TypeScript plugins or bash wrappers) with **lightweight manual logging** for high-value contextual information.

### Key Benefits

- **60-70% Token Reduction** - Automated logging captures commands (zero tokens)
- **Maintains Audit Trail** - All operations logged for grading/debugging
- **Preserves Context** - Critical decision-making rationale still captured
- **Faster Execution** - No permission prompts, no manual `echo` + `tee` commands
- **Dual Implementation** - Choose TypeScript plugin OR bash wrappers
- **Cross-Compatible** - Same log format as Claude Code hooks

---

## Architecture

### Two-Layer Logging System

**Layer 1: Automated (Plugin or Wrappers)**
- All bash commands
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

### Choose Your Approach

OpenCode supports **two approaches** for automated logging:

#### Option A: TypeScript Plugin (Recommended)

**Pros:**
- Native OpenCode integration
- Fastest performance (<1ms overhead)
- Type-safe
- Most similar to Claude Code hooks

**Cons:**
- Requires OpenCode with plugin API support

#### Option B: Bash Wrapper Scripts

**Pros:**
- Works with any OpenCode version
- No TypeScript compilation required
- More transparent (readable bash code)

**Cons:**
- Requires manually wrapping commands
- Slightly higher overhead (~5-10ms)

---

## Setup: Option A - TypeScript Plugin

### 1. Copy Plugin to Code Agent Workspace

```bash
# From architect agent workspace
cp -r ~/.claude/skills/architect-agent/templates/opencode/plugins/logger \
      ~/clients/project/src/project-name/.opencode/plugins/
```

### 2. Enable Plugin

Create or edit `.opencode/opencode.json`:

```json
{
  "plugins": ["./plugins/logger"]
}
```

### 3. Verify Plugin Loaded

Start OpenCode and check for no error messages. The plugin will:
- Hook into `onUserMessage` - logs user prompts
- Hook into `onToolCalled` - logs tool calls before execution
- Hook into `onToolResult` - logs results after execution

**Plugin Implementation:** `templates/opencode/plugins/logger/index.ts`

The plugin automatically checks for an active log session via `debugging/current_log_file.txt` and logs to that file. If no active session, it fails silently (no logging, no errors).

---

## Setup: Option B - Bash Wrapper Scripts

### 1. Copy Wrapper Scripts

```bash
# From architect agent workspace
cp -r ~/.claude/skills/architect-agent/templates/opencode/wrapper-scripts \
      ~/clients/project/src/project-name/debugging/
```

### 2. Make Scripts Executable

```bash
chmod +x ~/clients/project/src/project-name/debugging/wrapper-scripts/*.sh
```

### 3. Usage Patterns

#### Pattern 1: Generic Wrapper (run-with-logging.sh)

Wrap any command:

```bash
./debugging/wrapper-scripts/run-with-logging.sh task test
```

Logs:
```markdown
---
[14:23:45] TOOL: Bash
COMMAND: task test
[14:23:47] RESULT: ✅ Success
OUTPUT:
test_database.py::test_connection PASSED
47 passed in 2.34s
---
```

#### Pattern 2: Split Pre/Post Logging

For finer control:

```bash
# Before execution
./debugging/wrapper-scripts/log-tool-call.sh "Bash" "command=task test"

# Run command
task test

# After execution
./debugging/wrapper-scripts/log-tool-result.sh $? "$(cat output.txt)"
```

---

## Common Setup Steps (Both Options)

### 1. Decision Logging Script

Copy script to code agent workspace:

```bash
cp ~/.claude/skills/architect-agent/templates/debugging/scripts/log-decision.sh \
   ~/clients/project/src/project-name/debugging/scripts/log-decision.sh

chmod +x ~/clients/project/src/project-name/debugging/scripts/log-decision.sh
```

### 2. Permissions (OpenCode Settings)

OpenCode may have different permission patterns than Claude Code. Check OpenCode documentation for pre-approving script execution.

Equivalent concept to Claude Code's `.claude/settings.local.json`:

```json
{
  "permissions": {
    "allow": [
      "debugging/scripts/log-decision.sh",
      "debugging/wrapper-scripts/*.sh"
    ]
  }
}
```

*(Note: OpenCode permission syntax may differ - consult OpenCode docs)*

---

## Usage

### Session Management

OpenCode may not have built-in slash commands like Claude Code's `/log-start`. Instead, create your log session manually:

#### Start New Log File

Create script: `debugging/scripts/log-start.sh`

```bash
#!/bin/bash
# Generate log filename with timestamp and description
DESCRIPTION=${1:-"session"}
TIMESTAMP=$(date +%Y%m%d_%H%M%S)
LOG_FILE="debugging/logs/log-${TIMESTAMP}-${DESCRIPTION}.md"

# Create log directory if needed
mkdir -p debugging/logs

# Initialize log file
cat > "$LOG_FILE" <<EOF
# Log Session: $DESCRIPTION
**Started:** $(date +%Y-%m-%d\ %H:%M:%S)

## Goal
[Your goal here]

## Success Criteria
- [ ] Criterion 1
- [ ] Criterion 2

---

EOF

# Set as active log
echo "$LOG_FILE" > debugging/current_log_file.txt

echo "Log session started: $LOG_FILE"
```

Usage:
```bash
./debugging/scripts/log-start.sh "implement-api-endpoints"
```

#### Complete Session

Create script: `debugging/scripts/log-complete.sh`

```bash
#!/bin/bash
# Finalize current log session

if [ ! -f debugging/current_log_file.txt ]; then
    echo "Error: No active log session"
    exit 1
fi

LOG_FILE=$(cat debugging/current_log_file.txt)
TIMESTAMP="[$(date +%H:%M:%S)]"

cat >> "$LOG_FILE" <<EOF

---
$TIMESTAMP 🏁 Final Summary
**Status:** ✅ COMPLETE
**Completed:** $(date +%Y-%m-%d\ %H:%M:%S)
---
EOF

echo "Log session completed: $LOG_FILE"

# Clear active log
rm debugging/current_log_file.txt
```

Usage:
```bash
./debugging/scripts/log-complete.sh
```

---

### Automated Logging

#### With TypeScript Plugin

**Plugin automatically captures:**

Every time a command runs:
```bash
task test
```

Plugin logs:
```markdown
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

**You don't need to manually log this!** The plugin handles it automatically.

#### With Bash Wrappers

**Manually wrap commands:**

```bash
# Instead of: task test
# Use:
./debugging/wrapper-scripts/run-with-logging.sh task test
```

Output is identical to plugin approach.

---

### Manual Decision Logging

**IDENTICAL TO CLAUDE CODE** - Same script, same usage:

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

## Golden Rule (Updated for OpenCode)

**v1.0 Rule:** Log → Act → Log Result
**v2.0 Rule (Plugin):** Plugin Logs → You Explain Why → Plugin Logs Result
**v2.0 Rule (Wrapper):** Wrapper Logs → You Explain Why → Wrapper Logs Result

### Workflow Comparison

**OLD (v1.0) - Manual Everything:**
```bash
echo "[14:23:45] Running unit tests" | tee -a "$LOG_FILE"
echo "Decision: Testing database layer first" | tee -a "$LOG_FILE"
task test 2>&1 | tee -a "$LOG_FILE"
echo "[14:23:47] Result: All tests passed" | tee -a "$LOG_FILE"
```
**Token cost:** ~100 tokens

**NEW (v2.0 Plugin) - Hybrid:**
```bash
# Plugin automatically logs this:
# [14:23:45] TOOL: Bash, PARAMS: task test

# You only log the "why":
./debugging/scripts/log-decision.sh decision \
  "Testing database layer first - fastest feedback loop"

# Run command (plugin captures output automatically)
task test

# Plugin automatically logs:
# [14:23:47] RESULT: ✅ Success, OUTPUT: [test results]
```
**Token cost:** ~15 tokens

**NEW (v2.0 Wrapper) - Hybrid:**
```bash
# You log the "why":
./debugging/scripts/log-decision.sh decision \
  "Testing database layer first - fastest feedback loop"

# Run command with wrapper (captures everything)
./debugging/wrapper-scripts/run-with-logging.sh task test

# Wrapper automatically logs command and result
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

### NEVER Log Manually (Automation Handles It)

❌ Command execution timestamps
❌ Command outputs
❌ Exit codes
❌ File paths being read/written
❌ "Running X command" announcements

---

## Log Entry Format

### Automated (Plugin/Wrapper)

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

**Format is identical to Claude Code** - Architect agents can grade OpenCode logs the same way.

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

**NEW Protocol (v2.0 - Plugin or Wrapper):**
| Operation | Token Cost | Count | Total |
|-----------|-----------|-------|-------|
| Automation (plugin/wrapper) | 0 | 30 | 0 |
| Decision logging | 15 | 20 | 300 |
| Milestone checkpoints | 50 | 4 | 200 |
| Final summary | 100 | 1 | 100 |
| **TOTAL** | | | **600** |

**Savings:** 2,300 tokens (79% reduction)

---

## Migration from Claude Code

### From Claude Code to OpenCode (Same Project)

If switching an existing code agent workspace from Claude Code to OpenCode:

1. **Keep existing logs:** OpenCode uses same log format
2. **Remove hooks.json:** No longer needed
3. **Add plugin OR wrappers:** Choose one approach
4. **Keep decision script:** Works identically in OpenCode
5. **Update session management:** Replace `/log-start` with script
6. **Test logging:** Verify active log file is detected

### Clean OpenCode Setup

For new OpenCode code agent workspaces:

1. **Copy plugin:** `cp -r templates/opencode/plugins/logger .opencode/plugins/`
2. **Enable plugin:** Add to `.opencode/opencode.json`
3. **Copy scripts:** `cp templates/debugging/scripts/* debugging/scripts/`
4. **Create session mgmt:** Add `log-start.sh`, `log-complete.sh`
5. **Test:** Run `log-start.sh`, execute command, check log file

---

## Grading Impact

### Updated Rubric (Logging & Traceability = 10 points)

| Score | Criteria |
|-------|----------|
| 10 | Perfect logging: Automation capturing all commands, manual logging covers all decisions/rationale/investigations |
| 8 | Good logging: Automation working, most key decisions logged |
| 6 | Adequate logging: Some manual entries missing, but automation operational |
| 4 | Poor logging: Automation not configured or many decisions not logged |
| 0-3 | No logging or batch logs at end |

**Key Grading Notes:**
- Plugin and wrapper approaches graded identically (both get full credit)
- No deduction for using wrappers vs plugin (choice is user preference)
- Still required to log decisions, rationale, investigations manually
- Deduction for not using automation: -3 points
- Deduction for missing decision logs: -1 point per occurrence

**Critical Rule (unchanged):** No logs = Maximum grade of C+ (78%), even if work is perfect.

**Cross-Tool Grading:** Architect agents grade OpenCode logs using the same rubric as Claude Code logs (identical format).

---

## Troubleshooting

### Plugin Not Logging

**Symptom:** No automated entries in log file

**Check:**
1. Plugin directory exists: `ls -la .opencode/plugins/logger`
2. Plugin enabled: `cat .opencode/opencode.json | grep logger`
3. Log file active: `cat debugging/current_log_file.txt`
4. OpenCode supports plugins: Check OpenCode version
5. Restart OpenCode session

**Fallback:** Use wrapper scripts if plugin not supported

### Wrapper Script Permission Denied

**Symptom:** "Permission denied" when running wrapper scripts

**Fix:**
1. Make executable: `chmod +x debugging/wrapper-scripts/*.sh`
2. Check OpenCode permissions configuration
3. Try absolute path: `$(pwd)/debugging/wrapper-scripts/run-with-logging.sh`

### Log File Not Found

**Symptom:** "No active log session" error

**Fix:**
```bash
# Start new session
./debugging/scripts/log-start.sh "my-session"

# Verify active log
cat debugging/current_log_file.txt
```

### Decision Script Not Working

**Symptom:** `log-decision.sh` not logging

**Fix:**
1. Make executable: `chmod +x debugging/scripts/log-decision.sh`
2. Check active log session: `cat debugging/current_log_file.txt`
3. Verify log file writable: `touch $(cat debugging/current_log_file.txt)`

---

## Examples

### Complete Session Example (Plugin)

```bash
# 1. Start session
./debugging/scripts/log-start.sh "implement-fastapi-endpoints"

# 2. Plugin automatically logs all commands from here

# 3. Add context for first major decision
./debugging/scripts/log-decision.sh decision \
  "Using FastAPI instead of Flask - better async support for concurrent API calls"

# 4. Run commands (plugin logs automatically)
task test

# 5. Investigate error (manual context)
./debugging/scripts/log-decision.sh investigation \
  "Import error traced to missing pydantic package. Root cause: not in requirements.txt"

# 6. Fix and verify (plugin logs commands, you log verification)
pip install pydantic
./debugging/scripts/log-decision.sh verification \
  "Verified pydantic installed and tests now passing"

# 7. Milestone (manual)
./debugging/scripts/log-decision.sh milestone \
  "API endpoints complete - 12/12 tests passing, ready for integration tests"

# 8. Complete session
./debugging/scripts/log-complete.sh
```

### Complete Session Example (Wrappers)

```bash
# 1. Start session
./debugging/scripts/log-start.sh "implement-fastapi-endpoints"

# 2. Add context for first major decision
./debugging/scripts/log-decision.sh decision \
  "Using FastAPI instead of Flask - better async support for concurrent API calls"

# 3. Run commands with wrapper
./debugging/wrapper-scripts/run-with-logging.sh task test

# 4. Investigate error (manual context)
./debugging/scripts/log-decision.sh investigation \
  "Import error traced to missing pydantic package. Root cause: not in requirements.txt"

# 5. Fix and verify
./debugging/wrapper-scripts/run-with-logging.sh pip install pydantic
./debugging/scripts/log-decision.sh verification \
  "Verified pydantic installed and tests now passing"

# 6. Milestone (manual)
./debugging/scripts/log-decision.sh milestone \
  "API endpoints complete - 12/12 tests passing, ready for integration tests"

# 7. Complete session
./debugging/scripts/log-complete.sh
```

### Log Output (Identical for Both Approaches)

```markdown
# Log Session: implement-fastapi-endpoints
**Started:** 2025-01-17 14:23:00

## Goal
Implement FastAPI endpoints

## Success Criteria
- [ ] All endpoints functional
- [ ] Tests passing
---

[14:23:05] 🎯 DECISION: Using FastAPI instead of Flask - better async support for concurrent API calls

---
[14:23:10] TOOL: Bash
PARAMS: command="task test"
[14:23:12] RESULT: ❌ Failed (exit code: 1)
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
**Status:** ✅ COMPLETE
**Completed:** 2025-01-17 14:30:00
---
```

**Token Usage:**
- Automation (plugin or wrapper): 0 tokens
- 4 manual entries × 15 tokens = 60 tokens
- Total: ~60 tokens (vs ~500 tokens in v1.0)

---

## Comparison: Plugin vs Wrappers

| Feature | TypeScript Plugin | Bash Wrappers |
|---------|------------------|---------------|
| **Automatic Logging** | ✅ Fully automatic | ⚠️ Manual wrapping required |
| **Token Cost** | 0 | 0 |
| **Performance** | <1ms overhead | ~5-10ms overhead |
| **Setup Complexity** | Medium (TypeScript) | Low (bash) |
| **OpenCode Version** | Requires plugin API | Any version |
| **Transparency** | TypeScript code | Readable bash |
| **Grading** | Full credit | Full credit |
| **Maintenance** | Minimal | Minimal |

**Recommendation:**
- **Use Plugin** if you have OpenCode with plugin API support (easiest, fastest)
- **Use Wrappers** if you're on older OpenCode or prefer bash (more transparent)
- **Both work equally well** for grading purposes

---

## References

- [OpenCode Setup Guide](./opencode_setup_guide.md)
- [OpenCode Migration Guide](./opencode_migration_guide.md)
- [Claude Code vs OpenCode Comparison](./claude_vs_opencode_comparison.md)
- [Permissions Setup Protocol](./permissions_setup_protocol.md)
- [Grading Workflow](./instruction_grading_workflow.md)
- [Hybrid Logging Protocol (Claude Code)](./hybrid_logging_protocol.md)

---

**Version History:**
- **v2.0 OpenCode Edition (2025-01-17):** OpenCode adaptation with plugin and wrapper approaches
- **v2.0 (2025-01-15):** Original hybrid approach for Claude Code
- **v1.0 (2025-11-13):** Original fully-manual protocol
