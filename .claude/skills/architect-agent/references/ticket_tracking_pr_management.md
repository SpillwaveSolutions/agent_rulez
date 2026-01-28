# Ticket Tracking and PR Management

## Purpose

Provide architect agents with comprehensive protocols for:
1. Tracking tickets throughout their lifecycle
2. Creating well-structured PR descriptions based on tickets
3. Maintaining ticket status and documentation
4. Linking work artifacts to tickets

## Ticket Tracking Structure

### Directory Structure
```
ticket/
├── current_ticket.md          # Active ticket being worked on
├── feature/                    # Feature ticket details
│   └── ticket_id-name.md
├── bug/                        # Bug ticket details
│   └── ticket_id-name.md
└── archive/                    # Completed tickets
    └── ticket_id-name.md
```

### Current Ticket File Template

**File:** `ticket/current_ticket.md`

```markdown
# Current Ticket: [TICKET-ID] [Title]

**URL:** [Link to ticket in system]
**Status:** [In Progress/Blocked/Awaiting Review/etc]
**Priority:** [CRITICAL/HIGH/MEDIUM/LOW]
**Started:** YYYY-MM-DD
**Type:** [Feature/Bug Fix/Refactor/Infrastructure/Documentation]
**Code Agent Branch:** [branch-name]

## Overview

[2-3 sentence description of what this ticket involves]

## Status

### ✅ [Phase Name] Complete
- [x] Objective 1
- [x] Objective 2

### 🔄 [Phase Name] In Progress
- [x] Objective 1
- [ ] Objective 2 - Currently working on this
- [ ] Objective 3

### ⏳ [Phase Name] Pending
- [ ] Objective 1
- [ ] Objective 2

## Task Breakdown

### Task 1: [Task Name] (X minutes)
- [ ] Subtask 1
- [ ] Subtask 2
- [ ] Subtask 3

### Task 2: [Task Name] (Y minutes)
- [ ] Subtask 1
- [ ] Subtask 2

**Total Estimated Time:** XX-YY minutes

## Success Criteria (X required)

- [ ] Criterion 1 - Specific measurable outcome
- [ ] Criterion 2 - Specific measurable outcome
[... 10-15 total]

## Expected Output

**Deliverables:**
- File/component 1
- File/component 2
- Documentation updates

**Verification:**
- How to verify each deliverable
- Commands to run
- Expected results

## Key Facts

**Files to Modify:**
- `path/to/file1.ext`
- `path/to/file2.ext`

**Dependencies:**
- Other ticket IDs
- External services
- Configuration requirements

**Risks:**
- Potential issue 1
- Potential issue 2

## Required Agents

**MANDATORY:**
- `agent-name` - Purpose

**RECOMMENDED:**
- `agent-name` - Purpose

## Related Work

**Previous Tickets:**
- TICKET-XXX: [Description] (Status)

**Related Issues:**
- BUG-XXX: [Description] (Status)
- FEATURE-XXX: [Description] (Status)

## Activity Log

### YYYY-MM-DD HH:MM - [Activity Title]
[Description of what happened]
- Action 1 taken
- Action 2 taken
- Outcome

### YYYY-MM-DD HH:MM - [Activity Title]
[Description of what happened]

## Notes

- Important consideration 1
- Important consideration 2
- Constraints or limitations
```

### Detailed Ticket File Template

**File:** `ticket/feature/ticket_id-name.md` or `ticket/bug/ticket_id-name.md`

```markdown
# [TICKET-ID]: [Full Title]

**URL:** [Link to ticket]
**Type:** [Feature/Bug/Refactor/Infrastructure/Documentation]
**Priority:** [CRITICAL/HIGH/MEDIUM/LOW] - [Why]
**Status:** [In Progress/Complete/Blocked/Archived]
**Created:** YYYY-MM-DD
**Started:** YYYY-MM-DD
**Completed:** YYYY-MM-DD (if done)

## Problem Summary / Feature Request

[Detailed description of the problem or feature]

### [For Bugs] Symptoms
- Symptom 1
- Symptom 2

### [For Bugs] Root Cause
[Technical explanation of why the issue exists]

### [For Features] User Story
As a [user type], I want [goal] so that [benefit].

## Solution / Implementation Approach

[Detailed technical approach]

### Architecture Changes
- Change 1
- Change 2

### Files Affected
| File | Type | Description |
|------|------|-------------|
| file1 | Modified | What changed |
| file2 | Created | What it does |

## Technical Details

### [For Bugs] Error Information
```
Error messages, stack traces, logs
```

### [For Features] Requirements
- Requirement 1
- Requirement 2

### Dependencies
- Dependency 1
- Dependency 2

## Implementation Timeline

- Phase 1: XX minutes - [Description]
- Phase 2: YY minutes - [Description]
- Phase 3: ZZ minutes - [Description]
- **Total:** X hours / Y days

## Testing Strategy

### Unit Tests
- Test 1 description
- Test 2 description

### Integration Tests
- Test 1 description
- Test 2 description

### Manual Verification
- Step 1
- Step 2

## Documentation Requirements

- [ ] Code comments added
- [ ] Change documentation created
- [ ] README updated
- [ ] API documentation updated

## Rollback Procedure

If this needs to be reverted:
1. Step 1
2. Step 2
3. Verification after rollback

## Success Metrics

- Metric 1: [Target value]
- Metric 2: [Target value]

## Related Artifacts

**Instruction Files:**
- `instructions/instruct-YYYY_MM_DD-HH_MM-ticket_id_phase_description.md`

**Grade Files:**
- `grades/grade-YYYY_MM_DD-HH_MM-ticket_id_phase_description.md`

**Human Summaries:**
- `human/human-YYYY_MM_DD-HH_MM-ticket_id_phase_description.md`

**Logs:**
- `[code-agent]/debugging/logs/log-YYYY_MM_DD-HH_MM-ticket_id_phase_description.md`

**Change Documentation:**
- `[code-agent]/docs/changes/YYYY-MM-DD-ticket_id-description.md`

## Activity History

### YYYY-MM-DD HH:MM - [Activity]
[Detailed description]

## Notes

- Special consideration 1
- Special consideration 2
```

## PR Description Template

### Standard PR Template

```markdown
# [TICKET-ID]: [Title from Ticket]

## 🎫 Ticket

**Issue:** [Link to ticket]
**Type:** [Feature/Bug Fix/Refactor/Infrastructure/Documentation]
**Priority:** [Level]

## 🎯 Objective

[One paragraph description from ticket overview]

## 🐛 Problem (for bugs) / 🚀 Feature (for features)

### Problem Statement
[From ticket problem summary]

### Root Cause (for bugs)
[From ticket root cause section]

### Feature Requirements (for features)
[From ticket requirements]

## ✅ Solution

### Technical Approach
[From ticket solution section]

### Implementation Details
- Detail 1 from ticket
- Detail 2 from ticket

### Files Modified
[From ticket files affected table]
| File | Changes | Description |
|------|---------|-------------|
| file1 | lines X-Y | description |
| file2 | created | description |

## 📝 Changes Made

Based on instruction files:
- [description_1 from first instruction file] - What was implemented
- [description_2 from second instruction file] - What was implemented
- [description_3 from third instruction file] - What was implemented

## 🧪 Testing Performed

### Unit Tests
- ✅ All tests passing: X/X
- ✅ Coverage: X% (target: >= 60%)

### Integration Tests
- ✅ All tests passing: X/X
- ✅ Scenarios tested: [list]

### Manual Verification
- ✅ Verification 1 (from ticket success criteria)
- ✅ Verification 2 (from ticket success criteria)

### Test Commands
```bash
task test       # Unit tests
task cov        # Coverage report
task test-int   # Integration tests
```

## 📚 Documentation

- [x] Code comments added
- [x] Change documentation: `docs/changes/YYYY-MM-DD-ticket_id-description.md`
- [x] README updated (if applicable)
- [x] API docs updated (if applicable)

## 🔄 Rollback Procedure

From ticket rollback section:
1. Step 1
2. Step 2
3. Verification

## 📊 Related Logs

**Execution Logs:**
- `debugging/logs/log-YYYY_MM_DD-HH_MM-ticket_id_phase1_description.md`
- `debugging/logs/log-YYYY_MM_DD-HH_MM-ticket_id_phase2_description.md`

**Change Documentation:**
- `docs/changes/YYYY-MM-DD-ticket_id-description.md`

**Instruction Files:**
(Link to architect agent instruction files for reference)

## ✓ PR Checklist

### Code Quality
- [ ] Code follows project style guidelines
- [ ] Self-review completed
- [ ] No commented-out code or debug statements
- [ ] Error handling implemented
- [ ] Performance considerations addressed

### Testing
- [ ] All unit tests passing locally
- [ ] Coverage >= 60%
- [ ] Integration tests passing
- [ ] Manual testing completed
- [ ] Edge cases tested

### Documentation
- [ ] Code comments for complex logic
- [ ] README updated if needed
- [ ] Change documentation created
- [ ] API documentation updated

### Security
- [ ] No hardcoded secrets or credentials
- [ ] Input validation implemented
- [ ] Security best practices followed
- [ ] No new vulnerabilities introduced

### CI/CD
- [ ] All GitHub Actions checks passing
- [ ] Workflow changes tested (if applicable)
- [ ] No breaking changes to CI/CD

## 🔗 Related Pull Requests

- #XXX: [Description] (if related)

## 📌 Notes

From ticket notes section:
- Important note 1
- Important note 2
```

## Creating PR Descriptions from Tickets

### Process

1. **Read Current Ticket**
   ```bash
   cat ticket/current_ticket.md
   ```

2. **Read Detailed Ticket File**
   ```bash
   cat ticket/feature/ticket_id-name.md  # or ticket/bug/
   ```

3. **Extract Information:**
   - Title from ticket
   - Problem/Feature description
   - Solution approach
   - Files affected
   - Success criteria
   - Rollback procedure

4. **Read Instruction Files**
   ```bash
   ls instructions/instruct-*ticket_id*.md
   ```
   Extract descriptions from each instruction filename

5. **Read Grade Files (if available)**
   ```bash
   ls grades/grade-*ticket_id*.md
   ```
   Extract completion metrics

6. **Compile PR Description:**
   - Combine ticket information
   - List changes from instruction descriptions
   - Include testing results
   - Add links to documentation

### Automation Script Template

```bash
#!/bin/bash
# Generate PR description from ticket

TICKET_ID=$1
TICKET_FILE="ticket/feature/${TICKET_ID}-*.md"

# Read ticket information
TITLE=$(grep "^# " "$TICKET_FILE" | sed 's/# //')
PROBLEM=$(sed -n '/## Problem Summary/,/##/p' "$TICKET_FILE" | grep -v "^##")
SOLUTION=$(sed -n '/## Solution/,/##/p' "$TICKET_FILE" | grep -v "^##")

# Extract instruction descriptions
CHANGES=$(ls instructions/instruct-*${TICKET_ID}*.md | \
  xargs -I {} basename {} | \
  sed 's/instruct-[0-9_-]*-//' | \
  sed 's/\.md$//' | \
  sed 's/_/ /g' | \
  sed 's/^/- /')

# Create PR body
cat > pr-body.md <<EOF
# ${TITLE}

## 🎫 Ticket
[${TICKET_ID}](ticket-url)

## 🐛 Problem
${PROBLEM}

## ✅ Solution
${SOLUTION}

## 📝 Changes Made
${CHANGES}

## 🧪 Testing
- All unit tests passing
- Coverage >= 60%
- Manual verification complete

## 📚 Documentation
- Code comments added
- Change documentation created

## ✓ Checklist
- [x] Code reviewed
- [x] Tests passing
- [x] Documentation updated
EOF

cat pr-body.md
```

## Ticket Lifecycle Management

### 1. Create Ticket Detail File

When architect agent receives new ticket:

```bash
# Determine type
TICKET_TYPE="feature"  # or "bug"
TICKET_ID="TICKET-123"
TICKET_NAME="fix_auth_issue"

# Create detailed file
cat > "ticket/${TICKET_TYPE}/${TICKET_ID}-${TICKET_NAME}.md" <<EOF
[Use detailed ticket template]
EOF
```

### 2. Update Current Ticket

```bash
# Update current_ticket.md with active ticket
cat > "ticket/current_ticket.md" <<EOF
[Use current ticket template]
EOF
```

### 3. Update Throughout Work

After each phase/milestone:
```bash
# Update status section
# Update task checkboxes
# Add activity log entry
# Update success criteria progress
```

### 4. Archive on Completion

```bash
TICKET_ID="TICKET-123"
DATE=$(date +%Y-%m-%d)

# Move detailed file to archive
mv "ticket/feature/${TICKET_ID}-*.md" "ticket/archive/"

# Add completion info to archived file
echo "\n## Completion Summary\n" >> "ticket/archive/${TICKET_ID}-*.md"
echo "**Completed:** ${DATE}" >> "ticket/archive/${TICKET_ID}-*.md"
echo "**Final Grade:** A+ (98/100)" >> "ticket/archive/${TICKET_ID}-*.md"
echo "**Total Time:** X hours" >> "ticket/archive/${TICKET_ID}-*.md"

# Clear current_ticket.md or set to next ticket
```

## Integration with project-memory Skill

### Using project-memory for Docs Synchronization

```markdown
## After Ticket Completion

**Use project-memory skill to update code agent's docs/project_notes:**

1. **Update key_facts.md** with new configuration/infrastructure
2. **Update bugs.md** if bug was fixed (add BUG-XXX entry)
3. **Update decisions.md** if architectural decisions made
4. **Update issues.md** with ticket completion status

**Command:**
```bash
# Invoke project-memory skill
# Target: /path/to/code-agent/workspace/docs/project_notes
# Action: Update files with ticket outcomes
```

**Information to Transfer:**
- New facts discovered (ports, URLs, credentials locations)
- Bugs fixed with solutions
- Architectural decisions and rationale
- Ticket completion status

**Example Update:**
```markdown
### After PEAK-169 Completion

**key_facts.md updates:**
- WIF secret names: WIF_PROVIDER, WIF_SERVICE_ACCOUNT
- GitHub Actions workflows: infrastructure.yml, deploy-gcp.yml

**bugs.md updates:**
- BUG-031: GitHub Actions auth failure (secret name mismatch)
  - Solution: Use consistent WIF_* naming across workflows
  - Prevention: Workflow template with shared auth action

**issues.md updates:**
- PEAK-169: Complete (2025-10-28, Grade: A+)
```
```

## Quick Reference

### Ticket File Locations
- Active: `ticket/current_ticket.md`
- Feature details: `ticket/feature/TICKET-ID-name.md`
- Bug details: `ticket/bug/TICKET-ID-name.md`
- Archived: `ticket/archive/TICKET-ID-name.md`

### PR Creation Flow
1. Read ticket files
2. Extract key information
3. List changes from instruction files
4. Compile testing results
5. Add documentation links
6. Use PR template
7. Create PR via `gh pr create --body-file pr-body.md`

### Ticket Update Triggers
- Phase completion
- Blocker encountered
- Status change
- New information discovered
- Work completed

### project-memory Integration Points
- Ticket start: Check docs/project_notes for context
- During work: Note facts for later documentation
- Ticket complete: Update docs/project_notes via skill
