# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

**NOTE:** All project specifications, implementation plans, and feature tracking are now consolidated in the `.speckit/` directory. The legacy `specs/` and `.specify/` directories have been removed.

## Git Workflow Requirements

**CRITICAL: Always use feature branches for all work.**

- **NEVER commit directly to `main`** - All feature work MUST be done in a feature branch
- Create a feature branch before starting any work: `git checkout -b feature/<feature-name>`
- Push the feature branch and create a Pull Request for review
- Only merge to `main` via PR after review

**Branch Naming Convention:**
- Features: `feature/<feature-name>` (e.g., `feature/add-debug-command`)
- Bugfixes: `fix/<bug-description>` (e.g., `fix/config-parsing-error`)
- Documentation: `docs/<doc-topic>` (e.g., `docs/update-readme`)

**Workflow:**
1. `git checkout -b feature/<name>` - Create feature branch
2. Make changes and commit with conventional commit messages
3. `git push -u origin feature/<name>` - Push to remote
4. Create PR via `gh pr create` or GitHub UI
5. Merge after review

## Project Overview
...
## Repository Structure

```
docs/
  README.md              # Main CCH documentation
  USER_GUIDE_CLI.md      # CCH binary command reference
  USER_GUIDE_SKILL.md    # CCH skill usage guide
  prds/                  # Product requirements documents

.speckit/                # SDD methodology artifacts (consolidated)
  memory/                # Project constitution and decisions
  features/              # Feature specifications and plans
  templates/             # SDD artifact templates
  scripts/               # Automation scripts

.claude/skills/          # Claude Code skills
.gemini/skills/          # Gemini CLI skills (mirrors .claude)
.opencode/skill/         # OpenCode skills (mirrors .claude)
```

## Active Technologies
- **Binary:** Rust 2021 edition (no unsafe code), tokio (async), serde (JSON/YAML), clap (CLI), regex
- **Skill:** React 18+, TypeScript 5.9+, Vite, zod

## CCH Commands
...
## Configuration
...
## Exit Codes
...
<skills_system priority="1">
...
</skills_system>

