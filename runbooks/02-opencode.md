# Runbook 02: OpenCode Manual Testing

Manual testing of RuleZ with OpenCode CLI.

## Prerequisites

- [ ] OpenCode CLI installed (`opencode --version`)
- [ ] RuleZ binary built (`cargo build --release` from repo root)
- [ ] Know your RuleZ binary path

```bash
export RULEZ_BIN="/path/to/agent_rulez/target/release/rulez"
```

## Platform Notes

OpenCode uses different event names than Claude Code:

| OpenCode Event | Claude Code Equivalent |
|---------------|----------------------|
| `tool.execute.before` | `PreToolUse` |
| `tool.execute.after` | `PostToolUse` |
| `file.edited` | (file change hook) |
| `session.updated` | `SessionStart` |

RuleZ maps these automatically via the `rulez opencode hook` adapter command.

The OpenCode settings file is at `.opencode/settings.json` (project) or `~/.opencode/settings.json` (user).

---

## Use Case 1: Block Force Push

### Step 1: Create test project

```bash
mkdir -p /tmp/rulez-opencode-test/.claude
cd /tmp/rulez-opencode-test
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

### Step 3: Install for OpenCode

```bash
$RULEZ_BIN opencode install
```

Expected output:
```
Installing OpenCode hooks...

  Binary: /path/to/rulez
  Config: .opencode/settings.json
  Scope: project

✓ OpenCode hooks installed successfully!

Hook registered for events:
  * file.edited
  * tool.execute.before
  * tool.execute.after
  * session.updated
```

### Step 4: Verify with doctor

```bash
$RULEZ_BIN opencode doctor
```

This checks that the OpenCode settings.json is correctly configured.

### Step 5: Dry-run with `rulez debug`

The `rulez debug` command simulates Claude Code events, which is useful for verifying rule logic even when testing other platforms. The rule matching logic is identical across platforms.

```bash
# Should be BLOCKED
$RULEZ_BIN debug pre --tool Bash --command "git push --force origin main"

# Should be ALLOWED
$RULEZ_BIN debug pre --tool Bash --command "echo hello"
```

### Step 6: Live test with OpenCode

```bash
cd /tmp/rulez-opencode-test
opencode
```

In the OpenCode session, ask it to run `git push --force origin main`.

**What to observe**:
- RuleZ should intercept the `tool.execute.before` event
- The operation should be blocked

### Step 7: Verify logs

```bash
cat ~/.opencode/logs/rulez.log | tail -5
```

### Step 8: Cleanup

```bash
rm -rf /tmp/rulez-opencode-test
```

### Pass/Fail Criteria

| Check | Expected |
|-------|----------|
| `rulez opencode install` succeeds | Yes |
| `rulez opencode doctor` passes | Yes |
| `rulez debug pre` with force push | Blocked |
| Force push blocked in live OpenCode session | Yes |

---

## Use Case 2: Context Injection

### Step 1: Create test project

```bash
mkdir -p /tmp/rulez-opencode-inject/.claude/context
cd /tmp/rulez-opencode-inject
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
$RULEZ_BIN opencode install
$RULEZ_BIN opencode doctor
```

### Step 5: Dry-run

```bash
$RULEZ_BIN debug pre --tool Read --path "app.py"
```

Expected: `Allowed with injected context`

### Step 6: Live test

```bash
cd /tmp/rulez-opencode-inject
opencode
```

Ask OpenCode to read and improve `app.py`. The security context should influence its suggestions.

### Step 7: Cleanup

```bash
rm -rf /tmp/rulez-opencode-inject
```

---

## Use Case 3: Debug Dry-Run

Same as [01-claude-code.md Use Case 3](01-claude-code.md#use-case-3-debug-dry-run). The `rulez debug` command is platform-independent. Follow the steps in that runbook.

---

## Troubleshooting

### `rulez opencode doctor` reports issues

- Check that `.opencode/settings.json` exists and contains the hook entries
- Re-run `$RULEZ_BIN opencode install`

### OpenCode doesn't trigger hooks

- Verify OpenCode version supports the hooks API
- Check `~/.opencode/logs/rulez.log` for any entries
- Run `$RULEZ_BIN opencode doctor --json` for detailed diagnostics
