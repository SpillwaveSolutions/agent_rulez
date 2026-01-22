# Installation Guide - Architect Agent Skill v3.0

**Version:** 3.0 (Workspace Setup with Hooks Fix)
**Last Updated:** 2025-11-20

---

## What's New in v3.0

✅ **THE FIX:** Hooks MUST be in `.claude/settings.json` (NOT hooks.json!)
✅ **Automated Setup:** `setup-workspace.sh` script for instant workspace creation
✅ **Verification:** `verify-workspace.sh` script to validate installation
✅ **Complete Templates:** Ready-to-use architect and code agent workspaces
✅ **Enhanced Hook Logger:** Full argument capture for all major tools
✅ **OpenCode Support:** Wrapper scripts for dual-mode logging

---

## Quick Install (Recommended)

### Create New Workspace

```bash
# Navigate to templates
cd ~/.claude/skills/architect-agent/templates/

# Create code agent workspace
./setup-workspace.sh code-agent ~/projects/my-code-agent

# Create architect workspace (optional)
./setup-workspace.sh architect ~/projects/my-architect \
    --code-agent-path ~/projects/my-code-agent

# Verify installation
cd ~/projects/my-code-agent
~/projects/my-architect/verify-workspace.sh
```

**Done!** See [Quick Start Guide](./references/quick_start.md) for next steps.

---

## What Gets Installed

### Code Agent Workspace

**Core Configuration:**
- `CLAUDE.md` / `AGENTS.md` - Workspace configuration
- `.claude/settings.json` - **Hook configuration (CRITICAL!)**
- `.claude/hook-logger.py` - Enhanced hook logger

**Logging System:**
- `debugging/logs/` - Session log files
- `debugging/scripts/` - Logging control scripts
- `debugging/wrapper-scripts/` - OpenCode wrapper scripts
- `.opencode/shell-init.sh` - OpenCode shell functions

**Documentation:**
- `.claude/docs/logging_setup.md` - Complete logging guide
- `.claude/docs/testing_protocol.md` - Progressive testing
- `.claude/docs/agent_usage.md` - Agent usage guide

**Slash Commands:**
- `/log-start` - Start logging session
- `/log-checkpoint` - Create checkpoint
- `/log-complete` - Complete session
- `/instruct` - Review instructions

### Architect Workspace

**Core Configuration:**
- `CLAUDE.md` / `AGENTS.md` - Workspace configuration
- `.claude/settings.json` - Cross-workspace permissions

**Workspace Directories:**
- `instructions/` - Instruction files for code agent
- `human/` - Human summaries (10-25 bullets)
- `grades/` - Grading reports
- `ticket/` - Current ticket tracking
- `analysis/` - Investigation documents
- `docs/` - Architect documentation

**Slash Commands:**
- `/project.instruct` - Review instruction summary
- `/project.send` - Send instructions to code agent

---

## Verification

```bash
# From workspace directory
~/.claude/skills/architect-agent/templates/verify-workspace.sh
```

**Checks:**
- ✅ All required files present
- ✅ Scripts executable
- ✅ Hooks in settings.json (NOT hooks.json!)
- ✅ Valid JSON configuration
- ✅ CLAUDE.md and AGENTS.md identical
- ✅ Permissions configured correctly

---

## Troubleshooting

### Hooks Not Working

**Check hooks in correct file:**
```bash
grep "PostToolUse" .claude/settings.json
ls .claude/hooks.json 2>&1 | grep "No such file"
```

**See:** [hook_configuration_critical.md](./references/hook_configuration_critical.md)

### Permission Issues

**Check architect permissions:**
```bash
grep "debugging/instructions" .claude/settings.json
```

Must include absolute path to code agent's instructions directory.

---

## Next Steps

**See:** [Quick Start Guide](./references/quick_start.md)

---

**Last Updated:** 2025-11-20
**Version:** 3.0
