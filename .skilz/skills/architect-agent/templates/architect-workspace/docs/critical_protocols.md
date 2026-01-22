# Critical Protocols

**Version:** 3.0
**Last Updated:** [DATE]

---

## ⚠️ CRITICAL: File Location Protocol

### You Are the Architect Agent

**THIS workspace:**
```
[THIS_WORKSPACE_PATH]
```

**Code agent workspace (READ-ONLY for you):**
```
[PATH_TO_CODE_AGENT_WORKSPACE]
```

### File Writing Rules

**✅ ALWAYS write to YOUR workspace:**
- `instructions/instruct-*.md` - Instructions you create
- `human/human-*.md` - Human summaries
- `grades/grade-*.md` - Grading reports
- `ticket/` - Ticket tracking
- `analysis/` - Analysis documents

**❌ NEVER write directly to code agent workspace**
**✅ EXCEPT: Copy instructions to code agent on "send"**

### Instruction Destination

**Critical Path:**
```
[PATH_TO_CODE_AGENT_WORKSPACE]/debugging/instructions/current_instructions.md
```

**This is the ONLY file code agent reads for instructions.**

**Workflow:**
1. Write instruction to YOUR `instructions/` directory
2. Use `/project.send` to copy to code agent's `current_instructions.md`
3. Code agent reads from `current_instructions.md`

---

## ❌ NO AI Attribution

**CRITICAL: Never include AI attribution in code or documentation unless explicitly requested by user.**

**Examples of what NOT to include:**
```
❌ "Generated with assistance from Claude"
❌ "AI-assisted code"
❌ "Created by AI"
❌ "<!-- AI-generated -->"
```

**Exception:** Git commits include standard attribution:
```
✅ 🤖 Generated with [Claude Code](https://claude.com/claude-code)
   Co-Authored-By: Claude <noreply@anthropic.com>
```

**Why:** Professional code repositories don't include AI attribution. Code should stand on its own merits.

---

## GitHub Authentication

### Verify Authentication

**ALWAYS verify at session start:**
```bash
gh auth status
```

**Expected Output:**
```
✅ Logged in to github.com account [ORG]_[USER]
```

**Example:**
```
✅ Logged in to github.com account acme-developer
```

### If Not Authenticated

```bash
gh auth login
```

Follow prompts to authenticate.

### Wrong Account

```bash
# Logout
gh auth logout

# Login with correct account
gh auth login
```

---

## Permissions Setup

### Cross-Workspace Permissions

**Architect workspace needs WRITE to code agent instructions:**

**File:** `.claude/settings.json` (in architect workspace)
```json
{
  "allowedDirectories": [
    "[THIS_WORKSPACE_PATH]",
    "[PATH_TO_CODE_AGENT_WORKSPACE]/debugging/instructions"
  ]
}
```

**Test Permission:**
```bash
# From architect workspace
echo "test" > [CODE_AGENT]/debugging/instructions/test.txt

# Should succeed
rm [CODE_AGENT]/debugging/instructions/test.txt
```

### Code Agent Permissions

**File:** `.claude/settings.json` (in code agent workspace)
```json
{
  "allowedDirectories": [
    "[PATH_TO_CODE_AGENT_WORKSPACE]"
  ],
  "hooks": {
    "PostToolUse": [...]
  }
}
```

---

## Hook Configuration (Code Agent)

### ⚠️ CRITICAL: settings.json NOT hooks.json

**Code agent hooks MUST be in `.claude/settings.json`**

**❌ WRONG:**
```
.claude/hooks.json  ← Claude Code does NOT read this file
```

**✅ CORRECT:**
```
.claude/settings.json  ← Claude Code ONLY reads hooks from here
```

### Verification

**From code agent workspace:**
```bash
# Check settings.json exists
ls -la .claude/settings.json

# Check for hooks configuration
grep "PostToolUse" .claude/settings.json

# NO hooks.json should exist
ls .claude/hooks.json 2>&1 | grep "No such file"
```

---

## Timestamp Matching

### File Naming Convention

**Format:** `[PREFIX]-YYYYMMDD_HHMMSS-[DESCRIPTION].md`

**Related files MUST have matching timestamps:**
```
instructions/instruct-20251120_143045-implement_feature.md
human/human-20251120_143045-implement_feature.md
grades/grade-20251120_143045-implement_feature.md
```

**Why:**
- Easy correlation between related files
- Chronological sorting
- Clear audit trail

### Creating Matching Timestamps

**Method 1: Save timestamp**
```bash
TIMESTAMP=$(date +%Y%m%d_%H%M%S)
vim instructions/instruct-${TIMESTAMP}-description.md
vim human/human-${TIMESTAMP}-description.md
# Later, after work complete:
vim grades/grade-${TIMESTAMP}-description.md
```

**Method 2: Extract from instruction file**
```bash
# Get latest instruction
LATEST=$(ls -t instructions/instruct-*.md | head -1)

# Extract timestamp
TIMESTAMP=$(echo "$LATEST" | grep -oP '\d{8}_\d{6}')

# Create matching human summary
vim human/human-${TIMESTAMP}-description.md
```

---

## CLAUDE.md and AGENTS.md Synchronization

### Critical Rule

**AGENTS.md MUST be identical to CLAUDE.md**

**After editing CLAUDE.md:**
```bash
cp CLAUDE.md AGENTS.md

# Verify
diff CLAUDE.md AGENTS.md
# Should show: no differences
```

**Why:**
- CLAUDE.md: Read by Claude Code
- AGENTS.md: Read by agent systems (OpenCode, etc.)
- Both must have same information

---

## Session Start Checklist

**ALWAYS do at session start:**

1. **Check Current Ticket**
   ```bash
   cat ticket/current_ticket.md
   ```

2. **Verify GitHub Auth**
   ```bash
   gh auth status
   ```

3. **Confirm Workspace Location**
   ```bash
   pwd
   # Should be: [THIS_WORKSPACE_PATH]
   # NOT: [CODE_AGENT_WORKSPACE]
   ```

4. **Review Workflow**
   ```bash
   cat docs/workflow.md
   ```

---

## Common Mistakes

### Mistake 1: Writing to Code Agent Workspace

**Wrong:**
```bash
# Creating instruction in code agent workspace
vim [CODE_AGENT]/debugging/instructions/instruct-*.md
```

**Correct:**
```bash
# Create in YOUR workspace
vim instructions/instruct-*.md

# Send via command
/project.send
```

### Mistake 2: Hooks in Wrong File

**Wrong:**
```bash
# Hooks in code agent's .claude/hooks.json
cat [CODE_AGENT]/.claude/hooks.json
```

**Correct:**
```bash
# Hooks in code agent's .claude/settings.json
cat [CODE_AGENT]/.claude/settings.json
```

### Mistake 3: Mismatched Timestamps

**Wrong:**
```
instruct-20251120_143045-feature.md
human-20251120_150000-feature.md   ← Different time!
```

**Correct:**
```
instruct-20251120_143045-feature.md
human-20251120_143045-feature.md   ← Same time!
```

### Mistake 4: AI Attribution in Code

**Wrong:**
```python
# AI-generated function
def calculate_total():
    pass
```

**Correct:**
```python
def calculate_total():
    """Calculate total from items."""
    pass
```

---

## Emergency Procedures

### Code Agent Not Reading Instructions

**Check:**
1. Instruction in correct location: `[CODE_AGENT]/debugging/instructions/current_instructions.md`
2. File permissions allow code agent to read
3. File has content (not empty)

### Hooks Not Working

**Check:**
1. Hooks in `settings.json` NOT `hooks.json`
2. Hook logger executable: `ls -la .claude/hook-logger.py`
3. Active log session: `cat debugging/current_log_file.txt`
4. Valid JSON: `python3 -m json.tool .claude/settings.json`

### Permission Denied

**Check:**
1. Correct path in `allowedDirectories` of `.claude/settings.json`
2. Absolute paths (not relative)
3. Directory exists and is accessible

---

## References

**Architect Agent:**
- `~/.claude/skills/architect-agent/references/workspace_setup_complete.md`
- `docs/workflow.md` - Complete workflow guide
- `docs/hybrid_logging.md` - Logging overview

**Code Agent:**
- `[CODE_AGENT]/.claude/docs/logging_setup.md`
- `[CODE_AGENT]/.claude/docs/testing_protocol.md`

---

**Last Updated:** [DATE]
**Version:** 3.0 (Hybrid Logging v2.0)
