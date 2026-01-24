# CCH Integration Test Suite

End-to-end integration tests that verify CCH (Claude Context Hooks) works correctly when invoked by the real Claude CLI.

## Prerequisites

1. **Claude CLI** - Must be installed and in PATH
   ```bash
   claude --version
   ```

2. **CCH Binary** - Built automatically by the test runner
   ```bash
   cd cch_cli && cargo build --release
   ```

3. **Bash** - Tests use bash scripts

## Quick Start

```bash
# Run all integration tests
./run-all.sh

# Run a specific test
./use-cases/01-block-force-push/test.sh

# Run with debug output
DEBUG=1 ./run-all.sh

# Using Taskfile (from project root)
task integration-test
task itest  # alias
```

## Test Cases

| Test | Description | What It Verifies |
|------|-------------|------------------|
| `01-block-force-push` | Block dangerous git operations | CCH blocks `git push --force` |
| `02-context-injection` | Inject context for file types | CCH injects CDK context for `.cdk.ts` files |
| `03-session-logging` | Audit log creation | CCH creates JSON Lines logs with timing |
| `04-permission-explanations` | Permission request context | CCH provides context during permission prompts |

## Directory Structure

```
test/integration/
├── README.md              # This file
├── run-all.sh             # Master test runner
├── lib/
│   └── test-helpers.sh    # Shared test functions
├── use-cases/
│   ├── 01-block-force-push/
│   │   ├── .claude/
│   │   │   └── hooks.yaml
│   │   ├── test.sh
│   │   └── README.md
│   ├── 02-context-injection/
│   │   ├── .claude/
│   │   │   ├── hooks.yaml
│   │   │   └── context/
│   │   ├── sample.cdk.ts
│   │   ├── test.sh
│   │   └── README.md
│   ├── 03-session-logging/
│   │   └── ...
│   └── 04-permission-explanations/
│       └── ...
└── results/               # Test run outputs (gitignored)
```

## How Tests Work

1. **Setup** - Create temp workspace, copy use-case files
2. **Install** - Run `cch install` in the workspace
3. **Execute** - Invoke Claude CLI with specific prompts
4. **Verify** - Check CCH logs for expected behavior
5. **Cleanup** - Remove temp workspace

## Writing New Tests

1. Create a new directory under `use-cases/`:
   ```bash
   mkdir -p use-cases/05-my-test/.claude
   ```

2. Add `hooks.yaml` configuration:
   ```yaml
   version: "1.0"
   rules:
     - name: my-rule
       matchers:
         tools: ["Bash"]
       actions:
         block: true
   settings:
     log_level: "debug"
   ```

3. Create `test.sh`:
   ```bash
   #!/bin/bash
   set -euo pipefail
   
   SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
   source "$SCRIPT_DIR/../../lib/test-helpers.sh"
   
   start_test "05-my-test"
   check_prerequisites
   
   # Setup
   WORKSPACE=$(setup_workspace "$SCRIPT_DIR")
   install_cch "$WORKSPACE"
   
   # Test
   run_claude "$WORKSPACE" "Your prompt" "Bash" 3
   
   # Verify
   assert_log_contains "my-rule" "Rule should match"
   
   # Cleanup
   cleanup_workspace
   end_test
   ```

4. Make executable:
   ```bash
   chmod +x use-cases/05-my-test/test.sh
   ```

## Test Helper Functions

### Setup/Teardown
- `start_test "name"` - Begin a test
- `setup_workspace "/path"` - Create temp workspace
- `install_cch [workspace]` - Install CCH in workspace
- `cleanup_workspace` - Remove temp workspace
- `end_test` - Report results

### Claude CLI
- `run_claude <workspace> <prompt> [tools] [max_turns]` - Run Claude
- `CLAUDE_STDOUT` / `CLAUDE_STDERR` - Captured output

### Logging
- `clear_cch_logs` - Clear log file
- `get_log_line_count` - Current log size
- `log_contains "pattern"` - Check if log has pattern
- `log_contains_since <line> "pattern"` - Check recent entries

### Assertions
- `assert_true <condition> "message"` - Assert condition
- `assert_log_contains "pattern" "message"` - Assert log content
- `assert_claude_output_contains "pattern" "message"` - Assert Claude output
- `assert_file_exists "path" "message"` - Assert file exists

## Troubleshooting

### Claude CLI not found
```bash
# Verify installation
which claude
claude --version

# Add to PATH if needed
export PATH="$PATH:/path/to/claude"
```

### Tests hang
- Tests have timeouts, but Claude may wait for permissions
- Check if `--allowedTools` includes all needed tools
- Use `--max-turns` to limit iterations

### No log entries
- Verify CCH is installed: `cch validate`
- Check log path: `~/.claude/logs/cch.log`
- Enable debug: Set `debug_logs: true` in hooks.yaml

### Permission denied
```bash
chmod +x run-all.sh
chmod +x use-cases/*/test.sh
chmod +x lib/test-helpers.sh
```

## CI/CD Integration

```yaml
# GitHub Actions example
- name: Run Integration Tests
  run: |
    # Ensure Claude CLI is available
    claude --version
    
    # Run tests
    ./test/integration/run-all.sh
```

## Results

Test results are saved to `results/` as JSON files:

```json
{
  "test_name": "01-block-force-push",
  "status": "PASS",
  "assertions_passed": 5,
  "assertions_failed": 0,
  "timestamp": "2025-01-23T10:30:00Z",
  "duration_seconds": 15
}
```

## Related Documentation

- [CCH README](../../cch_cli/README.md) - CCH binary documentation
- [Hooks Configuration](../../cch_cli/docs/hooks-config.md) - hooks.yaml reference
- [Using Claude Code CLI Skill](../../.opencode/skill/using-claude-code-cli/SKILL.md) - CLI automation patterns
