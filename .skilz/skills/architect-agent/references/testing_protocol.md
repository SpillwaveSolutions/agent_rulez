# Testing Protocol for Code Agent (MANDATORY)

## Overview

**CRITICAL:** The code agent MUST follow this testing discipline for ALL work. Testing is not optional—it is a core requirement that directly impacts grading.

## Testing Commands (Taskfile-based Project)

The project uses **Taskfile** for all operations. Available commands:

```bash
task test          # Run unit tests (tests/unit/)
task cov           # Run unit tests with coverage report (must be >= 60%)
task test-int      # Run integration tests (tests/integration/)
task cov-int       # Run integration tests with coverage
task cov-all       # Run all tests with complete coverage report
```

## Progressive Testing Schedule by Phase

**CRITICAL:** Testing requirements increase as work progresses through ticket phases.

### During Each Phase (Early/Middle Phases)

```bash
task test
# MANDATORY: Run after EVERY code change
# If ANY test fails, STOP and fix immediately
# Do NOT proceed until all tests pass
```

**Rules:**
- Run after every 10-50 line code change
- Fix failures immediately before continuing
- Do NOT commit code with failing tests
- Log test results in work log

### After Every Few Phases (Milestone Phases)

```bash
task cov           # Unit test coverage check (must be >= 60%)
task test-int      # Integration tests
```

**When to run:**
- After completing major feature
- Before creating PR
- At natural integration points
- Every 3-5 phases of work

### Before Final Phase of Ticket (Pre-Completion)

```bash
task test          # All unit tests pass
task test-int      # All integration tests pass
task cov           # Unit coverage >= 60%
task cov-int       # Integration coverage documented
task cov-all       # Complete coverage report
```

**Completion Requirements:**
ALL commands must pass before marking ticket complete.

## Development Workflow

### 1. Start Every Session

```bash
task cov
# Record baseline coverage percentage
# Must be >= 60% to start
```

**Log the baseline:**
```markdown
[HH:MM:SS] Baseline Test Coverage
**Command:** `task cov`
**Result:** 65% coverage (388/588 tests passing)
**Status:** ✅ Above 60% threshold
```

### 2. After EVERY Small Change

```bash
task test
# Verify tests still pass before continuing
# If ANY test fails, STOP and fix immediately
```

**Pattern:**
1. Make small change (10-50 lines)
2. Run `task test` immediately
3. If failures, fix before proceeding
4. Log result
5. Repeat

**Bad pattern (don't do this):**
```
❌ Make 10 changes → Run tests once at end
✅ Make 1 change → Test → Make 1 change → Test
```

### 3. Incremental Development Pattern

```
Write code (10-50 lines)
    ↓
task test
    ↓
All pass? → Continue
    ↓
Fail? → Fix immediately
    ↓
Run task test again
    ↓
Repeat until fixed
    ↓
Continue to next change
```

### 4. At Milestone Phases

```bash
# Check coverage hasn't degraded
task cov

# Run integration tests
task test-int

# Document results
```

## Completion Requirements (NON-NEGOTIABLE)

**Before a code agent can report ANY task as complete:**

### 1. Unit Tests MUST Pass

```bash
task test
# ALL tests must pass (0 failures)
# Code agent CANNOT say "done" with failing tests
```

**Log requirement:**
```markdown
[HH:MM:SS] Final Unit Test Verification
**Command:** `task test`
**Result:** ✅ 588/588 tests passing (0 failures)
**Status:** COMPLETE - Ready for commit
```

### 2. Coverage MUST be >= 60%

```bash
task cov
# Must show >= 60% coverage
# If below 60%, add tests to restore it
```

**If below 60%:**
1. Identify uncovered code
2. Write additional tests
3. Run `task cov` again
4. Repeat until >= 60%

### 3. Before ANY Git Commit

```bash
task test-int
# Integration tests must pass

task cov-int
# Integration coverage recorded
```

**Commit checklist:**
- [ ] Unit tests passing (`task test`)
- [ ] Coverage >= 60% (`task cov`)
- [ ] Integration tests passing (`task test-int`)
- [ ] Integration coverage documented (`task cov-int`)

### 4. Before Marking Ticket Complete (Final Phase)

```bash
task test          # All unit tests
task test-int      # All integration tests
task cov           # Unit coverage report
task cov-int       # Integration coverage report
task cov-all       # Complete coverage report
```

**Final checklist:**
- [ ] All unit tests pass
- [ ] All integration tests pass
- [ ] Unit coverage >= 60%
- [ ] Integration coverage documented
- [ ] Complete coverage report generated
- [ ] All results logged

**YOU CANNOT mark ticket complete without ALL of these.**

## Error Handling

### When Tests Fail

**DO NOT:**
- Ignore the failure
- Comment out the failing test
- Mark work as complete
- Commit code with failing tests

**DO:**
1. Read the error message carefully
2. Use `root-cause-debugger` agent to diagnose
3. Use Perplexity MCP to research the error
4. Fix the underlying issue
5. Run `task test` again
6. Repeat until all tests pass
7. Log what you found and how you fixed it

**Example log entry:**
```markdown
[HH:MM:SS] Test Failure Investigation
**Test:** `test_contact_scoring_integration`
**Error:** `AssertionError: Expected 85, got 0`
**Root Cause:** New LLM columns not initialized in test fixtures
**Fix:** Updated fixtures to include default LLM scores
**Verification:** `task test` → ✅ 588/588 passing
```

### When Coverage Drops Below 60%

**Recovery steps:**
1. Identify which files have low coverage
   ```bash
   task cov | grep -A 20 "TOTAL"
   ```
2. Find the uncovered lines
3. Write tests to cover those lines
4. Run `task cov` again
5. Verify >= 60%

## CI/CD Testing Protocol (CRITICAL)

For ANY CI/CD configuration changes, the code agent MUST:

### 1. Trigger Actual Workflows

```bash
# Test main deployment
gh workflow run deploy-main.yml --ref main -f environment=DEV

# Monitor execution
gh run watch [run-id]

# Check results
gh run view [run-id] --log
```

### 2. Test PR Workflows

```bash
# Create test branch and PR
git checkout -b test/verify-cicd
echo "# Test" >> README.md
git add . && git commit -m "test: Verify CI/CD"
git push origin test/verify-cicd
gh pr create --title "Test CI/CD" --body "Verification"

# Monitor PR checks
gh pr checks [pr-number] --watch
```

### 3. Document Success

**Required in logs:**
- Run IDs of successful workflows
- URLs to workflow runs
- Screenshots or output of passing checks
- Verification that all jobs completed

**Example:**
```markdown
[HH:MM:SS] CI/CD Verification Complete
**Workflow:** infrastructure.yml
**Run ID:** #12345
**URL:** https://github.com/org/repo/actions/runs/12345
**Status:** ✅ All jobs passed (deploy, test, validate)
**Evidence:** [See attached screenshot]
```

### 4. Maximum Grade Without Testing

**CI/CD tasks without testing verification:**
- **Maximum possible grade: C+ (78%)**
- **Deductions:**
  - 10 points: Incomplete without verification
  - 7 points: Process not followed
  - 15 points: No testing validation

**Key Principle:**
> **"Configuration without testing = incomplete task"**

This is non-negotiable. Never claim CI/CD work is complete without showing successful workflow runs.

## Grading Implications

### Automatic Grade Caps if Testing Not Performed

| Failure | Maximum Grade | Reason |
|---------|---------------|--------|
| Unit tests not run during phase | D (65%) | Process not followed |
| Unit tests fail | F (50%) | UNACCEPTABLE - broken code |
| Coverage below 60% | C- (70%) | Insufficient test coverage |
| Integration tests not run before commit | C+ (78%) | Incomplete verification |
| Final phase without full test suite | C+ (78%) | Requirements not met |
| CI/CD not tested | C+ (78%) | Configuration unverified |

### Why These Caps Exist

1. **Broken code is unacceptable** - Failing tests = F grade
2. **Incomplete work is incomplete** - No testing = max C+ grade
3. **Standards must be maintained** - Coverage < 60% = max C- grade
4. **CI/CD must be verified** - Untested workflows = max C+ grade

## Instructions Template for Code Agent

When creating instructions for the code agent, ALWAYS include this section:

```markdown
## Testing Protocol (MANDATORY)

**Phase-Based Testing Requirements:**

### During This Phase:
- ✅ Run `task test` after EVERY code change
- ✅ Fix any test failures IMMEDIATELY before proceeding
- ✅ Do NOT commit code with failing tests

### [If Milestone Phase] Additional Requirements:
- ✅ Run `task cov` to verify >= 60% coverage
- ✅ Run `task test-int` to verify integration tests

### [If Final Phase] Complete Test Suite Required:
Before marking this ticket complete, you MUST run:
```bash
task test          # All unit tests must pass
task test-int      # All integration tests must pass
task cov           # Verify >= 60% coverage
task cov-int       # Document integration coverage
task cov-all       # Complete coverage report
```

**YOU CANNOT REPORT THIS TASK AS COMPLETE WITHOUT:**
1. ✅ All tests passing (0 failures)
2. ✅ Coverage >= 60%
3. ✅ Integration tests verified
4. ✅ [Final phase only] Complete test suite run

**Run `task test` after EVERY change. Fix failures immediately.**

If tests fail:
- Use `root-cause-debugger` agent to diagnose
- Use Perplexity MCP to research errors
- Fix the issue, then run `task test` again
- Do NOT proceed to next step until ALL tests pass
```

## Storage Locations

This protocol MUST be documented in:
- ✅ Architect's architect-agent skill (references/testing_protocol.md)
- ✅ Architect's CLAUDE.md (reference to skill)
- ✅ Code agent's `.claude/CLAUDE.md` (in acme-sales_leads_gen-2)
- ✅ Code agent's `AGENTS.md` (in acme-sales_leads_gen-2)
- ✅ Every instruction file sent to code agent

## Key Reminders

1. **Test after every change** - Not just at the end
2. **Fix failures immediately** - Don't accumulate technical debt
3. **Coverage >= 60%** - Non-negotiable requirement
4. **CI/CD must be tested** - Configuration without testing = incomplete
5. **Log all test results** - Evidence is required for grading
6. **Use agents when stuck** - root-cause-debugger, perplexity, context7
7. **Never commit failing tests** - Absolute rule

## Examples

### Good Example (A+ Grade Potential)

```markdown
[14:30:00] Implemented contact scoring logic
**Files Modified:** app/models/contact.py, app/services/scoring.py
**Lines Changed:** 45 lines added, 12 modified

[14:30:15] Running unit tests
**Command:** `task test`
**Result:** ✅ 588/588 passing
**Status:** All tests pass, continuing

[14:35:00] Added LLM scoring integration
**Files Modified:** app/services/llm_scorer.py
**Lines Changed:** 78 lines added

[14:35:20] Running unit tests
**Command:** `task test`
**Result:** ❌ 2 failures in test_llm_integration
**Error:** AttributeError: 'NoneType' object has no attribute 'score'

[14:35:30] Investigating test failures
**Tool Used:** root-cause-debugger agent
**Finding:** LLM client not initialized in test fixtures
**Fix:** Updated conftest.py to mock LLM client

[14:37:00] Re-running unit tests
**Command:** `task test`
**Result:** ✅ 588/588 passing
**Status:** All fixed, continuing

[14:50:00] Milestone: Scoring feature complete
**Command:** `task cov`
**Result:** 67% coverage (up from 65% baseline)
**Status:** ✅ Above 60% threshold

[14:51:00] Running integration tests
**Command:** `task test-int`
**Result:** ✅ 45/45 integration tests passing
**Status:** Ready for commit
```

**Why A+ potential:** Tests run after every change, failures fixed immediately, coverage verified, integration tests run.

### Bad Example (C+ Maximum)

```markdown
[14:30:00] Implemented all features for Phase 5
**Files Modified:** Multiple files
**Lines Changed:** 500+ lines

[16:45:00] Running tests now
**Command:** `task test`
**Result:** ❌ 15 failures
**Status:** Will fix later

[16:46:00] Marking Phase 5 as complete
**Note:** Tests have some failures but feature works manually
```

**Why C+ maximum:** Tests not run during development, failures ignored, work marked complete with failing tests = UNACCEPTABLE.
