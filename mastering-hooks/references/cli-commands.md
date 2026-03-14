# RuleZ CLI Commands Reference

Complete reference for all RuleZ CLI commands. All flag names and descriptions match `rulez --help` and `rulez <cmd> --help` output as of v2.2.1.

## Global Options

These options are available on every command and subcommand:

```
rulez [OPTIONS] <COMMAND>

Options:
      --debug-logs  Enable debug logging with full event and rule details
  -h, --help        Print help
  -V, --version     Print version
```

## Command Index

| Command | Description |
|---------|-------------|
| `rulez init` | Initialize RuleZ configuration in current project |
| `rulez install` | Install RuleZ hook into Claude Code settings |
| `rulez uninstall` | Uninstall RuleZ hook from Claude Code settings |
| `rulez debug` | Simulate an event to test rules |
| `rulez repl` | Start interactive debug mode |
| `rulez validate` | Validate configuration file |
| `rulez logs` | Query and display logs |
| `rulez explain` | Explain rules or events |
| `rulez test` | Run batch test scenarios from a YAML file |
| `rulez lint` | Analyze rule quality and detect issues |
| `rulez upgrade` | Check for and install newer rulez binary releases |
| `rulez gemini` | Gemini CLI utilities (install, hook, doctor) |
| `rulez copilot` | Copilot CLI utilities (install, hook, doctor) |
| `rulez opencode` | OpenCode CLI utilities (install, hook, doctor) |

---

## Commands

### init

Initialize RuleZ configuration in current project.

```
rulez init [OPTIONS]

Options:
  -f, --force          Overwrite existing configuration
      --with-examples  Create example context and validator files
```

**Examples**:

```bash
# Create default hooks.yaml
rulez init

# Overwrite existing config
rulez init --force

# Create config with example files
rulez init --with-examples
```

**Created files**:
```
.claude/
├── hooks.yaml           # Main configuration
└── context/
    └── .gitkeep         # Placeholder for context files
```

---

### install

Install RuleZ hook into Claude Code settings.

```
rulez install [OPTIONS]

Options:
  -g, --global           Install globally instead of project-local
  -b, --binary <BINARY>  Path to RuleZ binary (auto-detected if not specified)
```

**Examples**:

```bash
# Install for current project (default)
rulez install

# Install globally
rulez install --global

# Use specific binary path
rulez install --binary /usr/local/bin/rulez
```

**What it does**:
1. Locates `.claude/settings.json` (project) or `~/.claude/settings.json` (global)
2. Adds hook configuration entries for all supported events
3. Creates `.claude/rulez/install.json` audit trail

**Verification**:
```bash
cat .claude/settings.json | grep -A5 hooks
```

---

### uninstall

Uninstall RuleZ hook from Claude Code settings.

```
rulez uninstall [OPTIONS]

Options:
  -g, --global      Uninstall from global settings instead of project-local
```

**Examples**:

```bash
# Uninstall from current project
rulez uninstall

# Uninstall from global settings
rulez uninstall --global
```

---

### validate

Validate configuration file.

```
rulez validate [OPTIONS]

Options:
  -c, --config <CONFIG>  Path to configuration file
```

**Examples**:

```bash
# Validate project config
rulez validate

# Validate specific file
rulez validate --config /path/to/hooks.yaml
```

**Output examples**:

```bash
# Success
$ rulez validate
Configuration valid: 5 hooks defined

# Error
$ rulez validate
Error: Invalid event type 'PreTool' at hooks[0]
  Valid events: PreToolUse, PostToolUse, SessionStart, SessionEnd, ...
```

---

### explain

Explain rules or events. Has three subcommands plus legacy direct usage.

```
rulez explain [OPTIONS] [EVENT_ID] [COMMAND]

Commands:
  rule   Explain a specific rule's configuration and governance
  rules  List all configured rules
  event  Explain an event by session ID
```

#### explain rule

```
rulez explain rule [OPTIONS] <NAME>

Arguments:
  <NAME>  Name of the rule to explain

Options:
      --json        Output as JSON for machine parsing
      --no-stats    Skip activity statistics (faster)
```

**Example**:

```bash
rulez explain rule python-standards
```

**Sample output**:
```
Rule: python-standards
---
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

#### explain rules

List all configured rules.

```
rulez explain rules
```

#### explain event

Explain an event by session ID.

```
rulez explain event <EVENT_ID>

Arguments:
  <EVENT_ID>  Session/event ID
```

**Example**:

```bash
rulez explain event abc123-session-id
```

---

### debug

Simulate an event to test rules. Useful for verifying rule matching without waiting for real events.

```
rulez debug [OPTIONS] <EVENT_TYPE>

Arguments:
  <EVENT_TYPE>  Event type: PreToolUse, PostToolUse, SessionStart,
                PermissionRequest, UserPromptSubmit, SessionEnd, PreCompact

Options:
  -t, --tool <TOOL>        Tool name (e.g., Bash, Write, Read)
  -c, --command <COMMAND>  Command or pattern to test (for Bash/Glob/Grep)
  -p, --path <PATH>        File path (for Write/Edit/Read)
      --prompt <PROMPT>     User prompt text (for UserPromptSubmit events)
  -v, --verbose            Show verbose rule evaluation
      --json               Output structured JSON (for programmatic consumption)
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

# JSON output for scripting
rulez debug pre --tool Write --path test.py --json
```

**Sample output**:
```
Debugging PreToolUse event
---
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
```

---

### repl

Start interactive debug mode for testing rules in real-time.

```
rulez repl
```

Launches an interactive prompt where you can simulate events, test rule matching, and iterate on configuration without restarting.

---

### logs

Query and display execution logs.

```
rulez logs [OPTIONS]

Options:
  -l, --limit <LIMIT>        Number of recent log entries to show [default: 10]
      --since <SINCE>        Show logs since timestamp (RFC3339 format)
      --mode <MODE>          Filter by policy mode (enforce, warn, audit)
      --decision <DECISION>  Filter by decision (allowed, blocked, warned, audited)
```

**Examples**:

```bash
# Last 10 entries (default)
rulez logs

# Last 50 entries
rulez logs --limit 50

# Logs since a specific time
rulez logs --since 2026-03-14T00:00:00Z

# Only blocked decisions
rulez logs --decision blocked

# Only entries in enforce mode
rulez logs --mode enforce
```

**Sample output**:
```
RuleZ Execution Log
---
2026-03-14 14:32:01 | PreToolUse | python-standards | allowed
  Tool: Write, Path: src/api/handler.py
  Action: injected context

2026-03-14 14:31:45 | PreToolUse | block-force-push | blocked
  Tool: Bash, Command: git push --force origin main
  Reason: Force push to main is prohibited

2026-03-14 14:30:12 | PreToolUse | (no match) | allowed
  Tool: Read, Path: README.md
```

---

### test

Run batch test scenarios against your rules configuration. Accepts a YAML test file defining scenarios with expected outcomes (allow, block, or inject), reports pass/fail for each, and exits with code 1 if any test fails.

```
rulez test [OPTIONS] <TEST_FILE>

Arguments:
  <TEST_FILE>  Path to test scenarios YAML file

Options:
  -v, --verbose     Show detailed output for each test case
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

```
rulez lint [OPTIONS]

Options:
  -c, --config <CONFIG>  Path to configuration file
  -v, --verbose          Show detailed analysis
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

```
rulez upgrade [OPTIONS]

Options:
      --check       Only check for updates, do not install
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
Current version: 2.2.1
Checking GitHub releases for latest version...
Latest version: 2.3.0
Upgrade available: 2.2.1 -> 2.3.0
Downloading and installing 2.3.0...
Successfully upgraded to 2.3.0!
Restart rulez to use the new version.
```

---

## Multi-CLI Commands

RuleZ supports multiple AI coding assistants. Each platform has `install`, `hook`, and `doctor` subcommands.

### gemini

Gemini CLI utilities.

```
rulez gemini <COMMAND>

Commands:
  install  Install Gemini hook settings
  hook     Run Gemini hook runner (stdin -> Gemini JSON response)
  doctor   Diagnose Gemini hook installation and configuration
```

#### gemini install

Install RuleZ hooks for Gemini CLI. Registers hook entries in Gemini's `settings.json` for all supported events.

```
rulez gemini install [OPTIONS]

Options:
      --scope <SCOPE>    Settings scope (project, user, system) [default: project]
  -b, --binary <BINARY>  Path to rulez binary (auto-detected if not specified)
      --print            Print JSON snippet without writing
```

**Examples**:

```bash
# Install for current project
rulez gemini install

# Install for user scope
rulez gemini install --scope user

# Preview what would be written
rulez gemini install --print
```

#### gemini hook

Run the Gemini hook runner. Reads event JSON from stdin and outputs a Gemini-compatible JSON response. This is called by Gemini CLI automatically -- you do not normally invoke this directly.

```
rulez gemini hook
```

#### gemini doctor

Diagnose Gemini hook installation and configuration.

```
rulez gemini doctor [OPTIONS]

Options:
      --json        Output machine-readable JSON
```

**Examples**:

```bash
rulez gemini doctor
rulez gemini doctor --json
```

---

### copilot

Copilot CLI utilities.

```
rulez copilot <COMMAND>

Commands:
  install  Install Copilot hook files into .github/hooks
  hook     Run Copilot hook runner (stdin -> Copilot JSON response)
  doctor   Diagnose Copilot hook installation and configuration
```

#### copilot install

Install RuleZ hooks for GitHub Copilot CLI. Creates hook files in `.github/hooks/` and wrapper scripts.

```
rulez copilot install [OPTIONS]

Options:
  -b, --binary <BINARY>  Path to rulez binary (auto-detected if not specified)
      --print            Print JSON snippet without writing
```

**Examples**:

```bash
# Install for current project
rulez copilot install

# Preview the hooks file
rulez copilot install --print
```

#### copilot hook

Run the Copilot hook runner. Reads event JSON from stdin and outputs a Copilot-compatible JSON response. This is called by Copilot CLI automatically -- you do not normally invoke this directly.

```
rulez copilot hook
```

#### copilot doctor

Diagnose Copilot hook installation and configuration.

```
rulez copilot doctor [OPTIONS]

Options:
      --json        Output machine-readable JSON
```

**Examples**:

```bash
rulez copilot doctor
rulez copilot doctor --json
```

---

### opencode

OpenCode CLI utilities.

```
rulez opencode <COMMAND>

Commands:
  install  Install OpenCode hook settings
  hook     Run OpenCode hook runner (stdin -> RuleZ JSON response)
  doctor   Diagnose OpenCode hook installation and configuration
```

#### opencode install

Install RuleZ hooks for OpenCode. Registers hook entries in OpenCode's settings for all supported events.

```
rulez opencode install [OPTIONS]

Options:
      --scope <SCOPE>    Settings scope (project, user) [default: project]
  -b, --binary <BINARY>  Path to rulez binary (auto-detected if not specified)
      --print            Print JSON snippet without writing
```

**Examples**:

```bash
# Install for current project
rulez opencode install

# Install for user scope
rulez opencode install --scope user

# Preview what would be written
rulez opencode install --print
```

#### opencode hook

Run the OpenCode hook runner. Reads event JSON from stdin and outputs a RuleZ JSON response. This is called by OpenCode automatically -- you do not normally invoke this directly.

```
rulez opencode hook
```

#### opencode doctor

Diagnose OpenCode hook installation and configuration.

```
rulez opencode doctor [OPTIONS]

Options:
      --json        Output machine-readable JSON
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
| 1 | Configuration error (also used by `rulez test` and `rulez lint` on failure) |
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
