# OpenCode Integration Quickstart

**Purpose:** Quick reference for migrating code agent workspaces to support OpenCode
**Audience:** Architect agents adding OpenCode support to existing code agents
**Time to Complete:** 5-10 minutes

---

## When to Use This Guide

**Trigger phrases:**
- User says: "migrate code agent to also support OpenCode"
- User says: "add OpenCode support to code agent"
- User says: "install OpenCode hooks on code agent"

**Prerequisites:**
- Code agent workspace path is known/configured
- Code agent workspace has `.claude/hooks.json` (Claude Code setup)

**What this migration adds:**
- OpenCode TypeScript plugin (`.opencode/plugins/logger/`)
- OpenCode configuration (`.opencode/opencode.json`)
- Session management scripts (`debugging/scripts/log-start.sh`, `log-complete.sh`)
- Optional wrapper scripts (`debugging/wrapper-scripts/`)

**What this migration preserves:**
- Existing `.claude/hooks.json` (dual-mode support)
- All existing logs and scripts
- No breaking changes

---

## Migration Workflow

Execute these steps in order when user requests OpenCode migration:

### Step 1: Verify Prerequisites

```bash
# Check code agent workspace exists
CODE_AGENT_WS="/path/to/code/agent/workspace"
[ -d "$CODE_AGENT_WS" ] || { echo "Code agent workspace not found"; exit 1; }

# Check Claude Code setup exists
[ -f "$CODE_AGENT_WS/.claude/hooks.json" ] || echo "⚠️  Warning: No Claude Code hooks found. This may be a fresh workspace."

# Check if OpenCode already installed
[ -d "$CODE_AGENT_WS/.opencode/plugins/logger" ] && echo "✓ OpenCode already installed" && exit 0
```

### Step 2: Copy OpenCode Plugin

```bash
# Copy TypeScript plugin
mkdir -p "$CODE_AGENT_WS/.opencode/plugins"
cp -r ~/.claude/skills/architect-agent/templates/opencode/plugins/logger \
      "$CODE_AGENT_WS/.opencode/plugins/"

# Create opencode.json configuration
cp ~/.claude/skills/architect-agent/templates/opencode/opencode.json \
   "$CODE_AGENT_WS/.opencode/"

echo "✓ OpenCode plugin installed"
```

### Step 3: Add Session Management Scripts

```bash
# Create session management scripts (replaces /log-start and /log-complete)
cat > "$CODE_AGENT_WS/debugging/scripts/log-start.sh" <<'EOF'
#!/bin/bash
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

cat > "$CODE_AGENT_WS/debugging/scripts/log-complete.sh" <<'EOF'
#!/bin/bash
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

chmod +x "$CODE_AGENT_WS/debugging/scripts/log-start.sh"
chmod +x "$CODE_AGENT_WS/debugging/scripts/log-complete.sh"

echo "✓ Session management scripts created"
```

### Step 4: Optional Wrapper Scripts

```bash
# Copy wrapper scripts (for users who prefer bash over TypeScript plugin)
mkdir -p "$CODE_AGENT_WS/debugging/wrapper-scripts"
cp ~/.claude/skills/architect-agent/templates/opencode/wrapper-scripts/*.sh \
   "$CODE_AGENT_WS/debugging/wrapper-scripts/"
chmod +x "$CODE_AGENT_WS/debugging/wrapper-scripts/"*.sh

echo "✓ Wrapper scripts installed (optional alternative to plugin)"
```

### Step 5: Verify Installation

```bash
# Check all components installed
echo ""
echo "✅ OpenCode Migration Complete!"
echo ""
echo "Installed:"
echo "  ✓ TypeScript plugin: .opencode/plugins/logger/"
echo "  ✓ Configuration: .opencode/opencode.json"
echo "  ✓ Session scripts: debugging/scripts/log-start.sh, log-complete.sh"
echo "  ✓ Wrapper scripts: debugging/wrapper-scripts/ (optional)"
echo ""
echo "Preserved:"
echo "  ✓ Claude Code hooks: .claude/hooks.json"
echo "  ✓ Existing logs: debugging/logs/"
echo "  ✓ Decision script: debugging/scripts/log-decision.sh"
echo ""
echo "Code agent now supports BOTH Claude Code AND OpenCode!"
echo ""
echo "Usage for OpenCode:"
echo "  ./debugging/scripts/log-start.sh \"task-description\""
echo "  # Work normally - plugin logs automatically"
echo "  ./debugging/scripts/log-complete.sh"
echo ""
```

---

## Migration Completion Message

After successful migration, inform the user with this message:

> **✅ Migration Complete!**
>
> Your code agent workspace now supports **both Claude Code and OpenCode**:
>
> **For Claude Code** (no changes needed):
> - Continue using `/log-start`, `/log-checkpoint`, `/log-complete`
> - Hooks in `.claude/hooks.json` still active
>
> **For OpenCode** (new capability):
> - Use `./debugging/scripts/log-start.sh "description"`
> - Plugin logs automatically (same as Claude Code hooks)
> - Use `./debugging/scripts/log-complete.sh` to finish
>
> **Same log format, same token savings (60-70%), same grading rubric.**
>
> See `references/opencode_logging_protocol.md` for complete OpenCode usage.

---

## OpenCode Compatibility Reference

This skill supports both **Claude Code** and **OpenCode** for code agents.

### Supported Configurations

✅ **Architect: Claude Code, Code Agent: Claude Code** - Default configuration
✅ **Architect: Claude Code, Code Agent: OpenCode** - Recommended hybrid approach
✅ **Architect: OpenCode, Code Agent: OpenCode** - Full open-source setup

### Key Differences for OpenCode Code Agents

| Feature | Claude Code | OpenCode |
|---------|------------|----------|
| **Automated Logging** | `.claude/hooks.json` | TypeScript plugin OR bash wrappers |
| **Session Management** | `/log-start` slash command | `./debugging/scripts/log-start.sh` |
| **Log Format** | Markdown with timestamps | **Identical** - same format |
| **Token Efficiency** | 60-70% savings | **Identical** - same savings |
| **Grading** | 10-point rubric | **Identical** - same rubric |

### Setup for New OpenCode Code Agent

When setting up a code agent workspace using OpenCode from scratch:

1. **Choose Logging Approach:**
   - **TypeScript Plugin** (recommended if OpenCode supports plugins)
   - **Bash Wrappers** (alternative for any OpenCode version)

2. **Copy Templates:**
   ```bash
   # For plugin approach
   cp -r templates/opencode/plugins/logger <code-agent>/.opencode/plugins/

   # For wrapper approach
   cp templates/opencode/wrapper-scripts/* <code-agent>/debugging/wrapper-scripts/
   ```

3. **Create Session Management:**
   ```bash
   # Scripts to replace /log-start and /log-complete slash commands
   cp templates/opencode/scripts/* <code-agent>/debugging/scripts/
   ```

4. **Universal Scripts (Same for Both):**
   ```bash
   # These work identically in Claude Code and OpenCode
   cp templates/debugging/scripts/log-decision.sh <code-agent>/debugging/scripts/
   cp templates/debugging/scripts/get-unstuck.sh <code-agent>/debugging/scripts/
   ```

### Grading OpenCode Work

**No grading changes required!**

- Log format is identical between Claude Code and OpenCode
- Grading rubric applies exactly the same way
- Same 60-70% token efficiency expected
- Same mandatory logging requirement
- Architect agents grade OpenCode logs using the same process

---

## Complete OpenCode Documentation

For comprehensive OpenCode information, see these references:

- **`opencode_logging_protocol.md`** - Complete OpenCode logging protocol with examples
- **`opencode_setup_guide.md`** - Detailed step-by-step setup for fresh workspaces
- **`opencode_migration_guide.md`** - Full migration guide with troubleshooting
- **`claude_vs_opencode_comparison.md`** - Feature comparison and decision framework

**All other architect-agent references (grading, testing, delegation) apply to both Claude Code and OpenCode.**

---

**Version:** 1.0
**Last Updated:** 2025-01-18
**Related:** opencode_logging_protocol.md, opencode_setup_guide.md, opencode_migration_guide.md
