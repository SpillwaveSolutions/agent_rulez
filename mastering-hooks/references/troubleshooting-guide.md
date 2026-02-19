# RuleZ Troubleshooting Guide

Systematic procedures for diagnosing and fixing RuleZ issues.

## Quick Diagnostic Checklist

Run these commands in order when hooks aren't working:

```bash
# 1. Is RuleZ installed?
rulez --version

# 2. Is config valid?
rulez validate

# 3. Is RuleZ registered with Claude Code?
cat .claude/settings.json | grep -A10 hooks

# 4. What rules exist?
rulez explain config

# 5. Debug specific event
rulez debug PreToolUse --tool Write --path test.py -v
```

---

## Common Issues

### Issue: Hooks Not Firing

**Symptoms**: Claude Code runs tools without triggering any hooks.

**Diagnostic steps**:

1. **Check registration**:
   ```bash
   cat .claude/settings.json
   ```
   Look for the nested matcher/hooks structure:
   ```json
   {
     "hooks": {
       "PreToolUse": [{ "matcher": "*", "hooks": [{ "type": "command", "command": "/path/to/rulez", "timeout": 5 }] }],
       "PostToolUse": [{ "matcher": "*", "hooks": [{ "type": "command", "command": "/path/to/rulez", "timeout": 5 }] }],
       "Stop": [{ "matcher": "*", "hooks": [{ "type": "command", "command": "/path/to/rulez", "timeout": 5 }] }],
       "SessionStart": [{ "matcher": "*", "hooks": [{ "type": "command", "command": "/path/to/rulez", "timeout": 5 }] }]
     }
   }
   ```

2. **Re-register if missing**:
   ```bash
   rulez install --project
   ```

3. **Check config location**:
   ```bash
   ls -la .claude/hooks.yaml
   ```

4. **Verify config is valid**:
   ```bash
   rulez validate
   ```

**Common causes**:
- RuleZ not registered (run `rulez install`)
- hooks.yaml in wrong location
- YAML syntax error preventing load

---

### Issue: Specific Rule Not Matching

**Symptoms**: One rule doesn't trigger while others work.

**Diagnostic steps**:

1. **Debug the specific event**:
   ```bash
   rulez debug PreToolUse --tool Write --path src/main.py -vv
   ```

2. **Check rule definition**:
   ```bash
   rulez explain rule <rule-name>
   ```

3. **Verify matchers**:
   - `tools`: Exact names (`Write` not `write`)
   - `extensions`: Include dot (`.py` not `py`)
   - `directories`: Use forward slash (`src/` not `src\`)

**Common causes**:

| Issue | Bad | Good |
|-------|-----|------|
| Tool case | `tools: [write]` | `tools: [Write]` |
| Extension format | `extensions: [py]` | `extensions: [.py]` |
| Directory slash | `directories: [src]` | `directories: [src/]` |
| Regex escaping | `command_match: "file.py"` | `command_match: "file\\.py"` |

---

### Issue: "File Not Found" Error

**Symptoms**: Error message about missing file in action path.

**Diagnostic steps**:

1. **Check the file exists**:
   ```bash
   ls -la .claude/context/your-file.md
   ```

2. **Verify path is relative to project root**:
   ```yaml
   # Correct (relative to project root)
   path: .claude/context/standards.md

   # Wrong (absolute path)
   path: /Users/me/project/.claude/context/standards.md
   ```

3. **Check for typos in path**:
   ```bash
   rulez explain rule <rule-name> | grep path
   ```

**Resolution**:
- Create missing file
- Fix path in hooks.yaml
- Use `source: inline` for simple content

---

### Issue: Script Returns Invalid Output

**Symptoms**: "Invalid JSON" or "Unexpected script output" errors.

**Diagnostic steps**:

1. **Test script directly**:
   ```bash
   .claude/validators/your-script.sh
   ```

2. **Verify output format**:
   ```bash
   .claude/validators/your-script.sh | jq .
   ```

   Must output valid JSON:
   ```json
   {"continue": true, "context": "", "reason": ""}
   ```

3. **Check for stderr pollution**:
   ```bash
   .claude/validators/your-script.sh 2>&1
   ```

**Common causes**:
- Script prints to stdout before JSON
- Script outputs to stderr (captured in output)
- Missing quotes in JSON
- Non-zero exit code with no output

**Fix template**:
```bash
#!/bin/bash
# Suppress all output except final JSON
exec 2>/dev/null  # Suppress stderr

# Your logic here...

# Always output valid JSON
echo '{"continue": true, "context": "Done", "reason": ""}'
```

---

### Issue: Permission Denied on Script

**Symptoms**: "Permission denied" when running action script.

**Resolution**:
```bash
chmod +x .claude/validators/your-script.sh
```

**Prevention**: Always set executable bit when creating scripts:
```bash
touch .claude/validators/new-script.sh
chmod +x .claude/validators/new-script.sh
```

---

### Issue: Script Timeout

**Symptoms**: "Script exceeded timeout" error.

**Diagnostic steps**:

1. **Check script execution time**:
   ```bash
   time .claude/validators/slow-script.sh
   ```

2. **Increase timeout in config**:
   ```yaml
   action:
     type: run
     command: .claude/validators/slow-script.sh
     timeout: 60  # Increase from default 30
   ```

3. **Optimize script** if timeout is already high.

**Common causes**:
- Network calls in script
- Large file processing
- Waiting for user input (scripts must be non-interactive)

---

### Issue: YAML Syntax Error

**Symptoms**: `rulez validate` fails with parse error.

**Diagnostic steps**:

1. **Validate YAML syntax**:
   ```bash
   python -c "import yaml; yaml.safe_load(open('.claude/hooks.yaml'))"
   ```

2. **Check for common issues**:
   - Incorrect indentation (use 2 spaces, not tabs)
   - Missing quotes around special characters
   - Incorrect list format

**Common YAML mistakes**:

```yaml
# Wrong: tabs instead of spaces
hooks:
	- name: bad-indent  # TAB character

# Wrong: missing quotes on regex
match:
  command_match: .*force.*  # Needs quotes

# Wrong: missing dash for list item
hooks:
  name: missing-dash  # Should be "- name:"

# Correct
hooks:
  - name: correct-rule
    match:
      command_match: ".*force.*"
```

---

### Issue: enabled_when Not Working

**Symptoms**: Conditional rules always or never match.

**Diagnostic steps**:

1. **Check expression syntax**:
   ```bash
   rulez explain rule <rule-name>
   ```

2. **Verify variable availability**:
   ```bash
   rulez debug PreToolUse --tool Write --path test.py -vvv
   ```
   Look for available context variables.

3. **Test expression logic**:
   ```yaml
   # Debug by adding a simple always-true rule
   - name: debug-enabled-when
     event: PreToolUse
     match:
       enabled_when: "true"
     action:
       type: inject
       source: inline
       content: "Debug: enabled_when evaluated"
   ```

**Common mistakes**:

```yaml
# Wrong: using = instead of ==
enabled_when: "env.CI = 'true'"

# Wrong: missing quotes around string
enabled_when: "env.CI == true"

# Wrong: wrong variable path
enabled_when: "CI == 'true'"  # Should be env.CI

# Correct
enabled_when: "env.CI == 'true'"
```

---

### Issue: Context Not Appearing

**Symptoms**: inject action runs but context not visible to the AI assistant.

**Diagnostic steps**:

1. **Verify injection happened**:
   ```bash
   rulez logs --tail 5
   ```
   Look for "injected X bytes context"

2. **Check file content**:
   ```bash
   cat .claude/context/your-file.md
   ```

3. **Verify file is not empty**:
   ```bash
   wc -l .claude/context/your-file.md
   ```

**Common causes**:
- File exists but is empty
- File has wrong encoding (use UTF-8)
- Context too large (check for size limits)

---

### Issue: "missing field `event_type`" Parse Error

**Symptoms**: Every hook call fails with `hook error` and logs show `missing field 'event_type'`.

**Root cause**: Claude Code sends events with the field name `hook_event_name`, not `event_type`. If your RuleZ binary expects `event_type`, it can't parse the JSON.

**Resolution**:
1. Update RuleZ binary to the latest version which accepts both `hook_event_name` and `event_type` (via serde alias)
2. Rebuild and reinstall:
   ```bash
   cargo install --path rulez
   rulez install
   ```

**Protocol reference**: Claude Code's JSON event format:
```json
{
  "hook_event_name": "PreToolUse",
  "session_id": "abc123",
  "tool_name": "Bash",
  "tool_input": {"command": "git status"},
  "cwd": "/path/to/project",
  "transcript_path": "/path/to/transcript",
  "permission_mode": "default",
  "tool_use_id": "toolu_xxx"
}
```

Note: Claude Code does **not** send a `timestamp` field. RuleZ defaults to `Utc::now()`.

---

### Issue: Events Not Firing on Non-Claude Platforms

**Symptoms**: Rules work on Claude Code but not on Gemini CLI, Copilot, or OpenCode.

**Diagnostic steps**:

1. **Check platform support**: Not all events exist on all platforms. See [platform-adapters.md](platform-adapters.md).

2. **Common platform gaps**:

   | Event | Claude Code | Gemini | Copilot | OpenCode |
   |-------|-------------|--------|---------|----------|
   | `BeforeAgent` | Yes | Yes (dual) | No | No |
   | `AfterAgent` | Yes | Yes | No | No |
   | `BeforeModel` | No | Yes | No | No |
   | `PermissionRequest` | Yes | Via dual-fire | No | No |
   | `Stop` | Yes | No | No | No |

3. **Use universal events** for cross-platform rules: `PreToolUse`, `PostToolUse`, `SessionStart`, `SessionEnd`, `PreCompact`, `UserPromptSubmit`.

4. **Check dual-fire behavior**: On Gemini, `BeforeAgent` also fires `UserPromptSubmit`. If you have rules on both, both will trigger.

---

## Debugging Workflow

### Step-by-Step Debug Process

1. **Isolate the problem**:
   ```bash
   # Create minimal test rule
   cat > .claude/hooks-test.yaml << 'EOF'
   version: "1"
   hooks:
     - name: test-rule
       event: PreToolUse
       match:
         tools: [Write]
       action:
         type: inject
         source: inline
         content: "TEST: Hook fired!"
   EOF

   rulez validate --config .claude/hooks-test.yaml
   ```

2. **Test with debug command**:
   ```bash
   rulez debug PreToolUse --tool Write --path test.txt -vv
   ```

3. **Check logs**:
   ```bash
   rulez logs --tail 20 --json | jq .
   ```

4. **Incrementally add complexity** until you find what breaks.

---

## Log Analysis

### Reading Log Output

```bash
rulez logs --tail 10
```

**Log entry format**:
```
TIMESTAMP | EVENT | RULE_NAME | STATUS
  Details...
```

**Status meanings**:
- `matched`: Rule matched and action executed
- `skipped`: Rule didn't match
- `blocked`: Action blocked tool execution
- `error`: Action failed

### Filtering Logs

```bash
# Only errors
rulez logs --status error

# Specific rule
rulez logs --rule python-standards

# Last hour
rulez logs --since 1h

# JSON for parsing
rulez logs --json | jq 'select(.status == "error")'
```

---

## Getting Help

If you've tried the above and still have issues:

1. **Gather diagnostic info**:
   ```bash
   rulez --version --json > rulez-debug.txt
   rulez validate >> rulez-debug.txt 2>&1
   cat .claude/hooks.yaml >> rulez-debug.txt
   rulez logs --tail 50 --json >> rulez-debug.txt
   ```

2. **Check for known issues** in project documentation

3. **Create minimal reproduction** with test config

4. **Include in bug report**:
   - RuleZ version
   - OS and version
   - hooks.yaml content
   - Expected vs actual behavior
   - Debug command output
   - Platform (Claude Code, Gemini, Copilot, OpenCode)
