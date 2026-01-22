# Mastering GitHub CLI

A comprehensive Claude Code skill for GitHub CLI operations, CI/CD monitoring, workflow authoring, and automation.

## Overview

This skill provides command-line interface mastery for GitHub operations including:

- **Repository Search** - Find repos by files, directories, code patterns
- **CI/CD Monitoring** - Watch workflow runs, check PR status, download artifacts
- **Resource Creation** - Create PRs, issues, repos, branches from command line
- **Workflow Authoring** - Write GitHub Actions YAML with caching, matrix builds, OIDC
- **Automation** - Trigger workflows, batch operations, API access

## Installing with Skilz (Universal Installer)

The recommended way to install this skill across different AI coding agents is using the **skilz** universal installer.

### Install Skilz

```bash
pip install skilz
```

This skill supports [Agent Skill Standard](https://agentskills.io/) which means it supports 14 plus coding agents including Claude Code, OpenAI Codex, Cursor and Gemini.

### Git URL Options

You can use either `-g` or `--git` with HTTPS or SSH URLs:

```bash
# HTTPS URL
skilz install -g https://github.com/SpillwaveSolutions/mastering-github-agent-skill

# SSH URL
skilz install --git git@github.com:SpillwaveSolutions/mastering-github-agent-skill.git
```

### Claude Code

Install to user home (available in all projects):
```bash
skilz install -g https://github.com/SpillwaveSolutions/mastering-github-agent-skill
```

Install to current project only:
```bash
skilz install -g https://github.com/SpillwaveSolutions/mastering-github-agent-skill --project
```

### OpenCode

Install for [OpenCode](https://opencode.ai):
```bash
skilz install -g https://github.com/SpillwaveSolutions/mastering-github-agent-skill --agent opencode
```

Project-level install:
```bash
skilz install -g https://github.com/SpillwaveSolutions/mastering-github-agent-skill --project --agent opencode
```

### Gemini

Project-level install for Gemini:
```bash
skilz install -g https://github.com/SpillwaveSolutions/mastering-github-agent-skill --agent gemini
```

### OpenAI Codex

Install for OpenAI Codex:
```bash
skilz install -g https://github.com/SpillwaveSolutions/mastering-github-agent-skill --agent codex
```

Project-level install:
```bash
skilz install -g https://github.com/SpillwaveSolutions/mastering-github-agent-skill --project --agent codex
```

### Install from Skillzwave Marketplace

```bash
# Claude to user home dir ~/.claude/skills
skilz install SpillwaveSolutions_mastering-github-agent-skill/mastering-github-cli

# Claude skill in project folder ./claude/skills
skilz install SpillwaveSolutions_mastering-github-agent-skill/mastering-github-cli --project

# OpenCode install to user home dir ~/.config/opencode/skills
skilz install SpillwaveSolutions_mastering-github-agent-skill/mastering-github-cli --agent opencode

# OpenCode project level
skilz install SpillwaveSolutions_mastering-github-agent-skill/mastering-github-cli --agent opencode --project

# OpenAI Codex install to user home dir ~/.codex/skills
skilz install SpillwaveSolutions_mastering-github-agent-skill/mastering-github-cli

# OpenAI Codex project level ./.codex/skills
skilz install SpillwaveSolutions_mastering-github-agent-skill/mastering-github-cli --agent codex --project

# Gemini CLI (project level) -- only works with project level
skilz install SpillwaveSolutions_mastering-github-agent-skill/mastering-github-cli --agent gemini
```

See this site [skill Listing](https://skillzwave.ai/skill/SpillwaveSolutions__mastering-github-agent-skill__mastering-github-cli__SKILL/) to see how to install this exact skill to 14+ different coding agents.

### Other Supported Agents

Skilz supports 14+ coding agents including Windsurf, Qwen Code, Aidr, and more.

For the full list of supported platforms, visit [SkillzWave.ai/platforms](https://skillzwave.ai/platforms/) or see the [skilz-cli GitHub repository](https://github.com/SpillwaveSolutions/skilz-cli)

## Prerequisites

- [GitHub CLI](https://cli.github.com/) (`gh`) installed
- Authenticated: `gh auth login`
- Optional: `jq` for JSON processing

## Quick Examples

```bash
# Find repos with specific files
gh search code --filename SKILL.md

# Monitor CI/CD
gh run list --workflow=CI --status=failure
gh pr checks 123 --watch

# Create resources
gh pr create --fill
gh issue create --title "Bug" --label bug

# Trigger workflow
gh workflow run deploy.yml -f environment=staging
```

## Structure

```
mastering-github-cli/
├── SKILL.md              # Main skill file with quick reference
├── README.md             # This file
├── references/           # Detailed reference documentation
│   ├── search.md         # Code and repository search
│   ├── monitoring.md     # CI/CD monitoring commands
│   ├── resources.md      # Creating PRs, issues, repos
│   ├── automation.md     # Workflow triggers and batch ops
│   ├── api.md            # REST/GraphQL API access
│   └── workflow-authoring.md  # GitHub Actions YAML patterns
└── scripts/              # Ready-to-use automation scripts
    ├── find-repos-with-path.sh  # Find repos with specific paths
    ├── wait-for-run.sh          # Wait for workflow completion
    └── batch-search.sh          # Search >1000 results
```

## Rate Limits

| Endpoint | Limit | Notes |
|----------|-------|-------|
| REST API | 5,000/hour | Per authenticated user |
| Search API | 30/minute | All search endpoints |
| Code Search | 10/minute | More restrictive |

## License

MIT

---

<a href="https://skillzwave.ai/">Largest Agentic Marketplace for AI Agent Skills</a> and
<a href="https://spillwave.com/">SpillWave: Leaders in AI Agent Development.</a>
