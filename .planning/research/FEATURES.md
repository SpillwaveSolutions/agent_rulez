# Feature Landscape: v1.3 Policy Engine Features

**Domain:** Policy engines, YAML-based configuration, runtime validation
**Researched:** 2026-02-08
**Focus:** prompt_match, require_fields, inline scripts

---

## Executive Summary

This research covers three feature additions for RuleZ v1.3 that extend the rule-matching and validation capabilities:

1. **prompt_match** - Route rules based on user prompt text (intent-based routing)
2. **require_fields** - Validate tool inputs have required fields before execution
3. **Inline scripts** - Embed validator scripts directly in YAML without separate files

These features follow established patterns from policy engines (OPA, Kubernetes admission webhooks), API gateways (AWS API Gateway, Kong), and CI/CD systems (GitHub Actions, GitLab CI, Ansible).

**Key Finding:** All three features are table stakes for a comprehensive policy engine in 2026. They're expected features that users will assume exist based on patterns from other policy systems.

---

## Table Stakes Features

Features users expect based on industry patterns. Missing these makes RuleZ feel incomplete compared to modern policy engines.

### 1. prompt_match: Regex-Based Prompt Routing

| Aspect | Details | Complexity |
|--------|---------|------------|
| **What** | Match rules against user prompt text for intent-based routing | Medium |
| **Why Expected** | AI agent routing, intent classification, and policy engines universally match against user input text. See: [AI Agent Routing Guide](https://botpress.com/blog/ai-agent-routing), [Microsoft Intent-Based Routing](https://learn.microsoft.com/en-us/dynamics365/customer-service/administer/configure-intent-based-routing) | |
| **Standard Approach** | Regex pattern matching on prompt string field | |
| **Matchers Field** | `prompt_match: "regex pattern"` | |
| **Similar To** | Existing `command_match` matcher for Bash commands | |
| **Example Use Cases** | - Inject security context when user asks about production<br>- Route git operations based on user intent<br>- Warn when prompt mentions sensitive data | |
| **User Expectation** | If you can match tool commands, you should match user prompts | |

**YAML Example:**
```yaml
rules:
  - name: production-awareness
    matchers:
      prompt_match: "(production|prod|live).*deploy"
    actions:
      inject_inline: |
        ## Production Deployment Warning
        You mentioned deploying to production. Extra caution required.
```

**Precedent:**
- [AI routing systems](https://botpress.com/blog/ai-agent-routing) classify intent from user queries using LLMs or regex
- [Microsoft Dynamics 365](https://learn.microsoft.com/en-us/dynamics365/customer-service/administer/configure-intent-based-routing) routes based on intent extracted from user messages
- LLM orchestration frameworks use semantic routing to match user prompts to handlers

**Technical Notes:**
- Prompt text available in `UserPromptSubmit` event type
- Implementation similar to existing `command_match` regex matching
- Should support case-insensitive matching option
- Multi-line prompt support needed

---

### 2. require_fields: Required Field Validation

| Aspect | Details | Complexity |
|--------|---------|------------|
| **What** | Validate that tool inputs contain required fields before execution | Low |
| **Why Expected** | Every policy/validation system validates required fields. JSON Schema, Kubernetes admission webhooks, API gateways all provide this. See: [AWS API Gateway Validation](https://docs.aws.amazon.com/apigateway/latest/developerguide/api-gateway-method-request-validation.html), [Kubernetes Admission Webhooks](https://kubernetes.io/docs/reference/access-authn-authz/extensible-admission-controllers/) | |
| **Standard Approach** | List field names, block if any missing | |
| **Actions Field** | `require_fields: ["field1", "field2"]` | |
| **Failure Mode** | Block with error message listing missing fields | |
| **Example Use Cases** | - Require `file_path` in Write tool calls<br>- Require `command` in Bash tool calls<br>- Require security fields in git operations | |
| **User Expectation** | Basic schema validation - everyone has this | |

**YAML Example:**
```yaml
rules:
  - name: validate-write-inputs
    matchers:
      tools: ["Write"]
    actions:
      require_fields: ["file_path", "content"]
      # Blocks if either field missing
```

**Precedent:**
- [AWS API Gateway](https://docs.aws.amazon.com/apigateway/latest/developerguide/api-gateway-method-request-validation.html): Validates required properties in JSON Schema models
- [Kubernetes admission webhooks](https://kubernetes.io/docs/reference/access-authn-authz/extensible-admission-controllers/): Validate required fields exist before resource creation
- [JSON Schema](https://json-schema-everywhere.github.io/yaml): Standard `required` property for YAML validation
- [GitLab CI inputs](https://docs.gitlab.com/ci/inputs/): Type-checked inputs with built-in validation at pipeline creation time

**Technical Notes:**
- Field presence check only (value can be empty string, null)
- Check against `tool_input` JSON structure
- Support nested field paths with dot notation: `metadata.name`
- Error message format: `"Required fields missing: field1, field2"`
- Should run BEFORE other actions (early validation)

---

### 3. Inline Scripts: Embedded Validator Scripts

| Aspect | Details | Complexity |
|--------|---------|------------|
| **What** | Write validator scripts directly in YAML using multiline blocks | Medium |
| **Why Expected** | CI/CD systems, configuration tools, and policy engines all support inline scripts for single-file configs. See: [GitHub Actions inline scripts](https://docs.github.com/en/actions/creating-actions/metadata-syntax-for-github-actions), [Ansible inline Python](https://toptechtips.github.io/2023-06-10-ansible-python/), [Azure DevOps inline PowerShell](https://adamtheautomator.com/powershell-pipeline/) | |
| **Standard Approach** | YAML literal block (`|`) containing script code | |
| **Actions Field** | `run_inline: |` followed by script content | |
| **Language Support** | Any language supported by shebang line | |
| **Example Use Cases** | - Single-file hook configuration (no separate files)<br>- Quick validation logic without file management<br>- Shareable config snippets | |
| **User Expectation** | If `inject_inline` works, `run_inline` should too | |

**YAML Example:**
```yaml
rules:
  - name: validate-git-message
    matchers:
      tools: ["Bash"]
      command_match: "git commit"
    actions:
      run_inline: |
        #!/bin/bash
        # Check commit message follows conventional commits format
        commit_msg=$(git log -1 --pretty=%B)
        if ! [[ $commit_msg =~ ^(feat|fix|docs|chore): ]]; then
          echo "ERROR: Commit message must start with feat:, fix:, docs:, or chore:"
          exit 1
        fi
```

**Precedent:**
- [GitHub Actions](https://docs.github.com/en/actions/creating-actions/metadata-syntax-for-github-actions): Inline script validation with `run:` blocks
- [Azure DevOps](https://adamtheautomator.com/powershell-pipeline/): Inline PowerShell/Bash using pipe (`|`) symbol
- [Ansible](https://toptechtips.github.io/2023-06-10-ansible-python/): Inline Python scripts via script module
- [NGINX Lua](https://docs.nginx.com/nginx/admin-guide/dynamic-modules/lua/): Embedded Lua via `content_by_lua_block`
- [Traefik plugins](https://oneuptime.com/blog/post/2026-01-27-traefik-plugins/view): Inline validation logic in middleware configs

**Technical Notes:**
- Uses YAML literal block syntax (`|`) for multiline content
- Script written to temp file, executed like external script
- Same exit code semantics as existing `run` action
- Shebang line determines interpreter: `#!/bin/bash`, `#!/usr/bin/env python3`
- Trust level applies (default: `local`)
- No escaping complexity (unlike trying to inline in JSON)

---

## Differentiators

Features that set RuleZ apart. Not expected, but valuable for advanced use cases.

### 1. Composite Matching: prompt_match + command_match

**What:** Rules that match BOTH user prompt AND tool command.

**Value:** Enables sophisticated "user said X and is doing Y" policies that no other policy engine offers for LLM interactions.

**Example:**
```yaml
rules:
  - name: detect-forced-push-intent
    matchers:
      prompt_match: "(force|overwrite).*push"
      tools: ["Bash"]
      command_match: "git push.*--force"
    actions:
      inject_inline: |
        ## WARNING: Force Push Detected
        Your prompt mentioned forcing a push, and you're running git push --force.
        This will overwrite remote history. Are you sure?
```

**Complexity:** Low (composition of existing patterns)

**Why Valuable:** Catches misalignment between user intent and actual command. Unique to LLM-driven workflows.

---

### 2. Field Value Validation (Beyond Presence)

**What:** Validate field VALUES match patterns, not just presence.

**Example:**
```yaml
actions:
  require_fields:
    file_path: "^/src/.*\\.rs$"  # Must be Rust file in /src/
    content: ".{10,}"             # Must have at least 10 characters
```

**Value:** More expressive than simple presence checks.

**Complexity:** Medium (requires regex matching on values)

**Decision:** **Defer to v1.4**. Basic presence checks cover 90% of use cases. Value validation is nice-to-have.

---

### 3. Multi-Language Inline Scripts

**What:** Support multiple inline scripts in one rule (e.g., Python pre-check + Bash post-check).

**Example:**
```yaml
actions:
  run_inline_python: |
    # Python validation logic
  run_inline_bash: |
    # Bash cleanup logic
```

**Value:** Flexibility for complex validation flows.

**Complexity:** High (multiple script execution, ordering)

**Decision:** **Defer to v2.0**. Single inline script covers primary use case. Use separate rules if needed.

---

## Anti-Features

Features to explicitly NOT build. Common mistakes in policy engines.

### 1. ❌ Complex Expression Language in require_fields

**Temptation:** Support expressions like `require_fields: ["$env.CI == 'true' ? 'docker_image' : null"]`

**Why Avoid:**
- Complexity explosion (need full expression parser for field specs)
- Users will misuse it (hard to debug, unclear errors)
- `enabled_when` already handles conditional logic at rule level
- Precedent: AWS API Gateway, JSON Schema all use simple field lists

**Instead:** Use multiple rules with `enabled_when` to conditionally require fields:
```yaml
# Good: Clear, composable
- name: require-docker-in-ci
  enabled_when: 'env_CI == "true"'
  matchers:
    tools: ["Bash"]
  actions:
    require_fields: ["docker_image"]
```

---

### 2. ❌ Domain-Specific Prompt Classification

**Temptation:** Built-in intent classifiers like "is_security_question", "is_deployment_intent"

**Why Avoid:**
- Domain assumptions (what's "security" varies by org)
- Maintenance burden (need to update classifications)
- Inflexible (users can't customize)
- Precedent: AI routing systems let users define their own routes/intents

**Instead:** Provide regex matching (`prompt_match`). Users define their own intent patterns:
```yaml
# Good: User-defined patterns
- name: security-questions
  matchers:
    prompt_match: "(security|auth|password|secret|credential)"
```

---

### 3. ❌ Inline Script Auto-Detection (No Shebang)

**Temptation:** Detect script language from syntax (e.g., `if __name__ == "__main__":` → Python)

**Why Avoid:**
- Fragile heuristics (ambiguous syntax across languages)
- Surprising behavior (user expects bash, gets python)
- Precedent: Shell scripts, CI/CD systems all require explicit shebang
- Security risk (wrong interpreter → unexpected execution)

**Instead:** REQUIRE shebang line in inline scripts:
```yaml
# Good: Explicit interpreter
run_inline: |
  #!/usr/bin/env python3
  print("I am Python")
```

If shebang missing, default to `/bin/bash` with warning log.

---

### 4. ❌ require_fields with Type Coercion

**Temptation:** Auto-convert types: `require_fields: [{name: "count", type: "integer"}]`

**Why Avoid:**
- Adds type system complexity (JSON already typed)
- Coercion surprises (`"123"` becomes `123`?)
- Validation vs transformation confusion (policy should validate, not mutate)
- Precedent: Kubernetes admission webhooks separate mutation from validation

**Instead:** Check presence only. Use inline script for type validation if needed:
```yaml
# Good: Separation of concerns
actions:
  require_fields: ["count"]
  run_inline: |
    #!/bin/bash
    if ! [[ $count =~ ^[0-9]+$ ]]; then
      echo "count must be integer"
      exit 1
    fi
```

---

## Feature Dependencies

```
User Prompt
    ↓
[prompt_match matcher]  ← Standalone feature (Phase 4)
    ↓
[Tool Input JSON]
    ↓
[require_fields action] ← Validates fields exist (Phase 5)
    ↓
[run_inline action]     ← Custom validation logic (Phase 6)
    ↓
Tool Execution
```

**No hard dependencies** - Each feature is independently useful:
- `prompt_match` works without field validation
- `require_fields` works without prompt matching
- `run_inline` works without either

**Soft synergy:**
- `prompt_match` + `require_fields`: "When user asks about X, require field Y"
- `require_fields` + `run_inline`: Basic validation (fields exist) + advanced validation (field values correct)

---

## MVP Recommendation

**For v1.3 MVP, implement all three features:**

1. **prompt_match** (Phase 4) - Table stakes, users expect this
2. **require_fields** (Phase 5) - Table stakes, simple implementation
3. **run_inline** (Phase 6) - Table stakes, follows `inject_inline` pattern

**Rationale:**
- All three are table stakes features users expect
- Low to medium complexity (similar to existing features)
- High value (completes the policy engine story)
- No interdependencies (can ship independently if needed)

**Defer to post-v1.3:**
- Field value validation (regex on values) - v1.4
- Multi-language inline scripts - v2.0
- Composite matching UI helpers - v2.0

---

## Feature Comparison Matrix

Comparing RuleZ v1.3 features to established policy engines:

| Feature | RuleZ v1.3 | OPA/Rego | K8s Admission | AWS API Gateway | GitHub Actions |
|---------|-----------|----------|---------------|-----------------|----------------|
| **Prompt/Intent Matching** | ✅ prompt_match | ✅ Input queries | ❌ N/A | ❌ N/A | ❌ N/A |
| **Required Field Validation** | ✅ require_fields | ✅ Field checks | ✅ Schema validation | ✅ JSON Schema required | ✅ inputs.required |
| **Inline Scripts** | ✅ run_inline | ✅ Rego policies | ✅ Webhook code | ❌ Separate Lambda | ✅ run: blocks |
| **Regex Matching** | ✅ Yes | ✅ Yes | ✅ Yes | ✅ Yes | ✅ Yes |
| **Multi-line YAML** | ✅ Yes | ✅ Yes | ✅ Yes | ✅ Yes | ✅ Yes |

**Unique to RuleZ:**
- Prompt + command composite matching (no other system operates at LLM prompt level)
- Single YAML file for all policies (simpler than OPA/Rego modules)
- Directly integrated with Claude Code tool hooks (not external webhook)

---

## Expected Behavior Specifications

### prompt_match Behavior

**Input:** User prompt text from `UserPromptSubmit` event
**Matching:** Case-insensitive regex by default (configurable)
**Multi-line:** Support prompts spanning multiple lines
**Performance:** Compile regex once at config load time

**Example event:**
```json
{
  "hook_event_name": "UserPromptSubmit",
  "tool_input": {
    "prompt": "Deploy the app to production"
  }
}
```

**Matching logic:**
```rust
if let Some(prompt_pattern) = &matchers.prompt_match {
    let regex = Regex::new(prompt_pattern)?;
    let prompt_text = event.tool_input
        .get("prompt")
        .and_then(|p| p.as_str())
        .unwrap_or("");
    if !regex.is_match(prompt_text) {
        return Ok(false); // Rule doesn't match
    }
}
```

---

### require_fields Behavior

**Input:** Tool input JSON from event
**Validation:** Check field exists (not null, not undefined)
**Failure:** Block with descriptive error message
**Nested fields:** Support dot notation: `metadata.name`

**Example validation:**
```yaml
actions:
  require_fields: ["file_path", "content", "metadata.author"]
```

**Validation logic:**
```rust
if let Some(required) = &actions.require_fields {
    let missing = required.iter()
        .filter(|field| !field_exists(&event.tool_input, field))
        .collect::<Vec<_>>();

    if !missing.is_empty() {
        return Ok(Response::block(format!(
            "Required fields missing: {}",
            missing.join(", ")
        )));
    }
}
```

**Error message format:**
```json
{
  "continue": false,
  "reason": "Required fields missing: file_path, metadata.author"
}
```

---

### run_inline Behavior

**Input:** YAML literal block containing script
**Execution:** Write to temp file, execute with same semantics as `run` action
**Shebang:** Required (defaults to `/bin/bash` if missing with warning)
**Exit codes:** Same as external scripts (0=allow, non-zero=block)
**Trust level:** Defaults to `local` (consistent with external scripts)

**Example:**
```yaml
actions:
  run_inline: |
    #!/bin/bash
    if [[ $command =~ --force ]]; then
      echo "Force flag detected"
      exit 1
    fi
```

**Execution logic:**
```rust
if let Some(script_content) = &actions.run_inline {
    // Write to temp file
    let temp_file = write_temp_script(script_content)?;

    // Execute like external script
    let result = execute_script(&temp_file, &event)?;

    // Cleanup temp file
    std::fs::remove_file(temp_file)?;

    // Check exit code
    if !result.success() {
        return Ok(Response::block(result.stderr));
    }
}
```

---

## Complexity Assessment

| Feature | Implementation | Testing | Documentation | Total |
|---------|----------------|---------|---------------|-------|
| **prompt_match** | Low (regex matcher) | Low (unit tests) | Low (similar to command_match) | **Low** |
| **require_fields** | Low (JSON field check) | Medium (edge cases) | Low (straightforward) | **Low-Medium** |
| **run_inline** | Medium (temp file handling) | Medium (multi-language) | Medium (security notes) | **Medium** |

**Overall v1.3 Complexity:** Medium

**Risk Factors:**
- Inline scripts: Temp file cleanup, shebang parsing
- require_fields: Nested field path handling
- prompt_match: Multi-line prompt edge cases

**Mitigation:**
- Thorough unit tests for edge cases
- Clear error messages
- Documentation with examples

---

## Sources

### AI Agent Routing & Intent Classification
- [AI Agent Routing: Ultimate Guide (2026)](https://botpress.com/blog/ai-agent-routing)
- [AI Agent Routing Tutorial & Best Practices](https://www.patronus.ai/ai-agent-development/ai-agent-routing)
- [Microsoft Intent-Based Routing](https://learn.microsoft.com/en-us/dynamics365/customer-service/administer/configure-intent-based-routing)
- [Intent-Driven Natural Language Interface](https://medium.com/data-science-collective/intent-driven-natural-language-interface-a-hybrid-llm-intent-classification-approach-e1d96ad6f35d)
- [LLM Orchestration in 2026: Top Frameworks](https://research.aimultiple.com/llm-orchestration/)

### YAML Schema Validation & Required Fields
- [Yamale: Schema and Validator for YAML](https://github.com/23andMe/Yamale)
- [Schema Validation for YAML | JSON Schema Everywhere](https://json-schema-everywhere.github.io/yaml)
- [YAML Data Validation - Infrastructure as Code](https://infrastructureascode.ch/yaml_validation.html)
- [How to use YAML Schema to validate your YAML files](https://blog.picnic.nl/how-to-use-yaml-schema-to-validate-your-yaml-files-c82c049c2097)

### Inline Script Blocks
- [YAML Tutorial: Everything You Need to Get Started](https://www.cloudbees.com/blog/yaml-tutorial-everything-you-need-get-started)
- [Custom PowerShell Pipelines: Integrating with Azure DevOps](https://adamtheautomator.com/powershell-pipeline/)
- [YAML Syntax — Ansible Community Documentation](https://docs.ansible.com/projects/ansible/latest/reference_appendices/YAMLSyntax.html)

### Policy as Code & Validation Engines
- [Open Policy Agent (OPA) Documentation](https://www.openpolicyagent.org/docs)
- [Enabling Policy as Code with OPA and Rego](https://snyk.io/blog/opa-rego-usage-for-policy-as-code/)
- [Top 12 Policy as Code Tools in 2026](https://spacelift.io/blog/policy-as-code-tools)
- [How to Write Your First Rules in Rego](https://www.styra.com/blog/how-to-write-your-first-rules-in-rego-the-policy-language-for-opa/)

### Kubernetes Admission Control
- [Dynamic Admission Control | Kubernetes](https://kubernetes.io/docs/reference/access-authn-authz/extensible-admission-controllers/)
- [Kubernetes Admission Controllers and Webhooks Deep Dive](https://www.chkk.io/blog/kubernetes-admission-controllers)
- [How to Implement Kubernetes Admission Webhooks (2026)](https://oneuptime.com/blog/post/2026-01-30-kubernetes-admission-webhooks/view)
- [Kubernetes Validating Admission Policy: Native Alternative to Webhooks](https://medium.com/@chetanatole99/a-deep-dive-into-kubernetes-validating-admission-policy-the-native-alternative-to-webhooks-b35df05e6a5b)

### API Gateway Validation
- [Request Validation for REST APIs in API Gateway](https://docs.aws.amazon.com/apigateway/latest/developerguide/api-gateway-method-request-validation.html)
- [Set up Basic Request Validation in API Gateway](https://docs.aws.amazon.com/apigateway/latest/developerguide/api-gateway-request-validation-set-up.html)
- [JSON Schema Validation | KrakenD API Gateway](https://www.krakend.io/docs/endpoints/json-schema/)
- [Request Validation with API Gateway Models](https://www.sls.guru/blog/request-validation-with-api-gateway-models)

### CI/CD Inline Validation
- [GitHub Actions: Metadata Syntax](https://docs.github.com/en/actions/creating-actions/metadata-syntax-for-github-actions)
- [GitHub Actions: Smarter Editing, Clearer Debugging (2026)](https://github.blog/changelog/2026-01-29-github-actions-smarter-editing-clearer-debugging-and-a-new-case-function/)
- [GitLab CI/CD Inputs Documentation](https://docs.gitlab.com/ci/inputs/)
- [GitLab: CI/CD Inputs Secure Method to Pass Parameters](https://about.gitlab.com/blog/ci-cd-inputs-secure-and-preferred-method-to-pass-parameters-to-a-pipeline/)
- [Working with Python Scripts in Ansible](https://toptechtips.github.io/2023-06-10-ansible-python/)

### NGINX & Middleware Inline Scripts
- [Lua | NGINX Documentation](https://docs.nginx.com/nginx/admin-guide/dynamic-modules/lua/)
- [Lua NGINX Module Rocky Linux 10: Scripting Guide (2026)](https://www.getpagespeed.com/server-setup/nginx/lua-nginx-module-rocky-linux-10)
- [How to Use Traefik Plugins (2026)](https://oneuptime.com/blog/post/2026-01-27-traefik-plugins/view)
- [Traefik Proxy Middleware Overview](https://doc.traefik.io/traefik/middlewares/overview/)
