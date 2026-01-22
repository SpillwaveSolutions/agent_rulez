# Hook Logger Enhancements: Enhanced Argument Capture

**Version:** 2.0
**Date:** 2025-11-20
**Status:** ✅ Production Ready

---

## Overview

The enhanced hook logger captures detailed arguments for all major Claude Code tools, providing rich context in automated TOOL log entries without increasing conversation tokens.

**Key Improvement:** Instead of just logging tool names, we now capture:
- Command details and descriptions (Bash)
- File paths with offsets/limits (Read)
- Content sizes (Write)
- Edit operations with old/new strings (Edit)
- Search patterns with modes and paths (Grep)
- Glob patterns (Glob)
- Todo status summaries (TodoWrite)

---

## Enhanced Tool Support

### 1. Bash Tool
**Captures:** Command + Description

**Input JSON:**
```json
{
  "tool_name": "Bash",
  "tool_input": {
    "command": "./gradlew test --info 2>&1 | grep -A 10 FAILED",
    "description": "Get detailed test failure info"
  }
}
```

**Log Entry:**
```
---
[12:48:56] TOOL: Bash
COMMAND: ./gradlew test --info 2>&1 | grep -A 10 FAILED
DESC: Get detailed test failure info
---
```

**Why This Matters:**
- DESC field makes logs human-readable
- Complex commands are fully captured (up to 200 chars)
- Can see what the command is trying to accomplish

### 2. Read Tool
**Captures:** File path + Offset + Limit

**Input JSON:**
```json
{
  "tool_name": "Read",
  "tool_input": {
    "file_path": "/path/to/large-file.log",
    "offset": 100,
    "limit": 50
  }
}
```

**Log Entry:**
```
---
[12:50:15] TOOL: Read
FILE: /path/to/large-file.log
OFFSET: 100
LIMIT: 50
---
```

**Use Cases:**
- Pagination tracking (which part of file was read)
- Debugging read operations
- Understanding file access patterns

### 3. Write Tool
**Captures:** File path + Content size

**Input JSON:**
```json
{
  "tool_name": "Write",
  "tool_input": {
    "file_path": "/path/to/output.txt",
    "content": "... (large content) ..."
  }
}
```

**Log Entry:**
```
---
[12:51:22] TOOL: Write
FILE: /path/to/output.txt
SIZE: 1234 chars
---
```

**Why This Matters:**
- Know which files were created/overwritten
- Track content size (helps identify large writes)
- No need to log full content (saves tokens)

### 4. Edit Tool
**Captures:** File path + Old/New strings + Replace-all flag

**Input JSON:**
```json
{
  "tool_name": "Edit",
  "tool_input": {
    "file_path": "/path/to/code.ts",
    "old_string": "const oldValue = 123;",
    "new_string": "const newValue = 456;",
    "replace_all": false
  }
}
```

**Log Entry:**
```
---
[12:52:30] TOOL: Edit
FILE: /path/to/code.ts
OLD: const oldValue = 123;...
NEW: const newValue = 456;...
---
```

**With Replace-All:**
```
---
[12:52:45] TOOL: Edit
FILE: /path/to/code.ts
OLD: oldName...
NEW: newName...
REPLACE_ALL: True
---
```

**Use Cases:**
- See what changed in file edits
- Track replace-all operations (rename refactorings)
- Debugging edit failures

### 5. Grep Tool
**Captures:** Pattern + Path + Glob + Mode + Case-insensitive flag

**Input JSON:**
```json
{
  "tool_name": "Grep",
  "tool_input": {
    "pattern": "PostToolUse",
    "path": "/path/to/search",
    "glob": "*.json",
    "output_mode": "content",
    "-i": true
  }
}
```

**Log Entry:**
```
---
[12:53:10] TOOL: Grep
PATTERN: PostToolUse
PATH: /path/to/search
GLOB: *.json
MODE: content
CASE_INSENSITIVE: True
---
```

**Why This Matters:**
- Understand search scope (pattern + path + glob)
- Know output mode (content vs files_with_matches vs count)
- Track case-sensitive vs case-insensitive searches

### 6. Glob Tool
**Captures:** Pattern + Path

**Input JSON:**
```json
{
  "tool_name": "Glob",
  "tool_input": {
    "pattern": "**/*.py",
    "path": "/path/to/project"
  }
}
```

**Log Entry:**
```
---
[12:54:20] TOOL: Glob
PATTERN: **/*.py
PATH: /path/to/project
---
```

**Use Cases:**
- Track file discovery operations
- See search patterns used
- Debug glob matching issues

### 7. TodoWrite Tool
**Captures:** Todo count + Status summary

**Input JSON:**
```json
{
  "tool_name": "TodoWrite",
  "tool_input": {
    "todos": [
      {"content": "Task 1", "status": "completed"},
      {"content": "Task 2", "status": "completed"},
      {"content": "Task 3", "status": "in_progress"},
      {"content": "Task 4", "status": "pending"}
    ]
  }
}
```

**Log Entry:**
```
---
[12:55:30] TOOL: TodoWrite
TODOS: 4 items
STATUS: 2 done, 1 active, 1 pending
---
```

**Why This Matters:**
- Track progress through tasks
- See todo list updates
- No need to log full todo content

---

## Implementation

### Hook Logger Script

**Location:** `.claude/hook-logger.py`

**Core Logic:**
```python
#!/usr/bin/env python3
import json
import sys
from datetime import datetime
import os

try:
    # Read JSON from stdin
    data = json.load(sys.stdin)

    # Get current log file
    current_log_file = os.path.join(
        os.environ.get('CLAUDE_PROJECT_DIR', ''),
        'debugging',
        'current_log_file.txt'
    )

    if os.path.exists(current_log_file):
        with open(current_log_file, 'r') as f:
            log_file_path = f.read().strip()
    else:
        sys.exit(0)  # No active session

    if not log_file_path or not os.path.exists(log_file_path):
        sys.exit(0)  # Invalid log file

    # Extract tool information
    tool_name = data.get('tool_name', 'Unknown')
    tool_input = data.get('tool_input', {})

    # Format log entry
    timestamp = datetime.now().strftime('%H:%M:%S')
    log_entry = f"\n---\n[{timestamp}] TOOL: {tool_name}"

    # Tool-specific formatting...
    # (see complete script for all tool handlers)

    log_entry += "\n---"

    # Write to log
    with open(log_file_path, 'a') as f:
        f.write(log_entry + '\n')

    sys.exit(0)

except Exception as e:
    # Silent failure - don't break hooks
    sys.exit(0)
```

### Tool-Specific Handlers

**Bash Handler:**
```python
if tool_name == 'Bash':
    command = tool_input.get('command', '')
    description = tool_input.get('description', 'No description')
    log_entry += f"\nCOMMAND: {command[:200]}"  # First 200 chars
    log_entry += f"\nDESC: {description}"
```

**Read Handler:**
```python
elif tool_name == 'Read':
    file_path = tool_input.get('file_path', '')
    offset = tool_input.get('offset')
    limit = tool_input.get('limit')
    log_entry += f"\nFILE: {file_path}"
    if offset is not None:
        log_entry += f"\nOFFSET: {offset}"
    if limit is not None:
        log_entry += f"\nLIMIT: {limit}"
```

**Write Handler:**
```python
elif tool_name == 'Write':
    file_path = tool_input.get('file_path', '')
    content = tool_input.get('content', '')
    log_entry += f"\nFILE: {file_path}"
    log_entry += f"\nSIZE: {len(content)} chars"
```

**Edit Handler:**
```python
elif tool_name == 'Edit':
    file_path = tool_input.get('file_path', '')
    old_string = tool_input.get('old_string', '')
    new_string = tool_input.get('new_string', '')
    replace_all = tool_input.get('replace_all', False)
    log_entry += f"\nFILE: {file_path}"
    log_entry += f"\nOLD: {old_string[:100]}..."
    log_entry += f"\nNEW: {new_string[:100]}..."
    if replace_all:
        log_entry += f"\nREPLACE_ALL: True"
```

**Grep Handler:**
```python
elif tool_name == 'Grep':
    pattern = tool_input.get('pattern', '')
    path = tool_input.get('path', '')
    output_mode = tool_input.get('output_mode', 'files_with_matches')
    glob = tool_input.get('glob', '')
    case_insensitive = tool_input.get('-i', False)
    log_entry += f"\nPATTERN: {pattern}"
    if path:
        log_entry += f"\nPATH: {path}"
    if glob:
        log_entry += f"\nGLOB: {glob}"
    log_entry += f"\nMODE: {output_mode}"
    if case_insensitive:
        log_entry += f"\nCASE_INSENSITIVE: True"
```

**Glob Handler:**
```python
elif tool_name == 'Glob':
    pattern = tool_input.get('pattern', '')
    path = tool_input.get('path', '')
    log_entry += f"\nPATTERN: {pattern}"
    if path:
        log_entry += f"\nPATH: {path}"
```

**TodoWrite Handler:**
```python
elif tool_name == 'TodoWrite':
    todos = tool_input.get('todos', [])
    log_entry += f"\nTODOS: {len(todos)} items"
    # Show status summary
    completed = sum(1 for t in todos if t.get('status') == 'completed')
    in_progress = sum(1 for t in todos if t.get('status') == 'in_progress')
    pending = sum(1 for t in todos if t.get('status') == 'pending')
    log_entry += f"\nSTATUS: {completed} done, {in_progress} active, {pending} pending"
```

---

## Installation

### Copy Enhanced Hook Logger

```bash
# From architect-agent skill templates
cp ~/.claude/skills/architect-agent/templates/code-agent-workspace/.claude/hook-logger.py .claude/

# Make executable
chmod +x .claude/hook-logger.py

# Verify
ls -la .claude/hook-logger.py
```

### Verify Enhanced Logging

```bash
# Start log session
./debugging/scripts/log-start.sh "enhancement-test"

# Run various tool types
ls -la
grep "test" README.md
echo "test" > /tmp/test.txt

# Check for enhanced entries
tail -50 $(cat debugging/current_log_file.txt)

# Should see detailed TOOL entries with arguments
```

---

## Example Log Output

### Real Session Example

```
---
[12:28:25] TOOL: Bash
COMMAND: pwd
DESC: Print working directory
---

---
[12:28:25] TOOL: Bash
COMMAND: ls -la
DESC: List directory contents with details
---

---
[12:47:52] TOOL: Read
FILE: /Users/.../docs/testing_protocol.md
---

---
[12:48:56] TOOL: Bash
COMMAND: cat services/evinexus/build/reports/tests/test/index.html 2>/dev/null | grep -A 5 "failed" | head -30 || echo "Report file not found"
DESC: Check test failure report
---

---
[12:49:29] TOOL: Bash
COMMAND: ./gradlew test --info 2>&1 | grep -A 10 "FAILED" | head -100
DESC: Get detailed test failure info
---

---
[12:49:40] TOOL: TodoWrite
TODOS: 7 items
STATUS: 6 done, 1 active, 0 pending
---

---
[12:50:15] TOOL: Grep
PATTERN: PostToolUse
PATH: /path/.claude
MODE: content
---

---
[12:51:22] TOOL: Glob
PATTERN: **/*.py
PATH: /path/.claude
---

---
[12:52:30] TOOL: Edit
FILE: /path/test_hook_logging.txt
OLD: Line 3: Will be edited next....
NEW: Line 3: This line has been edited by the Edit tool!...
---

---
[12:53:10] TOOL: Write
FILE: /path/test_hook_logging.txt
SIZE: 123 chars
---
```

---

## Benefits

### 1. Rich Context Without Token Cost
- **Before:** Agent must describe every tool call in conversation
- **After:** Tool details captured automatically in log
- **Savings:** ~50-100 tokens per tool call eliminated from conversation

### 2. Debugging Capability
- See exact commands run
- Track file operations (which files read/written)
- Monitor search operations (patterns, paths, modes)
- Understand edit operations (what changed)

### 3. Audit Trail
- Complete record of all tool usage
- Timestamps for every operation
- Arguments preserved for reproduction
- Progress tracking via TodoWrite status

### 4. Log Analysis
- Can grep logs for specific operations
- Track patterns in tool usage
- Identify inefficiencies
- Measure progress through tasks

---

## Advanced Usage

### Custom Tool Handlers

To add support for additional tools, add handler in hook-logger.py:

```python
elif tool_name == 'YourTool':
    # Extract relevant parameters
    param1 = tool_input.get('param1', '')
    param2 = tool_input.get('param2', '')

    # Format log entry
    log_entry += f"\nPARAM1: {param1}"
    log_entry += f"\nPARAM2: {param2}"
```

### Filtering Sensitive Data

To filter sensitive information:

```python
def sanitize(text):
    # Remove API keys, passwords, etc.
    import re
    text = re.sub(r'api[_-]?key[=:]\s*\S+', 'api_key=***', text, flags=re.I)
    text = re.sub(r'password[=:]\s*\S+', 'password=***', text, flags=re.I)
    return text

# Use in handlers
log_entry += f"\nCOMMAND: {sanitize(command[:200])}"
```

### Conditional Logging

To log only certain tools:

```python
# At top of script, define allowed tools
ALLOWED_TOOLS = {'Bash', 'Grep', 'Edit'}

# In main logic
if tool_name not in ALLOWED_TOOLS:
    sys.exit(0)  # Skip logging for other tools
```

---

## Troubleshooting

### No Arguments Logged

**Symptom:** TOOL entries show tool name but no arguments

**Causes:**
1. Old version of hook-logger.py (before enhancements)
2. Tool not supported yet
3. Tool input empty or malformed

**Solutions:**
```bash
# Update hook logger
cp ~/.claude/skills/architect-agent/templates/code-agent-workspace/.claude/hook-logger.py .claude/
chmod +x .claude/hook-logger.py

# Test manually
echo '{"tool_name":"Bash","tool_input":{"command":"test","description":"test"}}' | .claude/hook-logger.py

# Check log
tail -5 $(cat debugging/current_log_file.txt)
```

### Truncated Arguments

**Symptom:** OLD/NEW strings show "..." even when short

**Cause:** Hardcoded 100-character limit in Edit handler

**Solution:** Adjust limit in hook-logger.py:
```python
log_entry += f"\nOLD: {old_string[:200]}..."  # Increase to 200
log_entry += f"\nNEW: {new_string[:200]}..."  # Increase to 200
```

### Missing Tool Support

**Symptom:** Some tools only show tool name, no arguments

**Cause:** Tool handler not implemented yet

**Solution:** Add handler (see "Custom Tool Handlers" above) or submit enhancement request

---

## Future Enhancements

**Planned Improvements:**
- Support for Task tool (agent invocations)
- Support for WebFetch tool (URL + prompt)
- Support for MultiEdit tool (multiple file edits)
- Support for Skill tool (skill name + arguments)
- Configurable argument truncation limits
- Sensitive data filtering (API keys, passwords)
- Conditional logging by tool type
- Log entry templates for custom tools

**Current Priority:** Production-ready core tool support

---

## Version History

### v2.0 (2025-11-20)
- ✅ Enhanced argument capture for all major tools
- ✅ Bash: command + description
- ✅ Read: file_path + offset/limit
- ✅ Write: file_path + content size
- ✅ Edit: file_path + old/new + replace_all
- ✅ Grep: pattern + path + glob + mode + case_insensitive
- ✅ Glob: pattern + path
- ✅ TodoWrite: count + status summary

### v1.0 (2025-11-19)
- Basic hook logging (tool name + file path only)
- Bash, Read, Write, Edit basic support
- Grep pattern logging

---

## References

**Related Documentation:**
- `hook_configuration_critical.md` - Settings.json fix
- `hybrid_logging_protocol.md` - Complete v2.0 protocol
- `opencode_wrapper_setup.md` - Dual-mode logging

**Implementation Files:**
- `.claude/hook-logger.py` - Enhanced hook logger script
- `.claude/settings.json` - Hook configuration
- `debugging/logs/*.log` - Log files with enhanced entries

---

**Last Updated:** 2025-11-20
**Status:** Production Ready
**Version:** 2.0
