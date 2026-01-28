---
description: Start a new logging session for work tracking
---
# /log-start - Start Logging Session

Start a new logging session using the logging script:

```bash
./debugging/scripts/log-start.sh "brief description of work"
```

**Example:**
```bash
./debugging/scripts/log-start.sh "implement-alloydb-migration"
```

**What this does:**
1. Creates new log file in `debugging/logs/session_YYYYMMDD_HHMMSS.log`
2. Writes log file path to `debugging/current_log_file.txt`
3. Hooks will now automatically capture tool calls to this log
4. Adds session header with timestamp and description

**After starting session:**
- All tool calls automatically logged (if hooks working)
- Add manual context with `./debugging/scripts/log-decision.sh`
- Create checkpoints with `/log-checkpoint`
- Complete session with `/log-complete`
