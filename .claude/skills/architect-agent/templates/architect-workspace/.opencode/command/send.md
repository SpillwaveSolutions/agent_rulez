---
description: Send instructions to code agent workspace
---
# /project.send - Send Instructions to Code Agent

Perform the following steps to send instructions to the code agent:

## Steps:

### 1. Identify Files
- Find the most recent instruction file in `instructions/instruct-*.md`
- Find the corresponding human summary in `human/human-*.md` (matching timestamp)

### 2. Copy Instruction
Copy the instruction file to code agent's canonical location:
```
[PATH_TO_CODE_AGENT_WORKSPACE]/debugging/instructions/current_instruction.md
```

### 3. Display Human Summary
Read the corresponding human summary and display the 10-25 bullet points to the user showing:
- Main objectives
- Key requirements
- Critical constraints
- Success criteria
- Testing requirements

### 4. Confirm
Report to user:
```
✅ Instructions sent to code agent
📄 File: current_instruction.md
📍 Location: [PATH_TO_CODE_AGENT_WORKSPACE]/debugging/instructions/

Summary shown above. Code agent can now be started to execute these instructions.
```
