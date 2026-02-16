# Runbook 01: Claude Code Manual Testing

Manual testing of RuleZ with Claude Code CLI.

## Prerequisites

- [ ] Claude Code CLI installed (`claude --version`)
- [ ] RuleZ binary built (`cargo build --release` from repo root)
- [ ] Know your RuleZ binary path (e.g., `/path/to/repo/target/release/rulez`)

Set a variable for convenience:

```bash
export RULEZ_BIN="/path/to/agent_rulez/target/release/rulez"
```

---

## Use Case 1: Block Force Push

**Goal**: Verify that RuleZ blocks `git push --force` when Claude tries to run it.

### Step 1: Create test project

```bash
mkdir -p /tmp/rulez-test-block/.claude
cd /tmp/rulez-test-block
git init
```

### Step 2: Write hooks.yaml

```bash
cat > .claude/hooks.yaml << 'EOF'
version: "1.0"

rules:
  - name: block-force-push
    description: "Prevents destructive force push operations"
    matchers:
      tools: ["Bash"]
      command_match: "git push.*--force|git push.*-f"
    actions:
      block: true

  - name: block-hard-reset
    description: "Prevents destructive hard reset operations"
    matchers:
      tools: ["Bash"]
      command_match: "git reset --hard"
    actions:
      block: true

settings:
  log_level: "debug"
  debug_logs: true
  fail_open: false
EOF
```

### Step 3: Install RuleZ

```bash
$RULEZ_BIN install
```

Expected output:
```
Installing RuleZ hook...
  Binary: /path/to/rulez
  Settings: .claude/settings.json
  Scope: project

✓ RuleZ installed successfully!
```

### Step 4: Dry-run with `rulez debug`

Test that the rule matches before going live:

```bash
# Should be BLOCKED
$RULEZ_BIN debug pre --tool Bash --command "git push --force origin main"
```

Expected output (look for):
```
Summary:
----------------------------------------
✗ Blocked: ...
```

```bash
# Should be ALLOWED
$RULEZ_BIN debug pre --tool Bash --command "echo hello"
```

Expected output:
```
Summary:
----------------------------------------
✓ Allowed (no matching rules)
```

```bash
# Also test the -f variant
$RULEZ_BIN debug pre --tool Bash --command "git push -f origin main"
```

Expected: Blocked.

### Step 5: Live test with Claude Code

```bash
cd /tmp/rulez-test-block
claude -p "Run this exact command: git push --force origin main" --allowedTools Bash --max-turns 2
```

**What to observe**:
- Claude should receive a block response from RuleZ
- Claude may report that the operation was blocked, or it may refuse on its own (both are acceptable)

### Step 6: Verify logs

```bash
# Check the RuleZ log for the block event
cat ~/.claude/logs/rulez.log | tail -5
```

Look for:
- `"block-force-push"` rule name in the log
- `"Block"` outcome

### Step 7: Cleanup

```bash
rm -rf /tmp/rulez-test-block
```

### Pass/Fail Criteria

| Check | Expected |
|-------|----------|
| `rulez debug pre --tool Bash --command "git push --force origin main"` | Blocked |
| `rulez debug pre --tool Bash --command "echo hello"` | Allowed |
| Log contains `block-force-push` after live test | Yes |
| Claude does not successfully run force push | Yes |

---

## Use Case 2: Context Injection

**Goal**: Verify that RuleZ injects context when Claude reads or edits files matching a pattern.

### Step 1: Create test project

```bash
mkdir -p /tmp/rulez-test-inject/.claude/context
cd /tmp/rulez-test-inject
```

### Step 2: Create sample files

```bash
# A context file that should be injected
cat > .claude/context/security-guidelines.md << 'EOF'
# Security Guidelines

- Never store secrets in source code
- Always use environment variables for API keys
- Validate all user input before processing
- Use parameterized queries for database access
EOF

# A sample source file to trigger the rule
cat > app.py << 'EOF'
import os

def get_api_key():
    return os.environ.get("API_KEY", "")

def process_input(user_data):
    # TODO: add validation
    return user_data
EOF
```

### Step 3: Write hooks.yaml

```bash
cat > .claude/hooks.yaml << 'EOF'
version: "1.0"

rules:
  - name: inject-security-context
    description: "Inject security guidelines when editing Python files"
    matchers:
      tools: ["Write", "Edit", "Read"]
      extensions: [".py"]
    actions:
      inject: ".claude/context/security-guidelines.md"

settings:
  log_level: "debug"
  debug_logs: true
EOF
```

### Step 4: Install RuleZ

```bash
$RULEZ_BIN install
```

### Step 5: Dry-run with `rulez debug`

```bash
# Should inject context (Read tool on a .py file)
$RULEZ_BIN debug pre --tool Read --path "app.py"
```

Expected output (look for):
```
Summary:
----------------------------------------
✓ Allowed with injected context (XXX chars)
```

```bash
# Should NOT inject (non-.py file)
$RULEZ_BIN debug pre --tool Read --path "README.md"
```

Expected:
```
✓ Allowed (no matching rules)
```

```bash
# JSON mode for detailed inspection
$RULEZ_BIN debug pre --tool Read --path "app.py" --json
```

Look for `"outcome": "Allow"` and `matchedRules` containing `"inject-security-context"`.

### Step 6: Live test with Claude Code

```bash
cd /tmp/rulez-test-inject
claude -p "Read app.py and suggest improvements" --allowedTools Read --max-turns 3
```

**What to observe**:
- Claude's response should reflect awareness of the security guidelines
- The injected context influences Claude to mention input validation, environment variables, etc.

### Step 7: Verify logs

```bash
cat ~/.claude/logs/rulez.log | tail -5
```

Look for:
- `"inject-security-context"` rule name
- Evidence of context injection (injected file path in the log)

### Step 8: Cleanup

```bash
rm -rf /tmp/rulez-test-inject
```

### Pass/Fail Criteria

| Check | Expected |
|-------|----------|
| `rulez debug pre --tool Read --path "app.py"` | Allowed with injected context |
| `rulez debug pre --tool Read --path "README.md"` | Allowed (no matching rules) |
| Log contains `inject-security-context` after live test | Yes |
| Claude's response mentions security practices | Yes (soft check) |

---

## Use Case 3: Debug Dry-Run

**Goal**: Verify the `rulez debug` command can simulate all event types and produce useful diagnostic output.

This use case does not require Claude Code -- it tests the debug tooling itself.

### Step 1: Create test project

```bash
mkdir -p /tmp/rulez-test-debug/.claude
cd /tmp/rulez-test-debug
```

### Step 2: Write hooks.yaml with multiple rules

```bash
cat > .claude/hooks.yaml << 'EOF'
version: "1.0"

rules:
  - name: block-rm-rf
    description: "Block recursive deletion commands"
    matchers:
      tools: ["Bash"]
      command_match: "rm\\s+-rf\\s+/"
    actions:
      block: true

  - name: inject-on-edit
    description: "Inject coding standards when editing files"
    matchers:
      tools: ["Edit", "Write"]
    actions:
      inject_inline: "Follow the project coding standards: use 4-space indentation, add docstrings to all public functions."

  - name: prompt-guard
    description: "Warn on prompts mentioning credentials"
    matchers:
      prompt_match:
        pattern: "password|secret|credential|api.key"
    actions:
      inject_inline: "SECURITY NOTICE: Be careful with credential-related operations. Never output secrets in plain text."

settings:
  log_level: "debug"
  debug_logs: true
EOF
```

### Step 3: Test each event type

**PreToolUse - Block test**:
```bash
$RULEZ_BIN debug pre --tool Bash --command "rm -rf /important-data"
```
Expected: `Blocked`

**PreToolUse - Allow test**:
```bash
$RULEZ_BIN debug pre --tool Bash --command "ls -la"
```
Expected: `Allowed (no matching rules)`

**PreToolUse - Inject test**:
```bash
$RULEZ_BIN debug pre --tool Edit --path "main.py"
```
Expected: `Allowed with injected context`

**UserPromptSubmit - Prompt guard test**:
```bash
$RULEZ_BIN debug prompt --prompt "Show me the database password"
```
Expected: `Allowed with injected context` (the security notice)

**UserPromptSubmit - Clean prompt**:
```bash
$RULEZ_BIN debug prompt --prompt "List all files in the project"
```
Expected: `Allowed (no matching rules)`

**SessionStart**:
```bash
$RULEZ_BIN debug session
```
Expected: Runs without error, shows `Allowed`

**PostToolUse**:
```bash
$RULEZ_BIN debug post --tool Bash --command "echo hello"
```
Expected: Runs without error

### Step 4: Test JSON output mode

```bash
$RULEZ_BIN debug pre --tool Bash --command "rm -rf /data" --json
```

Verify the JSON output has this structure:
```json
{
  "outcome": "Block",
  "reason": "...",
  "matchedRules": ["block-rm-rf"],
  "evaluationTimeMs": ...,
  "evaluations": [
    {
      "ruleName": "block-rm-rf",
      "matched": true,
      "timeMs": ...
    }
  ]
}
```

### Step 5: Test verbose mode

```bash
$RULEZ_BIN debug pre --tool Edit --path "main.py" --verbose
```

Look for additional output showing all rule names and their configurations.

### Step 6: Cleanup

```bash
rm -rf /tmp/rulez-test-debug
```

### Pass/Fail Criteria

| Check | Expected |
|-------|----------|
| `debug pre` with blocking command | Blocked |
| `debug pre` with safe command | Allowed |
| `debug pre` with Edit tool | Allowed with injected context |
| `debug prompt` with credential mention | Allowed with injected context |
| `debug prompt` with clean prompt | Allowed (no matching rules) |
| `debug session` | Completes without error |
| `debug post` | Completes without error |
| `--json` flag produces valid JSON | Yes |
| `--verbose` flag shows extra detail | Yes |

---

## Troubleshooting

### `rulez install` says "already installed"

```bash
$RULEZ_BIN uninstall
$RULEZ_BIN install
```

### No log entries appear

1. Check that `debug_logs: true` is set in hooks.yaml
2. Verify the log path: `ls -la ~/.claude/logs/rulez.log`
3. Run `$RULEZ_BIN validate` to check configuration

### `rulez debug` says "No configuration found"

Make sure you are in the test project directory that contains `.claude/hooks.yaml`:

```bash
cd /tmp/rulez-test-block
$RULEZ_BIN debug pre --tool Bash --command "test"
```

### Claude refuses the command on its own (before RuleZ)

This is expected for dangerous commands. Claude has its own safety checks. The key verification is that `rulez debug` shows `Blocked` -- this confirms RuleZ would have blocked it regardless of Claude's decision.
