---
description: Create a checkpoint in the current logging session
---
# /log-checkpoint - Create Checkpoint

Create a checkpoint in the current logging session:

```bash
./debugging/scripts/log-checkpoint.sh "checkpoint-name"
```

**Example:**
```bash
./debugging/scripts/log-checkpoint.sh "tests-passing"
```

**What this does:**
1. Adds a visual checkpoint marker to the log
2. Includes timestamp and checkpoint name
3. Helps organize log into logical sections

**When to use:**
- After completing a major step
- Before starting a new phase of work
- After tests pass
- After fixing a bug
- Before attempting risky changes

**Checkpoint format in log:**
```
========================================
CHECKPOINT: tests-passing
TIME: 2025-11-20 14:30:45
========================================
```
