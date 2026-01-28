# Complete Workspace Setup Guide

**Version:** 3.0 (Hooks Fix + Dual-Mode Logging)
**Date:** 2025-11-20
**Status:** ✅ Production Ready

---

## Overview

This guide provides a complete checklist for setting up both **Architect Agent** and **Code Agent** workspaces with:
- ✅ Hybrid Logging v2.0 (hooks + wrapper scripts)
- ✅ Correct hook configuration (settings.json)
- ✅ Enhanced argument capture
- ✅ OpenCode wrapper support
- ✅ Proper permissions
- ✅ Slash commands
- ✅ CLAUDE.md and AGENTS.md configured

---

## Architect Workspace Setup

**Location:** `/path/to/architect-workspace/`

### Required Files Checklist

#### 1. Core Configuration Files

**File:** `CLAUDE.md`
**Purpose:** Instructions for Claude Code when working in architect workspace
**Required Content:**
- ✅ Workspace role identification (Architect Agent)
- ✅ File location protocol (THIS workspace vs code agent workspace)
- ✅ Code agent workspace path (READ-ONLY)
- ✅ Quick reference section
- ✅ Essential documentation links
- ✅ At session start checklist
- ✅ Skills quick reference

**Verification:**
```bash
grep "Architect Agent" CLAUDE.md
grep "code agent workspace" CLAUDE.md
```

**File:** `AGENTS.md`
**Purpose:** Mirror of CLAUDE.md for agent systems
**Required:** Must be identical to CLAUDE.md

**Verification:**
```bash
diff CLAUDE.md AGENTS.md
# Should show: no differences
```

#### 2. Workspace Directories

**Directory:** `instructions/`
**Purpose:** Store instruction files created by architect
**Required Files:**
- `README.md` - Explains instruction format
- `instruct-*.md` files (created during work)

**Verification:**
```bash
ls -la instructions/
test -f instructions/README.md && echo "✅ README exists"
```

**Directory:** `human/`
**Purpose:** Human-readable summaries of instructions (10-25 bullets)
**Required Files:**
- `README.md` - Explains human summary format
- `human-*.md` files (created during work)

**Verification:**
```bash
ls -la human/
test -f human/README.md && echo "✅ README exists"
```

**Directory:** `grades/`
**Purpose:** Grading reports for code agent work
**Required Files:**
- `README.md` - Explains grading rubric
- `grade-*.md` files (created after code agent work)

**Verification:**
```bash
ls -la grades/
test -f grades/README.md && echo "✅ README exists"
```

**Directory:** `ticket/`
**Purpose:** Current ticket tracking
**Required Files:**
- `current_ticket.md` - Active ticket context
- `ticket_history.md` - Completed tickets

**Verification:**
```bash
ls -la ticket/
test -f ticket/current_ticket.md && echo "✅ Current ticket exists"
```

**Directory:** `analysis/`
**Purpose:** Investigation and analysis documents
**Required Files:**
- `README.md` - Purpose of analysis directory

**Verification:**
```bash
ls -la analysis/
test -f analysis/README.md && echo "✅ README exists"
```

**Directory:** `docs/`
**Purpose:** Architect-specific documentation
**Required Files:**
- `hybrid_logging.md` - Hybrid Logging v2.0 overview
- `workflow.md` - Architect workflow guide
- `technology_adaptations.md` - Project-specific tech
- `critical_protocols.md` - Critical protocols and gotchas

**Verification:**
```bash
ls -la docs/
test -f docs/hybrid_logging.md && echo "✅ Logging docs exist"
test -f docs/workflow.md && echo "✅ Workflow docs exist"
```

#### 3. Slash Commands

**Directory:** `.claude/commands/`
**Purpose:** Custom slash commands for architect agent

**Required Commands:**

**File:** `.claude/commands/project.instruct.md`
**Purpose:** Read instructions, show 10-25 bullet summary
**Content Template:**
```markdown
Read the current instruction file from the code agent workspace and create a concise 10-25 bullet point summary showing:
- Main objectives
- Key requirements
- Critical constraints
- Success criteria

Display this summary to help review what will be sent to the code agent.
```

**File:** `.claude/commands/project.send.md`
**Purpose:** Send instructions to code agent, show human summary
**Content Template:**
```markdown
1. Copy the instruction file to code agent's debugging/instructions/current_instructions.md
2. Read the corresponding human summary from human/ directory
3. Display the 10-25 bullet point summary to user
4. Confirm instructions sent successfully
```

**Verification:**
```bash
ls -la .claude/commands/
test -f .claude/commands/project.instruct.md && echo "✅ /project.instruct exists"
test -f .claude/commands/project.send.md && echo "✅ /project.send exists"
```

#### 4. Permissions Configuration

**File:** `.claude/settings.json`
**Purpose:** Architect workspace settings and permissions

**Required Permissions:**
```json
{
  "allowedDirectories": [
    "/path/to/architect-workspace",
    "/path/to/code-agent-workspace/debugging/instructions"
  ]
}
```

**Critical:** Architect needs WRITE access to code agent's `debugging/instructions/` directory

**Verification:**
```bash
python3 -m json.tool .claude/settings.json > /dev/null && echo "✅ Valid JSON"
grep "debugging/instructions" .claude/settings.json && echo "✅ Code agent instructions path configured"
```

---

## Code Agent Workspace Setup

**Location:** `/path/to/code-agent-workspace/`

### Required Files Checklist

#### 1. Core Configuration Files

**File:** `CLAUDE.md`
**Purpose:** Instructions for Claude Code when working in code agent workspace
**Required Content:**
- ✅ Workspace role identification (Code Agent)
- ✅ Instruction location (`debugging/instructions/current_instructions.md`)
- ✅ Technology stack
- ✅ Essential documentation links
- ✅ Quality assurance protocol
- ✅ Available agents reference

**Verification:**
```bash
grep "Code Agent" CLAUDE.md
grep "current_instructions.md" CLAUDE.md
```

**File:** `AGENTS.md`
**Purpose:** Mirror of CLAUDE.md for agent systems
**Required:** Must be identical to CLAUDE.md

**Verification:**
```bash
diff CLAUDE.md AGENTS.md
# Should show: no differences
```

#### 2. Claude Code Hook Configuration

**File:** `.claude/settings.json`
**Purpose:** ⚠️ **CRITICAL** - Hook configuration MUST be here
**Required Content:**
```json
{
  "hooks": {
    "PostToolUse": [
      {
        "matcher": "*",
        "hooks": [
          {
            "type": "command",
            "command": "$CLAUDE_PROJECT_DIR/.claude/hook-logger.py",
            "timeout": 5
          }
        ]
      }
    ]
  }
}
```

**Verification:**
```bash
test -f .claude/settings.json && echo "✅ settings.json exists"
python3 -m json.tool .claude/settings.json > /dev/null && echo "✅ Valid JSON"
grep "PostToolUse" .claude/settings.json && echo "✅ PostToolUse hook configured"
grep "hook-logger.py" .claude/settings.json && echo "✅ Hook logger path configured"
```

**File:** `.claude/hook-logger.py`
**Purpose:** Enhanced hook logger with full argument capture
**Required:** Executable Python script
**Key Features:**
- Reads JSON from stdin
- Checks for active log session
- Enhanced tool handlers (Bash, Read, Write, Edit, Grep, Glob, TodoWrite)
- Silent failure (doesn't break hooks)

**Verification:**
```bash
test -x .claude/hook-logger.py && echo "✅ Hook logger is executable"
python3 -m py_compile .claude/hook-logger.py && echo "✅ Valid Python syntax"
grep "PostToolUse" .claude/hook-logger.py && echo "✅ Contains tool handlers"
```

**CRITICAL CHECK:**
```bash
# Ensure NO hooks.json file exists (wrong location)
if [ -f .claude/hooks.json ]; then
    echo "❌ WARNING: hooks.json found - Claude Code will NOT read this file!"
    echo "   Move configuration to .claude/settings.json"
else
    echo "✅ No hooks.json (correct - using settings.json)"
fi
```

#### 3. Logging System

**Directory:** `debugging/`
**Purpose:** Logging system and infrastructure

**Required Subdirectories:**

**Directory:** `debugging/logs/`
**Purpose:** Session log files
**Files:** `session_YYYYMMDD_HHMMSS.log` (created during sessions)

**File:** `debugging/current_log_file.txt`
**Purpose:** Tracks active log session (contains path to current log)

**Directory:** `debugging/scripts/`
**Purpose:** Logging control scripts

**Required Scripts:**

**File:** `debugging/scripts/log-start.sh`
**Purpose:** Start new logging session
**Must be:** Executable
**Usage:** `./debugging/scripts/log-start.sh "description"`

**File:** `debugging/scripts/log-checkpoint.sh`
**Purpose:** Create checkpoint in current session
**Must be:** Executable
**Usage:** `./debugging/scripts/log-checkpoint.sh "checkpoint-name"`

**File:** `debugging/scripts/log-complete.sh`
**Purpose:** Finalize logging session
**Must be:** Executable
**Usage:** `./debugging/scripts/log-complete.sh`

**File:** `debugging/scripts/log-decision.sh`
**Purpose:** Log manual context (decisions, rationale, etc.)
**Must be:** Executable
**Usage:** `./debugging/scripts/log-decision.sh <type> "message"`

**Verification:**
```bash
# Check directories
test -d debugging/logs && echo "✅ logs/ directory exists"
test -d debugging/scripts && echo "✅ scripts/ directory exists"

# Check scripts are executable
test -x debugging/scripts/log-start.sh && echo "✅ log-start.sh executable"
test -x debugging/scripts/log-checkpoint.sh && echo "✅ log-checkpoint.sh executable"
test -x debugging/scripts/log-complete.sh && echo "✅ log-complete.sh executable"
test -x debugging/scripts/log-decision.sh && echo "✅ log-decision.sh executable"
```

#### 4. OpenCode Wrapper Scripts (Dual-Mode Support)

**Directory:** `debugging/wrapper-scripts/`
**Purpose:** Shell function wrappers for OpenCode users

**Required Scripts:**

**File:** `debugging/wrapper-scripts/run-with-logging.sh`
**Purpose:** Main wrapper that logs command + result
**Must be:** Executable
**Critical:** Must use `command` prefix to prevent recursion

**File:** `debugging/wrapper-scripts/log-tool-call.sh`
**Purpose:** Pre-execution logging
**Must be:** Executable

**File:** `debugging/wrapper-scripts/log-tool-result.sh`
**Purpose:** Post-execution logging
**Must be:** Executable

**Verification:**
```bash
test -d debugging/wrapper-scripts && echo "✅ wrapper-scripts/ directory exists"
test -x debugging/wrapper-scripts/run-with-logging.sh && echo "✅ run-with-logging.sh executable"
test -x debugging/wrapper-scripts/log-tool-call.sh && echo "✅ log-tool-call.sh executable"
test -x debugging/wrapper-scripts/log-tool-result.sh && echo "✅ log-tool-result.sh executable"

# Check for critical recursion prevention
grep 'command \$COMMAND' debugging/wrapper-scripts/run-with-logging.sh && echo "✅ Recursion prevention in place"
```

**File:** `.opencode/shell-init.sh`
**Purpose:** Export shell functions for OpenCode
**Must be:** Sourceable bash script
**Key Features:**
- Only activates if `$OPENCODE` environment variable set
- Exports wrapper functions (gradle, task, git, npm, etc.)
- Each function calls `run-with-logging.sh`

**Verification:**
```bash
test -f .opencode/shell-init.sh && echo "✅ shell-init.sh exists"
bash -n .opencode/shell-init.sh && echo "✅ Valid bash syntax"
grep "export -f" .opencode/shell-init.sh && echo "✅ Functions exported"
```

#### 5. Instruction Files

**Directory:** `debugging/instructions/`
**Purpose:** Receive instructions from architect agent

**File:** `debugging/instructions/current_instructions.md`
**Purpose:** Canonical location for current instructions
**Created by:** Architect agent (copied on "send")
**Read by:** Code agent

**Verification:**
```bash
test -d debugging/instructions && echo "✅ instructions/ directory exists"
```

**CRITICAL:** Architect agent needs WRITE permission to this directory

#### 6. Documentation

**Directory:** `.claude/docs/`
**Purpose:** Code agent-specific documentation

**Required Files:**

**File:** `.claude/docs/logging_setup.md`
**Purpose:** Hybrid Logging v2.0 setup guide
**Required Content:**
- What's installed (hooks + wrappers)
- How to use (Claude Code vs OpenCode)
- Token savings explanation
- Verification procedures
- Troubleshooting guide
- Critical fix documentation (settings.json)

**File:** `.claude/docs/testing_protocol.md`
**Purpose:** Progressive testing requirements
**Required Content:**
- When to test (after what changes)
- How to test (commands by project type)
- Quality gates
- Coverage requirements

**File:** `.claude/docs/agent_usage.md`
**Purpose:** Which agents to use when
**Required Content:**
- Agent descriptions
- When to invoke each agent
- Examples of agent usage

**Verification:**
```bash
test -d .claude/docs && echo "✅ docs/ directory exists"
test -f .claude/docs/logging_setup.md && echo "✅ Logging setup docs exist"
test -f .claude/docs/testing_protocol.md && echo "✅ Testing protocol exists"
test -f .claude/docs/agent_usage.md && echo "✅ Agent usage docs exist"
```

#### 7. Slash Commands

**Directory:** `.claude/commands/`
**Purpose:** Custom slash commands for code agent

**Required Commands:**

**File:** `.claude/commands/log-start.md`
**Purpose:** Start logging session
**Content:**
```markdown
Start a new logging session using: ./debugging/scripts/log-start.sh "description"
```

**File:** `.claude/commands/log-checkpoint.md`
**Purpose:** Create checkpoint
**Content:**
```markdown
Create a checkpoint in current session using: ./debugging/scripts/log-checkpoint.sh "checkpoint-name"
```

**File:** `.claude/commands/log-complete.md`
**Purpose:** Complete session
**Content:**
```markdown
Complete the current logging session using: ./debugging/scripts/log-complete.sh
```

**File:** `.claude/commands/project.instruct.md`
**Purpose:** Read and summarize current instructions
**Content:**
```markdown
Read debugging/instructions/current_instructions.md and create a 10-25 bullet point summary of:
- Main objectives
- Key requirements
- Critical constraints
- Success criteria

Display this summary for review.
```

**Verification:**
```bash
ls -la .claude/commands/
test -f .claude/commands/log-start.md && echo "✅ /log-start exists"
test -f .claude/commands/log-checkpoint.md && echo "✅ /log-checkpoint exists"
test -f .claude/commands/log-complete.md && echo "✅ /log-complete exists"
test -f .claude/commands/project.instruct.md && echo "✅ /project.instruct exists"
```

---

## Permission Setup

### Architect Agent Permissions

**File:** `/path/to/architect-workspace/.claude/settings.json`

**Required Directories:**
```json
{
  "allowedDirectories": [
    "/path/to/architect-workspace",
    "/path/to/code-agent-workspace/debugging/instructions"
  ]
}
```

**Why These Permissions:**
- Own workspace: Full read/write for creating instructions, grades, etc.
- Code agent instructions: Write access to send instructions

**Test Permission:**
```bash
# From architect workspace, test writing to code agent instructions
echo "test" > /path/to/code-agent-workspace/debugging/instructions/test.txt
rm /path/to/code-agent-workspace/debugging/instructions/test.txt
echo "✅ Architect can write to code agent instructions"
```

### Code Agent Permissions

**File:** `/path/to/code-agent-workspace/.claude/settings.json`

**Required Directories:**
```json
{
  "allowedDirectories": [
    "/path/to/code-agent-workspace"
  ],
  "hooks": {
    "PostToolUse": [
      {
        "matcher": "*",
        "hooks": [
          {
            "type": "command",
            "command": "$CLAUDE_PROJECT_DIR/.claude/hook-logger.py",
            "timeout": 5
          }
        ]
      }
    ]
  }
}
```

**Why These Permissions:**
- Own workspace: Full read/write for code, logs, tests, etc.
- Hooks configuration: Automated logging

**Test Permission:**
```bash
# From code agent workspace, test logging scripts
./debugging/scripts/log-start.sh "test"
test -f debugging/current_log_file.txt && echo "✅ Code agent can create log sessions"
./debugging/scripts/log-complete.sh
```

---

## Verification Procedures

### 1. Hook Verification (Claude Code)

**Test:** Verify hooks are capturing tool calls

```bash
# 1. Start log session
/log-start "hook-test"

# 2. Run commands
ls -la
pwd
date

# 3. Check for TOOL entries
LOG_FILE=$(cat debugging/current_log_file.txt)
TOOL_COUNT=$(grep -c "TOOL:" "$LOG_FILE")

if [ $TOOL_COUNT -ge 3 ]; then
    echo "✅ Hooks working - $TOOL_COUNT TOOL entries found"
else
    echo "❌ Hooks not working - only $TOOL_COUNT TOOL entries"
fi

# 4. Complete session
/log-complete
```

**Expected Output:**
```
✅ Hooks working - 3 TOOL entries found
```

**Sample TOOL Entry:**
```
[HH:MM:SS] TOOL: Bash
COMMAND: ls -la
DESC: List directory contents
---
```

### 2. Wrapper Verification (OpenCode)

**Test:** Verify wrapper scripts work in OpenCode

```bash
# 1. Set environment
export OPENCODE=1

# 2. Source shell init
source .opencode/shell-init.sh

# 3. Verify functions loaded
type gradle
# Should show: "gradle is a function"

# 4. Start log session
./debugging/scripts/log-start.sh "wrapper-test"

# 5. Run commands using wrappers
gradle --version
task --version

# 6. Check for TOOL entries
grep "TOOL:" $(cat debugging/current_log_file.txt)

# 7. Complete session
./debugging/scripts/log-complete.sh
```

**Expected Output:**
```
gradle is a function
[HH:MM:SS] TOOL: Bash
COMMAND: gradle --version
...
```

### 3. Permission Verification

**Test:** Verify cross-workspace permissions

**From Architect Workspace:**
```bash
# Test writing to code agent instructions
echo "# Test Instruction" > /path/to/code-agent-workspace/debugging/instructions/test-instruction.md

if [ -f /path/to/code-agent-workspace/debugging/instructions/test-instruction.md ]; then
    echo "✅ Architect can write to code agent instructions"
    rm /path/to/code-agent-workspace/debugging/instructions/test-instruction.md
else
    echo "❌ Architect cannot write to code agent instructions - permission issue"
fi
```

**From Code Agent Workspace:**
```bash
# Test executing logging scripts
./debugging/scripts/log-start.sh "perm-test"

if [ -f debugging/current_log_file.txt ]; then
    echo "✅ Code agent can execute logging scripts"
    ./debugging/scripts/log-complete.sh
else
    echo "❌ Code agent cannot execute logging scripts - permission issue"
fi
```

### 4. Configuration File Validation

**Test:** Verify all JSON configuration files are valid

```bash
# Architect workspace
python3 -m json.tool /path/to/architect-workspace/.claude/settings.json > /dev/null && echo "✅ Architect settings.json valid"

# Code agent workspace
python3 -m json.tool /path/to/code-agent-workspace/.claude/settings.json > /dev/null && echo "✅ Code agent settings.json valid"
```

### 5. Documentation Completeness

**Test:** Verify all required documentation exists

**Architect Workspace:**
```bash
cd /path/to/architect-workspace

# Core docs
test -f CLAUDE.md && echo "✅ CLAUDE.md"
test -f AGENTS.md && echo "✅ AGENTS.md"
test -f docs/hybrid_logging.md && echo "✅ Logging docs"
test -f docs/workflow.md && echo "✅ Workflow docs"
test -f docs/technology_adaptations.md && echo "✅ Tech docs"
test -f docs/critical_protocols.md && echo "✅ Protocol docs"
```

**Code Agent Workspace:**
```bash
cd /path/to/code-agent-workspace

# Core docs
test -f CLAUDE.md && echo "✅ CLAUDE.md"
test -f AGENTS.md && echo "✅ AGENTS.md"
test -f .claude/docs/logging_setup.md && echo "✅ Logging setup"
test -f .claude/docs/testing_protocol.md && echo "✅ Testing protocol"
test -f .claude/docs/agent_usage.md && echo "✅ Agent usage"
```

---

## Common Pitfalls

### 1. Hooks in Wrong File

**Problem:** Hooks configured in `.claude/hooks.json` instead of `.claude/settings.json`

**Symptom:** No TOOL entries in log, hooks never fire

**Solution:**
```bash
# Archive wrong file
mv .claude/hooks.json .claude/hooks.json.backup

# Move configuration to settings.json
cat > .claude/settings.json <<'EOF'
{
  "hooks": {
    "PostToolUse": [
      {
        "matcher": "*",
        "hooks": [
          {
            "type": "command",
            "command": "$CLAUDE_PROJECT_DIR/.claude/hook-logger.py",
            "timeout": 5
          }
        ]
      }
    ]
  }
}
EOF
```

**Impact:** CRITICAL - This single issue prevents all hook functionality

### 2. Hook Logger Not Executable

**Problem:** `.claude/hook-logger.py` missing execute permission

**Symptom:** Hooks configured but not logging

**Solution:**
```bash
chmod +x .claude/hook-logger.py
```

**Verification:**
```bash
ls -la .claude/hook-logger.py
# Should show: -rwxr-xr-x (x permission)
```

### 3. No Active Log Session

**Problem:** Hooks working, but no TOOL entries appear

**Symptom:** `debugging/current_log_file.txt` doesn't exist or empty

**Solution:**
```bash
# Start log session
./debugging/scripts/log-start.sh "session-name"

# Verify active session
cat debugging/current_log_file.txt
# Should show path to log file
```

**Why:** Hook logger only writes when active session exists

### 4. Wrapper Scripts Missing `command` Prefix

**Problem:** OpenCode wrapper scripts cause infinite recursion

**Symptom:** Commands hang indefinitely, timeout errors

**Solution:**
```bash
# Check run-with-logging.sh for "command" prefix
grep 'eval "command' debugging/wrapper-scripts/run-with-logging.sh

# Should see: eval "command $COMMAND"
```

**Fix if missing:**
```bash
# Edit run-with-logging.sh, line ~26
# Change: eval "$COMMAND"
# To: eval "command $COMMAND"
```

**Impact:** CRITICAL - Without this, wrappers cause infinite recursion

### 5. CLAUDE.md and AGENTS.md Out of Sync

**Problem:** Files have different content

**Symptom:** Inconsistent behavior between Claude Code and agent systems

**Solution:**
```bash
# Make AGENTS.md identical to CLAUDE.md
cp CLAUDE.md AGENTS.md

# Verify
diff CLAUDE.md AGENTS.md
# Should show: no differences
```

**Best Practice:** Update both files together, verify with diff

### 6. Incorrect Permission Paths

**Problem:** Architect can't write to code agent instructions

**Symptom:** "Permission denied" when trying to send instructions

**Solution:**
```bash
# Verify path in architect's settings.json
grep "debugging/instructions" /path/to/architect-workspace/.claude/settings.json

# Should match actual code agent path
ls -la /path/to/code-agent-workspace/debugging/instructions/
```

**Fix:**
Update `allowedDirectories` in architect's `.claude/settings.json` with correct absolute path

### 7. Missing Slash Commands

**Problem:** Commands like `/log-start` or `/project.instruct` not recognized

**Symptom:** "Unknown command" error

**Solution:**
```bash
# Check commands directory
ls -la .claude/commands/

# Verify file exists and has .md extension
test -f .claude/commands/log-start.md && echo "✅ Command exists"
```

**Fix:** Create missing command files in `.claude/commands/` directory

### 8. $CLAUDE_PROJECT_DIR Not Set

**Problem:** Hook script path uses `$CLAUDE_PROJECT_DIR` but variable empty

**Symptom:** Hooks don't execute, no error message

**Solution:**
```bash
# Verify running under Claude Code CLI
echo $CLAUDE_PROJECT_DIR
# Should show workspace path

# If empty, use absolute path in settings.json instead:
"command": "/absolute/path/to/.claude/hook-logger.py"
```

**Note:** `$CLAUDE_PROJECT_DIR` is automatically set by `claude` CLI

### 9. Invalid JSON in Configuration Files

**Problem:** Syntax error in `.claude/settings.json`

**Symptom:** Hooks don't work, no clear error

**Solution:**
```bash
# Validate JSON
python3 -m json.tool .claude/settings.json

# If errors, fix syntax (missing commas, quotes, etc.)
```

**Common Errors:**
- Missing commas between array elements
- Trailing commas (invalid in JSON)
- Unquoted keys
- Single quotes instead of double quotes

### 10. Wrapper Functions Not Exported

**Problem:** Shell functions defined but not exported in `.opencode/shell-init.sh`

**Symptom:** Commands run normally, no logging in OpenCode

**Solution:**
```bash
# Check for export commands
grep "export -f" .opencode/shell-init.sh

# Should see:
# export -f gradle
# export -f task
# etc.
```

**Fix:** Add `export -f functionname` for each wrapper function

---

## Quick Setup Checklist

### New Architect Workspace

- [ ] Create `CLAUDE.md` with architect agent configuration
- [ ] Copy to `AGENTS.md`
- [ ] Create directories: `instructions/`, `human/`, `grades/`, `ticket/`, `analysis/`, `docs/`
- [ ] Create `.claude/commands/project.instruct.md`
- [ ] Create `.claude/commands/project.send.md`
- [ ] Configure `.claude/settings.json` with code agent instructions path
- [ ] Create documentation in `docs/`
- [ ] Test write permission to code agent instructions

### New Code Agent Workspace

- [ ] Create `CLAUDE.md` with code agent configuration
- [ ] Copy to `AGENTS.md`
- [ ] Create `.claude/settings.json` with hooks configuration (NOT hooks.json!)
- [ ] Install `.claude/hook-logger.py` and make executable
- [ ] Create `debugging/logs/` directory
- [ ] Install logging scripts in `debugging/scripts/` and make executable
- [ ] Install wrapper scripts in `debugging/wrapper-scripts/` and make executable
- [ ] Create `.opencode/shell-init.sh` for OpenCode support
- [ ] Create `debugging/instructions/` directory
- [ ] Create documentation in `.claude/docs/`
- [ ] Create slash commands in `.claude/commands/`
- [ ] Test hooks with `/log-start` and verify TOOL entries
- [ ] Test wrappers in OpenCode (if applicable)

---

## Upgrade Existing Workspaces

### From v1.0 (Manual Logging) to v2.0 (Hybrid Logging)

**Architect Workspace:**
1. Update `CLAUDE.md` with v2.0 references
2. Update `AGENTS.md` (keep identical)
3. Add `/project.instruct` and `/project.send` commands
4. Update `docs/` with v2.0 documentation

**Code Agent Workspace:**
1. **CRITICAL:** Move hooks from `hooks.json` to `settings.json`
2. Update `.claude/hook-logger.py` with enhanced tool handlers
3. Install wrapper scripts in `debugging/wrapper-scripts/`
4. Create `.opencode/shell-init.sh`
5. Update `.claude/docs/logging_setup.md` with v2.0 info
6. Add `/project.instruct` command
7. Test hooks and wrappers

### Migration Script

```bash
#!/bin/bash
# migrate-to-v2.sh - Upgrade workspace to Hybrid Logging v2.0

echo "Migrating code agent workspace to v2.0..."

# 1. Move hooks.json to settings.json
if [ -f .claude/hooks.json ]; then
    echo "Moving hooks.json to hooks.json.backup..."
    mv .claude/hooks.json .claude/hooks.json.backup

    echo "Creating settings.json with hooks..."
    cat > .claude/settings.json <<'EOF'
{
  "hooks": {
    "PostToolUse": [
      {
        "matcher": "*",
        "hooks": [
          {
            "type": "command",
            "command": "$CLAUDE_PROJECT_DIR/.claude/hook-logger.py",
            "timeout": 5
          }
        ]
      }
    ]
  }
}
EOF
fi

# 2. Update hook logger (copy from template)
echo "Updating hook-logger.py..."
cp ~/.claude/skills/architect-agent/templates/code-agent-workspace/.claude/hook-logger.py .claude/

# 3. Install wrapper scripts
echo "Installing wrapper scripts..."
mkdir -p debugging/wrapper-scripts
cp ~/.claude/skills/architect-agent/templates/code-agent-workspace/debugging/wrapper-scripts/*.sh debugging/wrapper-scripts/
chmod +x debugging/wrapper-scripts/*.sh

# 4. Install OpenCode shell init
echo "Installing OpenCode support..."
mkdir -p .opencode
cp ~/.claude/skills/architect-agent/templates/code-agent-workspace/.opencode/shell-init.sh .opencode/

# 5. Test hooks
echo "Testing hooks..."
./debugging/scripts/log-start.sh "migration-test"
ls -la
grep "TOOL:" $(cat debugging/current_log_file.txt)
./debugging/scripts/log-complete.sh

echo "✅ Migration complete!"
```

---

## References

**Related Documentation:**
- `hook_configuration_critical.md` - Settings.json fix (THE critical discovery)
- `hook_logger_enhancements.md` - Enhanced argument capture
- `opencode_wrapper_setup.md` - Dual-mode logging for OpenCode
- `hybrid_logging_protocol.md` - Complete v2.0 protocol
- `permissions_setup_protocol.md` - Cross-workspace permissions
- `grading_rubrics.md` - Grading system including hook grading

**Template Files:**
- `templates/architect-workspace/` - All architect workspace files
- `templates/code-agent-workspace/` - All code agent workspace files

---

**Last Updated:** 2025-11-20
**Status:** Production Ready
**Version:** 3.0 (Hooks Fix + Dual-Mode Logging)
