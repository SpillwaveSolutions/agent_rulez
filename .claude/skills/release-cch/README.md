# release-cch Skill

CCH release workflow automation for Claude Code.

## Usage

Invoke via the `/cch-release` command:

```bash
/cch-release              # Interactive full workflow
/cch-release prepare      # Create branch, changelog, PR
/cch-release execute      # Merge PR, create tag
/cch-release verify       # Check release status
/cch-release hotfix v1.0.0 # Patch from existing tag
```

## Structure

```
release-cch/
├── SKILL.md              # Main skill documentation
├── README.md             # This file
├── scripts/              # Automation scripts
│   ├── read-version.sh   # Extract version from Cargo.toml
│   ├── generate-changelog.sh # Generate changelog from commits
│   ├── preflight-check.sh    # Pre-release verification
│   └── verify-release.sh     # Verify release completed
├── references/           # Additional documentation
│   ├── release-workflow.md   # Standard release diagram
│   ├── hotfix-workflow.md    # Hotfix release diagram
│   └── troubleshooting.md    # Common issues and solutions
└── templates/            # Reusable templates
    ├── changelog-entry.md    # Changelog entry template
    └── pr-body.md            # Pull request body template
```

## Quick Start

1. Update version in `Cargo.toml`
2. Run `/cch-release prepare`
3. Wait for CI to pass
4. Run `/cch-release execute`
5. Run `/cch-release verify`

## See Also

- [SKILL.md](SKILL.md) - Complete workflow documentation
- [references/troubleshooting.md](references/troubleshooting.md) - Problem solving
