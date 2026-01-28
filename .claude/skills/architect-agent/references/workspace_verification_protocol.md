# Workspace Verification Protocol

**Purpose:** Complete setup and verification procedure for code agent workspaces supporting both Claude Code and OpenCode.

**When to Use:**
- Setting up a new code agent workspace
- Verifying existing setup is working
- Troubleshooting logging issues
- After upgrading or modifying hooks/plugins

---

## Quick Verification Commands

### Check If Setup Complete

```bash
CODE_AGENT_DIR="/path/to/code-agent"

# Claude Code setup
ls -la "$CODE_AGENT_DIR/.claude/hook-logger.py"
ls -la "$CODE_AGENT_DIR/.claude/settings.json"

# OpenCode setup
ls -la "$CODE_AGENT_DIR/.opencode/plugin/logger.js"
ls -la "$CODE_AGENT_DIR/.opencode/opencode.json"

# Logging scripts
ls -la "$CODE_AGENT_DIR/debugging/scripts/log-start.sh"
ls -la "$CODE_AGENT_DIR/debugging/scripts/log-complete.sh"
```

### Quick Test (Both Tools)

```bash
cd "$CODE_AGENT_DIR"

# 1. Start log session
./debugging/scripts/log-start.sh "verification-test"

# 2. Test Claude Code hook
claude -p "echo 'Hook test'" 2>&1

# 3. Check log for entries
grep "TOOL:" debugging/logs/log-*-verification-test.md

# 4. Complete log session
./debugging/scripts/log-complete.sh
```

---

## Complete Setup Procedure

### Phase 1: Directory Structure

Create required directories in code agent workspace:

```bash
CODE_AGENT_DIR="/path/to/code-agent"
cd "$CODE_AGENT_DIR"

# Create .claude directory
mkdir -p .claude/commands
mkdir -p .claude/docs

# Create .opencode directory
mkdir -p .opencode/plugin

# Create debugging directories
mkdir -p debugging/logs
mkdir -p debugging/scripts
mkdir -p debugging/instructions
mkdir -p debugging/wrapper-scripts
```

### Phase 2: Install Claude Code Hooks

#### 2a. Create hook-logger.py

```bash
cat > .claude/hook-logger.py << 'PYTHONEOF'
#!/usr/bin/env python3
"""
Hook logger for Claude Code - captures tool calls automatically.
Reads from stdin (JSON), writes to active log file.
"""
import json
import os
import sys
from datetime import datetime
from pathlib import Path

def get_log_file():
    """Get active log file path from current_log_file.txt"""
    current_log_marker = Path("debugging/current_log_file.txt")
    if not current_log_marker.exists():
        return None
    log_path = current_log_marker.read_text().strip()
    if not log_path or not Path(log_path).exists():
        return None
    return log_path

def format_tool_entry(data):
    """Format tool data for log entry"""
    timestamp = datetime.now().strftime("%H:%M:%S")
    tool_name = data.get("tool_name", "Unknown")
    tool_input = data.get("tool_input", {})

    entry = f"\n[{timestamp}] TOOL: {tool_name}\n"

    if tool_name == "Bash":
        cmd = tool_input.get("command", "")
        desc = tool_input.get("description", "")
        entry += f"COMMAND: {cmd[:200]}\n"
        if desc:
            entry += f"DESC: {desc}\n"
    elif tool_name == "Read":
        entry += f"FILE: {tool_input.get('file_path', '')}\n"
    elif tool_name == "Write":
        path = tool_input.get("file_path", "")
        content = tool_input.get("content", "")
        entry += f"FILE: {path}\n"
        entry += f"SIZE: {len(content)} chars\n"
    elif tool_name == "Edit":
        entry += f"FILE: {tool_input.get('file_path', '')}\n"
        entry += f"REPLACE_ALL: {tool_input.get('replace_all', False)}\n"
    elif tool_name == "Grep":
        entry += f"PATTERN: {tool_input.get('pattern', '')}\n"
        entry += f"PATH: {tool_input.get('path', '.')}\n"
    elif tool_name == "Glob":
        entry += f"PATTERN: {tool_input.get('pattern', '')}\n"
    elif tool_name == "TodoWrite":
        todos = tool_input.get("todos", [])
        entry += f"ITEMS: {len(todos)}\n"
    else:
        # Generic handling for other tools
        for key, value in list(tool_input.items())[:3]:
            entry += f"{key.upper()}: {str(value)[:100]}\n"

    entry += "---\n"
    return entry

def main():
    log_file = get_log_file()
    if not log_file:
        sys.exit(0)  # No active session, silently exit

    try:
        input_data = sys.stdin.read()
        if not input_data.strip():
            sys.exit(0)

        data = json.loads(input_data)

        # Only log PostToolUse events with tool data
        if data.get("event") == "PostToolUse" and "tool_name" in data:
            entry = format_tool_entry(data)
            with open(log_file, "a") as f:
                f.write(entry)
    except Exception:
        sys.exit(0)  # Fail silently

if __name__ == "__main__":
    main()
PYTHONEOF

chmod +x .claude/hook-logger.py
```

#### 2b. Create settings.json with hooks

```bash
cat > .claude/settings.json << 'JSONEOF'
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
    ],
    "PreToolUse": [
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
    ],
    "UserPromptSubmit": [
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
    ],
    "SessionStart": [
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
    ],
    "Stop": [
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
JSONEOF
```

### Phase 3: Install OpenCode Plugin

#### 3a. Create logger.js plugin

```bash
cat > .opencode/plugin/logger.js << 'JSEOF'
/**
 * OpenCode Logger Plugin - Captures tool calls automatically
 * SPEC.md v4.1 compliant
 */
const fs = require('fs');
const path = require('path');

function getLogFile() {
  const markerPath = path.join(process.cwd(), 'debugging', 'current_log_file.txt');
  try {
    if (!fs.existsSync(markerPath)) return null;
    const logPath = fs.readFileSync(markerPath, 'utf8').trim();
    if (!logPath || !fs.existsSync(logPath)) return null;
    return logPath;
  } catch {
    return null;
  }
}

function log(message) {
  const logFile = getLogFile();
  if (!logFile) return;
  const timestamp = new Date().toLocaleTimeString('en-US', { hour12: false });
  fs.appendFileSync(logFile, `\n[${timestamp}] ${message}\n---\n`);
}

module.exports = {
  name: 'logger',
  version: '1.0.0',

  'tool.execute.before': (ctx) => {
    const logFile = getLogFile();
    if (logFile) {
      log(`TOOL: ${ctx.tool?.name || 'unknown'} (starting)`);
    }
  },

  'tool.execute.after': (ctx) => {
    const logFile = getLogFile();
    if (logFile) {
      const toolName = ctx.tool?.name || 'unknown';
      const args = ctx.tool?.args || {};
      let details = `TOOL: ${toolName}`;

      if (toolName === 'bash' || toolName === 'shell') {
        details += `\nCOMMAND: ${(args.command || '').slice(0, 200)}`;
      } else if (toolName === 'read' || toolName === 'write') {
        details += `\nFILE: ${args.path || args.file_path || ''}`;
      } else if (toolName === 'edit') {
        details += `\nFILE: ${args.path || args.file_path || ''}`;
      }

      log(details);
    }
  },

  'session.start': () => log('SESSION: Started'),
  'session.end': () => log('SESSION: Ended'),
  'message.user': (ctx) => log(`USER: ${(ctx.message?.content || '').slice(0, 100)}...`),
  'message.assistant': () => log('ASSISTANT: Response generated'),
  'error': (ctx) => log(`ERROR: ${ctx.error?.message || 'Unknown error'}`)
};
JSEOF
```

#### 3b. Create opencode.json configuration

```bash
cat > .opencode/opencode.json << 'JSONEOF'
{
  "plugins": [
    "./plugin/logger.js"
  ]
}
JSONEOF
```

#### 3c. Create package.json for plugin

```bash
cat > .opencode/package.json << 'JSONEOF'
{
  "name": "opencode-plugins",
  "version": "1.0.0",
  "description": "OpenCode plugins for logging",
  "private": true
}
JSONEOF
```

### Phase 4: Install Logging Scripts

#### 4a. log-start.sh

```bash
cat > debugging/scripts/log-start.sh << 'BASHEOF'
#!/bin/bash
set -euo pipefail

DESCRIPTION="${1:-session}"
TIMESTAMP=$(date +%Y_%m_%d-%H_%M)
LOG_FILE="debugging/logs/log-${TIMESTAMP}-${DESCRIPTION}.md"

mkdir -p debugging/logs

cat > "$LOG_FILE" << LOGEOF
# Log Session: $DESCRIPTION

**Started:** $(date '+%Y-%m-%d %H:%M:%S')
**Agent:** Code Agent

## Goal
[Document your goal here]

## Success Criteria
- [ ] Criterion 1
- [ ] Criterion 2

---

## Execution Log

LOGEOF

echo "$LOG_FILE" > debugging/current_log_file.txt
echo "Log session started: $LOG_FILE"
BASHEOF

chmod +x debugging/scripts/log-start.sh
```

#### 4b. log-complete.sh

```bash
cat > debugging/scripts/log-complete.sh << 'BASHEOF'
#!/bin/bash
set -euo pipefail

if [ ! -f debugging/current_log_file.txt ]; then
    echo "Error: No active log session"
    exit 1
fi

LOG_FILE=$(cat debugging/current_log_file.txt)
TIMESTAMP="[$(date +%H:%M:%S)]"

cat >> "$LOG_FILE" << LOGEOF

---
$TIMESTAMP FINAL SUMMARY
**Status:** COMPLETE
**Completed:** $(date '+%Y-%m-%d %H:%M:%S')
---
LOGEOF

echo "Log session completed: $LOG_FILE"
rm debugging/current_log_file.txt
BASHEOF

chmod +x debugging/scripts/log-complete.sh
```

#### 4c. log-decision.sh

```bash
cat > debugging/scripts/log-decision.sh << 'BASHEOF'
#!/bin/bash
set -euo pipefail

TYPE="${1:-note}"
MESSAGE="${2:-No message provided}"

if [ ! -f debugging/current_log_file.txt ]; then
    echo "Error: No active log session"
    exit 1
fi

LOG_FILE=$(cat debugging/current_log_file.txt)
TIMESTAMP="[$(date +%H:%M:%S)]"

echo "" >> "$LOG_FILE"
echo "$TIMESTAMP ${TYPE^^}: $MESSAGE" >> "$LOG_FILE"
echo "---" >> "$LOG_FILE"

echo "Logged $TYPE to: $LOG_FILE"
BASHEOF

chmod +x debugging/scripts/log-decision.sh
```

### Phase 5: Create LOGGING.md Protocol

```bash
cat > .claude/LOGGING.md << 'MDEOF'
# Logging Protocol - Hybrid Logging v2.0

**Status:** Installed and Configured
**Token Savings:** 60-70%

## Quick Start

### Starting a Session

```bash
./debugging/scripts/log-start.sh "task-description"
```

### Working Normally

**Claude Code:** Hooks automatically capture tool calls
**OpenCode:** Plugin automatically captures tool events

### Adding Manual Context

```bash
./debugging/scripts/log-decision.sh decision "Chose approach X because Y"
./debugging/scripts/log-decision.sh investigation "Root cause found"
./debugging/scripts/log-decision.sh observation "Important pattern noticed"
```

### Completing Session

```bash
./debugging/scripts/log-complete.sh
```

## What Gets Logged Automatically

- Bash commands with descriptions
- File reads (path, offset, limit)
- File writes (path, content size)
- File edits (path, changes)
- Search operations (pattern, path)
- Todo updates (item count, status)

## Grading Impact

- 10 points for logging quality
- Hooks must capture all tool calls
- Manual context for decisions/investigations

## Verification

```bash
# Start test session
./debugging/scripts/log-start.sh "hook-test"

# Run some commands (Claude Code will capture these)
ls -la
pwd

# Check log entries
grep "TOOL:" debugging/logs/log-*-hook-test.md

# Complete session
./debugging/scripts/log-complete.sh
```
MDEOF
```

---

## Permissions Configuration

### Code Agent Permissions

Create `.claude/settings.local.json` in code agent workspace:

```json
{
  "permissions": {
    "allow": [
      "Bash(./debugging/scripts/log-start.sh:*)",
      "Bash(./debugging/scripts/log-complete.sh:*)",
      "Bash(./debugging/scripts/log-decision.sh:*)",
      "Bash(./debugging/scripts/get-unstuck.sh:*)",
      "Bash(debugging/scripts/log-start.sh:*)",
      "Bash(debugging/scripts/log-complete.sh:*)",
      "Bash(debugging/scripts/log-decision.sh:*)",
      "Bash(task:*)",
      "Bash(./gradlew:*)",
      "Bash(gradle:*)",
      "Bash(git status:*)",
      "Bash(git add:*)",
      "Bash(git commit:*)",
      "Bash(git diff:*)",
      "Bash(git log:*)",
      "Write(debugging/**)",
      "Read(debugging/**)",
      "Read(.claude/**)",
      "Read(//Users/<username>/.claude/skills/architect-agent/references/**)"
    ]
  }
}
```

### Architect Agent Permissions

Add to architect agent's `.claude/settings.local.json`:

```json
{
  "permissions": {
    "allow": [
      "Read(//<code-agent-path>/debugging/**)",
      "Read(//<code-agent-path>/.claude/**)",
      "Write(//<code-agent-path>/debugging/instructions/**)",
      "Bash(cd //<code-agent-path> && claude:*)",
      "Bash(cd //<code-agent-path> && opencode:*)"
    ]
  }
}
```

---

## Verification Procedure

### Step 1: Verify Files Exist

```bash
CODE_AGENT_DIR="/path/to/code-agent"
cd "$CODE_AGENT_DIR"

echo "=== Claude Code Files ==="
[ -f ".claude/hook-logger.py" ] && echo "hook-logger.py" || echo "MISSING: hook-logger.py"
[ -f ".claude/settings.json" ] && echo "settings.json" || echo "MISSING: settings.json"
[ -x ".claude/hook-logger.py" ] && echo "hook-logger.py is executable" || echo "WARNING: hook-logger.py not executable"

echo ""
echo "=== OpenCode Files ==="
[ -f ".opencode/plugin/logger.js" ] && echo "logger.js" || echo "MISSING: logger.js"
[ -f ".opencode/opencode.json" ] && echo "opencode.json" || echo "MISSING: opencode.json"

echo ""
echo "=== Logging Scripts ==="
[ -x "debugging/scripts/log-start.sh" ] && echo "log-start.sh" || echo "MISSING: log-start.sh"
[ -x "debugging/scripts/log-complete.sh" ] && echo "log-complete.sh" || echo "MISSING: log-complete.sh"
[ -x "debugging/scripts/log-decision.sh" ] && echo "log-decision.sh" || echo "MISSING: log-decision.sh"
```

### Step 2: Verify Hook Configuration

```bash
echo "=== Checking settings.json for hooks ==="
grep -q "PostToolUse" .claude/settings.json && echo "PostToolUse hook configured" || echo "MISSING: PostToolUse hook"
grep -q "hook-logger.py" .claude/settings.json && echo "hook-logger.py referenced" || echo "MISSING: hook-logger.py reference"

echo ""
echo "=== Checking opencode.json for plugins ==="
grep -q "logger.js" .opencode/opencode.json && echo "logger.js plugin configured" || echo "MISSING: logger.js plugin"
```

### Step 3: Test Claude Code Hook

```bash
cd "$CODE_AGENT_DIR"

# Start log session
./debugging/scripts/log-start.sh "claude-hook-test"

# Verify session started
[ -f "debugging/current_log_file.txt" ] && echo "Log session active" || echo "ERROR: No active session"

# Run Claude Code with a simple task
claude -p "Run: echo 'Claude Code hook test'"

# Wait for hook to process
sleep 2

# Check for TOOL entries
LOG_FILE=$(cat debugging/current_log_file.txt)
if grep -q "TOOL:" "$LOG_FILE"; then
    echo "SUCCESS: Claude Code hook is capturing tool calls"
    grep "TOOL:" "$LOG_FILE" | head -5
else
    echo "FAILURE: No TOOL entries found in log"
fi

# Complete session
./debugging/scripts/log-complete.sh
```

### Step 4: Test OpenCode Plugin (if installed)

```bash
cd "$CODE_AGENT_DIR"

# Start log session
./debugging/scripts/log-start.sh "opencode-plugin-test"

# Run OpenCode with a simple task
opencode run "Run: echo 'OpenCode plugin test'"

# Wait for plugin to process
sleep 2

# Check for TOOL entries
LOG_FILE=$(cat debugging/current_log_file.txt)
if grep -q "TOOL:" "$LOG_FILE"; then
    echo "SUCCESS: OpenCode plugin is capturing tool calls"
    grep "TOOL:" "$LOG_FILE" | head -5
else
    echo "WARNING: No TOOL entries found (OpenCode may not be installed)"
fi

# Complete session
./debugging/scripts/log-complete.sh
```

### Step 5: Full Integration Test

```bash
cd "$CODE_AGENT_DIR"

echo "=== Full Integration Test ==="

# Start session
./debugging/scripts/log-start.sh "integration-test"

# Log a manual decision
./debugging/scripts/log-decision.sh decision "Testing full integration workflow"

# Run Claude Code task (this should trigger hooks)
claude -p "Read debugging/current_log_file.txt and list the first 10 lines"

# Check results
LOG_FILE=$(cat debugging/current_log_file.txt)
echo ""
echo "=== Log Contents ==="
cat "$LOG_FILE"

# Verify entries
echo ""
echo "=== Verification ==="
DECISION_COUNT=$(grep -c "DECISION:" "$LOG_FILE" || echo "0")
TOOL_COUNT=$(grep -c "TOOL:" "$LOG_FILE" || echo "0")

echo "Manual entries (DECISION): $DECISION_COUNT"
echo "Automated entries (TOOL): $TOOL_COUNT"

if [ "$DECISION_COUNT" -gt 0 ] && [ "$TOOL_COUNT" -gt 0 ]; then
    echo ""
    echo "SUCCESS: Both manual and automated logging working!"
else
    echo ""
    echo "WARNING: Missing entries - check hook configuration"
fi

# Complete session
./debugging/scripts/log-complete.sh
```

---

## Troubleshooting

### Hook Not Capturing

**Symptom:** No TOOL entries in log file

**Checks:**
1. Is there an active log session?
   ```bash
   cat debugging/current_log_file.txt
   ```

2. Is hook-logger.py executable?
   ```bash
   chmod +x .claude/hook-logger.py
   ```

3. Is settings.json valid JSON?
   ```bash
   python3 -m json.tool .claude/settings.json
   ```

4. Is hook using correct path?
   ```bash
   grep "command" .claude/settings.json
   # Should show: $CLAUDE_PROJECT_DIR/.claude/hook-logger.py
   ```

### OpenCode Plugin Not Working

**Symptom:** No entries from OpenCode

**Checks:**
1. Is plugin path correct in opencode.json?
   ```bash
   cat .opencode/opencode.json
   ```

2. Is logger.js valid JavaScript?
   ```bash
   node -c .opencode/plugin/logger.js
   ```

3. Does OpenCode support plugins in your version?

### Log Session Not Starting

**Symptom:** "No active log session" errors

**Fix:**
```bash
./debugging/scripts/log-start.sh "your-task-name"
```

### Permission Denied

**Symptom:** Scripts fail with permission errors

**Fix:**
```bash
chmod +x debugging/scripts/*.sh
chmod +x .claude/hook-logger.py
```

---

## Summary

| Component | Location | Purpose |
|-----------|----------|---------|
| hook-logger.py | .claude/ | Claude Code tool capture |
| settings.json | .claude/ | Hook configuration |
| logger.js | .opencode/plugin/ | OpenCode tool capture |
| opencode.json | .opencode/ | Plugin configuration |
| log-start.sh | debugging/scripts/ | Start log session |
| log-complete.sh | debugging/scripts/ | Complete log session |
| log-decision.sh | debugging/scripts/ | Log manual entries |
| current_log_file.txt | debugging/ | Active session marker |

**Key Requirement:** Both hooks require an active log session. Always run `log-start.sh` before testing!

---

**Version:** 1.0
**Last Updated:** 2025-11-25
**Related:** logging_protocol.md, hybrid_logging_protocol.md, opencode_integration_quickstart.md
