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
