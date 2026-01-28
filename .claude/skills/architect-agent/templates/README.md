# Workspace Templates - Architect Agent Skill v3.0

**Version:** 3.0 (Hooks Fix + Automation)
**Last Updated:** 2025-11-20

---

## Contents

This directory contains complete workspace templates for the architect-agent workflow:

- **[architect-workspace/](./architect-workspace/)** - Template for architect agent workspace
- **[code-agent-workspace/](./code-agent-workspace/)** - Template for code agent workspace
- **[setup-workspace.sh](./setup-workspace.sh)** - Automated setup script
- **[verify-workspace.sh](./verify-workspace.sh)** - Verification script

---

## Quick Start

### Create New Workspace

```bash
# From this directory
./setup-workspace.sh code-agent ~/projects/my-code-agent

# With architect workspace
./setup-workspace.sh architect ~/projects/my-architect \
    --code-agent-path ~/projects/my-code-agent
```

### Verify Installation

```bash
cd ~/projects/my-code-agent
../verify-workspace.sh
```

---

## Template Structure

### Architect Workspace (14 files)

```
architect-workspace/
├── CLAUDE.md                           # Workspace configuration
├── AGENTS.md                           # Mirror of CLAUDE.md
├── .claude/
│   ├── settings.json                   # Permissions configuration
│   └── commands/
│       ├── project.instruct.md         # /project.instruct command
│       └── project.send.md             # /project.send command
├── instructions/
│   └── README.md                       # Instruction format guide
├── human/
│   └── README.md                       # Human summary format
├── grades/
│   └── README.md                       # Grading rubric
├── ticket/
│   └── current_ticket.md               # Current ticket template
├── analysis/
│   └── README.md                       # Analysis document guide
└── docs/
    ├── hybrid_logging.md               # Logging overview
    ├── workflow.md                     # Architect workflow
    ├── technology_adaptations.md       # Project-specific tech
    └── critical_protocols.md           # Critical protocols
```

### Code Agent Workspace (19 files)

```
code-agent-workspace/
├── CLAUDE.md                           # Workspace configuration
├── AGENTS.md                           # Mirror of CLAUDE.md
├── .claude/
│   ├── settings.json                   # Hook configuration (CRITICAL!)
│   ├── hook-logger.py                  # Enhanced hook logger
│   ├── commands/
│   │   ├── log-start.md                # /log-start command
│   │   ├── log-checkpoint.md           # /log-checkpoint command
│   │   ├── log-complete.md             # /log-complete command
│   │   └── instruct.md                 # /instruct command
│   └── docs/
│       ├── logging_setup.md            # Complete logging guide
│       ├── testing_protocol.md         # Progressive testing
│       └── agent_usage.md              # Agent usage guide
├── debugging/
│   ├── logs/                           # Session log files (empty)
│   ├── instructions/                   # Instructions from architect (empty)
│   ├── scripts/
│   │   ├── log-start.sh                # Start logging session
│   │   ├── log-complete.sh             # Complete session
│   │   ├── log-decision.sh             # Log manual context
│   │   └── get-unstuck.sh              # Unstuck helper
│   └── wrapper-scripts/
│       ├── run-with-logging.sh         # Main wrapper
│       ├── log-tool-call.sh            # Pre-execution logger
│       └── log-tool-result.sh          # Post-execution logger
└── .opencode/
    └── shell-init.sh                   # OpenCode shell functions
```

---

## Setup Script

**Usage:**
```bash
./setup-workspace.sh <workspace-type> <workspace-path> [options]
```

**workspace-type:**
- `architect` - Create architect agent workspace
- `code-agent` - Create code agent workspace

**Options:**
- `--code-agent-path <path>` - Path to code agent (required for architect)
- `--skip-git` - Skip git initialization
- `--force` - Overwrite existing files

**Examples:**
```bash
# Code agent only
./setup-workspace.sh code-agent ~/projects/my-project

# Architect + code agent
./setup-workspace.sh architect ~/projects/my-architect \
    --code-agent-path ~/projects/my-project
```

**What it does:**
1. Creates workspace directory structure
2. Copies all template files
3. Sets executable permissions on scripts
4. Replaces placeholders with actual paths
5. Initializes git repository (optional)

---

## Verify Script

**Usage:**
```bash
# From workspace directory
/path/to/verify-workspace.sh
```

**Checks:**
- ✅ All required files present
- ✅ Correct directory structure
- ✅ Scripts executable
- ✅ **Hooks in settings.json (NOT hooks.json!)**
- ✅ Valid JSON configuration
- ✅ CLAUDE.md and AGENTS.md identical
- ✅ Permissions configured
- ✅ Critical fixes in place (recursion prevention)

**Exit codes:**
- `0` - All checks passed
- `0` - Passed with warnings (review warnings)
- `1` - Failed (fix errors before using)

---

## Critical Features

### ⚠️ Hooks in settings.json

**THE most critical fix in v3.0:**

❌ **WRONG:** `.claude/hooks.json` - Claude Code does NOT read this file
✅ **CORRECT:** `.claude/settings.json` - Claude Code ONLY reads hooks from here

The templates have hooks correctly configured in `settings.json`.

### Enhanced Hook Logger

The v3.0 hook logger captures full arguments for all major tools:

- **Bash:** Command (200 chars) + description
- **Read:** File path + offset/limit
- **Write:** File path + content size
- **Edit:** File path + old/new strings + replace_all flag
- **Grep:** Pattern + path + glob + mode + case_insensitive
- **Glob:** Pattern + path
- **TodoWrite:** Count + status breakdown

### Dual-Mode Logging

Templates support both:
- **Claude Code:** Hook-based automated logging
- **OpenCode:** Wrapper script-based logging

Same result: 60-70% token savings.

---

## Customization Required

### Code Agent CLAUDE.md

**Replace these placeholders:**
- `[PROJECT_NAME]` - Your project name
- `[CLIENT_NAME]` - Client name
- `[TECH_STACK]` - Technology stack
- `[JIRA_PROJECT_KEY]` - JIRA project key
- `[PROJECT_PURPOSE]` - Brief purpose description
- `[BUILD_COMMAND]` - Build command
- `[TEST_COMMAND]` - Test command
- `[CLEAN_COMMAND]` - Clean command
- `[DATE]` - Current date (auto-filled by setup script)

### Architect Workspace

**Update these paths:**
- `.claude/settings.json` - Code agent instructions path
- `CLAUDE.md` - Code agent workspace path
- `docs/technology_adaptations.md` - Project-specific details

**Paths must be absolute (not relative).**

---

## Token Savings

**Without hooks (manual logging):**
```
I'm going to run: ls -la
<output shown>
The output shows...
```
~100 tokens per command × 30 commands = **3,000 tokens**

**With hooks (automated logging):**
```
<just decisions and analysis>
```
~25 tokens per command × 30 commands = **750 tokens**

**Savings: 2,250 tokens per session (75% reduction)**

---

## Documentation

**See parent directory for complete guides:**
- [../references/installation.md](../references/installation.md) - Installation guide
- [../references/upgrade.md](../references/upgrade.md) - Upgrade guide from v1.0/v2.0
- [../references/quick_start.md](../references/quick_start.md) - Quick start (5/10 min)
- [../references/workspace_setup_complete.md](../references/workspace_setup_complete.md) - Complete setup checklist
- [../references/hook_configuration_critical.md](../references/hook_configuration_critical.md) - Critical hooks fix

---

## Support

**If something doesn't work:**

1. Run verification script: `./verify-workspace.sh`
2. Check verification output for specific errors
3. Review troubleshooting in references/installation.md or references/upgrade.md
4. Consult reference documentation

**Most common issue:** Hooks in hooks.json instead of settings.json

---

**Last Updated:** 2025-11-20
**Version:** 3.0 (Hooks Fix + Automation)
