# Logging Protocol for Code Agent (MANDATORY)

## Overview

**CRITICAL:** Every action, decision, command, and result MUST be logged in real-time. Logging is not optional—it's how the architect agent verifies your work and grades your performance.

## Golden Rule: Log → Act → Log Result

1. **[HH:MM:SS] LOG** what you're about to do
2. **EXECUTE** the command/action
3. **[HH:MM:SS] LOG** the result immediately
4. **REPEAT** for every action

**Never batch logs at the end. Log in real-time as work progresses.**

## Immediate Setup (BEFORE ANY WORK)

### Step 1: Create Log File FIRST

```bash
# Create log file with matching description
LOG_FILE="debugging/logs/log-$(date +%Y_%m_%d-%H_%M)-DESCRIPTION.md"
cat > "$LOG_FILE" << 'EOF'
# [TASK TITLE] - Execution Log
**Date:** $(date +%Y-%m-%d)
**Start Time:** $(date +%H:%M:%S)
**Agent:** Claude Code
**Task:** [Brief task description]

---

## Execution Timeline

EOF

echo "✅ Log file created: $LOG_FILE"
```

**The description MUST match the instruction filename:**
- Instruction: `instruct-2025_10_26-22_00-tkt123_phase6_cicd_integration.md`
- Log: `log-2025_10_26-22_00-tkt123_phase6_cicd_integration.md`

### Step 2: Set Log Variable

```bash
# Set for entire session
export LOG_FILE="debugging/logs/log-2025_10_26-22_00-tkt123_phase6_cicd_integration.md"
```

## Real-Time Logging with `tee`

### Use `tee` for ALL Commands

```bash
# ALWAYS pipe commands through tee to capture output in real-time

# Good example:
echo "[$(date +%H:%M:%S)] Running unit tests" | tee -a "$LOG_FILE"
task test 2>&1 | tee -a "$LOG_FILE"
echo "[$(date +%H:%M:%S)] Result: Tests completed" | tee -a "$LOG_FILE"
echo "" | tee -a "$LOG_FILE"

# Bad example (don't do this):
task test  # ❌ Output not captured in log
echo "Tests completed" >> "$LOG_FILE"  # ❌ No actual output logged
```

### Log Entry Format

```markdown
[HH:MM:SS] Action: [What you're doing]
**Command:** `exact-command-here`
**Purpose:** [Why running this]
**Output:**
```
[command output via tee]
```
**Result:** ✅ Success / ❌ Failed
**Next:** [What's next]
---
```

## Practical Examples

### Example 1: Running Tests

```bash
# Log before
echo "[$(date +%H:%M:%S)] Running unit tests" | tee -a "$LOG_FILE"
echo "**Command:** \`task test\`" | tee -a "$LOG_FILE"
echo "**Purpose:** Verify all tests pass after code changes" | tee -a "$LOG_FILE"
echo "**Output:**" | tee -a "$LOG_FILE"
echo '```' | tee -a "$LOG_FILE"

# Run command with tee
task test 2>&1 | tee -a "$LOG_FILE"
TEST_RESULT=${PIPESTATUS[0]}

# Log result immediately
echo '```' | tee -a "$LOG_FILE"
if [ $TEST_RESULT -eq 0 ]; then
    echo "**Result:** ✅ All tests passing (588/588)" | tee -a "$LOG_FILE"
    echo "**Next:** Continue with next task" | tee -a "$LOG_FILE"
else
    echo "**Result:** ❌ Tests failed" | tee -a "$LOG_FILE"
    echo "**Next:** Investigate failures using root-cause-debugger" | tee -a "$LOG_FILE"
fi
echo "---" | tee -a "$LOG_FILE"
echo "" | tee -a "$LOG_FILE"
```

### Example 2: Deploying Infrastructure

```bash
# Log decision first
echo "[$(date +%H:%M:%S)] Decision: Deploy backend-api first (dependency for migrations)" | tee -a "$LOG_FILE"
echo "**Rationale:** Migrations need backend service to exist" | tee -a "$LOG_FILE"
echo "" | tee -a "$LOG_FILE"

# Log action
echo "[$(date +%H:%M:%S)] Deploying backend-api via Pulumi" | tee -a "$LOG_FILE"
echo "**Command:** \`pulumi up --yes --stack acme-sales_leads_gen-qa\`" | tee -a "$LOG_FILE"
echo "**Purpose:** Deploy backend API to Cloud Run" | tee -a "$LOG_FILE"
echo "**Output:**" | tee -a "$LOG_FILE"
echo '```' | tee -a "$LOG_FILE"

# Execute with tee
cd infrastructure
pulumi up --yes --stack acme-sales_leads_gen-qa 2>&1 | tee -a "../$LOG_FILE"
DEPLOY_RESULT=${PIPESTATUS[0]}
cd ..

# Log result
echo '```' | tee -a "$LOG_FILE"
if [ $DEPLOY_RESULT -eq 0 ]; then
    echo "**Result:** ✅ Deployment successful" | tee -a "$LOG_FILE"
    SERVICE_URL=$(gcloud run services describe backend-api --region=us-central1 --format='value(status.url)')
    echo "**Service URL:** $SERVICE_URL" | tee -a "$LOG_FILE"
    echo "**Next:** Run database migrations" | tee -a "$LOG_FILE"
else
    echo "**Result:** ❌ Deployment failed" | tee -a "$LOG_FILE"
    echo "**Next:** Investigate error using Perplexity MCP" | tee -a "$LOG_FILE"
fi
echo "---" | tee -a "$LOG_FILE"
echo "" | tee -a "$LOG_FILE"
```

### Example 3: Investigating Errors

```bash
# Log the error
echo "[$(date +%H:%M:%S)] ❌ Error Encountered" | tee -a "$LOG_FILE"
echo "**Command:** \`gcloud run deploy backend-api\`" | tee -a "$LOG_FILE"
echo "**Error:** Permission denied: artifactregistry.repositories.uploadArtifacts" | tee -a "$LOG_FILE"
echo "**Status:** FAILED" | tee -a "$LOG_FILE"
echo "" | tee -a "$LOG_FILE"

# Log investigation
echo "[$(date +%H:%M:%S)] Investigating error with Perplexity" | tee -a "$LOG_FILE"
echo "**Tool:** Perplexity MCP" | tee -a "$LOG_FILE"
echo "**Query:** Permission artifactregistry.repositories.uploadArtifacts denied" | tee -a "$LOG_FILE"
# [Run Perplexity query]
echo "**Finding:** Service account needs Artifact Registry Writer role" | tee -a "$LOG_FILE"
echo "" | tee -a "$LOG_FILE"

# Log fix attempt
echo "[$(date +%H:%M:%S)] Applying fix: Add IAM role" | tee -a "$LOG_FILE"
echo "**Command:** \`gcloud projects add-iam-policy-binding...\`" | tee -a "$LOG_FILE"
echo '```' | tee -a "$LOG_FILE"
gcloud projects add-iam-policy-binding acme-labs-sales_leads_gen \
    --member="serviceAccount:acme-ingestion-qa@acme-labs-sales_leads_gen.iam.gserviceaccount.com" \
    --role="roles/artifactregistry.writer" 2>&1 | tee -a "$LOG_FILE"
echo '```' | tee -a "$LOG_FILE"
echo "**Result:** ✅ Permission granted" | tee -a "$LOG_FILE"
echo "" | tee -a "$LOG_FILE"

# Log retry
echo "[$(date +%H:%M:%S)] Retrying original command" | tee -a "$LOG_FILE"
echo "**Command:** \`gcloud run deploy backend-api\`" | tee -a "$LOG_FILE"
echo '```' | tee -a "$LOG_FILE"
gcloud run deploy backend-api --image=... 2>&1 | tee -a "$LOG_FILE"
echo '```' | tee -a "$LOG_FILE"
echo "**Result:** ✅ SUCCESS - Deployment complete" | tee -a "$LOG_FILE"
echo "---" | tee -a "$LOG_FILE"
echo "" | tee -a "$LOG_FILE"
```

### Example 4: Making Decisions

```bash
# Log the decision point
echo "[$(date +%H:%M:%S)] Decision Point: VPC Connector Configuration" | tee -a "$LOG_FILE"
echo "**Context:** Cloud Run Job failing with 'no pg_hba.conf entry'" | tee -a "$LOG_FILE"
echo "**Investigation:** Checking working services (backend-api, ingestion)" | tee -a "$LOG_FILE"
echo '```' | tee -a "$LOG_FILE"
gcloud run services describe backend-api --region=us-central1 --format='yaml(spec.template.spec.containers[0].vpc)' 2>&1 | tee -a "$LOG_FILE"
gcloud run services describe ingestion --region=us-central1 --format='yaml(spec.template.spec.containers[0].vpc)' 2>&1 | tee -a "$LOG_FILE"
echo '```' | tee -a "$LOG_FILE"
echo "**Finding:** Working services have NO VPC connector" | tee -a "$LOG_FILE"
echo "**Decision:** Remove VPC connector from job to match working pattern" | tee -a "$LOG_FILE"
echo "**Rationale:** Working services prove VPC connector unnecessary for database access" | tee -a "$LOG_FILE"
echo "" | tee -a "$LOG_FILE"
```

### Example 5: Verification After Action

```bash
# Always verify actions worked
echo "[$(date +%H:%M:%S)] Verification: Check service is healthy" | tee -a "$LOG_FILE"
echo "**Command:** \`curl \$SERVICE_URL/health\`" | tee -a "$LOG_FILE"
echo "**Purpose:** Verify backend API responds correctly" | tee -a "$LOG_FILE"
echo '```' | tee -a "$LOG_FILE"
curl -f "$SERVICE_URL/health" 2>&1 | tee -a "$LOG_FILE"
HEALTH_RESULT=${PIPESTATUS[0]}
echo '```' | tee -a "$LOG_FILE"

if [ $HEALTH_RESULT -eq 0 ]; then
    echo "**Result:** ✅ Service is healthy" | tee -a "$LOG_FILE"
else
    echo "**Result:** ❌ Health check failed" | tee -a "$LOG_FILE"
    echo "**Next:** Check Cloud Logging for errors" | tee -a "$LOG_FILE"
fi
echo "---" | tee -a "$LOG_FILE"
echo "" | tee -a "$LOG_FILE"
```

## What to Log

### ALWAYS Log These:

1. **Commands before execution**
   - Exact command with all flags
   - Purpose/rationale

2. **Command output via `tee`**
   - Complete stdout/stderr
   - Exit codes

3. **Decisions and rationale**
   - Why choosing approach A over B
   - What alternatives were considered

4. **Errors and investigations**
   - Exact error messages
   - Research conducted (Perplexity, context7)
   - Solutions attempted

5. **Verifications**
   - How you verified action worked
   - What you checked
   - Results of verification

6. **Deviations from plan**
   - What instruction said to do
   - What you did instead
   - Why you deviated

7. **Milestones**
   - Major steps completed
   - Tests passing
   - Coverage checkpoints

### Update Frequency

**Log after:**
- Every command execution
- Every 3 commands maximum
- Every file modification
- Every error
- Every 5 minutes
- Completing each subtask

**Never wait until end to log!**

## Logging Checkpoints

### Session Start (Mandatory)

```markdown
[HH:MM:SS] Session Started
**Goal:** [Brief task description]
**Success Criteria:**
- [ ] Criterion 1
- [ ] Criterion 2
- [ ] Criterion 3
**Baseline:**
- Tests: X/X passing
- Coverage: X%
---
```

### Every 15 Minutes (Status Update)

```markdown
[HH:MM:SS] ⏱️ Status Checkpoint
**Elapsed:** 15 minutes
**Progress:**
- ✅ Task 1 complete
- 🔄 Task 2 in progress (60% done)
- ⏳ Task 3 pending
**Issues:** None / [Description if any]
**Next:** [What's next]
---
```

### Before Completion (Final Summary)

```markdown
[HH:MM:SS] 🏁 Final Summary
**All Success Criteria Met:**
- [x] Criterion 1 - Evidence: [where to verify]
- [x] Criterion 2 - Evidence: [where to verify]
- [x] Criterion 3 - Evidence: [where to verify]

**Test Results:**
- Unit tests: X/X passing ✅
- Integration tests: X/X passing ✅
- Coverage: X% (>= 60%) ✅

**Agents Used:**
- [x] qa-enforcer - Final validation
- [x] change-explainer - Documentation analysis
- [x] docs-sync-editor - README.md updated

**Files Modified:**
- file1.py - [what changed]
- file2.md - [what changed]

**Total Duration:** X hours Y minutes
**Status:** ✅ COMPLETE
---
```

## Template for Instructions

**Every instruction to code agent MUST start with:**

```markdown
# [Task Title]

## CRITICAL: Logging Requirements (READ FIRST)

**MANDATORY:** Create log file IMMEDIATELY before any work:

```bash
# 1. Create log file with matching description
export LOG_FILE="debugging/logs/log-$(date +%Y_%m_%d-%H_%M)-MATCHING_DESCRIPTION.md"
cat > "$LOG_FILE" << 'EOF'
# [Task Title] - Execution Log
**Date:** $(date +%Y-%m-%d)
**Start Time:** $(date +%H:%M:%S)
**Agent:** Claude Code
**Task:** [Task description]
---
## Execution Timeline
EOF

# 2. Log EVERY command with tee:
echo "[$(date +%H:%M:%S)] Action description" | tee -a "$LOG_FILE"
echo "**Command:** \`your-command\`" | tee -a "$LOG_FILE"
echo '```' | tee -a "$LOG_FILE"
your-command 2>&1 | tee -a "$LOG_FILE"
RESULT=${PIPESTATUS[0]}
echo '```' | tee -a "$LOG_FILE"
echo "**Result:** ✅/❌" | tee -a "$LOG_FILE"
echo "---" | tee -a "$LOG_FILE"

# 3. Log decisions, errors, verifications immediately
# 4. Update log after every 3 commands or 5 minutes
# 5. NEVER batch logs at end
```

## Golden Rules

1. **Log → Act → Log Result** (never skip this pattern)
2. **Use `tee -a "$LOG_FILE"` for ALL command output**
3. **Capture exit codes:** `RESULT=${PIPESTATUS[0]}`
4. **Log decisions and rationale** (not just commands)
5. **Verify every action** and log the verification
6. **Real-time logging** (not batch at end)
7. **Timestamps on everything:** `[$(date +%H:%M:%S)]`

## Why This Matters

The architect agent:
- ✅ **CAN'T verify your work** without logs
- ✅ **CAN'T grade properly** without evidence
- ✅ **CAN'T see your problem-solving** without decision logs
- ✅ **CAN'T confirm resilience** without seeing retries

**No logs = Maximum grade of C+ (78%), even if work is perfect**

Logging is NOT overhead—it's proof of work and evidence of quality.

## Grading Impact

**Logging & Traceability (10 points):**

| Score | Criteria |
|-------|----------|
| 10 | Perfect logging, real-time updates via tee |
| 8 | Good logging, mostly timely |
| 6 | Adequate logging, some delays |
| 4 | Poor logging, significant gaps |
| 0-3 | No logging or batch logs at end |

**Additional deductions:**
- Not using `tee` for command output: -2 points
- Batch logging at end instead of real-time: -3 points
- Missing decision rationale: -1 point per occurrence
- Missing error investigation logs: -2 points
- No verification logs: -2 points
- Log filename doesn't match instruction: -1 point
