# Code Agent CLAUDE.md Template

## Purpose

This template provides the sections that should be added to a code agent's CLAUDE.md file to enable the instruction-grading workflow with an architect agent.

## Template Section for Code Agent CLAUDE.md

Add this section to the code agent's CLAUDE.md file:

```markdown
## Architect-Delegated Instructions Protocol

### How to Receive Instructions

When the architect agent delegates work to you:

1. **Instruction Location**: `debugging/instructions/<uuid>-YYYYMMDD-HHMM.md`
   - UUID: 6-character hex identifier (e.g., `a1b2c3`)
   - Format: YYYYMMDD-HHMM (e.g., `20251030-1430`)

2. **Notification Format**:
   ```
   📄 File: debugging/instructions/a1b2c3-20251030-1430.md

   📋 Summary (10 points):
   1. Task 1 description
   2. Task 2 description
   ...
   10. Task 10 description

   ✅ Instructions ready. Run "run instructions"
   ```

3. **Commands**:
   - `"run instructions"` - Auto-runs if only one instruction file exists
   - `"run instruction <uuid>"` - Runs specific instruction by UUID (e.g., "run instruction a1b2c3")

### How Instructions are Graded

**Target Score**: 95% or higher

**Grading Process**:
1. You complete work and signal: "instructions completed, ready for grading"
2. Architect agent evaluates your work against rubric in instruction file
3. Architect assigns score (0-100 points)

**Outcomes**:
- **Score ≥95%**: Success! Instruction file deleted, work complete
- **Score <95%**: Improvement needed
  - Old instruction renamed: `<uuid>-YYYYMMDD-HHMM-graded-<score>.md`
  - New improvement instruction created with different UUID
  - You receive targeted guidance on what to improve

### Post-Grading Actions

**If you receive a score <95%:**

1. **You'll see**:
   ```
   📊 Grade: 82%

   ❌ Missing (18 points):
   - Issue 1 description (-10 points)
   - Issue 2 description (-5 points)
   - Issue 3 description (-3 points)

   📝 Created improvement instructions: debugging/instructions/d4e5f6-20251030-1645.md

   Summary:
   1. Specific fix 1
   2. Specific fix 2
   3. Specific fix 3

   Tell code agent: "improve your score"
   ```

2. **You respond**: "improve your score"

3. **You then**:
   - Read new improvement instruction
   - Reference original graded file for context
   - Implement targeted fixes
   - Signal completion: "improvements completed, ready for re-grading"

4. **Cycle repeats** until score ≥95%

### File Lifecycle in debugging/instructions/

**Active instruction**:
```
debugging/instructions/a1b2c3-20251030-1430.md
```

**After grading (score <95%)**:
```
debugging/instructions/
├── a1b2c3-20251030-1430-graded-82.md  (reference from previous attempt)
└── d4e5f6-20251030-1645.md             (new improvement instruction to work on)
```

**After next grading (score ≥95%)**:
```
debugging/instructions/
└── (empty - all work completed successfully)
```

**Maximum files**: 0-2 at any time
- Old graded files automatically cleaned up on next grading cycle
- Successful completion deletes all related files

---

## Learnings from Architect (Auto-Updated)

<!-- This section is automatically updated by the architect agent based on grading outcomes -->
<!-- DO NOT manually edit this section - architect agent maintains it -->

### Error Handling Patterns

<!-- Added by architect after repeated failures or successes -->
<!-- Example:
- Always wrap database operations in try-catch with specific error types
- Log errors with context: `logger.error({operation, params, error})`
- Return structured error responses: `{success: false, error: {code, message}}`
-->

### Testing Requirements

<!-- Added after grading feedback -->
<!-- Example:
- Minimum 80% code coverage for new features
- Include edge cases: null, undefined, empty arrays
- Integration tests must cover happy path + error cases
- Always include: invalid credentials, expired tokens, missing headers
-->

### Code Organization

<!-- Added after successful high-scoring implementations -->
<!-- Example:
- Separate business logic from HTTP handlers (scored 97% on auth feature)
- Use dependency injection for testability
- Keep functions under 50 lines
-->

### Project-Specific Conventions

<!-- Added based on this codebase's patterns -->
<!-- Example:
- Use async/await, not callbacks
- Configuration via environment variables, validated at startup
- API responses follow `{data, error, metadata}` structure
-->

### Performance Patterns

<!-- Added after performance-related work -->
<!-- Example:
- Profile before optimizing (scored 96% on query optimization)
- Use connection pooling for database
- Cache frequently accessed data with TTL
-->

### Security Patterns

<!-- Added after security-related work -->
<!-- Example:
- Always validate and sanitize user input
- Use parameterized queries (never string concatenation)
- Hash passwords with bcrypt (min 10 rounds)
- Implement rate limiting on authentication endpoints
-->
```

## Customization Guide

### When Architect Updates This Section

The architect agent will add entries to the "Learnings from Architect" section when:

1. **Pattern Failure** (score <95% on same type of task 2+ times)
   - Documents what went wrong
   - Provides correct approach
   - Prevents future mistakes

2. **Significant Success** (score ≥98%)
   - Captures successful patterns
   - Marks as reusable approach
   - Encourages consistency

3. **Explicit Learning** (architect identifies general principle)
   - Documents architectural decisions
   - Clarifies project conventions
   - Establishes standards

### Update Format

Each learning entry should include:
- **Context**: When/why this pattern emerged (score, task type)
- **Pattern**: Clear, actionable guidance
- **Example**: Code snippet or command if applicable

### Example Learning Entry

```markdown
### Testing Requirements

**Added: 2025-10-30 - After score 82% on auth feature (attempt 1), 96% on retry**

**What was missing**:
- Integration tests did not cover error cases
- Only tested happy path scenarios

**Correct pattern**:
- Integration tests must cover both success and failure scenarios
- Required error cases for auth:
  - Invalid credentials (401)
  - Expired tokens (401)
  - Missing authorization header (401)
  - Malformed tokens (400)
  - Rate limit exceeded (429)

**Code example**:
```javascript
describe('POST /auth/login', () => {
  it('should return 401 for invalid credentials', async () => {
    const response = await request(app)
      .post('/auth/login')
      .send({ username: 'user', password: 'wrong' });
    expect(response.status).toBe(401);
    expect(response.body.error).toBeDefined();
  });
});
```
```

## Integration with Existing CLAUDE.md

This template section should be **added** to the code agent's existing CLAUDE.md, not replace it. Typical placement:

1. **Before**: Existing project-specific instructions
2. **This section**: Architect-Delegated Instructions Protocol
3. **After**: Learnings from Architect section (auto-updated)
4. **End**: Existing documentation, references, etc.

## Architect Agent Responsibilities

When the architect agent updates the code agent's CLAUDE.md:

1. **Preserve existing content**: Never delete or modify existing sections
2. **Update only "Learnings from Architect"**: Add new entries under appropriate subsections
3. **Follow format**: Use consistent markdown formatting
4. **Include context**: Always note when/why the learning was added
5. **Be specific**: Provide actionable guidance, not generic advice

## Code Agent Responsibilities

When working with architect-delegated instructions:

1. **Read instructions carefully**: Understand full requirements before starting
2. **Follow established patterns**: Check "Learnings from Architect" for relevant guidance
3. **Signal clearly**: Use exact phrases for completion/grading
4. **Ask questions**: If instruction is unclear, ask architect before proceeding
5. **Document work**: Create detailed logs in `debugging/logs/`
6. **Test thoroughly**: Follow testing requirements in instruction

## Example Full CLAUDE.md Section

```markdown
# CLAUDE.md - MyProject Code Agent

## Project Overview
[Existing project description...]

## Build and Development
[Existing build commands...]

## Architect-Delegated Instructions Protocol

### How to Receive Instructions
[Content from template above...]

### How Instructions are Graded
[Content from template above...]

### Post-Grading Actions
[Content from template above...]

### File Lifecycle in debugging/instructions/
[Content from template above...]

---

## Learnings from Architect (Auto-Updated)

### Testing Requirements

**Added: 2025-10-30 - Authentication feature (82% → 96%)**
- Integration tests must cover error cases
- Required: invalid credentials, expired tokens, missing headers

### Code Organization

**Added: 2025-10-28 - User service refactor (97%)**
- Separate business logic from HTTP handlers
- Pattern successfully applied in auth implementation

## [Existing sections continue...]
```

## Validation Checklist

Before deploying to code agent:

- [ ] Template section added to CLAUDE.md
- [ ] "Learnings from Architect" section initialized (empty subsections)
- [ ] Existing CLAUDE.md content preserved
- [ ] Formatting is consistent
- [ ] Code agent understands command syntax ("run instructions", "improve your score")
- [ ] Architect agent has write access to code agent's CLAUDE.md
- [ ] Code agent workspace path configured in architect's CLAUDE.md
