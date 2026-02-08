# Codebase Concerns

**Analysis Date:** 2026-02-06

## Tech Debt

### Phase 2 Governance Implementation Incomplete
- **Issue:** PolicyMode, trust levels, and governance metadata are implemented in models (`cch_cli/src/models.rs`) but only partially integrated into the core processing logic. While Phase 2.2 governance logging exists, mode-specific behavior (enforce/warn/audit) is not fully executed in all code paths.
- **Files:**
  - `cch_cli/src/models.rs` (defines PolicyMode, Confidence, TrustLevel enums)
  - `cch_cli/src/hooks.rs` (execute_rule_actions_with_mode() at line 150)
  - `cch_cli/src/cli/explain.rs` (incomplete mode explanation output)
- **Impact:** Users may enable rule modes but not see expected behavior in all scenarios. The feature set is larger than implementation completeness.
- **Fix approach:**
  1. Complete execute_rule_actions_with_mode() implementation for all action types (inject, run, block)
  2. Add integration tests covering mode behavior per scenario (enforce vs warn vs audit)
  3. Update CLI commands to fully report mode state in explain/debug commands
  4. Document mode interaction rules in CLAUDE.md

### Schema Validation Not Integrated into UI
- **Issue:** `public/schema/hooks-schema.json` referenced in CLAUDE.md ("to be implemented in M3") but not yet present. RuleZ UI validates YAML structure but lacks comprehensive JSON Schema validation.
- **Files:**
  - `rulez_ui/src/lib/yaml-utils.ts` (validates YAML syntax only)
  - `rulez_ui/src/components/editor/ValidationPanel.tsx` (basic validation, no schema enforcement)
  - `rulez_ui/CLAUDE.md` (mentions "to be implemented in M3")
- **Impact:** Users cannot get IDE-like schema hints while editing. Rule definitions can be syntactically valid YAML but semantically invalid for CCH.
- **Fix approach:**
  1. Create comprehensive hooks-schema.json with rule structure, matchers, actions
  2. Integrate with monaco-yaml for live schema validation and autocomplete
  3. Update ValidationPanel to surface schema errors
  4. Add schema version to Config struct for forward compatibility

### Unwrap/Panic Calls in Test Code
- **Issue:** Multiple `unwrap()` calls in test and initialization code. While safe in tests, these indicate lack of proper error propagation patterns in some areas.
- **Files:**
  - `cch_cli/src/models.rs` (lines in serialization tests)
  - `cch_cli/src/logging.rs` (JSON serialization operations)
- **Impact:** Test failures can panic rather than return errors, making debugging harder. Masquerades as safety when error handling is actually incomplete.
- **Fix approach:**
  1. Replace test unwraps with proper Result handling and test assertions
  2. Use `expect()` only where panics are truly acceptable
  3. Audit logging.rs for runtime unwraps and convert to proper error returns
  4. Add clippy rule forbidding unwrap in non-test code

### Hard-coded Performance Constants
- **Issue:** Performance targets (5ms p95 cold start, <50MB resident memory, <10ms rule matching) are defined in constitution.md but not enforced by tests or benchmarks. No automated performance regression detection.
- **Files:**
  - `.speckit/constitution.md` (lines 188-192, 336-348)
  - `cch_cli/src/` (no performance test infrastructure)
- **Impact:** Performance targets can silently degrade without detection. Dependencies on fast startup and rule matching are not validated in CI.
- **Fix approach:**
  1. Implement benchmark suite in cch_cli/benches/ using criterion
  2. Add PQ (Performance Qualification) tests to CI on main/develop
  3. Create baseline performance metrics per platform
  4. Set CI checks to fail on >10% latency regression

---

## Known Bugs

### Port 1420 Collision in Development Workflow
- **Symptoms:** Running `task run-app` fails if port 1420 is already in use from previous Tauri dev session. Error: "Cannot bind to port 1420".
- **Files:** `Taskfile.yml` (lines 718f739 commit: "fix: auto-kill port 1420 before starting run-app task")
- **Trigger:**
  1. Run `task run-app` to start Tauri dev environment
  2. Kill terminal without properly stopping dev server
  3. Immediately run `task run-app` again
- **Workaround:** `lsof -ti :1420 | xargs kill -9` then retry, or restart system

### Playwright E2E Test Selector Fragility
- **Symptoms:** 21 Playwright E2E tests were failing with broken selectors (commit 6baf216).
- **Files:** `rulez_ui/tests/` (fixed with data-testid selectors)
- **Trigger:** Component refactoring, CSS class changes, or structural changes without updating selectors
- **Current status:** Fixed in commit 6baf216, but indicates need for stable selector strategy
- **Workaround:** Run tests with `--headed` to debug selector mismatches before committing

### Divide-by-Zero in Memory Stability Test
- **Symptoms:** PQ memory stability test could crash if memory measurements are zero or uniform.
- **Files:** `cch_cli/tests/pq_memory.rs` (fixed in commit 72a87db)
- **Trigger:** Unlikely in production but occurred in test execution
- **Current status:** Fixed with guard against divide-by-zero

---

## Security Considerations

### External Script Execution Trust Model Incomplete
- **Risk:** `run` action executes validator scripts with configurable trust levels (Local, Verified, Untrusted) but trust enforcement is incomplete. `TrustLevel::Untrusted` scripts still execute if referenced in rules.
- **Files:**
  - `cch_cli/src/models.rs` (TrustLevel enum at line 98)
  - `cch_cli/src/hooks.rs` (execute_script() allows all trust levels, line ~530)
  - `cch_cli/src/cli/install.rs` (trust warning output incomplete)
- **Current mitigation:**
  - Timeout protection (5s default, configurable)
  - Controlled environment variables
  - Script path validation
- **Recommendations:**
  1. Implement TrustLevel blocking: reject Untrusted scripts unless explicitly enabled
  2. Add allowlist of trusted script paths
  3. Sandbox script execution with restricted environment
  4. Log script execution with provenance for audit trail
  5. Add `--allow-untrusted-scripts` flag to CLI for explicit override

### Configuration File Permissions Not Enforced
- **Risk:** `~/.claude/hooks.yaml` and `.claude/hooks.yaml` are world-readable if user has permissive umask. No validation that config file permissions prevent unauthorized modification.
- **Files:**
  - `cch_cli/src/cli/init.rs` (creates config files)
  - `cch_cli/src/config.rs` (Config::load())
- **Current mitigation:** Users must manage file permissions manually
- **Recommendations:**
  1. In init.rs, chmod config files to 0600 (user read/write only)
  2. In Config::load(), warn if config has overly permissive permissions
  3. Document recommended umask in USER_GUIDE_CLI.md
  4. Consider refusing to load configs with world-readable permissions (fail-closed)

### No Input Validation on Event JSON
- **Risk:** `process_event()` accepts Event from stdin without verifying structure. Malformed JSON could cause panics or unexpected behavior.
- **Files:**
  - `cch_cli/src/main.rs` (reads stdin as JSON)
  - `cch_cli/src/models.rs` (Event struct)
- **Current mitigation:** serde_json error handling, but minimal schema validation
- **Recommendations:**
  1. Add JSON Schema validation to Event deserialization
  2. Reject events missing required fields (hook_event_name, session_id)
  3. Validate command/tool_name against known safe patterns
  4. Add size limits on event payloads (prevent DoS via huge context)

### Regex Injection Vulnerability in Rule Matching
- **Risk:** User-supplied command_patterns are compiled as regex without validation. Malicious patterns could cause ReDoS (Regular Expression Denial of Service).
- **Files:**
  - `cch_cli/src/hooks.rs` (matches_command_pattern())
  - `cch_cli/src/config.rs` (line 1: `#![allow(clippy::regex_creation_in_loops)]`)
- **Current mitigation:** Default timeout of 5s per script execution
- **Recommendations:**
  1. Validate regex patterns on config load, reject patterns with known DoS characteristics
  2. Use regex with timeout/size limits during matching
  3. Document safe regex pattern guidelines in CONFIG_GUIDE.md
  4. Add rule-level timeout override for regex matching

---

## Performance Bottlenecks

### Regex Compilation in Inner Loop
- **Problem:** `#![allow(clippy::regex_creation_in_loops)]` in config.rs suggests regex patterns are compiled for every rule check, not cached.
- **Files:** `cch_cli/src/config.rs` (line 1), `cch_cli/src/hooks.rs` (matches_rule implementation)
- **Cause:** Config object recompiled on each event, regex patterns recompiled per match attempt
- **Impact:** Latency increases with rule count. Target: <10ms per event, but N regex compilations per check could exceed this at >100 rules
- **Improvement path:**
  1. Implement regex caching: compile patterns once during Config::load() and reuse
  2. Create CompiledRule struct with pre-compiled Regex fields
  3. Measure improvement with benches/regex-perf.rs
  4. Set threshold alerts if cache miss rate > 1%

### UI YAML Parsing on Every Edit
- **Problem:** YamlEditor component likely re-parses entire YAML on every character input
- **Files:** `rulez_ui/src/components/editor/YamlEditor.tsx` (line 156)
- **Cause:** No debounce or incremental parsing strategy
- **Impact:** Editor input latency >100ms on large configs (10+KB YAML)
- **Improvement path:**
  1. Add debounce(300ms) to YAML parsing
  2. Implement incremental parsing to update validation without full reparse
  3. Measure editor input latency with ProfilerMarker API
  4. Keep <16ms input-to-feedback latency (60fps requirement)

### Tauri File I/O Synchronous Blocking
- **Problem:** RuleZ UI file operations may block the main thread if Tauri commands are not async
- **Files:** `rulez_ui/src-tauri/src/commands/` (config.rs, debug.rs)
- **Cause:** Insufficient investigation of async patterns in Tauri 2.0
- **Impact:** UI freezes during file save/load, desktop app responsiveness <60fps
- **Improvement path:**
  1. Audit Tauri command implementations for async patterns
  2. Convert blocking I/O to async/await in Rust backend
  3. Add timeout to file operations (5s max)
  4. Show progress indicator for file operations >500ms

---

## Fragile Areas

### CCH Core Event Processing Pipeline
- **Files:**
  - `cch_cli/src/main.rs` (JSON parsing from stdin)
  - `cch_cli/src/hooks.rs` (evaluate_rules, execute_rule_actions_with_mode)
  - `cch_cli/src/config.rs` (Config::load with fallback hierarchy)
- **Why fragile:**
  - Config loading depends on event.cwd field from Claude Code; if missing or incorrect, wrong config loaded
  - Regex matching against user input can timeout or panic on pathological patterns
  - Log file I/O failures are silently ignored (line 86: `let _ = log_entry(entry).await`)
  - No recovery path if configuration is corrupted (only blank config fallback)
- **Safe modification:**
  1. Always test with edge cases: missing cwd, empty event, huge command strings
  2. Add integration tests for config resolution scenarios (project + global, global only, neither)
  3. Use explicit Result types instead of ignoring logging errors
  4. Log critical errors to stderr even if structured logging fails
- **Test coverage:**
  - Config resolution: tested in config.rs line 172-331
  - Rule matching: limited testing, needs more edge case coverage
  - Event processing: relies on integration tests, unit coverage is thin

### RuleZ UI Tauri Desktop Integration
- **Files:**
  - `rulez_ui/src/lib/tauri.ts` (IPC command wrappers)
  - `rulez_ui/src-tauri/src/commands/` (Rust implementation)
  - `rulez_ui/src/components/editor/YamlEditor.tsx` (Monaco integration)
- **Why fragile:**
  - Dual-mode architecture (Tauri + web) may have divergent behavior
  - Monaco editor integration is complex and CSS can break easily
  - Error handling between frontend and Rust backend is inconsistent
  - No validation that Tauri app and CCH binary versions match
- **Safe modification:**
  1. Test all file operations in BOTH Tauri and web mode
  2. Use Page Object Model pattern for E2E tests (already done)
  3. Version-lock Tauri and Monaco versions in package.json
  4. Add explicit error boundaries and fallback UI for Tauri failures
- **Test coverage:**
  - Tauri IPC: 55 lines in tauri.test.ts, needs expansion for error cases
  - E2E: improved in commit 6baf216, but selector stability still at risk
  - Web mode: minimal testing, could regress

### Configuration Validation Logic
- **Files:** `cch_cli/src/config.rs` (validate method line 123-143)
- **Why fragile:**
  - Validation only checks rule name uniqueness and format, not semantic validity
  - Regex compilation in validation could fail and mask actual syntax errors
  - No validation of matcher/action combinations (e.g., block + inject rarely make sense)
  - Can't validate that referenced files (for inject action) actually exist
- **Safe modification:**
  1. Add comprehensive validation suite with semantic checks
  2. Separate syntax validation (YAML parsing) from semantic validation (rule logic)
  3. Return detailed error messages pinpointing invalid fields
  4. Add `--warnings` flag to report likely mistakes (e.g., block_if_match with no matchers)
- **Test coverage:**
  - Basic validation tested in config.rs tests
  - No tests for invalid matcher combinations or file existence

---

## Scaling Limits

### Configuration Load Time Linear with Rule Count
- **Current capacity:** ~100 rules load in <100ms
- **Limit:** At 1000+ rules, Config::load() could exceed 5ms target (with regex compilation)
- **Scaling path:**
  1. Implement lazy regex compilation only for rules that might match
  2. Add config file size limit (e.g., max 10MB) to prevent accidental large configs
  3. Implement rule indexing by tool/extension to skip non-matching rules
  4. Profile with cch_cli benches/config-load.rs and set regression thresholds

### Log File Rotation Not Implemented
- **Current capacity:** Logs grow unbounded at ~100KB per 1000 rules evaluated
- **Limit:** User system disk space, potential multi-GB logs after weeks of use
- **Scaling path:**
  1. Implement log rotation: daily files with max size (e.g., 100MB per day)
  2. Add `cch logs clean` command to archive/delete old logs
  3. Add log retention policy to Settings (default: 30 days)
  4. Measure log growth in PQ stress tests and set warning thresholds

### UI Editor Performance with Large Configs
- **Current capacity:** ~50KB YAML (400+ rules) editable with latency <200ms
- **Limit:** >100KB YAML causes noticeable lag, potential out-of-memory on low-end systems
- **Scaling path:**
  1. Implement config splitting: primary global config + project-specific overrides
  2. Lazy-load rules in tree view (only show first 20, load on scroll)
  3. Add file size warning in ValidationPanel (warn >50KB, error >500KB)
  4. Implement async YAML parsing with web worker

---

## Dependencies at Risk

### Monaco Editor Version Constraint
- **Package:** `@monaco-editor/react` ^4.7.0
- **Risk:** Monaco has frequent breaking changes in major versions. Current version 4.x may reach EOL.
- **Impact:** Updating Monaco could break editor layout or autocompletion
- **Migration plan:**
  1. Monitor Monaco 5.x releases and breaking changes
  2. Create isolated upgrade test suite (E2E for editor-specific functionality)
  3. Maintain upgrade fork for 1-2 major versions before mandatory upgrade
  4. Document required config changes in upgrade guide

### Tauri Version Stability
- **Package:** `@tauri-apps/cli` ^2.3.0
- **Risk:** Tauri 2.x is relatively new; migration from 1.x had breaking changes. Future 3.x unknown.
- **Impact:** Desktop builds could fail, IPC protocol changes could break backend communication
- **Migration plan:**
  1. Lock Tauri version in Cargo.toml until 2.x reaches 2.10+ (stability threshold)
  2. Create Tauri upgrade branch early with extensive testing
  3. Test on all 3 platforms (macOS, Windows, Linux) before merging
  4. Provide migration guide for users with custom Tauri extensions

### TypeScript/React Version Support
- **Packages:** `typescript` ^5.7.3, `react` ^18.3.1
- **Risk:** React 19 is stable, current ^18 version may receive security patches only. TypeScript 6 might require config changes.
- **Impact:** Security vulnerabilities in React 18, potential incompatibility with new Node.js versions
- **Migration plan:**
  1. Plan React 18→19 migration after RuleZ UI M9 completion (when API stabilizes)
  2. Test TypeScript 6 compatibility in parallel development
  3. Update CLAUDE.md with minimum supported versions before each release

---

## Missing Critical Features

### Conflict Resolution Between Global and Project Configs
- **Problem:** If both ~/.claude/hooks.yaml and .claude/hooks.yaml exist with overlapping rule names, no clear conflict resolution. Currently loads project-only and ignores global config.
- **Blocks:** Users cannot compose global policies (org standards) with project policies (team rules)
- **Solution:**
  1. Load both configs and merge rules with conflict handling
  2. Define merge strategy: project rules override global by name, or all rules active
  3. Add config merge policy setting (override, combine, merge-by-name)
  4. Update Config::load() to return merged configuration
  5. Document in USER_GUIDE_CLI.md with examples

### Rule Inheritance / Policy Packs
- **Problem:** No way to define policy templates or inherit from standard rulesets. Each project must define all rules from scratch.
- **Blocks:** Enterprise deployments cannot enforce org-wide policies that teams extend
- **Solution:**
  1. Design policy pack format (extends, imports fields in YAML)
  2. Implement policy resolution: load base pack, merge project overrides
  3. Add `cch init --template=<url>` to bootstrap from remote policy pack
  4. Maintain standard policy pack repository (e.g., spillwave/cch-policies)

### Audit Trail Immutability
- **Problem:** JSON Lines logs can be modified after creation. No cryptographic integrity verification.
- **Blocks:** Governance use case requires tamper-proof audit logs
- **Solution:**
  1. Add optional log signing with user's private key
  2. Implement HMAC-based log integrity checking
  3. Add `cch logs verify` command to validate log chain integrity
  4. Document in governance guide for compliance workflows

---

## Test Coverage Gaps

### CLI Install/Uninstall Commands Untested in E2E
- **What's not tested:**
  - `cch install` registering with Claude Code properly
  - `cch uninstall` removing all traces from Claude Code settings
  - Integration between installed CCH hook and actual Claude Code execution
- **Files:**
  - `cch_cli/src/cli/install.rs`
  - `cch_cli/src/cli/uninstall.rs`
  - Integration tests depend on Claude CLI availability
- **Risk:** Users cannot install/uninstall CCH without manual intervention, regressions go undetected
- **Priority:** High
- **Fix:**
  1. Add integration tests that invoke `cch install` and verify Claude settings.json change
  2. Add hook event injection tests to verify installed hook actually runs
  3. Test uninstall cleanup: verify settings.json reverted and logs untouched
  4. Add CI step to test on actual Claude CLI installation

### Warn Mode Behavior Untested
- **What's not tested:**
  - Rules with `mode: warn` never block but still inject warning context
  - Warn mode interaction with other modes (priority resolution)
  - Warn mode logging in both enforce and audit rules
- **Files:**
  - `cch_cli/src/hooks.rs` (execute_rule_actions_with_mode line ~200)
  - No dedicated test file for mode behavior
- **Risk:** Mode feature (Phase 2.2) could have silent bugs
- **Priority:** High
- **Fix:**
  1. Create `cch_cli/tests/oq_mode_behavior.rs` with comprehensive mode tests
  2. Add test scenarios: warn-only, enforce+warn, audit+enforce combinations
  3. Verify log decision field reflects actual behavior
  4. Add E2E tests using `cch debug` with mode-specific rules

### Tauri IPC Error Handling Edge Cases
- **What's not tested:**
  - Tauri command timeouts (what happens if CCH takes >30s?)
  - Invalid command responses (non-JSON, truncated, oversized)
  - Concurrent file operations (multiple editors writing simultaneously)
  - CCH binary not found or wrong version
- **Files:**
  - `rulez_ui/src/lib/tauri.ts` (IPC wrappers)
  - `rulez_ui/tests/` (E2E tests don't cover error paths)
- **Risk:** UI hangs, incorrect data displayed, silent failures
- **Priority:** Medium
- **Fix:**
  1. Add timeout handling tests in Playwright (retry after timeout)
  2. Mock Tauri command failures and test fallback behavior
  3. Add version mismatch detection and UI error message
  4. Test max payload size handling in both directions

### Regex Pattern Edge Cases
- **What's not tested:**
  - ReDoS (Regular Expression Denial of Service) patterns
  - Case sensitivity interactions (`.rs` vs `.RS`)
  - Unicode handling in command patterns
  - Empty or null patterns
- **Files:**
  - `cch_cli/src/hooks.rs` (matches_command_pattern)
  - No regex-specific test file
- **Risk:** Performance regression, unexpected matches
- **Priority:** Medium
- **Fix:**
  1. Create regex pattern test suite with pathological patterns
  2. Add performance benchmarks for regex matching
  3. Document regex pattern best practices
  4. Add validation warning for patterns with nested quantifiers

---

## Summary

**Critical Issues (Fix before release):**
- Phase 2 governance mode behavior incomplete
- Schema validation missing from UI
- External script trust model not enforced
- Test coverage gaps for install/uninstall and mode behavior

**High Priority (Plan next sprint):**
- Regex compilation performance and ReDoS vulnerability
- Log rotation and unbounded growth
- Port 1420 collision handling (temporary fix exists, needs permanent solution)
- Global/project config conflict resolution

**Medium Priority (Next phase):**
- Performance regression detection framework
- Policy pack / rule inheritance
- Selector fragility in E2E tests
- Audit trail immutability

**Low Priority (Backlog):**
- Memory stability under sustained load
- UI performance with very large configs (>100KB)
- Comprehensive error messages in all paths

