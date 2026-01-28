---
description: Complete the current logging session
---
# /log-complete - Complete Logging Session

Complete the current logging session:

```bash
./debugging/scripts/log-complete.sh
```

**What this does:**
1. Adds completion footer to log file
2. Clears `debugging/current_log_file.txt` (stops hook logging)
3. Finalizes the session

**Session footer added:**
```
========================================
SESSION COMPLETED
TIME: 2025-11-20 14:45:30
========================================
```

**After completion:**
- Hooks stop logging (no active session)
- Log file preserved in `debugging/logs/`
- Ready to start new session with `/log-start`

**Always complete sessions** to:
- Mark clear end point in logs
- Stop unnecessary hook logging
- Keep logs organized
