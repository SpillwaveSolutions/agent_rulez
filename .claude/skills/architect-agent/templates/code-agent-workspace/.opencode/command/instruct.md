---
description: Review and execute current instructions from architect agent
---
# /project.instruct - Review Current Instructions

Read and summarize the current instruction file from the architect agent:

```bash
cat debugging/instructions/current_instructions.md
```

Then create a concise 10-25 bullet point summary showing:

## Summary Format:

**Main Objectives:**
- [Key goal 1]
- [Key goal 2]

**Key Requirements:**
- [Critical requirement 1]
- [Critical requirement 2]

**Critical Constraints:**
- [Constraint 1]
- [Constraint 2]

**Success Criteria:**
- [How to verify success 1]
- [How to verify success 2]

**Testing Requirements:**
- [Test 1]
- [Test 2]

Display this summary to help understand what needs to be accomplished.

**If no instruction file exists:**
```
No current instructions found at debugging/instructions/current_instructions.md
```

# Start executing the instructions
Now run the instructions
