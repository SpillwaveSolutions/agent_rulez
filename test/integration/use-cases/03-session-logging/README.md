# Integration Test: Session Logging

## Purpose

Verify that CCH (Claude Context Hooks) creates proper audit logs with timing information when processing Claude CLI events.

## What This Tests

1. **Log file creation** - CCH should create log entries in `~/.claude/logs/cch.log`
2. **JSON Lines format** - Each log entry should be valid JSON on a single line
3. **Required fields** - Entries should contain timestamp, event_type, session_id
4. **Timing information** - Entries should include processing time metrics

## Configuration

The test uses this `hooks.yaml` configuration:

```yaml
version: "1.0"

rules:
  - name: log-all-bash
    matchers:
      tools: ["Bash"]
    actions:
      log_level: "debug"

  - name: log-file-operations
    matchers:
      tools: ["Write", "Edit", "Read"]
    actions:
      log_level: "debug"

settings:
  log_level: "debug"
  debug_logs: true
```

## Expected Log Format

Each log entry should be a JSON object with structure like:

```json
{
  "timestamp": "2025-01-23T10:30:00Z",
  "event_type": "PreToolUse",
  "session_id": "abc123",
  "tool_name": "Bash",
  "rules_matched": ["log-all-bash"],
  "outcome": "Allow",
  "timing": {
    "processing_ms": 5,
    "rules_evaluated": 2
  }
}
```

## Running the Test

```bash
# From the integration test directory
./use-cases/03-session-logging/test.sh

# Or run all tests
./run-all.sh
```

## Success Criteria

- Log file exists at `~/.claude/logs/cch.log`
- Log entries are created when Claude CLI runs
- Each entry is valid JSON
- Entries contain required fields
- Timing information is captured

## Log Location

CCH writes logs to: `~/.claude/logs/cch.log`

View logs with:
```bash
# Recent entries
cch logs --limit 10

# Tail live
tail -f ~/.claude/logs/cch.log

# Query specific session
cch logs --session <session-id>
```

## Notes

- Logs use JSON Lines format (one JSON object per line)
- Debug logging captures full event details
- Log rotation is handled automatically by CCH
