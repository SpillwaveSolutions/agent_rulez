# Runbook 03: Gemini CLI Manual Testing

Manual testing of RuleZ with Gemini CLI.

## Prerequisites

- [ ] Gemini CLI installed (`gemini --version`)
- [ ] RuleZ binary built (`cargo build --release` from repo root)
- [ ] Know your RuleZ binary path

```bash
export RULEZ_BIN="/path/to/agent_rulez/target/release/rulez"
```

## Platform Notes

Gemini CLI has the richest event model of all supported platforms (11 event types):

| Gemini Event | Description |
|-------------|-------------|
| `BeforeTool` | Before a tool executes (similar to `PreToolUse`) |
| `AfterTool` | After a tool executes (similar to `PostToolUse`) |
| `BeforeAgent` | Before an agent turn |
| `AfterAgent` | After an agent turn |
| `BeforeModel` | Before a model call |
| `AfterModel` | After a model call |
| `BeforeToolSelection` | Before tool selection |
| `SessionStart` | Session begins |
| `SessionEnd` | Session ends |
| `Notification` | General notifications |
| `PreCompact` | Before context compaction |

RuleZ maps these automatically via the `rulez gemini hook` adapter command.

The Gemini settings file is at `.gemini/settings.json` (project), `~/.gemini/settings.json` (user), or system-level.

---

## Use Case 1: Block Force Push

### Step 1: Create test project

```bash
mkdir -p /tmp/rulez-gemini-test/.claude
cd /tmp/rulez-gemini-test
git init
```

Note: hooks.yaml is always in `.claude/hooks.yaml` regardless of the platform.

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

settings:
  log_level: "debug"
  debug_logs: true
  fail_open: false
EOF
```

### Step 3: Install for Gemini

```bash
$RULEZ_BIN gemini install
```

Expected output:
```
Installing Gemini CLI hooks...

  Binary: /path/to/rulez
  Config: .gemini/settings.json
  Scope: project

✓ Gemini hooks installed successfully!

Hook registered for events:
  * BeforeTool
  * AfterTool
  * BeforeAgent
  * AfterAgent
  * BeforeModel
  * AfterModel
  * BeforeToolSelection
  * SessionStart
  * SessionEnd
  * Notification
  * PreCompact
```

### Step 4: Verify with doctor

```bash
$RULEZ_BIN gemini doctor
```

### Step 5: Dry-run with `rulez debug`

```bash
# Should be BLOCKED
$RULEZ_BIN debug pre --tool Bash --command "git push --force origin main"

# Should be ALLOWED
$RULEZ_BIN debug pre --tool Bash --command "echo hello"
```

### Step 6: Live test with Gemini CLI

```bash
cd /tmp/rulez-gemini-test
gemini
```

In the Gemini session, ask it to run `git push --force origin main`.

**What to observe**:
- RuleZ should intercept the `BeforeTool` event
- The operation should be blocked

### Step 7: Verify logs

```bash
cat ~/.gemini/logs/rulez.log | tail -5
```

### Step 8: Cleanup

```bash
rm -rf /tmp/rulez-gemini-test
```

### Pass/Fail Criteria

| Check | Expected |
|-------|----------|
| `rulez gemini install` succeeds | Yes |
| `rulez gemini doctor` passes | Yes |
| `rulez debug pre` with force push | Blocked |
| Force push blocked in live Gemini session | Yes |

---

## Use Case 2: Context Injection

### Step 1: Create test project

```bash
mkdir -p /tmp/rulez-gemini-inject/.claude/context
cd /tmp/rulez-gemini-inject
```

### Step 2: Create sample files

```bash
cat > .claude/context/security-guidelines.md << 'EOF'
# Security Guidelines

- Never store secrets in source code
- Always use environment variables for API keys
- Validate all user input before processing
EOF

cat > app.py << 'EOF'
import os

def get_api_key():
    return os.environ.get("API_KEY", "")
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

### Step 4: Install and verify

```bash
$RULEZ_BIN gemini install
$RULEZ_BIN gemini doctor
```

### Step 5: Dry-run

```bash
$RULEZ_BIN debug pre --tool Read --path "app.py"
```

Expected: `Allowed with injected context`

### Step 6: Live test

```bash
cd /tmp/rulez-gemini-inject
gemini
```

Ask Gemini to read and improve `app.py`. The security context should influence its suggestions.

### Step 7: Cleanup

```bash
rm -rf /tmp/rulez-gemini-inject
```

---

## Use Case 3: Debug Dry-Run

Same as [01-claude-code.md Use Case 3](01-claude-code.md#use-case-3-debug-dry-run). The `rulez debug` command is platform-independent. Follow the steps in that runbook.

---

## Troubleshooting

### `rulez gemini doctor` reports issues

- Check that `.gemini/settings.json` exists and contains the hook entries
- Re-run `$RULEZ_BIN gemini install`

### Gemini CLI doesn't trigger hooks

- Verify your Gemini CLI version supports the hooks/extensions API
- Check `~/.gemini/logs/rulez.log` for any entries
- Run `$RULEZ_BIN gemini doctor --json` for detailed diagnostics

### Gemini uses different tool names

Gemini CLI may use different tool names than Claude Code. If rules don't match, check what tool names Gemini reports:

```bash
# Check logs for actual tool names
cat ~/.gemini/logs/rulez.log | grep tool_name
```

Adjust your `matchers.tools` values accordingly.
