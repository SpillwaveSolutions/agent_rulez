# Feature Landscape: v1.4 Stability & Polish

**Domain:** Rust CLI Policy Engine - Stability & Developer Experience Features
**Researched:** 2026-02-10
**Confidence:** HIGH

## Executive Summary

v1.4 focuses on **stabilizing and polishing** the existing RuleZ policy engine, NOT adding new user-facing policy features. The milestone addresses four specific gaps identified in v1.3 tech debt:

1. **JSON Schema validation** for hook event payloads
2. **Debug CLI improvements** for simulating UserPromptSubmit events
3. **E2E test stabilization** for cross-platform reliability
4. **Tauri UI build fixes** for multi-platform CI integration

**Key insight:** These are **infrastructure features** that improve developer experience, test reliability, and correctness validation — not end-user features like new rule matchers or actions.

---

## Table Stakes Features

Features that v1.4 MUST deliver to be considered complete. These address known gaps and technical debt.

| Feature | Why Expected | Complexity | Dependencies | Notes |
|---------|--------------|------------|--------------|-------|
| **JSON Schema event validation** | Validate incoming hook events match Claude Code's schema before processing | Medium | schemars 1.2 (schema gen), jsonschema 0.41 (validation, already exists) | Draft-07 and 2020-12 support required |
| **Pre-compiled schema caching** | Performance requirement: schema validation <0.1ms per event | Low | OnceCell or lazy_static (already exists) | Load-time compilation, not per-event |
| **Fail-open validation mode** | Schema errors shouldn't block valid events (existing fail_open pattern) | Low | None | Log warnings, continue processing |
| **Debug CLI: UserPromptSubmit simulation** | Close testing gap for prompt_match rules (v1.3 tech debt #3) | Low | None (existing tokio stdin) | Enable `rulez debug prompt --prompt "text"` |
| **E2E tests: cross-platform paths** | Tests must pass on Linux, macOS, Windows in CI | Medium | fs::canonicalize, PathBuf (stdlib) | Resolve symlinks, platform-agnostic paths |
| **E2E tests: broken pipe fixes** | Tests failing on Linux due to unread stdio pipes (v1.3 bug) | Low | Command::wait_with_output() (stdlib) | Always drain stdout/stderr or use Stdio::null() |
| **Tauri 2.0: webkit2gtk-4.1** | Correct Linux dependency for Tauri 2.0 (4.0 removed in Ubuntu 24.04) | Low | System package (apt-get) | CI configuration, not code change |
| **Tauri 2.0: CI matrix builds** | Multi-platform CI (Linux, macOS, Windows) to catch platform-specific issues | Medium | GitHub Actions, tauri-apps/tauri-action | Prevent "works on my machine" bugs |

### Why These Are Table Stakes

**Context from v1.3 Milestone Audit:**
- Debug CLI cannot test prompt_match rules (gap identified in tech debt)
- E2E tests have broken pipe issues on Linux (CI failures documented)
- Tauri app builds locally but not in CI (no GitHub Actions workflow)
- No validation of incoming hook events (assumed Claude Code sends valid JSON)

**User expectation:** A "Stability & Polish" milestone means fixing known bugs and closing testing gaps. Missing any of these would leave v1.3 issues unresolved.

---

## Differentiators

Features that set v1.4 apart from a minimal bug-fix release. Not expected, but valuable for long-term maintainability.

| Feature | Value Proposition | Complexity | Dependencies | Notes |
|---------|-------------------|------------|--------------|-------|
| **JSON Schema draft version validation** | Prevent breaking changes when schemas evolve (fail-closed on unsupported drafts) | Low | None | Require explicit `$schema` field, reject draft-04/draft-06 |
| **Schema performance benchmarks in CI** | Guarantee <10ms processing budget with regression tests | Medium | cargo bench, criterion | Fail CI if p95 latency exceeds 10ms |
| **LRU regex cache** | Fix unbounded REGEX_CACHE growth (v1.3 tech debt #1) | Low | lru crate | Max 100 compiled regexes, evict least-recently-used |
| **Debug CLI correlation IDs** | Trace event flow across debug invocations for reproducibility | Low | uuid crate | Log correlation ID with every decision |
| **Debug CLI `--clean` flag** | Force fresh state (clear caches) for guaranteed test isolation | Low | None | Alternative to default cache reuse |
| **E2E symlink resolution tests** | Explicitly test macOS /var symlink handling (common CI failure source) | Low | std::os::unix::fs::symlink | Unix-only test, validates fs::canonicalize() works |
| **Tauri build artifact verification** | CI validates correct binary name before upload (prevent stale cache issues) | Low | which, --version check | Detect renamed binaries in CI cache |
| **Cross-platform CI matrix for E2E** | Run E2E tests on ubuntu-22.04, ubuntu-24.04, macos-latest, windows-latest | Medium | GitHub Actions matrix | Catch platform-specific bugs before release |

### Why These Are Differentiators

**Generic stability features** (schema validation, E2E tests, CI) are table stakes.

**RuleZ-specific quality measures** (draft version enforcement, performance regression tests, cache limits, correlation IDs) go beyond "fix the bugs" to "prevent future bugs."

**Example:** Most projects add JSON Schema validation. Few enforce draft version compatibility and fail-closed on missing `$schema` fields. Fewer still benchmark validation overhead in CI.

---

## Anti-Features

Features to explicitly NOT build in v1.4.

| Anti-Feature | Why Avoid | What to Do Instead |
|--------------|-----------|-------------------|
| **New rule matchers or actions** | v1.4 is stability, not new features. Defer to v1.5 | Document feature requests in `.planning/backlog/` |
| **Fail-closed schema validation by default** | Would block valid events if schema file missing/outdated | Fail-open with warnings (matches existing `fail_open: true` pattern) |
| **Custom JSON Schema draft support** | Maintenance burden, compatibility risk | Support only LTS drafts: draft-07 and 2020-12 |
| **Interactive debug CLI (REPL mode)** | Already exists (`rulez repl`), don't duplicate | Extend existing REPL with UserPromptSubmit support |
| **Debug CLI persistent state across invocations** | Causes flaky tests, unreproducible bugs | Default to stateless, offer `--clean` flag for explicit cache clearing |
| **E2E tests with full Tauri app** | Slow, brittle, requires GUI automation | Test web mode with Playwright (Tauri commands have fallbacks) |
| **Tauri builds on every PR** | Slow CI (5-10 minutes per platform), expensive | Run on release branches only, E2E tests on every PR |
| **Support for Ubuntu 20.04 / Debian 11** | webkit2gtk-4.1 not available, Tauri 2.0 incompatible | Document minimum OS: Ubuntu 22.04+ / Debian 12+ |
| **Regex engine switching (fancy-regex vs regex)** | Security vs performance trade-off, not v1.4 scope | Defer to v1.5 if DDoS concerns arise |
| **Schema migration tools** | Scope creep, users can manually update schemas | Document breaking changes in CHANGELOG, provide examples |

### Rationale

**v1.4 purpose:** Fix existing functionality, NOT add new capabilities.

**Examples of scope creep to avoid:**
- "While fixing debug CLI, let's add interactive mode" → NO (already exists)
- "While adding schema validation, let's support custom drafts" → NO (maintenance burden)
- "While fixing E2E tests, let's add full Tauri E2E" → NO (too slow, unnecessary)

**Rule of thumb:** If it wasn't in v1.3 tech debt or doesn't fix a known bug, defer to v1.5.

---

## Feature Dependencies

Dependencies between v1.4 features (build order implications).

```
Feature Graph:

JSON Schema Validation (Phase 1)
├── Pre-compiled schema caching (required for performance)
├── Draft version validation (required for correctness)
└── Fail-open mode (required for backwards compat)

Debug CLI UserPromptSubmit (Phase 2)
├── Event struct already has `prompt` field (v1.3) ✅
├── tokio stdin support (v1.0) ✅
└── LRU regex cache (optional, fixes v1.3 tech debt)

E2E Test Stabilization (Phase 3)
├── Cross-platform path handling (required for CI matrix)
├── Broken pipe fixes (required for Linux CI)
├── Binary artifact validation (required after v1.3 rename)
└── Symlink resolution (optional, prevents macOS CI failures)

Tauri CI Integration (Phase 4)
├── E2E tests passing (Phase 3) — validates build artifacts work
├── webkit2gtk-4.1 on Linux (required for Tauri 2.0)
└── GitHub Actions matrix (required for multi-platform)
```

**Critical path:** Phases 1, 2, 3 are independent (can run in parallel). Phase 4 depends on Phase 3 (E2E tests must pass before Tauri builds are useful).

**Suggested order:**
1. Phase 1 + Phase 2 in parallel (schema validation + debug CLI, no dependencies)
2. Phase 3 (E2E fixes, validates Phases 1-2 work correctly)
3. Phase 4 (Tauri CI, validates everything works on all platforms)

---

## Feature Complexity Analysis

Breakdown by implementation effort and risk.

### Low Complexity (1-2 days each)

| Feature | Why Low | Risk |
|---------|---------|------|
| Fail-open validation mode | Pattern already exists in config.rs | None |
| Debug CLI `--prompt` flag | Add to clap args, pass to Event struct | None |
| Broken pipe fixes | Replace `.spawn()` with `.output()` | None |
| LRU regex cache | Drop-in replacement for HashMap | Low (breaking change in cache semantics) |
| Tauri webkit dependency | CI config update, not code | None |
| Binary artifact validation | Shell script in GitHub Actions | None |

### Medium Complexity (3-5 days each)

| Feature | Why Medium | Risk |
|---------|------------|------|
| JSON Schema event validation | New validation.rs module, integrate into main.rs | Medium (performance if not cached) |
| Pre-compiled schema caching | Requires OnceCell in Rule struct, serde skip attribute | Low (well-understood pattern) |
| Draft version validation | Parse `$schema` field, map to JSONSchema::Draft enum | Low (explicit error messages) |
| Cross-platform path handling | Refactor all path usage to PathBuf, test on Windows | Medium (symlinks, path separators) |
| E2E CI matrix | GitHub Actions matrix, 4 platforms × 3 test suites | Low (slow CI, may need caching tuning) |
| Tauri CI builds | Multi-platform matrix, system deps, artifact uploads | High (platform-specific failures) |

### High Complexity (5+ days)

| Feature | Why High | Risk |
|---------|----------|------|
| Schema performance benchmarks | Criterion integration, CI regression tests, p95 tracking | Medium (flaky benchmarks, CI noise) |
| Cross-platform E2E test suite | Path handling, symlinks, broken pipes, Windows line endings | High (platform-specific edge cases) |

**Total effort estimate:** 15-25 developer-days (3-5 weeks for single developer).

---

## MVP Recommendation

Minimum viable v1.4 that delivers on "Stability & Polish" promise.

### Prioritize (Must Have)

**Phase 1: JSON Schema Validation**
1. Add schemars 1.2 dependency
2. Generate schemas for HookEvent and related types
3. Pre-compile schemas at config load (OnceCell)
4. Validate events in main.rs before processing (fail-open)
5. Require `$schema` field, support draft-07 and 2020-12 only

**Phase 2: Debug CLI UserPromptSubmit**
1. Add UserPromptSubmit to SimEventType enum
2. Add `--prompt` flag to debug CLI args
3. Pass prompt to Event struct in build_event()
4. Test: `rulez debug prompt --prompt "text"` matches prompt_match rules

**Phase 3: E2E Test Stabilization**
1. Fix broken pipe: use `.output()` or `.wait_with_output()`
2. Canonicalize paths with `fs::canonicalize()` before comparison
3. Use PathBuf for all path operations (not string concatenation)
4. Add CI matrix: ubuntu-latest, macos-latest, windows-latest

**Phase 4: Tauri CI Integration**
1. Create `.github/workflows/tauri-build.yml`
2. Install webkit2gtk-4.1-dev on Linux (Ubuntu 22.04 runner)
3. Build on macOS, Linux, Windows (matrix strategy)
4. Validate binary artifacts before upload

**Success criteria:** All v1.3 tech debt items resolved, CI green on all platforms.

### Defer (Nice to Have)

- LRU regex cache (v1.3 tech debt, but not blocking)
- Debug CLI correlation IDs (useful for tracing, but not critical)
- Debug CLI `--clean` flag (workaround: restart process)
- E2E symlink resolution tests (covered by fs::canonicalize)
- Schema performance benchmarks (validate manually, add to CI later)
- Cross-platform CI for core (already have CI, extend to more platforms later)

### Rationale

**MVP delivers:**
- ✅ Schema validation for correctness
- ✅ Debug CLI feature parity with production events
- ✅ E2E tests passing on all platforms
- ✅ Tauri builds in CI

**Deferred features:**
- Not blocking release (LRU cache, correlation IDs)
- Can be added incrementally (benchmarks, extended CI)
- Workarounds exist (`--clean` flag → restart process)

---

## Expected Behavior (Table Stakes Detail)

### JSON Schema Validation

**User perspective:**
```bash
# Hook event from Claude Code
echo '{"hook_event_name": "PreToolUse", "tool_name": "Bash", ...}' | rulez

# Valid event → processes normally
# Invalid event → logs warning, processes anyway (fail-open)
```

**Developer perspective:**
```rust
// Automatic schema generation from types
#[derive(Deserialize, JsonSchema)]
struct HookEvent {
    hook_event_name: String,
    tool_name: Option<String>,
    // ... fields match schema exactly
}

// Validation at startup (not per-event)
let schema = schema_for!(HookEvent);
let validator = JSONSchema::compile(&schema)?;

// Per-event validation (<0.1ms)
if let Err(e) = validator.validate(&event_json) {
    warn!("Schema validation failed: {}", e);  // Log, don't block
}
```

**Expected behavior:**
- Schema compiled once at config load
- Validation <0.1ms per event
- Fail-open: invalid events log warnings but process anyway
- Supported drafts: draft-07, 2020-12 (reject others)
- Missing `$schema` field → error at config load

### Debug CLI UserPromptSubmit

**User perspective:**
```bash
# Before v1.4: Cannot test prompt_match rules
rulez debug PreToolUse --tool Bash --command "git push"  # Works
rulez debug UserPromptSubmit --prompt "write a function"  # Error: unsupported

# After v1.4: Can test prompt_match rules
rulez debug prompt --prompt "write a function to parse JSON"
# Output: Shows which rules matched, including prompt_match rules

# Short aliases
rulez debug prompt --prompt "refactor this code"
rulez debug user-prompt-submit --prompt "delete database"
```

**Developer perspective:**
```rust
// Add to SimEventType enum
enum SimEventType {
    PreToolUse,
    PostToolUse,
    SessionStart,
    PermissionRequest,
    UserPromptSubmit,  // NEW
}

// Build event with prompt field
let event = Event {
    hook_event_name: "UserPromptSubmit",
    prompt: Some(prompt_text),  // NEW: Set from --prompt flag
    // ... other fields
};
```

**Expected behavior:**
- `rulez debug prompt --prompt "text"` simulates UserPromptSubmit
- Event has `prompt` field populated (existing field in Event struct)
- Rules with `prompt_match` matchers evaluate correctly
- No state leakage between invocations

### E2E Test Cross-Platform Reliability

**User perspective:**
```bash
# Before v1.4: Tests pass locally (macOS), fail in CI (Linux)
cargo test --test e2e_git_push_block
# macOS: ✅ 1 test passed
# Linux CI: ❌ SIGPIPE error

# After v1.4: Tests pass everywhere
cargo test --test e2e_*
# macOS: ✅ All tests passed
# Linux: ✅ All tests passed
# Windows: ✅ All tests passed
```

**Developer perspective:**
```rust
// Before: Broken pipe on Linux
let mut child = Command::cargo_bin("rulez")
    .stdin(Stdio::piped())
    .stdout(Stdio::piped())  // ❌ Pipe created but never read
    .spawn()?;
child.stdin.as_mut().unwrap().write_all(event_json.as_bytes())?;
let status = child.wait()?;  // ❌ SIGPIPE if rulez writes to stdout

// After: Always drain pipes
let output = Command::cargo_bin("rulez")
    .write_stdin(event_json)
    .output()?;  // ✅ Reads stdout/stderr automatically
assert_eq!(output.status.code(), Some(0));

// Path handling: Before
let config_path = format!("{}/.claude/hooks.yaml", cwd);  // ❌ Breaks on Windows

// Path handling: After
let config_path = PathBuf::from(&cwd).join(".claude").join("hooks.yaml");  // ✅
```

**Expected behavior:**
- E2E tests use `Command::output()` or `wait_with_output()` (always drain pipes)
- All paths use `PathBuf`, not string concatenation
- Symlinks resolved with `fs::canonicalize()` before comparison
- Tests pass on Linux, macOS, Windows in CI matrix

### Tauri 2.0 CI Builds

**User perspective (contributor):**
```bash
# Before v1.4: Local build works, CI fails
cd rulez-ui
bun run build:tauri
# Local (macOS): ✅ Builds successfully
# CI (Linux): ❌ Error: libwebkit2gtk-4.0-dev not found

# After v1.4: CI builds on all platforms
git push origin feature/my-ui-change
# GitHub Actions:
#   - ✅ Linux build (ubuntu-22.04, webkit2gtk-4.1)
#   - ✅ macOS build (macos-latest)
#   - ✅ Windows build (windows-latest)
```

**Developer perspective:**
```yaml
# .github/workflows/tauri-build.yml
jobs:
  build-tauri:
    strategy:
      matrix:
        platform: [ubuntu-22.04, macos-latest, windows-latest]
    runs-on: ${{ matrix.platform }}
    steps:
      - name: Install Tauri deps (Linux)
        if: matrix.platform == 'ubuntu-22.04'
        run: |
          sudo apt-get install -y libwebkit2gtk-4.1-dev  # ✅ Correct version
```

**Expected behavior:**
- CI builds Tauri app on Linux, macOS, Windows
- Linux uses webkit2gtk-4.1-dev (NOT 4.0)
- Builds succeed on ubuntu-22.04 (NOT ubuntu-latest/24.04 where 4.1 may be unstable)
- Artifacts uploaded only on release branches (not every PR, too slow)

---

## Sources

**JSON Schema Validation (HIGH confidence):**
- [jsonschema Rust crate](https://docs.rs/jsonschema) — Performance: 20-470x faster than alternatives, reusable validators
- [schemars documentation](https://graham.cool/schemars/) — Automatic schema generation from Rust types
- [jsonschema GitHub](https://github.com/Stranger6667/jsonschema) — Benchmarks, regex engine configuration
- [GSoC 2026: JSON Schema Compatibility Checker](https://github.com/json-schema-org/community/issues/984) — Breaking changes between drafts
- [JSON Schema Validation: Common Mistakes](https://www.countermetrics.org/validation-results/) — 41% of validation failures are missing required fields
- [JSON Schema Data Types Guide](https://blog.postman.com/json-schema-data-types/) — Draft-07 vs 2020-12 differences

**Debug CLI & Event Simulation (MEDIUM confidence):**
- [AWS EventBridge test-event-pattern](https://docs.aws.amazon.com/cli/latest/reference/events/test-event-pattern.html) — CLI patterns for event testing
- [Event-Driven Testing: Key Strategies](https://optiblack.com/insights/event-driven-testing-key-strategies) — Event recording, playback, simulation patterns
- [Stripe CLI webhook testing](https://www.projectschool.dev/blogs/devessentials/How-to-Test-Webhooks-Using-Stripe-CLI) — Simulating events locally

**E2E Testing (HIGH confidence):**
- [Testing - Command Line Applications in Rust](https://rust-cli.github.io/book/tutorial/testing.html) — assert_cmd best practices
- [E2E Testing for Rust CLI Applications](https://www.slingacademy.com/article/approaches-for-end-to-end-testing-in-rust-cli-applications/) — assert_cmd integration tests
- [E2E Testing Best Practices 2026](https://oneuptime.com/blog/post/2026-01-30-e2e-testing-best-practices/view) — Simulation, automation, CI/CD integration
- [Rust Stdio pipes documentation](https://doc.rust-lang.org/stable/std/process/struct.Stdio.html) — Deadlock warnings, buffer sizes
- [os_pipe.rs cross-platform pipes](https://github.com/oconnor663/os_pipe.rs) — Pipe deadlock prevention

**Tauri 2.0 CI (HIGH confidence):**
- [Tauri GitHub Actions Guide](https://v2.tauri.app/distribute/pipelines/github/) — Official CI/CD setup
- [tauri-apps/tauri-action](https://github.com/tauri-apps/tauri-action) — Multi-platform builds (Windows x64, Linux x64/ARM64, macOS x64/ARM64)
- [Tauri 2.0 webkit2gtk-4.1 requirement](https://github.com/tauri-apps/tauri/issues/9662) — Ubuntu 24.04 removed webkit2gtk-4.0-dev
- [Tauri v2 Prerequisites](https://v2.tauri.app/start/prerequisites/) — System dependencies per platform

**Project Context (HIGH confidence):**
- `.planning/research/STACK.md` — Existing dependencies, performance requirements
- `.planning/research/ARCHITECTURE.md` — v1.3 event processing pipeline
- `.planning/research/PITFALLS.md` — Known issues (broken pipe, schema draft incompatibility, webkit version)
- `CLAUDE.md` — Pre-Push Checklist, binary rename history, CI failures
- `MEMORY.md` — Stale binary artifacts, broken pipe on Linux lessons

---

**Researched:** 2026-02-10
**Valid until:** 2026-05-10 (90 days — stable ecosystem, mature crates)

**Confidence breakdown:**
- JSON Schema features: **HIGH** (official docs, benchmarks, draft compatibility research)
- Debug CLI simulation: **MEDIUM** (patterns from AWS/Stripe CLIs, not Rust-specific)
- E2E test stabilization: **HIGH** (Rust official docs, assert_cmd best practices, stdio deadlock warnings)
- Tauri CI integration: **HIGH** (official Tauri docs, GitHub issue confirming webkit2gtk-4.1 requirement)

**Roadmap implications:**
- v1.4 is **polish milestone**, not feature expansion
- Phases 1-2 add correctness checks (schema validation, debug parity)
- Phases 3-4 improve developer experience (reliable tests, working CI)
- No new user-facing features (defer prompt injection, new matchers to v1.5)
