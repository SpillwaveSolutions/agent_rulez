# Resilience and Recovery Protocol

## Overview

Always instruct the code agent to be resilient and adaptive. Failures are opportunities to demonstrate problem-solving, not reasons to give up.

## Core Principles

### 1. Be Resilient and Adaptive

- Retry failed operations with different approaches
- Use workarounds when blocked, but document them
- Deviate from plan if needed, but log why
- Never give up on first failure

### 2. Use Tools for Recovery

- Use `context7` MCP for library documentation
- Use `perplexity` MCP to research errors
- Use `root-cause-debugger` agent for complex issues
- Use `qa-enforcer` agent to verify all changes

### 3. Verify Every Action

- Check return codes for all commands
- Verify resources were actually created
- Test file existence after creation
- Validate configuration changes took effect

### 4. Document Everything

- Log successful approaches
- Log failed attempts and why they failed
- Log workarounds and deviations
- Log verification results for every action

## Error Recovery Protocol

### When You Encounter Errors:

1. Log the exact error message
2. Use context7 to look up the error
3. Use perplexity to research solutions
4. Try alternative approaches
5. Document what worked/didn't work

### Example:

```markdown
[14:30:00] Command Failed
**Command:** `gcloud run deploy backend-api`
**Error:** "Permission denied: artifactregistry.repositories.uploadArtifacts"
**Status:** ❌ FAILED

[14:30:15] Researching Error
**Tool Used:** Perplexity MCP
**Query:** "Permission artifactregistry.repositories.uploadArtifacts denied Cloud Build"
**Finding:** Service account needs Artifact Registry Writer role

[14:31:00] Applying Fix
**Command:** `gcloud projects add-iam-policy-binding PROJECT --member=serviceAccount:SA --role=roles/artifactregistry.writer`
**Result:** ✅ Permission granted

[14:32:00] Retrying Original Command
**Command:** `gcloud run deploy backend-api`
**Result:** ✅ SUCCESS - Deployment complete
```

## Verification Requirements

### After EVERY action:

- Check return codes
- Verify the resource/file exists
- Test that it works as expected
- Log the verification results

### Example for Cloud Resource:

```bash
# Create resource
gcloud run services create backend-api --image=...
RESULT=$?

# Check return code
if [ $RESULT -ne 0 ]; then
  echo "❌ Failed to create service"
  # Log error and try alternative
fi

# Verify it exists
gcloud run services describe backend-api --region=us-central1
if [ $? -eq 0 ]; then
  echo "✅ Service created successfully"
fi

# Log verification result
```

## Deviation Protocol

It's OK to deviate from the plan if needed:

- Try the instructed approach first
- If blocked, find a workaround
- Document WHY you deviated
- Document WHAT you did instead
- Ensure the goal is still achieved

### Example:

```markdown
[14:45:00] Deviation from Plan
**Original Plan:** Use VPC connector for Cloud Run Job
**Problem:** VPC connector causing connection failures
**Investigation:** Compared with working services (backend-api, ingestion)
**Finding:** Working services have NO VPC connector
**Decision:** Remove VPC connector to match working pattern
**Result:** ✅ Job now works correctly
**Justification:** Working services prove VPC connector unnecessary
```

## Instructions Template

When creating instructions for the code agent, include:

```markdown
## Error Recovery Protocol

If you encounter errors:
1. Log the exact error message
2. Use context7 to look up the error
3. Use perplexity to research solutions
4. Try alternative approaches
5. Document what worked/didn't work

## Verification Requirements

After EVERY action:
- Check return codes
- Verify the resource/file exists
- Test that it works as expected
- Log the verification results

## Deviation Protocol

It's OK to deviate from the plan if needed:
- Try the instructed approach first
- If blocked, find a workaround
- Document WHY you deviated
- Document WHAT you did instead
- Ensure the goal is still achieved
```

## Grading Impact

**Resilience & Adaptability (10 points):**

| Score | Behavior |
|-------|----------|
| 10 | Excellent recovery, smart workarounds |
| 8 | Good recovery, reasonable adaptations |
| 6 | Basic recovery, some rigidity |
| 4 | Poor recovery, stuck on failures |
| 0-3 | No recovery attempts, gave up |

**What evaluators look for:**
- Did the code agent retry failures?
- Were alternative approaches attempted?
- Were workarounds documented?
- Was research conducted (context7, perplexity)?
- Were deviations justified?
