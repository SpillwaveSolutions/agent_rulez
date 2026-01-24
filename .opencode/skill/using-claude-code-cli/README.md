# Using Claude Code CLI

[![Agent Skill Standard](https://img.shields.io/badge/Agent%20Skill-Standard-blue)](https://agentskills.io/)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Skilz Compatible](https://img.shields.io/badge/skilz-compatible-green)](https://github.com/SpillwaveSolutions/skilz-cli)

A Claude Code skill for programmatically invoking Claude Code CLI from Python orchestrators, shell scripts, and automation pipelines.

## Overview

This skill provides comprehensive patterns for:

- **Tool Pre-Approval** - Eliminate permission prompts with `--allowedTools`
- **Directory Access** - Grant Claude access to additional directories with `--add-dir`
- **Hooks Configuration** - Set up logging, monitoring, and blocking hooks
- **Sandbox Mode** - Safe execution for untrusted operations
- **Python Patterns** - Synchronous, async, and parallel subprocess invocation
- **JSON Extraction** - Parse structured output from CLI responses
- **Fallback Strategies** - OpenCode CLI as backup

## Quick Start

```bash
# Minimal automation command
claude -p "Your prompt" --allowedTools Write Read Edit

# Full automation with all options
claude \
  --model sonnet \
  --add-dir /path/to/skills \
  --allowedTools Write Read Edit Bash Task \
  --output-format json \
  -p "Your automation prompt"
```

## Installation

### Using Skilz Universal Installer (Recommended)

The recommended way to install this skill across different AI coding agents is using the [skilz](https://github.com/SpillwaveSolutions/skilz-cli) universal installer.

This skill supports the [Agent Skill Standard](https://agentskills.io/) which means it works with 14+ coding agents including Claude Code, OpenAI Codex, Cursor, and Gemini CLI.

#### Install Skilz

```bash
pip install skilz
```

#### Install from SkillzWave Marketplace

The easiest way to install is from the [SkillzWave Marketplace](https://skillzwave.ai/skill/SpillwaveSolutions__using-claude-code-cli-agent-skill__using-claude-code-cli__SKILL/):

```bash
# Claude Code (user-level - available in all projects)
skilz install SpillwaveSolutions_using-claude-code-cli-agent-skill/using-claude-code-cli

# Claude Code (project-level)
skilz install SpillwaveSolutions_using-claude-code-cli-agent-skill/using-claude-code-cli --project

# OpenCode
skilz install SpillwaveSolutions_using-claude-code-cli-agent-skill/using-claude-code-cli --agent opencode

# OpenAI Codex
skilz install SpillwaveSolutions_using-claude-code-cli-agent-skill/using-claude-code-cli --agent codex

# Gemini CLI (project-level only)
skilz install SpillwaveSolutions_using-claude-code-cli-agent-skill/using-claude-code-cli --agent gemini
```

#### Install from GitHub

You can also install directly from GitHub using HTTPS or SSH:

```bash
# HTTPS
skilz install -g https://github.com/SpillwaveSolutions/using-claude-code-cli-agent-skill

# SSH
skilz install --git git@github.com:SpillwaveSolutions/using-claude-code-cli-agent-skill.git
```

**Agent-specific GitHub installs:**

```bash
# Claude Code (user-level)
skilz install -g https://github.com/SpillwaveSolutions/using-claude-code-cli-agent-skill

# Claude Code (project-level)
skilz install -g https://github.com/SpillwaveSolutions/using-claude-code-cli-agent-skill --project

# OpenCode
skilz install -g https://github.com/SpillwaveSolutions/using-claude-code-cli-agent-skill --agent opencode

# OpenAI Codex
skilz install -g https://github.com/SpillwaveSolutions/using-claude-code-cli-agent-skill --agent codex

# Gemini CLI
skilz install -g https://github.com/SpillwaveSolutions/using-claude-code-cli-agent-skill --agent gemini
```

#### Installation Locations

| Agent | User-Level Location | Project-Level Location |
|-------|---------------------|------------------------|
| Claude Code | `~/.claude/skills/` | `./.claude/skills/` |
| OpenCode | `~/.config/opencode/skills/` | `./.opencode/skills/` |
| OpenAI Codex | `~/.codex/skills/` | `./.codex/skills/` |
| Gemini CLI | N/A | `./GEMINI.md` (inline) |

#### Other Supported Agents

Skilz supports 14+ coding agents including Windsurf, Qwen Code, Aidr, Cursor, and more.

For the full list of supported platforms, visit:
- [SkillzWave Platforms](https://skillzwave.ai/platforms/)
- [skilz-cli GitHub Repository](https://github.com/SpillwaveSolutions/skilz-cli)

## File Structure

```
using-claude-code-cli/
├── SKILL.md                           # Main skill document
├── README.md                          # This file
├── .gitignore                         # Git ignore patterns
└── references/
    ├── cli-reference.md               # Complete CLI commands and flags
    ├── subprocess-patterns.md         # Python invocation patterns
    ├── hooks-examples.md              # Hook configuration and scripts
    ├── json-extraction.md             # Parsing JSON from output
    └── orchestrator_example.py        # Complete Python orchestrator class
```

## Triggers

The skill activates on queries containing:
- "claude cli", "claude subprocess", "claude permissions"
- "--allowedTools", "--add-dir", "--settings"
- "spawn claude", "headless claude", "parallel agents"

## Use Cases

| Scenario | Reference |
|----------|-----------|
| Python subprocess invocation | [subprocess-patterns.md](references/subprocess-patterns.md) |
| Setting up hooks for logging | [hooks-examples.md](references/hooks-examples.md) |
| Parsing JSON from output | [json-extraction.md](references/json-extraction.md) |
| Complete orchestrator class | [orchestrator_example.py](references/orchestrator_example.py) |
| CLI flag reference | [cli-reference.md](references/cli-reference.md) |

## Links

- [Skill Listing on SkillzWave](https://skillzwave.ai/skill/SpillwaveSolutions__using-claude-code-cli-agent-skill__using-claude-code-cli__SKILL/)
- [GitHub Repository](https://github.com/SpillwaveSolutions/using-claude-code-cli-agent-skill)
- [Agent Skill Standard](https://agentskills.io/)
- [SkillzWave Marketplace](https://skillzwave.ai/)
- [SpillWave Solutions](https://spillwave.com/)

## License

MIT
