# Decision Types for Logging

**Version:** 1.0
**Purpose:** Standardized decision type labels for consistent logging across all agents

---

## Overview

When code agents log their work, they should categorize entries using these standard decision types. This enables better traceability, grading, and audit trails.

---

## Decision Types

### 1. `decision`

**Purpose:** A choice made between alternatives

**When to use:**
- Selecting between implementation approaches
- Choosing a library or framework
- Picking a design pattern

**Format:**
```markdown
**Decision:** [What was decided]
**Alternatives:** [What other options existed]
**Chosen:** [Which option was selected]
```

**Example:**
```bash
./debugging/scripts/log-decision.sh decision "Use Repository pattern for data access instead of direct database calls"
```

---

### 2. `rationale`

**Purpose:** Explanation of why a decision was made

**When to use:**
- Justifying a technical choice
- Explaining trade-offs considered
- Documenting business or technical reasoning

**Format:**
```markdown
**Rationale:** [Why this choice was made]
**Benefits:** [Expected positive outcomes]
**Trade-offs:** [What was sacrificed]
```

**Example:**
```bash
./debugging/scripts/log-decision.sh rationale "Repository pattern chosen for testability - allows mocking data layer in unit tests without database"
```

---

### 3. `investigation`

**Purpose:** Research or discovery activities

**When to use:**
- Exploring unfamiliar code
- Debugging to find root cause
- Researching documentation or APIs
- Understanding system behavior

**Format:**
```markdown
**Investigation:** [What was being investigated]
**Findings:** [What was discovered]
**Implications:** [What this means for the task]
```

**Example:**
```bash
./debugging/scripts/log-decision.sh investigation "Traced authentication flow - JWT validation happens in middleware before controller"
```

---

### 4. `verification`

**Purpose:** Confirming something works as expected

**When to use:**
- After implementing a feature
- After fixing a bug
- After deployment or configuration change
- Running tests to confirm behavior

**Format:**
```markdown
**Verification:** [What was verified]
**Method:** [How it was verified]
**Result:** [Success/Failure + details]
```

**Example:**
```bash
./debugging/scripts/log-decision.sh verification "API endpoint returns 200 with expected payload after auth header added"
```

---

### 5. `deviation`

**Purpose:** Documenting departure from instructions or plans

**When to use:**
- Instructions cannot be followed exactly
- Unexpected circumstances require different approach
- Blockers necessitate workaround

**Format:**
```markdown
**Deviation:** [What changed from plan]
**Reason:** [Why the deviation was necessary]
**Impact:** [How this affects outcome]
```

**Example:**
```bash
./debugging/scripts/log-decision.sh deviation "Used axios instead of fetch - fetch not available in Node 16 environment"
```

---

### 6. `milestone`

**Purpose:** Marking significant progress points

**When to use:**
- Completing a major task section
- Achieving a success criterion
- Reaching a checkpoint in multi-step work

**Format:**
```markdown
**Milestone:** [What was achieved]
**Progress:** [X of Y steps complete]
**Next:** [What comes next]
```

**Example:**
```bash
./debugging/scripts/log-decision.sh milestone "Database schema migration complete - 3 of 5 phases done"
```

---

## Quick Reference Table

| Type | When to Use | Key Elements |
|------|------------|--------------|
| `decision` | Making choices | Alternatives, selection, reasoning |
| `rationale` | Explaining why | Benefits, trade-offs, justification |
| `investigation` | Researching/discovering | Findings, implications |
| `verification` | Confirming correctness | Method, result |
| `deviation` | Changing from plan | Reason, impact |
| `milestone` | Progress checkpoint | Achievement, next steps |

---

## Usage Examples

### In Log Files

```markdown
## 14:32:15 - Investigation
**Investigation:** Checking why tests fail on CI but pass locally
**Findings:** CI uses Node 18, local uses Node 20 - API difference in fetch
**Implications:** Need to polyfill or use axios for compatibility

## 14:45:22 - Decision
**Decision:** Use axios for HTTP requests
**Alternatives:** fetch with polyfill, node-fetch, got
**Chosen:** axios - most stable, works across Node versions

## 14:46:00 - Rationale
**Rationale:** axios has consistent API across environments
**Benefits:** No polyfill needed, better error handling
**Trade-offs:** Additional dependency

## 15:12:45 - Verification
**Verification:** Tests pass on both Node 18 and Node 20
**Method:** Ran `npm test` in both environments
**Result:** ✅ All 47 tests passing

## 15:15:00 - Milestone
**Milestone:** HTTP client migration complete
**Progress:** 2 of 4 tasks complete
**Next:** Update API service layer
```

---

## Grading Impact

Proper use of decision types affects grading:

| Category | Points | Impact |
|----------|--------|--------|
| Logging & Traceability | 10 | Missing decision types = -3 points |
| Problem Solving | 10 | No investigation logs = -2 points |
| Communication | 15 | No milestone tracking = -2 points |

---

## Integration with Scripts

### Using log-decision.sh

```bash
# Log a decision
./debugging/scripts/log-decision.sh decision "Selected PostgreSQL over MySQL for JSON support"

# Log an investigation
./debugging/scripts/log-decision.sh investigation "Root cause found: connection pool exhaustion"

# Log a verification
./debugging/scripts/log-decision.sh verification "API responds in <100ms after index added"

# Log a milestone
./debugging/scripts/log-decision.sh milestone "Phase 1 complete - core API implemented"
```

### Script Location

The `log-decision.sh` script should be at:
```
[CODE_AGENT_WORKSPACE]/debugging/scripts/log-decision.sh
```

---

**Last Updated:** 2025-11-27
**Version:** 1.0
