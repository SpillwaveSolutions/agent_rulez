---
last_modified: 2026-03-16
last_validated: 2026-03-16
---

# RuleZ Test -- Batch Test Scenarios

Validate your hooks configuration by running test scenarios from a YAML file. Catch rule regressions before they reach production.

## Overview

`rulez test` runs batch test scenarios defined in a YAML file against your hooks configuration. Each test case simulates an event (like a tool execution or file write) and checks whether the outcome matches your expectation -- allow, block, or inject. This lets you verify that your rules behave correctly and catch regressions whenever you modify your hooks.yaml. The command exits with code 1 if any test fails, making it ideal for CI pipelines.

## Prerequisites

- **RuleZ v2.2+** installed and on your PATH (`rulez --version`)
- A `hooks.yaml` configuration file with rules (run `rulez init` to create one)
- A test YAML file with test scenarios (you will create one below)

## Quick Start: Write Your First Test

### Step 1: Create a test file

Create a file called `tests/hooks-test.yaml` with two simple test cases -- one that should be blocked and one that should be allowed:

```yaml
tests:
  - name: "Block rm -rf commands"
    event_type: "PreToolUse"
    tool: "Bash"
    command: "rm -rf /"
    expected: "block"

  - name: "Allow ls commands"
    event_type: "PreToolUse"
    tool: "Bash"
    command: "ls -la"
    expected: "allow"
```

This assumes your hooks.yaml has a rule that blocks `rm -rf` commands. If you used `rulez init --with-examples`, the default rules include this.

### Step 2: Run the tests

```bash
rulez test tests/hooks-test.yaml
```

If both tests pass:

```
Running 2 test(s) from tests/hooks-test.yaml
============================================================

  PASS  Block rm -rf commands
  PASS  Allow ls commands

============================================================
2 passed, 0 failed, 2 total
```

### Step 3: See what failure looks like

Modify the second test to expect `block` instead of `allow`:

```yaml
  - name: "Allow ls commands"
    event_type: "PreToolUse"
    tool: "Bash"
    command: "ls -la"
    expected: "block"          # This will fail -- ls is allowed
```

Run again with `--verbose` to see the reason:

```bash
rulez test tests/hooks-test.yaml --verbose
```

```
Running 2 test(s) from tests/hooks-test.yaml
============================================================

  PASS  Block rm -rf commands
  FAIL  Allow ls commands
        expected: block, actual: allow
        reason: No matching rules found

============================================================
1 passed, 1 failed, 2 total
```

The `--verbose` flag shows why the test failed -- no rule matched the `ls` command to block it.

## CLI Flags

| Flag | Short | Description |
|------|-------|-------------|
| `<file.yaml>` | -- | Positional argument (required). Path to test YAML file. |
| `--verbose` | `-v` | Show the reason for failed test cases. |

**Exit codes:**
- `0` -- All tests passed
- `1` -- One or more tests failed

## Test YAML Schema

A test file has a top-level `tests` array. Each entry is a TestCase with the following fields:

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `name` | string | Yes | Descriptive name for the test case. Shown in pass/fail output. |
| `event_type` | string | Yes | Event to simulate. Common values: `PreToolUse`, `PostToolUse`, `SessionStart`. Supports aliases like `pre`, `post`, `session`. |
| `tool` | string | No | Tool name to simulate (e.g., `Bash`, `Write`, `Read`, `Grep`). |
| `command` | string | No | Command string for Bash tool scenarios (e.g., `rm -rf /`). |
| `path` | string | No | File path for Write/Read tool scenarios (e.g., `src/main.rs`). |
| `prompt` | string | No | Prompt text for `prompt_match` rule scenarios. |
| `expected` | string | Yes | Expected outcome: `allow`, `block`, or `inject`. |

### Field details

**`event_type`** must be one of the event types RuleZ recognizes. The most common are:
- `PreToolUse` (alias: `pre`) -- Before a tool runs (used by most block/inject rules)
- `PostToolUse` (alias: `post`) -- After a tool returns output
- `SessionStart` (alias: `session`) -- When a session begins

See the [CLI Commands Reference](../../mastering-hooks/references/cli-commands.md) for the full list of event types and aliases.

**`expected`** determines the pass/fail check:
- `allow` -- The event passes through with no block and no context injection
- `block` -- The event is denied (the response has `continue: false`)
- `inject` -- The event passes through but with context injected into the response

## Complete Runnable Example

Here is a full test file with 6 diverse scenarios and the corresponding hooks.yaml rules.

### hooks.yaml

```yaml
rules:
  - name: deny-rm-rf
    description: "Block rm -rf commands"
    matchers:
      tools: [Bash]
      command_match: "rm\\s+-rf"
    actions:
      block: true
      message: "Blocked: rm -rf is not allowed"
    priority: 100

  - name: deny-force-push
    description: "Block git force-push"
    matchers:
      tools: [Bash]
      command_match: "git\\s+push.*--force"
    actions:
      block: true
      message: "Blocked: force-push is not allowed"
    priority: 100

  - name: inject-python-standards
    description: "Inject Python coding standards when editing .py files"
    matchers:
      tools: [Write]
      extensions: [py]
    actions:
      inject: ".claude/context/python-standards.md"
    priority: 10

  - name: block-secret-writes
    description: "Prevent writing to secret files"
    matchers:
      tools: [Write]
      extensions: [env, pem, key]
    actions:
      block: true
      message: "Blocked: cannot write secret files"
    priority: 100

  - name: audit-file-reads
    description: "Log file read events"
    matchers:
      tools: [Read]
    actions:
      run: "logger 'file read'"
    priority: 1
```

### tests/hooks-test.yaml

```yaml
tests:
  - name: "Block dangerous rm -rf"
    event_type: "PreToolUse"
    tool: "Bash"
    command: "rm -rf /"
    expected: "block"

  - name: "Allow safe ls command"
    event_type: "PreToolUse"
    tool: "Bash"
    command: "ls -la src/"
    expected: "allow"

  - name: "Inject Python standards on .py write"
    event_type: "PreToolUse"
    tool: "Write"
    path: "src/main.py"
    expected: "inject"

  - name: "Block writing .env files"
    event_type: "PreToolUse"
    tool: "Write"
    path: "config/.env"
    expected: "block"

  - name: "Allow reading any file"
    event_type: "PreToolUse"
    tool: "Read"
    path: "src/main.rs"
    expected: "allow"

  - name: "Block git force-push"
    event_type: "PreToolUse"
    tool: "Bash"
    command: "git push origin main --force"
    expected: "block"
```

### Running the tests

```bash
$ rulez test tests/hooks-test.yaml
Running 6 test(s) from tests/hooks-test.yaml
============================================================

  PASS  Block dangerous rm -rf
  PASS  Allow safe ls command
  PASS  Inject Python standards on .py write
  PASS  Block writing .env files
  PASS  Allow reading any file
  PASS  Block git force-push

============================================================
6 passed, 0 failed, 6 total
```

## CI Integration

Add `rulez test` to your CI pipeline to catch rule regressions automatically. The command exits with code 1 when any test fails, so CI will fail the build.

### GitHub Actions

```yaml
jobs:
  validate-hooks:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4

      - name: Install RuleZ
        run: |
          curl -sL https://github.com/SpillwaveSolutions/rulez/releases/latest/download/rulez-linux-amd64 -o rulez
          chmod +x rulez
          sudo mv rulez /usr/local/bin/

      - name: Validate hooks configuration
        run: rulez test tests/hooks-test.yaml
```

This ensures that any PR that changes hooks.yaml will be validated against your test scenarios before merging.

## Troubleshooting

### "Failed to read test file: tests/hooks-test.yaml"

The test file path is incorrect or the file does not exist.

- Check the file exists: `ls tests/hooks-test.yaml`
- Use an absolute or correct relative path: `rulez test ./tests/hooks-test.yaml`

### "unknown event type" error

The `event_type` field contains an unrecognized value. Valid event types include:
- `PreToolUse` (aliases: `pre`, `pretooluse`, `pre-tool-use`)
- `PostToolUse` (aliases: `post`, `posttooluse`, `post-tool-use`)
- `SessionStart` (aliases: `session`, `start`, `sessionstart`)
- `BeforeAgent` (aliases: `subagent`, `beforeagent`, `before-agent`)

Check spelling carefully -- event types are case-insensitive but must match one of the recognized names or aliases.

### Test expects "block" but gets "allow"

The rule's matchers do not match the simulated event. Common causes:

- **Wrong tool name:** Rule matches `Bash` but test uses `bash` (tool names are case-sensitive)
- **Missing command:** Rule uses `command_match` but test case has no `command` field
- **Regex mismatch:** The `command_match` regex does not match the test command string. Use `rulez debug pre --tool Bash --command "your command" -v` to test interactively
- **Rule disabled:** Check that the rule does not have `metadata.enabled: false`

### Tests pass locally but fail in CI

RuleZ loads configuration from `.claude/hooks.yaml` by default. In CI:

- Ensure `.claude/hooks.yaml` is checked into the repository
- Verify the working directory is correct (CI may run from a different path)
- Check that context files referenced by `inject` actions exist in the repo

## Further Reading

- [CLI Commands Reference](../../mastering-hooks/references/cli-commands.md) -- Full `rulez test` flag reference
- [Hooks YAML Schema](../../mastering-hooks/references/hooks-yaml-schema.md) -- Complete rule syntax and field definitions
- [External Logging](external-logging.md) -- Audit logging for test runs
