# OpenCode Migration Guide

**Purpose:** Guide for migrating existing code agent workspaces from Claude Code to OpenCode
**Audience:** Users switching their code agent from Claude Code to OpenCode
**Estimated Time:** 15-20 minutes

---

## Table of Contents

1. [Should You Migrate?](#should-you-migrate)
2. [What Changes](#what-changes)
3. [What Stays the Same](#what-stays-the-same)
4. [Pre-Migration Checklist](#pre-migration-checklist)
5. [Migration Steps](#migration-steps)
6. [Post-Migration Verification](#post-migration-verification)
7. [Troubleshooting](#troubleshooting)
8. [Rollback Instructions](#rollback-instructions)

---

## Should You Migrate?

### Reasons to Migrate to OpenCode

✅ **Open Source** - Full transparency, community-driven development
✅ **No Vendor Lock-in** - Not tied to Anthropic's Claude Code product
✅ **Customizable** - Can modify and extend behavior
✅ **Cost** - May have different pricing model than Claude Code
✅ **Features** - OpenCode may have features not in Claude Code

### Reasons to Stay with Claude Code

✅ **Official Support** - Maintained by Anthropic
✅ **Native Integration** - Built specifically for Claude
✅ **Slash Commands** - Both tools now support slash commands (same format!)
✅ **Simpler Setup** - Hooks.json vs JavaScript plugin
✅ **Documentation** - Official Claude Code documentation

### Hybrid Approach (Best of Both)

**Recommended for many users:**
- **Architect Agent** uses **Claude Code** (benefits from slash commands, native integration)
- **Code Agent** uses **OpenCode** (open source, customizable, cost-effective)
- **Same log format** ensures grading compatibility

This guide focuses on migrating the **code agent workspace** from Claude Code → OpenCode.

---

## What Changes

### Files/Configuration to Change

| Claude Code | OpenCode Equivalent | Action |
|------------|-------------------|--------|
| `.claude/hooks.json` | `.opencode/plugin/logger.js` OR `debugging/wrapper-scripts/*.sh` | Remove `.claude/hooks.json`, add plugin or wrappers |
| `.claude/settings.local.json` | OpenCode-specific permissions config | Update permissions syntax (consult OpenCode docs) |
| `.claude/commands/*.md` | `.opencode/command/*.md` | Copy slash commands (same format, different directory) |
| `/log-start` slash command | `/log-start` OR `./debugging/scripts/log-start.sh` | OpenCode now has native slash commands too! |
| `/log-checkpoint` slash command | `/log-checkpoint` OR `./debugging/scripts/log-decision.sh milestone` | Use slash command or script |
| `/log-complete` slash command | `/log-complete` OR `./debugging/scripts/log-complete.sh` | Use slash command or script |

### Slash Command Parity (NEW)

**OpenCode now supports slash commands!** The directory structure differs slightly:

| Claude Code | OpenCode |
|------------|----------|
| `.claude/commands/` | `.opencode/command/` |

The file format is identical (markdown with optional YAML frontmatter). The templates now include pre-configured OpenCode slash commands that mirror Claude Code's commands exactly.

### Behavioral Changes

- **Automated Logging:** Hooks → Plugin or Wrappers
- **Session Management:** Slash commands → Bash scripts
- **Permission Model:** May differ (consult OpenCode docs)
- **Tool Availability:** OpenCode may have different MCP servers

---

## What Stays the Same

### Files/Scripts That Don't Change

✅ **`debugging/scripts/log-decision.sh`** - Works identically
✅ **`debugging/scripts/get-unstuck.sh`** - Works identically
✅ **`debugging/logs/`** - Same log file format and location
✅ **`debugging/current_log_file.txt`** - Same active log pointer
✅ **`instructions/`** - Architect still writes here
✅ **`human/`** - Architect still writes here

### Protocols That Don't Change

✅ **Log Format** - Markdown with timestamps, identical format
✅ **Manual Logging** - Decision, rationale, investigation, etc.
✅ **Grading Rubric** - Same 10-point logging criteria
✅ **Two-Layer Approach** - Automated + manual still applies
✅ **Token Savings** - Same 60-70% reduction achieved

### Workflow That Doesn't Change

✅ **Architect creates instructions** → Code agent reads and implements
✅ **Code agent logs work** → Architect reads logs and grades
✅ **Cross-workspace access** → Same permissions patterns
✅ **Test → Fix → Verify cycle** → Same development workflow

---

## Pre-Migration Checklist

### 1. Backup Current Workspace

```bash
# Create backup of code agent workspace
cd ~/clients/project/src/
tar -czf code-agent-backup-$(date +%Y%m%d).tar.gz code-agent-workspace/

# Verify backup
ls -lh code-agent-backup-*.tar.gz
```

### 2. Complete In-Progress Work

- [ ] Finish current instruction implementation
- [ ] Complete active log session (`/log-complete` or complete manually)
- [ ] Commit all changes to git
- [ ] Ensure no unsaved work

### 3. Document Current State

```bash
# Save current Claude Code config
cp .claude/hooks.json .claude/hooks.json.backup
cp .claude/settings.local.json .claude/settings.local.json.backup

# List current logs
ls -lt debugging/logs/ > debugging/pre-migration-logs.txt
```

### 4. Verify OpenCode Installation

```bash
# Check OpenCode is installed
which opencode

# Check version
opencode --version

# Test basic functionality
opencode --help
```

---

## Migration Steps

### Step 1: Remove Claude Code Configuration

```bash
# Move Claude Code config to backup
mkdir -p .claude-backup
mv .claude/hooks.json .claude-backup/
mv .claude/settings.local.json .claude-backup/

# Keep .claude directory for reference
# (You can delete .claude entirely if preferred)
```

### Step 2: Choose Automation Approach

**Option A: TypeScript Plugin (Recommended)**

```bash
# Create plugin directory
mkdir -p .opencode/plugins

# Copy plugin from architect workspace
cp -r ~/.claude/skills/architect-agent/templates/opencode/plugins/logger \
      .opencode/plugins/

# Create opencode.json
cat > .opencode/opencode.json <<'EOF'
{
  "plugins": ["./plugins/logger"],
  "description": "OpenCode configuration for architect-agent code agent workspace"
}
EOF
```

**Option B: Bash Wrappers**

```bash
# Copy wrapper scripts
mkdir -p debugging/wrapper-scripts
cp ~/.claude/skills/architect-agent/templates/opencode/wrapper-scripts/*.sh \
   debugging/wrapper-scripts/

# Make executable
chmod +x debugging/wrapper-scripts/*.sh
```

### Step 3: Add Session Management Scripts

```bash
# Create log-start.sh
cat > debugging/scripts/log-start.sh <<'EOF'
#!/bin/bash
# Start a new log session
set -euo pipefail

DESCRIPTION=${1:-"session"}
TIMESTAMP=$(date +%Y%m%d_%H%M%S)
LOG_FILE="debugging/logs/log-${TIMESTAMP}-${DESCRIPTION}.md"

mkdir -p debugging/logs

cat > "$LOG_FILE" <<LOGEOF
# Log Session: $DESCRIPTION
**Started:** $(date '+%Y-%m-%d %H:%M:%S')

## Goal
[Document your goal here]

## Success Criteria
- [ ] Criterion 1
- [ ] Criterion 2

---

LOGEOF

echo "$LOG_FILE" > debugging/current_log_file.txt
echo "✅ Log session started: $LOG_FILE"
EOF

chmod +x debugging/scripts/log-start.sh

# Create log-complete.sh
cat > debugging/scripts/log-complete.sh <<'EOF'
#!/bin/bash
# Complete the current log session
set -euo pipefail

if [ ! -f debugging/current_log_file.txt ]; then
    echo "❌ Error: No active log session"
    exit 1
fi

LOG_FILE=$(cat debugging/current_log_file.txt)
TIMESTAMP="[$(date +%H:%M:%S)]"

cat >> "$LOG_FILE" <<LOGEOF

---
$TIMESTAMP 🏁 Final Summary
**Status:** ✅ COMPLETE
**Completed:** $(date '+%Y-%m-%d %H:%M:%S')
---
LOGEOF

echo "✅ Log session completed: $LOG_FILE"
rm debugging/current_log_file.txt
EOF

chmod +x debugging/scripts/log-complete.sh
```

### Step 4: Update Permissions (OpenCode-Specific)

Consult OpenCode documentation for permission configuration. The concept is similar:

**Claude Code (OLD):**
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

**OpenCode (adapt syntax):**
```json
{
  "permissions": {
    "allow": [
      "debugging/scripts/log-decision.sh",
      "debugging/scripts/log-start.sh",
      "debugging/scripts/log-complete.sh",
      "debugging/wrapper-scripts/*.sh"
    ]
  }
}
```

*(Exact syntax depends on OpenCode version - check documentation)*

### Step 5: Verify Script Compatibility

```bash
# Verify all scripts are executable
ls -la debugging/scripts/
# Should show:
# log-decision.sh (executable)
# log-start.sh (executable)
# log-complete.sh (executable)
# get-unstuck.sh (executable, if present)

# Test decision logging (should work unchanged)
./debugging/scripts/log-start.sh "migration-test"
./debugging/scripts/log-decision.sh decision "Testing migration to OpenCode"
cat $(cat debugging/current_log_file.txt)
./debugging/scripts/log-complete.sh
```

---

## Post-Migration Verification

### 1. Test Complete Workflow

```bash
# Start new log session
./debugging/scripts/log-start.sh "post-migration-test"

# Verify active log created
cat debugging/current_log_file.txt

# Test manual decision logging
./debugging/scripts/log-decision.sh decision "Migrated to OpenCode successfully"
./debugging/scripts/log-decision.sh rationale "Open source, customizable, same log format"

# Test automated logging

# If using TypeScript plugin:
#   Just run a command (plugin logs automatically)
ls -la debugging/

# If using bash wrappers:
./debugging/wrapper-scripts/run-with-logging.sh ls -la debugging/

# Check log file
LOG_FILE=$(cat debugging/current_log_file.txt)
cat "$LOG_FILE"

# Should contain:
# ✅ Session header
# ✅ Manual decision entries
# ✅ Automated command logs (from plugin or wrapper)

# Complete session
./debugging/scripts/log-complete.sh

# Verify session closed
[ ! -f debugging/current_log_file.txt ] && echo "✅ Session management working"
```

### 2. Verify Log Format Compatibility

```bash
# Compare new OpenCode log to old Claude Code log
OLD_LOG=$(ls -t debugging/logs/*.md | grep -v "$(date +%Y%m%d)" | head -n 1)
NEW_LOG=$(ls -t debugging/logs/*.md | head -n 1)

echo "Old Claude Code log:"
head -n 30 "$OLD_LOG"

echo ""
echo "New OpenCode log:"
head -n 30 "$NEW_LOG"

# Both should have:
# - Markdown format
# - Timestamps in [HH:MM:SS] format
# - Decision entries with emoji markers
# - Tool call/result entries with --- separators
```

### 3. Verify Cross-Workspace Access

From **architect agent workspace**:

```bash
# Test reading code agent logs
CODE_AGENT_LOGS="~/clients/project/src/code-agent-workspace/debugging/logs"
ls -lt "$CODE_AGENT_LOGS/" | head -n 5

# Should list logs without errors
```

### 4. Test First Real Task

- [ ] Receive new instruction from architect
- [ ] Start log session with meaningful description
- [ ] Log first decision
- [ ] Execute commands (with plugin or wrapper)
- [ ] Complete task and finish log session
- [ ] Architect can read and grade log successfully

---

## Troubleshooting

### OpenCode Won't Start

**Symptom:** OpenCode crashes or shows error on startup

**Diagnosis:**
```bash
# Check OpenCode version
opencode --version

# Check opencode.json syntax
cat .opencode/opencode.json | jq .

# Check plugin errors (if using plugin)
ls -la .opencode/plugins/logger/
```

**Solutions:**
1. Verify OpenCode is properly installed
2. Check opencode.json syntax with `jq`
3. If plugin errors, try wrapper approach instead
4. Check OpenCode logs for specific errors

### Automated Logging Not Working

**Symptom:** Commands execute but no automated logs

**Plugin Approach:**
```bash
# Verify plugin loaded
# (Check OpenCode startup logs)

# Verify plugin files exist
ls -la .opencode/plugins/logger/

# Check active log session
cat debugging/current_log_file.txt
```

**Wrapper Approach:**
```bash
# Verify wrappers are executable
ls -la debugging/wrapper-scripts/

# Test wrapper manually
./debugging/wrapper-scripts/run-with-logging.sh echo "test"

# Check if output was logged
cat $(cat debugging/current_log_file.txt)
```

### Session Management Scripts Not Working

**Symptom:** Can't start or complete log sessions

**Diagnosis:**
```bash
# Check scripts exist and are executable
ls -la debugging/scripts/log-*.sh

# Test script directly
bash -x debugging/scripts/log-start.sh test-session
```

**Solutions:**
```bash
# Make executable
chmod +x debugging/scripts/log-*.sh

# Check for syntax errors
bash -n debugging/scripts/log-start.sh
bash -n debugging/scripts/log-complete.sh

# Recreate scripts from migration steps if needed
```

### Grading Incompatibility

**Symptom:** Architect can't grade OpenCode logs properly

**Cause:** Log format differences

**Solution:**
```bash
# Ensure log format matches Claude Code:
# - Markdown format
# - Timestamps: [HH:MM:SS]
# - Decision entries: [HH:MM:SS] 🎯 DECISION: ...
# - Tool calls: --- separator, TOOL:, PARAMS:, RESULT:

# Compare to reference format in references/opencode_logging_protocol.md
```

### Permission Errors

**Symptom:** "Permission denied" when running scripts

**Solutions:**
1. Make all scripts executable
2. Configure OpenCode permissions (check OpenCode docs)
3. Try absolute paths instead of relative
4. Check file system permissions

---

## Rollback Instructions

If migration fails or you need to revert:

### Quick Rollback

```bash
# Restore Claude Code configuration
cp .claude-backup/hooks.json .claude/
cp .claude-backup/settings.local.json .claude/

# Remove OpenCode configuration
rm -rf .opencode

# Remove session management scripts
rm debugging/scripts/log-start.sh
rm debugging/scripts/log-complete.sh

# Existing logs remain intact
# Existing decision script unchanged
```

### Full Rollback from Backup

```bash
# Extract full backup
cd ~/clients/project/src/
tar -xzf code-agent-backup-YYYYMMDD.tar.gz

# Verify restoration
diff -r code-agent-workspace/ code-agent-workspace-backup/
```

---

## Migration Checklist

Use this checklist to track migration progress:

### Pre-Migration

- [ ] Backup workspace created
- [ ] In-progress work completed
- [ ] Current state documented
- [ ] OpenCode installed and verified

### Migration

- [ ] Claude Code config backed up
- [ ] `.claude/hooks.json` removed
- [ ] OpenCode automation chosen (plugin or wrapper)
- [ ] Automation installed and configured
- [ ] Session management scripts created
- [ ] Permissions updated for OpenCode
- [ ] All scripts made executable

### Post-Migration

- [ ] Complete workflow tested
- [ ] Log format verified compatible
- [ ] Cross-workspace access verified
- [ ] First real task completed successfully
- [ ] Architect can grade logs properly

### Cleanup (Optional)

- [ ] `.claude` directory removed (or kept for reference)
- [ ] Backup files removed (after 30 days of successful operation)
- [ ] Documentation updated to reflect OpenCode usage

---

## Side-by-Side Comparison

### Starting a Log Session

**Claude Code:**
```bash
# Use slash command
/log-start
```

**OpenCode:**
```bash
# Use bash script
./debugging/scripts/log-start.sh "task-description"
```

### Automated Command Logging

**Claude Code:**
```bash
# Just run the command - hooks log automatically
task test
```

**OpenCode (Plugin):**
```bash
# Just run the command - plugin logs automatically
task test
```

**OpenCode (Wrapper):**
```bash
# Wrap the command
./debugging/wrapper-scripts/run-with-logging.sh task test
```

### Manual Decision Logging

**Both Claude Code and OpenCode (IDENTICAL):**
```bash
./debugging/scripts/log-decision.sh decision "Using async approach"
```

### Completing a Log Session

**Claude Code:**
```bash
# Use slash command
/log-complete
```

**OpenCode:**
```bash
# Use bash script
./debugging/scripts/log-complete.sh
```

---

## Related Documentation

- [OpenCode Logging Protocol](./opencode_logging_protocol.md) - Complete protocol for OpenCode
- [OpenCode Setup Guide](./opencode_setup_guide.md) - Setting up from scratch
- [Claude vs OpenCode Comparison](./claude_vs_opencode_comparison.md) - Detailed feature comparison
- [Hybrid Logging Protocol (Claude Code)](./hybrid_logging_protocol.md) - Original protocol

---

**Last Updated:** 2025-01-17
**Version:** 1.0
