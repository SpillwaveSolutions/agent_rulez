# Project Research Summary

**Project:** RuleZ UI Desktop App v1.5 Production Features
**Domain:** Desktop Application - Policy Configuration Management (Tauri 2.0 + React 18)
**Researched:** 2026-02-10
**Confidence:** HIGH

## Executive Summary

RuleZ UI v1.5 transforms the validated M1 scaffold (Tauri 2.0 + React 18 + Monaco Editor) into a production-ready desktop application by adding four critical features: log viewing with virtual scrolling, settings persistence, live file watching, and configuration diffing. The research establishes that these additions follow well-documented patterns (TanStack Virtual for large lists, Tauri plugins for native features, Zustand for state) but introduces specific risks that must be addressed: Monaco bundle duplication (can cause 2.4 MB bloat), JSONL IPC bottlenecks (5-30 second freezes with large logs), and Linux inotify limits (silent failures with file watching).

The recommended approach builds incrementally on the M1 foundation using separate Zustand stores for each domain (logs, config, UI), implements streaming/pagination for log data, watches specific files rather than directories, and maintains the dual-mode architecture for E2E testing. Each new feature integrates cleanly with existing patterns — log viewer adds a new RightPanel tab, settings enhance existing uiStore, file watching extends configStore, and diff view reuses the read/write infrastructure.

Key risks center on performance (bundle size, serialization overhead, memory leaks) and cross-platform consistency (WebView differences, PATH separator handling, inotify limits). The research provides concrete prevention strategies: Vite deduplication for Monaco, streaming APIs with events for logs, polling fallbacks for file watching, auto-waiting selectors for E2E tests, and platform-aware path handling. All strategies are validated against 2026 documentation and real-world evidence from GitHub issues, Tauri discussions, and community patterns.

## Key Findings

### Recommended Stack

Research identifies NEW libraries needed for v1.5 production features, all chosen for integration with the validated M1 scaffold (React 18.3.1, Tauri 2.0, Monaco 4.7.0, Zustand 5.0.3). The focus is on lightweight, well-maintained libraries that follow established patterns.

**Core technologies:**
- **@tanstack/react-virtual (^3.13.18):** Headless virtual scrolling for log viewer — lightweight (10-15kb), handles 100K+ rows at 60fps, variable-height row support for JSONL metadata. Chosen over react-window for flexibility with log entry complexity.
- **tauri-plugin-fs-watch (2.0):** Cross-platform file watching via Tauri ecosystem — wraps notify crate (industry standard used by rust-analyzer, cargo-watch), integrates with Tauri permission system. Essential for live config reload without polling waste.
- **tauri-plugin-store (2.0):** Secure settings persistence — native encrypted storage with file permissions, better than localStorage for desktop apps. Integrates with Zustand persist middleware for seamless state management.
- **notify (^6.1.1):** Rust file watcher backend — cross-platform (Linux inotify, macOS FSEvents, Windows ReadDirectoryChangesW), efficient event debouncing. Already used by Tauri CLI internally.
- **date-fns (^3.3.1):** Log timestamp formatting — 2kb gzipped vs 67kb moment.js, immutable API, TypeScript-native. For parsing JSONL timestamps in log viewer.
- **react-joyride (^2.9.3):** Onboarding tours — mature (5.1k stars), accessible (WCAG 2.1), simpler API than Intro.js. For first-run experience.
- **similar (2.4):** Rust diff algorithm for config comparison — used in git-diff, handles line-by-line comparison. Powers side-by-side config diff view.

**Critical decision: NO new Monaco dependency** — use built-in `registerCompletionItemProvider` API for YAML autocomplete instead of additional library. Prevents bundle bloat.

### Expected Features

**Must have (table stakes):**
- **YAML autocomplete for RuleZ fields** — Users expect schema-driven completions (standard in VS Code, IntelliJ). Monaco's built-in API supports this without new dependencies.
- **Inline validation with error tooltips** — Red squiggles and hover details are universal IDE patterns. Schema already exists; wire to Monaco's `setModelMarkers` API.
- **Audit log viewer with filtering** — Policy management tools require audit trails (Microsoft Purview, Oracle Audit). Must handle 10K-100K entries efficiently with virtual scrolling.
- **Import/export configurations** — Universal in config management (VS Code profiles, Visual Studio .vsconfig). Simple YAML file operations via Tauri.
- **Binary path configuration** — Desktop apps calling CLI tools must allow custom paths (Git GUI, Docker Desktop). Settings panel with file picker.
- **Error recovery for missing binary** — Graceful degradation when dependencies absent. Show actionable error with install instructions.

**Should have (competitive):**
- **Rule correlation in audit viewer** — Click log entry → jump to rule definition in editor. Differentiator requiring YAML AST parsing with line numbers.
- **Inline documentation on hover** — Monaco's hover provider showing field descriptions from schema. Reduces need for external docs.
- **Template gallery with preview** — 10-15 pre-built rule templates (block dangerous ops, inject context). Accelerates onboarding (Scrut has 75+ templates; we target focused set).
- **Config diff view** — Side-by-side comparison of global vs project configs. Common in network tools (SolarWinds, ManageEngine).
- **First-run wizard** — Multi-step onboarding (binary setup, template selection, test). Critical for reducing time to first working rule (<10 minutes target).

**Defer (v2+):**
- **Rule evaluation timeline** — Visual timeline showing which rules fired with timing. Requires CLI changes (not in scope for v1.5).
- **Real-time multi-user editing** — "Google Docs for rules" creates extreme complexity, conflicts with file-based storage. Use Git for collaboration.
- **Cloud sync/backup** — Adds infrastructure, privacy concerns. Document Git/Dropbox patterns instead.
- **AI-powered rule generation** — Unpredictable output, LLM integration complexity. Comprehensive templates suffice.

### Architecture Approach

v1.5 maintains the M1 dual-mode architecture (Tauri IPC with web fallbacks) while adding production features as clean extensions: new Zustand stores for isolated domains (logStore for audit data, separate from configStore/uiStore), new Tauri command modules (logs.rs, settings.rs), and new component groups (logs/, settings/, diff/). Zero breaking changes to M1 scaffold.

**Major components:**
1. **Log Viewer (logs/)** — Virtual scrolling list displaying JSONL audit logs via TanStack Virtual. Integrates as new RightPanel tab. Streams data in 1000-entry chunks via Tauri events to avoid IPC bottleneck (critical: loading 100K entries as single JSON serialization causes 5-30 second freeze). Backed by new logStore with filter state (level, outcome, rule names, date range).

2. **Settings Persistence (settings/)** — Tauri Store plugin wrapping native encrypted storage. Integrates with existing uiStore via persist middleware. Stores theme, editor font size, auto-save preferences, custom binary path. Settings UI accessible from header menu. Storage location platform-specific (macOS: ~/Library/Application Support, Linux: ~/.config, Windows: AppData/Roaming).

3. **File Watching (enhanced configStore)** — notify crate via tauri-plugin-fs watches specific config files (NOT directories to avoid Linux inotify exhaustion). Emits Tauri events on change, triggers reload in configStore. Includes polling fallback for Linux when inotify limit hit. Shows toast notification on external edits. Cleanup via unlisten() on file close to prevent memory leaks.

4. **Config Diffing (diff/)** — Uses similar crate for text diffing, Monaco diff editor for rendering. Compares global (~/.claude/hooks.yaml) vs project (.claude/hooks.yaml) side-by-side with color-coded changes. "Compare" button in sidebar when both configs exist. Diff computation ~5ms for typical config (50-100 lines).

**Key architectural pattern: Streaming over bulk loading** — All large data (logs, potentially large configs) use pagination/streaming instead of single IPC invoke to avoid JSON serialization bottleneck. Example: read_logs_page(offset, limit) instead of read_all_logs().

### Critical Pitfalls

1. **Monaco bundle duplication (CRITICAL - Performance)** — Bundling monaco-yaml separately causes 2+ MB of duplicate monaco-editor code, 3-5 second load times, and non-functional autocomplete. Prevention: Configure Vite to deduplicate with alias (`'monaco-editor': 'monaco-editor/esm/vs/editor/editor.api'`), use vite-plugin-monaco-editor, verify with `bun why monaco-editor` showing single instance. Detection: Bundle size >2 MB, error "Unexpected usage at EditorSimpleWorker.loadForeignModule". Phase 1 MUST configure this.

2. **JSONL IPC bottleneck (CRITICAL - Performance)** — Loading 100K line JSONL logs (10 MB) via single Tauri invoke causes 5-30 second freeze due to JSON serialization (10 MB file → 20 MB Vec<LogEntry> → 30 MB JSON → 60 MB JS objects). Prevention: Implement streaming with pagination (read_logs_page with 1000 entry chunks), use Tauri events for streaming (emit log-chunk), add TanStack Virtual for rendering. Phase 2 MUST implement streaming.

3. **Linux inotify exhaustion (CRITICAL - Production)** — Watching directories with >8192 files exhausts Linux inotify limit (fs.inotify.max_user_watches), causing silent file watching failures. Works on macOS/Windows (different mechanisms). Prevention: Watch specific files NOT directories (RecursiveMode::NonRecursive), add polling fallback, check limits at startup, show warning if watch fails with ENOSPC. Phase 3 MUST use file-specific watches.

4. **Playwright E2E flakiness (HIGH - Test reliability)** — Monaco's async module loading causes "element not found" errors in CI due to WebView timing differences (WKWebView 200ms, WebView2 1200ms, WebKitGTK 800ms). Prevention: Replace all `waitForTimeout()` with auto-waiting selectors (`page.waitForSelector('.monaco-editor .view-lines')`), add custom Monaco matchers, serialize test runs in CI (`--workers=1`). Phase 4 MUST eliminate timeouts.

5. **Windows PATH separator issues (HIGH - Correctness)** — Using Unix `:` separator breaks binary detection on Windows (uses `;`). Prevention: Use `path.delimiter` (platform-aware), check for .exe/.cmd extensions on Windows, use `which` library for cross-platform path resolution. Phase 5 MUST test on Windows.

## Implications for Roadmap

Based on research, the following phase structure addresses feature dependencies, mitigates pitfalls, and builds incrementally on M1 scaffold:

### Phase 1: Monaco Editor Enhancements (YAML Autocomplete)
**Rationale:** Foundation for all editor-based features. Must configure Monaco bundle properly BEFORE other features add complexity. Schema-driven autocomplete is table stakes (users expect this from VS Code experience).

**Delivers:**
- JSON Schema integration with monaco-yaml
- Custom completion provider for RuleZ-specific fields (inject_inline, enabled_when, etc.)
- Inline validation error markers
- Hover documentation for YAML fields

**Addresses:** YAML autocomplete (must-have), inline validation (must-have), inline documentation (competitive)

**Avoids:** Pitfall #1 (Monaco bundle duplication) — MUST configure Vite deduplication, verify bundle size <1.5 MB

**Research flag:** Standard pattern (Monaco API well-documented), skip phase-specific research

### Phase 2: Audit Log Viewer with Virtual Scrolling
**Rationale:** Independent from editor features, provides critical visibility into rule behavior. Must implement streaming/pagination from start to avoid refactoring later.

**Delivers:**
- JSONL log parser with pagination (1000 entries/chunk)
- Virtual scrolling via TanStack Virtual (handles 100K+ entries)
- Filter controls (level, outcome, rule name, date range)
- Log stats display (total entries, time range)
- "Logs" tab in RightPanel

**Addresses:** Audit log viewer (must-have), filtering (must-have)

**Avoids:** Pitfall #2 (JSONL IPC bottleneck) — MUST implement streaming with events, not bulk loading

**Research flag:** Standard pattern (virtual scrolling well-documented), skip phase-specific research

### Phase 3: Settings Persistence and File Watching
**Rationale:** These features work together (settings store custom binary path, file watching uses it). Both extend existing stores (uiStore, configStore) without new infrastructure.

**Delivers:**
- Tauri Store plugin integration
- Settings persistence (theme, font, auto-save, binary path)
- File watcher for config files (specific files, not directories)
- Live config reload on external edits
- Toast notifications for file changes
- Settings UI panel

**Addresses:** Binary path config (must-have), error recovery (must-have), theme persistence (table stakes)

**Avoids:** Pitfall #3 (inotify exhaustion) — watch specific files, add polling fallback; Pitfall #6 (memory leaks) — cleanup listeners in useEffect

**Research flag:** Moderate complexity (file watching cross-platform nuances) — consider `/gsd:research-phase` for Linux inotify handling

### Phase 4: Configuration Diffing
**Rationale:** Builds on existing file reading infrastructure. Simple addition using Monaco's built-in diff editor.

**Delivers:**
- Side-by-side config diff view (global vs project)
- Diff stats (additions, deletions, modifications)
- "Compare" button in sidebar
- Color-coded change highlighting

**Addresses:** Config diff view (competitive)

**Avoids:** No critical pitfalls — straightforward Monaco API usage

**Research flag:** Standard pattern (Monaco diff editor documented), skip phase-specific research

### Phase 5: Template Gallery and First-Run Experience
**Rationale:** Onboarding features come after core functionality works. Templates provide fast path to first working rule.

**Delivers:**
- 10-15 pre-built rule templates
- Template preview with live YAML
- First-run wizard (binary detection, template selection, test)
- Onboarding tour with react-joyride
- Template insertion into editor

**Addresses:** Template gallery (competitive), first-run wizard (competitive), snippets (should-have)

**Avoids:** Pitfall #5 (Windows PATH) — test binary detection on all platforms

**Research flag:** Standard pattern (joyride well-documented), skip phase-specific research

### Phase 6: E2E Test Stabilization
**Rationale:** Critical for CI reliability before distribution. Must eliminate flakiness before production release.

**Delivers:**
- Remove all `waitForTimeout()` calls
- Add auto-waiting selectors for Monaco
- Custom Playwright matchers for Monaco state
- Consistent WebView in CI (ubuntu-22.04)
- Test retry logic for known-flaky interactions
- 100% pass rate over 10 runs

**Addresses:** Test reliability (internal quality)

**Avoids:** Pitfall #4 (E2E flakiness) — MUST replace timeouts with auto-waiting

**Research flag:** Requires phase-specific research (`/gsd:research-phase`) — Monaco async loading timing, WebView differences need deeper investigation

### Phase 7: Cross-Platform Polish and Distribution
**Rationale:** Final phase ensures consistent experience across platforms and sets up deployment.

**Delivers:**
- WebView rendering normalization (scrollbars, fonts)
- Windows binary path handling
- Linux inotify limit documentation
- Tauri updater plugin setup (requires code signing)
- Platform-specific CI testing (macOS, Windows, Linux)
- Binary path detection fallbacks

**Addresses:** Cross-platform consistency, distribution

**Avoids:** Pitfall #5 (Windows PATH), Pitfall #9 (WebView differences)

**Research flag:** Moderate complexity (code signing costs, platform differences) — consider `/gsd:research-phase` for auto-updater setup

### Phase Ordering Rationale

- **Phase 1 first:** Monaco configuration must be correct before adding features that depend on it. Bundle size regression would be costly to fix later.
- **Phase 2-3 parallel possible:** Log viewer and settings/file-watching are independent domains with separate Zustand stores. Can be developed concurrently.
- **Phase 4 after 3:** Config diff uses file reading infrastructure validated in file watching.
- **Phase 5 after 1-4:** Onboarding depends on core features working (editor, logs, settings).
- **Phase 6 before 7:** Must have stable tests before distribution to avoid shipping flaky builds.
- **Phase 7 last:** Polish and distribution depend on all features complete.

### Research Flags

**Phases likely needing deeper research during planning:**
- **Phase 3 (File Watching):** Linux inotify limits, cross-platform differences, resource exhaustion patterns need investigation beyond general docs
- **Phase 6 (E2E Stabilization):** Monaco async loading timing, WebView rendering differences, Playwright with Tauri needs platform-specific research
- **Phase 7 (Distribution):** Code signing costs ($300-500/year for certificates), auto-updater security, platform-specific packaging

**Phases with standard patterns (skip research-phase):**
- **Phase 1 (Monaco):** Well-documented Monaco API, Vite deduplication is established pattern
- **Phase 2 (Logs):** TanStack Virtual has comprehensive docs, JSONL parsing is straightforward
- **Phase 4 (Diff):** Monaco diff editor API well-documented, similar crate usage clear
- **Phase 5 (Onboarding):** react-joyride has clear examples, template patterns standard

## Confidence Assessment

| Area | Confidence | Notes |
|------|------------|-------|
| Stack | HIGH | All libraries verified in 2026 docs with active maintenance. TanStack Virtual 3.13.18, Tauri plugins 2.0, notify 6.1.1 all confirmed. Version compatibility validated. |
| Features | MEDIUM | Feature expectations based on VS Code (HIGH), policy management tools (MEDIUM), competitor analysis (MEDIUM). Some features (rule correlation, timeline) less validated. |
| Architecture | HIGH | Integration patterns follow existing M1 scaffold (proven working). Dual-mode maintained, Zustand store separation clear, Tauri IPC patterns established. |
| Pitfalls | HIGH | All critical pitfalls sourced from GitHub issues, Tauri discussions, real production failures. Prevention strategies verified in 2026 documentation. |

**Overall confidence:** HIGH

Research is comprehensive with verified sources. Main uncertainty is feature prioritization (what users value most) rather than technical implementation. Architecture and stack choices follow proven patterns.

### Gaps to Address

**During planning:**
- **Binary detection edge cases:** Handle Rust installed via rustup vs package manager vs manual. Research suggests ~/.cargo/bin, but Windows scoop/chocolatey installs may differ. Validate during Phase 5 with Windows CI.
- **Log volume limits:** Research covers 100K lines, but what happens at 1M+ lines? Consider lazy loading with backend indexing if users report issues post-launch.
- **Monaco schema validation performance:** Schema complexity may slow down validation on large files. Monitor in Phase 1; consider debouncing if users report lag.

**During implementation:**
- **File watching debounce timing:** Research suggests 500ms, but optimal timing depends on user editing patterns. Make configurable in settings if users report issues (too fast = conflicts, too slow = perceived lag).
- **E2E test environment:** Playwright with Tauri needs validation. Research indicates Playwright-CDP exists, but M1 tests already working suggests web mode sufficient. Validate in Phase 6.

## Sources

### Primary (HIGH confidence)
- [Tauri 2.0 Official Documentation](https://v2.tauri.app/) — IPC architecture, plugin system, security scope, WebView versions
- [TanStack Virtual Documentation](https://tanstack.com/virtual/latest) — Virtual scrolling API, performance benchmarks, v3.13.18 release notes
- [Monaco Editor API](https://microsoft.github.io/monaco-editor/) — Completion provider API, diff editor, marker system
- [monaco-yaml GitHub Repository](https://github.com/remcohaszing/monaco-yaml) — Schema integration, bundle duplication issues (#214), v5.3.1 compatibility
- [Zustand Documentation](https://zustand.docs.pmnd.rs/) — Store patterns, persist middleware, cleanup best practices
- [notify crate documentation](https://docs.rs/notify/latest/notify/) — Cross-platform file watching, inotify limits, platform differences
- [Playwright Documentation](https://playwright.dev/) — Auto-waiting, WebView testing, flaky test prevention

### Secondary (MEDIUM confidence)
- [BrowserStack Playwright Flaky Tests Guide (2026)](https://www.browserstack.com/guide/playwright-flaky-tests) — Timeout patterns, retry strategies
- [Visual Studio 2026 Settings Experience](https://devblogs.microsoft.com/visualstudio/a-first-look-at-the-all%E2%80%91new-ux-in-visual-studio-2026/) — Config management UX patterns
- [Policy Management Software Comparison](https://peoplemanagingpeople.com/tools/best-policy-management-software/) — Audit trail requirements, template galleries
- [Zustand GitHub Discussions](https://github.com/pmndrs/zustand/discussions) — Memory leak patterns (#2540), subscriber cleanup (#2054)
- [cross-platform-node-guide](https://github.com/ehmicky/cross-platform-node-guide) — PATH separator handling, environment variables

### Tertiary (LOW confidence)
- [tauri-plugin-fs-watch version verification](https://crates.io/) — Could not verify exact 2.0 version due to website JS blocking. Assumed 2.0 based on Tauri 2.0 ecosystem. **Verify with `cargo search tauri-plugin-fs-watch` before adding.**
- [react-joyride accessibility](https://www.npmjs.com/package/react-joyride) — WCAG 2.1 compliance claimed but not independently verified

---
*Research completed: 2026-02-10*
*Ready for roadmap: yes*
