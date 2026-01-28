# release-cch Skill

CCH release workflow automation for Claude Code.

## Overview

This skill provides automated release workflows for the Claude Context Hooks (CCH) project, including:

- **Prepare Release**: Create branch, run checks, generate changelog, create PR
- **Execute Release**: Merge PR, tag, push to trigger CI/CD
- **Verify Release**: Monitor workflows, check assets
- **Hotfix Release**: Patch existing releases with minimal changes

## Usage

Invoke via the `/cch-release` command:

```bash
/cch-release           # Interactive mode
/cch-release prepare   # Prepare a new release
/cch-release execute   # Execute after PR merge
/cch-release verify    # Verify release status
/cch-release hotfix v1.0.0  # Create hotfix
```

## Structure

```
.claude/skills/release-cch/
├── SKILL.md                    # Main skill documentation
├── README.md                   # This file
├── scripts/
│   ├── read-version.sh         # Extract version from Cargo.toml
│   ├── generate-changelog.sh   # Generate changelog from commits
│   ├── preflight-check.sh      # Pre-release validation
│   └── verify-release.sh       # Release verification
├── references/
│   ├── release-workflow.md     # Standard release diagram
│   ├── hotfix-workflow.md      # Hotfix process diagram
│   └── troubleshooting.md      # Common issues and solutions
└── templates/
    ├── changelog-entry.md      # Changelog template
    └── pr-body.md              # PR body template
```

## Scripts

All scripts are executable and can be run from the repo root:

| Script | Purpose |
|--------|---------|
| `read-version.sh` | Reads version from `Cargo.toml` |
| `generate-changelog.sh` | Parses conventional commits |
| `preflight-check.sh` | Runs all pre-release checks |
| `verify-release.sh` | Monitors release workflow |

## Requirements

- Git
- GitHub CLI (`gh`)
- Rust toolchain (for cargo commands)
- Repository: `SpillwaveSolutions/code_agent_context_hooks`

## Migrated From

This skill was migrated from `.opencode/skill/release-cch/` to Claude Code format.

Key changes:
- Path depth adjusted for `.claude/skills/` (5 levels vs 4)
- All path references updated to `.claude/skills/release-cch/`
