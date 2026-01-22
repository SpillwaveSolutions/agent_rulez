# Architect Agent Workflow

**Version:** 3.0 (Hybrid Logging v2.0)
**Last Updated:** [DATE]

---

## Core Workflow

### 1. Planning Phase

**Input:** JIRA ticket, user request, or problem statement

**Steps:**
1. Review `ticket/current_ticket.md` for context
2. Create analysis document if needed (`analysis/`)
3. Research technical approach
4. Break down into implementable tasks

**Output:** Clear understanding of what needs to be done

---

### 2. Instruction Creation

**File Location:** `instructions/instruct-YYYYMMDD_HHMMSS-description.md`

**File Naming Convention:**
```
instruct-20251120_143045-implement_api_refactor.md
```

**Instruction Structure:**
```markdown
# [Title]

## Context
[Background information]

## Objectives
- Objective 1
- Objective 2

## Requirements
[Technical requirements]

## Constraints
[Limitations and gotchas]

## Implementation Steps
1. Step 1
2. Step 2

## Success Criteria
- Criterion 1
- Criterion 2

## Testing
[Required tests]

## References
[Links to docs]
```

**Critical:**
- Write instructions in THIS workspace (`instructions/`)
- Do NOT write directly to code agent workspace
- Instructions copied to code agent on "send"

---

### 3. Human Summary Creation

**File Location:** `human/human-YYYYMMDD_HHMMSS-description.md`

**Must match instruction timestamp!**

**Format:** 10-25 bullet points covering:
- Main objectives (2-4 bullets)
- Key requirements (3-6 bullets)
- Critical constraints (2-4 bullets)
- Success criteria (2-4 bullets)
- Testing requirements (2-4 bullets)

**Purpose:**
- Quick review before sending
- Show user what code agent will do
- Documentation for later reference

---

### 4. Sending Instructions

**Command:** `/project.send`

**What Happens:**
1. Reads latest instruction file from `instructions/`
2. Copies to code agent's `debugging/instructions/current_instructions.md`
3. Reads matching human summary from `human/`
4. Displays 10-25 bullet summary to user
5. Confirms instructions sent

**Manual Alternative:**
```bash
# Copy instruction
cp instructions/instruct-[TIMESTAMP]-[DESC].md \
   [CODE_AGENT_WORKSPACE]/debugging/instructions/current_instructions.md

# Show human summary
cat human/human-[TIMESTAMP]-[DESC].md
```

---

### 5. Monitoring Execution

**While Code Agent Works:**
- Monitor logs: `[CODE_AGENT_WORKSPACE]/debugging/logs/`
- Check progress via log checkpoints
- Available to answer questions if blocked

**What to Watch:**
- Hooks capturing all tool calls
- Manual decisions being logged
- Tests being run
- Quality gates being met

---

### 6. Grading Completed Work

**File Location:** `grades/grade-YYYYMMDD_HHMMSS-description.md`

**Must match instruction timestamp!**

**Grading Categories (100 points):**
1. Instruction Adherence (25 points)
2. Code Quality (20 points)
3. Testing & Validation (20 points)
4. Logging & Traceability (10 points)
5. Communication & Documentation (15 points)
6. Problem Solving (10 points)

**Grade Levels:**
- 90-100: Excellent (production ready)
- 80-89: Good (minor improvements)
- 70-79: Satisfactory (some rework)
- 60-69: Needs Improvement (significant issues)
- 0-59: Unacceptable (major problems)

**Grading Process:**
1. Review code agent's log file
2. Check code changes
3. Verify tests passing
4. Review hook quality (automatic logging)
5. Assess problem-solving approach
6. Create grade document with scores and feedback

---

### 7. Iteration (if needed)

**If Grade < 80:**
1. Create new instruction file addressing issues
2. Reference previous grade in new instructions
3. Send updated instructions
4. Monitor execution again
5. Grade again

**If Grade >= 80:**
1. Mark ticket phase complete
2. Update `ticket/current_ticket.md`
3. Archive instruction, human summary, and grade
4. Move to next phase or ticket

---

## File Naming Best Practices

**Timestamp Format:** `YYYYMMDD_HHMMSS`
- Year Month Day underscore Hour Minute Second
- Example: `20251120_143045`

**Description:** Brief, lowercase with underscores
- Keep under 50 characters
- Use clear, descriptive names
- Example: `implement_api_refactor`

**Matching Files:**
```
instructions/instruct-20251120_143045-implement_api_refactor.md
human/human-20251120_143045-implement_api_refactor.md
grades/grade-20251120_143045-implement_api_refactor.md
```

**Why Matching Timestamps:**
- Easy to correlate related files
- Chronological sorting works correctly
- Clear audit trail

---

## Workspace Organization

**This Workspace (Architect Agent):**
```
architect-workspace/
├── CLAUDE.md                    # Workspace configuration
├── AGENTS.md                    # Mirror of CLAUDE.md
├── instructions/                # Instruction files (write here)
├── human/                       # Human summaries (write here)
├── grades/                      # Grade files (write here)
├── ticket/                      # Current ticket tracking
├── analysis/                    # Analysis documents
├── docs/                        # Architect documentation
└── .claude/
    ├── settings.json            # Permissions
    └── commands/                # Slash commands
```

**Code Agent Workspace (READ-ONLY for you):**
```
code-agent-workspace/
├── CLAUDE.md
├── AGENTS.md
├── debugging/
│   ├── instructions/           # Instructions sent HERE
│   │   └── current_instructions.md  # Canonical instruction
│   ├── logs/                   # Execution logs
│   └── scripts/                # Logging scripts
└── .claude/
    ├── settings.json           # Hooks configuration
    └── hook-logger.py          # Hook logger script
```

---

## Common Pitfalls

### ❌ Writing Directly to Code Agent Workspace
**Wrong:**
```bash
# Writing instruction directly to code agent
echo "..." > [CODE_AGENT]/debugging/instructions/current_instructions.md
```

**Correct:**
```bash
# Write to YOUR workspace first
echo "..." > instructions/instruct-[TIMESTAMP]-[DESC].md

# Then use /project.send to copy
/project.send
```

### ❌ Forgetting Human Summary
**Problem:** Created instruction but no human summary

**Solution:** Always create matching human summary with same timestamp

### ❌ Not Verifying Hooks Working
**Problem:** Sent instructions but didn't verify hooks capturing

**Solution:** Check first few log entries to confirm hooks operational

### ❌ Incomplete Grading
**Problem:** Grade doesn't cover all 6 categories

**Solution:** Use grading rubric template, score each category

---

## Quick Reference

**Start Session:**
```bash
# Check current ticket
cat ticket/current_ticket.md

# Verify GitHub auth
gh auth status
```

**Create Instructions:**
```bash
# Create instruction
vim instructions/instruct-$(date +%Y%m%d_%H%M%S)-[DESCRIPTION].md

# Create matching human summary
vim human/human-$(date +%Y%m%d_%H%M%S)-[DESCRIPTION].md
```

**Send Instructions:**
```bash
/project.send
```

**Grade Work:**
```bash
# Review logs
cat [CODE_AGENT]/debugging/logs/session_*.log

# Create grade
vim grades/grade-[MATCHING_TIMESTAMP]-[DESCRIPTION].md
```

---

## References

**Skill Resources:**
- `~/.claude/skills/architect-agent/references/instruction_structure.md`
- `~/.claude/skills/architect-agent/references/grading_rubrics.md`
- `~/.claude/skills/architect-agent/references/hybrid_logging_protocol.md`

**This Workspace:**
- `docs/hybrid_logging.md` - Logging overview
- `docs/technology_adaptations.md` - Project-specific tech
- `docs/critical_protocols.md` - Critical gotchas

---

**Last Updated:** [DATE]
**Version:** 3.0 (Hybrid Logging v2.0)
