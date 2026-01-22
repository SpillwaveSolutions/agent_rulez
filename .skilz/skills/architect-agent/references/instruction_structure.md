# Comprehensive Instruction Structure Template

## Purpose

This template ensures architect agents create complete, well-structured instructions that maximize code agent success rates through clear structure, comprehensive logging requirements, explicit success criteria, and proper agent usage.

## Prerequisites (Verify Before Creating Instructions)

### 1. Permissions Setup

**CRITICAL:** Verify both workspaces have permissions configured in `.claude/settings.local.json` to avoid 50+ approval prompts per session.

**Architect Workspace Permissions:**
- Write access to code agent's `instructions/` and `human/` directories
- Read access to code agent's `debugging/` directory
- Git/GitHub command permissions

**Code Agent Workspace Permissions:**
- Read access to architect agent's `references/` directory
- Execute permissions for `debugging/scripts/log.sh` and `start-log.sh`
- Bash permissions for test commands (`task test:*`, etc.)

**If permissions not configured:** Instruct user to see `references/permissions_setup_protocol.md` for complete setup guide.

### 2. Logging Scripts

**Recommended:** Verify code agent workspace has logging scripts configured:
- `debugging/scripts/log.sh` - Log messages with auto-timestamps
- `debugging/scripts/start-log.sh` - Initialize logging session

**Benefits:**
- Zero permission prompts for logging
- Automatic timestamps
- Consistent formatting

**If not configured:** Instructions should use manual logging with `tee` (see Logging Requirements section below).

### 3. Code Agent Configuration Files

**Required Files in Code Agent Workspace:**
- `.claude/LOGGING.md` - Logging protocol documentation
- `.claude/CLAUDE.md` - Code agent configuration and reminders
- `.claude/AGENTS.md` (optional) - Agent-specific collaboration protocols

**If missing:** Templates available at:
- `references/code_agent_claude_template.md`
- `references/code_agent_agents_template.md`

### 4. Ultrathink Canonical Filename Protocol

**IMPORTANT:** Code agent workspace uses canonical filename `current_instructions.md` for active work.

**Architect Workflow:**
1. Create instruction in architect workspace with timestamp: `instructions/instruct-2025_11_14-14_30-description.md`
2. Activate for code agent: `./scripts/activate-instruction.sh <file> <code-workspace>`
3. Instruction copied to code workspace as `debugging/instructions/current_instructions.md`
4. After completion, code agent archives as `archive/instruct-2025_11_14-14_30-completed.md`

**Benefits:**
- Zero ambiguity: Only ONE active instruction
- Clean workspace: No accumulation of timestamped files
- QUEUE.md tracks lifecycle: Active → Queued → Completed
- Proactive detection: Code agent checks for work on session start

**See:** `references/permissions_setup_protocol.md` - Ultrathink Canonical Filename Protocol section

## Complete Instruction Template

```markdown
# INSTRUCT: [Clear, Action-Oriented Title]

**Date:** YYYY-MM-DD
**Ticket:** [TICKET-ID]
**Priority:** [CRITICAL/HIGH/MEDIUM/LOW] - [Impact description]
**Agent:** Code Agent
**Type:** [Feature/Bug Fix/Refactor/Infrastructure/Documentation]

## 📋 Table of Contents
1. [Goal](#goal)
2. [Context](#context)
3. [Success Criteria](#success-criteria)
4. [Task Breakdown](#task-breakdown)
5. [Logging Requirements](#logging-requirements)
6. [Testing Protocol](#testing-protocol)
7. [Required Agents](#required-agents)
8. [Error Handling](#error-handling)
9. [Completion Checklist](#completion-checklist)

## 🎯 Goal {#goal}

[One paragraph describing what needs to be accomplished and why]

## 📖 Context {#context}

### Problem
[Describe the problem or requirement]

### Root Cause (for bugs)
[Explain why the issue exists]

### Architecture
[Relevant architectural information]
- Current state
- Desired state
- Key components involved

### Why This Matters
[Business or technical impact]

## ✅ Success Criteria (X required) {#success-criteria}

**Definition of Done:**
- [ ] Criterion 1 - [Specific, measurable outcome]
- [ ] Criterion 2 - [Specific, measurable outcome]
- [ ] Criterion 3 - [Specific, measurable outcome]
... [10-15 total criteria]

**Verification Methods:**
- How to verify each criterion
- Commands to run
- Expected outputs

**Acceptance Metrics:**
- Performance targets
- Coverage requirements
- Quality standards

## 🛠️ Task Breakdown {#task-breakdown}

### Task 1: [Task Name]

**What:** [Brief description]

**Why:** [Reason this task is necessary]

**File:** [File path if applicable]

**Commands:**
```bash
# Command 1
command with all flags

# Command 2
another command
```

**Code Changes (if applicable):**
```language
// BEFORE:
old code

// AFTER:
new code
```

**Verification:**
```bash
# Verification command
verification-command

# Expected output
expected result
```

**Verification Checklist:**
- [ ] Verification item 1
- [ ] Verification item 2

[Repeat for each task...]

## 📊 Logging Requirements {#logging-requirements}

**Reference:** `.claude/LOGGING.md` in code agent workspace

**Log File:** `debugging/logs/log-YYYY_MM_DD-HH_MM-ticket_id_phase_description.md`

### Log File Structure

```markdown
# [TICKET-ID]: [Task Title] - Execution Log

**Date:** YYYY-MM-DD
**Start Time:** HH:MM:SS
**Ticket:** [URL to ticket]
**Goal:** [One sentence goal]

## Initial State Analysis
[HH:MM:SS] [Analysis of current state]

## Task 1: [Task Name]
### [HH:MM:SS] [Action Description]
**Command:** \`command-here\`
\`\`\`
command output
\`\`\`
**Result:** ✅ Success / ❌ Failed
**Verification:** [How verified]
---

## [HH:MM:SS] CHECKPOINT: [Description]
**Progress:** X% complete
**Completed:**
  - ✅ Task 1
**In Progress:**
  - 🔄 Task 2
**Remaining:**
  - ⏳ Task 3
**Issues:** None / [Describe issues]
**Time Elapsed:** XX minutes
---

## Summary
**End Time:** HH:MM:SS
**Total Duration:** XX minutes
**Status:** ✅ Complete / ⚠️ Partial / ❌ Failed
**Files Modified:** [Count]
**Tests:** [Results]
```

### Mandatory Logging Practices

**Real-Time Logging with tee:**
```bash
export LOG_FILE="debugging/logs/log-$(date +%Y_%m_%d-%H_%M)-description.md"

# Log every command
echo "[$(date +%H:%M:%S)] Action description" | tee -a "$LOG_FILE"
echo "**Command:** \`your-command\`" | tee -a "$LOG_FILE"
echo '```' | tee -a "$LOG_FILE"
your-command 2>&1 | tee -a "$LOG_FILE"
RESULT=${PIPESTATUS[0]}
echo '```' | tee -a "$LOG_FILE"
if [ $RESULT -eq 0 ]; then
    echo "**Result:** ✅ Success" | tee -a "$LOG_FILE"
else
    echo "**Result:** ❌ Failed" | tee -a "$LOG_FILE"
fi
echo "---" | tee -a "$LOG_FILE"
```

**Checkpoint Requirements:**
- After every 3-5 commands
- Every 5 minutes
- Before/after destructive operations
- At phase boundaries

**Log Every:**
- Commands with full output
- Decisions and reasoning
- Errors and recovery attempts
- Verification results
- Agent invocations

## 🧪 Testing Protocol (MANDATORY) {#testing-protocol}

**This is [feature/bug/refactor] work - Testing Required**

### During This Phase:
```bash
# Run after EVERY code change (10-50 lines)
task test
# If ANY test fails, STOP and fix immediately
```

### [If Milestone Phase] Additional Requirements:
```bash
task cov        # Verify >= 60% coverage
task test-int   # Run integration tests
```

### [If Final Phase] Complete Test Suite Required:
Before marking this ticket complete, you MUST run:
```bash
task test       # All unit tests must pass
task test-int   # All integration tests must pass
task cov        # Verify >= 60% coverage
task cov-int    # Document integration coverage
task cov-all    # Complete coverage report
```

**YOU CANNOT REPORT THIS TASK AS COMPLETE WITHOUT:**
1. ✅ All tests passing (0 failures)
2. ✅ Coverage >= 60%
3. ✅ Integration tests verified
4. ✅ [Final phase only] Complete test suite run

**If tests fail:**
- Use `root-cause-debugger` agent to diagnose
- Use Perplexity MCP to research errors
- Fix the issue, then run `task test` again
- Do NOT proceed until ALL tests pass

## 🤖 Required Agents {#required-agents}

**MANDATORY Agents (Must use):**
- `change-explainer` - After creating change documentation
  - **When:** After creating docs/changes/ file
  - **Purpose:** Analyze and enhance change documentation
  - **Make MANDATORY:** YOU CANNOT complete without running this

- `qa-enforcer` - Final technical validation
  - **When:** Before marking work complete
  - **Purpose:** Validate all technical requirements met
  - **Make MANDATORY:** Final gate before completion

**RECOMMENDED Agents (Use if needed):**
- `root-cause-debugger` - If errors or test failures occur
- `python-expert-engineer` - For complex Python features
- `docs-sync-editor` - If README.md or CLAUDE.md affected
- `mermaid-architect` - For architecture documentation

**Agent Usage Timing:**
1. Complete all implementation tasks
2. Run tests and fix any failures
3. Create change documentation
4. Run `change-explainer` on documentation
5. Run `qa-enforcer` for final validation
6. Mark work complete

**DO NOT:**
- Use qa-enforcer to create documentation (wrong agent!)
- Skip mandatory agents (automatic grade deduction)
- Make mandatory agents "optional" in instructions

## 🚨 Error Handling {#error-handling}

### If [Specific Error Type]:
1. [First troubleshooting step]
2. [Second troubleshooting step]
3. [Research command]: `perplexity-ask "[error message]"`
4. [Retry pattern]
5. [Escalation criteria]

### If Tests Fail:
1. Read test output carefully
2. Identify failing test name
3. Use `root-cause-debugger` agent
4. Fix the issue
5. Run `task test` again
6. Do NOT proceed until passing

### If Commands Fail:
1. Log the error completely
2. Check return code
3. Research error via Perplexity MCP
4. Try alternative approach
5. Document both failure and solution

### If Blocked:
1. Document blocker completely
2. Research solutions (context7, perplexity)
3. Use appropriate debugging agent
4. Try workaround if available
5. Do NOT skip verifications

## ✓ Completion Checklist {#completion-checklist}

### Implementation
- [ ] Task 1 complete with verification
- [ ] Task 2 complete with verification
- [ ] Task 3 complete with verification
[... all tasks]

### Testing
- [ ] All unit tests passing (`task test`)
- [ ] Coverage >= 60% (`task cov`)
- [ ] [If applicable] Integration tests passing
- [ ] [If final phase] Complete test suite run

### Documentation
- [ ] Execution log complete with all timestamps
- [ ] Change documentation created
- [ ] Code comments added where appropriate
- [ ] README updated if needed

### Agent Usage
- [ ] `change-explainer` run on documentation
- [ ] `qa-enforcer` validation complete
- [ ] [Optional agents if used] documented

### Verification
- [ ] All success criteria checked
- [ ] All verifications passed
- [ ] No unresolved errors
- [ ] Checkpoint logs created

### Git & CI/CD
- [ ] Changes committed with descriptive message
- [ ] Commit message includes ticket reference
- [ ] [If CI/CD changes] Workflow triggered and tested
- [ ] [If CI/CD changes] All jobs passing

## 📝 Documentation Requirements

### Change Documentation
**File:** `docs/changes/YYYY-MM-DD-ticket_id-description.md`

**Required Sections:**
```markdown
# [TICKET-ID]: [Title]

**Date:** YYYY-MM-DD
**Type:** [Feature/Bug Fix/Refactor]
**Priority:** [Level]
**Ticket:** [URL]

## Problem
[What was broken or needed]

## Root Cause
[Why the issue existed]

## Solution
[What was implemented]

## Verification
[How the fix was tested]

## Rollback Procedure
If this needs to be reverted:
1. [Specific steps to undo]
2. [Verification after rollback]

## Files Changed
| File | Changes | Description |
|------|---------|-------------|
| file1 | lines X-Y | description |

## Related Issues
- [Ticket links]
```

### PR Description Template
```markdown
## [TICKET-ID]: [Title]

### 🎫 Ticket
[Link to ticket]

### 🐛 Problem / 🚀 Feature
[Brief description]

### ✅ Solution
[Technical approach]

### 📝 Changes
- Change 1 description
- Change 2 description

### 🧪 Testing
- [ ] All unit tests passing
- [ ] Coverage >= 60%
- [ ] Integration tests passing
- [ ] Manual testing completed

### 📋 Checklist
- [ ] Code reviewed
- [ ] Documentation updated
- [ ] No security issues
```

## 📌 Timeline Estimate

- Task 1: X minutes
- Task 2: Y minutes
- Task 3: Z minutes
- Testing: A minutes
- Documentation: B minutes
- **Total:** XX-YY minutes

## 📚 Reference Files

**In Code Agent Workspace:**
- `.claude/LOGGING.md` - Logging protocol
- `.claude/CLAUDE.md` - Agent instructions
- `AGENTS.md` - Agent workflow requirements

**In Architect Agent Workspace:**
- `instructions/[this file]` - These instructions
- `human/human-[matching].md` - Human-readable summary

## 🎯 Notes

- **Priority:** [Why this is important]
- **Complexity:** [HIGH/MEDIUM/LOW]
- **Risk:** [Potential risks]
- **Dependencies:** [What else this affects]
- **Rollback:** [Easy/Moderate/Difficult and why]
```

## Key Improvements from Feedback

### 1. Add More Structure
- ✅ Clear section headers with emoji icons
- ✅ Table of contents for navigation
- ✅ Success criteria section with checkboxes
- ✅ Completion checklist at end
- ✅ Verification methods for each task

### 2. Include Logging Requirements
- ✅ Reference to LOGGING.md in code agent workspace
- ✅ Timestamp format requirements `[HH:MM:SS]`
- ✅ Checkpoint protocols (every 3-5 commands, every 5 minutes)
- ✅ Real-time logging with `tee` examples
- ✅ Log file structure template

### 3. Add Agent Instructions
- ✅ Specify which sub-agents to use
- ✅ Include qa-enforcer for validation (MANDATORY)
- ✅ When to use each agent
- ✅ DO NOT anti-patterns
- ✅ Agent usage timing sequence

### 4. Documentation Requirements
- ✅ Specify change documentation creation
- ✅ Include rollback procedures
- ✅ PR description template
- ✅ Files changed table format

### 5. Additional Improvements
- ✅ Testing protocol section (progressive testing)
- ✅ Error handling section (specific patterns)
- ✅ Timeline estimates
- ✅ Reference files list
- ✅ Priority/complexity/risk notes

## Usage in Architect Agent Workflow

When creating instructions:

1. **Copy this template** as starting point
2. **Fill in all sections** completely
3. **Customize for task** (add/remove tasks as needed)
4. **Be specific** (exact commands, file paths, line numbers)
5. **Include examples** (before/after code snippets)
6. **Define success** (clear, measurable criteria)
7. **Plan for failure** (error handling for known issues)

## Grade Impact

Using this structured template improves code agent grades:

- **+5 points:** Clear structure and navigation
- **+5 points:** Complete logging requirements with examples
- **+3 points:** Explicit success criteria
- **+3 points:** Proper agent usage instructions
- **+2 points:** Error handling guidance
- **+2 points:** Documentation requirements

**Total potential improvement: +20 points** (from poorly structured instructions)
