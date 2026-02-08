# Project Research Summary

**Project:** RuleZ v1.3 Advanced Matching & Validation
**Domain:** AI policy engine with sub-10ms performance requirements
**Researched:** 2026-02-08
**Confidence:** HIGH

## Executive Summary

RuleZ v1.3 extends the policy engine with three new capabilities: prompt matching for intent-based routing, field validation for required parameter enforcement, and inline script blocks for single-file configurations. Research shows these are **table stakes features** users expect from policy engines based on patterns from OPA, Kubernetes admission webhooks, CI/CD systems (GitHub Actions, GitLab CI), and API gateways (AWS API Gateway, Kong).

The recommended approach is **minimalist and performance-focused**: extend existing evalexpr for inline validation (zero new dependencies), reuse regex crate for prompt matching (zero new dependencies), and add jsonschema 0.41 for field validation (single new dependency). This maintains RuleZ's sub-10ms processing budget while adding powerful capabilities. Total overhead: +200KB binary size, +2-3s compile time, <5ms worst-case runtime.

Critical risks center on **security and performance**: catastrophic regex backtracking can turn 3ms processing into 3000ms DoS, inline scripts without sandboxing pose RCE threats, and nested JSON validation can exceed the 10ms budget. Mitigations are well-documented: validate regex patterns at config load, defer unsandboxed inline scripts to v1.4+, and limit field depth to 5 levels. Build order recommendation: Phase 4 (prompt_match), Phase 5 (require_fields), Phase 6 (inline scripts with sandboxing).

## Key Findings

### Recommended Stack

**Stack decision: Add ONE dependency, extend TWO existing.**

For v1.3's three capabilities:
1. **Inline scripting** - Extend evalexpr 13.1 with custom functions (NO new dependency)
2. **Prompt matching** - Use existing regex crate (NO new dependency)
3. **Field validation** - Add jsonschema 0.41 for JSON Schema validation (NEW)

**Why this approach:**
- **Performance preserved:** Evalexpr already proven <1ms, regex <100μs, jsonschema 2-52x faster than alternatives
- **Zero bloat:** No heavyweight scripting engines (Rhai +500KB, mlua unsafe FFI), no NLP libraries (rust-bert 50MB+)
- **Battle-tested:** jsonschema 0.41 released Feb 2025, evalexpr 13.1 validated in RuleZ v1.2 with 245 tests

**Core technologies:**
- **evalexpr 13.1** (existing): Custom function API for inline validators - proven <1ms, zero compilation overhead
- **regex** (existing): Pattern matching for prompts - reuse command_match infrastructure, <100μs per match
- **jsonschema 0.41** (NEW): JSON Schema validation - 2-52x faster than alternatives, standards-compliant (Draft 7, 2019-09, 2020-12)

**Rejected alternatives:**
- Rhai 1.24 scripting: +500KB binary, +5-10s compile time, 7 dependencies, violates sub-10ms requirement
- mlua (Lua): Unsafe FFI, external C library, state management complexity across async boundaries
- rust-bert NLP: 50MB models, GPU required, 100ms+ latency - completely breaks sub-10ms budget
- fancy-regex: Backreference support not needed, slower than standard regex

### Expected Features

**All three features are table stakes** - users expect these based on established policy engine patterns.

**Must have (table stakes):**
- **prompt_match** — Intent-based routing standard in AI agent systems (Botpress, Microsoft Dynamics 365, LLM orchestration frameworks)
- **require_fields** — Basic schema validation present in AWS API Gateway, Kubernetes admission webhooks, GitLab CI, JSON Schema
- **Inline scripts** — Single-file configs standard in GitHub Actions, Azure DevOps, Ansible, NGINX Lua, Traefik plugins

**Competitive edge (unique to RuleZ):**
- **Composite matching** — prompt_match + command_match enables "user said X and is doing Y" policies unique to LLM workflows
- **Single YAML file** — Simpler than OPA/Rego modules, directly integrated with Claude Code (not external webhook)

**Defer (v2+):**
- **Field value validation** (v1.4) — Regex patterns on field values beyond presence checks - nice-to-have, 90% covered by presence
- **Multi-language inline scripts** (v2.0) — Multiple scripts per rule adds complexity, use separate rules instead
- **Semantic prompt matching** (v2.0) — Embedding-based intent detection - regex sufficient for v1.3

**Anti-features (explicitly avoid):**
- Complex expression language in require_fields (use enabled_when for conditionals)
- Domain-specific prompt classifiers (users define own patterns)
- Auto-detection of script language without shebang (fragile heuristics, security risk)
- Type coercion in field validation (separation of concerns - validate, don't mutate)

### Architecture Approach

**All three features integrate as additive extensions to existing pipeline** - no structural changes required.

v1.3 extends two pipelines:
1. **Matchers pipeline** (models.rs::Matchers) - Add prompt_match field, follows exact command_match pattern
2. **Actions pipeline** (models.rs::Actions) - Add require_fields validation (executes first), extend RunAction enum for inline scripts

**Major components:**
1. **prompt_match matcher** (Phase 4) - Regex evaluation in hooks.rs::matches_rule(), reuses existing regex compilation, <100μs overhead
2. **require_fields action** (Phase 5) - Field validation before all other actions in execute_rule_actions(), simple HashMap lookup, <0.1ms overhead
3. **Inline script execution** (Phase 6) - New execute_inline_script() function, temp file management, same semantics as file-based scripts, ~1-5ms temp file overhead

**Architecture patterns validated:**
- **Matcher extensibility** — Add field to Matchers, extend matches_rule() with AND logic (proven by command_match)
- **Action extensibility** — Add field to Actions, respect execution order (proven by inject_inline, inject_command v1.2)
- **RunAction enum extensibility** — Add variant with #[serde(untagged)] (proven by Simple vs Extended variants Phase 2.4)
- **Event flexibility** — tool_input: Option<serde_json::Value> supports arbitrary fields (no schema enforcement)

**Integration points:**
- prompt_match: Add prompt: Option<String> to HookEvent, compile regex at config load (same as command_match)
- require_fields: Validate before block checks (line 484 in execute_rule_actions), fail-closed on missing fields
- Inline scripts: Detect variant via RunAction::is_inline(), write to temp file with UUID, execute via interpreter from shebang

### Critical Pitfalls

Research identified 9 pitfalls across 3 severity levels. Top 3 critical:

1. **Catastrophic regex backtracking (Phase 4)** — Nested quantifiers like (a+)+ cause O(2^n) complexity, turning 3ms into 3000ms DoS. **Prevention:** Validate patterns for nested quantifiers at config load, use Rust regex crate (DFA-based, safe by default), add 1ms timeout via RegexBuilder::size_limit(). Document that fancy-regex is forbidden.

2. **Script execution without sandboxing (Phase 6)** — Inline scripts execute with full process permissions, enabling RCE, credential theft, data exfiltration. **Prevention:** DEFER unsandboxed inline scripts to v1.4 OR implement seccomp + Landlock (Linux) or Microsoft LiteBox (2026 release). Strip environment variables, read-only filesystem except /tmp, 1-second CPU limit (not wall-clock). Require shebang validation, hash script content to prevent TOCTOU.

3. **Nested JSON validation overhead (Phase 5)** — Deeply nested fields (100+ levels) cause quadratic parsing and memory exhaustion. 7-level nesting: ~1ms, 100-level: >10ms (exceeds budget). **Prevention:** Limit field paths to 5 levels max, pre-parse paths at config load (cache as Vec<&str>), validate JSON depth on event receipt, benchmark with criterion (target <0.5ms for 5 levels).

**Moderate risks:**
- **CPU timeout insufficient (Phase 6)** — Wall-clock timeout doesn't stop CPU-bound loops. Add setrlimit(RLIMIT_CPU) for 1 CPU second limit.
- **Prompt injection bypass (Phase 4)** — Base64, spacing, Unicode tricks bypass pattern matching. Normalize input (lowercase, alphanumeric filter), decode common encodings.
- **Schema parsing overhead (Phase 5)** — Parsing schema per event scales poorly. Pre-compile validators at config load with OnceCell, use jsonschema validator reuse.

**Minor risks (refactoring opportunities):**
- Regex compilation in hot path (known issue with clippy suppression in lib.rs:23, config.rs:1) - fix in Phase 4
- Missing shebang validation for scripts - add whitelist of allowed interpreters
- Fail-open for typo'd field paths - define fail-closed semantics with strict_validation flag

## Implications for Roadmap

Based on research, suggested phase structure follows **simplest-to-most-complex** build order with **dependency isolation**.

### Phase 4: prompt_match (Matcher)

**Rationale:** Cleanest integration (mirrors command_match exactly), validates Matchers pipeline extensibility, no new execution logic (just matcher evaluation).

**Delivers:**
- Intent-based rule routing from UserPromptSubmit events
- Regex patterns for prompt text matching (case-insensitive by default)
- Composite matching capability (prompt + command)

**Addresses features:**
- prompt_match (table stakes from FEATURES.md)
- Composite matching differentiator

**Avoids pitfalls:**
- MUST validate regex patterns for catastrophic backtracking (Pitfall 1)
- MUST add input normalization layer (Pitfall 5)
- SHOULD pre-compile regexes at config load (Pitfall 7 refactor opportunity)

**Implementation complexity:** Low (~30-40 LOC + tests)
**Research needed:** NO (standard pattern, well-documented)

---

### Phase 5: require_fields (Action)

**Rationale:** Extends Actions pipeline (validates extensibility), no external dependencies (pure validation), tests action execution order (must execute before blocks).

**Delivers:**
- Required field validation for tool_input JSON
- Fail-closed blocking with helpful error messages
- Support for nested field paths with dot notation (up to 5 levels)

**Addresses features:**
- require_fields (table stakes from FEATURES.md)

**Avoids pitfalls:**
- MUST implement 5-level depth limit (Pitfall 3)
- MUST pre-compile field paths at config load (Pitfall 6)
- MUST define fail-closed semantics for missing fields (Pitfall 9)

**Uses stack:**
- jsonschema 0.41 for JSON Schema validation (add to Cargo.toml)
- Existing serde_json::Value for field access

**Implementation complexity:** Low (~50-60 LOC + validation + tests)
**Research needed:** NO (JSON Schema well-documented, jsonschema crate battle-tested)

---

### Phase 6: Inline Script Blocks (Action)

**Rationale:** Most complex (temp file management, interpreter detection, sandboxing), builds on existing script execution infrastructure, validates enum extensibility.

**Delivers:**
- Inline script syntax in YAML using literal blocks (|)
- Shebang-based interpreter detection (Python, Bash, Node)
- Same exit code semantics as file-based scripts

**Addresses features:**
- Inline scripts (table stakes from FEATURES.md)

**Avoids pitfalls:**
- **CRITICAL DECISION POINT:** Defer to v1.4 OR implement sandboxing first
- If shipped in v1.3:
  - MUST implement seccomp + Landlock sandboxing (Linux only)
  - MUST add CPU time limits via setrlimit(RLIMIT_CPU) (Pitfall 4)
  - MUST validate shebangs and whitelist interpreters (Pitfall 8)
  - MUST strip environment variables from script context (Pitfall 2)
  - MUST ensure temp file cleanup even on error

**Uses stack:**
- Extend evalexpr 13.1 with custom functions (alternative to full script execution)
- Existing tokio::fs for temp file I/O
- Existing uuid crate for temp file naming

**Implementation complexity:** Medium-High (~150-200 LOC + extensive tests + security validation)
**Research needed:** YES - Script sandboxing approaches for macOS (LiteBox is Linux-only)

**Alternative approach (RECOMMENDED):**
- Ship external script files only in v1.3 (safer, easier to audit)
- Defer inline scripts to v1.4 with proper cross-platform sandboxing
- OR: Ship inline validation via evalexpr custom functions (no separate script execution)

---

### Phase Ordering Rationale

**Why this order:**
1. **Dependency isolation** - Each phase is independently useful, no hard dependencies between features
2. **Complexity progression** - Start with simplest (prompt_match: ~40 LOC) to validate integration patterns, end with most complex (inline scripts: ~200 LOC + security)
3. **Risk management** - Address performance risks (Phases 4-5) before security risks (Phase 6)
4. **Incremental value** - Ship prompt_match + require_fields without inline scripts if sandboxing proves complex

**Soft synergies (not dependencies):**
- prompt_match + require_fields: "When user asks about X, require field Y"
- require_fields + inline scripts: Basic validation (presence) + advanced validation (value correctness)

### Research Flags

**Phases needing deeper research during planning:**
- **Phase 6 (Inline Scripts):** REQUIRES additional research on cross-platform sandboxing (LiteBox Linux-only, macOS App Sandbox limited). Alternatives: defer to v1.4, use evalexpr custom functions instead of full script execution, or Linux-only release.

**Phases with standard patterns (skip research-phase):**
- **Phase 4 (prompt_match):** Well-documented regex patterns, existing RuleZ command_match precedent
- **Phase 5 (require_fields):** JSON Schema industry standard, jsonschema crate documentation comprehensive

## Confidence Assessment

| Area | Confidence | Notes |
|------|------------|-------|
| Stack | HIGH | Evalexpr proven in v1.2 (245 tests), jsonschema 0.41 released Feb 2025 with benchmarks, regex already in use |
| Features | HIGH | Table stakes validated across OPA, Kubernetes, GitHub Actions, AWS API Gateway - multiple independent sources |
| Architecture | HIGH | Integration patterns proven by recent v1.2 additions (inject_inline, inject_command, priority sorting) |
| Pitfalls | HIGH | Catastrophic backtracking documented in regex literature, sandboxing validated by 2026 LiteBox release, nested JSON validated by jsonschema benchmarks |

**Overall confidence:** HIGH

### Gaps to Address

**During implementation:**
- **Performance validation required:** Benchmark all three features together to confirm <10ms requirement (criterion suite)
- **macOS sandboxing:** Research macOS-specific sandboxing options if inline scripts must ship in v1.3 (App Sandbox entitlements, defer to Linux-only, or use evalexpr custom functions)
- **Fail-closed semantics:** Define strict_validation config flag for field path typos (security vs UX tradeoff)

**During testing:**
- **Fuzz testing:** Malicious regex patterns, deeply nested JSON, prompt injection attempts
- **Edge cases:** Multi-line prompts, Unicode in field paths, temp file cleanup on panic/timeout
- **Integration:** Priority + mode combinations (warn mode, audit mode, multiple rules with prompt_match)

**Post-v1.3 (defer to v1.4+):**
- Semantic prompt matching (embedding-based, requires ML model integration)
- Field value validation beyond presence checks (regex on values)
- serde_json_path integration for complex JSONPath queries

## Sources

### Primary (HIGH confidence)
- [evalexpr 13.1 documentation](https://docs.rs/evalexpr) — Custom function API, performance
- [Rust regex documentation](https://docs.rs/regex/latest/regex/) — Pattern matching capabilities, safety guarantees
- [jsonschema-rs GitHub](https://github.com/Stranger6667/jsonschema) — Performance benchmarks (2-52x faster), 0.41 release notes
- [Catastrophic Backtracking - Regular-Expressions.info](https://www.regular-expressions.info/catastrophic.html) — Regex complexity analysis
- [Microsoft LiteBox - Rust Sandboxing (2026)](https://securityboulevard.com/2026/02/microsoft-unveils-litebox-a-rust-based-approach-to-secure-sandboxing/) — Script sandboxing approach
- [Rust Sandboxing with seccomp and Landlock (2026)](https://oneuptime.com/blog/post/2026-01-07-rust-sandboxing-seccomp-landlock/view) — Linux sandboxing implementation

### Secondary (MEDIUM confidence)
- [AI Agent Routing: Ultimate Guide (2026)](https://botpress.com/blog/ai-agent-routing) — Intent-based routing patterns
- [AWS API Gateway Request Validation](https://docs.aws.amazon.com/apigateway/latest/developerguide/api-gateway-method-request-validation.html) — Required field validation precedent
- [Kubernetes Admission Control](https://kubernetes.io/docs/reference/access-authn-authz/extensible-admission-controllers/) — Policy validation patterns
- [GitHub Actions Metadata Syntax](https://docs.github.com/en/actions/creating-actions/metadata-syntax-for-github-actions) — Inline script precedent
- [JSON Interoperability Vulnerabilities - Bishop Fox](https://bishopfox.com/blog/json-interoperability-vulnerabilities) — Nested JSON risks
- [Perplexity BrowseSafe Prompt Injection](https://bdtechtalks.com/2026/01/19/perplexity-browsesafe-prompt-injection/) — Prompt matching bypass techniques

### Tertiary (LOW confidence - not relied upon)
- Various blog posts on Rust scripting (LogRocket, medium.com) — Ecosystem overviews
- WebSearch results on NLP libraries — Discovery only, not used for decisions

---
*Research completed: 2026-02-08*
*Ready for roadmap: yes*
*Critical decision: Phase 6 inline scripts - defer to v1.4 or implement sandboxing first*
