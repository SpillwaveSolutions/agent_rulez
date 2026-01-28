# Workflow: Send Instructions to Code Agent

**Trigger:** User says "send instructions to code agent" or "send them"

## Prerequisites

Before sending, verify:

1. Instruction file exists in `instructions/`
2. Human summary exists in `human/` with matching timestamp
3. Code agent workspace path is known

```bash
# Find latest instruction
LATEST=$(ls -t instructions/instruct-*.md | head -1)
echo "Latest instruction: $LATEST"

# Extract timestamp
TIMESTAMP=$(echo "$LATEST" | grep -oP '\d{8}_\d{6}')
echo "Timestamp: $TIMESTAMP"

# Verify matching human summary exists
ls human/human-${TIMESTAMP}-*.md
```

## Workflow Steps

### 1. Display Human Summary

Before copying, show the user what will be sent:

```bash
cat human/human-${TIMESTAMP}-*.md
```

### 2. Copy Instructions (SIMPLE BASH - No Agent!)

**CRITICAL:** Use simple bash copy. Do NOT spawn agents or use Task tool for this.

```bash
# Copy instruction to code agent
cp "$LATEST" [CODE_AGENT_WORKSPACE]/debugging/instructions/current_instructions.md

# Verify copy succeeded
ls -la [CODE_AGENT_WORKSPACE]/debugging/instructions/current_instructions.md
```

### 3. Confirm Success

```bash
echo "✅ Instructions sent to code agent"
echo "📍 Location: [CODE_AGENT_WORKSPACE]/debugging/instructions/current_instructions.md"
echo ""
echo "Code agent should now:"
echo "1. Run /log-start to initialize logging"
echo "2. Read current_instructions.md"
echo "3. Execute the instructions"
echo "4. Run /log-complete when done"
```

## Why Keep It Simple

- File copy is trivial - no agent processing needed
- Spawning agents wastes tokens
- Direct bash is faster and more reliable
- Architect stays in control

## Common Issues

### Issue: Permission Denied

```bash
# Check if path exists
ls -la [CODE_AGENT_WORKSPACE]/debugging/instructions/

# If directory missing, create it
mkdir -p [CODE_AGENT_WORKSPACE]/debugging/instructions/
```

### Issue: Can't Find Latest Instruction

```bash
# List all instructions by date
ls -lt instructions/instruct-*.md

# Use specific file if needed
cp instructions/instruct-[SPECIFIC].md [CODE_AGENT]/debugging/instructions/current_instructions.md
```

### Issue: Code Agent Workspace Path Unknown

Check your `CLAUDE.md` for the code agent workspace path. If not configured:

```bash
# Add to CLAUDE.md
echo "**Code Agent Workspace:** /path/to/code-agent" >> CLAUDE.md
```

## Quick Checklist

- [ ] Latest instruction file identified
- [ ] Matching human summary verified
- [ ] Human summary displayed to user
- [ ] Instructions copied via bash
- [ ] Copy verified successful
- [ ] User informed of next steps

## After Sending

The architect should:
1. Monitor code agent progress (check logs periodically)
2. Be available to answer questions if code agent is blocked
3. Prepare to grade work when code agent completes