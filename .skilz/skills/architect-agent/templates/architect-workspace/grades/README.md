# Grades Directory

This directory contains grading reports for code agent work.

## File Naming Convention

```
grade-YYYYMMDD_HHMMSS-brief_description.md
```

**Must match corresponding instruction file timestamp.**

**Example:** `grade-20251120_143045-implement_api_refactor.md`

## Grading Rubric (100 points)

### 1. Instruction Adherence (25 points)
- Followed all instructions exactly
- Completed all requirements
- No unauthorized deviations

### 2. Code Quality (20 points)
- Clean, maintainable code
- Proper error handling
- Security best practices
- No anti-patterns

### 3. Testing & Validation (20 points)
- All tests passing
- Adequate coverage
- Integration tests included
- Edge cases handled

### 4. Logging & Traceability (10 points)
- Hooks capturing all tool calls
- Manual decisions logged
- Clear progression in logs

### 5. Communication & Documentation (15 points)
- Clear progress updates
- Code comments where needed
- README updates if applicable

### 6. Problem Solving (10 points)
- Effective debugging
- Root cause analysis
- Creative solutions to blockers

## Grade Levels

- **90-100:** Excellent - Production ready, no issues
- **80-89:** Good - Minor improvements needed
- **70-79:** Satisfactory - Some rework required
- **60-69:** Needs Improvement - Significant issues
- **0-59:** Unacceptable - Major problems, needs re-work

## Workflow

1. Code agent completes work following instructions
2. Architect reviews logs, code changes, test results
3. Architect creates grade report with scores and feedback
4. Grade informs future instruction quality and agent performance

## See Also

- `instructions/` directory - Original instructions for comparison
- `~/.claude/skills/architect-agent/references/grading_rubrics.md` - Detailed rubrics
