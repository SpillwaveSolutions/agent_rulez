# Permissions Setup Protocol

**Version:** 1.0
**Last Updated:** 2025-11-14
**Purpose:** Configure `.claude/settings.local.json` permissions for architect-code agent collaboration

---

## Overview

Architect agents and code agents work in **separate workspaces** and need explicit permissions to access each other's files and execute collaborative workflows. Without proper permissions, agents face constant approval prompts that slow execution dramatically.

### The Problem

**Without Permissions:**
- Every file read/write requires user approval
- Every bash command requires confirmation
- Logging, grading, instruction delivery become painfully slow
- Example: 50+ prompts for a single logging session

**With Permissions:**
- Silent, fast execution
- No interruptions for approved operations
- Agents work autonomously within defined boundaries

---

## Workspace Architecture

### Typical Setup

```
~/.claude/skills/architect-agent/          # Architect workspace
├── .claude/
│   └── settings.local.json                 # Architect permissions
├── references/                             # Protocol documents
├── templates/                              # Instruction templates
└── examples/                               # Example instructions

~/clients/project/src/project-name/         # Code agent workspace
├── .claude/
│   ├── settings.local.json                 # Code agent permissions
│   ├── CLAUDE.md                           # Code agent instructions
│   └── AGENTS.md                           # Agent-specific protocols
├── instructions/                           # Architect writes here
├── human/                                  # Human-readable summaries
└── debugging/                              # Logs and debugging info
```

### Cross-Workspace Access Patterns

1. **Architect → Code Workspace**
   - Write instruction files to `instructions/`
   - Write human summaries to `human/`
   - Read logs from `debugging/logs/`
   - Read code to understand context

2. **Code Agent → Architect Workspace**
   - Read protocol documents from `references/`
   - Read instruction templates from `templates/`
   - Read examples from `examples/`

---

## Permission Patterns

### 1. Architect Agent Permissions

**File:** `~/.claude/skills/architect-agent/.claude/settings.local.json`

```json
{
  "permissions": {
    "allow": [
      "mcp__github__create_issue",
      "Bash(gh repo view:*)",
      "Bash(gh auth:*)",
      "Bash(gh issue create:*)",
      "Bash(git add:*)",
      "Bash(git commit:*)",
      "Bash(git push:*)",
      "Bash(gh pr create:*)",
      "Bash(cat:*)",
      "Bash(git check-ignore:*)",
      "Bash(gh pr view:*)",
      "Bash(test:*)",

      "Write(//Users/<username>/clients/*/src/**/instructions/**)",
      "Write(//Users/<username>/clients/*/src/**/human/**)",
      "Read(//Users/<username>/clients/*/src/**/debugging/**)",
      "Read(//Users/<username>/clients/*/src/**/*.md)",
      "Read(//Users/<username>/clients/*/src/**/*.py)",
      "Read(//Users/<username>/clients/*/src/**/*.ts)",
      "Read(//Users/<username>/clients/*/src/**/*.js)",
      "Read(//Users/<username>/clients/*/src/**/*.java)",
      "Read(//Users/<username>/clients/*/src/.claude/**)"
    ],
    "deny": [],
    "ask": []
  }
}
```

**Key Patterns:**
- `Write(//absolute/path/**/directory/**)` - Write to specific directories
- `Read(//absolute/path/**/*.ext)` - Read files by extension
- `Bash(command:*)` - Execute specific bash commands
- Use `**` for recursive matching
- Use `*` for single-level wildcards

### 2. Code Agent Permissions

**File:** `~/clients/project/src/project-name/.claude/settings.local.json`

```json
{
  "permissions": {
    "allow": [
      "mcp__github__create_issue",
      "Bash(gh repo view:*)",
      "Bash(gh auth:*)",
      "Bash(gh issue create:*)",
      "Bash(git add:*)",
      "Bash(git commit:*)",
      "Bash(git push:*)",
      "Bash(gh pr create:*)",
      "Bash(cat:*)",
      "Bash(git check-ignore:*)",
      "Bash(gh pr view:*)",
      "Bash(test:*)",

      "Read(//Users/<username>/.claude/skills/architect-agent/references/**)",
      "Read(//Users/<username>/.claude/skills/architect-agent/templates/**)",
      "Read(//Users/<username>/.claude/skills/architect-agent/examples/**)",

      "Bash(./debugging/scripts/log.sh:*)",
      "Bash(./debugging/scripts/start-log.sh:*)",
      "Bash(./debugging/scripts/log-decision.sh:*)",
      "Bash(./debugging/scripts/get-unstuck.sh:*)",
      "Bash(debugging/scripts/log.sh:*)",
      "Bash(debugging/scripts/start-log.sh:*)",
      "Bash(debugging/scripts/log-decision.sh:*)",
      "Bash(debugging/scripts/get-unstuck.sh:*)",
      "Bash(task test:*)",
      "Bash(.specify/scripts/**)",

      "Write(//Users/<username>/clients/*/src/**/debugging/**)",
      "Write(//Users/<username>/clients/*/src/**/*.py)",
      "Write(//Users/<username>/clients/*/src/**/*.ts)",
      "Write(//Users/<username>/clients/*/src/**/*.md)",
      "Edit(//Users/<username>/clients/*/src/**/*.py)",
      "Edit(//Users/<username>/clients/*/src/**/*.ts)",
      "Read(//Users/<username>/clients/*/src/**)"
    ],
    "deny": [],
    "ask": []
  }
}
```

**Key Patterns:**
- Read access to architect workspace references
- Write access to own workspace
- Execute project-specific scripts without prompts
- Logging scripts with both `./` and without prefix

---

## Script-Based Protocols

### Problem: Permission Prompts for Repetitive Operations

Protocols that require frequent file operations (logging, checkpoints, status updates) cause permission prompt spam:

```bash
# ❌ BAD: Every echo/cat/heredoc requires approval
echo "[$(date +%H:%M:%S)] Message" >> debugging/logs/log.md
cat >> debugging/logs/log.md <<EOF
Content here
EOF
```

### Solution: Pre-Approved Scripts

Create bash scripts for repetitive operations and grant blanket permission:

```bash
# ✅ GOOD: Script approved once, used many times
./debugging/scripts/log.sh "Message"
./debugging/scripts/log.sh --success "Task complete"
```

**Benefits:**
- One-time permission grant
- Consistent formatting
- Automatic timestamps
- No interruptions

### Example: Logging Scripts

**1. Create Scripts**

`debugging/scripts/start-log.sh`:
```bash
#!/bin/bash
# Start new logging session
set -e

LOG_FILE="debugging/logs/$1"
mkdir -p "$(dirname "$LOG_FILE")"
touch "$LOG_FILE"
echo "$LOG_FILE" > debugging/current_log_file.txt
echo "Started logging session: $LOG_FILE"
```

`debugging/scripts/log.sh`:
```bash
#!/bin/bash
# Append to active log with timestamp
set -e

CURRENT_LOG=$(cat debugging/current_log_file.txt 2>/dev/null)
if [ -z "$CURRENT_LOG" ]; then
    echo "Error: No active log session"
    exit 1
fi

TIMESTAMP="[$(date +%H:%M:%S)]"
echo "$TIMESTAMP $1" >> "$CURRENT_LOG"
```

**2. Grant Permissions**

Add to `.claude/settings.local.json`:
```json
{
  "permissions": {
    "allow": [
      "Bash(./debugging/scripts/log.sh:*)",
      "Bash(./debugging/scripts/start-log.sh:*)",
      "Bash(debugging/scripts/log.sh:*)",
      "Bash(debugging/scripts/start-log.sh:*)"
    ]
  }
}
```

**3. Document in Protocols**

Update `AGENTS.md` or `CLAUDE.md`:
```markdown
## Logging Protocol

**Use logging scripts (NO permission prompts!):**

```bash
# Start session
./debugging/scripts/start-log.sh log_2025_11_14-14_56-description.md

# Log messages
./debugging/scripts/log.sh "Regular message"
./debugging/scripts/log.sh --success "Task completed"
./debugging/scripts/log.sh --error "Problem detected"
```
```

### Additional Protocol Scripts

**Key Principle:** Code agent works primarily in `debugging/` (gitignored). All cross-workspace operations should use scripts in `debugging/scripts/` to minimize permission prompts.

#### 1. Instruction Management

**`debugging/scripts/read-instruction.sh`** - Code agent reads instruction from architect or local debugging/instructions/

```bash
#!/bin/bash
# Read instruction file and display with formatting
set -e

INSTRUCTION_FILE="$1"

if [ -z "$INSTRUCTION_FILE" ]; then
    echo "Usage: ./debugging/scripts/read-instruction.sh <uuid-or-filename>"
    exit 1
fi

# Check debugging/instructions first (iterative workflow)
if [ -f "debugging/instructions/$INSTRUCTION_FILE"*.md ]; then
    cat debugging/instructions/"$INSTRUCTION_FILE"*.md
    exit 0
fi

# Check if full path provided
if [ -f "$INSTRUCTION_FILE" ]; then
    cat "$INSTRUCTION_FILE"
    exit 0
fi

echo "Error: Instruction file not found"
exit 1
```

**Permissions:**
```json
{
  "allow": [
    "Bash(./debugging/scripts/read-instruction.sh:*)",
    "Bash(debugging/scripts/read-instruction.sh:*)"
  ]
}
```

#### 2. Checkpointing

**`debugging/scripts/checkpoint.sh`** - Log formatted checkpoint with progress

```bash
#!/bin/bash
# Log checkpoint with progress tracking
set -e

MESSAGE="$1"
PROGRESS="${2:-0}"

CURRENT_LOG=$(cat debugging/current_log_file.txt 2>/dev/null)
if [ -z "$CURRENT_LOG" ]; then
    echo "Error: No active log session"
    exit 1
fi

cat >> "$CURRENT_LOG" <<EOF

## [$(date +%H:%M:%S)] CHECKPOINT: $MESSAGE
**Progress:** ${PROGRESS}% complete
**Timestamp:** $(date +%Y-%m-%d\ %H:%M:%S)
---

EOF

echo "Checkpoint logged: $MESSAGE ($PROGRESS%)"
```

**Usage:**
```bash
./debugging/scripts/checkpoint.sh "Database schema complete" 33
./debugging/scripts/checkpoint.sh "API endpoints implemented" 66
./debugging/scripts/checkpoint.sh "All tests passing" 100
```

**Permissions:**
```json
{
  "allow": [
    "Bash(./debugging/scripts/checkpoint.sh:*)",
    "Bash(debugging/scripts/checkpoint.sh:*)"
  ]
}
```

#### 3. Status Signaling

**`debugging/scripts/signal-complete.sh`** - Signal completion to architect

```bash
#!/bin/bash
# Signal work completion by creating status file
set -e

STATUS="${1:-complete}"  # complete, ready-for-grading, improvements-done

cat > debugging/status.txt <<EOF
STATUS: $STATUS
TIMESTAMP: $(date +%Y-%m-%d\ %H:%M:%S)
LOG_FILE: $(cat debugging/current_log_file.txt 2>/dev/null || echo "N/A")
EOF

echo "Status signaled: $STATUS"
echo "Architect can check debugging/status.txt"
```

**Usage:**
```bash
# After completing instructions
./debugging/scripts/signal-complete.sh complete

# After initial implementation (ready for grading)
./debugging/scripts/signal-complete.sh ready-for-grading

# After improvements based on grade
./debugging/scripts/signal-complete.sh improvements-done
```

**Permissions:**
```json
{
  "allow": [
    "Bash(./debugging/scripts/signal-complete.sh:*)",
    "Bash(debugging/scripts/signal-complete.sh:*)"
  ]
}
```

#### 4. Grade Display

**`debugging/scripts/show-grade.sh`** - Fetch and display grade from architect

```bash
#!/bin/bash
# Show grade for current work
set -e

UUID="$1"

if [ -z "$UUID" ]; then
    echo "Usage: ./debugging/scripts/show-grade.sh <uuid>"
    exit 1
fi

GRADE_FILE="debugging/instructions/grade-${UUID}*.md"

if ls $GRADE_FILE 1> /dev/null 2>&1; then
    cat $GRADE_FILE
else
    echo "No grade found for UUID: $UUID"
    echo "Grade may not be created yet or UUID is incorrect"
    exit 1
fi
```

**Usage:**
```bash
./debugging/scripts/show-grade.sh a1b2c3d4
```

**Permissions:**
```json
{
  "allow": [
    "Bash(./debugging/scripts/show-grade.sh:*)",
    "Bash(debugging/scripts/show-grade.sh:*)"
  ]
}
```

#### 5. Test Execution Wrapper

**`debugging/scripts/run-tests.sh`** - Run tests with logging

```bash
#!/bin/bash
# Run tests and log results
set -e

TEST_TYPE="${1:-all}"  # all, unit, integration, coverage

CURRENT_LOG=$(cat debugging/current_log_file.txt 2>/dev/null)

case "$TEST_TYPE" in
    unit)
        echo "[$(date +%H:%M:%S)] Running unit tests..." | tee -a "$CURRENT_LOG"
        task test 2>&1 | tee -a "$CURRENT_LOG"
        ;;
    integration)
        echo "[$(date +%H:%M:%S)] Running integration tests..." | tee -a "$CURRENT_LOG"
        task test-int 2>&1 | tee -a "$CURRENT_LOG"
        ;;
    coverage)
        echo "[$(date +%H:%M:%S)] Running coverage tests..." | tee -a "$CURRENT_LOG"
        task cov 2>&1 | tee -a "$CURRENT_LOG"
        ;;
    all)
        echo "[$(date +%H:%M:%S)] Running all tests..." | tee -a "$CURRENT_LOG"
        task test 2>&1 | tee -a "$CURRENT_LOG"
        task test-int 2>&1 | tee -a "$CURRENT_LOG"
        task cov 2>&1 | tee -a "$CURRENT_LOG"
        ;;
    *)
        echo "Usage: ./debugging/scripts/run-tests.sh [unit|integration|coverage|all]"
        exit 1
        ;;
esac

echo "[$(date +%H:%M:%S)] Test execution complete" | tee -a "$CURRENT_LOG"
```

**Usage:**
```bash
./debugging/scripts/run-tests.sh unit
./debugging/scripts/run-tests.sh integration
./debugging/scripts/run-tests.sh coverage
./debugging/scripts/run-tests.sh all
```

**Permissions:**
```json
{
  "allow": [
    "Bash(./debugging/scripts/run-tests.sh:*)",
    "Bash(debugging/scripts/run-tests.sh:*)",
    "Bash(task test:*)",
    "Bash(task test-int:*)",
    "Bash(task cov:*)"
  ]
}
```

#### 6. Architect: Send Instructions

**`scripts/send-instruction.sh`** - Architect sends instruction to code agent (in architect workspace)

```bash
#!/bin/bash
# Send instruction from architect to code agent workspace
set -e

INSTRUCTION_FILE="$1"
CODE_WORKSPACE="$2"

if [ -z "$INSTRUCTION_FILE" ] || [ -z "$CODE_WORKSPACE" ]; then
    echo "Usage: ./scripts/send-instruction.sh <instruction-file> <code-workspace-path>"
    exit 1
fi

if [ ! -f "$INSTRUCTION_FILE" ]; then
    echo "Error: Instruction file not found: $INSTRUCTION_FILE"
    exit 1
fi

mkdir -p "$CODE_WORKSPACE/debugging/instructions"

# Copy instruction to code agent's debugging/instructions
cp "$INSTRUCTION_FILE" "$CODE_WORKSPACE/debugging/instructions/"

FILENAME=$(basename "$INSTRUCTION_FILE")
echo "✅ Sent: $FILENAME"
echo "📍 Location: $CODE_WORKSPACE/debugging/instructions/$FILENAME"
```

**Usage (from architect workspace):**
```bash
./scripts/send-instruction.sh \
    instructions/instruct-2025_11_14-14_30-uuid123.md \
    ~/clients/peak6/src/peak6-contactmanager-2
```

**Permissions (architect workspace):**
```json
{
  "allow": [
    "Bash(./scripts/send-instruction.sh:*)",
    "Bash(scripts/send-instruction.sh:*)",
    "Write(//Users/<username>/clients/*/src/**/debugging/instructions/**)"
  ]
}
```

### Complete Script Permissions Set

**Code Agent Workspace `.claude/settings.local.json`:**

```json
{
  "permissions": {
    "allow": [
      "Bash(git add:*)",
      "Bash(git commit:*)",
      "Bash(git push:*)",
      "Bash(./debugging/scripts/log.sh:*)",
      "Bash(./debugging/scripts/start-log.sh:*)",
      "Bash(./debugging/scripts/checkpoint.sh:*)",
      "Bash(./debugging/scripts/signal-complete.sh:*)",
      "Bash(./debugging/scripts/show-grade.sh:*)",
      "Bash(./debugging/scripts/read-instruction.sh:*)",
      "Bash(./debugging/scripts/run-tests.sh:*)",
      "Bash(debugging/scripts/log.sh:*)",
      "Bash(debugging/scripts/start-log.sh:*)",
      "Bash(debugging/scripts/checkpoint.sh:*)",
      "Bash(debugging/scripts/signal-complete.sh:*)",
      "Bash(debugging/scripts/show-grade.sh:*)",
      "Bash(debugging/scripts/read-instruction.sh:*)",
      "Bash(debugging/scripts/run-tests.sh:*)",
      "Bash(task test:*)",
      "Bash(task test-int:*)",
      "Bash(task cov:*)",
      "Read(//Users/<username>/.claude/skills/architect-agent/references/**)",
      "Write(//Users/<username>/clients/*/src/**/debugging/**)",
      "Read(//Users/<username>/clients/*/src/**/*.py)",
      "Read(//Users/<username>/clients/*/src/**/*.md)",
      "Edit(//Users/<username>/clients/*/src/**/*.py)",
      "Edit(//Users/<username>/clients/*/src/**/*.ts)"
    ],
    "deny": [],
    "ask": []
  }
}
```

**Key Pattern:** All cross-workspace operations go through `debugging/scripts/` to minimize permission surface area and keep code agent work in gitignored space.

---

## Ultrathink Canonical Filename Protocol

**Problem Solved:** Eliminates instruction confusion by using ONE canonical filename for active work, with lifecycle management via QUEUE.md.

### The Protocol

**Core Principle:** Only ONE active instruction exists at any time: `debugging/instructions/current_instructions.md`

**Lifecycle States:**
1. **Active** - `current_instructions.md` exists (code agent is working on it)
2. **Queued** - Listed in `QUEUE.md` but not yet activated
3. **Completed** - Archived as `archive/instruct-{TIMESTAMP}-completed.md`

### Directory Structure

```
debugging/instructions/
├── current_instructions.md       # ← ONLY active instruction (or absent if no work)
├── QUEUE.md                      # ← Lifecycle tracker (Active/Queued/Completed)
└── archive/
    ├── instruct-2025_11_13-15_30-proj123_phase1-completed.md
    └── instruct-2025_11_14-09_00-proj123_phase2-completed.md
```

### QUEUE.md Format

```markdown
# Instruction Queue

**Last Updated:** 2025-11-14 14:30

## Active (Currently Working)
- [ACTIVE] 2025-11-13 15:30 - PROJ-123/124 Phase 1: Dependencies & Type Hints
  - File: `current_instructions.md`
  - Started: 2025-11-13 15:30
  - Status: In progress

## Queued (Not Started)
- [QUEUED] Phase 2: Core Implementation
- [QUEUED] Phase 3: Testing

## Completed (Archived)
- [COMPLETE] 2025-11-13 15:30 - PROJ-123/124 Phase 1
  - Archived: `archive/instruct-2025_11_13-15_30-proj123_phase1-completed.md`
  - Completed: 2025-11-13 18:00
  - Grade: A+ (98/100)
```

### Enhanced Scripts for Ultrathink Protocol

#### 1. Architect: Activate Instruction

**`scripts/activate-instruction.sh`** - Architect activates instruction for code agent

```bash
#!/bin/bash
# Activate instruction for code agent (architect workspace)
set -e

INSTRUCTION_FILE="$1"
CODE_WORKSPACE="$2"

if [ -z "$INSTRUCTION_FILE" ] || [ -z "$CODE_WORKSPACE" ]; then
    echo "Usage: ./scripts/activate-instruction.sh <instruction-file> <code-workspace-path>"
    exit 1
fi

if [ ! -f "$INSTRUCTION_FILE" ]; then
    echo "Error: Instruction file not found: $INSTRUCTION_FILE"
    exit 1
fi

# Check if there's already an active instruction
if [ -f "$CODE_WORKSPACE/debugging/instructions/current_instructions.md" ]; then
    echo "⚠️  WARNING: Active instruction already exists!"
    echo "Current: $CODE_WORKSPACE/debugging/instructions/current_instructions.md"
    echo ""
    read -p "Archive current instruction and activate new one? (y/n) " -n 1 -r
    echo
    if [[ ! $REPLY =~ ^[Yy]$ ]]; then
        echo "Activation cancelled"
        exit 1
    fi
fi

mkdir -p "$CODE_WORKSPACE/debugging/instructions"
mkdir -p "$CODE_WORKSPACE/debugging/instructions/archive"

# Copy instruction as current_instructions.md
cp "$INSTRUCTION_FILE" "$CODE_WORKSPACE/debugging/instructions/current_instructions.md"

# Extract instruction metadata
TITLE=$(grep "^# INSTRUCT:" "$INSTRUCTION_FILE" | sed 's/# INSTRUCT: //')
DATE=$(grep "^\*\*Date:\*\*" "$INSTRUCTION_FILE" | awk '{print $2}' | tr -d '*')

# Update QUEUE.md
QUEUE_FILE="$CODE_WORKSPACE/debugging/instructions/QUEUE.md"
if [ ! -f "$QUEUE_FILE" ]; then
    cat > "$QUEUE_FILE" <<EOF
# Instruction Queue

**Last Updated:** $(date +%Y-%m-%d\ %H:%M)

## Active (Currently Working)
- [ACTIVE] $DATE - $TITLE
  - File: \`current_instructions.md\`
  - Started: $(date +%Y-%m-%d\ %H:%M)
  - Status: Ready for execution

## Queued (Not Started)

## Completed (Archived)

EOF
else
    # Add to Active section (replace existing active if present)
    # This is a simplified version - production should use proper sed/awk
    echo "Manual QUEUE.md update required - add:"
    echo "- [ACTIVE] $DATE - $TITLE"
fi

echo "✅ Instruction activated!"
echo "📍 Location: $CODE_WORKSPACE/debugging/instructions/current_instructions.md"
echo "📋 Title: $TITLE"
echo "📅 Date: $DATE"
echo ""
echo "Code agent will detect this automatically on next session start."
```

**Permissions:**
```json
{
  "allow": [
    "Bash(./scripts/activate-instruction.sh:*)",
    "Bash(scripts/activate-instruction.sh:*)",
    "Write(//Users/<username>/clients/*/src/**/debugging/instructions/**)"
  ]
}
```

#### 2. Code Agent: Check for Instructions

**`debugging/scripts/check-instruction.sh`** - Code agent checks for active instruction

```bash
#!/bin/bash
# Check if there's an active instruction to execute
set -e

INSTRUCTION_FILE="debugging/instructions/current_instructions.md"

if [ -f "$INSTRUCTION_FILE" ]; then
    echo "✅ Active instruction found!"
    echo ""

    # Extract key metadata
    TITLE=$(grep "^# INSTRUCT:" "$INSTRUCTION_FILE" | sed 's/# INSTRUCT: //')
    DATE=$(grep "^\*\*Date:\*\*" "$INSTRUCTION_FILE" | awk '{print $2}' | tr -d '*')
    PRIORITY=$(grep "^\*\*Priority:\*\*" "$INSTRUCTION_FILE" | cut -d: -f2- | xargs)

    echo "📋 Title: $TITLE"
    echo "📅 Date: $DATE"
    echo "⚡ Priority: $PRIORITY"
    echo ""
    echo "To view full instruction:"
    echo "  cat debugging/instructions/current_instructions.md"
    echo ""
    echo "Ready to execute this instruction?"

    exit 0
else
    echo "ℹ️  No active instruction"
    echo ""
    echo "Check QUEUE.md for queued or completed work:"
    if [ -f "debugging/instructions/QUEUE.md" ]; then
        cat debugging/instructions/QUEUE.md
    else
        echo "  (QUEUE.md not found)"
    fi

    exit 1
fi
```

**Permissions:**
```json
{
  "allow": [
    "Bash(./debugging/scripts/check-instruction.sh:*)",
    "Bash(debugging/scripts/check-instruction.sh:*)"
  ]
}
```

#### 3. Code Agent: Complete Instruction

**`debugging/scripts/complete-instruction.sh`** - Archive instruction and update QUEUE.md

```bash
#!/bin/bash
# Complete current instruction and archive it
set -e

INSTRUCTION_FILE="debugging/instructions/current_instructions.md"

if [ ! -f "$INSTRUCTION_FILE" ]; then
    echo "Error: No active instruction found"
    exit 1
fi

# Extract timestamp from instruction header
DATE=$(grep "^\*\*Date:\*\*" "$INSTRUCTION_FILE" | awk '{print $2}' | tr -d '*')
TITLE=$(grep "^# INSTRUCT:" "$INSTRUCTION_FILE" | sed 's/# INSTRUCT: //')

# Convert date format: 2025-11-13 → 2025_11_13
TIMESTAMP=$(echo "$DATE" | tr '-' '_')

# Create archive filename
ARCHIVE_NAME="instruct-${TIMESTAMP}-completed.md"
ARCHIVE_PATH="debugging/instructions/archive/$ARCHIVE_NAME"

# Create archive directory if needed
mkdir -p debugging/instructions/archive

# Move to archive
mv "$INSTRUCTION_FILE" "$ARCHIVE_PATH"

echo "✅ Instruction completed and archived!"
echo "📁 Archived: $ARCHIVE_PATH"
echo ""

# Update QUEUE.md
QUEUE_FILE="debugging/instructions/QUEUE.md"
if [ -f "$QUEUE_FILE" ]; then
    # Update QUEUE.md to move from Active → Completed
    # This is a marker for manual update - production version would use sed/awk
    echo "📋 Update QUEUE.md:"
    echo "   Move [ACTIVE] → [COMPLETE]"
    echo "   Add archived filename: $ARCHIVE_NAME"
    echo "   Add completion timestamp: $(date +%Y-%m-%d\ %H:%M)"
else
    echo "⚠️  QUEUE.md not found - skipping update"
fi

# Create status signal
cat > debugging/status.txt <<EOF
STATUS: instruction-completed
TIMESTAMP: $(date +%Y-%m-%d\ %H:%M:%S)
ARCHIVED: $ARCHIVE_PATH
TITLE: $TITLE
READY_FOR_GRADING: yes
EOF

echo ""
echo "📊 Status signaled: Ready for grading"
echo "🎯 Architect can now grade this work"
```

**Permissions:**
```json
{
  "allow": [
    "Bash(./debugging/scripts/complete-instruction.sh:*)",
    "Bash(debugging/scripts/complete-instruction.sh:*)"
  ]
}
```

#### 4. Enhanced Status Signaling

**`debugging/scripts/signal-status.sh`** - Comprehensive status signaling

```bash
#!/bin/bash
# Signal various statuses to architect
set -e

STATUS="${1:-unknown}"
MESSAGE="${2:-}"

TIMESTAMP=$(date +%Y-%m-%d\ %H:%M:%S)
LOG_FILE=$(cat debugging/current_log_file.txt 2>/dev/null || echo "N/A")

case "$STATUS" in
    ready-for-grading)
        cat > debugging/status.txt <<EOF
STATUS: ready-for-grading
TIMESTAMP: $TIMESTAMP
LOG_FILE: $LOG_FILE
MESSAGE: Work completed, awaiting grade
INSTRUCTION: $([ -f debugging/instructions/current_instructions.md ] && echo "current_instructions.md" || echo "archived")
EOF
        echo "✅ Status: Ready for grading"
        ;;

    in-progress)
        PROGRESS="${2:-0}"
        cat > debugging/status.txt <<EOF
STATUS: in-progress
TIMESTAMP: $TIMESTAMP
LOG_FILE: $LOG_FILE
PROGRESS: ${PROGRESS}%
MESSAGE: ${MESSAGE:-Working on implementation}
INSTRUCTION: current_instructions.md
EOF
        echo "🔄 Status: In progress (${PROGRESS}%)"
        ;;

    blocked)
        cat > debugging/status.txt <<EOF
STATUS: blocked
TIMESTAMP: $TIMESTAMP
LOG_FILE: $LOG_FILE
BLOCKER: ${MESSAGE:-Unknown blocker}
INSTRUCTION: current_instructions.md
NEEDS_HELP: yes
EOF
        echo "🚫 Status: Blocked - ${MESSAGE}"
        echo "📢 Architect attention required"
        ;;

    improvements-done)
        cat > debugging/status.txt <<EOF
STATUS: improvements-done
TIMESTAMP: $TIMESTAMP
LOG_FILE: $LOG_FILE
MESSAGE: Improvements based on grade completed
INSTRUCTION: current_instructions.md
READY_FOR_REGRADING: yes
EOF
        echo "✅ Status: Improvements complete, ready for re-grading"
        ;;

    *)
        cat > debugging/status.txt <<EOF
STATUS: $STATUS
TIMESTAMP: $TIMESTAMP
LOG_FILE: $LOG_FILE
MESSAGE: ${MESSAGE}
EOF
        echo "ℹ️  Status: $STATUS"
        ;;
esac

# Log status change
if [ -f "$LOG_FILE" ] && [ "$LOG_FILE" != "N/A" ]; then
    echo "" >> "$LOG_FILE"
    echo "## [$TIMESTAMP] STATUS CHANGE: $STATUS" >> "$LOG_FILE"
    echo "**Message:** ${MESSAGE}" >> "$LOG_FILE"
    echo "---" >> "$LOG_FILE"
fi

echo "📝 Status written to debugging/status.txt"
```

**Usage:**
```bash
# Signal ready for grading
./debugging/scripts/signal-status.sh ready-for-grading

# Signal progress
./debugging/scripts/signal-status.sh in-progress 45

# Signal blocker
./debugging/scripts/signal-status.sh blocked "Waiting for API credentials"

# Signal improvements done
./debugging/scripts/signal-status.sh improvements-done

# Custom status
./debugging/scripts/signal-status.sh testing "Running integration tests"
```

**Permissions:**
```json
{
  "allow": [
    "Bash(./debugging/scripts/signal-status.sh:*)",
    "Bash(debugging/scripts/signal-status.sh:*)"
  ]
}
```

#### 5. Architect: Check Code Agent Status

**`scripts/check-code-agent-status.sh`** - Architect checks code agent status

```bash
#!/bin/bash
# Check code agent status (architect workspace)
set -e

CODE_WORKSPACE="$1"

if [ -z "$CODE_WORKSPACE" ]; then
    echo "Usage: ./scripts/check-code-agent-status.sh <code-workspace-path>"
    exit 1
fi

STATUS_FILE="$CODE_WORKSPACE/debugging/status.txt"
INSTRUCTION_FILE="$CODE_WORKSPACE/debugging/instructions/current_instructions.md"
QUEUE_FILE="$CODE_WORKSPACE/debugging/instructions/QUEUE.md"

echo "🔍 Code Agent Status Check"
echo "========================================"
echo ""

# Check for active instruction
if [ -f "$INSTRUCTION_FILE" ]; then
    TITLE=$(grep "^# INSTRUCT:" "$INSTRUCTION_FILE" | sed 's/# INSTRUCT: //')
    echo "📋 Active Instruction: YES"
    echo "   Title: $TITLE"
else
    echo "📋 Active Instruction: NO"
fi
echo ""

# Check status file
if [ -f "$STATUS_FILE" ]; then
    echo "📊 Latest Status:"
    cat "$STATUS_FILE"
else
    echo "📊 Latest Status: No status file found"
fi
echo ""

# Check QUEUE.md
if [ -f "$QUEUE_FILE" ]; then
    echo "📋 Queue Summary:"
    grep -E "^\[ACTIVE\]|^\[QUEUED\]|^\[COMPLETE\]" "$QUEUE_FILE" | head -n 10
else
    echo "📋 Queue: QUEUE.md not found"
fi
echo ""
echo "========================================"
```

**Usage (from architect workspace):**
```bash
./scripts/check-code-agent-status.sh ~/clients/peak6/src/peak6-contactmanager-2
```

**Permissions:**
```json
{
  "allow": [
    "Bash(./scripts/check-code-agent-status.sh:*)",
    "Bash(scripts/check-code-agent-status.sh:*)",
    "Read(//Users/<username>/clients/*/src/**/debugging/**)"
  ]
}
```

### Code Agent Session Start Protocol

**CRITICAL:** Code agent's `CLAUDE.md` must have this at the TOP:

```markdown
# Code Agent Workspace

## ⚡ IMMEDIATE ACTION REQUIRED - CHECK FOR ACTIVE INSTRUCTION

**BEFORE doing ANYTHING else in this session, run:**

```bash
./debugging/scripts/check-instruction.sh
```

**If active instruction found:**
1. Read it: `cat debugging/instructions/current_instructions.md`
2. Start logging session: `./debugging/scripts/start-log.sh log_$(date +%Y_%m_%d-%H_%M)-description.md`
3. Execute the instruction following ALL protocols
4. Signal completion: `./debugging/scripts/complete-instruction.sh`

**If no active instruction:**
- Check `debugging/instructions/QUEUE.md` for queued work
- Wait for architect to activate next instruction
```

### Workflow Summary

**Architect → Code Agent (Activate):**
```bash
# 1. Architect activates instruction
./scripts/activate-instruction.sh \
    instructions/instruct-2025_11_14-14_30-phase2.md \
    ~/clients/peak6/src/peak6-contactmanager-2

# Result: Instruction copied as current_instructions.md
```

**Code Agent Session Start:**
```bash
# 1. Code agent checks for work
./debugging/scripts/check-instruction.sh

# 2. If found, read it
cat debugging/instructions/current_instructions.md

# 3. Start work
./debugging/scripts/start-log.sh log_$(date +%Y_%m_%d-%H_%M)-phase2.md

# 4. Execute instruction...
# [... implementation work ...]

# 5. Signal progress
./debugging/scripts/signal-status.sh in-progress 50

# 6. Complete work
./debugging/scripts/complete-instruction.sh
./debugging/scripts/signal-status.sh ready-for-grading
```

**Architect Checks Status:**
```bash
# Architect checks if code agent is done
./scripts/check-code-agent-status.sh ~/clients/peak6/src/peak6-contactmanager-2

# If status shows "ready-for-grading", grade the work
```

### Benefits of Ultrathink Protocol

1. **Zero Ambiguity** - Only ONE active instruction file (`current_instructions.md`)
2. **Clean Workspace** - No accumulation of timestamped files
3. **Clear Lifecycle** - QUEUE.md tracks Active → Queued → Completed
4. **Proactive Detection** - Code agent checks for work on session start
5. **Complete Audit Trail** - Archived instructions preserve history
6. **Status Transparency** - Both agents know exactly what's happening
7. **No Permission Spam** - All operations use pre-approved scripts

### File Permissions for Ultrathink Protocol

**Code Agent Workspace:**
```json
{
  "allow": [
    "Bash(./debugging/scripts/check-instruction.sh:*)",
    "Bash(./debugging/scripts/complete-instruction.sh:*)",
    "Bash(./debugging/scripts/signal-status.sh:*)",
    "Bash(debugging/scripts/check-instruction.sh:*)",
    "Bash(debugging/scripts/complete-instruction.sh:*)",
    "Bash(debugging/scripts/signal-status.sh:*)",
    "Write(//Users/<username>/clients/*/src/**/debugging/**)",
    "Read(//Users/<username>/clients/*/src/**/debugging/**)"
  ]
}
```

**Architect Workspace:**
```json
{
  "allow": [
    "Bash(./scripts/activate-instruction.sh:*)",
    "Bash(./scripts/check-code-agent-status.sh:*)",
    "Bash(scripts/activate-instruction.sh:*)",
    "Bash(scripts/check-code-agent-status.sh:*)",
    "Write(//Users/<username>/clients/*/src/**/debugging/instructions/**)",
    "Read(//Users/<username>/clients/*/src/**/debugging/**)"
  ]
}
```

---

## Setup Checklist

### For New Architect-Code Agent Collaboration

- [ ] **Create code agent workspace directory structure**
  - [ ] `instructions/` - For instruction files
  - [ ] `human/` - For human summaries
  - [ ] `debugging/logs/` - For execution logs
  - [ ] `debugging/scripts/` - For protocol scripts

- [ ] **Configure architect agent permissions**
  - [ ] Create `.claude/settings.local.json` in architect workspace
  - [ ] Add write access to code workspace `instructions/` and `human/`
  - [ ] Add read access to code workspace for context gathering
  - [ ] Add git/GitHub command permissions

- [ ] **Configure code agent permissions**
  - [ ] Create `.claude/settings.local.json` in code workspace
  - [ ] Add read access to architect workspace `references/`
  - [ ] Add protocol script execution permissions
  - [ ] Add write access to own workspace

- [ ] **Create protocol scripts**
  - [ ] `debugging/scripts/log.sh` - Logging utility
  - [ ] `debugging/scripts/start-log.sh` - Session management
  - [ ] Make scripts executable: `chmod +x debugging/scripts/*.sh`
  - [ ] Grant permissions in settings.local.json

- [ ] **Document permissions in project docs**
  - [ ] Update code workspace `CLAUDE.md`
  - [ ] Update code workspace `AGENTS.md`
  - [ ] Document script usage patterns

---

## Common Permission Patterns

### Git Operations

```json
{
  "allow": [
    "Bash(git add:*)",
    "Bash(git commit:*)",
    "Bash(git push:*)",
    "Bash(git status:*)",
    "Bash(git diff:*)",
    "Bash(git log:*)",
    "Bash(git checkout:*)",
    "Bash(git branch:*)"
  ]
}
```

### GitHub CLI Operations

```json
{
  "allow": [
    "Bash(gh issue create:*)",
    "Bash(gh issue list:*)",
    "Bash(gh pr create:*)",
    "Bash(gh pr view:*)",
    "Bash(gh pr list:*)",
    "Bash(gh repo view:*)"
  ]
}
```

### File Operations by Extension

```json
{
  "allow": [
    "Read(//absolute/path/**/*.py)",
    "Read(//absolute/path/**/*.ts)",
    "Read(//absolute/path/**/*.md)",
    "Write(//absolute/path/**/*.py)",
    "Edit(//absolute/path/**/*.ts)"
  ]
}
```

### Directory-Scoped Operations

```json
{
  "allow": [
    "Write(//absolute/path/**/instructions/**)",
    "Write(//absolute/path/**/human/**)",
    "Read(//absolute/path/**/debugging/**)",
    "Edit(//absolute/path/**/src/**)"
  ]
}
```

### Project Build/Test Commands

```json
{
  "allow": [
    "Bash(task build:*)",
    "Bash(task test:*)",
    "Bash(npm test:*)",
    "Bash(pytest:*)",
    "Bash(./gradlew test:*)",
    "Bash(mvn test:*)"
  ]
}
```

---

## Path Resolution

### Absolute vs Relative Paths

**In `.claude/settings.local.json`:**
- Use `//` prefix for absolute paths
- Use `**` for recursive directory matching
- Use `*` for single-level wildcards

**Examples:**
```json
{
  "allow": [
    "Read(//Users/username/project/**/*.py)",           // ✅ Absolute path
    "Write(//Users/username/project/output/**)",        // ✅ Absolute path
    "Bash(./scripts/build.sh:*)",                       // ✅ Relative to workspace
    "Bash(scripts/build.sh:*)"                          // ✅ Both with/without ./
  ]
}
```

### Finding Absolute Paths

```bash
# Get absolute path of current directory
pwd

# Get absolute path of specific file/directory
realpath path/to/file

# Get home directory
echo $HOME
```

---

## Security Considerations

### Principle of Least Privilege

Grant only the permissions needed for the workflow:

**❌ Too Broad:**
```json
{
  "allow": [
    "Bash(*)",           // Allows ANY bash command
    "Write(//**)",       // Allows writing ANYWHERE
    "Read(//**)"         // Allows reading EVERYTHING
  ]
}
```

**✅ Appropriately Scoped:**
```json
{
  "allow": [
    "Bash(git add:*)",                                  // Specific git commands
    "Write(//Users/user/projects/myproject/output/**)", // Specific directories
    "Read(//Users/user/projects/myproject/src/**)"      // Project-scoped reading
  ]
}
```

### Sensitive Operations

**Always require approval for:**
- File deletion: `Bash(rm:*)`, `Bash(rm -rf:*)`
- System changes: `Bash(sudo:*)`
- Package installation: `Bash(npm install:*)`, `Bash(pip install:*)`
- Force operations: `Bash(git push --force:*)`

**Safe to auto-approve:**
- Read operations: `Read(**)`
- Logging scripts: `Bash(./debugging/scripts/log.sh:*)`
- Status commands: `Bash(git status:*)`, `Bash(task --list:*)`
- Test execution: `Bash(task test:*)`

---

## Troubleshooting

### Permission Denied Errors

**Symptom:** Agent gets "permission denied" or asks for approval

**Solutions:**
1. Check if path is absolute: `//Users/...` not `~/...`
2. Check wildcard patterns match the actual path
3. Check both `./script.sh` and `script.sh` variants
4. Reload Claude Code after editing settings.local.json

### Script Not Found

**Symptom:** `./debugging/scripts/log.sh: No such file or directory`

**Solutions:**
1. Verify script exists: `ls -la debugging/scripts/`
2. Check script is executable: `chmod +x debugging/scripts/*.sh`
3. Use absolute path in permission: `Bash(//Users/.../debugging/scripts/log.sh:*)`

### Permission Not Applied

**Symptom:** Permission added but still prompts

**Solutions:**
1. Restart Claude Code session
2. Check JSON syntax is valid
3. Verify no typos in permission patterns
4. Check path separators (Unix: `/`, Windows: `\`)

---

## Examples from Real Workflows

### Logging Scripts (Peak6 Example)

**Problem:** Code agent required 20+ prompts per logging session

**Solution:**
1. Created `debugging/scripts/log.sh` and `start-log.sh`
2. Added permissions:
   ```json
   {
     "allow": [
       "Bash(./debugging/scripts/log.sh:*)",
       "Bash(./debugging/scripts/start-log.sh:*)",
       "Bash(debugging/scripts/log.sh:*)",
       "Bash(debugging/scripts/start-log.sh:*)"
     ]
   }
   ```
3. Updated `AGENTS.md` to instruct using scripts
4. Result: Zero permission prompts for logging

### Instruction Delivery

**Problem:** Architect couldn't write instruction files to code workspace

**Solution:**
1. Architect workspace permissions:
   ```json
   {
     "allow": [
       "Write(//Users/user/clients/*/src/**/instructions/**)",
       "Write(//Users/user/clients/*/src/**/human/**)"
     ]
   }
   ```
2. Created `instructions/` and `human/` directories in code workspace
3. Result: Seamless instruction delivery

### Grading Workflow

**Problem:** Architect couldn't read logs to grade work

**Solution:**
1. Architect workspace permissions:
   ```json
   {
     "allow": [
       "Read(//Users/user/clients/*/src/**/debugging/**)"
     ]
   }
   ```
2. Result: Architect can read logs and grade work

---

## Templates

### Minimal Architect Settings

```json
{
  "permissions": {
    "allow": [
      "Bash(git add:*)",
      "Bash(git commit:*)",
      "Bash(git push:*)",
      "Bash(gh issue create:*)",
      "Bash(gh pr create:*)",
      "Write(//Users/<username>/clients/*/src/**/instructions/**)",
      "Write(//Users/<username>/clients/*/src/**/human/**)",
      "Read(//Users/<username>/clients/*/src/**/debugging/**)"
    ],
    "deny": [],
    "ask": []
  }
}
```

### Minimal Code Agent Settings

```json
{
  "permissions": {
    "allow": [
      "Bash(git add:*)",
      "Bash(git commit:*)",
      "Bash(git push:*)",
      "Read(//Users/<username>/.claude/skills/architect-agent/references/**)",
      "Bash(./debugging/scripts/log.sh:*)",
      "Bash(./debugging/scripts/start-log.sh:*)",
      "Bash(task test:*)",
      "Write(//Users/<username>/clients/*/src/**/debugging/**)"
    ],
    "deny": [],
    "ask": []
  }
}
```

---

## OpenCode-Specific Permissions

**Note:** OpenCode may use a different permission syntax and configuration file structure than Claude Code. Consult OpenCode documentation for the exact format.

### Conceptual Equivalents

The permission concepts remain the same for OpenCode, but the syntax may differ:

**Claude Code Format:**
```json
{
  "permissions": {
    "allow": [
      "Bash(./debugging/scripts/log-decision.sh:*)"
    ]
  }
}
```

**OpenCode Format (Hypothetical - Check OpenCode Docs):**
```json
{
  "permissions": {
    "allow": [
      "debugging/scripts/log-decision.sh",
      "debugging/wrapper-scripts/*.sh"
    ]
  }
}
```

### OpenCode Code Agent Permissions

**Likely Location:** `.opencode/settings.json` or `.opencode/config.json`

**Required Permissions for Code Agent:**

```json
{
  "permissions": {
    "allow": [
      "debugging/scripts/log-decision.sh",
      "debugging/scripts/log-start.sh",
      "debugging/scripts/log-complete.sh",
      "debugging/scripts/get-unstuck.sh",
      "debugging/wrapper-scripts/run-with-logging.sh",
      "debugging/wrapper-scripts/log-tool-call.sh",
      "debugging/wrapper-scripts/log-tool-result.sh"
    ]
  }
}
```

### OpenCode Architect Agent Permissions

**For cross-workspace access:**

```json
{
  "permissions": {
    "allow": [
      "read:<code-agent-workspace>/debugging/**",
      "write:<code-agent-workspace>/instructions/**",
      "write:<code-agent-workspace>/human/**"
    ]
  }
}
```

**Note:** Exact syntax depends on OpenCode version. Key principles:
- Code agent needs to execute logging scripts without prompts
- Architect needs to read code agent logs
- Architect needs to write instructions and summaries
- Both need cross-workspace file access

### Migration Notes

When migrating from Claude Code to OpenCode:
1. Review OpenCode permission documentation
2. Translate Claude Code patterns to OpenCode syntax
3. Test cross-workspace access
4. Verify script execution works without prompts
5. Confirm logging workflow operates silently

**See Also:**
- [OpenCode Setup Guide](./opencode_setup_guide.md) - Detailed OpenCode setup
- [OpenCode Migration Guide](./opencode_migration_guide.md) - Migrating from Claude Code
- [OpenCode Logging Protocol](./opencode_logging_protocol.md) - OpenCode-specific logging

---

## References

- [Claude Code Permissions Documentation](https://code.claude.com/docs/en/configuration/permissions)
- [Logging Protocol](./logging_protocol.md)
- [Delegation Protocol](./instruction_structure.md)
- [Grading Workflow](./instruction_grading_workflow.md)
- [OpenCode Setup Guide](./opencode_setup_guide.md)
- [OpenCode Logging Protocol](./opencode_logging_protocol.md)

---

**Last Updated:** 2025-01-17 (Added OpenCode support)
