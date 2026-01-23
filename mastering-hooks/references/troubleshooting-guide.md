# CCH Troubleshooting Guide

Systematic procedures for diagnosing and fixing CCH issues.

## Quick Diagnostic Checklist

Run these commands in order when hooks aren't working:

```bash
# 1. Is CCH installed?
cch --version

# 2. Is config valid?
cch validate

# 3. Is CCH registered with Claude Code?
cat .claude/settings.json | grep -A10 hooks

# 4. What rules exist?
cch explain config

# 5. Debug specific event
cch debug PreToolUse --tool Write --path test.py -v
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
   Look for:
   ```json
   {
     "hooks": {
       "PreToolUse": "cch run-hook PreToolUse",
       "PostToolUse": "cch run-hook PostToolUse"
     }
   }
   ```

2. **Re-register if missing**:
   ```bash
   cch install --project
   ```

3. **Check config location**:
   ```bash
   ls -la .claude/hooks.yaml
   ```

4. **Verify config is valid**:
   ```bash
   cch validate
   ```

**Common causes**:
- CCH not registered (run `cch install`)
- hooks.yaml in wrong location
- YAML syntax error preventing load

---

### Issue: Specific Rule Not Matching

**Symptoms**: One rule doesn't trigger while others work.

**Diagnostic steps**:

1. **Debug the specific event**:
   ```bash
   cch debug PreToolUse --tool Write --path src/main.py -vv
   ```

2. **Check rule definition**:
   ```bash
   cch explain rule <rule-name>
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
   cch explain rule <rule-name> | grep path
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

**Symptoms**: `cch validate` fails with parse error.

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
   cch explain rule <rule-name>
   ```

2. **Verify variable availability**:
   ```bash
   cch debug PreToolUse --tool Write --path test.py -vvv
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

**Symptoms**: inject action runs but context not visible to Claude.

**Diagnostic steps**:

1. **Verify injection happened**:
   ```bash
   cch logs --tail 5
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
   
   cch validate --config .claude/hooks-test.yaml
   ```

2. **Test with debug command**:
   ```bash
   cch debug PreToolUse --tool Write --path test.txt -vv
   ```

3. **Check logs**:
   ```bash
   cch logs --tail 20 --json | jq .
   ```

4. **Incrementally add complexity** until you find what breaks.

---

## Log Analysis

### Reading Log Output

```bash
cch logs --tail 10
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
cch logs --status error

# Specific rule
cch logs --rule python-standards

# Last hour
cch logs --since 1h

# JSON for parsing
cch logs --json | jq 'select(.status == "error")'
```

---

## Getting Help

If you've tried the above and still have issues:

1. **Gather diagnostic info**:
   ```bash
   cch --version --json > cch-debug.txt
   cch validate >> cch-debug.txt 2>&1
   cat .claude/hooks.yaml >> cch-debug.txt
   cch logs --tail 50 --json >> cch-debug.txt
   ```

2. **Check for known issues** in project documentation

3. **Create minimal reproduction** with test config

4. **Include in bug report**:
   - CCH version
   - OS and version
   - hooks.yaml content
   - Expected vs actual behavior
   - Debug command output
