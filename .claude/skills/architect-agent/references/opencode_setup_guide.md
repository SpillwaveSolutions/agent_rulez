# OpenCode Setup Guide

**Purpose:** Complete step-by-step instructions for setting up OpenCode-based code agent workspaces with the architect-agent skill
**Audience:** Architect agents setting up new code agent workspaces using OpenCode
**Estimated Time:** 10-15 minutes

---

## Table of Contents

1. [Prerequisites](#prerequisites)
2. [Directory Structure](#directory-structure)
3. [Setup Path A: TypeScript Plugin](#setup-path-a-typescript-plugin-recommended)
4. [Setup Path B: Bash Wrappers](#setup-path-b-bash-wrappers-alternative)
5. [Common Setup Steps](#common-setup-steps-both-paths)
6. [Verification](#verification)
7. [Troubleshooting](#troubleshooting)
8. [Next Steps](#next-steps)

---

## Prerequisites

### Required

- **OpenCode installed** - `opencode --version` should work
- **Code agent workspace** - A directory where the code agent will work
- **Architect agent workspace** - Access to `~/.claude/skills/architect-agent`

### Optional

- **TypeScript/Node.js** - Only if using TypeScript plugin (Path A)
- **jq** - For JSON manipulation (helpful for verification)

---

## Directory Structure

Your code agent workspace should have this structure after setup:

```
code-agent-workspace/
├── .opencode/
│   ├── opencode.json              # OpenCode configuration
│   └── plugins/                   # Path A: Plugin approach
│       └── logger/
│           ├── index.ts
│           └── README.md
├── debugging/
│   ├── current_log_file.txt       # Active log session pointer
│   ├── logs/                      # Log files stored here
│   ├── scripts/                   # Manual logging scripts
│   │   ├── log-start.sh           # Start log session
│   │   ├── log-complete.sh        # Complete log session
│   │   └── log-decision.sh        # Manual decision logging
│   └── wrapper-scripts/           # Path B: Wrapper approach
│       ├── run-with-logging.sh
│       ├── log-tool-call.sh
│       └── log-tool-result.sh
├── instructions/                  # (Architect writes here)
└── human/                         # (Architect writes here)
```

---

## Setup Path A: TypeScript Plugin (Recommended)

Use this approach if:
- You have OpenCode with plugin API support
- You want fully automatic logging (no manual wrapping)
- You prefer the fastest performance

### Step 1: Create Plugin Directory

```bash
cd ~/clients/project/src/code-agent-workspace
mkdir -p .opencode/plugins
```

### Step 2: Copy Plugin

```bash
cp -r ~/.claude/skills/architect-agent/templates/opencode/plugins/logger \
      .opencode/plugins/
```

### Step 3: Verify Plugin Files

```bash
ls -la .opencode/plugins/logger/
# Should show:
# index.ts
# README.md
```

### Step 4: Create OpenCode Configuration

```bash
cat > .opencode/opencode.json <<'EOF'
{
  "plugins": ["./plugins/logger"],
  "description": "OpenCode configuration for architect-agent code agent workspace"
}
EOF
```

### Step 5: Verify Plugin Configuration

```bash
cat .opencode/opencode.json | jq .plugins
# Should show: ["./plugins/logger"]
```

### Step 6: Test Plugin (Optional)

If OpenCode is already running in this workspace, restart it to load the plugin:

```bash
# Exit OpenCode and restart
opencode .
```

Check startup logs for:
- ✅ No error messages about plugin loading
- ✅ Plugin name appears in loaded plugins list

---

## Setup Path B: Bash Wrappers (Alternative)

Use this approach if:
- OpenCode doesn't support plugins (older version)
- You prefer bash scripts over TypeScript
- You want more transparent/readable automation

### Step 1: Create Wrapper Scripts Directory

```bash
cd ~/clients/project/src/code-agent-workspace
mkdir -p debugging/wrapper-scripts
```

### Step 2: Copy Wrapper Scripts

```bash
cp ~/.claude/skills/architect-agent/templates/opencode/wrapper-scripts/*.sh \
   debugging/wrapper-scripts/
```

### Step 3: Make Scripts Executable

```bash
chmod +x debugging/wrapper-scripts/*.sh
```

### Step 4: Verify Wrapper Scripts

```bash
ls -la debugging/wrapper-scripts/
# Should show (all executable):
# run-with-logging.sh
# log-tool-call.sh
# log-tool-result.sh
```

### Step 5: Test Wrapper (Optional)

```bash
# Create dummy log session
mkdir -p debugging/logs
echo "debugging/logs/test.md" > debugging/current_log_file.txt
touch debugging/logs/test.md

# Test wrapper
./debugging/wrapper-scripts/run-with-logging.sh echo "Hello World"

# Check log file
cat debugging/logs/test.md
# Should show logged command and output

# Clean up test
rm debugging/current_log_file.txt debugging/logs/test.md
```

---

## Common Setup Steps (Both Paths)

These steps are required regardless of which automation approach you chose.

### Step 1: Create Debugging Directory Structure

```bash
cd ~/clients/project/src/code-agent-workspace
mkdir -p debugging/{logs,scripts}
```

### Step 2: Copy Manual Logging Scripts

```bash
# Copy decision logging script
cp ~/.claude/skills/architect-agent/templates/debugging/scripts/log-decision.sh \
   debugging/scripts/

# Make executable
chmod +x debugging/scripts/log-decision.sh
```

### Step 3: Copy Get Unstuck Script (Optional but Recommended)

```bash
# Copy research orchestration script
cp ~/.claude/skills/architect-agent/templates/debugging/scripts/get-unstuck.sh \
   debugging/scripts/

# Make executable
chmod +x debugging/scripts/get-unstuck.sh
```

### Step 4: Create Session Management Scripts

#### Create log-start.sh

```bash
cat > debugging/scripts/log-start.sh <<'EOF'
#!/bin/bash
# Start a new log session

set -euo pipefail

# Get description from first argument or use default
DESCRIPTION=${1:-"session"}

# Generate log filename with timestamp
TIMESTAMP=$(date +%Y%m%d_%H%M%S)
LOG_FILE="debugging/logs/log-${TIMESTAMP}-${DESCRIPTION}.md"

# Create log directory if needed
mkdir -p debugging/logs

# Initialize log file
cat > "$LOG_FILE" <<LOGEOF
# Log Session: $DESCRIPTION
**Started:** $(date '+%Y-%m-%d %H:%M:%S')

## Goal
[Document your goal here]

## Success Criteria
- [ ] Criterion 1
- [ ] Criterion 2
- [ ] Criterion 3

---

LOGEOF

# Set as active log
echo "$LOG_FILE" > debugging/current_log_file.txt

echo "✅ Log session started: $LOG_FILE"
echo ""
echo "To complete session, run: ./debugging/scripts/log-complete.sh"
EOF

chmod +x debugging/scripts/log-start.sh
```

#### Create log-complete.sh

```bash
cat > debugging/scripts/log-complete.sh <<'EOF'
#!/bin/bash
# Complete the current log session

set -euo pipefail

# Check for active log session
if [ ! -f debugging/current_log_file.txt ]; then
    echo "❌ Error: No active log session"
    echo "Start a session with: ./debugging/scripts/log-start.sh <description>"
    exit 1
fi

LOG_FILE=$(cat debugging/current_log_file.txt)
TIMESTAMP="[$(date +%H:%M:%S)]"

# Append final summary
cat >> "$LOG_FILE" <<LOGEOF

---
$TIMESTAMP 🏁 Final Summary
**Status:** ✅ COMPLETE
**Completed:** $(date '+%Y-%m-%d %H:%M:%S')

## Summary
[Summarize what was accomplished]

## Test Results
[Document final test results]

## Files Modified
[List key files changed]

---
LOGEOF

echo "✅ Log session completed: $LOG_FILE"
echo ""
echo "Log file saved for grading review"

# Clear active log
rm debugging/current_log_file.txt
EOF

chmod +x debugging/scripts/log-complete.sh
```

### Step 5: Verify All Scripts

```bash
ls -la debugging/scripts/
# Should show (all executable):
# log-start.sh
# log-complete.sh
# log-decision.sh
# get-unstuck.sh (if copied)
```

---

## Permissions Configuration (OpenCode-Specific)

OpenCode may handle permissions differently than Claude Code. Check OpenCode's documentation for pre-approving script execution.

### Conceptual Equivalent (Adapt to OpenCode Syntax)

If OpenCode supports a similar permission model:

```json
{
  "permissions": {
    "allow": [
      "Bash(./debugging/scripts/log-decision.sh:*)",
      "Bash(debugging/scripts/log-decision.sh:*)",
      "Bash(./debugging/scripts/get-unstuck.sh:*)",
      "Bash(debugging/wrapper-scripts/*.sh:*)"
    ]
  }
}
```

**Action Required:** Consult OpenCode documentation for:
- Where to configure permissions (`.opencode/settings.json`? `.opencode/config.json`?)
- Syntax for pre-approving bash scripts
- Whether wildcards are supported

---

## Verification

### 1. Verify Directory Structure

```bash
# From code agent workspace root
tree -d -L 3 .
```

Expected output:
```
.
├── .opencode
│   └── plugins          # (if using plugin approach)
│       └── logger
├── debugging
│   ├── logs
│   ├── scripts
│   └── wrapper-scripts  # (if using wrapper approach)
├── instructions
└── human
```

### 2. Verify Scripts are Executable

```bash
# All should return "executable"
[ -x debugging/scripts/log-start.sh ] && echo "executable" || echo "not executable"
[ -x debugging/scripts/log-complete.sh ] && echo "executable" || echo "not executable"
[ -x debugging/scripts/log-decision.sh ] && echo "executable" || echo "not executable"
```

### 3. Test Complete Workflow

```bash
# Start log session
./debugging/scripts/log-start.sh "verification-test"

# Verify active log created
cat debugging/current_log_file.txt
# Should show: debugging/logs/log-YYYYMMDD_HHMMSS-verification-test.md

# Test manual decision logging
./debugging/scripts/log-decision.sh decision "This is a test decision"

# Test automated logging (choose one):

# If using plugin:
#   Just run a command - plugin logs automatically
ls -la

# If using wrapper:
./debugging/wrapper-scripts/run-with-logging.sh ls -la

# Check log file
LOG_FILE=$(cat debugging/current_log_file.txt)
cat "$LOG_FILE"

# Should contain:
# - Session header with goal/criteria
# - Decision entry
# - Automated command log

# Complete session
./debugging/scripts/log-complete.sh

# Verify session completed
[ ! -f debugging/current_log_file.txt ] && echo "✅ Session closed properly"

# Review final log
ls -lt debugging/logs/ | head -n 2
```

### 4. Verify Cross-Workspace Access (Architect → Code Agent)

From **architect agent workspace**:

```bash
# Test read access to code agent logs
CODE_AGENT_WORKSPACE="~/clients/project/src/code-agent-workspace"
ls -la "$CODE_AGENT_WORKSPACE/debugging/logs/"

# Should list log files without permission errors
```

From **code agent workspace**:

```bash
# Test read access to architect references
ARCHITECT_WORKSPACE="~/.claude/skills/architect-agent"
ls -la "$ARCHITECT_WORKSPACE/references/"

# Should list reference docs without permission errors
```

---

## Troubleshooting

### Plugin Not Loading (Path A)

**Symptom:** OpenCode starts but no automated logging

**Diagnosis:**
```bash
# Check plugin directory exists
ls -la .opencode/plugins/logger/

# Check opencode.json syntax
cat .opencode/opencode.json | jq .

# Check OpenCode version
opencode --version
```

**Solutions:**
1. Verify OpenCode supports plugins (check documentation)
2. Try restarting OpenCode
3. Check OpenCode logs for plugin errors
4. Fallback to wrapper scripts (Path B)

### Wrapper Scripts Not Working (Path B)

**Symptom:** "Permission denied" or "command not found"

**Diagnosis:**
```bash
# Check scripts exist
ls -la debugging/wrapper-scripts/

# Check executable bit
file debugging/wrapper-scripts/*.sh
```

**Solutions:**
```bash
# Make executable
chmod +x debugging/wrapper-scripts/*.sh

# Try absolute path
$(pwd)/debugging/wrapper-scripts/run-with-logging.sh ls -la

# Check OpenCode permissions config
```

### Scripts Not Logging

**Symptom:** Scripts run but no log entries created

**Diagnosis:**
```bash
# Check for active log session
cat debugging/current_log_file.txt
# If error: no active log session

# Check log file exists and is writable
LOG_FILE=$(cat debugging/current_log_file.txt)
touch "$LOG_FILE"
```

**Solutions:**
```bash
# Start log session
./debugging/scripts/log-start.sh "troubleshooting"

# Verify active log
cat debugging/current_log_file.txt

# Test logging again
./debugging/scripts/log-decision.sh decision "test"
```

### Permission Errors

**Symptom:** "Permission denied" when running scripts

**Solutions:**
1. Make all scripts executable: `chmod +x debugging/scripts/*.sh`
2. Check OpenCode permissions configuration
3. Try running with absolute paths
4. Verify filesystem permissions (not in read-only directory)

### Cross-Workspace Access Issues

**Symptom:** Architect can't read code agent logs

**Solutions:**
1. Verify path is correct (use absolute paths)
2. Check filesystem permissions (`chmod` if needed)
3. Ensure directories exist before accessing
4. Test with simple `ls` first before complex operations

---

## Next Steps

### For Architect Agents

1. **Write Instructions:** Create instruction file in `instructions/`
2. **Write Human Summary:** Create summary in `human/`
3. **Inform Code Agent:** Notify code agent that workspace is ready
4. **Monitor Logs:** Read `debugging/logs/` files to track progress

### For Code Agents

1. **Read Instructions:** Start with `instructions/` file from architect
2. **Start Log Session:** Run `./debugging/scripts/log-start.sh "<description>"`
3. **Log Decisions:** Use `./debugging/scripts/log-decision.sh` for context
4. **Run Commands:**
   - Plugin: Just execute commands normally
   - Wrapper: Use `./debugging/wrapper-scripts/run-with-logging.sh <command>`
5. **Complete Session:** Run `./debugging/scripts/log-complete.sh` when done

### First Task Checklist

- [ ] Log session started
- [ ] First decision logged manually
- [ ] First command logged automatically (plugin or wrapper)
- [ ] Log file readable and properly formatted
- [ ] Session can be completed successfully

---

## Quick Reference

### Essential Commands

```bash
# Start log session
./debugging/scripts/log-start.sh "task-description"

# Log decision/rationale/investigation
./debugging/scripts/log-decision.sh <type> "message"
# Types: decision, rationale, investigation, verification, deviation, milestone

# Run command with logging (wrapper approach only)
./debugging/wrapper-scripts/run-with-logging.sh <command>

# Complete log session
./debugging/scripts/log-complete.sh

# Check active log
cat debugging/current_log_file.txt

# View current log
cat $(cat debugging/current_log_file.txt)

# List all logs
ls -lt debugging/logs/
```

### File Paths Quick Reference

| Path | Purpose |
|------|---------|
| `.opencode/plugins/logger/` | TypeScript plugin (Path A) |
| `.opencode/opencode.json` | OpenCode configuration |
| `debugging/current_log_file.txt` | Active log session pointer |
| `debugging/logs/` | All log files stored here |
| `debugging/scripts/log-decision.sh` | Manual decision logging |
| `debugging/scripts/log-start.sh` | Start log session |
| `debugging/scripts/log-complete.sh` | Complete log session |
| `debugging/wrapper-scripts/run-with-logging.sh` | Wrapper for commands (Path B) |
| `instructions/` | Architect writes instructions here |
| `human/` | Architect writes human summaries here |

---

## Related Documentation

- [OpenCode Logging Protocol](./opencode_logging_protocol.md) - Complete protocol details
- [OpenCode Migration Guide](./opencode_migration_guide.md) - Migrating from Claude Code
- [Claude vs OpenCode Comparison](./claude_vs_opencode_comparison.md) - Feature comparison
- [Permissions Setup Protocol](./permissions_setup_protocol.md) - Cross-workspace permissions
- [Grading Workflow](./instruction_grading_workflow.md) - How work is graded

---

**Last Updated:** 2025-01-17
**Version:** 1.0
