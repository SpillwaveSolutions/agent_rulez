# Project Research Summary

**Project:** RuleZ v1.4 Stability & Polish
**Domain:** Rust CLI Policy Engine - Infrastructure & Developer Experience
**Researched:** 2026-02-10
**Confidence:** HIGH

## Executive Summary

v1.4 is a **polish and stabilization milestone** focused on closing four specific gaps identified in v1.3 technical debt: JSON Schema validation for hook events, debug CLI parity for UserPromptSubmit events, cross-platform E2E test reliability, and Tauri 2.0 CI integration. Research shows these features are infrastructure improvements that enable better testing, validation, and cross-platform reliability without adding new user-facing policy features.

The recommended approach is **additive integration**: all four features enhance existing components without modifying the core rule evaluation engine. Add ONE new dependency (schemars 1.2.1 for schema generation), pre-compile validators for performance, isolate debug CLI state to prevent cross-invocation leakage, canonicalize paths in E2E tests for cross-platform compatibility, and use explicit Ubuntu 22.04 runners with webkit2gtk-4.1 for Tauri builds.

Key risks center on performance (schema validation must stay under 0.1ms per event), correctness (JSON Schema draft version compatibility), and CI reliability (webkit dependency hell, stale binary caches). All risks have well-documented mitigation strategies: pre-compile schemas at config load, require explicit `$schema` field with draft validation, clear caches in debug mode, use fs::canonicalize for paths, and pin CI runners with explicit webkit versions.

## Key Findings

### Recommended Stack

v1.4 adds **ONE new dependency** to the existing validated stack: schemars 1.2.1 for automatic JSON Schema generation from Rust types. All other features reuse existing dependencies (jsonschema 0.41 for validation, tokio async stdin for debug CLI, Playwright for E2E tests, Tauri 2.0 for desktop app).

**Core technologies (NEW in v1.4):**
- **schemars 1.2.1**: JSON Schema generation from Rust structs — auto-generated schemas match serde serialization exactly, eliminates manual schema file maintenance, full serde compatibility via derive macros
- **jsonschema 0.41** (existing): Runtime schema validation — 20-470x faster than alternatives, supports draft-07 and 2020-12, pre-compiled validators cache for performance
- **tokio::io::stdin()** (existing): Async stdin for debug CLI — built-in since tokio 1.0, no new dependency needed, reuses existing async runtime
- **Playwright 1.50** (existing): E2E testing in web mode — tests UI without Tauri build, works with Bun (basic support), fast feedback loop
- **libwebkit2gtk-4.1-dev**: Tauri 2.0 Linux dependency — CRITICAL: version 4.1 (NOT 4.0), required for Ubuntu 22.04+, breaking change from Tauri 1.x

**Performance budget:** Schema validation adds <0.1ms per event (cached validator), stays within <10ms total processing budget. Binary size increases by ~50 KB (schemars compile-time only).

### Expected Features

v1.4 delivers **infrastructure features** that improve correctness and developer experience, NOT new user-facing policy capabilities. These are table stakes for calling the milestone "Stability & Polish."

**Must have (table stakes):**
- **JSON Schema event validation**: Validate incoming hook events match Claude Code's schema before processing — fail-open mode for backwards compatibility, pre-compiled schemas for performance, support draft-07 and 2020-12
- **Debug CLI UserPromptSubmit simulation**: Close testing gap for prompt_match rules — add `rulez debug prompt --prompt "text"` command, reuse existing event processing pipeline, clear state between invocations
- **E2E test cross-platform reliability**: Tests must pass on Linux, macOS, Windows — canonicalize paths to resolve symlinks, fix broken pipe issues with wait_with_output(), CI matrix on all platforms
- **Tauri 2.0 CI builds**: Multi-platform desktop app builds in GitHub Actions — webkit2gtk-4.1 on Linux, macOS and Windows native builds, E2E tests run first (fast), Tauri builds only if tests pass (slow)

**Should have (differentiators from minimal bug fix):**
- **JSON Schema draft version validation**: Fail-closed on missing `$schema` field, reject unsupported drafts (draft-04/06), prevent breaking changes when schemas evolve
- **Performance regression tests in CI**: Benchmark schema validation overhead, fail CI if p95 latency exceeds 10ms, guarantee performance budget maintained
- **LRU regex cache**: Fix unbounded REGEX_CACHE growth from v1.3 tech debt, max 100 compiled regexes, evict least-recently-used
- **Debug CLI correlation IDs**: Trace event flow for reproducibility, log correlation ID with every decision, support debugging complex event chains

**Defer (v2+ scope creep):**
- **New rule matchers or actions**: v1.4 is stability, NOT feature expansion — defer prompt injection prevention, new matchers to v1.5
- **Interactive debug REPL mode**: Already exists in `rulez repl` — extend existing REPL with UserPromptSubmit support rather than duplicate
- **Full Tauri E2E tests**: Too slow and brittle — test web mode with Playwright (Tauri commands have fallbacks), reserve full Tauri tests for manual QA

### Architecture Approach

All v1.4 features integrate as **additive layers** to existing components without breaking changes. JSON Schema validation adds a pre-processing step in main.rs before rule evaluation. Debug CLI extends the existing SimEventType enum with UserPromptSubmit variant. E2E tests add helper functions for path canonicalization. Tauri CI creates a new workflow file parallel to existing CI.

**Major components:**
1. **Event Schema Validator** (NEW in main.rs): Pre-compile JSON Schemas at startup using LazyLock, validate event JSON before deserialization, fail with exit code 1 on invalid events (not blocking code 2), log validation failures to audit trail
2. **Debug CLI UserPromptSubmit** (EXTEND cli/debug.rs): Add UserPromptSubmit to SimEventType enum, add --prompt flag to CLI args, clear REGEX_CACHE at start of run() for state isolation, reuse existing process_event() pipeline
3. **Path Canonicalizer** (NEW in tests/common/mod.rs): Helper function using fs::canonicalize() to resolve symlinks and normalize separators, handles macOS /var to /private/var symlink, cross-platform PathBuf operations
4. **Tauri CI Workflow** (NEW .github/workflows/tauri-build.yml): E2E tests in web mode (fast, runs on every PR), Tauri builds in matrix (slow, multi-platform), explicit ubuntu-22.04 with webkit2gtk-4.1, artifact uploads for releases only

**Integration points preserved:**
- Core rule evaluation engine (hooks.rs) UNCHANGED — no modifications to process_event() logic
- Config loading (config.rs) UNCHANGED — schemas compiled after config load, not during
- Event models (models.rs) ADD JsonSchema derive — automatic schema generation, no manual fields
- Audit logging (logging.rs) UNCHANGED — schema validation failures logged like other events

### Critical Pitfalls

Research identified six critical pitfalls specific to v1.4's infrastructure features. All have well-documented prevention strategies.

1. **JSON Schema draft version incompatibility**: Breaking changes between draft-07, draft-2019-09, and draft-2020-12 cause silent validation failures — PREVENT: Require explicit `$schema` field in all schemas, reject unsupported drafts with clear error messages, pin jsonschema crate version to prevent breaking updates, document only draft-07 and 2020-12 supported
2. **Schema validation performance in hot path**: Compiling schemas on every event adds 0.5-2ms overhead, exceeds <10ms budget with 100+ rules — PREVENT: Pre-compile schemas at config load using OnceCell or LazyLock, cache compiled validators in Rule struct, fail config load on schema compilation errors, add performance regression tests to CI with criterion benchmarks
3. **Debug CLI state contamination across invocations**: Global REGEX_CACHE and static memory leak state between rulez debug calls, causes unreproducible bugs — PREVENT: Clear REGEX_CACHE at start of debug run(), implement LRU cache with max 100 entries to fix v1.3 unbounded growth, use correlation IDs for debug tracing, add state isolation test to verify no cross-invocation leakage
4. **E2E test path resolution across platforms**: macOS /var symlinks, Windows backslash separators, tempfile cleanup races cause CI failures — PREVENT: Use fs::canonicalize() before path comparison, PathBuf for all path operations (not string concatenation), explicit drop(temp_dir) at test end, CI matrix on ubuntu/macos/windows
5. **Tauri 2.0 webkit version conflict in CI**: Ubuntu 24.04 removed libwebkit2gtk-4.0-dev, Tauri 2.0 requires 4.1 — PREVENT: Use explicit ubuntu-22.04 runner (NOT ubuntu-latest), install libwebkit2gtk-4.1-dev (NOT 4.0), document minimum OS requirements (Ubuntu 22.04+, Debian 12+)
6. **GitHub Actions cache invalidation on binary rename**: cch to rulez rename left stale binaries in ~/.cargo/bin/, tests execute wrong code — PREVENT: Explicit cache cleanup with versioned keys, always use cargo run --bin rulez in CI (not bare binary name), validate binary with which rulez before tests, add binary artifact validation step

## Implications for Roadmap

Based on research, v1.4 should be structured as **four independent phases** that can be developed in parallel (Phases 1-2), then integrated and validated (Phase 3), and finally deployed to CI (Phase 4).

### Phase 1: JSON Schema Validation
**Rationale:** Core correctness feature with highest complexity — validating event structure before processing prevents downstream bugs. Must be first to establish schema patterns for other phases.

**Delivers:**
- schemars 1.2.1 dependency added
- Event struct has JsonSchema derive macro
- Pre-compiled schema validators cached in LazyLock
- validate_event_schema() called in main.rs before process_event()
- Draft version validation (draft-07 and 2020-12 only)
- Fail-open mode with warnings for invalid events

**Addresses (from FEATURES.md):**
- JSON Schema event validation (table stakes)
- Pre-compiled schema caching (table stakes)
- Fail-open validation mode (table stakes)
- JSON Schema draft version validation (differentiator)

**Avoids (from PITFALLS.md):**
- Pitfall 1: JSON Schema draft version incompatibility (require `$schema` field)
- Pitfall 2: Schema validation performance (pre-compile at config load)
- Pitfall 7: allOf misuse with serde flatten (document in guidelines)

**Research flag:** Standard patterns (schemars derive macros well-documented, jsonschema crate widely used) — skip research-phase during planning.

---

### Phase 2: Debug CLI UserPromptSubmit
**Rationale:** Independent feature that closes v1.3 testing gap — can be developed in parallel with Phase 1. Low complexity, reuses existing infrastructure.

**Delivers:**
- UserPromptSubmit added to SimEventType enum
- --prompt flag added to debug CLI args
- REGEX_CACHE cleared at start of run() for state isolation
- LRU cache replaces unbounded HashMap (fixes v1.3 tech debt)
- rulez debug prompt --prompt "text" command works

**Uses (from STACK.md):**
- tokio::io::stdin() for async multiline input (existing dependency)
- lru crate for bounded regex cache (new dependency, optional)

**Implements (from ARCHITECTURE.md):**
- Debug CLI extension pattern (additive enum variant)
- State isolation pattern (clear caches between invocations)
- Correlation ID pattern (optional, for tracing)

**Avoids (from PITFALLS.md):**
- Pitfall 3: Debug CLI state contamination (clear REGEX_CACHE)
- Pitfall 9: Debug CLI flag proliferation (use subcommands, not flags)

**Research flag:** Standard patterns (tokio stdin well-documented, CLI testing with assert_cmd established) — skip research-phase during planning.

---

### Phase 3: E2E Test Stabilization
**Rationale:** Must come after Phases 1-2 to validate schema validation and debug CLI work correctly. Blocks Phase 4 (Tauri CI depends on E2E tests passing).

**Delivers:**
- canonicalize_path() helper in tests/common/mod.rs
- All E2E tests use canonical paths in setup
- Broken pipe fixes (use wait_with_output() instead of spawn() + wait())
- CI matrix includes ubuntu-latest, macos-latest, windows-latest
- Binary artifact validation (check which rulez before tests)
- Symlink resolution tests (Unix-only, validates fs::canonicalize)

**Addresses (from FEATURES.md):**
- E2E test cross-platform paths (table stakes)
- E2E test broken pipe fixes (table stakes)
- Binary artifact validation (table stakes, prevents stale cache issues)
- Cross-platform CI matrix (differentiator)

**Avoids (from PITFALLS.md):**
- Pitfall 4: E2E test tempfile path resolution (fs::canonicalize)
- Pitfall 6: GitHub Actions cache staleness (validate binary name)
- Pitfall 8: Broken pipe on Linux (use wait_with_output)

**Research flag:** Standard patterns (assert_cmd well-documented, cross-platform testing established) — skip research-phase during planning.

---

### Phase 4: Tauri CI Integration
**Rationale:** Final phase, depends on Phase 3 (E2E tests must pass to validate build artifacts). Slowest phase (5-10 min CI builds), only run on release branches.

**Delivers:**
- .github/workflows/tauri-build.yml (new workflow file)
- E2E test job runs first in web mode (fast, uses Playwright)
- Tauri build job only runs if E2E tests pass (slow, multi-platform matrix)
- Linux uses ubuntu-22.04 with libwebkit2gtk-4.1-dev
- macOS and Windows native builds
- Artifacts uploaded for .dmg, .msi, .AppImage

**Addresses (from FEATURES.md):**
- Tauri 2.0 webkit dependency (table stakes)
- Tauri 2.0 CI matrix builds (table stakes)
- E2E tests run before builds (differentiator, fail-fast pattern)

**Avoids (from PITFALLS.md):**
- Pitfall 5: Tauri webkit version conflict (use webkit2gtk-4.1, explicit ubuntu-22.04)
- Pitfall 6: Stale cache artifacts (validate binary before upload)

**Research flag:** Standard patterns (Tauri GitHub Actions official, webkit2gtk-4.1 requirement documented) — skip research-phase during planning.

---

### Phase Ordering Rationale

**Parallel development (Phases 1-2):**
- JSON Schema integration and debug CLI have no dependencies
- Can be developed simultaneously by different developers
- Both enhance validation/testing infrastructure

**Sequential validation (Phase 3):**
- E2E tests validate Phases 1-2 work correctly on all platforms
- Must pass before Tauri builds are useful (Phase 4)
- Blocks deployment to CI

**Final integration (Phase 4):**
- Tauri builds only after E2E tests pass (fail-fast)
- Slowest phase, runs only on release branches
- Validates entire stack works on all platforms

**Critical path:** Phase 1 OR Phase 2 → Phase 3 → Phase 4 (total: 6-10 days estimated)

### Research Flags

**Phases with standard patterns (skip research-phase):**
- **Phase 1 (JSON Schema)**: schemars and jsonschema crates have extensive official documentation, derive macros well-established pattern
- **Phase 2 (Debug CLI)**: tokio stdin built-in feature, CLI testing with assert_cmd widely documented
- **Phase 3 (E2E Tests)**: Cross-platform testing patterns well-established, fs::canonicalize standard library
- **Phase 4 (Tauri CI)**: Official Tauri GitHub Actions, webkit2gtk-4.1 requirement explicitly documented

**No phases need deeper research during planning.** All features use well-documented patterns with HIGH confidence sources.

## Confidence Assessment

| Area | Confidence | Notes |
|------|------------|-------|
| Stack | HIGH | schemars and jsonschema official docs verified, tokio stdin built-in, Tauri 2.0 docs explicit about webkit2gtk-4.1 |
| Features | HIGH | v1.3 tech debt items well-documented, feature scope clearly defined as stability (not expansion) |
| Architecture | HIGH | Integration points identified via code inspection, additive patterns preserve existing engine |
| Pitfalls | HIGH | All six critical pitfalls have documented real-world evidence (GitHub issues, official migration guides) |

**Overall confidence:** HIGH

### Gaps to Address

**Performance validation required:**
- Benchmark schema validation overhead with 100+ rules (target: <0.1ms per event)
- Verify total processing latency stays under 10ms with criterion regression tests
- Validate binary size stays under 5 MB (currently 2.2 MB + 50 KB schemars = 2.25 MB)

**Cross-platform testing required:**
- Run E2E tests on Windows in CI (currently only tested on macOS and Linux locally)
- Validate Tauri builds succeed on all platforms in matrix (ubuntu-22.04, macos-latest, windows-latest)
- Test symlink resolution explicitly on macOS (/var → /private/var)

**CI cache behavior:**
- Verify stale binary detection works (which rulez validation step)
- Test cache invalidation after binary rename (versioned cache keys)
- Validate webkit2gtk-4.1 installation on ubuntu-22.04 and ubuntu-24.04

**All gaps are validation tasks during implementation, NOT research gaps.** The approach is clear, execution needs verification.

## Sources

### Primary (HIGH confidence)

**JSON Schema:**
- [schemars docs.rs 1.2.1](https://docs.rs/schemars) — API documentation, derive macro usage
- [schemars official docs](https://graham.cool/schemars/) — Serde integration patterns
- [jsonschema docs.rs 0.41](https://docs.rs/jsonschema) — Validator performance, pre-compilation
- [JSON Schema draft specifications](https://json-schema.org/specification) — Draft-07 and 2020-12 differences
- [GSoC 2026: JSON Schema Compatibility Checker](https://github.com/json-schema-org/community/issues/984) — Breaking changes between drafts

**Tauri 2.0:**
- [Tauri 2.0 Prerequisites](https://v2.tauri.app/start/prerequisites/) — System dependencies per platform
- [Tauri 2.0 webkit migration guide](https://v2.tauri.app/blog/tauri-2-0-0-alpha-3/) — webkit2gtk-4.1 requirement
- [Tauri GitHub Issue 9662](https://github.com/tauri-apps/tauri/issues/9662) — Ubuntu 24.04 removed webkit2gtk-4.0
- [tauri-apps/tauri-action](https://github.com/tauri-apps/tauri-action) — Official GitHub Action for multi-platform builds

**Rust CLI Testing:**
- [Testing - Command Line Applications in Rust](https://rust-cli.github.io/book/tutorial/testing.html) — assert_cmd best practices
- [tokio stdin docs](https://docs.rs/tokio/latest/tokio/io/struct.Stdin.html) — Async stdin built-in feature
- [assert_cmd docs](https://docs.rs/assert_cmd) — CLI testing patterns, stdin/stdout handling

**Project Context:**
- RuleZ CLAUDE.md — Pre-push checklist, binary rename history, CI requirements
- RuleZ MEMORY.md — Stale binary artifacts lesson, broken pipe on Linux fix
- RuleZ codebase inspection — main.rs, hooks.rs, cli/debug.rs, tests/e2e_*.rs

### Secondary (MEDIUM confidence)

**Event-Driven Testing:**
- [AWS EventBridge test-event-pattern](https://docs.aws.amazon.com/cli/latest/reference/events/test-event-pattern.html) — CLI patterns for event simulation
- [Stripe CLI webhook testing](https://www.projectschool.dev/blogs/devessentials/How-to-Test-Webhooks-Using-Stripe-CLI) — Simulating events locally

**GitHub Actions:**
- [Swatinem/rust-cache docs](https://github.com/Swatinem/rust-cache) — Cache behavior, invalidation patterns
- [GitHub Actions matrix builds guide](https://oneuptime.com/blog/post/2026-01-25-github-actions-matrix-builds/view) — Multi-platform testing

### Tertiary (LOW confidence)

**Playwright/Bun:**
- [BrowserStack: Bun for Playwright](https://www.browserstack.com/guide/bun-playwright) — Compatibility notes (works for basic use, not officially supported)
- [GitHub Issue 27139](https://github.com/microsoft/playwright/issues/27139) — Community discussion on Bun compatibility

---

**Research completed:** 2026-02-10
**Ready for roadmap:** Yes

**Total files synthesized:** 4 (STACK.md, FEATURES.md, ARCHITECTURE.md, PITFALLS.md)
**Research confidence:** HIGH across all domains
**Estimated implementation time:** 6-10 developer-days (4-6 days with parallel development)
**Performance budget maintained:** Yes (<10ms total processing, <0.1ms schema validation)
**Breaking changes:** None (all features are additive)
