# Migration Guide: v1.0 → v2.0 Hybrid Logging Protocol

**Target Audience:** Existing code agent workspaces using manual logging protocol (v1.0)
**Goal:** Migrate to hook-based hybrid logging for 60-70% token reduction
**Estimated Migration Time:** 10-15 minutes per workspace

---

## Quick Overview

### What's Changing

**OLD (v1.0) - Fully Manual:**
```bash
echo "[14:23:45] Running tests" | tee -a "$LOG_FILE"
task test 2>&1 | tee -a "$LOG_FILE"
echo "[14:23:47] Result: Tests passed" | tee -a "$LOG_FILE"
```
**Token cost:** ~100 tokens per command

**NEW (v2.0) - Hybrid:**
```bash
# Hooks auto-log this (0 tokens)
task test

# You only log the "why" (15 tokens)
./debugging/scripts/log-decision.sh decision \
  "Testing DB layer first - fastest feedback"
```
**Token cost:** ~15 tokens per command

**Savings:** 85% reduction in logging tokens

---

## Migration Checklist

### Prerequisites

- [ ] Existing code agent workspace with `.claude/` directory
- [ ] Current logging uses v1.0 protocol (manual `echo` + `tee`)
- [ ] Access to architect agent skill workspace (`~/.claude/skills/architect-agent/`)

### Step-by-Step Migration

#### Step 1: Install Hook Configuration

**Copy hooks template to code agent workspace:**

```bash
# Navigate to code agent workspace
cd ~/clients/project/src/project-name

# Copy hooks.json
cp ~/.claude/skills/architect-agent/templates/hooks.json .claude/

# Verify
cat .claude/hooks.json
```

**Expected output:**
```json
{
  "hooks": {
    "user-prompt-submit-hook": { ... },
    "tool-call-hook": { ... },
    "tool-result-hook": { ... }
  }
}
```

✅ **Checkpoint:** Hooks configuration file exists

#### Step 2: Install Decision Logging Script

**Copy and make executable:**

```bash
# Create scripts directory if needed
mkdir -p debugging/scripts

# Copy script
cp ~/.claude/skills/architect-agent/templates/debugging/scripts/log-decision.sh \
   debugging/scripts/

# Make executable
chmod +x debugging/scripts/log-decision.sh

# Test
./debugging/scripts/log-decision.sh
```

**Expected output:**
```
Usage: ./debugging/scripts/log-decision.sh <type> <message>
Types: decision, rationale, investigation, verification, deviation, milestone
```

✅ **Checkpoint:** Script exists and is executable

#### Step 3: Update Permissions

**Edit `.claude/settings.local.json`:**

```bash
# Open settings
code .claude/settings.local.json
# or
nano .claude/settings.local.json
```

**Add to `permissions.allow` array:**

```json
{
  "permissions": {
    "allow": [
      "Bash(./debugging/scripts/log-decision.sh:*)",
      "Bash(debugging/scripts/log-decision.sh:*)",

      // ... keep all existing permissions ...
    ]
  }
}
```

✅ **Checkpoint:** Permissions updated

#### Step 4: Update Documentation

**Update AGENTS.md or CLAUDE.md:**

Add reference to new protocol:

```markdown
## Logging Protocol

**Version:** 2.0 (Hybrid)
**Protocol:** See `~/.claude/skills/architect-agent/references/hybrid_logging_protocol.md`

### Automated Logging (Hooks)
- All commands automatically logged (0 tokens)
- Tool calls, outputs, exit codes captured
- Timestamps added automatically

### Manual Logging (High-Value Context)
Use `log-decision.sh` for:
- Decisions: Why you chose approach A over B
- Rationale: Reasoning behind technical choices
- Investigations: Root cause analysis
- Verifications: How you confirmed it works
- Deviations: When diverging from instructions
- Milestones: Major progress markers

### Commands
```bash
# Start session (creates log file)
/log-start

# Log decisions manually
./debugging/scripts/log-decision.sh decision "Using async for better performance"

# Checkpoint at milestones
/log-checkpoint

# Complete session
/log-complete
```

### Slash Commands (Unchanged)
- `/log-start` - Start new log file
- `/log-checkpoint` - Manual milestone logging
- `/log-complete` - Finalize session
```

✅ **Checkpoint:** Documentation updated

#### Step 5: Test the Migration

**Create test log session:**

```bash
# Start logging session
/log-start

# Run a command (hooks will auto-log)
ls -la

# Add manual decision context
./debugging/scripts/log-decision.sh decision \
  "Testing hybrid logging migration"

# Check log file
cat debugging/logs/log_*.md | tail -50
```

**Expected log output:**
```markdown
[14:23:00] Session Started
**Goal:** Migration testing
...

---
[14:23:05] TOOL: Bash
PARAMS: command="ls -la"
[14:23:05] RESULT: ✅ Success
OUTPUT:
total 128
drwxr-xr-x  10 user  staff   320 Jan 15 14:23 .
drwxr-xr-x   8 user  staff   256 Jan 15 14:20 ..
...
---

[14:23:10] 🎯 DECISION: Testing hybrid logging migration
```

✅ **Checkpoint:** Hooks working, manual logging working

#### Step 6: Remove Old Manual Logging Code

**If your AGENTS.md or CLAUDE.md has old manual logging instructions:**

❌ **Remove:**
```markdown
## Logging Protocol (OLD v1.0)

Every command must be logged:
```bash
echo "[$(date +%H:%M:%S)] Running tests" | tee -a "$LOG_FILE"
task test 2>&1 | tee -a "$LOG_FILE"
echo "[$(date +%H:%M:%S)] Result: Tests completed" | tee -a "$LOG_FILE"
```
```

✅ **Replace with:**
```markdown
## Logging Protocol (v2.0 Hybrid)

See Step 4 documentation above.
```

---

## Verification

### Test Scenarios

#### Scenario 1: Basic Command Logging

**Run:**
```bash
/log-start
task test
./debugging/scripts/log-decision.sh milestone "Tests passing"
/log-complete
```

**Verify log contains:**
- ✅ Session start marker
- ✅ Automated tool logging (TOOL: Bash, PARAMS, RESULT)
- ✅ Manual milestone entry (🏁 MILESTONE)
- ✅ Session completion summary

#### Scenario 2: Decision Logging

**Run:**
```bash
./debugging/scripts/log-decision.sh decision "Using PostgreSQL for ACID compliance"
./debugging/scripts/log-decision.sh rationale "Need concurrent writes and transactions"
```

**Verify log contains:**
```markdown
[HH:MM:SS] 🎯 DECISION: Using PostgreSQL for ACID compliance
[HH:MM:SS] 💭 RATIONALE: Need concurrent writes and transactions
```

#### Scenario 3: Error Investigation

**Run:**
```bash
./debugging/scripts/log-decision.sh investigation \
  "Import error traced to missing dependency in requirements.txt"

./debugging/scripts/log-decision.sh verification \
  "Added pydantic==2.5.0, re-ran tests, all passing"
```

**Verify log contains:**
```markdown
[HH:MM:SS] 🔍 INVESTIGATION: Import error traced to missing dependency in requirements.txt
[HH:MM:SS] ✓ VERIFICATION: Added pydantic==2.5.0, re-ran tests, all passing
```

---

## Troubleshooting

### Issue 1: Hooks Not Logging

**Symptoms:**
- Commands run but no automated entries in log
- Only manual entries appear

**Diagnosis:**
```bash
# Check hooks file exists
ls -la .claude/hooks.json

# Check hooks syntax
cat .claude/hooks.json | jq .

# Check active log file
cat debugging/current_log_file.txt
```

**Fixes:**
1. Ensure hooks.json exists in `.claude/` directory
2. Validate JSON syntax with `jq` or JSON validator
3. Restart Claude Code session
4. Ensure log session started with `/log-start`

### Issue 2: Permission Denied for log-decision.sh

**Symptoms:**
- `./debugging/scripts/log-decision.sh: Permission denied`
- Approval prompt every time

**Diagnosis:**
```bash
# Check executable
ls -la debugging/scripts/log-decision.sh

# Check permissions config
cat .claude/settings.local.json | jq '.permissions.allow'
```

**Fixes:**
1. Make executable: `chmod +x debugging/scripts/log-decision.sh`
2. Add both variants to permissions:
   - `Bash(./debugging/scripts/log-decision.sh:*)`
   - `Bash(debugging/scripts/log-decision.sh:*)`
3. Restart Claude Code session

### Issue 3: Log File Not Found

**Symptoms:**
- `Error: No active log session`

**Diagnosis:**
```bash
# Check if log session active
cat debugging/current_log_file.txt

# Check logs directory
ls -la debugging/logs/
```

**Fixes:**
1. Start log session: `/log-start`
2. Verify `debugging/current_log_file.txt` points to valid file
3. Check `debugging/logs/` directory exists

### Issue 4: Old Manual Logging Still Used

**Symptoms:**
- Code agent still using `echo` + `tee`
- High token consumption
- Grading deduction (-2 points)

**Fix:**
1. Update AGENTS.md/CLAUDE.md to reference hybrid protocol
2. Remove old manual logging examples
3. Train code agent on new protocol
4. Add reminder at top of instructions: "Use hybrid logging v2.0"

---

## Rollback Plan

If migration causes issues:

### Emergency Rollback

```bash
# 1. Remove hooks
rm .claude/hooks.json

# 2. Remove decision script permissions
# Edit .claude/settings.local.json and remove log-decision.sh lines

# 3. Revert AGENTS.md/CLAUDE.md to v1.0 protocol

# 4. Resume v1.0 manual logging
```

**Note:** Logs created during hybrid testing remain valid and readable.

---

## Migration Validation Checklist

Before declaring migration complete:

- [ ] Hooks configuration file exists (`.claude/hooks.json`)
- [ ] Decision logging script exists and is executable
- [ ] Permissions updated for `log-decision.sh`
- [ ] AGENTS.md or CLAUDE.md references hybrid protocol
- [ ] Test scenario 1 passed (basic command logging)
- [ ] Test scenario 2 passed (decision logging)
- [ ] Test scenario 3 passed (error investigation)
- [ ] Log files contain both automated and manual entries
- [ ] No permission prompts when using `log-decision.sh`
- [ ] Slash commands (`/log-start`, `/log-complete`) still work

---

## Post-Migration

### Expected Improvements

1. **Token Reduction:** 60-70% fewer tokens per session
2. **Faster Execution:** No manual `echo` + `tee` commands
3. **Better Logs:** More consistent automated entries
4. **Focused Manual Logging:** Only high-value context

### Monitoring

**First 3 Sessions After Migration:**

Track these metrics:
- Token usage per session (should decrease significantly)
- Log quality (automated entries present?)
- Permission prompts (should be zero for logging)
- Grading scores (logging & traceability should improve)

**Red Flags:**
- Token usage same or higher → hooks not working
- Permission prompts → permissions not configured correctly
- No automated entries → hooks not installed or log session not started
- Lower grading scores → manual decision logging insufficient

### Support

If issues persist after following this guide:

1. Check `hybrid_logging_protocol.md` for detailed usage
2. Review `permissions_setup_protocol.md` for permission troubleshooting
3. Compare with template files in architect workspace:
   - `templates/hooks.json`
   - `templates/debugging/scripts/log-decision.sh`

---

## Summary

### Before Migration (v1.0)
- 100% manual logging with `echo` + `tee`
- ~2,400-4,500 tokens per 30-command session
- Frequent permission prompts
- Manual timestamp management

### After Migration (v2.0)
- Automated hook logging (0 tokens)
- Manual decision logging (~300-600 tokens per session)
- 60-70% token reduction
- Zero permission prompts (with proper setup)
- Better log consistency

### Time Investment
- Migration: 10-15 minutes
- ROI: Immediate token savings on every session

---

**Migration Date:** _____________________
**Migrated By:** _____________________
**Validation Completed:** _____________________
