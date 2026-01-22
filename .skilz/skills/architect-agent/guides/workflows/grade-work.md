# Workflow: Grade Completed Work

**Trigger:** User says "grade the code agent's work" or "evaluate completed work"

## Prerequisites

Before grading, verify:

```bash
# Check grades directory exists
ls -la grades/

# Verify you have access to code agent logs
ls -la [CODE_AGENT_WORKSPACE]/debugging/logs/
```

## Workflow Steps

### 1. Locate the Original Instructions

Find the instruction file that corresponds to the work being graded:

```bash
ls -lt instructions/instruct-*.md | head -5
```

Extract the timestamp for the grade file:

```bash
TIMESTAMP=$(echo "instruct-20251127_143045-description.md" | grep -oP '\d{8}_\d{6}')
```

### 2. Review Code Agent Logs

```bash
# Find the log file
ls -lt [CODE_AGENT]/debugging/logs/log_*.md | head -3

# Review the log
cat [CODE_AGENT]/debugging/logs/log_[DATE]-description.md
```

**What to look for:**
- Pre-work checklist completed
- Decision types used (decision, rationale, investigation, verification, deviation, milestone)
- Tool calls captured
- Tests run and results
- Blockers encountered and solutions

### 3. Review Code Changes

```bash
cd [CODE_AGENT_WORKSPACE]
git log --oneline -10
git diff HEAD~1
```

**What to verify:**
- Changes match instruction requirements
- Code quality standards met
- No unrelated changes

### 4. Verify Tests

```bash
# Run tests in code agent workspace
cd [CODE_AGENT_WORKSPACE]
[TEST_COMMAND]  # e.g., npm test, ./gradlew test, pytest
```

### 5. Apply Grading Rubric

**Total: 100 points**

| Category | Points | Criteria |
|----------|--------|----------|
| Instruction Adherence | 25 | Followed all requirements, completed objectives |
| Code Quality | 20 | Clean code, error handling, no anti-patterns |
| Testing & Validation | 20 | Tests pass, adequate coverage, edge cases |
| Logging & Traceability | 10 | Hooks working, decision types used, clear progression |
| Communication & Documentation | 15 | Progress updates, code comments, README updates |
| Problem Solving | 10 | Effective debugging, root cause analysis, creative solutions |

### 6. Create Grade File

**File:** `grades/grade-${TIMESTAMP}-description.md`

**Structure:**
```markdown
# Grade: [Title]

**Date:** YYYY-MM-DD
**Instruction File:** instruct-${TIMESTAMP}-description.md
**Log File:** log_[DATE]-description.md

## Overall Score: XX/100

**Grade Level:** [Excellent/Good/Satisfactory/Needs Improvement/Unacceptable]

## Category Scores

### 1. Instruction Adherence (XX/25)
- [What was done well]
- [What was missed]

### 2. Code Quality (XX/20)
- [Strengths]
- [Issues]

### 3. Testing & Validation (XX/20)
- Test results: [Pass/Fail]
- Coverage: XX%
- [Comments]

### 4. Logging & Traceability (XX/10)
- Pre-work checklist: [Complete/Incomplete]
- Decision types used: [List]
- Hook capture: [Working/Not working]

### 5. Communication & Documentation (XX/15)
- [Documentation quality]
- [Code comments]

### 6. Problem Solving (XX/10)
- [How blockers were handled]
- [Creative solutions]

## Summary

[2-3 sentence summary of overall performance]

## Action Items (if grade < 80)

- [ ] Issue 1 to fix
- [ ] Issue 2 to fix

## Recommendations

[Suggestions for future work]
```

### 7. Determine Next Action

**If Grade >= 80:**
- Mark phase/ticket complete
- Archive instruction, human summary, and grade
- Move to next phase or ticket

**If Grade < 80:**
- Create new instructions addressing issues
- Reference this grade in new instructions
- Re-execute and re-grade

## Grade Levels

| Score | Level | Action |
|-------|-------|--------|
| 90-100 | Excellent | Production ready, proceed |
| 80-89 | Good | Minor cleanup, then proceed |
| 70-79 | Satisfactory | Some rework required |
| 60-69 | Needs Improvement | Significant issues |
| 0-59 | Unacceptable | Major problems, full redo |

## Quick Checklist

- [ ] Original instructions located
- [ ] Code agent logs reviewed
- [ ] Code changes verified
- [ ] Tests executed and verified
- [ ] All 6 categories scored
- [ ] Grade file created with matching timestamp
- [ ] Next action determined