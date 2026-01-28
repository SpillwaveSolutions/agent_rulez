# Get Unstuck Protocol

**Version:** 1.0
**Last Updated:** 2025-01-15
**Purpose:** Systematic multi-channel research strategy when blocked after 2-3 failed attempts

---

## Overview

When a code agent encounters a problem that fails after **2-3 attempts**, they MUST trigger the **"Get Unstuck" protocol** - a comprehensive research strategy using all available tools (MCPs, skills, web search) to find solutions.

**Architect agents:** Include this protocol in instructions and grade code agents on whether they use it when stuck.

---

## Trigger Conditions

**Code agent MUST activate "Get Unstuck" protocol when:**

1. ✅ **2-3 failed attempts** at solving the same problem
2. ✅ **Error persists** despite different approaches
3. ✅ **Stuck for 10+ minutes** on single issue
4. ✅ **Unfamiliar territory** (new library, new API, new error)

**DO NOT use for:**
- ❌ First attempt at solving a problem (normal debugging first)
- ❌ Simple syntax errors
- ❌ Known issues with documented solutions in your memory

---

## The Protocol

### Step 1: Acknowledge Being Stuck

**Log the decision to get unstuck:**

```bash
./debugging/scripts/log-decision.sh investigation \
  "STUCK after 3 failed attempts. Error: [exact error message]. Entering Get Unstuck protocol."
```

**Document what's been tried:**
```bash
./debugging/scripts/log-decision.sh investigation \
  "Attempts so far: 1) [approach 1] - failed because [reason], 2) [approach 2] - failed because [reason], 3) [approach 3] - failed because [reason]"
```

### Step 2: Multi-Channel Research

**Execute searches based on problem type. Use ALL applicable channels:**

#### 2A. Perplexity MCP Search (AI-Powered Research)

**When to use:** First choice for recent issues, best practices, "how to" questions

```typescript
// Use perplexity MCP tool
mcp__perplexity-ask__perplexity_ask({
  messages: [
    {
      role: "user",
      content: "I'm getting error: [exact error message]. Context: [what you're trying to do]. I've tried: 1) [approach 1], 2) [approach 2], 3) [approach 3]. What am I missing?"
    }
  ]
})
```

**Log the research:**
```bash
./debugging/scripts/log-decision.sh investigation \
  "Perplexity search: [query]. Key findings: [bullet points of insights]"
```

#### 2B. Brave MCP Search (Fresh Web Results)

**When to use:** For specific error messages, version-specific issues, recent changes

```typescript
// Use brave web search MCP
mcp__brave-search__brave_web_search({
  query: "[exact error message] [library name] [version]",
  count: 10
})
```

**Search query variations to try:**
1. `"[exact error message]" [library name]`
2. `[error message] solution [library] [version]`
3. `[library] [version] common issues`
4. `[error code] stackoverflow`

**Log the research:**
```bash
./debugging/scripts/log-decision.sh investigation \
  "Brave search: [query]. Top 3 results: 1) [summary], 2) [summary], 3) [summary]"
```

#### 2C. Regular WebSearch (Fallback)

**When to use:** If MCP searches unavailable or need broader results

```typescript
// Use built-in WebSearch tool
WebSearch({
  query: "[problem description] [technology] [version] solution"
})
```

#### 2D. Context7 MCP (API/Library Documentation)

**When to use:** Problem involves API usage, library configuration, or "how to use X"

**Step 1: Identify the library**
```bash
./debugging/scripts/log-decision.sh investigation \
  "This is an API/library issue. Researching [library-name] documentation via Context7."
```

**Step 2: Resolve library ID**
```typescript
mcp__context7__resolve-library-id({
  libraryName: "[library-name]"
})
```

**Step 3: Get relevant documentation**
```typescript
mcp__context7__get-library-docs({
  context7CompatibleLibraryID: "/org/project",
  topic: "[specific topic like 'authentication', 'configuration', 'error handling']",
  tokens: 5000
})
```

**Log the research:**
```bash
./debugging/scripts/log-decision.sh investigation \
  "Context7 docs for [library] on topic '[topic]'. Key findings: [summary of relevant sections]"
```

#### 2E. Gemini Skill (Alternative AI Perspective)

**When to use:** After gathering search results, need fresh analytical perspective

**Check if available and use:**

```bash
# Log the gemini consultation
./debugging/scripts/log-decision.sh investigation \
  "Consulting Gemini skill with full error context and research findings"

# Use gemini skill (if available)
# Provide: original problem, all attempts, error messages, search results
# Ask: "What am I missing? What should I try next?"
```

**Log Gemini's perspective:**
```bash
./debugging/scripts/log-decision.sh investigation \
  "Gemini insight: [key observation]. Suggested approach: [recommendation]"
```

### Step 3: Synthesize Findings

**Combine all research:**

```bash
./debugging/scripts/log-decision.sh decision \
  "Based on research (Perplexity: [insight], Brave: [insight], Context7: [insight], Gemini: [insight]), trying NEW approach: [description]"

./debugging/scripts/log-decision.sh rationale \
  "This approach differs from previous attempts by: [key differences]. Why it should work: [reasoning]"
```

### Step 4: Try New Approach

**Implement based on research:**
- Apply the solution from research
- Log each step
- Verify results
- Document outcome

### Step 5: Document Success or Escalation

**If UNSTUCK:**

```bash
./debugging/scripts/log-decision.sh milestone \
  "UNSTUCK: Problem solved using [which resource provided solution]. Root cause was: [explanation]. Solution: [what fixed it]"
```

**If STILL STUCK after Get Unstuck protocol:**

```bash
./debugging/scripts/log-decision.sh investigation \
  "Get Unstuck protocol completed. Tried: Perplexity, Brave, Context7, Gemini. Still blocked. Research summary: [what was found]. Requesting human assistance."
```

---

## Decision Tree: Which Tools When

```
Problem after 2-3 attempts
         |
         v
  Is it API/Library related?
         |
    YES  |  NO
     |   |
     v   v
Context7  Error message?
first     |
     YES  |  NO
      |   |
      v   v
   Brave  Conceptual?
   first  |
      YES |  NO
       |  |
       v  v
  Perplexity  ALL channels
  first       in parallel

All channels converge to:
         |
         v
    Synthesize
         |
         v
   Try new approach
         |
    SUCCESS? --NO--> Still stuck after
         |           Get Unstuck?
        YES          Ask human
         |
         v
   Document solution
```

---

## Orchestration Script

**`debugging/scripts/get-unstuck.sh`** - Automates multi-search

**Features:**
- Auto-detects which MCPs are available
- Runs searches in parallel for speed
- Falls back to alternatives if MCP unavailable
- Aggregates results
- Logs everything automatically
- Provides actionable recommendations

**Usage:**
```bash
./debugging/scripts/get-unstuck.sh \
  --error "ImportError: No module named 'pydantic'" \
  --context "Installing FastAPI dependencies" \
  --attempts 3 \
  --type library
```

**Script decides:**
- Which MCPs to use based on availability
- Whether to consult Gemini
- Whether it's API-related (trigger Context7)
- Query variations to try

---

## MCP Availability Fallbacks

### Priority Order by Problem Type

**For API/Library Issues:**
1. Context7 MCP (best for API docs)
2. Perplexity MCP (if Context7 unavailable)
3. Brave MCP + WebSearch
4. WebFetch official docs directly

**For Error Messages:**
1. Brave MCP (best for specific errors)
2. Perplexity MCP
3. WebSearch built-in
4. Context7 if library-related

**For Conceptual/How-To:**
1. Perplexity MCP (best for explanations)
2. Brave MCP
3. WebSearch built-in
4. Context7 for library-specific concepts

**For Unknown Problems:**
- Use ALL channels in parallel
- Synthesize findings from all sources

**If NO MCPs available:**
1. Built-in WebSearch tool (always available)
2. WebFetch to read documentation pages
3. Grep codebase for similar patterns
4. Check project CLAUDE.md for past solutions

---

## Grading Criteria

### Resilience & Adaptability (10 points)

**Updated scoring for Get Unstuck protocol:**

| Score | Criteria |
|-------|----------|
| 10 | Used Get Unstuck protocol when stuck (2-3 attempts), comprehensive multi-channel research, documented findings, found solution |
| 8-9 | Used Get Unstuck protocol, most channels researched, good synthesis, found solution or made progress |
| 6-7 | Used some research tools, partial Get Unstuck, incomplete documentation |
| 4-5 | Minimal research, repeated same approach 4+ times before trying Get Unstuck |
| 0-3 | Gave up without Get Unstuck protocol, or never tried systematic research |

**Bonus Points:**
- **+2 points:** Used Get Unstuck proactively (after 2 attempts, not 5+)
- **+1 point:** Documented which specific resource solved the problem
- **+1 point:** Updated CLAUDE.md with learning for future sessions

**Deductions:**
- **-3 points:** Repeated same failed approach 5+ times without using Get Unstuck
- **-5 points:** Asked for help without trying Get Unstuck protocol first
- **-2 points:** Used Get Unstuck but skipped available channels (e.g., had Context7 but didn't use it for API issue)
- **-4 points:** Gave up after single search attempt (didn't try multiple channels)

---

## Architect Agent Instructions Template

**Include this in every instruction where complexity/unknowns exist:**

```markdown
## Get Unstuck Protocol (MANDATORY)

If you encounter a problem that fails after **2-3 attempts**, you MUST use the Get Unstuck protocol:

1. **Log that you're stuck:** Document what you've tried
2. **Multi-channel research:**
   - Perplexity MCP: AI-powered search for the error/problem
   - Brave MCP: Web search for specific error messages
   - Context7 MCP: If API/library related, get official docs
   - WebSearch: Fallback if MCPs unavailable
   - Gemini skill: Alternative AI perspective (if available)
3. **Synthesize findings:** Combine insights from all sources
4. **Try new approach:** Based on research, not previous failed attempts
5. **Document outcome:** What worked, which resource helped, root cause

**You will be graded on:**
- Whether you used Get Unstuck when stuck (not optional)
- How many research channels you used
- Quality of synthesis and new approach
- Documentation of findings

**Automatic deductions:**
- -5 points: Asking for help without trying Get Unstuck
- -3 points: Repeating same approach 5+ times
- -2 points: Skipping available research channels
```

---

## Architect Agent Grading Checklist

When grading code agent work, check:

- [ ] **Did they get stuck?** (2-3 failed attempts on same problem)
- [ ] **Did they acknowledge being stuck?** (logged investigation)
- [ ] **Did they use Get Unstuck protocol?** (if stuck and didn't, major deduction)
- [ ] **Which channels did they use?**
  - [ ] Perplexity MCP
  - [ ] Brave MCP
  - [ ] Context7 MCP (if API-related)
  - [ ] WebSearch (if MCPs unavailable)
  - [ ] Gemini skill (if available)
- [ ] **Did they document research findings?** (investigation logs)
- [ ] **Did they synthesize findings?** (decision + rationale logs)
- [ ] **Did new approach differ from previous?** (not just retry #4)
- [ ] **Did they document which resource solved it?** (milestone log)
- [ ] **Did they update CLAUDE.md with learning?** (bonus point)

**Common failure patterns to deduct for:**
- Tried same approach 5+ times without research
- Asked for help without Get Unstuck
- Used only 1 search channel when others available
- No synthesis (just tried first Google result)
- Didn't document which resource helped

---

## Examples

### Example 1: OAuth API 401 Error

**Situation:** 3 failed authentication attempts

```bash
# Step 1: Acknowledge
./debugging/scripts/log-decision.sh investigation \
  "STUCK after 3 auth attempts. Error: 401 Unauthorized. Entering Get Unstuck."

./debugging/scripts/log-decision.sh investigation \
  "Attempts: 1) Basic auth header - 401, 2) Different token format - 401, 3) Regenerated token - 401"

# Step 2A: Perplexity
mcp__perplexity-ask__perplexity_ask({
  messages: [{
    role: "user",
    content: "Getting 401 Unauthorized with OAuth2 API. Tried: basic auth header, different token formats, regenerated token. What's wrong?"
  }]
})

./debugging/scripts/log-decision.sh investigation \
  "Perplexity: OAuth2 requires 'Bearer ' prefix before token in Authorization header"

# Step 2B: Brave
mcp__brave-search__brave_web_search({
  query: "OAuth2 401 Unauthorized Bearer token",
  count: 10
})

./debugging/scripts/log-decision.sh investigation \
  "Brave: Multiple Stack Overflow posts confirm Bearer prefix required"

# Step 2C: Context7
mcp__context7__get-library-docs({
  context7CompatibleLibraryID: "/oauth/oauth2-library",
  topic: "authentication headers"
})

./debugging/scripts/log-decision.sh investigation \
  "Context7: Official docs show 'Authorization: Bearer {token}' format"

# Step 3: Synthesize
./debugging/scripts/log-decision.sh decision \
  "All three sources (Perplexity + Brave + Context7) confirm: need 'Bearer ' prefix. Previous attempts used raw token."

./debugging/scripts/log-decision.sh rationale \
  "OAuth2 spec requires Bearer prefix to identify token type. My attempts missed this."

# Step 4: Try new approach
# Add "Bearer " prefix to Authorization header

# Step 5: Document success
./debugging/scripts/log-decision.sh milestone \
  "UNSTUCK via Context7 + Perplexity + Brave convergence. Root cause: missing 'Bearer ' prefix. Authentication now working."
```

**Grading:** 10/10 - Perfect Get Unstuck execution

### Example 2: Module Not Found (Bad Example)

**Situation:** Module not found after 5 attempts

```bash
# Attempt 1-5: All just "pip install pydantic"
# NO Get Unstuck protocol used
# Just kept retrying same command

./debugging/scripts/log-decision.sh investigation \
  "Still can't import pydantic. Tried pip install 5 times. Asking for help."
```

**Grading:** 3/10 - Failed to use Get Unstuck protocol
- Deduction: -5 points for asking help without Get Unstuck
- Deduction: -3 points for repeating same approach 5 times

### Example 3: Configuration Error (Good Example)

**Situation:** Service won't start, silent failure

```bash
# Step 1: Acknowledge (after 3 attempts)
./debugging/scripts/log-decision.sh investigation \
  "STUCK after 3 attempts. Service fails silently. Attempts: 1) checked env vars, 2) recreated config, 3) restarted service. Entering Get Unstuck."

# Step 2: Research (Gemini provides key insight)
./debugging/scripts/log-decision.sh investigation \
  "Gemini insight: Silent failures often mean exception caught too broadly. Add debug logging to see actual error."

# Step 3: Follow Gemini's advice
# Add debug logging

# Step 4: Discover real error
./debugging/scripts/log-decision.sh investigation \
  "Debug logs revealed true error: DATABASE_URL missing 'postgresql://' prefix"

# Step 5: Document
./debugging/scripts/log-decision.sh milestone \
  "UNSTUCK via Gemini insight. Gemini suggested debug logging which revealed root cause. Fixed DB URL format."
```

**Grading:** 10/10 + 1 bonus = 11/10 (capped at 10)
- Used Get Unstuck after 3 attempts (proactive)
- Documented which resource helped (Gemini)

---

## Integration with Hybrid Logging

All Get Unstuck activities use hybrid logging:

**Automated (hooks log):**
- MCP tool calls
- Command executions
- File reads/writes

**Manual (log-decision.sh):**
- Investigation: Being stuck, what was tried
- Investigation: Research findings from each channel
- Decision: Synthesized approach
- Rationale: Why new approach should work
- Milestone: Solution and which resource helped

---

## Script Permissions

**Add to code agent `.claude/settings.local.json`:**

```json
{
  "permissions": {
    "allow": [
      "Bash(./debugging/scripts/get-unstuck.sh:*)",
      "Bash(debugging/scripts/get-unstuck.sh:*)"
    ]
  }
}
```

---

## Success Metrics

**Track in code agent CLAUDE.md:**

```markdown
## Get Unstuck History

### Recent Successes
- **2025-01-15**: OAuth Bearer prefix → Context7 + Perplexity
- **2025-01-16**: Pydantic version conflict → Perplexity
- **2025-01-17**: Silent config error → Gemini insight

### Most Helpful Resources
1. Context7: 45% of solutions (API/library issues)
2. Perplexity: 35% of solutions (general errors)
3. Gemini: 15% of solutions (complex/subtle issues)
4. Brave: 5% of solutions (specific version issues)

### Lessons Learned
- Always check Context7 first for API issues
- Gemini excels at suggesting debugging approaches
- Perplexity best for "what am I missing?" questions
```

---

## References

- [Hybrid Logging Protocol](./hybrid_logging_protocol.md) - For logging research
- [Grading Rubrics](./grading_rubrics.md) - For resilience scoring
- [Permissions Setup Protocol](./permissions_setup_protocol.md) - For script permissions

---

**Version History:**
- **v1.0 (2025-01-15):** Initial Get Unstuck protocol with multi-channel research
