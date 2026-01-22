# Pre-Work Checklist

**Version:** 1.0
**Purpose:** Verification steps before code agent begins any work

---

## Overview

This checklist ensures the code agent workspace is properly configured before starting any task. Complete ALL items before executing instructions.

---

## Checklist

### 1. Verify Instruction File Exists

```bash
# Check for active instructions
cat debugging/instructions/current_instructions.md

# Expected: Instruction content displays
# If missing: Wait for architect to send instructions
```

**What to verify:**
- [ ] File exists at `debugging/instructions/current_instructions.md`
- [ ] File contains valid instruction content
- [ ] Objectives and success criteria are clear

---

### 2. Initialize Logging Session

```bash
# Start new log session
/log-start "description_matching_instruction"

# Verify log file created
cat debugging/current_log_file.txt
```

**What to verify:**
- [ ] `/log-start` command executed
- [ ] `debugging/current_log_file.txt` contains log file path
- [ ] Log file exists at the specified path

**Example:**
```bash
/log-start "proj123_phase2_api_implementation"
# Should create: debugging/logs/log_2025_11_27-14_30-proj123_phase2_api_implementation.md
```

---

### 3. Verify Hooks Are Operational

```bash
# Check hooks configuration
grep -A 5 "PostToolUse" .claude/settings.json

# Verify hook logger exists
ls -la .claude/hook-logger.py
```

**What to verify:**
- [ ] Hooks defined in `.claude/settings.json` (NOT `.claude/hooks.json`)
- [ ] Hook logger script exists and is executable
- [ ] `current_log_file.txt` path matches active log

**If hooks are in wrong file:**
```bash
# WRONG - Claude Code doesn't read this
ls .claude/hooks.json

# CORRECT - Claude Code reads hooks from here
ls .claude/settings.json
```

---

### 4. Confirm Build Environment

```bash
# Verify build tools available (project-specific)
# Examples:
./gradlew --version       # Java/Gradle
npm --version             # Node.js
poetry --version          # Python
```

**What to verify:**
- [ ] Build tool is installed and accessible
- [ ] Correct version matches project requirements
- [ ] Dependencies are installed

---

### 5. Run Initial Tests

```bash
# Run tests to confirm baseline (project-specific)
# Examples:
./gradlew test           # Java/Gradle
npm test                 # Node.js
pytest                   # Python
```

**What to verify:**
- [ ] Tests run without configuration errors
- [ ] Note any pre-existing failures (don't fix unless instructed)
- [ ] Test framework is properly configured

---

### 6. Check Git Status

```bash
# Verify clean working directory
git status

# Check current branch
git branch --show-current
```

**What to verify:**
- [ ] Working directory is clean (no uncommitted changes)
- [ ] On correct branch for the task
- [ ] Remote is up to date

---

### 7. Review Success Criteria

Before starting, re-read the instruction's success criteria:

```bash
# Extract success criteria from instructions
grep -A 20 "Success Criteria" debugging/instructions/current_instructions.md
```

**What to verify:**
- [ ] All success criteria are understood
- [ ] Success criteria are measurable
- [ ] You know how to verify each criterion

---

## Quick Reference Card

```
┌─────────────────────────────────────────────────────────┐
│                  PRE-WORK CHECKLIST                     │
├─────────────────────────────────────────────────────────┤
│ □ 1. cat debugging/instructions/current_instructions.md │
│ □ 2. /log-start "description"                           │
│ □ 3. cat debugging/current_log_file.txt                 │
│ □ 4. grep "PostToolUse" .claude/settings.json           │
│ □ 5. [build-tool] --version                             │
│ □ 6. [test-command]                                     │
│ □ 7. git status                                         │
│ □ 8. Review success criteria                            │
└─────────────────────────────────────────────────────────┘
```

---

## Common Issues

### Issue: No instructions found

```
cat: debugging/instructions/current_instructions.md: No such file or directory
```

**Solution:** Wait for architect to send instructions via `/project.send`

---

### Issue: Log file not created

```
cat debugging/current_log_file.txt
cat: debugging/current_log_file.txt: No such file or directory
```

**Solution:** Run `/log-start "description"` to initialize logging

---

### Issue: Hooks not working

**Symptoms:**
- Tool calls not appearing in log file
- `debugging/current_log_file.txt` is stale or wrong

**Solutions:**
1. Verify hooks in `.claude/settings.json` (not `hooks.json`)
2. Run `/log-start` to update `current_log_file.txt`
3. Check hook logger is executable: `chmod +x .claude/hook-logger.py`

---

### Issue: Tests fail before starting

**Action:** Log as pre-existing condition, don't fix unless instructed

```bash
./debugging/scripts/log-decision.sh investigation "Pre-existing test failures: 3 tests in AuthServiceTest"
```

---

## Grading Impact

Skipping the pre-work checklist affects grading:

| Issue | Points Lost |
|-------|-------------|
| No `/log-start` at beginning | -5 points |
| `current_log_file.txt` not updated | -7 points |
| Hooks not verified | -3 points |
| Instructions not reviewed | -5 points |

---

## Post-Checklist Actions

Once checklist is complete:

1. **Log checklist completion:**
   ```bash
   ./debugging/scripts/log-decision.sh milestone "Pre-work checklist complete - ready to begin implementation"
   ```

2. **Begin first task from instructions**

3. **Continue logging throughout work**

---

**Last Updated:** 2025-11-27
**Version:** 1.0
