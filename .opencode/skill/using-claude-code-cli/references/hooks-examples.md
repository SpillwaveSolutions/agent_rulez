# Hooks Configuration Examples

Patterns for configuring Claude Code hooks in automation scenarios.

## Contents

- [Hook Script Template](#hook-script-template)
- [Hook Types Reference](#hook-types-reference)
- [Settings JSON Generation](#settings-json-generation)
- [Environment Variables](#environment-variables)

---

## Hook Script Template

Hook scripts receive JSON on stdin and should exit 0 on success:

```python
#!/usr/bin/env python3
"""Example hook script for session logging."""
import json
import sys
from datetime import datetime
from pathlib import Path


def main():
    try:
        data = json.load(sys.stdin)
        timestamp = datetime.now().strftime('%H:%M:%S')

        # Extract event information
        event_type = data.get('event_type', 'tool')
        tool_name = data.get('tool', {}).get('name', 'Unknown')

        # Log to session file
        log_path = Path.home() / '.claude' / 'session.log'
        with open(log_path, 'a') as f:
            f.write(f"[{timestamp}] {event_type}: {tool_name}\n")

        sys.exit(0)
    except Exception:
        sys.exit(0)  # Silent failure - don't break the hook chain


if __name__ == '__main__':
    main()
```

**Requirements:**
- Shebang: `#!/usr/bin/env python3`
- Executable: `chmod +x hook.py`
- Exit 0 on success (non-zero blocks the operation for PreToolUse)

---

## Hook Types Reference

| Hook Type | Triggered When | Can Block |
|-----------|----------------|-----------|
| `PreToolUse` | Before a tool runs | Yes (exit non-zero) |
| `PostToolUse` | After a tool completes | No |
| `UserPromptSubmit` | When user submits a prompt | Yes |
| `Notification` | On notifications | No |
| `Stop` | When agent stops | No |
| `SubagentStop` | When a subagent (Task) stops | No |
| `PermissionRequest` | When permission is requested | Yes |

**PreToolUse blocking example:**

```python
#!/usr/bin/env python3
"""Block dangerous operations."""
import json
import sys

BLOCKED_PATTERNS = ['rm -rf', 'DROP TABLE', 'DELETE FROM']

def main():
    data = json.load(sys.stdin)
    tool = data.get('tool', {})

    if tool.get('name') == 'Bash':
        command = tool.get('input', {}).get('command', '')
        for pattern in BLOCKED_PATTERNS:
            if pattern in command:
                print(f"Blocked dangerous command: {pattern}", file=sys.stderr)
                sys.exit(1)  # Non-zero blocks the operation

    sys.exit(0)

if __name__ == '__main__':
    main()
```

---

## Settings JSON Generation

Generate settings JSON with absolute paths at runtime:

```python
import json
from pathlib import Path


def generate_settings_json(project_dir: Path) -> str:
    """Generate hooks settings with absolute paths.

    Args:
        project_dir: Root directory of the project

    Returns:
        JSON string for --settings flag
    """
    hook_script = project_dir / ".claude" / "hooks" / "logger.py"

    settings = {
        "hooks": {
            "PostToolUse": [{
                "matcher": "*",
                "hooks": [{
                    "type": "command",
                    "command": str(hook_script.absolute()),
                    "timeout": 5
                }]
            }],
            "PreToolUse": [{
                "matcher": "Bash",  # Only for Bash tool
                "hooks": [{
                    "type": "command",
                    "command": str(hook_script.absolute()),
                    "timeout": 5
                }]
            }],
        },
        "permissions": {
            "additionalDirectories": [
                str(project_dir / ".claude" / "skills"),
                str(project_dir / "templates"),
            ]
        },
        "sandbox": {
            "enabled": True,
            "autoAllowBashIfSandboxed": True
        }
    }
    return json.dumps(settings)
```

**Usage:**
```python
settings = generate_settings_json(Path("/path/to/project"))
cmd = ["claude", "--settings", settings, "-p", "Your prompt"]
```

---

## Environment Variables

Key variables available in hooks and automation:

| Variable | Purpose | Set By |
|----------|---------|--------|
| `CLAUDE_PROJECT_DIR` | Project root directory | Claude |
| `TMPDIR` | Temp directory (`/tmp/claude/` in sandbox) | Claude |

**Using in hook scripts:**
```python
import os

project_dir = os.environ.get('CLAUDE_PROJECT_DIR', os.getcwd())
log_path = os.path.join(project_dir, '.claude', 'session.log')
```

**Using in settings.json for portable paths:**
```json
{
  "hooks": {
    "PostToolUse": [{
      "matcher": "*",
      "hooks": [{
        "type": "command",
        "command": "$CLAUDE_PROJECT_DIR/.claude/hooks/logger.py"
      }]
    }]
  }
}
```

---

## Sandbox Settings Options

Full sandbox configuration reference:

```json
{
  "sandbox": {
    "enabled": true,
    "autoAllowBashIfSandboxed": true,
    "excludedCommands": ["mmdc", "chromium"],
    "network": {
      "allowUnixSockets": ["/var/run/docker.sock"],
      "allowLocalBinding": true
    }
  }
}
```

| Option | Default | Purpose |
|--------|---------|---------|
| `enabled` | false | Enable sandbox mode |
| `autoAllowBashIfSandboxed` | false | Skip Bash permission prompts in sandbox |
| `excludedCommands` | [] | Commands that bypass sandbox restrictions |
| `network.allowUnixSockets` | [] | Unix sockets to allow |
| `network.allowLocalBinding` | false | Allow binding to local ports |

---

## See Also

- [subprocess-patterns.md](subprocess-patterns.md) - Python invocation patterns
- [json-extraction.md](json-extraction.md) - Parsing JSON from output
- [orchestrator_example.py](orchestrator_example.py) - Complete orchestrator class
