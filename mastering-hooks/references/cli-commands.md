# RuleZ CLI Commands Reference

Complete reference for all RuleZ binary commands.

## Global Options

```bash
rulez [OPTIONS] <COMMAND>

Options:
  --config <PATH>    Override config file path
  --json             Output in JSON format
  --verbose, -v      Increase verbosity (use -vv, -vvv for more)
  --quiet, -q        Suppress non-error output
  --help, -h         Show help
  --version, -V      Show version
```

---

## Commands

### version

Display version and API information.

```bash
rulez --version
# Output: rulez 1.8.0

rulez --version --json
# Output: {"version": "1.8.0", "api_version": "1.8.0", "git_sha": "abc1234"}
```

**Use case**: Verify installation, check API compatibility.

---

### init

Initialize RuleZ configuration in current project.

```bash
rulez init [OPTIONS]

Options:
  --force           Overwrite existing configuration
  --template <NAME> Use a specific template (default, minimal, security)
```

**Examples**:

```bash
# Create default hooks.yaml
rulez init

# Overwrite existing config
rulez init --force

# Use minimal template
rulez init --template minimal
```

**Created files**:
```
.claude/
├── hooks.yaml           # Main configuration
└── context/
    └── .gitkeep         # Placeholder for context files
```

**Default template contents**:
```yaml
version: "1"

hooks:
  # Example: Inject coding standards for Python files
  # - name: python-standards
  #   event: PreToolUse
  #   match:
  #     tools: [Write, Edit]
  #     extensions: [.py]
  #   action:
  #     type: inject
  #     source: file
  #     path: .claude/context/python-standards.md
```

---

### install

Register RuleZ with Claude Code.

```bash
rulez install [OPTIONS]

Options:
  --project         Install for current project only (default)
  --user            Install globally for user
```

**Examples**:

```bash
# Install for current project
rulez install --project

# Install globally
rulez install --user
```

**What it does**:
1. Locates `.claude/settings.json`
2. Adds hook configuration entries
3. Creates `.claude/rulez/install.json` audit trail

**Verification**:
```bash
cat .claude/settings.json | grep -A5 hooks
```

---

### uninstall

Remove RuleZ registration from Claude Code.

```bash
rulez uninstall [OPTIONS]

Options:
  --project         Uninstall from current project (default)
  --user            Uninstall globally
```

---

### validate

Validate configuration file.

```bash
rulez validate [OPTIONS]

Options:
  --config <PATH>   Validate specific file
  --strict          Fail on warnings too
```

**Examples**:

```bash
# Validate project config
rulez validate

# Validate specific file
rulez validate --config /path/to/hooks.yaml

# Strict mode (warnings are errors)
rulez validate --strict
```

**Output examples**:

```bash
# Success
$ rulez validate
Configuration valid: 5 hooks defined

# Error
$ rulez validate
Error: Invalid event type 'PreTool' at hooks[0]
  Valid events: PreToolUse, PostToolUse, BeforeAgent, AfterAgent, ...

# Warning (non-strict)
$ rulez validate
Warning: Hook 'unused-rule' has no matching events in typical usage
Configuration valid: 5 hooks defined (1 warning)
```

---

### explain

Analyze and explain configuration.

```bash
rulez explain <SUBCOMMAND>

Subcommands:
  config            Explain entire configuration
  rule <NAME>       Explain specific rule
  event <EVENT>     Show rules for specific event
```

**Examples**:

```bash
# Full configuration overview
rulez explain config

# Specific rule
rulez explain rule python-standards

# Rules for an event
rulez explain event PreToolUse
```

**Sample output** for `rulez explain rule python-standards`:
```
Rule: python-standards
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
Event:      PreToolUse
Priority:   50 (medium)
Enabled:    true

Matchers:
  - tools: [Write, Edit]
  - extensions: [.py]

Action:
  Type: inject
  Source: file
  Path: .claude/context/python-standards.md

Triggers when:
  Write or Edit tool is used on any file ending with .py

Effect:
  Injects content from .claude/context/python-standards.md
  into Claude's context before the tool executes.
```

---

### debug

Debug hook matching and execution.

```bash
rulez debug <EVENT> [OPTIONS]

Options:
  --tool <NAME>        Simulate tool name
  --path <PATH>        Simulate file path
  --command <CMD>      Simulate Bash command
  --prompt <TEXT>      Simulate user prompt
  --verbose, -v        Show detailed matching
  --dry-run            Don't execute actions
```

**Event aliases** (case-insensitive):

| Input | Resolves To |
|-------|-------------|
| `pre`, `pretooluse`, `pre-tool-use` | `PreToolUse` |
| `post`, `posttooluse`, `post-tool-use` | `PostToolUse` |
| `session`, `start`, `sessionstart` | `SessionStart` |
| `end`, `sessionend`, `session-end` | `SessionEnd` |
| `permission`, `perm`, `permissionrequest` | `PermissionRequest` |
| `prompt`, `user-prompt`, `userpromptsubmit`, `user-prompt-submit` | `UserPromptSubmit` |
| `compact`, `precompact`, `pre-compact` | `PreCompact` |
| `subagent`, `beforeagent`, `before-agent`, `subagentstart` | `BeforeAgent` |
| `afteragent`, `after-agent`, `subagentstop` | `AfterAgent` |
| `idle`, `teammateidle` | `TeammateIdle` |
| `task`, `taskcompleted` | `TaskCompleted` |

**Examples**:

```bash
# Debug Write tool on Python file
rulez debug PreToolUse --tool Write --path src/main.py -v

# Debug Bash command
rulez debug pre --tool Bash --command "git push --force" -v

# Debug user prompt
rulez debug prompt --prompt "Deploy to production" -v

# Debug agent events
rulez debug beforeagent -v

# Use short alias
rulez debug subagent -v
```

**Sample output**:
```
Debugging PreToolUse event
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
Simulated context:
  tool.name: Write
  tool.input.path: src/main.py

Rule matching:
  [SKIP] block-force-push
    - tools: [Bash] does not match Write
  [MATCH] python-standards
    - tools: [Write, Edit] matches Write
    - extensions: [.py] matches .py
  [SKIP] js-standards
    - extensions: [.js, .ts] does not match .py

Matched rules: 1
  1. python-standards (priority: 50)
     Action: inject from .claude/context/python-standards.md

Dry run: No actions executed
```

---

### repl

Interactive debug mode for testing rules in real-time.

```bash
rulez repl [OPTIONS]

Options:
  --config <PATH>   Use specific config file
```

---

### logs

Query hook execution logs.

```bash
rulez logs [OPTIONS]

Options:
  --tail <N>         Show last N entries (default: 10)
  --since <TIME>     Show logs since time (e.g., "1h", "30m", "2024-01-01")
  --event <EVENT>    Filter by event type
  --rule <NAME>      Filter by rule name
  --status <STATUS>  Filter by status (matched, blocked, error)
  --json             Output as JSON
```

**Examples**:

```bash
# Last 10 entries
rulez logs

# Last 50 entries
rulez logs --tail 50

# Logs from last hour
rulez logs --since 1h

# Only blocked actions
rulez logs --status blocked

# Specific rule
rulez logs --rule python-standards --tail 20

# JSON output for parsing
rulez logs --json | jq '.[] | select(.status == "error")'
```

**Sample output**:
```
RuleZ Execution Log
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
2024-01-15 14:32:01 | PreToolUse | python-standards | matched
  Tool: Write, Path: src/api/handler.py
  Action: injected 1.2KB context

2024-01-15 14:31:45 | PreToolUse | block-force-push | blocked
  Tool: Bash, Command: git push --force origin main
  Reason: Force push to main is prohibited

2024-01-15 14:30:12 | PreToolUse | (no match)
  Tool: Read, Path: README.md
```

---

### run (Manual Execution)

Manually execute a hook for testing.

```bash
rulez run <RULE_NAME> [OPTIONS]

Options:
  --context <JSON>   Provide simulated context
  --dry-run          Show what would happen
```

**Examples**:

```bash
# Test a rule manually
rulez run python-standards --context '{"tool": {"name": "Write", "input": {"path": "test.py"}}}'

# Dry run
rulez run security-check --dry-run
```

---

### test

Run batch test scenarios against your rules configuration. Accepts a YAML test file defining scenarios with expected outcomes (allow, block, or inject), reports pass/fail for each, and exits with code 1 if any test fails.

```bash
rulez test <TEST_FILE> [OPTIONS]

Options:
  -v, --verbose    Show detailed output for each test case (e.g., block reasons)
```

**Test file format** (`tests.yaml`):

```yaml
tests:
  - name: "Block force push"
    event_type: PreToolUse
    tool: Bash
    command: "git push --force"
    expected: block

  - name: "Allow normal read"
    event_type: PreToolUse
    tool: Read
    path: "src/main.rs"
    expected: allow

  - name: "Inject Python standards"
    event_type: PreToolUse
    tool: Write
    path: "app.py"
    expected: inject
```

Each test case supports fields: `name`, `event_type`, `tool`, `command`, `path`, `prompt`, and `expected` (one of `allow`, `block`, `inject`).

**Examples**:

```bash
# Run all test scenarios
rulez test tests.yaml

# Verbose output (shows block reasons on failure)
rulez test tests.yaml --verbose
```

**Sample output**:

```
Running 3 test(s) from tests.yaml
============================================================

  PASS  Block force push
  PASS  Allow normal read
  FAIL  Inject Python standards
        expected: inject, actual: allow

============================================================
2 passed, 1 failed, 3 total
```

---

### lint

Analyze rule configuration for quality issues: duplicate rule names, overlapping rules, dead (disabled) rules, missing descriptions, invalid regex, conflicting actions, and missing priorities.

```bash
rulez lint [OPTIONS]

Options:
  -c, --config <PATH>    Path to configuration file (default: .claude/hooks.yaml)
  -v, --verbose          Show detailed analysis (e.g., glob consolidation suggestions)
```

Diagnostics are categorized by severity:
- **ERROR** -- Issues that will cause incorrect behavior (duplicate names, no matchers, conflicting actions)
- **WARN** -- Issues worth investigating (overlapping rules, dead rules, missing descriptions, invalid regex)
- **INFO** -- Optimization suggestions (missing priority, glob consolidation)

Exits with code 1 if any errors are found.

**Examples**:

```bash
# Lint default config
rulez lint

# Lint a specific file
rulez lint --config /path/to/hooks.yaml

# Show verbose analysis with optimization hints
rulez lint --verbose
```

**Sample output**:

```
rulez lint -- Rule Quality Analysis
==================================

Loaded 5 rules from .claude/hooks.yaml

[ERROR] duplicate-rule-name: Rules at positions 1 and 3 both have the name 'block-push'
[WARN]  dead-rule: Rule 'old-checker' is disabled (metadata.enabled: false) -- consider removing it
[WARN]  no-description: Rule 'quick-fix' has no description
[INFO]  missing-priority: Rule 'standards' has no explicit priority (using default 0)

Summary: 1 error, 2 warnings, 1 info
```

---

### upgrade

Self-update the rulez binary to the latest GitHub release. Downloads the appropriate binary for your platform and replaces the current installation.

```bash
rulez upgrade [OPTIONS]

Options:
  --check    Only check for updates, do not install
```

**Examples**:

```bash
# Check if an update is available
rulez upgrade --check

# Download and install the latest version
rulez upgrade
```

**Sample output**:

```
Current version: 2.2.0
Checking GitHub releases for latest version...
Latest version: 2.3.0
Upgrade available: 2.2.0 -> 2.3.0
Downloading and installing 2.3.0...
Successfully upgraded to 2.3.0!
Restart rulez to use the new version.
```

---

## Multi-Platform Commands

RuleZ supports multiple AI coding assistants. Each platform has `install` and `doctor` subcommands.

### gemini install

Install RuleZ hooks for Gemini CLI. Registers hook entries in Gemini's `settings.json` for all supported events.

```bash
rulez gemini install [OPTIONS]

Options:
  --scope <SCOPE>       Settings scope: project, user, or system (default: project)
  -b, --binary <PATH>   Path to rulez binary (auto-detected if not specified)
  --print               Print JSON snippet without writing (alias: --dry-run)
```

**Events registered**: BeforeTool, AfterTool, BeforeAgent, AfterAgent, BeforeModel, AfterModel, BeforeToolSelection, SessionStart, SessionEnd, Notification, PreCompact

**Examples**:

```bash
# Install for current project
rulez gemini install

# Install for user scope
rulez gemini install --scope user

# Preview what would be written
rulez gemini install --print
```

---

### gemini doctor

Diagnose Gemini hook installation and configuration.

```bash
rulez gemini doctor [OPTIONS]

Options:
  --json    Output machine-readable JSON
```

**Examples**:

```bash
# Run diagnostics
rulez gemini doctor

# Machine-readable output
rulez gemini doctor --json
```

---

### copilot install

Install RuleZ hooks for GitHub Copilot CLI. Creates hook files in `.github/hooks/` and wrapper scripts.

```bash
rulez copilot install [OPTIONS]

Options:
  -b, --binary <PATH>   Path to rulez binary (auto-detected if not specified)
  --print               Print JSON snippet without writing (alias: --dry-run)
```

**Events registered**: preToolUse, postToolUse

**Examples**:

```bash
# Install for current project
rulez copilot install

# Preview the hooks file
rulez copilot install --print
```

---

### copilot doctor

Diagnose Copilot hook installation and configuration.

```bash
rulez copilot doctor [OPTIONS]

Options:
  --json    Output machine-readable JSON
```

**Examples**:

```bash
rulez copilot doctor
rulez copilot doctor --json
```

---

### opencode install

Install RuleZ hooks for OpenCode. Registers hook entries in OpenCode's settings for all supported events.

```bash
rulez opencode install [OPTIONS]

Options:
  --scope <SCOPE>       Settings scope: project or user (default: project)
  -b, --binary <PATH>   Path to rulez binary (auto-detected if not specified)
  --print               Print JSON snippet without writing (alias: --dry-run)
```

**Events registered**: file.edited, tool.execute.before, tool.execute.after, session.updated

**Examples**:

```bash
# Install for current project
rulez opencode install

# Install for user scope
rulez opencode install --scope user

# Preview what would be written
rulez opencode install --print
```

---

### opencode doctor

Diagnose OpenCode hook installation and configuration.

```bash
rulez opencode doctor [OPTIONS]

Options:
  --json    Output machine-readable JSON
```

**Examples**:

```bash
rulez opencode doctor
rulez opencode doctor --json
```

---

## Exit Codes

| Code | Meaning |
|------|---------|
| 0 | Success |
| 1 | Configuration error |
| 2 | Validation error |
| 3 | Runtime error |

---

## Environment Variables

| Variable | Purpose | Default |
|----------|---------|---------|
| `RULEZ_CONFIG` | Override config path | `.claude/hooks.yaml` |
| `RULEZ_LOG_LEVEL` | Log verbosity | `info` |
| `RULEZ_LOG_FILE` | Log file path | `~/.claude/logs/rulez.log` |
| `RULEZ_TIMEOUT` | Default script timeout | `30` |
| `NO_COLOR` | Disable colored output | (unset) |

---

## Shell Completion

Generate shell completions:

```bash
# Bash
rulez completions bash > /etc/bash_completion.d/rulez

# Zsh
rulez completions zsh > ~/.zsh/completions/_rulez

# Fish
rulez completions fish > ~/.config/fish/completions/rulez.fish
```
