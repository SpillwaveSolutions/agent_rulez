# Phase 13: Log Viewer - Research

**Researched:** 2026-02-12
**Domain:** React virtual scrolling, Tauri file I/O, JSON Lines log parsing, audit log visualization
**Confidence:** HIGH

## Summary

Phase 13 adds an audit log viewer to the RuleZ UI desktop app (Tauri + React + TypeScript). The log file at `~/.claude/logs/rulez.log` uses **JSON Lines format** (one JSON object per line), where each entry is a `LogEntry` struct with fields for timestamp, event_type, outcome, tool_name, session_id, timing, decision, mode, and more. The current log has ~14K entries (~5MB); the requirement targets 100K+ entries at 60fps.

The UI already has a three-panel layout (Sidebar | MainContent | RightPanel) with tabs on the RightPanel (Simulator, Rules, Settings). The log viewer should be added as a **new tab** in the RightPanel or as a new **main content view** toggled from the sidebar/header. E2E test scaffolding already exists with expected `data-testid="log-entry"` selectors, text search input, severity dropdown, date range pickers, export buttons, and copy buttons.

**Primary recommendation:** Use `@tanstack/react-virtual` (v3.13.18) for virtual scrolling. Parse logs on the Rust side via a new Tauri command that leverages the existing `LogQuery`/`QueryFilters` infrastructure. Use Tauri's native file dialog/write for export. Map "severity" to the `outcome`/`decision` fields since the log format has no `level` field.

## Standard Stack

### Core
| Library | Version | Purpose | Why Standard |
|---------|---------|---------|--------------|
| `@tanstack/react-virtual` | 3.13.18 | Virtual scrolling for 100K+ rows | Headless, lightweight (~15KB), 60fps proven, active maintenance, most popular React virtualizer |
| `@tauri-apps/plugin-fs` | ^2.0.0 | File write for export | Tauri v2 standard plugin for filesystem access |
| `@tauri-apps/plugin-dialog` | ^2.0.0 | Save-file dialog for export | Tauri v2 standard plugin for native file dialogs |

### Supporting
| Library | Version | Purpose | When to Use |
|---------|---------|---------|-------------|
| `@tauri-apps/plugin-clipboard-manager` | ^2.0.0 | Copy log entries to clipboard | LOG-07 requirement, `writeText()` API |
| `zustand` | ^5.0.3 | Log viewer state management | Already in project, use for logStore |

### Not Needed
| Problem | Don't Add | Why |
|---------|-----------|-----|
| CSV generation | `papaparse` or `csv-writer` | Log entries are simple objects; manual CSV generation is ~20 lines |
| Date picker | `react-datepicker` | Use native HTML `<input type="date">` — sufficient for date range filtering |
| Text search | `fuse.js` or `lunr` | Simple `String.includes()` is adequate for text filtering on pre-filtered data |

### Alternatives Considered
| Instead of | Could Use | Tradeoff |
|------------|-----------|----------|
| `@tanstack/react-virtual` | `react-virtuoso` | react-virtuoso is batteries-included with grouping/infinite scroll but heavier and opinionated; TanStack is lighter and matches project's headless/minimal pattern |
| `@tanstack/react-virtual` | `react-window` | react-window is simpler but less maintained, no dynamic sizing support |
| Tauri plugin-fs + plugin-dialog | Web File API (`Blob` + download link) | Tauri plugins give native save dialog and write to any path; web API requires blob URL workaround and doesn't work well in Tauri WebView |

**Installation:**
```bash
# Frontend packages
cd rulez_ui
bun add @tanstack/react-virtual @tauri-apps/plugin-fs @tauri-apps/plugin-dialog @tauri-apps/plugin-clipboard-manager

# Rust plugins (add to rulez_ui/src-tauri/Cargo.toml)
# tauri-plugin-fs = "2"
# tauri-plugin-dialog = "2"
# tauri-plugin-clipboard-manager = "2"
```

**Tauri plugin registration (main.rs):**
```rust
tauri::Builder::default()
    .plugin(tauri_plugin_store::Builder::default().build())
    .plugin(tauri_plugin_shell::init())
    .plugin(tauri_plugin_fs::init())           // NEW
    .plugin(tauri_plugin_dialog::init())       // NEW
    .plugin(tauri_plugin_clipboard_manager::init()) // NEW
```

## Architecture Patterns

### Recommended Project Structure
```
rulez_ui/
├── src/
│   ├── components/
│   │   └── logs/
│   │       ├── LogViewer.tsx          # Main log viewer container
│   │       ├── LogEntryRow.tsx        # Single log entry row (virtualized)
│   │       ├── LogFilterBar.tsx       # Search, severity, date range filters
│   │       └── LogExportMenu.tsx      # Export button with JSON/CSV options
│   ├── stores/
│   │   └── logStore.ts               # Zustand store for log state
│   ├── lib/
│   │   └── tauri.ts                  # Add log query functions here (existing pattern)
│   └── types/
│       └── index.ts                  # Add LogEntry types here (existing pattern)
└── src-tauri/
    └── src/
        └── commands/
            ├── mod.rs                # Add `pub mod logs;`
            └── logs.rs              # New: read_logs, export_logs Tauri commands
```

### Pattern 1: Rust-Side Log Parsing (Tauri Command)
**What:** Parse the JSON Lines log file on the Rust side, apply filters, return structured data to the frontend.
**When to use:** Always — this is the recommended approach for LOG-05 performance requirement.
**Why:** The Rust side already has `LogQuery` and `QueryFilters` in `cch_cli/src/logging.rs`. Parsing 100K+ JSON lines in JavaScript would block the main thread; Rust does it in milliseconds. The Tauri command pattern (`#[tauri::command]`) is already established in `debug.rs` and `config.rs`.

**Example (Rust command):**
```rust
// src-tauri/src/commands/logs.rs
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Serialize, Deserialize)]
pub struct LogEntryDto {
    pub timestamp: String,
    pub event_type: String,
    pub session_id: String,
    pub tool_name: Option<String>,
    pub rules_matched: Vec<String>,
    pub outcome: String,
    pub processing_ms: u64,
    pub rules_evaluated: usize,
    pub decision: Option<String>,
    pub mode: Option<String>,
    // Flattened for frontend consumption
    pub response_reason: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct LogQueryParams {
    pub text_filter: Option<String>,
    pub outcome_filter: Option<String>,   // "allow", "block", "inject"
    pub decision_filter: Option<String>,  // "allowed", "blocked", "warned", "audited"
    pub since: Option<String>,            // RFC3339
    pub until: Option<String>,            // RFC3339
    pub limit: Option<usize>,
}

#[tauri::command]
pub async fn read_logs(params: LogQueryParams) -> Result<Vec<LogEntryDto>, String> {
    // Read from ~/.claude/logs/rulez.log
    // Parse JSON Lines, apply filters, return DTOs
}

#[tauri::command]
pub async fn get_log_stats() -> Result<LogStats, String> {
    // Return total entries, file size, date range — for UI status bar
}
```

**Example (Frontend invocation — following existing lib/tauri.ts pattern):**
```typescript
// In lib/tauri.ts
export async function readLogs(params: LogQueryParams): Promise<LogEntryDto[]> {
  if (isTauri()) {
    const { invoke } = await import("@tauri-apps/api/core");
    return invoke<LogEntryDto[]>("read_logs", { params });
  }
  return mockReadLogs(params);
}
```

### Pattern 2: Virtual Scrolling with TanStack Virtual
**What:** Render only visible rows using `useVirtualizer` hook from `@tanstack/react-virtual`.
**When to use:** For the log entry list display (LOG-05).

**Example:**
```tsx
// Source: @tanstack/react-virtual official docs + adapted for this project
import { useVirtualizer } from '@tanstack/react-virtual';

function LogList({ entries }: { entries: LogEntryDto[] }) {
  const parentRef = useRef<HTMLDivElement>(null);

  const rowVirtualizer = useVirtualizer({
    count: entries.length,
    getScrollElement: () => parentRef.current,
    estimateSize: () => 36, // Fixed row height for log entries
    overscan: 10,           // Render 10 extra rows outside viewport
  });

  return (
    <div ref={parentRef} style={{ height: '100%', overflow: 'auto' }}>
      <div style={{
        height: `${rowVirtualizer.getTotalSize()}px`,
        width: '100%',
        position: 'relative',
      }}>
        {rowVirtualizer.getVirtualItems().map((virtualRow) => (
          <LogEntryRow
            key={virtualRow.index}
            data-testid="log-entry"
            entry={entries[virtualRow.index]}
            style={{
              position: 'absolute',
              top: 0,
              left: 0,
              width: '100%',
              height: `${virtualRow.size}px`,
              transform: `translateY(${virtualRow.start}px)`,
            }}
          />
        ))}
      </div>
    </div>
  );
}
```

### Pattern 3: Zustand Log Store
**What:** Centralized state for log entries, filters, loading status.
**When to use:** Manage log viewer state across components.

**Example:**
```typescript
// src/stores/logStore.ts
import { create } from 'zustand';

interface LogState {
  entries: LogEntryDto[];
  totalCount: number;
  isLoading: boolean;
  textFilter: string;
  outcomeFilter: string | null;     // "allow" | "block" | "inject" | null
  decisionFilter: string | null;    // "allowed" | "blocked" | "warned" | "audited" | null
  sinceFilter: string | null;       // ISO date string
  untilFilter: string | null;       // ISO date string
}

interface LogActions {
  loadLogs: () => Promise<void>;
  setTextFilter: (text: string) => void;
  setOutcomeFilter: (outcome: string | null) => void;
  setDecisionFilter: (decision: string | null) => void;
  setSinceFilter: (since: string | null) => void;
  setUntilFilter: (until: string | null) => void;
  exportLogs: (format: 'json' | 'csv') => Promise<void>;
  copyEntry: (entry: LogEntryDto) => Promise<void>;
}

export const useLogStore = create<LogState & LogActions>((set, get) => ({
  // ... state + actions
}));
```

### Pattern 4: Export via Tauri Plugins
**What:** Use `@tauri-apps/plugin-dialog` for save dialog, `@tauri-apps/plugin-fs` for writing.
**Example:**
```typescript
import { save } from '@tauri-apps/plugin-dialog';
import { writeTextFile } from '@tauri-apps/plugin-fs';

async function exportToJson(entries: LogEntryDto[]) {
  const path = await save({
    filters: [{ name: 'JSON', extensions: ['json'] }],
    defaultPath: 'rulez-logs.json',
  });
  if (path) {
    await writeTextFile(path, JSON.stringify(entries, null, 2));
  }
}

async function exportToCsv(entries: LogEntryDto[]) {
  const headers = ['timestamp', 'event_type', 'tool_name', 'outcome', 'decision', 'rules_matched'];
  const rows = entries.map(e =>
    headers.map(h => JSON.stringify(e[h] ?? '')).join(',')
  );
  const csv = [headers.join(','), ...rows].join('\n');

  const path = await save({
    filters: [{ name: 'CSV', extensions: ['csv'] }],
    defaultPath: 'rulez-logs.csv',
  });
  if (path) {
    await writeTextFile(path, csv);
  }
}
```

### Pattern 5: Clipboard Copy
**What:** Copy a single log entry's JSON to the clipboard.
**Example:**
```typescript
// Option A: Web API (works in Tauri WebView, simpler)
await navigator.clipboard.writeText(JSON.stringify(entry, null, 2));

// Option B: Tauri plugin (more reliable, especially on Linux)
import { writeText } from '@tauri-apps/plugin-clipboard-manager';
await writeText(JSON.stringify(entry, null, 2));
```
**Recommendation:** Use `navigator.clipboard.writeText()` first (simpler, no extra plugin). Fall back to Tauri plugin only if needed (Linux Wayland issues). Since we're adding the clipboard plugin for robustness anyway, use the Tauri plugin.

### Anti-Patterns to Avoid
- **Parsing JSON in the renderer process:** Don't read the entire log file via `readTextFile` and parse 100K+ JSON lines in JavaScript — it will freeze the UI. Parse on Rust side.
- **Loading all entries into React state at once:** While we can hold 100K entries in memory (each ~200 bytes = ~20MB), the initial parse+transfer should be paginated or chunked if needed. Start by returning all entries from Rust (fast), and paginate only if transfer size becomes a problem.
- **Using `Array.filter()` on every keystroke:** Debounce text search to 300ms, and re-query the Rust backend with filters rather than filtering in JS.
- **Re-rendering the entire list on filter changes:** `useVirtualizer` handles this — only visible rows re-render.

## Don't Hand-Roll

| Problem | Don't Build | Use Instead | Why |
|---------|-------------|-------------|-----|
| Virtual scrolling | Custom scroll observer | `@tanstack/react-virtual` | Off-by-one errors, scroll position calculation, overscan handling — all solved |
| JSON Lines parsing | Custom line splitter | `serde_json::from_str` per line (already in `LogQuery::query`) | Handles escaped newlines, malformed entries gracefully |
| File save dialog | Custom file path input | `@tauri-apps/plugin-dialog` `save()` | Native OS dialog, path validation, overwrite confirmation |
| Log rotation awareness | Custom rotation detection | Existing `LogRotator` in `cch_cli/src/logging.rs` | Already handles `.1`, `.2` suffixes |

**Key insight:** The Rust backend already has a complete `LogQuery` + `QueryFilters` system. Don't rebuild log parsing or filtering logic in TypeScript. Expose the existing Rust infrastructure through a new Tauri command.

## Current State Analysis

### Log Format (VERIFIED — HIGH confidence)
The log file `~/.claude/logs/rulez.log` is **JSON Lines** format. Each line is a serialized `LogEntry`:

```json
{"timestamp":"2026-02-10T18:32:34.748760Z","event_type":"PreToolUse","session_id":"e2e-no-config","tool_name":"Bash","rules_matched":[],"outcome":"allow","timing":{"processing_ms":0,"rules_evaluated":0},"metadata":{},"event_details":{"tool_type":"Bash","command":"git push --force"},"response":{"continue":true}}
```

**Key fields per LogEntry:**
| Field | Type | Always Present | Description |
|-------|------|----------------|-------------|
| `timestamp` | ISO 8601 DateTime | Yes | Microsecond precision UTC |
| `event_type` | String | Yes | "PreToolUse", "PostToolUse", "PermissionRequest", etc. |
| `session_id` | String | Yes | Session identifier |
| `tool_name` | String? | No | "Bash", "Write", "Edit", "Read", etc. |
| `rules_matched` | String[] | Yes | Names of matched rules (may be empty) |
| `outcome` | Enum | Yes | "allow", "block", "inject" |
| `timing.processing_ms` | u64 | Yes | Processing time in ms |
| `timing.rules_evaluated` | usize | Yes | Number of rules checked |
| `decision` | Enum? | No | "allowed", "blocked", "warned", "audited" |
| `mode` | Enum? | No | "enforce", "warn", "audit" |
| `priority` | i32? | No | Rule priority |
| `metadata` | Object? | Yes (often empty) | injected_files, validator_output |
| `event_details` | Object? | No | Tool-specific details (command, file_path) |
| `response` | Object? | No | continue flag, reason, context_length |

### Severity Mapping (CRITICAL DESIGN DECISION)
The requirements say "filter by severity level (error, warn, info, debug)" (LOG-03), but **the actual log format has no `level` field**. The E2E test mock data (`tests/fixtures/mock-logs.json`) uses `{level, message, source}` which is NOT the real format.

**Recommended mapping from LogEntry fields to "severity":**
| UI Severity | Source Fields | Criteria |
|-------------|---------------|----------|
| Error | `outcome` = "block" OR `decision` = "blocked" | An operation was blocked |
| Warning | `decision` = "warned" OR `mode` = "warn" | A warning was issued |
| Info | `outcome` = "allow" AND no warning | Normal allowed operation |
| Debug | `event_details` present with `raw_event` | Debug-level detail entries |

This maps the domain concept (policy outcomes) to severity labels users expect.

### Existing UI Layout
- **AppShell**: Header | (Sidebar | MainContent | RightPanel) | StatusBar
- **RightPanel tabs**: "Simulator" | "Rules" | "Settings" — stored in `uiStore.ts` as `RightPanelTab`
- **No log viewer component exists yet** — no `*Log*` or `*log*` component files
- **RightPanelTab type**: `"simulator" | "tree" | "settings"` — needs `"logs"` added

### Existing Tauri Commands Pattern
Commands are in `rulez_ui/src-tauri/src/commands/` with `mod.rs` re-exporting. Each command:
- Uses `#[tauri::command]` attribute
- Returns `Result<T, String>` (serialized to JSON over IPC)
- Is registered in `main.rs` `invoke_handler`
- Has a corresponding TypeScript wrapper in `src/lib/tauri.ts` with `isTauri()` check + mock fallback

### Existing Rust Infrastructure to Reuse
- `cch_cli::logging::LogQuery` — reads log file, parses JSON Lines, applies filters
- `cch_cli::logging::QueryFilters` — session_id, tool_name, rule_name, outcome, since, until, mode, decision, limit
- `cch_cli::models::LogEntry` — full struct with serde Serialize/Deserialize
- `cch_cli::models::Outcome` — Allow, Block, Inject
- `cch_cli::models::Decision` — Allowed, Blocked, Warned, Audited
- `cch_cli::models::PolicyMode` — Enforce, Warn, Audit
- `cch_cli::logging::Logger::default_log_path()` — returns `~/.claude/logs/rulez.log`

### E2E Test Scaffolding Already Exists
Phase 17 already scaffolded:
- `rulez_ui/tests/pages/log-viewer.page.ts` — POM with selectors
- `rulez_ui/tests/log-viewer.spec.ts` — 8 test cases
- `rulez_ui/tests/fixtures/mock-logs.json` — **WRONG FORMAT** (needs update to match real LogEntry)

Expected test selectors from POM:
- `data-testid="log-entry"` on each row
- Search/filter placeholder: `/search|filter/i`
- Severity select: `combobox` with name `/severity/i`
- Export button: `/export/i`
- Copy button: `/copy/i` inside each entry
- Date range: labels `/from/i` and `/to/i`

### Dependencies Already in Project
| Package | Version | Status |
|---------|---------|--------|
| `react` | ^18.3.1 | Present |
| `zustand` | ^5.0.3 | Present |
| `@tauri-apps/api` | ^2.5.0 | Present |
| `@tauri-apps/plugin-store` | 2.4.2 | Present |
| `@tauri-apps/plugin-shell` | ^2.2.1 | Present |
| `tailwindcss` | ^4.0.6 | Present |
| `@tanstack/react-virtual` | — | **NOT present, needs install** |
| `@tauri-apps/plugin-fs` | — | **NOT present, needs install** |
| `@tauri-apps/plugin-dialog` | — | **NOT present, needs install** |
| `@tauri-apps/plugin-clipboard-manager` | — | **NOT present, needs install** |

### Tauri Plugin Setup Required
Tauri v2 plugins require:
1. **Cargo.toml dependency**: `tauri-plugin-X = "2"` in `[dependencies]`
2. **Plugin registration**: `.plugin(tauri_plugin_X::init())` in `main.rs`
3. **Frontend package**: `bun add @tauri-apps/plugin-X`
4. **Capabilities/permissions**: Tauri v2 uses a capability-based permission system. The project currently has NO `capabilities/` directory, meaning it uses default permissions. Adding plugins may require creating `src-tauri/capabilities/default.json`.

## Common Pitfalls

### Pitfall 1: Mock Data Format Mismatch
**What goes wrong:** E2E tests use `mock-logs.json` with `{level, message, source}` format, but the real log uses `LogEntry` with `{event_type, outcome, decision, tool_name}`. Tests will fail against real data.
**Why it happens:** Mock data was scaffolded before analyzing the actual log format.
**How to avoid:** Update `mock-logs.json` to match real `LogEntry` structure. Update the mock function in `lib/tauri.ts` to return properly shaped data.
**Warning signs:** E2E tests pass against mocks but fail against real app.

### Pitfall 2: Blocking UI with Large File Parsing
**What goes wrong:** Reading and parsing 100K+ JSON lines in the JavaScript renderer thread freezes the UI for seconds.
**Why it happens:** `readTextFile` returns the whole file as a string, then `JSON.parse` per line on the main thread.
**How to avoid:** Parse on Rust side via Tauri command. The existing `LogQuery::query()` does this already.
**Warning signs:** UI becomes unresponsive when opening log viewer with a large file.

### Pitfall 3: Tauri IPC Transfer Size
**What goes wrong:** Returning 100K+ serialized log entries over Tauri IPC may be slow (100K entries * ~300 bytes = ~30MB JSON).
**Why it happens:** Tauri serializes command results to JSON and transfers via IPC bridge.
**How to avoid:** Default to a reasonable limit (e.g., 10K entries). Add pagination or "load more" for larger datasets. Apply server-side filters before transfer.
**Warning signs:** Slow initial load of log viewer, long delay between clicking "Logs" tab and seeing data.

### Pitfall 4: Debouncing Text Filter
**What goes wrong:** Re-querying the Rust backend on every keystroke in the search box causes jank and excessive IPC calls.
**Why it happens:** Each keystroke triggers a full log re-read.
**How to avoid:** Debounce text input to 300ms. Optionally, do a first pass of Rust-side filtering on load, then filter the in-memory JS array for text search (since text search is just `String.includes()`).
**Warning signs:** Typing in the search box feels laggy.

### Pitfall 5: Missing Tauri Capability Permissions
**What goes wrong:** Plugin calls fail at runtime with "permission denied" errors.
**Why it happens:** Tauri v2 requires explicit capability declarations for plugin access.
**How to avoid:** Create `src-tauri/capabilities/default.json` with required permissions for fs (read/write scope), dialog, and clipboard plugins.
**Warning signs:** Works in `tauri dev` but fails in production build. Or plugin calls return errors.

### Pitfall 6: Date Picker Time Zone Handling
**What goes wrong:** Date range filter misses entries due to timezone conversion.
**Why it happens:** HTML `<input type="date">` returns local dates; log timestamps are UTC.
**How to avoid:** Convert local dates to UTC start/end of day before sending to Rust filter.
**Warning signs:** Filtering by "today" misses entries from the current UTC day.

## Code Examples

### LogEntry TypeScript Type (matches real format)
```typescript
// Source: Derived from cch_cli/src/models.rs LogEntry struct
export interface LogEntryDto {
  timestamp: string;          // ISO 8601 UTC
  event_type: string;         // "PreToolUse", "PostToolUse", etc.
  session_id: string;
  tool_name: string | null;
  rules_matched: string[];
  outcome: "allow" | "block" | "inject";
  timing: {
    processing_ms: number;
    rules_evaluated: number;
  };
  decision: "allowed" | "blocked" | "warned" | "audited" | null;
  mode: "enforce" | "warn" | "audit" | null;
  priority: number | null;
  // Flattened response
  response_continue: boolean | null;
  response_reason: string | null;
  // Event details (simplified)
  event_detail_command: string | null;  // For Bash events
  event_detail_file_path: string | null; // For Write/Edit/Read events
}
```

### Severity Derivation Function
```typescript
export type Severity = 'error' | 'warn' | 'info' | 'debug';

export function deriveSeverity(entry: LogEntryDto): Severity {
  if (entry.outcome === 'block' || entry.decision === 'blocked') return 'error';
  if (entry.decision === 'warned' || entry.mode === 'warn') return 'warn';
  if (entry.outcome === 'inject') return 'info'; // context injection is informational
  return 'info'; // default: allowed operations
}
```

### CSV Generation (no library needed)
```typescript
export function entriesToCsv(entries: LogEntryDto[]): string {
  const headers = [
    'timestamp', 'event_type', 'tool_name', 'outcome',
    'decision', 'mode', 'rules_matched', 'processing_ms',
    'session_id', 'response_reason'
  ];

  const escapeField = (val: unknown): string => {
    const str = val == null ? '' : String(val);
    return str.includes(',') || str.includes('"') || str.includes('\n')
      ? `"${str.replace(/"/g, '""')}"`
      : str;
  };

  const rows = entries.map(entry =>
    headers.map(h => escapeField((entry as Record<string, unknown>)[h])).join(',')
  );

  return [headers.join(','), ...rows].join('\n');
}
```

### Rust Tauri Command (leveraging existing LogQuery)
```rust
// Note: This is a simplified illustration. The actual implementation
// will need to handle the cch_cli dependency or inline the parsing logic.
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Serialize)]
pub struct LogEntryDto {
    pub timestamp: String,
    pub event_type: String,
    pub session_id: String,
    pub tool_name: Option<String>,
    pub rules_matched: Vec<String>,
    pub outcome: String,
    pub timing: TimingDto,
    pub decision: Option<String>,
    pub mode: Option<String>,
    pub priority: Option<i32>,
    pub response_continue: Option<bool>,
    pub response_reason: Option<String>,
    pub event_detail_command: Option<String>,
    pub event_detail_file_path: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct TimingDto {
    pub processing_ms: u64,
    pub rules_evaluated: usize,
}

#[derive(Debug, Deserialize)]
pub struct LogQueryParams {
    pub text_filter: Option<String>,
    pub outcome_filter: Option<String>,
    pub decision_filter: Option<String>,
    pub since: Option<String>,
    pub until: Option<String>,
    pub limit: Option<usize>,
}

#[tauri::command]
pub async fn read_logs(params: LogQueryParams) -> Result<Vec<LogEntryDto>, String> {
    let log_path = get_log_path();
    if !log_path.exists() {
        return Ok(Vec::new());
    }

    let content = tokio::fs::read_to_string(&log_path)
        .await
        .map_err(|e| format!("Failed to read log file: {}", e))?;

    let mut entries: Vec<LogEntryDto> = Vec::new();

    for line in content.lines() {
        if line.trim().is_empty() { continue; }
        // Parse as serde_json::Value for flexibility
        let value: serde_json::Value = serde_json::from_str(line)
            .map_err(|e| format!("Parse error: {}", e))?;
        // Convert to DTO...
        // Apply text filter (check all string fields)
        // Apply outcome/decision/time filters
        entries.push(/* converted DTO */);
    }

    // Sort newest first
    entries.sort_by(|a, b| b.timestamp.cmp(&a.timestamp));

    // Apply limit
    if let Some(limit) = params.limit {
        entries.truncate(limit);
    }

    Ok(entries)
}

fn get_log_path() -> PathBuf {
    let mut path = dirs::home_dir().expect("home dir");
    path.push(".claude");
    path.push("logs");
    path.push("rulez.log");
    path
}
```

## State of the Art

| Old Approach | Current Approach | When Changed | Impact |
|--------------|------------------|--------------|--------|
| `react-window` (Brian Vaughn) | `@tanstack/react-virtual` (Tanner Linsley) | 2022+ | Headless, framework-agnostic, actively maintained |
| Tauri v1 `tauri::api::dialog` | Tauri v2 `@tauri-apps/plugin-dialog` | Tauri v2 (2024) | Plugin-based architecture, explicit permissions |
| Tauri v1 `tauri::api::fs` | Tauri v2 `@tauri-apps/plugin-fs` | Tauri v2 (2024) | Scoped access, `readTextFileLines` streaming API |
| Clipboard v1 `@tauri-apps/api/clipboard` | `@tauri-apps/plugin-clipboard-manager` | Tauri v2 (2024) | Plugin-based, `writeText`/`readText` API |

**Deprecated/outdated:**
- `react-window`: Still works but Brian Vaughn recommends TanStack Virtual as the successor
- `react-virtualized`: Predecessor to react-window, even more outdated
- Tauri v1 APIs: All replaced with plugin-based architecture in v2

## Risk Assessment

### Risk 1: cch_cli Dependency from Tauri Binary
**Severity:** MEDIUM
**Issue:** The `rulez_ui/src-tauri` Cargo project does NOT depend on `cch_cli`. The log parsing code (`LogQuery`, `QueryFilters`, `LogEntry`) lives in `cch_cli`. To reuse it, we'd need to either:
  1. Add `cch_cli` as a Cargo dependency of `rulez-ui` (may pull in too many deps)
  2. Extract the parsing logic into a shared crate
  3. Duplicate/inline the parsing logic in the Tauri command (simplest)
**Recommendation:** Option 3 — inline the parsing logic. The log command only needs to read JSON Lines and filter by field values. This is ~50 lines of Rust. Avoids coupling the UI binary to the CLI binary.

### Risk 2: Tauri v2 Permissions Configuration
**Severity:** LOW-MEDIUM
**Issue:** The project has no `capabilities/` directory. Adding fs/dialog/clipboard plugins may require explicit capability declarations.
**Mitigation:** Create `src-tauri/capabilities/default.json` with the needed permissions. Test in both dev and release modes.

### Risk 3: Transfer Size for 100K+ Entries
**Severity:** LOW
**Issue:** 100K entries serialized as JSON over IPC could be 20-30MB.
**Mitigation:** Default limit of 10,000 entries. Users can adjust. Rust-side filtering reduces data before transfer. Most real-world usage will have far fewer entries (current production: 14K).

## Dependencies and Prerequisites

### Must Be Complete Before Phase 13
| Dependency | Status | Why Needed |
|------------|--------|------------|
| Phase 12: YAML Editor Enhancements | ✅ Complete | Memory management patterns, Monaco disposal patterns |
| Phase 11: Settings Foundation | ✅ Complete | Settings store pattern, binary path resolution |
| Tauri v2 plugin system understanding | ✅ Researched | Know how to register plugins, set permissions |

### Phase 13 Provides for Later Phases
| Consumer | What They Need |
|----------|---------------|
| Phase 14 (Config Management) | Tauri plugin-fs and plugin-dialog patterns established here |
| Phase 17 (E2E Testing) | Log viewer component with data-testid attributes matching the already-scaffolded POM |

## Open Questions

1. **Where should the Log Viewer live in the UI?**
   - What we know: RightPanel currently has 3 tabs (Simulator, Rules, Settings). A log viewer with 100K entries needs more horizontal space than the 320px right panel.
   - What's unclear: Should it be a 4th right panel tab, or a full-width main content view?
   - Recommendation: **Full-width main content view**, toggled by adding a "Logs" navigation item in the header or sidebar. The log table needs width for columns (timestamp, event, tool, outcome, etc.). The right panel is too narrow.

2. **Should text search filter locally or re-query Rust?**
   - What we know: Re-querying Rust for every filter change is clean but has IPC overhead.
   - What's unclear: Whether 10K entries in JS memory is too much for `Array.filter()`.
   - Recommendation: **Hybrid approach** — load entries once from Rust (with time/outcome/decision filters applied server-side), then filter the loaded entries in JS for text search. This gives instant text search with server-side heavy filtering.

3. **Should we update the E2E mock data format?**
   - What we know: `tests/fixtures/mock-logs.json` uses wrong format (`{level, message, source}` vs real `LogEntry`).
   - Recommendation: **Yes**, update mock data to match real format. This is a prerequisite for E2E tests to work.

## Sources

### Primary (HIGH confidence)
- `cch_cli/src/logging.rs` — Log writing/querying infrastructure (lines 1-357)
- `cch_cli/src/models.rs` — LogEntry struct, Outcome/Decision/PolicyMode enums (lines 1345-1520)
- `~/.claude/logs/rulez.log` — Actual log file (14,382 lines, 4.97MB, JSON Lines format)
- `/websites/tanstack_virtual` (Context7) — TanStack Virtual API, `useVirtualizer` hook patterns
- `/tauri-apps/tauri-docs` (Context7) — Tauri v2 plugin-fs (`readTextFile`, `readTextFileLines`), plugin-dialog (`save`), plugin-clipboard (`writeText`)

### Secondary (MEDIUM confidence)
- Perplexity search — TanStack Virtual v3.13.18 as latest, comparison with react-virtuoso and react-window
- Existing codebase patterns: `rulez_ui/src/lib/tauri.ts` (invoke pattern), `rulez_ui/src-tauri/src/commands/debug.rs` (Tauri command pattern)

### Tertiary (LOW confidence)
- E2E test scaffolding in `rulez_ui/tests/log-viewer.spec.ts` — selectors may need adjustment as implementation evolves

## Metadata

**Confidence breakdown:**
- Standard stack: HIGH — Context7-verified APIs, npm-verified versions, project-verified patterns
- Architecture: HIGH — Based on existing codebase patterns (commands/, stores/, lib/tauri.ts)
- Log format: HIGH — Verified against actual log file and source code
- Pitfalls: HIGH — Based on analysis of actual data sizes, format mismatches, and IPC constraints
- Severity mapping: MEDIUM — Reasonable mapping but may need user feedback

**Research date:** 2026-02-12
**Valid until:** 2026-03-14 (30 days — stack is stable)
