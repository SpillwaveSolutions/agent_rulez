# Domain Pitfalls: v1.4 Stability & Polish Features

**Project:** RuleZ Policy Engine
**Milestone:** v1.4 - JSON Schema validation, debug CLI, E2E tests, Tauri CI
**Researched:** 2026-02-10
**Confidence:** HIGH

## Summary

v1.4 adds JSON Schema validation to an existing event processing pipeline, extends debug CLI with new event types, fixes E2E tests for CLI binary testing, and adds cross-platform Tauri 2.0 builds to GitHub Actions CI. This research identifies pitfalls specific to **adding these features to an existing system** that already experienced CI issues (binary rename artifacts, broken pipe on Linux).

**Context from v1.3 Tech Debt:**
- Debug CLI cannot simulate `UserPromptSubmit` or pass prompt text via flags
- Unbounded regex cache (REGEX_CACHE) needs LRU/max-size guard
- Debug CLI help text doesn't mention prompt-related event types
- No sandboxing for inline shell scripts (deferred to v1.4)

**v1.4-Specific Risks:**
1. **JSON Schema draft compatibility** - Breaking changes between drafts
2. **Schema validation performance** - Event processing pipeline overhead
3. **Debug CLI state management** - Stateful event simulation pitfalls
4. **E2E test tempfile handling** - Path resolution and cleanup issues
5. **Tauri CI dependency hell** - webkit2gtk version conflicts, cross-platform builds
6. **GitHub Actions cache staleness** - Binary rename artifacts, stale test results

## Critical Pitfalls

### Pitfall 1: JSON Schema Draft Version Incompatibility

**Severity:** CRITICAL - Correctness + Breaking Changes

**What goes wrong:** Using different JSON Schema draft versions in validation causes silent failures or breaking changes when schemas are upgraded.

**Why it happens:** JSON Schema has breaking changes between draft-07, draft-2019-09, and draft-2020-12. The `jsonschema` Rust crate supports multiple drafts, but defaults to the latest (2020-12). If RuleZ config files or user schemas specify older drafts, validation behavior changes unexpectedly.

**Real-world evidence (2026):**
- GSoC 2026 project proposed a "JSON Schema Compatibility Checker" because breaking changes between versions are common
- Core keyword `dependencies` split into `dependentSchemas` and `dependentRequired` between draft-07 and 2020-12
- Future JSON Schema will enforce strict backward/forward compatibility, but current versions DO NOT

**RuleZ-specific scenario:**
```yaml
# User's hooks.yaml references draft-07 schema
rules:
  - matchers:
      event_schema:
        $schema: "http://json-schema.org/draft-07/schema#"
        properties:
          tool_name: { type: "string" }
        dependencies:
          tool_name: ["tool_input"]  # draft-07 syntax
```

```rust
// RuleZ validates with jsonschema crate (defaults to 2020-12)
let schema = serde_json::from_str(schema_str)?;
let compiled = JSONSchema::compile(&schema)?;  // ❌ Interprets draft-07 as 2020-12
```

**Consequences:**
- Validation fails silently (schema ignored)
- Breaking change when upgrading `jsonschema` crate
- User configs break without clear error messages
- Security policies bypass if validation fails open

**Prevention strategy:**

1. **Explicitly specify and validate draft version:**
   ```rust
   use jsonschema::{Draft, JSONSchema};

   fn compile_schema(schema: &serde_json::Value) -> Result<JSONSchema> {
       // Check $schema field
       let draft_version = schema.get("$schema")
           .and_then(|s| s.as_str())
           .ok_or("Missing $schema field - required for validation")?;

       // Only support LTS versions (draft-07 and 2020-12)
       let draft = match draft_version {
           "http://json-schema.org/draft-07/schema#" => Draft::Draft7,
           "https://json-schema.org/draft/2020-12/schema" => Draft::Draft202012,
           _ => return Err(format!("Unsupported schema draft: {}", draft_version)),
       };

       JSONSchema::options()
           .with_draft(draft)
           .compile(schema)
   }
   ```

2. **Fail-closed on missing `$schema`:**
   - NEVER default to a draft version
   - Require explicit `$schema` field in all user schemas
   - Config validation must catch this before runtime

3. **Document supported drafts in YAML schema:**
   ```yaml
   # .claude/hooks-schema.yaml
   event_schema:
     type: object
     required: ["$schema"]
     properties:
       $schema:
         type: string
         enum:
           - "http://json-schema.org/draft-07/schema#"
           - "https://json-schema.org/draft/2020-12/schema"
         description: "Required. Only draft-07 and 2020-12 supported."
   ```

4. **Add migration warning for draft-04/draft-06:**
   ```rust
   if draft_version.contains("draft-04") || draft_version.contains("draft-06") {
       warn!(
           "JSON Schema {} is deprecated. Migrate to draft-07 or 2020-12. \
            See https://json-schema.org/specification for migration guide.",
           draft_version
       );
   }
   ```

5. **Pin `jsonschema` crate version:**
   ```toml
   [dependencies]
   jsonschema = "=0.18.0"  # Exact version, not "0.18" (prevents breaking updates)
   ```

**Warning signs:**
- Schema validation passes in tests but fails in production
- `jsonschema` crate update breaks existing configs
- User reports "my schema stopped working after update"
- Logs show "schema compilation failed" with cryptic errors

**Detection:**
```bash
# Test schema with different drafts
cat > /tmp/test-schema.json <<EOF
{
  "$schema": "http://json-schema.org/draft-07/schema#",
  "properties": { "test": { "type": "string" } },
  "dependencies": { "test": ["other"] }
}
EOF

# Should fail gracefully if draft not supported
rulez validate --schema /tmp/test-schema.json
```

**Phase mapping:** Phase 1 (JSON Schema Integration) MUST validate draft versions.

---

### Pitfall 2: Schema Validation Performance in Event Pipeline

**Severity:** CRITICAL - Performance

**What goes wrong:** Adding JSON Schema validation to the hot path (per-event processing) causes unacceptable latency if schemas are not pre-compiled and cached.

**Why it happens:** The `jsonschema` crate documentation explicitly warns: "For better performance when validating multiple instances against the same schema, build a validator once and reuse it."

**Performance data (from research):**
- Schema compilation: ~0.5-2ms per schema (depending on complexity)
- Validation with cached validator: ~0.01-0.1ms
- Validation with recompilation: ~0.5-2ms
- **RuleZ budget:** <10ms total per event (currently <3ms)

**RuleZ-specific scenario:**
```rust
// WRONG: Compiles schema on every event
pub fn process_event(event: &Event, config: &Config) -> Response {
    for rule in &config.rules {
        if let Some(schema_str) = &rule.matchers.event_schema {
            let schema = serde_json::from_str(schema_str)?;  // ❌ Parse JSON
            let validator = JSONSchema::compile(&schema)?;   // ❌ Compile schema
            if !validator.is_valid(&serde_json::to_value(event)?) {
                return Response::block("Schema validation failed");
            }
        }
    }
}

// With 100 rules, each with schema: 100 * 0.5ms = 50ms ❌ 5x OVER BUDGET
```

**Consequences:**
- p95 latency exceeds 10ms target
- Processing time scales linearly with rule count
- User-visible slowdown on every Claude Code interaction
- May trigger Claude Code's hook timeout (default unknown)

**Prevention strategy:**

1. **Pre-compile schemas at config load time:**
   ```rust
   use jsonschema::JSONSchema;
   use once_cell::sync::OnceCell;

   pub struct Rule {
       // ... existing fields ...

       #[serde(skip)]  // Don't serialize this field
       pub compiled_schema: OnceCell<JSONSchema>,
   }

   impl Config {
       pub fn load(path: Option<&Path>) -> Result<Self> {
           let mut config: Config = /* ... load from YAML ... */;

           // Pre-compile all schemas
           for rule in &mut config.rules {
               if let Some(schema_str) = &rule.matchers.event_schema {
                   let schema_value = serde_json::from_str(schema_str)?;
                   let validator = compile_schema(&schema_value)?;  // From Pitfall 1
                   rule.compiled_schema.set(validator).unwrap();
               }
           }

           Ok(config)
       }
   }
   ```

2. **Benchmark schema validation overhead:**
   ```rust
   // benches/schema_validation.rs
   use criterion::{black_box, criterion_group, criterion_main, Criterion};

   fn bench_schema_validation(c: &mut Criterion) {
       let config = Config::load(Some(Path::new("fixtures/100-rules.yaml"))).unwrap();
       let event = fixture_event("pre-tool-use.json");

       c.bench_function("validate event with 100 schemas", |b| {
           b.iter(|| {
               black_box(hooks::process_event(&event, &config))
           });
       });
   }

   criterion_group!(benches, bench_schema_validation);
   criterion_main!(benches);
   ```

3. **Add performance regression test to CI:**
   ```yaml
   # .github/workflows/ci.yml
   - name: Benchmark schema validation
     run: |
       cargo bench --bench schema_validation -- --save-baseline main
       if [ -f target/criterion/*/new/estimates.json ]; then
         LATENCY=$(jq '.mean.point_estimate' target/criterion/*/new/estimates.json)
         if (( $(echo "$LATENCY > 10000000" | bc -l) )); then  # 10ms in nanoseconds
           echo "::error::Schema validation exceeds 10ms budget: ${LATENCY}ns"
           exit 1
         fi
       fi
   ```

4. **Fail config load on schema compilation errors:**
   ```rust
   // Don't defer errors to runtime - catch at startup
   if rule.compiled_schema.get().is_none() {
       return Err(format!(
           "Failed to compile schema for rule '{}': invalid schema syntax",
           rule.name.unwrap_or("<unnamed>".to_string())
       ));
   }
   ```

**Warning signs:**
- `cargo bench` shows increasing latency with rule count
- Flamegraph shows `jsonschema::compile` in hot path
- `Config::load()` time is constant but event processing time scales with rules
- Memory usage spikes during event processing (schema allocation)

**Detection:**
```bash
# Profile with 100-rule config
cargo build --release
hyperfine --warmup 3 \
  'echo "{\"hook_event_name\":\"PreToolUse\"}" | target/release/rulez'

# Should be <10ms; if >50ms, schemas aren't cached
```

**Phase mapping:** Phase 1 (JSON Schema Integration) MUST implement schema caching.

---

### Pitfall 3: Debug CLI Event State Contamination

**Severity:** HIGH - Correctness

**What goes wrong:** Adding `UserPromptSubmit` event simulation to debug CLI without proper state isolation causes cross-event state leakage and unreproducible bugs.

**Why it happens:** Event simulators (discrete event simulation) maintain global state (clock, event queue, system state). If the debug CLI shares state between `rulez debug` invocations, previous event side-effects (e.g., prompt text stored in static memory, regex cache pollution) leak into subsequent tests.

**Real-world patterns (from research):**
- Discrete Event Simulation requires: clock, priority queue, state updates committed immediately
- Event emulators need programmatic event generation without shared mutable state
- Amazon EventBridge's `evb-cli` uses correlation IDs to debug event flow

**RuleZ-specific scenario:**
```rust
// WRONG: Global state shared across debug invocations
lazy_static! {
    static ref REGEX_CACHE: Mutex<HashMap<String, Regex>> = Mutex::new(HashMap::new());
    static ref LAST_PROMPT: Mutex<Option<String>> = Mutex::new(None);  // ❌ Leaks across tests
}

// Test 1: Simulate UserPromptSubmit with prompt "delete database"
$ rulez debug user-prompt-submit --prompt "delete database"
# REGEX_CACHE now contains patterns matched, LAST_PROMPT = Some("delete database")

// Test 2: Simulate PreToolUse without prompt
$ rulez debug pre-tool-use --command "git push"
# ❌ LAST_PROMPT still contains "delete database" from Test 1
# Rules that check prompt will unexpectedly match!
```

**Consequences:**
- Debug mode produces different results than production
- Test isolation violations cause flaky tests
- Unbounded REGEX_CACHE (v1.3 tech debt) grows indefinitely across debug invocations
- Cannot reproduce production bugs in debug mode

**Prevention strategy:**

1. **Reset global state between debug invocations:**
   ```rust
   pub fn debug_event(event_type: EventType, params: DebugParams) -> Result<Response> {
       // Clear caches before processing
       REGEX_CACHE.lock().unwrap().clear();

       // Build isolated event context
       let event = build_event(event_type, params)?;
       let config = Config::load(None)?;  // Load fresh config

       // Process with clean state
       process_event(&event, &config)
   }
   ```

2. **Implement LRU cache with size limit (addresses v1.3 tech debt):**
   ```rust
   use lru::LruCache;
   use std::num::NonZeroUsize;

   lazy_static! {
       static ref REGEX_CACHE: Mutex<LruCache<String, Regex>> = {
           let cache_size = NonZeroUsize::new(100).unwrap();  // Max 100 regexes
           Mutex::new(LruCache::new(cache_size))
       };
   }
   ```

3. **Use correlation IDs for debug tracing:**
   ```rust
   pub struct DebugContext {
       correlation_id: Uuid,
       event_chain: Vec<EventType>,  // Track event sequence
   }

   // Log with correlation ID
   info!(
       correlation_id = %ctx.correlation_id,
       "Processing event {} in debug mode",
       event.hook_event_name
   );
   ```

4. **Add `--clean` flag to force fresh state:**
   ```bash
   # Default: reuse caches (faster, but may have state leakage)
   rulez debug pre-tool-use --command "git push"

   # Explicit clean state (slower, guaranteed isolation)
   rulez debug --clean pre-tool-use --command "git push"
   ```

5. **Validate state isolation in tests:**
   ```rust
   #[test]
   fn test_debug_state_isolation() {
       // Pollute state
       let _r1 = debug_event(
           EventType::UserPromptSubmit,
           DebugParams { prompt: Some("delete database".to_string()), ..Default::default() }
       );

       // Verify clean state
       let r2 = debug_event(
           EventType::PreToolUse,
           DebugParams { command: Some("git push".to_string()), ..Default::default() }
       );

       // r2 should NOT have access to "delete database" prompt
       assert!(!r2.matched_rules.iter().any(|r| r.contains("delete")));
   }
   ```

**Warning signs:**
- Debug results differ from production for identical events
- `rulez debug` output changes based on invocation order
- REGEX_CACHE size grows unbounded (check with `cargo flamegraph`)
- Cannot reproduce user-reported bugs with `rulez debug`

**Detection:**
```bash
# Test state isolation
rulez debug user-prompt-submit --prompt "secret data" > /tmp/out1.json
rulez debug pre-tool-use --command "echo test" > /tmp/out2.json

# out2.json should NOT contain "secret data"
grep -i "secret" /tmp/out2.json && echo "STATE LEAK DETECTED"
```

**Phase mapping:** Phase 2 (Debug CLI Improvements) MUST implement state isolation.

---

### Pitfall 4: E2E Test Tempfile Path Resolution Across Platforms

**Severity:** HIGH - Test Reliability

**What goes wrong:** E2E tests using `assert_cmd` and `tempfile` fail on Windows or in CI due to incorrect path resolution, symlink handling, or tempfile cleanup race conditions.

**Why it happens:** `assert_cmd::Command::write_stdin()` uses paths relative to `env::current_dir`, NOT `Command::current_dir`. Windows uses backslashes and has different tempfile locations. Symlinks behave differently on macOS vs Linux.

**Real-world evidence (from research):**
- `assert_cmd` docs: "Paths relative to `env::current_dir`, not `Command::current_dir`"
- GitHub Actions matrix builds: Windows/Unix have path separator and line ending incompatibilities
- Cross-compilation: "Windows and Unix have compatibility issues developers should actively handle rather than ignore"

**RuleZ-specific scenario (from `e2e_git_push_block.rs`):**
```rust
fn setup_claude_code_event(config_name: &str, command: &str) -> (tempfile::TempDir, String) {
    let temp_dir = setup_test_env(config_name);
    let cwd = temp_dir.path().to_string_lossy().to_string();  // ❌ May have symlinks

    let event = serde_json::json!({
        "cwd": cwd,  // ❌ On macOS, /var -> /private/var symlink confuses path matching
        // ...
    });

    (temp_dir, serde_json::to_string(&event).unwrap())
}

// Later...
let output = Command::cargo_bin("rulez")
    .current_dir(temp_dir.path())  // ❌ Different from env::current_dir
    .write_stdin(event_json)
    .output()?;
```

**Consequences on different platforms:**
- **macOS:** `/var/folders/...` is symlink to `/private/var/folders/...`, causing cwd mismatch
- **Windows:** `C:\Users\...` vs `C:/Users/...` (backslash/forward slash)
- **Linux (tmpfs):** `/tmp` cleanup race if test runner is too fast
- **CI:** Parallel test execution causes tempdir conflicts

**Prevention strategy:**

1. **Canonicalize paths before comparing:**
   ```rust
   use std::fs;

   fn setup_claude_code_event(config_name: &str, command: &str) -> (tempfile::TempDir, String) {
       let temp_dir = setup_test_env(config_name);

       // Resolve symlinks (macOS /var -> /private/var)
       let canonical_path = fs::canonicalize(temp_dir.path())
           .unwrap_or_else(|_| temp_dir.path().to_path_buf());

       let cwd = canonical_path.to_string_lossy().to_string();

       let event = serde_json::json!({
           "cwd": cwd,
           // ...
       });

       (temp_dir, serde_json::to_string(&event).unwrap())
   }
   ```

2. **Use platform-agnostic path handling:**
   ```rust
   use std::path::PathBuf;

   // WRONG: String manipulation
   let config_path = format!("{}/.claude/hooks.yaml", cwd);  // ❌ Breaks on Windows

   // RIGHT: Path API
   let config_path = PathBuf::from(&cwd)
       .join(".claude")
       .join("hooks.yaml");
   ```

3. **Ensure tempdir cleanup with explicit drop:**
   ```rust
   #[test]
   fn test_e2e_git_push_blocked() {
       let (temp_dir, event_json) = setup_claude_code_event("block-all-push.yaml", "git push");

       let output = Command::cargo_bin("rulez")
           .current_dir(temp_dir.path())
           .write_stdin(event_json)
           .output()
           .expect("command should run");

       assert_eq!(output.status.code(), Some(2));

       // Explicit cleanup before test ends
       drop(temp_dir);  // Force cleanup, don't wait for scope exit
   }
   ```

4. **Add cross-platform test matrix:**
   ```yaml
   # .github/workflows/e2e.yml
   jobs:
     e2e-tests:
       strategy:
         matrix:
           os: [ubuntu-latest, macos-latest, windows-latest]
       runs-on: ${{ matrix.os }}
       steps:
         - name: Run E2E tests
           run: cargo test --test e2e_* -- --nocapture
   ```

5. **Test symlink handling explicitly:**
   ```rust
   #[test]
   #[cfg(unix)]
   fn test_e2e_symlink_cwd() {
       use std::os::unix::fs::symlink;

       let temp_dir = tempfile::tempdir().unwrap();
       let symlink_dir = tempfile::tempdir().unwrap();

       // Create symlink to temp_dir
       let symlink_path = symlink_dir.path().join("link");
       symlink(temp_dir.path(), &symlink_path).unwrap();

       // Event cwd points to symlink
       let event = json!({
           "hook_event_name": "PreToolUse",
           "cwd": symlink_path.to_string_lossy(),
           // ...
       });

       // Should still find .claude/hooks.yaml via canonical path
       let output = Command::cargo_bin("rulez")
           .current_dir(&symlink_path)
           .write_stdin(serde_json::to_string(&event).unwrap())
           .output()
           .unwrap();

       assert!(output.status.success());
   }
   ```

**Warning signs:**
- E2E tests pass locally but fail in CI
- Tests fail only on macOS or Windows
- Flaky tests with "file not found" errors
- Tempdir cleanup errors in CI logs

**Detection:**
```bash
# Test on all platforms
cargo test --test e2e_* -- --nocapture

# macOS: Check for symlink resolution
ls -la /var/folders  # Should see symlink

# Windows: Check path separators
echo %TEMP%  # Should use backslashes
```

**Phase mapping:** Phase 3 (E2E Test Fixes) MUST handle cross-platform paths.

---

### Pitfall 5: Tauri 2.0 webkit2gtk Version Conflict in CI

**Severity:** CRITICAL - Build Failures

**What goes wrong:** Tauri 2.0 requires `libwebkit2gtk-4.1-dev` but Ubuntu 24.04 removed `libwebkit2gtk-4.0-dev`, causing CI builds to fail with "dependency not found" errors.

**Why it happens:** Tauri v1 used webkit2gtk 4.0, but Tauri v2 migrated to webkit2gtk 4.1 for Flatpak support. Ubuntu 24.04 and Debian 13 dropped the 4.0 packages from repositories.

**Real-world evidence (from research):**
- GitHub Issue #9662: "libwebkit2gtk-4.0 not available in Ubuntu 24 & Debian 13 repositories"
- Tauri docs: "For Ubuntu, Tauri 2.0 requires webkit2gtk-4.1-dev (works on Ubuntu 22.04+)"
- Migration guide: "Tauri v2 migrated to webkit2gtk-4.1 as a result of aiming to add flatpak support"

**RuleZ CI scenario:**
```yaml
# .github/workflows/ci.yml
jobs:
  build-tauri:
    runs-on: ubuntu-latest  # Uses ubuntu-24.04 by default
    steps:
      - name: Install dependencies
        run: |
          sudo apt-get update
          sudo apt-get install -y libwebkit2gtk-4.0-dev  # ❌ Package not found on 24.04
```

**Error message:**
```
E: Package 'libwebkit2gtk-4.0-dev' has no installation candidate
Error: Process completed with exit code 100.
```

**Consequences:**
- All Tauri CI builds fail on ubuntu-latest
- Cannot test Tauri UI changes in pull requests
- Forced to use older Ubuntu runners (22.04) which are deprecated
- Manual local builds work but CI is broken

**Prevention strategy:**

1. **Use correct webkit2gtk version for Tauri 2.0:**
   ```yaml
   # .github/workflows/tauri-build.yml
   jobs:
     build-tauri:
       runs-on: ubuntu-22.04  # Explicit version, supports both 4.0 and 4.1
       steps:
         - name: Install Tauri 2.0 dependencies
           run: |
             sudo apt-get update
             sudo apt-get install -y \
               libwebkit2gtk-4.1-dev \  # Tauri 2.0 requirement
               libgtk-3-dev \
               libayatana-appindicator3-dev \
               librsvg2-dev \
               curl \
               wget \
               file \
               libssl-dev
   ```

2. **Pin runner OS version explicitly:**
   ```yaml
   # WRONG: Uses latest (currently 24.04)
   runs-on: ubuntu-latest

   # RIGHT: Explicit version for stability
   runs-on: ubuntu-22.04
   ```

3. **Add fallback for webkit2gtk-4.0 if building both v1 and v2:**
   ```yaml
   - name: Install webkit2gtk (version-aware)
     run: |
       # Try 4.1 first (Tauri v2), fallback to 4.0 (Tauri v1)
       sudo apt-get install -y libwebkit2gtk-4.1-dev || \
       sudo apt-get install -y libwebkit2gtk-4.0-dev
   ```

4. **Test on multiple Ubuntu versions:**
   ```yaml
   strategy:
     matrix:
       os: [ubuntu-22.04, ubuntu-24.04]
       include:
         - os: ubuntu-22.04
           webkit: libwebkit2gtk-4.1-dev
         - os: ubuntu-24.04
           webkit: libwebkit2gtk-4.1-dev
   ```

5. **Document system dependencies in README:**
   ```markdown
   ## Tauri UI Development

   ### Ubuntu 22.04+ / Debian 12+
   ```bash
   sudo apt-get install libwebkit2gtk-4.1-dev libgtk-3-dev \
     libayatana-appindicator3-dev librsvg2-dev
   ```

   ### Ubuntu 20.04 / Debian 11 (NOT SUPPORTED)
   Tauri 2.0 requires webkit2gtk-4.1 which is not available on older distributions.
   Upgrade to Ubuntu 22.04+ or use Tauri 1.x.
   ```

**Warning signs:**
- CI builds fail with "Package 'libwebkit2gtk-4.0-dev' has no installation candidate"
- Local builds work but GitHub Actions fail
- Tauri build succeeds on macOS/Windows but fails on Linux
- `cargo build` in rulez-ui/ fails with webkit linker errors

**Detection:**
```bash
# Check available webkit versions
apt-cache search webkit2gtk

# Should see:
# - libwebkit2gtk-4.1-dev (Tauri v2)
# On Ubuntu 24.04, libwebkit2gtk-4.0-dev is MISSING

# Test Tauri build locally
cd rulez-ui
cargo tauri build  # Should complete without webkit errors
```

**Phase mapping:** Phase 4 (Tauri CI Setup) MUST use webkit2gtk-4.1.

---

### Pitfall 6: GitHub Actions Rust Cache Invalidation on Binary Rename

**Severity:** HIGH - CI Performance + Stale Artifacts

**What goes wrong:** Renaming binary from `cch` to `rulez` leaves stale cached binaries in `~/.cargo/bin/`, causing tests to execute old code or CI to upload wrong artifacts.

**Why it happens:** GitHub Actions `Swatinem/rust-cache` caches the entire `target/` directory and `~/.cargo/bin/`. When a binary is renamed, the cache contains both old (`cch`) and new (`rulez`) binaries, but CI scripts may execute the wrong one.

**Real-world evidence (from research):**
- rust-cache action "removes old binaries that were present before the action ran"
- Cache invalidation: "each repo is limited to 10GB total cache size, which fills quickly with whole target/ directory"
- Binary caching: "compiled binaries cached in `~/.cargo-install/<crate-name>`, expire after 7 days of inactivity"

**RuleZ-specific history (from project context):**
> "RuleZ already had CI issues: binary rename from cch to rulez caused stale binary artifacts"

**CI failure scenario:**
```yaml
# .github/workflows/ci.yml
jobs:
  test:
    runs-on: ubuntu-latest
    steps:
      - uses: Swatinem/rust-cache@v2  # Restores cache with old 'cch' binary

      - name: Build
        run: cargo build --release --bin rulez  # Builds new 'rulez' binary

      - name: Test
        run: cargo test  # ❌ Tests may invoke cached 'cch' via $PATH

      - name: Upload artifact
        uses: actions/upload-artifact@v4
        with:
          name: rulez-binary
          path: target/release/rulez  # ✓ Correct binary

      # BUT: If another job uses 'cch' in scripts, fails
      - name: E2E test
        run: |
          cch --version  # ❌ Uses stale cached binary, not 'rulez'
```

**Consequences:**
- Tests execute against old code, false positives
- CI uploads wrong binary to releases
- E2E tests fail with "command not found: rulez"
- Debugging wastes hours on cache invalidation

**Prevention strategy:**

1. **Explicitly clear cache on binary rename:**
   ```yaml
   # AFTER renaming binary, force cache invalidation
   - name: Clear Rust cache
     run: |
       rm -rf ~/.cargo/bin/cch
       rm -rf target/release/cch
       rm -rf target/debug/cch
       cargo clean
   ```

2. **Use cache key versioning:**
   ```yaml
   - uses: Swatinem/rust-cache@v2
     with:
       # Include binary name in cache key
       key: rulez-v2-${{ hashFiles('**/Cargo.lock') }}
       # When binary renames, key changes, cache invalidates
   ```

3. **Always use `cargo run` instead of bare binary name:**
   ```yaml
   # WRONG: Executes whatever is in $PATH (may be cached)
   - run: rulez --version

   # RIGHT: Executes freshly built binary
   - run: cargo run --bin rulez -- --version
   ```

4. **Validate binary in CI before tests:**
   ```yaml
   - name: Validate binary
     run: |
       EXPECTED_NAME="rulez"
       ACTUAL_PATH=$(which rulez || echo "")

       if [ -z "$ACTUAL_PATH" ]; then
         echo "Binary not found in PATH"
         exit 1
       fi

       if [[ "$ACTUAL_PATH" != *"$EXPECTED_NAME"* ]]; then
         echo "Wrong binary in PATH: $ACTUAL_PATH"
         exit 1
       fi

       # Check version contains expected build info
       cargo run --bin rulez -- --version | grep -q "rulez" || exit 1
   ```

5. **Remove old binaries in CI setup:**
   ```yaml
   - name: Cleanup old binaries
     run: |
       # Remove any previously installed binaries
       rm -f ~/.cargo/bin/cch
       rm -f ~/.cargo/bin/rulez

       # Force rebuild
       cargo build --release --bin rulez
   ```

**Warning signs:**
- CI test results don't match local test results
- "command not found" errors for newly renamed binary
- `which rulez` shows wrong path in CI
- Artifact uploads contain old binary name
- Tests pass in CI but features don't work in release

**Detection:**
```bash
# Locally test binary name
cargo build --release
ls -lh target/release/ | grep -E "(cch|rulez)"

# Should only see 'rulez', NOT 'cch'

# In CI, validate:
which rulez
rulez --version
# Should NOT find 'cch'
```

**Phase mapping:** Phase 3 (E2E Test Fixes) MUST validate binary artifacts.

---

## Moderate Pitfalls

### Pitfall 7: JSON Schema `allOf` Misuse with `#[serde(flatten)]`

**Severity:** MEDIUM - Correctness

**What goes wrong:** Using `#[serde(flatten)]` to map JSON Schema's `allOf` construct is incorrect and can result in structs where no valid data deserializes correctly.

**Why it happens:** JSON Schema's `allOf` applies ALL constraints (intersection), but Serde's `flatten` merges fields (union). They have opposite semantics.

**Real-world evidence (from research):**
> "Using `#[serde(flatten)]` to map JSON Schema's `allOf` construct is wrong and can result in structs for which no data results in valid deserialization or serializations that don't match the given schema."

**Example:**
```yaml
# JSON Schema with allOf (intersection semantics)
event_schema:
  $schema: "http://json-schema.org/draft-07/schema#"
  allOf:
    - properties:
        tool_name: { type: "string" }
      required: ["tool_name"]
    - properties:
        tool_input: { type: "object" }
      required: ["tool_input"]
  # Valid data MUST have both tool_name AND tool_input
```

```rust
// WRONG: Serde flatten (union semantics)
#[derive(Deserialize)]
struct Event {
    #[serde(flatten)]
    base: BaseEvent,  // Has tool_name

    #[serde(flatten)]
    extended: ExtendedEvent,  // Has tool_input

    // Problem: If fields overlap, last one wins (not intersection)
}
```

**Prevention:**
- Don't use `#[serde(flatten)]` for `allOf` schemas
- Validate with `jsonschema` crate instead of Serde deserialization
- If using Serde, manually validate constraints after deserialization

**Phase mapping:** Phase 1 (JSON Schema Integration) - Document this in schema guidelines.

---

### Pitfall 8: Broken Pipe on Linux from Unread stdio in Tests

**Severity:** MEDIUM - Test Reliability

**What goes wrong:** CLI tests that spawn processes but don't read stdout/stderr cause SIGPIPE on Linux, resulting in non-zero exit codes and test failures.

**Why it happens (from project context):**
> "piped-but-unread stdio caused broken pipe on Linux"

**Example:**
```rust
// WRONG: Doesn't read stdout
let mut child = Command::cargo_bin("rulez")
    .stdin(Stdio::piped())
    .stdout(Stdio::piped())  // ❌ Pipe created but never read
    .spawn()?;

child.stdin.as_mut().unwrap().write_all(event_json.as_bytes())?;
let status = child.wait()?;  // ❌ SIGPIPE if rulez writes to stdout

// RIGHT: Read stdout before waiting
let output = child.wait_with_output()?;  // Reads and returns stdout/stderr
```

**Prevention:**
- Always use `wait_with_output()` if stdout/stderr are piped
- Or use `Stdio::null()` if output is not needed
- Never create pipe without reading it

**Phase mapping:** Phase 3 (E2E Test Fixes) - Audit all test spawn() calls.

---

### Pitfall 9: Debug CLI Flag Proliferation Without Subcommands

**Severity:** MEDIUM - UX

**What goes wrong:** Adding flags for each event type (`--pre-tool-use`, `--post-tool-use`, `--user-prompt-submit`, etc.) creates complex, hard-to-use CLI.

**Why it happens:** Extending existing `rulez debug` with flags seems simpler than refactoring to subcommands.

**Better design:**
```bash
# WRONG: Flag proliferation
rulez debug --event-type user-prompt-submit --prompt "text" --cwd /path

# RIGHT: Subcommands
rulez debug user-prompt-submit --prompt "text" --cwd /path
rulez debug pre-tool-use --command "git push" --cwd /path
```

**Prevention:**
- Use `clap` subcommands from the start
- Event types are naturally subcommands, not flags
- Easier to add help text per event type

**Phase mapping:** Phase 2 (Debug CLI Improvements) - Use subcommands, not flags.

---

## Technical Debt Patterns

| Shortcut | Immediate Benefit | Long-term Cost | When Acceptable |
|----------|-------------------|----------------|-----------------|
| Skip JSON Schema draft validation | Faster implementation | Breaking changes on schema updates, silent failures | Never (fail-closed requirement) |
| Compile schemas on every event | No caching complexity | 5-10x performance penalty, exceeds latency budget | Only in MVP with <10 rules |
| Share state across debug invocations | Faster execution (cache reuse) | Flaky tests, unreproducible bugs | Only with `--clean` flag option |
| Use `ubuntu-latest` in CI | Automatic updates | Breaking changes when GitHub updates runner OS | Never for system dependencies (webkit) |
| Use string paths instead of PathBuf | Less typing | Cross-platform failures, Windows incompatibility | Only in examples, never in core code |
| Skip cross-platform E2E tests | Faster CI | Platform-specific bugs reach production | Only if CI time >30min (currently <5min) |

## Integration Gotchas

| Integration | Common Mistake | Correct Approach |
|-------------|----------------|------------------|
| `jsonschema` crate | Recompile schema on every validation | Pre-compile at config load, store in `OnceCell<JSONSchema>` |
| `assert_cmd` testing | Use relative paths for stdin | Canonicalize paths, use `PathBuf`, test on Windows |
| GitHub Actions rust-cache | Assume cache invalidates on binary rename | Explicit cleanup or versioned cache keys |
| Tauri 2.0 Linux builds | Install `libwebkit2gtk-4.0-dev` | Use `libwebkit2gtk-4.1-dev` on Ubuntu 22.04+ |
| Debug CLI with global state | Share REGEX_CACHE across invocations | Clear state or use LRU cache with size limit |

## Performance Traps

| Trap | Symptoms | Prevention | When It Breaks |
|------|----------|------------|----------------|
| Schema compilation in hot path | Latency scales with rule count | Pre-compile at config load | >20 rules with schemas |
| Unbounded REGEX_CACHE | Memory grows indefinitely | LRU cache with max 100 entries | Long-running processes (daemons) |
| JSON Schema draft mismatch | Validation silently fails | Require `$schema` field, fail if missing | User upgrades `jsonschema` crate |
| Tempfile symlink non-resolution | macOS tests pass, CI fails | `fs::canonicalize()` before path comparison | macOS `/var` symlink |
| Broken pipe on unread stdio | Tests fail with SIGPIPE on Linux | Always `wait_with_output()` if pipes exist | Linux CI runners |

## "Looks Done But Isn't" Checklist

- [ ] **JSON Schema validation:** Tested with all supported drafts (07, 2020-12), NOT just latest
- [ ] **Schema performance:** Benchmarked with 100+ rules, stays <10ms p95
- [ ] **Debug CLI state:** Verified no cross-invocation state leakage with automated test
- [ ] **E2E tests:** Run on Linux, macOS, Windows in CI matrix (not just locally)
- [ ] **Tauri builds:** CI uses `webkit2gtk-4.1-dev`, tested on Ubuntu 22.04 AND 24.04
- [ ] **Binary rename:** Validated no stale artifacts with explicit `which rulez` check in CI
- [ ] **Cross-platform paths:** Used `PathBuf` everywhere, tested Windows backslashes
- [ ] **Regex cache:** LRU limit enforced, tested cache eviction with >100 unique patterns

## Recovery Strategies

| Pitfall | Recovery Cost | Recovery Steps |
|---------|---------------|----------------|
| Schema draft incompatibility | LOW | Add `$schema` validation to config load, reject unsupported drafts |
| Schema compilation overhead | MEDIUM | Refactor to `OnceCell<JSONSchema>`, add benchmarks to CI |
| Debug state contamination | LOW | Add `REGEX_CACHE.clear()` at start of debug command |
| E2E path resolution | MEDIUM | Refactor to use `fs::canonicalize()`, add Windows CI job |
| Tauri webkit dependency | LOW | Update CI to use `libwebkit2gtk-4.1-dev`, pin runner to `ubuntu-22.04` |
| Stale binary cache | MEDIUM | Clear `~/.cargo/bin/`, invalidate cache key, rebuild |
| Broken pipe in tests | LOW | Replace `.spawn()` + `.wait()` with `.output()` or `.wait_with_output()` |

## Pitfall-to-Phase Mapping

| Pitfall | Prevention Phase | Verification |
|---------|------------------|--------------|
| P1: JSON Schema draft incompatibility | Phase 1 (Schema Integration) | Config validation rejects missing `$schema`, unit tests for draft-07 and 2020-12 |
| P2: Schema validation performance | Phase 1 (Schema Integration) | Criterion benchmark shows <10ms with 100 rules |
| P3: Debug CLI state contamination | Phase 2 (Debug CLI) | Integration test verifies state isolation between invocations |
| P4: E2E tempfile paths | Phase 3 (E2E Fixes) | CI matrix runs on ubuntu/macos/windows, all pass |
| P5: Tauri webkit dependency | Phase 4 (Tauri CI) | CI build succeeds on ubuntu-22.04 and ubuntu-24.04 |
| P6: Stale binary cache | Phase 3 (E2E Fixes) | CI validates binary name with `which rulez` before tests |
| P7: `allOf` with `flatten` | Phase 1 (Schema Integration) | Documentation explicitly warns against this pattern |
| P8: Broken pipe on Linux | Phase 3 (E2E Fixes) | All tests use `wait_with_output()`, Linux CI passes |
| P9: Debug CLI flag proliferation | Phase 2 (Debug CLI) | CLI uses subcommands, help text is clear and concise |

## Sources

**JSON Schema (HIGH confidence):**
- [GSoC 2026: JSON Schema Compatibility Checker](https://github.com/json-schema-org/community/issues/984)
- [The Last Breaking Change - JSON Schema Blog](https://json-schema.org/blog/posts/the-last-breaking-change)
- [Rust and JSON Schema: odd couple or perfect strangers](https://ahl.dtrace.org/2024/01/22/rust-and-json-schema/)
- [jsonschema crate - docs.rs](https://docs.rs/jsonschema)
- [jsonschema - crates.io: Rust Package Registry](https://crates.io/crates/jsonschema)
- [GitHub - Stranger6667/jsonschema: A high-performance JSON Schema validator for Rust](https://github.com/Stranger6667/jsonschema)

**Tauri 2.0 Dependencies (HIGH confidence):**
- [libwebkit2gtk-4.0 not available in Ubuntu 24 - GitHub Issue #9662](https://github.com/tauri-apps/tauri/issues/9662)
- [Tauri v2 Prerequisites](https://v2.tauri.app/start/prerequisites/)
- [Migration to webkit2gtk-4.1 on Linux port](https://v2.tauri.app/blog/tauri-2-0-0-alpha-3/)
- [Problem with WebKitGTK · tauri-apps/tauri · Discussion #9088](https://github.com/tauri-apps/tauri/discussions/9088)
- [GitHub | Tauri](https://v2.tauri.app/distribute/pipelines/github/)
- [GitHub - tauri-apps/tauri-action: Build your Web application as a Tauri binary](https://github.com/tauri-apps/tauri-action)

**CLI Testing (HIGH confidence):**
- [Testing - Command Line Applications in Rust](https://rust-cli.github.io/book/tutorial/testing.html)
- [assert_cmd documentation](https://docs.rs/assert_cmd/latest/assert_cmd/)
- [stdin handling API issue - GitHub #73](https://github.com/assert-rs/assert_cmd/issues/73)
- [Master Cross-Platform Development in Rust | Windows, macOS, Linux Guide](https://codezup.com/building-cross-platform-tools-rust-guide-windows-macos-linux/)
- [How I test Rust command-line apps with assert_cmd](https://alexwlchan.net/2025/testing-rust-cli-apps-with-assert-cmd/)

**Rust CLI Best Practices (MEDIUM confidence):**
- [Guide: Building Beautiful & User-Friendly Rust CLI Tools](https://gist.github.com/g1ibby/786cc16cc981090abb6692d5d40a6e1b)
- [5 Tips for Writing Small CLI Tools in Rust - Pascal's Scribbles](https://deterministic.space/rust-cli-tips.html)
- [dialoguer - Rust](https://docs.rs/dialoguer)

**GitHub Actions (MEDIUM confidence):**
- [Rust Cache Action](https://github.com/Swatinem/rust-cache)
- [GitHub Actions Matrix Builds](https://oneuptime.com/blog/post/2026-01-25-github-actions-matrix-builds/view)
- [Tauri GitHub Actions](https://v2.tauri.app/distribute/pipelines/github/)

**Discrete Event Simulation (MEDIUM confidence):**
- [Discrete-event simulation - James Hanlon](https://www.jameswhanlon.com/discrete-event-simulation.html)
- [Event-Driven Architecture: The Hard Parts - Three Dots Labs](https://threedots.tech/episode/event-driven-architecture/)

**Project Context (HIGH confidence):**
- RuleZ v1.3 Milestone Audit (internal)
- RuleZ CI history: binary rename, broken pipe issues (project context)
- Existing E2E tests: `e2e_git_push_block.rs` (codebase)

---

**Last Updated:** 2026-02-10
**Next Review:** After Phase 1 implementation (validate performance assumptions with benchmarks)
