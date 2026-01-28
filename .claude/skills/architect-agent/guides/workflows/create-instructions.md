# Workflow: Create Instructions

**Trigger:** User says "write instructions for code agent" or "create instructions"

## Prerequisites

Before creating instructions, verify:

```bash
# Check architect workspace structure exists
ls -la instructions/ human/ grades/ ticket/
```

If missing, inform user to initialize workspace first.

## Workflow Steps

### 1. Gather Context

- Review `ticket/current_ticket.md` for ticket details
- Check `analysis/` for any prior research
- Ask clarifying questions if requirements unclear

### 2. Generate Timestamp

```bash
TIMESTAMP=$(date +%Y%m%d_%H%M%S)
# Example: 20251127_143045
```

### 3. Create Technical Instructions

**File:** `instructions/instruct-${TIMESTAMP}-description.md`

**Structure:**
```markdown
# INSTRUCT: [Clear Title]

**Date:** YYYY-MM-DD
**Ticket:** [TICKET-ID] (if applicable)
**Phase:** [Phase number if multi-phase]

## Context
[Background information code agent needs]

## Objectives
- Primary objective
- Secondary objectives

## Requirements
- Specific technical requirements
- Quality standards to meet

## Constraints
- What NOT to do
- Known limitations
- Gotchas to avoid

## Implementation Steps
1. First step with details
2. Second step with details
3. Continue as needed

## Success Criteria
- [ ] Criterion 1
- [ ] Criterion 2
- [ ] All tests pass

## Testing Requirements
- Required test types
- Minimum coverage expectations
- Specific test scenarios

## References
- Links to relevant documentation
- Related analysis files
```

### 4. Create Human Instructions (Executable Documentation)

**File:** `human/human-${TIMESTAMP}-description.md`

**CRITICAL:** Human instructions are NOT summaries. They must be **executable documentation** that a human can follow manually when code agents are unavailable.

**Load:** `references/human_instruction_structure.md` for the complete template.

**Required Elements:**

```markdown
# [Task Name] - Manual Execution Guide

**Date:** YYYY-MM-DD
**Estimated Time:** X minutes
**Difficulty:** Easy | Medium | Advanced
**Corresponding Code Agent Instructions:** instructions/instruct-${TIMESTAMP}-description.md

---

## Prerequisites

Before starting, ensure:
- [ ] [Specific requirement with verification command]

## Overview

### What This Accomplishes
[2-3 sentences explaining the goal and why it matters]

---

## Step 1: [Descriptive Step Name]

### Why This Step
[1-2 sentences explaining purpose - humans benefit from understanding why]

### Commands
```bash
# Copy-pasteable commands
exact-command --with --flags
```

### Expected Output
```
[What success looks like]
```

### Verification
```bash
# How to confirm it worked
```

### If This Fails

| Error | Cause | Solution |
|-------|-------|----------|
| error-message | root-cause | fix-command |

---

[Repeat for each step...]

---

## Final Verification
[How to confirm entire task succeeded]

## Rollback Procedure
[How to undo if something goes wrong]

## Summary
| Step | What You Did | Time |
|------|--------------|------|
```

**Examples:** See `examples/human-instructions/` for complete examples.

### 5. Verify Files Match

```bash
# Both files should have same timestamp
ls -la instructions/instruct-${TIMESTAMP}-*.md
ls -la human/human-${TIMESTAMP}-*.md
```

### 6. Display Human Summary

After creating both files, display the human summary to user for review before sending.

## Quick Checklist

- [ ] Ticket context reviewed
- [ ] Technical instructions created in `instructions/`
- [ ] Human instructions created in `human/` (executable, not summary!)
- [ ] Timestamps match between files
- [ ] Success criteria are measurable
- [ ] Testing requirements specified

### Human Instruction Checklist

- [ ] Every step has copy-pasteable commands
- [ ] Expected output shown for each command
- [ ] Verification steps provided
- [ ] Troubleshooting table with common errors
- [ ] "Why This Step" explanations included
- [ ] Prerequisites clearly listed
- [ ] Rollback procedure documented

## Next Action

After user approves:
- Run "send instructions to code agent"
- Or use `/project.send` command