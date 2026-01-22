# Workflow: Initialize Architect Workspace

**Trigger:** User says "set up architect agent" or "initialize workspace"

## Prerequisites

Before initializing, verify directories do NOT already exist:

```bash
# Check for existing workspace
if [ -d "instructions" ] || [ -d "grades" ] || [ -d "human" ]; then
    echo "❌ Workspace already exists. Aborting to prevent overwrite."
    exit 1
fi
```

If directories exist, inform user this appears to be an existing workspace.

## Workflow Steps

### 1. Confirm Code Agent Location

Ask user:
```
Where is the code agent workspace located?
Example: /Users/user/projects/my-project
```

Store this path for CLAUDE.md configuration.

### 2. Create Directory Structure

```bash
# Core directories with archives
mkdir -p instructions/archive
mkdir -p human/archive
mkdir -p grades/archive
mkdir -p analysis/archive

# Ticket tracking
mkdir -p ticket/{feature,bug,archive}
touch ticket/current_ticket.md

# Documentation
mkdir -p docs
```

### 3. Create CLAUDE.md

Create `CLAUDE.md` with workspace configuration including:
- Code agent workspace path
- Directory structure reference
- Skill trigger documentation
- File naming conventions

### 4. Create AGENTS.md (Mirror)

```bash
cp CLAUDE.md AGENTS.md
```

### 5. Initialize Current Ticket

Create `ticket/current_ticket.md` with template for tracking work.

### 6. Display Confirmation

Show user what was created and next steps.

## Quick Checklist

- [ ] Code agent workspace path obtained
- [ ] Verified no existing workspace
- [ ] Directory structure created
- [ ] CLAUDE.md created with code agent path
- [ ] AGENTS.md mirrors CLAUDE.md
- [ ] current_ticket.md initialized
- [ ] User informed of next steps

## Template Alternative

For faster setup, use the template installer:

```bash
cd ~/.claude/skills/architect-agent/templates/
./setup-workspace.sh architect [WORKSPACE_PATH] --code-agent-path [CODE_AGENT_PATH]
```