# SDD Installation & Setup Guide

## Installation Methods

### Method 1: Persistent Installation (Recommended)

```bash
uv tool install specify-cli --from git+https://github.com/github/spec-kit.git
```

**Benefits:**
- Tool stays installed and available in PATH
- Better tool management: `uv tool list`, `uv tool upgrade`, `uv tool uninstall`
- Cleaner shell configuration

**Usage after installation:**
```bash
specify init <PROJECT_NAME>
specify check
```

**Upgrade:**
```bash
uv tool install specify-cli --force --from git+https://github.com/github/spec-kit.git
```

### Method 2: One-time Usage

```bash
uvx --from git+https://github.com/github/spec-kit.git specify init <PROJECT_NAME>
```

## Verify Installation

```bash
specify check
```

This checks for:
- git
- AI coding agents (claude, gemini, copilot, cursor, windsurf, etc.)
- Other required tools

## Prerequisites

- Linux/macOS/Windows
- Supported AI coding agent
- `uv` for package management
- Python 3.11+
- Git

## Environment Variables

### SPECIFY_FEATURE

**Purpose**: Override feature detection for non-Git repositories

**Usage**:
```bash
export SPECIFY_FEATURE=001-photo-albums
```

**When needed**: Working on a specific feature when not using Git branches

**Important**: Must be set in the context of the agent before using `/speckit.plan` or follow-up commands

## Troubleshooting

### Git Credential Manager on Linux

If having Git authentication issues on Linux:

```bash
#!/usr/bin/env bash
set -e
echo "Downloading Git Credential Manager v2.6.1..."
wget https://github.com/git-ecosystem/git-credential-manager/releases/download/v2.6.1/gcm-linux_amd64.2.6.1.deb
echo "Installing Git Credential Manager..."
sudo dpkg -i gcm-linux_amd64.2.6.1.deb
echo "Configuring Git to use GCM..."
git config --global credential.helper manager
echo "Cleaning up..."
rm gcm-linux_amd64.2.6.1.deb
```

### Common Issues

**Issue**: "specify: command not found"
- **Solution**: Ensure `uv` is installed and PATH is configured correctly
- **Check**: Run `uv tool list` to verify installation

**Issue**: Slash commands not available in AI agent
- **Solution**: Ensure you ran `specify init` in the project directory
- **Check**: Look for `.claude/commands/` directory with command definitions

**Issue**: Feature detection not working
- **Solution**: Either use Git branches or set `SPECIFY_FEATURE` environment variable
- **Check**: Verify you're on the correct Git branch or the env var is set in agent context

**Issue**: AI agent not generating expected artifacts
- **Solution**: Ensure you're following the workflow sequence in order
- **Check**: Review previous artifacts for completeness before proceeding

## Initialization Options

### Basic Initialization

```bash
# Basic initialization
specify init my-project

# With specific AI assistant
specify init my-project --ai claude
specify init my-project --ai cursor-agent
specify init my-project --ai windsurf
specify init my-project --ai copilot

# Initialize in current directory
specify init . --ai claude
# or
specify init --here --ai claude

# Force merge into non-empty directory
specify init . --force --ai claude

# With PowerShell scripts (Windows/cross-platform)
specify init my-project --ai copilot --script ps

# Skip git initialization
specify init my-project --ai gemini --no-git

# Debug mode
specify init my-project --ai claude --debug

# With GitHub token (corporate environments)
specify init my-project --ai claude --github-token ghp_your_token_here
```

## Resources

- GitHub Spec-Kit Repository: https://github.com/github/spec-kit
- Issues/Support: https://github.com/github/spec-kit/issues
- License: MIT
