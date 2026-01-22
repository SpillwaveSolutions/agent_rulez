# Instruction-Grading Workflow Protocol

## Overview

This protocol enables architect agents to send instructions to code agents, track their execution, grade the work, and iteratively improve until achieving ≥95% quality scores. The workflow uses a single `current_instructions.md` file in the code agent's `debugging/instructions/` directory as a temporary workspace while maintaining all history in the architect agent's directories.

## Directory Separation

### Architect Agent Workspace (YOU work here)
```
/path/to/architect/workspace/
├── instructions/         # Original instructions + archive
├── grades/              # All grade history
├── human/               # Human summaries + archive
├── ticket/              # Ticket tracking
└── CLAUDE.md            # References code agent location
```

### Code Agent Workspace (THEY work there)
```
/path/to/code/agent/workspace/
├── src/                 # Their codebase
├── debugging/
│   ├── logs/           # Execution logs they create
│   └── instructions/   # TEMPORARY workspace - single file only
│       └── current_instructions.md
├── CLAUDE.md           # Includes instruction protocol
└── AGENTS.md           # Includes delegation protocol
```

## File Naming Convention

### Architect Agent Directories
Standard naming (existing convention):
```
instructions/instruct-YYYY_MM_DD-HH_MM-ticket_id_phase_description.md
grades/grade-YYYY_MM_DD-HH_MM-ticket_id_phase_description.md
human/human-YYYY_MM_DD-HH_MM-ticket_id_phase_description.md
```

### Code Agent Temporary Instructions
```
debugging/instructions/current_instructions.md    # Always ONE file, simple name
```

**Principle**: ONE instruction active at a time. All history preserved in architect workspace only.

## Complete Workflow

### Phase 1: Send Instructions to Code Agent

**You tell architect agent:** "send instructions to code agent"

**Architect agent actions:**
1. Create standard instruction file in architect workspace:
   ```
   instructions/instruct-2025_10_30-14_30-tkt123_phase2_implement_auth.md
   ```

2. Copy instruction to code agent workspace with simple naming:
   ```
   /path/to/code-agent/debugging/instructions/current_instructions.md
   ```

3. Generate adaptive summary (based on complexity):
   - Simple task: 3-5 bullet points
   - Medium task: 7-10 bullet points
   - Complex task: 10-15 bullet points with categorization

4. Display to user:
   ```
   📄 File: debugging/instructions/current_instructions.md

   📋 Summary (10 points):
   1. Implement JWT-based authentication
   2. Create user login endpoint
   3. Add password hashing with bcrypt
   4. Implement token refresh mechanism
   5. Add middleware for protected routes
   6. Write unit tests for auth logic
   7. Write integration tests for endpoints
   8. Update API documentation
   9. Add rate limiting to login endpoint
   10. Ensure 80%+ code coverage

   ✅ Instructions ready. Tell code agent to "run instructions"
   ```

### Phase 2: Code Agent Execution

**You tell code agent:** "run instructions"

**Code agent actions:**
1. Check for `debugging/instructions/current_instructions.md`
2. If exists: Run it immediately (no selection needed)
3. If not exists: Report "No current instructions found"
4. Execute tasks per instruction file
5. Create logs in `debugging/logs/`
6. Signal completion: "Instructions completed, ready for grading"

**Simplicity**: Always one file, no multi-file detection needed.

### Phase 3: Grading

**You tell architect agent:** "grade the work"

**Architect agent actions:**

#### Step 1: Grade Current Work
1. Read code agent's logs from `debugging/logs/`
2. Review code changes
3. Apply grading rubric (see `grading_rubrics.md`)
4. Calculate score (0-100)

#### Step 2: Handle Based on Score

**If score ≥95% (SUCCESS):**
```
1. Display grade report
2. Delete current_instructions.md from code agent workspace
3. Archive in architect workspace (already exists)
4. Optionally update code agent's CLAUDE.md with learnings

Output:
✅ Grade: 97%

Excellent work! All criteria met:
- Completeness: 25/25
- Code Quality: 20/20
- Testing: 19/20
- Documentation: 15/15
- Resilience: 10/10
- Logging: 8/10

Instruction file deleted from code agent workspace.

Added to code agent's CLAUDE.md:
## Authentication Patterns
- JWT implementation with refresh tokens (scored 97%)
- Pattern: Separate auth logic from HTTP handlers
- Reuse for future auth features
```

**If score <95% (NEEDS IMPROVEMENT):**
```
1. Display detailed grade report
2. Create improvement instruction in architect workspace:
   instructions/instruct-2025_10_30-16_45-tkt123_phase2b_improve_auth.md

3. REPLACE current_instructions.md in code agent workspace with improvement instructions

4. Display feedback

Output:
📊 Grade: 82%

❌ Missing (18 points):
- Rate limiting not implemented (-10)
- Integration tests incomplete (-5)
- Documentation missing examples (-3)

📝 Updated current_instructions.md with improvement guidance

Summary:
1. Add express-rate-limit to login endpoint
2. Complete integration test suite
3. Add curl examples to API docs

Tell code agent: "improve your score"

Files in code agent workspace:
- current_instructions.md (contains improvement guidance)
```

5. Update code agent's CLAUDE.md with pattern to avoid:
```markdown
## Testing Requirements
- Integration tests must cover happy path + error cases
- You missed error case testing in auth implementation
- Always include: invalid credentials, expired tokens, missing headers
```

### Phase 4: Improvement Iteration

**You tell code agent:** "improve your score"

**Code agent actions:**
1. Read `current_instructions.md` (now contains improvement guidance)
2. Reference logs from previous attempt for context
3. Implement fixes
4. Signal: "Improvements completed, ready for re-grading"

**You tell architect agent:** "grade the work"

**Architect agent re-grading:**
1. Grade new work
2. If ≥95%: Delete `current_instructions.md`, done!
3. If <95%: REPLACE `current_instructions.md` with new improvement instructions, repeat

## Improvement Instruction Structure

When creating improvement instructions, include:

```markdown
# Improvement Instructions
Previous Attempt Score: 82%

## What Was Missing

Based on grading of previous attempt:
- Rate limiting not implemented (lost 10 points)
- Integration tests incomplete (lost 5 points)
- Documentation missing examples (lost 3 points)

## Target Improvement Areas

### 1. Add Rate Limiting (Priority: High)
**What to do:**
- Install express-rate-limit package
- Add to login endpoint: max 5 attempts per 15 minutes
- Test with multiple rapid requests

**Success criteria:**
- Rate limiter blocks after 5 attempts
- Returns 429 status code
- Includes Retry-After header

### 2. Complete Integration Tests (Priority: High)
**What to do:**
- Add error case tests:
  - Invalid credentials
  - Expired tokens
  - Missing authorization headers
- Ensure all endpoints covered

**Success criteria:**
- Test coverage ≥80% for auth module
- All error cases tested
- Tests pass consistently

### 3. Add Documentation Examples (Priority: Medium)
**What to do:**
- Add curl examples to API docs
- Show successful login
- Show error responses

**Success criteria:**
- Examples work when copy-pasted
- Cover both success and error cases

## Context from Previous Attempt

You already completed:
✅ JWT authentication implementation
✅ Password hashing with bcrypt
✅ Token refresh mechanism
✅ Protected route middleware
✅ Basic unit tests

Only work on the improvement areas above.
```

## File Lifecycle Examples

### Success on First Try
```
State 1: Fresh instruction
debugging/instructions/current_instructions.md

↓ Code agent executes

State 2: Grading (97% - success!)
Architect deletes: current_instructions.md

State 3: Clean
debugging/instructions/ (empty, or just README.md)
```

### Two Iteration Cycle
```
State 1: Fresh instruction
debugging/instructions/current_instructions.md

↓ Grade: 82%

State 2: After first grading
Architect REPLACES current_instructions.md with improvement guidance

debugging/instructions/current_instructions.md (now contains improvements needed)

↓ Code agent improves
↓ Grade: 96%

State 3: After second grading (success!)
Architect deletes: current_instructions.md

State 4: Clean
debugging/instructions/ (empty)
```

### Three Iteration Cycle
```
State 1: Fresh instruction
debugging/instructions/current_instructions.md

↓ Grade: 82%

State 2: After first grading
debugging/instructions/current_instructions.md (replaced with improvement v1)

↓ Grade: 89% (still <95%)

State 3: After second grading
debugging/instructions/current_instructions.md (replaced with improvement v2)

↓ Grade: 97% (success!)

State 4: After third grading
Architect deletes: current_instructions.md

State 5: Clean
debugging/instructions/ (empty)
```

## Memory Updates to Code Agent

### CLAUDE.md Updates

Architect updates code agent's CLAUDE.md to embed learnings:

**When to update:**
- Pattern failure (score <95% on same task type 2+ times)
- Significant success (score ≥98%)
- Explicit learning identified

**Example additions:**

```markdown
## Architect-Delegated Instructions Protocol

### How to Receive Instructions
1. Architect creates: `debugging/instructions/current_instructions.md`
2. You receive notification with summary
3. Command: "run instructions" - auto-runs the file

### How Instructions are Graded
- Target score: 95%
- Architect evaluates against rubric in instruction file
- Score ≥95%: Instruction deleted (success)
- Score <95%: File replaced with improvement guidance

### Post-Grading Actions
- "improve your score" - read current_instructions.md for improvement guidance
- Architect updates this section with learnings

---

## Learnings from Architect (Auto-Updated)

### Error Handling Patterns
<!-- Added by architect after repeated failures -->
- Always wrap database operations in try-catch with specific error types
- Log errors with context: `logger.error({operation, params, error})`
- Return structured error responses: `{success: false, error: {code, message}}`

### Testing Requirements
<!-- Added after grading feedback on auth feature (score: 82% → 96%) -->
- Minimum 80% code coverage for new features
- Include edge cases: null, undefined, empty arrays
- Integration tests must cover happy path + error cases
- You missed error case testing in auth implementation
- Always include: invalid credentials, expired tokens, missing headers

### Code Organization
<!-- Added after successful high-scoring implementations -->
- Separate business logic from HTTP handlers (score: 97% on auth feature)
- Use dependency injection for testability
- Keep functions under 50 lines

### Project-Specific Conventions
<!-- Added based on this codebase's patterns -->
- Use async/await, not callbacks
- Configuration via environment variables, validated at startup
- API responses follow `{data, error, metadata}` structure
```

### AGENTS.md Updates

Architect updates code agent's AGENTS.md to document collaboration patterns:

```markdown
## Architect Agent Delegation Protocol

### Receiving Work Instructions
When architect agent delegates work to you (code agent):

1. **Instruction Location**: `debugging/instructions/current_instructions.md`

2. **Instruction Format**:
   ```markdown
   # Instructions

   ## Objective
   [Clear goal]

   ## Success Criteria (Grading Rubric)
   - [ ] Criterion 1 (20 points)
   - [ ] Criterion 2 (30 points)

   ## Tasks
   1. Specific task with acceptance criteria
   2. ...
   ```

3. **Your Response**:
   - Acknowledge receipt
   - Confirm understanding
   - Execute tasks
   - Signal completion: "instructions completed, ready for grading"

4. **Grading Cycle**:
   - Architect evaluates your work
   - Score ≥95%: Success, instruction deleted
   - Score <95%: Improvement instruction replaces current file
   - You respond to: "improve your score"

---

## Architect-Provided Patterns (Auto-Updated)

### When You Struggled Before
<!-- Architect adds these after <95% scores -->

**Database Migrations (Failed 2x, then succeeded)**:
- ❌ Don't modify existing migrations
- ✅ Create new migration file
- ✅ Test rollback before committing

**API Design (Score: 88% → 97%)**:
- Improvement: Added request validation schemas
- Learning: Define schemas before implementing handlers

### When You Excelled
<!-- Architect adds these after ≥95% scores -->

**Authentication Feature (Score: 98%)**:
- Pattern: JWT + refresh token architecture
- Testing: Mocked time-based token expiry
- Reuse this pattern for authorization features

**Performance Optimization (Score: 96%)**:
- Approach: Profiled first, optimized hot paths only
- Tools: Used benchmark suite before/after
- Apply to future optimization tasks
```

## Grading Cleanup Protocol

### When Architect Grades

**Step 1: Grade current work**
- Apply rubric
- Calculate score

**Step 2: Take action based on score**

If score ≥95%:
```bash
# Delete current instruction (success!)
rm /path/to/code-agent/debugging/instructions/current_instructions.md
```

If score <95%:
```bash
# REPLACE current_instructions.md with improvement instructions
# (no renaming, no graded files, just replace)
cp /path/to/architect/instructions/instruct-...-phase2b-improvement.md \
   /path/to/code-agent/debugging/instructions/current_instructions.md
```

### Maximum Files in Directory

**Always:** 0-1 files
- 0 files: No active work
- 1 file: `current_instructions.md` (current task to work on)

**No graded reference files** - all history in architect workspace

## Code Agent Commands

### Running Instructions

**Instruction present:**
```
You: "run instructions"
Code Agent: Automatically runs current_instructions.md
```

**No instruction present:**
```
You: "run instructions"
Code Agent: "No current instructions found in debugging/instructions/"
```

### Improving Score

**After receiving <95% grade:**
```
You: "improve your score"
Code Agent:
1. Reads current_instructions.md (now contains improvement guidance)
2. References previous logs for context
3. Implements targeted improvements
4. Signals completion
```

## Architect Agent Implementation Notes

### Summary Generation
```python
def generate_summary(instruction_content, complexity):
    """
    Generate adaptive summary based on complexity

    Args:
        instruction_content: Full instruction text
        complexity: "simple" | "medium" | "complex"

    Returns:
        List of summary points (3-15 items)
    """
    if complexity == "simple":
        return extract_key_points(instruction_content, max_points=5)
    elif complexity == "medium":
        return extract_key_points(instruction_content, max_points=10)
    else:  # complex
        return extract_categorized_points(instruction_content, max_points=15)
```

### Grading Flow
```python
def grade_work():
    # Step 1: Grade
    score = evaluate_against_rubric()

    # Step 2: Handle result
    if score >= 95:
        delete_current_instructions()
        update_code_agent_memory_if_needed()
        return success_report(score)
    else:
        improvement_content = create_improvement_guidance(score, gaps)
        replace_current_instructions(improvement_content)
        update_code_agent_memory_with_patterns()
        return improvement_report(score)
```

## Integration with Existing Architect Agent Workflow

### Preserving Existing Protocols

This instruction-grading workflow **supplements** the existing architect agent workflow. All existing protocols remain in effect:

- **Logging Protocol** (`logging_protocol.md`): Still required
- **Testing Protocol** (`testing_protocol.md`): Still required
- **Grading Rubrics** (`grading_rubrics.md`): Same scoring system
- **File Naming** (`file_naming.md`): Architect workspace uses existing convention
- **All other references**: Unchanged

### Dual File Creation

When sending instructions to code agent:

1. **Create standard instruction** in architect workspace:
   ```
   instructions/instruct-2025_10_30-14_30-tkt123_phase2_implement_auth.md
   ```
   - Full detail
   - Follows existing instruction_structure.md template
   - Archived normally in architect workspace

2. **Copy to code agent workspace** with simple naming:
   ```
   debugging/instructions/current_instructions.md
   ```
   - Same content
   - Simple naming for code agent convenience
   - Temporary (deleted after successful grading)

### Grade Storage

Grades always stored in architect workspace:
```
grades/grade-2025_10_30-16_45-tkt123_phase2_implement_auth.md
```

No grades stored in code agent workspace.

## Summary

This workflow provides:
1. **Clear instruction delivery** via single `current_instructions.md` file
2. **Iterative improvement** until quality threshold (95%) achieved
3. **Automatic cleanup** after successful completion
4. **Memory retention** by updating code agent's CLAUDE.md and AGENTS.md
5. **Full compatibility** with existing architect agent protocols
6. **Separation of concerns** between architect planning and code execution
7. **Maximum simplicity** - one file, no UUID confusion, no multi-file selection
