# Domain Pitfalls: RuleZ UI Desktop App (v1.5)

**Domain:** Tauri 2.0 Desktop App with Monaco Editor, File Watching, Log Viewing
**Project:** RuleZ Policy Engine
**Milestone:** v1.5 - Production-ready desktop UI
**Researched:** 2026-02-10
**Confidence:** HIGH

## Summary

v1.5 adds production features to existing Tauri 2.0 + React 18 + Monaco scaffold. The app reads/writes YAML config files and JSONL audit logs via Tauri IPC. This research focuses on pitfalls when **adding production hardening** to a working prototype, not building from scratch.

**Existing Foundation (v1.4):**
- Dual-mode architecture (Tauri desktop + web testing mode)
- Monaco editor scaffold (basic editing, no autocomplete)
- Basic file tree sidebar
- Playwright E2E tests (flaky/unstable)
- Tauri CI builds working

**Known Issues to Address:**
- Tauri shell scope still references "cch" binary (renamed to "rulez")
- E2E tests are flaky (timing issues, WebView differences)
- Monaco autocomplete needs implementation (monaco-yaml schema integration)
- No file watching (configs don't reload on external changes)
- No log viewing (JSONL audit logs not displayable)
- Binary path detection not implemented

**v1.5-Specific Risks:**
1. **Monaco-YAML schema integration** - Bundle size explosion, worker configuration failures
2. **Tauri IPC with large JSONL files** - JSON serialization bottleneck, memory issues
3. **File watching cross-platform** - FSEvents vs inotify differences, resource limits
4. **Playwright E2E flakiness** - Monaco async loading, WebView rendering differences
5. **Binary path detection** - Cross-platform PATH issues, permission problems
6. **Zustand state management** - Memory leaks with file watchers, stale state
7. **Desktop distribution** - Code signing costs, auto-update complexity, platform-specific issues

## Critical Pitfalls

### Pitfall 1: Monaco Editor Bundle Size Explosion with Duplicate Instances

**Severity:** CRITICAL - Performance + UX

**What goes wrong:** Bundling `monaco-yaml` and `monaco-editor` separately causes 2+ MB of duplicate code, slow initial load times, and non-functional YAML autocomplete.

**Why it happens:** Build tools (Vite, Webpack) treat `monaco-editor` as a separate dependency when imported by `monaco-yaml`. Without proper configuration, the editor appears twice in the bundle.

**Real-world evidence (2026):**
- monaco-yaml GitHub: "If monaco-editor appears twice in your bundle, this causes substantially larger bundle sizes and non-functional features"
- Diagnosis command: `npm ls monaco-editor` or `yarn why monaco-editor`
- Vite users report "Unexpected usage at EditorSimpleWorker.loadForeignModule" error

**RuleZ UI scenario:**
```typescript
// src/components/editor/ConfigEditor.tsx
import * as monaco from 'monaco-editor';  // ❌ 1.2 MB
import { configureMonacoYaml } from 'monaco-yaml';  // ❌ Another 1.2 MB (includes monaco-editor)

// Total bundle: 2.4 MB just for editor
// App load time: 3-5 seconds on slow connections
```

**Consequences:**
- Initial load takes 3-5 seconds (vs. 500ms target)
- Lighthouse performance score <50
- Schema autocomplete doesn't work (two editor instances conflict)
- Memory usage 2x higher than necessary
- Mobile Safari may refuse to load >3 MB bundles

**Prevention strategy:**

1. **Configure Vite to deduplicate monaco-editor:**
   ```typescript
   // vite.config.ts
   import { defineConfig } from 'vite';
   import monacoEditorPlugin from 'vite-plugin-monaco-editor';

   export default defineConfig({
     plugins: [
       monacoEditorPlugin({
         languageWorkers: ['editorWorkerService', 'yaml'],
         // Exclude duplicate languages
         customWorkers: [
           {
             label: 'yaml',
             entry: 'monaco-yaml/yaml.worker',
           },
         ],
       }),
     ],
     resolve: {
       alias: {
         // Force single monaco-editor instance
         'monaco-editor': 'monaco-editor/esm/vs/editor/editor.api',
       },
     },
     optimizeDeps: {
       include: ['monaco-editor', 'monaco-yaml'],
       esbuildOptions: {
         target: 'esnext',
       },
     },
   });
   ```

2. **Use dynamic imports to code-split editor:**
   ```typescript
   // src/components/editor/ConfigEditor.tsx
   import { lazy, Suspense } from 'react';

   const MonacoEditor = lazy(() => import('./MonacoEditorLazy'));

   export function ConfigEditor() {
     return (
       <Suspense fallback={<EditorSkeleton />}>
         <MonacoEditor />
       </Suspense>
     );
   }
   ```

3. **Verify bundle size in CI:**
   ```yaml
   # .github/workflows/ui-build.yml
   - name: Check bundle size
     run: |
       cd rulez-ui
       bun run build
       BUNDLE_SIZE=$(du -sb dist/assets/*.js | awk '{sum+=$1} END {print sum}')
       MAX_SIZE=$((1500 * 1024))  # 1.5 MB max for JS

       if [ $BUNDLE_SIZE -gt $MAX_SIZE ]; then
         echo "::error::Bundle size ${BUNDLE_SIZE} exceeds ${MAX_SIZE}"
         echo "Check for duplicate monaco-editor instances: bun why monaco-editor"
         exit 1
       fi
   ```

4. **Monitor monaco-editor instances at runtime:**
   ```typescript
   // src/lib/debug.ts
   export function checkMonacoDuplicates() {
     const instances = document.querySelectorAll('.monaco-editor');
     if (instances.length > 1) {
       console.error('Multiple Monaco instances detected!', instances);
       // In dev mode, throw error
       if (import.meta.env.DEV) {
         throw new Error('Duplicate Monaco instances - check bundle config');
       }
     }
   }
   ```

5. **Create custom YAML worker wrapper:**
   ```typescript
   // src/workers/yaml.worker.ts
   import 'monaco-yaml/yaml.worker';

   // Vite requires this for proper worker handling
   export default {};
   ```

   ```typescript
   // src/lib/monaco.ts
   import { MonacoEnvironment } from 'monaco-editor';

   (window as any).MonacoEnvironment = {
     getWorker(_: unknown, label: string) {
       if (label === 'yaml') {
         return new Worker(
           new URL('../workers/yaml.worker.ts', import.meta.url),
           { type: 'module' }
         );
       }
       return new Worker(
         new URL('monaco-editor/esm/vs/editor/editor.worker.js', import.meta.url),
         { type: 'module' }
       );
     },
   } as MonacoEnvironment;
   ```

**Warning signs:**
- Bundle size >2 MB for single JS file
- `bun why monaco-editor` shows multiple versions
- Console error: "EditorSimpleWorker.loadForeignModule"
- YAML autocomplete doesn't appear
- Initial load time >2 seconds

**Detection:**
```bash
# Check for duplicate monaco-editor
cd rulez-ui
bun why monaco-editor

# Should show single instance, NOT multiple paths

# Build and check size
bun run build
du -h dist/assets/*.js | sort -h

# Largest JS file should be <1.5 MB
```

**Phase mapping:** Phase 1 (Monaco Autocomplete) MUST configure Vite properly.

---

### Pitfall 2: Tauri IPC JSON Serialization Bottleneck with Large JSONL Logs

**Severity:** CRITICAL - Performance + Memory

**What goes wrong:** Loading large JSONL audit logs (>10 MB, 100K+ lines) via Tauri IPC causes 5-30 second freezes due to JSON serialization overhead and memory exhaustion.

**Why it happens:** Tauri IPC uses JSON-RPC protocol requiring all data to be JSON-serializable. Large JSONL files get parsed, deserialized to Rust structs, re-serialized to JSON, sent over IPC, then parsed again in JavaScript.

**Real-world evidence (2026):**
- Tauri Discussion #7699: "Deprecate JSON in IPC" - serialization is major bottleneck
- Benchmark: 10 MB data takes ~5ms on macOS but ~200ms on Windows
- JSON doesn't support BigInt or binary data, requires array conversion
- "When sharing large amounts of data, serializing in backend and deserializing in frontend can lead to significant costs"

**RuleZ UI scenario:**
```rust
// src-tauri/src/commands/logs.rs
#[tauri::command]
async fn load_audit_logs(path: String) -> Result<Vec<LogEntry>, String> {
    let contents = fs::read_to_string(path)?;  // 10 MB file

    let entries: Vec<LogEntry> = contents
        .lines()
        .map(|line| serde_json::from_str(line).unwrap())
        .collect();  // ❌ Allocates 20 MB (10 MB file + 10 MB Vec<LogEntry>)

    Ok(entries)  // ❌ Serializes to JSON (30 MB total), sends over IPC
}
```

```typescript
// src/lib/tauri.ts
export async function loadAuditLogs(path: string): Promise<LogEntry[]> {
  const logs = await invoke<LogEntry[]>('load_audit_logs', { path });
  // ❌ Received 30 MB JSON, parses to JS objects (60 MB total)
  return logs;
}
```

**Performance breakdown for 100K line JSONL (10 MB):**
- Rust: Read file (50ms) + Parse JSONL (100ms) + Serialize to JSON (200ms) = 350ms
- IPC: Transfer 30 MB JSON (100ms on localhost, 300ms over WebSocket)
- JS: Parse JSON (300ms) + Render in React (500ms) = 800ms
- **Total: 1.2 seconds MINIMUM, up to 5+ seconds on Windows**

**Consequences:**
- App freezes for 5-30 seconds when viewing logs
- Memory usage spikes to 200+ MB
- Out of memory errors on machines with <8 GB RAM
- UI becomes unresponsive (React can't update during JSON parse)
- Users think app crashed

**Prevention strategy:**

1. **Implement streaming with pagination:**
   ```rust
   // src-tauri/src/commands/logs.rs
   use std::io::{BufRead, BufReader};

   #[tauri::command]
   async fn load_audit_logs_page(
       path: String,
       offset: usize,
       limit: usize,
   ) -> Result<LogPage, String> {
       let file = File::open(path)?;
       let reader = BufReader::new(file);

       let entries: Vec<LogEntry> = reader
           .lines()
           .skip(offset)
           .take(limit)
           .filter_map(|line| {
               line.ok()
                   .and_then(|l| serde_json::from_str(&l).ok())
           })
           .collect();

       Ok(LogPage {
           entries,
           total: count_lines(&path)?,  // Cache this
           offset,
           limit,
       })
   }
   ```

2. **Use streaming API with async iteration:**
   ```rust
   // src-tauri/src/commands/logs.rs
   use tauri::Window;

   #[tauri::command]
   async fn stream_audit_logs(window: Window, path: String) -> Result<(), String> {
       let file = File::open(path)?;
       let reader = BufReader::new(file);

       const CHUNK_SIZE: usize = 1000;
       let mut chunk = Vec::with_capacity(CHUNK_SIZE);

       for line in reader.lines() {
           let entry: LogEntry = serde_json::from_str(&line.unwrap())?;
           chunk.push(entry);

           if chunk.len() >= CHUNK_SIZE {
               // Emit event with chunk
               window.emit("log-chunk", &chunk)?;
               chunk.clear();
           }
       }

       // Send remaining entries
       if !chunk.is_empty() {
           window.emit("log-chunk", &chunk)?;
       }

       window.emit("log-stream-complete", ())?;
       Ok(())
   }
   ```

   ```typescript
   // src/lib/tauri.ts
   import { listen } from '@tauri-apps/api/event';

   export async function streamAuditLogs(
     path: string,
     onChunk: (entries: LogEntry[]) => void,
   ): Promise<void> {
     const unlisten = await listen<LogEntry[]>('log-chunk', (event) => {
       onChunk(event.payload);
     });

     await listen('log-stream-complete', () => {
       unlisten();
     });

     await invoke('stream_audit_logs', { path });
   }
   ```

3. **Add virtual scrolling for log display:**
   ```typescript
   // src/components/logs/LogViewer.tsx
   import { useVirtualizer } from '@tanstack/react-virtual';

   export function LogViewer({ entries }: { entries: LogEntry[] }) {
     const parentRef = useRef<HTMLDivElement>(null);

     const virtualizer = useVirtualizer({
       count: entries.length,
       getScrollElement: () => parentRef.current,
       estimateSize: () => 35,  // Row height
       overscan: 10,
     });

     return (
       <div ref={parentRef} className="h-full overflow-auto">
         <div
           style={{
             height: `${virtualizer.getTotalSize()}px`,
             position: 'relative',
           }}
         >
           {virtualizer.getVirtualItems().map((item) => (
             <LogRow
               key={item.key}
               entry={entries[item.index]}
               style={{
                 position: 'absolute',
                 top: 0,
                 left: 0,
                 width: '100%',
                 transform: `translateY(${item.start}px)`,
               }}
             />
           ))}
         </div>
       </div>
     );
   }
   ```

4. **Cache parsed logs with invalidation:**
   ```rust
   // src-tauri/src/log_cache.rs
   use lru::LruCache;
   use std::sync::Mutex;

   lazy_static! {
       static ref LOG_CACHE: Mutex<LruCache<String, Vec<LogEntry>>> = {
           Mutex::new(LruCache::new(NonZeroUsize::new(5).unwrap()))
       };
   }

   pub fn get_cached_logs(path: &str, mtime: SystemTime) -> Option<Vec<LogEntry>> {
       let cache = LOG_CACHE.lock().unwrap();
       cache.peek(&format!("{}-{:?}", path, mtime)).cloned()
   }
   ```

5. **Add loading indicators and progress:**
   ```typescript
   // src/components/logs/LogLoader.tsx
   export function LogLoader({ path }: { path: string }) {
     const [progress, setProgress] = useState(0);
     const [entries, setEntries] = useState<LogEntry[]>([]);

     useEffect(() => {
       let count = 0;
       streamAuditLogs(path, (chunk) => {
         setEntries((prev) => [...prev, ...chunk]);
         count += chunk.length;
         setProgress(count);
       });
     }, [path]);

     return (
       <>
         {progress > 0 && <Progress value={progress} />}
         <LogViewer entries={entries} />
       </>
     );
   }
   ```

**Warning signs:**
- App freezes when clicking "View Logs"
- Memory usage spikes >500 MB
- Browser DevTools shows "Long Task" warnings >500ms
- React DevTools shows long render times
- Console shows "Out of memory" errors

**Detection:**
```bash
# Create large test log
seq 1 100000 | awk '{print "{\"timestamp\":\"2026-02-10T12:00:00Z\",\"event\":\"test\",\"line\":" $1 "}"}' > /tmp/large.jsonl

# Test load time
time rulez-ui --load-logs /tmp/large.jsonl

# Should be <2 seconds; if >5 seconds, pagination needed
```

**Phase mapping:** Phase 2 (Log Viewing) MUST implement streaming/pagination.

---

### Pitfall 3: File Watching Resource Exhaustion on Linux (inotify Limits)

**Severity:** CRITICAL - Production Failures

**What goes wrong:** File watching stops working when monitoring >8192 files on Linux due to inotify limits, causing configs to not reload and users to think the app is broken.

**Why it happens:** Linux inotify has per-user limits (`fs.inotify.max_user_watches`, default 8192). Watching a directory tree with many files exhausts the limit, causing silent failures.

**Real-world evidence (2026):**
- notify crate docs: "fs.inotify.max_user_watches specifies the upper limit for the number of watches per user"
- Common error: "no space left on device" or "too many open files"
- Rust notify crate: "When watching a very large amount of files, notify may fail to receive all events"
- macOS FSEvents and Windows don't have this limit (cross-platform inconsistency)

**RuleZ UI scenario:**
```rust
// src-tauri/src/file_watcher.rs
use notify::{Watcher, RecursiveMode};

#[tauri::command]
async fn watch_config_files() -> Result<(), String> {
    let mut watcher = notify::recommended_watcher(|res| {
        match res {
            Ok(event) => println!("File changed: {:?}", event),
            Err(e) => eprintln!("Watch error: {:?}", e),  // ❌ Silent failure on inotify limit
        }
    })?;

    // Watch global and project configs
    watcher.watch(Path::new("~/.claude"), RecursiveMode::Recursive)?;  // ❌ May have 10K+ files
    watcher.watch(Path::new(".claude"), RecursiveMode::Recursive)?;

    Ok(())
}
```

**When it breaks:**
- User has large `~/.claude/` directory (multiple projects, logs, cache)
- `~/.claude/logs/` contains 10K+ JSONL log files
- inotify watch count: 10K files × 1 watch = 10K watches > 8192 limit
- `watcher.watch()` **silently fails** - no error returned
- Users edit configs externally, app doesn't reload them

**Consequences:**
- File watching silently stops working on Linux
- Users edit `.claude/hooks.yaml` in external editor, RuleZ UI doesn't update
- No error message - users think app is broken
- Works on macOS/Windows (FSEvents/ReadDirectoryChangesW don't have limits)
- GitHub issues: "File watching doesn't work on Ubuntu"

**Prevention strategy:**

1. **Watch specific files, not entire directories:**
   ```rust
   // src-tauri/src/file_watcher.rs
   use notify::{Watcher, RecursiveMode};

   #[tauri::command]
   async fn watch_specific_configs(paths: Vec<String>) -> Result<(), String> {
       let mut watcher = notify::recommended_watcher(event_handler)?;

       // Watch ONLY the specific config files, NOT directories
       for path in paths {
           if Path::new(&path).is_file() {
               watcher.watch(Path::new(&path), RecursiveMode::NonRecursive)?;
           }
       }

       Ok(())
   }
   ```

2. **Use polling fallback on Linux when inotify fails:**
   ```rust
   // src-tauri/src/file_watcher.rs
   use notify::{PollWatcher, RecommendedWatcher, Watcher};
   use std::time::Duration;

   fn create_watcher() -> Result<Box<dyn Watcher>, notify::Error> {
       // Try inotify first (fast)
       match RecommendedWatcher::new(event_handler, Config::default()) {
           Ok(watcher) => Ok(Box::new(watcher)),
           Err(_) => {
               // Fallback to polling (slower but always works)
               warn!("inotify watcher failed, using polling fallback");
               let config = Config::default()
                   .with_poll_interval(Duration::from_secs(2));
               Ok(Box::new(PollWatcher::new(event_handler, config)?))
           }
       }
   }
   ```

3. **Check inotify limits at startup:**
   ```rust
   // src-tauri/src/system_check.rs
   #[cfg(target_os = "linux")]
   fn check_inotify_limits() -> Result<(), String> {
       let max_watches = fs::read_to_string("/proc/sys/fs/inotify/max_user_watches")
           .map_err(|e| format!("Cannot read inotify limits: {}", e))?
           .trim()
           .parse::<usize>()
           .unwrap_or(8192);

       if max_watches < 16384 {
           warn!(
               "inotify max_user_watches is low ({}). File watching may fail. \
                Increase with: sudo sysctl fs.inotify.max_user_watches=524288",
               max_watches
           );
       }

       Ok(())
   }
   ```

4. **Show warning when watch fails:**
   ```rust
   // src-tauri/src/file_watcher.rs
   impl FileWatcher {
       pub fn watch(&mut self, path: &Path) -> Result<(), String> {
           match self.watcher.watch(path, RecursiveMode::NonRecursive) {
               Ok(()) => Ok(()),
               Err(notify::Error::Io(e)) if e.raw_os_error() == Some(28) => {
                   // ENOSPC: No space left on device (inotify limit)
                   Err(format!(
                       "Cannot watch file (inotify limit reached). \
                        Increase fs.inotify.max_user_watches. \
                        See: https://github.com/guard/listen/wiki/Increasing-the-amount-of-inotify-watchers"
                   ))
               }
               Err(e) => Err(format!("Watch failed: {}", e)),
           }
       }
   }
   ```

5. **Document inotify increase in README:**
   ```markdown
   ## File Watching on Linux

   If file watching doesn't work, increase inotify limits:

   ```bash
   # Temporary (until reboot)
   sudo sysctl fs.inotify.max_user_watches=524288

   # Permanent
   echo "fs.inotify.max_user_watches=524288" | sudo tee -a /etc/sysctl.conf
   sudo sysctl -p
   ```
   ```

**Warning signs:**
- File watching works locally (macOS) but not in Docker (Linux)
- Configs don't reload when edited externally
- Console error: "no space left on device" but disk has free space
- `cat /proc/sys/fs/inotify/max_user_watches` shows low number
- Works with small projects, fails with large monorepos

**Detection:**
```bash
# Check current inotify limit
cat /proc/sys/fs/inotify/max_user_watches

# Should be >16384; if 8192, will hit limits

# Count current watches
find /proc/*/fd -lname anon_inode:inotify -printf '%hinfo/%f\n' 2>/dev/null | \
  xargs cat | grep -c '^inotify'

# Test watching many files
for i in {1..10000}; do touch /tmp/test_$i; done
# Try watching /tmp - should fail
```

**Phase mapping:** Phase 3 (File Watching) MUST use file-specific watches + polling fallback.

---

### Pitfall 4: Playwright E2E Flakiness with Monaco Editor Async Loading

**Severity:** HIGH - Test Reliability

**What goes wrong:** E2E tests pass locally but fail in CI with "element not found" or "element not clickable" due to Monaco editor's async module loading and WebView rendering differences.

**Why it happens:** Monaco loads language workers asynchronously, triggers re-renders, and WebView rendering timing differs from Chromium. Playwright's default timeouts don't account for Monaco's initialization sequence.

**Real-world evidence (2026):**
- BrowserStack Guide: "Flaky tests in Playwright are automated tests that pass during one execution and fail the next, even when no changes have been made to the codebase"
- Primary cause: "Artificial timeouts (`waitForTimeout()`) should be replaced with auto-waiting actions"
- Tauri WebView differences: WKWebView (macOS), WebView2 (Windows), WebKitGTK (Linux) have different rendering speeds
- Monaco issue #2755: "Autocomplete suggestions UI doesn't immediately show up when returning a promise"

**RuleZ UI test scenario:**
```typescript
// tests/editor.spec.ts (FLAKY VERSION)
test("should show YAML autocomplete", async ({ page }) => {
  await page.goto("/");

  // Click to open file
  await page.click('[data-testid="file-hooks.yaml"]');

  // Try to type in editor
  await page.click('.monaco-editor');  // ❌ Element not yet rendered
  await page.keyboard.type('rule');

  // Wait for autocomplete
  await page.waitForTimeout(500);  // ❌ Artificial timeout

  // Check autocomplete appears
  await expect(page.locator('.monaco-suggest')).toBeVisible();  // ❌ Flaky
});
```

**Failure modes:**
- **CI (Linux WebKitGTK):** Monaco takes 800ms to load, test clicks before render
- **macOS (WKWebView):** Monaco loads in 200ms, test passes
- **Windows (WebView2):** Monaco takes 1200ms, test times out
- **Parallel test runs:** Shared worker resources cause race conditions

**Consequences:**
- E2E tests fail randomly in CI (30-50% pass rate)
- Developers disable flaky tests, reducing coverage
- False negatives block valid PRs
- Debugging wastes hours (works locally, fails in CI)
- Cannot trust test results

**Prevention strategy:**

1. **Use Playwright auto-waiting instead of timeouts:**
   ```typescript
   // tests/editor.spec.ts (STABLE VERSION)
   import { expect, test } from "@playwright/test";

   test("should show YAML autocomplete", async ({ page }) => {
     await page.goto("/");

     // Wait for Monaco to fully initialize
     await page.waitForSelector('.monaco-editor .view-lines', {
       state: 'visible',
       timeout: 10000,  // Longer timeout for CI
     });

     // Click specific file
     await page.click('[data-testid="file-hooks.yaml"]');

     // Wait for editor content to load
     await expect(page.locator('.monaco-editor .view-line')).toHaveCount(
       { min: 1 },
       { timeout: 5000 }
     );

     // Focus editor (auto-waits for clickable)
     await page.click('.monaco-editor textarea');

     // Type to trigger autocomplete
     await page.keyboard.type('rules:', { delay: 50 });

     // Trigger autocomplete manually (Ctrl+Space)
     await page.keyboard.press('Control+Space');

     // Wait for suggestions widget (NOT timeout)
     await expect(page.locator('.suggest-widget')).toBeVisible({
       timeout: 3000,
     });
   });
   ```

2. **Add custom Playwright matchers for Monaco:**
   ```typescript
   // tests/helpers/monaco-matchers.ts
   import { expect as baseExpect, Locator } from '@playwright/test';

   async function toHaveMonacoText(locator: Locator, expected: string) {
     // Extract text from Monaco's DOM structure
     const lines = await locator.locator('.view-line').allTextContents();
     const text = lines.join('\n');

     return {
       pass: text.includes(expected),
       message: () => `Expected Monaco to contain "${expected}", got "${text}"`,
     };
   }

   export const expect = baseExpect.extend({
     toHaveMonacoText,
   });
   ```

3. **Stub slow operations in test mode:**
   ```typescript
   // src/lib/monaco.ts
   export async function initializeMonaco() {
     if (import.meta.env.TEST) {
       // Skip worker loading in tests
       return Promise.resolve();
     }

     // Production: Full initialization
     await import('monaco-yaml');
     // ... worker setup
   }
   ```

4. **Use consistent WebView in CI:**
   ```yaml
   # .github/workflows/e2e.yml
   jobs:
     e2e-tests:
       runs-on: ubuntu-22.04  # Consistent WebKitGTK version
       steps:
         - name: Install WebKitGTK
           run: sudo apt-get install -y libwebkit2gtk-4.1-dev

         - name: Run Playwright tests
           run: |
             cd rulez-ui
             bun run test:e2e --retries=2 --workers=1  # Serialize tests, allow retries
   ```

5. **Add retry logic for known-flaky interactions:**
   ```typescript
   // tests/helpers/retry.ts
   export async function retryUntilVisible(
     page: Page,
     selector: string,
     action: () => Promise<void>,
     maxRetries = 3,
   ): Promise<void> {
     for (let i = 0; i < maxRetries; i++) {
       await action();

       try {
         await page.waitForSelector(selector, { timeout: 2000 });
         return;  // Success
       } catch (e) {
         if (i === maxRetries - 1) throw e;
         await page.waitForTimeout(500);  // Brief pause before retry
       }
     }
   }
   ```

6. **Test Monaco initialization separately:**
   ```typescript
   // tests/monaco-init.spec.ts
   test("Monaco editor initializes within 5 seconds", async ({ page }) => {
     await page.goto("/");

     const startTime = Date.now();

     await page.waitForSelector('.monaco-editor', { state: 'attached' });
     await page.waitForSelector('.view-lines', { state: 'visible' });

     const initTime = Date.now() - startTime;

     expect(initTime).toBeLessThan(5000);  // Fail if too slow
   });
   ```

**Warning signs:**
- Tests pass 3/5 times locally, fail in CI
- "element not found" errors for `.monaco-editor` or `.suggest-widget`
- Tests with `waitForTimeout()` fail randomly
- Different pass rates on macOS vs Linux
- Parallel test runs have lower pass rate

**Detection:**
```bash
# Run tests 10 times to detect flakiness
for i in {1..10}; do
  bun run test:e2e && echo "PASS $i" || echo "FAIL $i"
done

# Should see 10/10 PASS; if <8/10, tests are flaky

# Check for artificial timeouts
grep -r "waitForTimeout" tests/
# Should find ZERO occurrences
```

**Phase mapping:** Phase 4 (E2E Test Stabilization) MUST eliminate `waitForTimeout()`.

---

### Pitfall 5: Binary Path Detection Failures on Windows (PATH Separator Issues)

**Severity:** HIGH - Correctness

**What goes wrong:** `rulez` binary detection fails on Windows due to incorrect PATH parsing, causing UI to show "binary not found" even when installed.

**Why it happens:** Windows uses semicolon (`;`) as PATH separator (vs. colon `:` on Unix), and `process.env.PATH` is case-insensitive but not normalized. Additionally, Windows executable extensions (`.exe`, `.cmd`) are implicit.

**Real-world evidence (2026):**
- cross-platform-node-guide: "Windows uses `%ENV_VAR%`, POSIX uses `$ENV_VAR`"
- PATH separator: Windows `;` vs Unix `:`
- Node.js path.delimiter: `process.platform === 'win32' ? ';' : ':'`
- Environment variables case-insensitive on Windows: `PATH`, `Path`, `path` all valid

**RuleZ UI scenario:**
```typescript
// src/lib/binary-finder.ts (WRONG)
export function findRulezBinary(): string | null {
  const paths = process.env.PATH.split(':');  // ❌ Breaks on Windows (uses ';')

  for (const dir of paths) {
    const binaryPath = path.join(dir, 'rulez');  // ❌ Missing '.exe' on Windows
    if (fs.existsSync(binaryPath)) {
      return binaryPath;
    }
  }

  return null;
}
```

**Failure on Windows:**
```
PATH = C:\Windows\System32;C:\Program Files\Rust\bin;C:\Users\alice\.cargo\bin

paths.split(':')  // ❌ Returns single element (no ':' in Windows PATH)
  => ["C:\Windows\System32;C:\Program Files\Rust\bin;C:\Users\alice\.cargo\bin"]

path.join(paths[0], 'rulez')
  => "C:\Windows\System32;C:\Program Files\Rust\bin;C:\Users\alice\.cargo\bin\rulez"
  // ❌ Invalid path

fs.existsSync(...)  // ❌ Always false
```

**Consequences:**
- UI shows "rulez binary not found" on Windows
- Simulator tab disabled (requires binary)
- Users must manually configure binary path
- Works on macOS/Linux, fails only on Windows
- GitHub issues: "Can't find rulez on Windows"

**Prevention strategy:**

1. **Use Node.js built-in path delimiter:**
   ```typescript
   // src/lib/binary-finder.ts
   import path from 'path';
   import fs from 'fs';

   export function findRulezBinary(): string | null {
     const pathEnv = process.env.PATH || '';
     const paths = pathEnv.split(path.delimiter);  // ✓ Uses ':' or ';' based on platform

     // Windows: Check PATH, Path, path (case-insensitive)
     const pathKey = Object.keys(process.env).find(
       (key) => key.toUpperCase() === 'PATH'
     );
     const pathValue = pathKey ? process.env[pathKey] : '';

     for (const dir of pathValue.split(path.delimiter)) {
       const candidates = [
         path.join(dir, 'rulez'),
         path.join(dir, 'rulez.exe'),  // Windows
         path.join(dir, 'rulez.cmd'),  // Windows
       ];

       for (const candidate of candidates) {
         if (fs.existsSync(candidate)) {
           return candidate;
         }
       }
     }

     return null;
   }
   ```

2. **Use cross-platform `which` library:**
   ```typescript
   // src/lib/binary-finder.ts
   import which from 'which';

   export async function findRulezBinary(): Promise<string | null> {
     try {
       // Automatically handles PATH parsing, extensions, case-sensitivity
       const binaryPath = await which('rulez');
       return binaryPath;
     } catch (e) {
       return null;
     }
   }
   ```

3. **Fallback to common installation paths:**
   ```typescript
   // src/lib/binary-finder.ts
   const COMMON_PATHS = [
     // Unix
     '/usr/local/bin/rulez',
     path.join(os.homedir(), '.cargo/bin/rulez'),

     // Windows
     path.join(process.env.USERPROFILE || '', '.cargo', 'bin', 'rulez.exe'),
     'C:\\Program Files\\rulez\\rulez.exe',
   ];

   export function findRulezBinary(): string | null {
     // Try PATH first
     const inPath = which.sync('rulez', { nothrow: true });
     if (inPath) return inPath;

     // Fallback to common paths
     for (const candidate of COMMON_PATHS) {
       if (fs.existsSync(candidate)) {
         return candidate;
       }
     }

     return null;
   }
   ```

4. **Test on Windows in CI:**
   ```yaml
   # .github/workflows/e2e.yml
   strategy:
     matrix:
       os: [ubuntu-22.04, macos-latest, windows-latest]

   steps:
     - name: Install rulez
       run: cargo install --path rulez

     - name: Verify binary in PATH
       shell: bash
       run: |
         which rulez || where rulez  # Unix: which, Windows: where
         rulez --version

     - name: Run UI tests
       run: |
         cd rulez-ui
         bun run test:e2e
   ```

5. **Show helpful error with install instructions:**
   ```typescript
   // src/components/simulator/BinaryCheck.tsx
   export function BinaryCheck() {
     const [binaryPath, setBinaryPath] = useState<string | null>(null);

     useEffect(() => {
       findRulezBinary().then(setBinaryPath);
     }, []);

     if (!binaryPath) {
       return (
         <Alert variant="warning">
           <AlertTitle>rulez binary not found</AlertTitle>
           <AlertDescription>
             The simulator requires the <code>rulez</code> CLI.
             <br />
             Install with: <code>cargo install --path rulez</code>
             <br />
             Or download from: <a href="...">Releases</a>
           </AlertDescription>
         </Alert>
       );
     }

     return <Simulator binaryPath={binaryPath} />;
   }
   ```

**Warning signs:**
- UI works on macOS/Linux, fails on Windows
- Console error: "rulez not found" but `where rulez` shows path
- Tests pass locally (Unix), fail in Windows CI
- Binary path detection works in Node.js but fails in Electron/Tauri
- PATH environment variable not split correctly

**Detection:**
```bash
# Test on Windows
echo %PATH%  # Should show semicolons

# Test binary detection
node -e "console.log(process.env.PATH.split(require('path').delimiter))"

# Should show multiple paths, NOT single string

# Verify rulez.exe in PATH
where rulez  # Windows
which rulez  # Unix
```

**Phase mapping:** Phase 5 (Binary Path Detection) MUST test on Windows.

---

### Pitfall 6: Zustand Store Memory Leaks with File Watcher Subscriptions

**Severity:** HIGH - Memory + Stability

**What goes wrong:** File watcher event listeners registered in Zustand stores are never cleaned up, causing memory leaks and eventually crashing the app after prolonged use.

**Why it happens:** Zustand stores persist for the application lifetime, but Tauri event listeners registered in stores create closures that capture store state. Without cleanup, listeners accumulate and old state references prevent garbage collection.

**Real-world evidence (2026):**
- Zustand Discussion #2540: "Memory leak issue when stores are created in React Context"
- Zustand Discussion #2054: "Will not cleaning up subscriber result in a memory leak? Yes."
- Zustand docs: "Use `store.destroy()` in tests to prevent state leaks"
- React: "Always return cleanup function from useEffect hooks"

**RuleZ UI scenario:**
```typescript
// src/stores/configStore.ts (LEAKY VERSION)
import { create } from 'zustand';
import { listen } from '@tauri-apps/api/event';

interface ConfigStore {
  config: Config | null;
  loadConfig: () => Promise<void>;
  watchConfig: () => void;  // ❌ No cleanup
}

export const useConfigStore = create<ConfigStore>((set, get) => ({
  config: null,

  loadConfig: async () => {
    const config = await invoke<Config>('load_config');
    set({ config });
  },

  watchConfig: async () => {
    // ❌ Event listener never removed
    await listen('config-changed', async () => {
      const { loadConfig } = get();  // Captures store reference
      await loadConfig();
    });
  },
}));
```

```typescript
// src/App.tsx
export function App() {
  const watchConfig = useConfigStore((s) => s.watchConfig);

  useEffect(() => {
    watchConfig();  // ❌ Registers listener, never cleans up
  }, []);  // Runs once on mount

  // ❌ If App remounts (HMR, navigation), creates duplicate listener
  // ❌ Old listener still references old store state
  // ❌ Memory leak: old closures + old state never garbage collected
}
```

**Memory leak progression:**
1. Initial mount: 1 listener, 50 KB memory
2. HMR reload (dev mode): 2 listeners, 100 KB memory
3. After 10 reloads: 10 listeners, 500 KB memory
4. After 100 reloads: 100 listeners, 5 MB memory
5. Eventually: Out of memory crash

**Consequences:**
- Memory usage grows over time (10-50 MB per hour in dev mode)
- App becomes sluggish after extended use
- Multiple config reloads on single file change (duplicate listeners)
- HMR in dev mode causes exponential listener growth
- Production: App crashes after days of continuous use

**Prevention strategy:**

1. **Return cleanup function from Zustand actions:**
   ```typescript
   // src/stores/configStore.ts
   import { create } from 'zustand';
   import { UnlistenFn, listen } from '@tauri-apps/api/event';

   interface ConfigStore {
     config: Config | null;
     watcherUnlisten: UnlistenFn | null;
     startWatching: () => Promise<void>;
     stopWatching: () => void;
   }

   export const useConfigStore = create<ConfigStore>((set, get) => ({
     config: null,
     watcherUnlisten: null,

     startWatching: async () => {
       // Clean up existing listener
       const { watcherUnlisten } = get();
       if (watcherUnlisten) {
         watcherUnlisten();
       }

       // Register new listener
       const unlisten = await listen('config-changed', async () => {
         const config = await invoke<Config>('load_config');
         set({ config });
       });

       set({ watcherUnlisten: unlisten });
     },

     stopWatching: () => {
       const { watcherUnlisten } = get();
       if (watcherUnlisten) {
         watcherUnlisten();
         set({ watcherUnlisten: null });
       }
     },
   }));
   ```

2. **Use React hooks for cleanup:**
   ```typescript
   // src/App.tsx
   export function App() {
     const { startWatching, stopWatching } = useConfigStore();

     useEffect(() => {
       startWatching();

       // ✓ Cleanup on unmount
       return () => {
         stopWatching();
       };
     }, [startWatching, stopWatching]);
   }
   ```

3. **Store unlisteners in Set for multiple watchers:**
   ```typescript
   // src/stores/configStore.ts
   interface ConfigStore {
     watchers: Set<UnlistenFn>;
     addWatcher: (path: string) => Promise<void>;
     removeAllWatchers: () => void;
   }

   export const useConfigStore = create<ConfigStore>((set, get) => ({
     watchers: new Set(),

     addWatcher: async (path: string) => {
       const unlisten = await listen(`file-changed:${path}`, handler);
       set((state) => {
         state.watchers.add(unlisten);
         return { watchers: state.watchers };
       });
     },

     removeAllWatchers: () => {
       const { watchers } = get();
       watchers.forEach((unlisten) => unlisten());
       set({ watchers: new Set() });
     },
   }));
   ```

4. **Test for memory leaks in E2E tests:**
   ```typescript
   // tests/memory-leak.spec.ts
   import { test, expect } from '@playwright/test';

   test('should not leak memory on config reload', async ({ page }) => {
     await page.goto('/');

     // Get initial memory usage
     const initialMemory = await page.evaluate(() => {
       if (performance.memory) {
         return performance.memory.usedJSHeapSize;
       }
       return 0;
     });

     // Reload config 50 times
     for (let i = 0; i < 50; i++) {
       await page.click('[data-testid="reload-config"]');
       await page.waitForTimeout(100);
     }

     // Force garbage collection (requires --expose-gc flag)
     await page.evaluate(() => {
       if (global.gc) global.gc();
     });

     const finalMemory = await page.evaluate(() => {
       if (performance.memory) {
         return performance.memory.usedJSHeapSize;
       }
       return 0;
     });

     // Memory should not grow >2x
     const growth = finalMemory / initialMemory;
     expect(growth).toBeLessThan(2);
   });
   ```

5. **Use Zustand middleware for automatic cleanup:**
   ```typescript
   // src/lib/zustand-cleanup.ts
   import { StateCreator, StoreMutatorIdentifier } from 'zustand';

   export const cleanupMiddleware = <T>(
     config: StateCreator<T>,
   ): StateCreator<T> => (set, get, api) => {
     const cleanup = new Set<() => void>();

     // Wrap set to track cleanup functions
     const wrappedSet: typeof set = (partial, replace) => {
       if (typeof partial === 'function') {
         const next = partial(get());
         if (next && typeof next.cleanup === 'function') {
           cleanup.add(next.cleanup);
         }
       }
       return set(partial, replace);
     };

     // Add destroy method
     (api as any).destroy = () => {
       cleanup.forEach((fn) => fn());
       cleanup.clear();
     };

     return config(wrappedSet, get, api);
   };
   ```

**Warning signs:**
- Chrome DevTools Memory tab shows growing heap
- Multiple event listeners for same event (check Tauri DevTools)
- App slows down after prolonged use
- HMR causes duplicate events
- Memory profiler shows uncollected closures

**Detection:**
```typescript
// src/lib/debug.ts
export function monitorEventListeners() {
  if (import.meta.env.DEV) {
    setInterval(() => {
      const listeners = (window as any).__TAURI__?.event?._listeners || {};
      const counts = Object.entries(listeners).map(([event, list]) =>
        `${event}: ${(list as any[]).length}`
      );

      if (counts.some((c) => parseInt(c.split(': ')[1]) > 5)) {
        console.warn('Possible event listener leak:', counts);
      }
    }, 5000);
  }
}
```

**Phase mapping:** Phase 3 (File Watching) MUST implement cleanup.

---

## Moderate Pitfalls

### Pitfall 7: Tauri Shell Scope Still References Old Binary Name "cch"

**Severity:** MEDIUM - Correctness

**What goes wrong:** UI cannot execute `rulez` binary because Tauri shell scope still allows "cch", not "rulez".

**Why it happens:** Binary was renamed from `cch` to `rulez` but `tauri.conf.json` shell scope wasn't updated.

**Evidence from project:**
- tauri.conf.json lines 49-52: `"name": "cch", "cmd": "cch"`
- Should be: `"name": "rulez", "cmd": "rulez"`

**Prevention:**
```json
// rulez-ui/src-tauri/tauri.conf.json
{
  "plugins": {
    "shell": {
      "open": true,
      "scope": [
        {
          "name": "rulez",
          "cmd": "rulez",
          "args": true
        }
      ]
    }
  }
}
```

**Phase mapping:** Phase 5 (Binary Integration) - Update immediately.

---

### Pitfall 8: Monaco YAML Schema URI Triggers Unnecessary Network Requests

**Severity:** MEDIUM - Performance

**What goes wrong:** Monaco-YAML tries to download schemas from `http://example.com/schema.json` URLs even when schema is provided inline.

**Why it happens:** Using generic URIs like `http://example.com` triggers download attempts. Using filename in URI prevents this.

**Real-world evidence (2026):**
- monaco-yaml GitHub Issue #214: "Changing URI to include filename resolves download attempts"

**Prevention:**
```typescript
// src/lib/monaco-schema.ts
import { configureMonacoYaml } from 'monaco-yaml';

configureMonacoYaml(monaco, {
  schemas: [
    {
      uri: 'https://rulez.local/hooks-schema.json',  // ✓ Filename in URI
      fileMatch: ['**/hooks.yaml'],
      schema: {
        $schema: 'http://json-schema.org/draft-07/schema#',
        // ... schema definition
      },
    },
  ],
});
```

**Phase mapping:** Phase 1 (Monaco Autocomplete) - Use proper URIs.

---

### Pitfall 9: WebView Rendering Differences Cause Layout Shifts

**Severity:** MEDIUM - UX

**What goes wrong:** UI looks correct on macOS but has layout issues on Windows/Linux due to WebView rendering differences.

**Why it happens:** WKWebView (macOS), WebView2 (Windows), WebKitGTK (Linux) have different font rendering, scrollbar styles, and CSS support.

**Real-world evidence (2026):**
- Tauri Discussion #12311: "Layout and rendering differences between Windows and Linux"
- WebView versions differ: macOS (Safari-based), Windows (Chromium-based), Linux (WebKit-based)
- Use normalize.css for consistency

**Prevention:**
```css
/* src/styles/globals.css */
/* Normalize scrollbars across platforms */
::-webkit-scrollbar {
  width: 8px;
  height: 8px;
}

::-webkit-scrollbar-track {
  background: var(--background);
}

::-webkit-scrollbar-thumb {
  background: var(--muted-foreground);
  border-radius: 4px;
}

/* Force consistent font rendering */
body {
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
  text-rendering: optimizeLegibility;
}
```

**Phase mapping:** Phase 6 (Cross-Platform Polish) - Test on all platforms.

---

## Technical Debt Patterns

| Shortcut | Immediate Benefit | Long-term Cost | When Acceptable |
|----------|-------------------|----------------|-----------------|
| Load entire JSONL file via IPC | Simple implementation | App freezes on large files, OOM | Only for <1000 line logs |
| Use `waitForTimeout()` in E2E tests | Tests pass quickly | Flaky tests, CI failures | Never (use auto-waiting) |
| Watch entire directories with inotify | Simpler code | Linux resource exhaustion | Only if <100 files total |
| Bundle Monaco without code-splitting | Faster dev setup | 3+ second load times | Only in internal tools |
| Skip binary path detection fallbacks | Works on developer machines | Fails on Windows, user installs | Never (support all platforms) |
| Don't cleanup Zustand event listeners | Less boilerplate | Memory leaks in production | Never (always cleanup) |

## Integration Gotchas

| Integration | Common Mistake | Correct Approach |
|-------------|----------------|------------------|
| monaco-yaml | Bundle includes duplicate monaco-editor | Configure Vite to deduplicate, use shared instance |
| Tauri IPC (large data) | Send 10 MB JSONL in single invoke() | Stream with events + pagination |
| notify (Linux) | Watch directories recursively | Watch specific files, use polling fallback |
| Playwright + Monaco | Use `waitForTimeout()` for editor load | Wait for `.view-lines` selector, use auto-waiting |
| Windows PATH | Split on `:` (Unix) | Use `path.delimiter` (platform-aware) |
| Zustand + Tauri events | Register listeners without cleanup | Store unlisten functions, cleanup on unmount |

## Performance Traps

| Trap | Symptoms | Prevention | When It Breaks |
|------|----------|------------|----------------|
| Monaco bundle duplication | 3+ second load, 2+ MB JS | Vite deduplication config | Every page load |
| JSONL IPC without streaming | 5-30 second freezes | Pagination + virtual scrolling | >10K log lines |
| inotify watch exhaustion | File watching stops | Watch specific files, not dirs | >8K files (Linux default) |
| Monaco async loading | E2E tests fail randomly | Auto-waiting, not timeouts | CI (different WebView timing) |
| Event listener accumulation | Memory grows 10-50 MB/hour | Cleanup in useEffect | Long-running sessions |

## Security Mistakes

| Mistake | Risk | Prevention |
|---------|------|------------|
| Shell scope allows any binary | Command injection | Allowlist only "rulez" binary |
| No validation on file paths from IPC | Path traversal | Validate paths are within `.claude/` |
| Eval YAML as code | Code execution | Use `yaml.parse()`, never `eval()` |
| No CSP (Content Security Policy) | XSS attacks | Configure CSP in tauri.conf.json |
| Unsigned binaries | SmartScreen warnings | Code sign with EV certificate |

## UX Pitfalls

| Pitfall | User Impact | Better Approach |
|---------|-------------|-----------------|
| No loading indicator for logs | App appears frozen | Show progress bar with streaming |
| File watching fails silently | Configs don't update | Show error banner with fix instructions |
| Binary not found, no guidance | Feature appears broken | Show install instructions, link to docs |
| Monaco loads slowly on first edit | Typing feels laggy | Preload Monaco on app startup |
| No error handling for IPC failures | Blank screen, no context | Toast notifications with retry button |

## "Looks Done But Isn't" Checklist

- [ ] **Monaco autocomplete:** Schema appears in editor, but bundle size >2 MB (deduplication failed)
- [ ] **Log viewing:** Works with 100 lines, but freezes with 10K lines (no pagination)
- [ ] **File watching:** Works on macOS, but fails on Linux with many files (no inotify limit check)
- [ ] **E2E tests:** Pass locally, but fail in CI 50% of time (artificial timeouts, not auto-waiting)
- [ ] **Binary detection:** Works on Unix, but fails on Windows (PATH separator issue)
- [ ] **Zustand stores:** Work in production, but memory leaks in long-running sessions (no cleanup)
- [ ] **Cross-platform UI:** Perfect on macOS, but layout shifts on Windows (WebView differences)
- [ ] **Tauri shell scope:** Can execute commands, but binary name is outdated ("cch" not "rulez")

## Recovery Strategies

| Pitfall | Recovery Cost | Recovery Steps |
|---------|---------------|----------------|
| Monaco bundle duplication | LOW | Add Vite deduplication config, verify with `bun why monaco-editor` |
| JSONL IPC freeze | MEDIUM | Refactor to streaming API with events, add virtual scrolling |
| inotify exhaustion | LOW | Switch to file-specific watches, add polling fallback |
| E2E flakiness | MEDIUM | Remove all `waitForTimeout()`, use Playwright auto-waiting |
| Windows PATH issues | LOW | Use `path.delimiter`, add Windows to CI matrix |
| Zustand memory leak | MEDIUM | Add cleanup to all event listeners, test with memory profiler |
| WebView rendering diff | LOW | Add normalize.css, test on all platforms |
| Shell scope outdated | LOW | Update tauri.conf.json, redeploy |

## Pitfall-to-Phase Mapping

| Pitfall | Prevention Phase | Verification |
|---------|------------------|--------------|
| P1: Monaco bundle duplication | Phase 1 (Autocomplete) | Bundle size <1.5 MB, `bun why monaco-editor` shows single instance |
| P2: JSONL IPC bottleneck | Phase 2 (Log Viewing) | Load 100K line log in <2 seconds |
| P3: inotify exhaustion | Phase 3 (File Watching) | Works with >10K files, shows error if limit hit |
| P4: E2E flakiness | Phase 4 (Test Stabilization) | 100% pass rate over 10 runs |
| P5: Windows PATH issues | Phase 5 (Binary Detection) | CI passes on Windows, macOS, Linux |
| P6: Zustand memory leak | Phase 3 (File Watching) | Memory stable after 1 hour runtime |
| P7: Shell scope outdated | Phase 5 (Binary Detection) | Can execute `rulez` from UI |
| P8: Schema URI downloads | Phase 1 (Autocomplete) | No network requests in DevTools |
| P9: WebView rendering diff | Phase 6 (Polish) | Consistent layout on all platforms |

## Sources

**Monaco Editor (HIGH confidence):**
- [monaco-yaml GitHub](https://github.com/remcohaszing/monaco-yaml)
- [Monaco YAML Issue #214 - Schema URI downloads](https://github.com/remcohaszing/monaco-yaml/issues/214)
- [Monaco Editor Issue #2755 - Async autocomplete](https://github.com/microsoft/monaco-editor/issues/2755)
- [Monaco Editor Issue #4033 - Performance with large files](https://github.com/microsoft/monaco-editor/issues/4033)
- [Vite Plugin Monaco Editor](https://github.com/vbenjs/vite-plugin-monaco-editor)

**Tauri IPC (HIGH confidence):**
- [Tauri IPC Documentation](https://v2.tauri.app/concept/inter-process-communication/)
- [Tauri Discussion #7699 - Deprecate JSON in IPC](https://github.com/tauri-apps/tauri/discussions/7699)
- [Tauri Issue #7127 - Binary data in IPC](https://github.com/tauri-apps/tauri/issues/7127)
- [Tauri WebView Versions](https://v2.tauri.app/reference/webview-versions/)

**File Watching (HIGH confidence):**
- [Rust notify crate documentation](https://docs.rs/notify/latest/notify/)
- [notify crate - Platform differences](https://github.com/notify-rs/notify)
- [Parcel Watcher Issue #171 - FSEvents vs inotify](https://github.com/parcel-bundler/watcher/issues/171)
- [Notify 9.0 RC - Debouncing enhancements](https://cargo-run.news/p/notify-9-0-rc-enhances-filesystem-watching-with-robust-debouncing)

**Playwright Testing (HIGH confidence):**
- [Playwright Flaky Tests Guide (2026)](https://www.browserstack.com/guide/playwright-flaky-tests)
- [Tauri Testing Documentation](https://v2.tauri.app/develop/tests/)
- [Playwright Best Practices (2026)](https://www.browserstack.com/guide/playwright-best-practices)
- [Playwright CDP with Tauri](https://github.com/Haprog/playwright-cdp)

**Cross-Platform (HIGH confidence):**
- [cross-platform-node-guide - Environment Variables](https://github.com/ehmicky/cross-platform-node-guide/blob/main/docs/4_terminal/environment_variables.md)
- [cross-platform-node-guide - File Paths](https://github.com/ehmicky/cross-platform-node-guide/blob/main/docs/3_filesystem/file_paths.md)
- [Tauri Discussion #12311 - Layout differences](https://github.com/tauri-apps/tauri/discussions/12311)

**State Management (MEDIUM confidence):**
- [Zustand Discussion #2540 - Memory leaks](https://github.com/pmndrs/zustand/discussions/2540)
- [Zustand Discussion #2054 - Subscriber cleanup](https://github.com/pmndrs/zustand/discussions/2054)
- [Zustand Discussion #1394 - Avoiding stale state](https://github.com/pmndrs/zustand/discussions/1394)
- [Zustand Documentation - Persisting Store Data](https://zustand.docs.pmnd.rs/integrations/persisting-store-data)

**Performance (MEDIUM confidence):**
- [JSONL Performance Guide](https://ndjson.com/performance/)
- [stream-json - Node.js streaming JSON parser](https://github.com/uhop/stream-json)
- [TanStack Virtual](https://tanstack.com/virtual)

**Tauri Security (HIGH confidence):**
- [Tauri Command Scopes](https://v2.tauri.app/security/scope/)
- [Tauri Shell Security Advisory](https://github.com/tauri-apps/plugins-workspace/security/advisories/GHSA-c9pr-q8gx-3mgp)
- [Tauri Updater Plugin](https://v2.tauri.app/plugin/updater/)

**Project Context (HIGH confidence):**
- RuleZ UI tauri.conf.json (shell scope references "cch")
- RuleZ UI test files (flaky E2E tests)
- RuleZ v1.4 completion (Tauri CI working)

---

**Last Updated:** 2026-02-10
**Next Review:** After Phase 1 implementation (validate Monaco bundle size)
