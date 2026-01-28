# Grading Rubrics for Code Agent Work

## Overview

After the code agent completes work, perform thorough verification and grading based on these rubrics. The grade reflects completeness, resilience, verification thoroughness, and documentation quality.

## Grading Categories (100 Points Total)

### 1. Completeness (25 points)

**Measures:** Were all requirements and success criteria met?

| Score | Criteria |
|-------|----------|
| 25 | All requirements met, all success criteria checked off |
| 20 | Most requirements met, minor gaps |
| 15 | Significant requirements missing |
| 10 | Major functionality incomplete |
| 0-9 | Critical failures, work unusable |

**Check:**
- [ ] All files created that were supposed to be created
- [ ] All modifications made as specified
- [ ] All tests written and passing
- [ ] All success criteria from instructions met

### 2. Code Quality (20 points)

**Measures:** Quality, maintainability, and correctness of code

| Score | Criteria |
|-------|----------|
| 20 | Excellent quality, follows best practices, well-structured |
| 16 | Good quality, minor improvements possible |
| 12 | Acceptable quality, some issues present |
| 8 | Poor quality, significant issues |
| 0-7 | Unacceptable quality, major problems |

**Check:**
- Code follows project conventions
- Proper error handling
- No security vulnerabilities
- Readable and maintainable
- Appropriate abstractions

### 3. Testing & Verification (20 points)

**Measures:** Test coverage, verification of actions, proof of correctness

| Score | Criteria |
|-------|----------|
| 20 | Comprehensive testing, all actions verified |
| 16 | Good testing, most actions verified |
| 12 | Basic testing, some verification gaps |
| 8 | Minimal testing, poor verification |
| 0-7 | No testing or verification |

**Check:**
- [ ] Unit tests written and passing
- [ ] Integration tests run (if applicable)
- [ ] Coverage >= 60% (for code projects)
- [ ] Every command's return code checked
- [ ] Resources verified to exist after creation
- [ ] Configurations tested to work

### 4. Documentation (15 points)

**Measures:** Log quality, change documentation, inline comments

| Score | Criteria |
|-------|----------|
| 15 | Excellent logs, comprehensive change docs |
| 12 | Good logs, adequate documentation |
| 9 | Basic logs, minimal documentation |
| 6 | Poor logs, insufficient documentation |
| 0-5 | No logs or documentation |

**Check:**
- [ ] Log file created with matching description
- [ ] Every action logged with timestamp
- [ ] Results logged for every action
- [ ] `change-explainer` used (if significant changes)
- [ ] `docs-sync-editor` used (if docs affected)

### 5. Resilience & Adaptability (10 points)

**Measures:** Recovery from errors, workarounds, adaptive problem-solving

**UPDATED for Get Unstuck Protocol** (multi-channel research when stuck)

| Score | Criteria |
|-------|----------|
| 10 | Used Get Unstuck protocol when stuck (2-3 attempts), comprehensive multi-channel research, documented findings, found solution or made significant progress |
| 8-9 | Used Get Unstuck protocol, most channels researched, good synthesis, found solution |
| 6-7 | Used some research tools, partial Get Unstuck, incomplete documentation OR didn't use Get Unstuck but recovered through other means |
| 4-5 | Minimal research, repeated same approach 4+ times before trying Get Unstuck |
| 0-3 | Gave up without Get Unstuck protocol, or never tried systematic research |

**Check:**
- [ ] Used Get Unstuck protocol when stuck (after 2-3 failed attempts)
- [ ] Logged acknowledgment of being stuck with attempts documented
- [ ] Used Perplexity MCP for AI-powered research
- [ ] Used Brave MCP or WebSearch for web results
- [ ] Used Context7 MCP for API/library issues (if applicable)
- [ ] Used Gemini skill for alternative perspective (if available and helpful)
- [ ] Synthesized findings from multiple sources (decision + rationale logs)
- [ ] New approach differed meaningfully from previous failed attempts
- [ ] Documented which resource solved the problem (milestone log)
- [ ] Workarounds documented when blocked
- [ ] Deviations from plan justified in logs

**Bonus Points:**
- **+2 points:** Used Get Unstuck proactively (after 2 attempts, not 5+)
- **+1 point:** Documented which specific resource/channel solved the problem
- **+1 point:** Updated CLAUDE.md with learning for future sessions

**Deductions:**
- **-3 points:** Repeated same failed approach 5+ times without using Get Unstuck
- **-5 points:** Asked for help without trying Get Unstuck protocol first
- **-2 points:** Used Get Unstuck but skipped available channels (e.g., had Context7 but didn't use for API issue)
- **-4 points:** Gave up after single search attempt (didn't try multiple channels)
- **-2 points:** No synthesis of research findings (just tried first result)

### 6. Logging & Traceability (10 points)

**Measures:** Log completeness, timestamp accuracy, update frequency

**UPDATED for Hybrid Logging Protocol v2.0** (uses hooks + manual decision logging)

| Score | Criteria |
|-------|----------|
| 10 | Perfect logging: Hooks capturing all commands, manual logging covers all decisions/rationale/investigations |
| 8 | Good logging: Hooks working, most key decisions logged manually |
| 6 | Adequate logging: Some manual decision entries missing, but hooks operational |
| 4 | Poor logging: Hooks not configured OR many decisions not logged |
| 0-3 | No logging or batch logs at end (v1.0 style manual logging only) |

**Check:**
- [ ] Log filename matches instruction description
- [ ] Hooks configured in `.claude/hooks.json` and capturing tool calls automatically
- [ ] Timestamps in [HH:MM:SS] format (automatic via hooks)
- [ ] Manual decision logging used for: decisions, rationale, investigations, verifications, deviations, milestones
- [ ] `log-decision.sh` script present and executable in `debugging/scripts/`
- [ ] Permissions configured for `log-decision.sh` (no approval prompts)
- [ ] Final summary with outcomes via `/log-complete`

**Deductions:**
- Hooks not configured or not working: -3 points
- Missing manual logging for key decisions: -1 point per occurrence (max -5)
- Still using old v1.0 manual `echo` + `tee` instead of hooks: -2 points (wasteful token usage)

## Grade Calculation

**Total Score = Sum of all categories (max 100)**

### Grade Scale

| Score | Grade | Meaning |
|-------|-------|---------|
| 95-100 | A+ | Outstanding, exemplary work |
| 90-94 | A | Excellent work, minor improvements possible |
| 85-89 | A- | Very good work, few issues |
| 80-84 | B+ | Good work, some issues |
| 75-79 | B | Satisfactory work, notable issues |
| 70-74 | B- | Acceptable work, significant issues |
| 65-69 | C+ | Marginal work, major issues |
| 60-64 | C | Poor work, extensive issues |
| 55-59 | C- | Very poor work, critical issues |
| 50-54 | D | Unacceptable work, mostly failed |
| 0-49 | F | Failed, unusable work |

## Automatic Grade Caps

Certain failures result in automatic maximum grades regardless of other work quality:

### Testing Failures (Non-Negotiable)

| Failure | Maximum Grade | Deduction |
|---------|---------------|-----------|
| Unit tests not run during phase | D (65%) | -35 points |
| Unit tests fail | F (50%) | UNACCEPTABLE |
| Coverage below 60% | C- (70%) | -30 points |
| Integration tests not run before commit | C+ (78%) | -22 points |
| Final phase without full test suite | C+ (78%) | -22 points |

### CI/CD Failures (Critical)

| Failure | Maximum Grade | Deduction |
|---------|---------------|-----------|
| CI/CD configured but not tested | C+ (78%) | -22 points |
| Workflows not triggered | C (65%) | -35 points |
| No successful run IDs documented | C (65%) | -35 points |

### Agent Usage Failures

| Failure | Deduction |
|---------|-----------|
| Using qa-enforcer for documentation creation | -2 points |
| Not using change-explainer after creating docs | -1 point |
| Not using docs-sync-editor when README/CLAUDE.md need updates | -1 point |
| Making mandatory agents "optional" | -2 points |
| Not using qa-enforcer before completion | -5 points |

## Grading Process

### 1. Review Logs

Check the code agent's `${CODE_AGENT_DIR}/debugging/logs/` directory:
- Verify EVERY action has corresponding verification
- Check that failed attempts were retried
- Confirm workarounds are documented

### 2. Verify Deliverables

- Check files that were supposed to be created exist
- Review files mentioned in logs were actually modified
- Validate code correctness and structure
- Confirm test coverage is adequate

### 3. Verify Actions

For every action in the logs, check:
- **Commands**: Return codes were checked
- **Resources**: Created resources were verified to exist
- **Configurations**: Changes were tested to work
- **Tests**: Tests were written AND results logged

### 4. Calculate Score

Use the rubric above to score each category, then sum for total.

### 5. Write Grade Document

Create: `grades/grade-${DATE}-${TIME}-<description>.md`

**Grade Document Structure:**

```markdown
# Grade: [Task Description]

**Date**: YYYY-MM-DD HH:MM
**Overall Grade**: [Letter] ([Percentage])

## Executive Summary

[2-3 sentences summarizing overall performance]

## Category Scores

| Category | Score | Max | Percentage | Notes |
|----------|-------|-----|-----------|-------|
| Completeness | XX | 25 | XX% | [brief note] |
| Code Quality | XX | 20 | XX% | [brief note] |
| Testing & Verification | XX | 20 | XX% | [brief note] |
| Documentation | XX | 15 | XX% | [brief note] |
| Resilience & Adaptability | XX | 10 | XX% | [brief note] |
| Logging & Traceability | XX | 10 | XX% | [brief note] |
| **TOTAL** | **XX** | **100** | **XX%** | |

## Detailed Evaluation

### ✅ Strengths

[List what was done well]

### ⚠️ Issues Found

[List problems, gaps, or concerns]

### 🔍 Evidence

[Specific examples from logs and code]

## Recommendations

[Suggestions for improvement]

## Final Verdict

Grade: [Letter] ([Score]/100)

[Concluding statement]
```

## Key Principles

### Be Thorough and Critical

- Don't accept claims without evidence
- Check that resources actually exist
- Verify tests were run, not just written
- Confirm CI/CD workflows actually passed

### Fair but Strict

- Reward excellent work generously
- Apply automatic caps strictly
- Deduct points proportionally to impact
- Document reasoning clearly

### Evidence-Based

- Reference specific log entries
- Quote from code when applicable
- Link to successful run IDs
- Show proof of verification

## Common Pitfalls to Watch For

1. **"Tests should pass"** vs **"Tests verified passing"**
   - First is a claim, second is evidence

2. **"Configured CI/CD"** vs **"Tested CI/CD (run #12345 SUCCESS)"**
   - Configuration without testing = incomplete

3. **"Fixed the bug"** vs **"Fixed bug, added test, verified no regression"**
   - Fix without test = incomplete

4. **"Used qa-enforcer"** vs **Proper agent sequence**
   - Check agents were used for their actual purpose

## Grade Report Storage

Save grade documents at:
```
grades/grade-YYYY_MM_DD-HH_MM-<description>.md
```

The description MUST match the instruction file's description:
- Instruction: `instruct-2025_10_26-22_00-tkt123_phase6_cicd_integration.md`
- Grade: `grade-2025_10_27-02_00-tkt123_phase6_cicd_integration.md`

Timestamps will differ (instruction created before work, grade after), but description must be identical.
