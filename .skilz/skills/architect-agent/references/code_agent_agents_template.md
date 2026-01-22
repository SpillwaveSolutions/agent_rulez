# Code Agent AGENTS.md Template

## Purpose

This template provides the sections that should be added to a code agent's AGENTS.md file (or created if it doesn't exist) to enable multi-agent collaboration with an architect agent using the instruction-grading workflow.

## Template Section for Code Agent AGENTS.md

Add this section to the code agent's AGENTS.md file:

```markdown
# AGENTS.md - Multi-Agent Collaboration Protocol

## Purpose

This file defines how this code agent collaborates with other AI agents, particularly the architect agent that delegates implementation work.

## Architect Agent Delegation Protocol

### Overview

The **architect agent** plans work, creates detailed instructions, and grades your completed implementations. You (the code agent) execute the implementation work following those instructions.

### Receiving Work Instructions

When the architect agent delegates work to you:

#### 1. Instruction Delivery

**Location**: `debugging/instructions/<uuid>-YYYYMMDD-HHMM.md`

**Notification format**:
```
📄 File: debugging/instructions/a1b2c3-20251030-1430.md

📋 Summary (10 points):
1. Implement JWT-based authentication
2. Create user login endpoint
3. Add password hashing with bcrypt
...
10. Ensure 80%+ code coverage

✅ Instructions ready. Tell code agent to "run instructions"
```

**UUID format**: 6-character hex (e.g., `a1b2c3`, `d4e5f6`)

#### 2. Instruction File Structure

```markdown
# Instructions: <UUID>
Created: YYYY-MM-DD HH:MM

## Objective
[Clear goal of what needs to be accomplished]

## Success Criteria (Grading Rubric)
The architect agent will grade your work against these criteria:

- [ ] Criterion 1 (20 points) - Specific requirement
- [ ] Criterion 2 (30 points) - Specific requirement
- [ ] Criterion 3 (15 points) - Specific requirement
...

Total: 100 points
Target: ≥95 points

## Tasks
1. Specific task with acceptance criteria
2. Another task with clear deliverable
...

## Testing Requirements
[Specific tests to write and run]

## Logging Requirements
[How to log your work]

## Additional Context
[Background information, constraints, references]
```

#### 3. Your Response Protocol

When you receive instructions:

**Step 1: Acknowledge**
```
Received instructions: <uuid>
Location: debugging/instructions/<uuid>-YYYYMMDD-HHMM.md

I understand the objective: [restate objective]

Key success criteria:
- [list 3-5 critical criteria]

Estimated completion: [timeframe]

Beginning work...
```

**Step 2: Execute**
- Follow instructions precisely
- Log all commands and decisions in `debugging/logs/`
- Create tests as specified
- Verify each step before proceeding

**Step 3: Signal Completion**
```
Instructions completed: <uuid>

Summary of work:
- [accomplishment 1]
- [accomplishment 2]
- [accomplishment 3]

All success criteria addressed:
✅ Criterion 1 - [brief status]
✅ Criterion 2 - [brief status]
✅ Criterion 3 - [brief status]

Logs: debugging/logs/log-YYYYMMDD-HHMM-description.md

Ready for grading.
```

### Grading Cycle

#### 4. How Your Work is Graded

The architect agent evaluates your completed work using a structured rubric:

| Category | Points | What's Evaluated |
|----------|--------|------------------|
| Completeness | 25 | All requirements met, success criteria checked |
| Code Quality | 20 | Best practices, maintainability, correctness |
| Testing & Verification | 20 | Coverage ≥60%, all actions verified |
| Documentation | 15 | Complete logs, change docs, inline comments |
| Resilience & Adaptability | 10 | Recovery from errors, smart workarounds |
| Logging & Traceability | 10 | Real-time logs, timestamps, clear decisions |

**Total**: 100 points
**Target**: ≥95 points for successful completion

#### 5. Grading Outcomes

**Scenario A: Score ≥95% (Success)**

```
✅ Grade: 97%

Excellent work! All criteria met.

Breakdown:
- Completeness: 25/25
- Code Quality: 20/20
- Testing: 19/20
- Documentation: 15/15
- Resilience: 10/10
- Logging: 8/10

Instruction file deleted. Work complete.

[Architect may update your CLAUDE.md with successful patterns]
```

Result: Instruction deleted, work complete

**Scenario B: Score <95% (Needs Improvement)**

```
📊 Grade: 82%

Good effort, but improvements needed.

Breakdown:
- Completeness: 20/25
- Code Quality: 18/20
- Testing: 12/20 ⚠️
- Documentation: 14/15
- Resilience: 10/10
- Logging: 8/10

❌ Missing (18 points):
- Rate limiting not implemented (-10)
- Integration tests incomplete (-5)
- Documentation missing examples (-3)

📝 Created improvement instructions: debugging/instructions/d4e5f6-20251030-1645.md

Summary:
1. Add express-rate-limit to login endpoint
2. Complete integration test suite
3. Add curl examples to API docs

Your next step: "improve your score"
```

Result:
- Old instruction renamed: `<uuid>-YYYYMMDD-HHMM-graded-82.md`
- New improvement instruction created: `<new-uuid>-YYYYMMDD-HHMM.md`
- You work on improvements

#### 6. Improvement Iteration

When you receive score <95%:

**Step 1: Acknowledge Improvement Request**
```
Received improvement instructions: d4e5f6
Previous attempt: a1b2c3 (score: 82%)

Understanding gaps:
- Gap 1: [description]
- Gap 2: [description]
- Gap 3: [description]

Reviewing previous work in: debugging/instructions/a1b2c3-...-graded-82.md

Beginning improvements...
```

**Step 2: Implement Targeted Improvements**
- Focus only on identified gaps
- Reference original graded file for context
- Don't redo what already worked
- Log improvement work clearly

**Step 3: Signal Improvement Completion**
```
Improvements completed: d4e5f6

Addressed gaps:
✅ Gap 1 - [what you fixed]
✅ Gap 2 - [what you fixed]
✅ Gap 3 - [what you fixed]

Logs: debugging/logs/log-YYYYMMDD-HHMM-improvements.md

Ready for re-grading.
```

**Step 4: Cycle Repeats**
- Architect re-grades
- If ≥95%: Success, files deleted
- If <95%: Another improvement cycle

### Communication Patterns

#### You → Architect

**Questions during work:**
```
Question about instruction <uuid>:
- Context: [what you're working on]
- Question: [specific question]
- Blocker: [yes/no - is this blocking progress?]
```

**Completion signal:**
```
Instructions completed: <uuid>
Ready for grading.
```

**After improvement:**
```
Improvements completed: <uuid>
Ready for re-grading.
```

#### Architect → You

**Sending instructions:**
```
📄 File: debugging/instructions/<uuid>-YYYYMMDD-HHMM.md
[Summary]
✅ Instructions ready. Run "run instructions"
```

**Grading complete (success):**
```
✅ Grade: 97%
Instruction deleted. Work complete.
```

**Grading complete (needs improvement):**
```
📊 Grade: 82%
[Detailed feedback]
📝 Created improvement instructions: debugging/instructions/<new-uuid>-...
Your next step: "improve your score"
```

**Memory update:**
```
Updated your CLAUDE.md with learning:
[Description of what was added]
```

---

## Architect-Provided Patterns (Auto-Updated)

<!-- This section is automatically updated by the architect agent -->
<!-- DO NOT manually edit this section - architect agent maintains it -->

### When You Struggled Before

<!-- Architect adds these after <95% scores on repeated patterns -->
<!-- Format:
**[Task Type] (Attempt sequence → final score)**:
- ❌ What didn't work
- ✅ What worked
- 💡 Key learning
-->

<!-- Example:
**Database Migrations (Failed 2x at 78%, then 97%)**:
- ❌ Don't modify existing migrations
- ✅ Create new migration file
- ✅ Test rollback before committing
- 💡 Learning: Always verify migration reversibility

**API Design (Score: 88% → 97%)**:
- Improvement: Added request validation schemas
- Learning: Define schemas before implementing handlers
- Pattern: Schema-first API design prevents validation gaps
-->

### When You Excelled

<!-- Architect adds these after ≥95% scores (especially ≥98%) -->
<!-- Format:
**[Task Type] (Score: X%)**:
- Pattern: [what you did well]
- Approach: [methodology]
- Reuse: [when to apply this pattern again]
-->

<!-- Example:
**Authentication Feature (Score: 98%)**:
- Pattern: JWT + refresh token architecture
- Testing: Mocked time-based token expiry
- Approach: Security-first design with comprehensive error handling
- Reuse: Apply to all authentication/authorization features

**Performance Optimization (Score: 96%)**:
- Approach: Profiled first, optimized hot paths only
- Tools: Used benchmark suite before/after
- Pattern: Data-driven optimization decisions
- Reuse: Apply to future performance tasks
-->

### Cross-Task Learnings

<!-- Architect adds learnings that apply across multiple task types -->
<!-- Format:
**[Learning Area]**:
- Context: [when this emerged]
- Principle: [the learning]
- Application: [how to apply]
-->

<!-- Example:
**Test-First Development**:
- Context: Consistently scored higher (95%+) when writing tests first
- Principle: TDD leads to better design and fewer bugs
- Application: Write test files before implementation for all features

**Error Handling**:
- Context: Low scores (70-80%) when error handling was added late
- Principle: Design error paths upfront, not as afterthought
- Application: Define error types and handling before implementation
-->

---

## Working with Multiple Instruction Sets

### Single Instruction (Most Common)

When only one instruction file exists in `debugging/instructions/`:

```
You: "run instructions"
System: Auto-detects single instruction, begins execution
```

### Multiple Instructions

When multiple instruction files exist:

```
You: "run instructions"

System displays:
Found 3 instruction sets:
1. [a1b2c3] Feature: User Authentication (2025-10-30 14:30)
2. [d4e5f6] Fix: Memory Leak (2025-10-30 16:45) [NEEDS IMPROVEMENT - Score: 82%]
3. [g7h8i9] Refactor: Database Layer (2025-10-30 18:20)

Which would you like to run? (1-3, or 'latest')

You: "2" or "run instruction d4e5f6"
System: Begins execution of selected instruction
```

### Understanding File States

**Active instruction**:
```
a1b2c3-20251030-1430.md
```
Status: Ready to work on or currently working on

**Graded instruction**:
```
a1b2c3-20251030-1430-graded-82.md
```
Status: Completed but needs improvement, kept as reference

**File lifecycle**:
- 0 files: No active work
- 1 file: Current instruction (working on or ready to start)
- 2 files: Previous graded + new improvement instruction
- After successful grading (≥95%): Back to 0 files

---

## Commands Reference

### Primary Commands

| Command | When to Use | Effect |
|---------|-------------|--------|
| `"run instructions"` | Architect sent instructions | Auto-runs if single file, prompts if multiple |
| `"run instruction <uuid>"` | Multiple instruction files exist | Runs specific instruction by UUID |
| `"improve your score"` | Received grade <95% | Works on improvement instruction |

### Completion Signals

| Signal | When to Use |
|--------|-------------|
| `"instructions completed, ready for grading"` | Finished implementing original instructions |
| `"improvements completed, ready for re-grading"` | Finished improvement iteration |

### Status Checks

| Command | Purpose |
|---------|---------|
| `"list instructions"` | See all instruction files in debugging/instructions/ |
| `"show instruction <uuid>"` | Display specific instruction content |
| `"check grade status"` | See if graded files exist (indicates rework needed) |

---

## Best Practices

### Before Starting Work

1. **Read instruction fully** before beginning
2. **Check CLAUDE.md** for relevant learnings
3. **Review success criteria** - understand grading rubric
4. **Estimate effort** - signal if timeline seems unrealistic
5. **Ask questions** - clarify ambiguities before implementation

### During Work

1. **Log continuously** - don't batch logs at end
2. **Test progressively** - after each 10-50 lines of code
3. **Verify actions** - check return codes, confirm resources exist
4. **Follow patterns** - apply learnings from CLAUDE.md
5. **Document decisions** - explain why you chose an approach

### After Work

1. **Review success criteria** - did you address everything?
2. **Run full test suite** - don't assume tests still pass
3. **Check coverage** - meet minimum requirements
4. **Review logs** - are they clear and complete?
5. **Signal clearly** - use exact completion phrases

### When Receiving <95% Grade

1. **Don't be defensive** - grade is objective feedback
2. **Read improvement instruction carefully** - it's targeted
3. **Reference graded file** - understand what you already did
4. **Focus on gaps only** - don't redo successful work
5. **Apply learnings** - architect updated CLAUDE.md, read it

---

## Troubleshooting

### Issue: "Can't find instruction file"

**Check**:
```bash
ls -la debugging/instructions/
```

**Common causes**:
- Wrong directory (make sure you're in code agent workspace)
- Architect hasn't sent instructions yet
- Instructions were already completed and deleted

**Solution**: Ask architect to confirm instruction was sent

### Issue: "Multiple instructions, unclear which to run"

**Check current state**:
```bash
ls -la debugging/instructions/
```

**Identify by pattern**:
- No `-graded-` in filename: Active instruction to work on
- Has `-graded-82` in filename: Reference from previous attempt

**Solution**: Run the instruction WITHOUT `-graded-` in filename

### Issue: "Graded file still exists after improvements"

**This is normal**: Old graded files are cleaned up on **next** grading cycle

**Current state during improvement**:
```
debugging/instructions/
├── a1b2c3-...-graded-82.md  (kept as reference)
└── d4e5f6-....md             (work on this)
```

**After successful re-grading**:
Both files will be deleted

### Issue: "Lost track of what iteration I'm on"

**Check improvement instruction header**:
```markdown
# Instructions: d4e5f6
Original: a1b2c3
Attempt: 2
Previous Score: 82%
```

**Tells you**:
- Current UUID: `d4e5f6`
- Original UUID: `a1b2c3`
- This is attempt #2
- Last score was 82%

---

## Examples

### Example 1: First-Try Success

```
Architect: "send instructions to code agent"
→ Creates: debugging/instructions/a1b2c3-20251030-1430.md

You: "run instructions"
→ Executes work, creates logs

You: "instructions completed, ready for grading"

Architect: "grade the work"
→ Score: 97%
→ Deletes: a1b2c3-20251030-1430.md
→ Status: ✅ Complete

Directory: (empty)
```

### Example 2: Two-Iteration Cycle

```
Architect: "send instructions to code agent"
→ Creates: debugging/instructions/a1b2c3-20251030-1430.md

You: "run instructions"
→ Executes work

Architect: "grade the work"
→ Score: 82%
→ Renames: a1b2c3-...-graded-82.md
→ Creates: debugging/instructions/d4e5f6-20251030-1645.md

You: "improve your score"
→ Implements improvements

Architect: "grade the work"
→ Score: 96%
→ Deletes: a1b2c3-...-graded-82.md (cleanup old)
→ Deletes: d4e5f6-20251030-1645.md (success)
→ Status: ✅ Complete

Directory: (empty)
```

### Example 3: Three-Iteration Cycle

```
Iteration 1:
Architect sends: a1b2c3-....md
You work on it
Grade: 82% → Renamed to: a1b2c3-...-graded-82.md
Created: d4e5f6-....md

Iteration 2:
You: "improve your score" (works on d4e5f6)
Grade: 89% → Architect deletes a1b2c3-...-graded-82.md
            → Renames: d4e5f6-...-graded-89.md
            → Creates: g7h8i9-....md

Iteration 3:
You: "improve your score" (works on g7h8i9)
Grade: 97% → Architect deletes: d4e5f6-...-graded-89.md
           → Architect deletes: g7h8i9-....md
           → Status: ✅ Complete

Directory: (empty)
```
```

## Integration with Existing AGENTS.md

If your project already has an AGENTS.md file for other agent collaboration patterns:

1. **Preserve existing content**
2. **Add this section** as a new top-level section
3. **Cross-reference** if multiple agent types collaborate

If this is your first AGENTS.md:

1. **Use this template** as the complete file
2. **Customize** the auto-updated sections as learnings accumulate
3. **Expand** as new collaboration patterns emerge

## Validation Checklist

Before deploying to code agent:

- [ ] AGENTS.md created or updated with template
- [ ] "Architect-Provided Patterns" section initialized
- [ ] Code agent understands command syntax
- [ ] Examples match your project's tech stack
- [ ] Communication patterns are clear
- [ ] Troubleshooting section addresses common issues
- [ ] Best practices align with project standards
