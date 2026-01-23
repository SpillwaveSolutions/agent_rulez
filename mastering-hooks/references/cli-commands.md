# CCH CLI Commands Reference

Complete reference for all CCH binary commands.

## Global Options

```bash
cch [OPTIONS] <COMMAND>

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
cch --version
# Output: cch 0.2.1

cch --version --json
# Output: {"version": "0.2.1", "api_version": "0.2.1", "git_sha": "abc1234"}
```

**Use case**: Verify installation, check API compatibility.

---

### init

Initialize CCH configuration in current project.

```bash
cch init [OPTIONS]

Options:
  --force           Overwrite existing configuration
  --template <NAME> Use a specific template (default, minimal, security)
```

**Examples**:

```bash
# Create default hooks.yaml
cch init

# Overwrite existing config
cch init --force

# Use minimal template
cch init --template minimal
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

Register CCH with Claude Code.

```bash
cch install [OPTIONS]

Options:
  --project         Install for current project only (default)
  --user            Install globally for user
  --uninstall       Remove CCH registration
```

**Examples**:

```bash
# Install for current project
cch install --project

# Install globally
cch install --user

# Remove registration
cch install --uninstall
```

**What it does**:
1. Locates `.claude/settings.json`
2. Adds hook configuration entries
3. Creates `.claude/cch/install.json` audit trail

**Verification**:
```bash
cat .claude/settings.json | grep -A5 hooks
```

---

### validate

Validate configuration file.

```bash
cch validate [OPTIONS]

Options:
  --config <PATH>   Validate specific file
  --strict          Fail on warnings too
```

**Examples**:

```bash
# Validate project config
cch validate

# Validate specific file
cch validate --config /path/to/hooks.yaml

# Strict mode (warnings are errors)
cch validate --strict
```

**Output examples**:

```bash
# Success
$ cch validate
Configuration valid: 5 hooks defined

# Error
$ cch validate
Error: Invalid event type 'PreTool' at hooks[0]
  Valid events: PreToolUse, PostToolUse, PermissionRequest, ...

# Warning (non-strict)
$ cch validate
Warning: Hook 'unused-rule' has no matching events in typical usage
Configuration valid: 5 hooks defined (1 warning)
```

---

### explain

Analyze and explain configuration.

```bash
cch explain <SUBCOMMAND>

Subcommands:
  config            Explain entire configuration
  rule <NAME>       Explain specific rule
  event <EVENT>     Show rules for specific event
```

**Examples**:

```bash
# Full configuration overview
cch explain config

# Specific rule
cch explain rule python-standards

# Rules for an event
cch explain event PreToolUse
```

**Sample output** for `cch explain rule python-standards`:
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
cch debug <EVENT> [OPTIONS]

Options:
  --tool <NAME>        Simulate tool name
  --path <PATH>        Simulate file path
  --command <CMD>      Simulate Bash command
  --prompt <TEXT>      Simulate user prompt
  --verbose, -v        Show detailed matching
  --dry-run            Don't execute actions
```

**Examples**:

```bash
# Debug Write tool on Python file
cch debug PreToolUse --tool Write --path src/main.py -v

# Debug Bash command
cch debug PreToolUse --tool Bash --command "git push --force" -v

# Debug user prompt
cch debug UserPromptSubmit --prompt "Deploy to production" -v
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

### logs

Query hook execution logs.

```bash
cch logs [OPTIONS]

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
cch logs

# Last 50 entries
cch logs --tail 50

# Logs from last hour
cch logs --since 1h

# Only blocked actions
cch logs --status blocked

# Specific rule
cch logs --rule python-standards --tail 20

# JSON output for parsing
cch logs --json | jq '.[] | select(.status == "error")'
```

**Sample output**:
```
CCH Execution Log
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
cch run <RULE_NAME> [OPTIONS]

Options:
  --context <JSON>   Provide simulated context
  --dry-run          Show what would happen
```

**Examples**:

```bash
# Test a rule manually
cch run python-standards --context '{"tool": {"name": "Write", "input": {"path": "test.py"}}}'

# Dry run
cch run security-check --dry-run
```

---

## Exit Codes

| Code | Meaning |
|------|---------|
| 0 | Success |
| 1 | General error |
| 2 | Configuration error |
| 3 | Validation failed |
| 4 | Hook blocked action |
| 5 | Script execution failed |

---

## Environment Variables

| Variable | Purpose | Default |
|----------|---------|---------|
| `CCH_CONFIG` | Override config path | `.claude/hooks.yaml` |
| `CCH_LOG_LEVEL` | Log verbosity | `info` |
| `CCH_LOG_FILE` | Log file path | `~/.claude/cch/logs/` |
| `CCH_TIMEOUT` | Default script timeout | `30` |
| `NO_COLOR` | Disable colored output | (unset) |

---

## Shell Completion

Generate shell completions:

```bash
# Bash
cch completions bash > /etc/bash_completion.d/cch

# Zsh
cch completions zsh > ~/.zsh/completions/_cch

# Fish
cch completions fish > ~/.config/fish/completions/cch.fish
```
