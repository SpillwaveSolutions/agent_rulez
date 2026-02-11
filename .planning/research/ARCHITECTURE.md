# Architecture Integration: v1.5 RuleZ UI Production Features

**Domain:** Desktop Application for Policy Configuration Management
**Milestone:** v1.5 - Log Viewer, Settings Persistence, File Watching, Config Management
**Researched:** 2026-02-10
**Confidence:** HIGH (based on Tauri 2.0 docs, React patterns, existing M1 scaffold analysis)

---

## Executive Summary

v1.5 adds **four production features** to the existing RuleZ UI scaffold (M1) without breaking the established architecture:

1. **Log Viewer** — Real-time JSONL audit log streaming with virtual scrolling
2. **Settings Persistence** — User preferences stored via Tauri Store plugin
3. **File Watching** — Live config reload when hooks.yaml changes externally
4. **Config Diffing** — Visual comparison of global vs project configurations

**Integration Pattern:** All features follow the **existing M1 architecture** (Zustand stores, Tauri commands, dual-mode support). New features add:
- 1 new Zustand store (`logStore`)
- 1 new Tauri command module (`logs.rs`)
- 3 new component groups (logs/, settings/, diff/)
- 0 breaking changes to existing scaffold

**Performance Impact:** Virtual scrolling handles 100K+ log lines. File watching uses debounced Tauri fs-watch plugin (<50ms latency).

**Dependencies:** 2 new Rust crates (`tauri-plugin-fs 2.0`, `tauri-plugin-store 2.0`), 1 new React library (`react-window` for virtual scrolling).

---

## Existing Architecture (M1 Baseline)

### M1 Scaffold Overview

```
┌─────────────────────────────────────────────────────────────────────┐
│                           RuleZ UI (M1)                              │
├─────────────────────────────────────────────────────────────────────┤
│  React 18 + TypeScript + Tailwind CSS 4                             │
│                                                                       │
│  ┌─────────────────────┐  ┌─────────────────────┐                   │
│  │   Layout Layer      │  │   State Layer       │                   │
│  ├─────────────────────┤  ├─────────────────────┤                   │
│  │ • AppShell          │  │ • configStore       │                   │
│  │ • Header            │  │ • editorStore       │                   │
│  │ • Sidebar           │  │ • uiStore           │                   │
│  │ • MainContent       │  └─────────────────────┘                   │
│  │ • RightPanel        │                                             │
│  │ • StatusBar         │                                             │
│  └─────────────────────┘                                             │
│                                                                       │
│  ┌─────────────────────────────────────────────────────────┐         │
│  │              Component Groups (18 total)                │         │
│  ├─────────────────────────────────────────────────────────┤         │
│  │ editor/    │ YamlEditor, RuleTreeView, ValidationPanel  │         │
│  │ files/     │ FileTabBar                                 │         │
│  │ simulator/ │ DebugSimulator, EventForm, ResultView      │         │
│  │ layout/    │ 6 shell components                         │         │
│  │ ui/        │ ThemeToggle, ConfirmDialog                 │         │
│  └─────────────────────────────────────────────────────────┘         │
├─────────────────────────────────────────────────────────────────────┤
│                         Tauri 2.0 Backend                            │
│                                                                       │
│  Rust Commands (src-tauri/src/commands/)                             │
│  ┌────────────────────┐  ┌────────────────────┐                     │
│  │  config.rs         │  │  debug.rs          │                     │
│  ├────────────────────┤  ├────────────────────┤                     │
│  │ list_config_files  │  │ run_debug          │                     │
│  │ read_config        │  │ validate_config    │                     │
│  │ write_config       │  └────────────────────┘                     │
│  └────────────────────┘                                              │
├─────────────────────────────────────────────────────────────────────┤
│                      Dual-Mode Architecture                          │
│                                                                       │
│  src/lib/tauri.ts::isTauri()                                         │
│      ├─> Desktop Mode: Real Tauri IPC commands                      │
│      └─> Web Mode: Mock data fallbacks (for Playwright E2E)         │
└─────────────────────────────────────────────────────────────────────┘
```

### M1 Data Flow

```
User Action (e.g., open config file)
    ↓
Component Event Handler
    ↓
Tauri Abstraction Layer (src/lib/tauri.ts)
    ├─> isTauri() === true?
    │   ├─> YES: invoke("list_config_files", { projectDir })
    │   │       ↓
    │   │   Tauri Backend (commands/config.rs)
    │   │       ↓
    │   │   Filesystem I/O
    │   │       ↓
    │   │   Return ConfigFile[]
    │   │
    │   └─> NO: mockListConfigFiles(projectDir)
    │           ↓
    │       Return mock data
    ↓
Update Zustand Store (configStore.setGlobalConfig)
    ↓
React Re-render (subscribers notified)
    ↓
UI Update
```

### M1 Key Patterns

| Pattern | Implementation | Purpose |
|---------|---------------|---------|
| **Dual-Mode IPC** | `src/lib/tauri.ts::isTauri()` | Run in desktop (Tauri) or browser (E2E tests) |
| **State Management** | Zustand stores (configStore, editorStore, uiStore) | Global state without prop drilling |
| **Component Organization** | Grouped by feature (editor/, simulator/, layout/) | Maintainability |
| **Monaco Integration** | `@monaco-editor/react` + `monaco-yaml` | YAML editing with JSON Schema validation |
| **Tauri Commands** | Rust functions with `#[tauri::command]` macro | Type-safe IPC |

---

## v1.5 Integration Points

### Integration Point 1: Log Viewer with Streaming

**Where:** New component group `components/logs/` + new Tauri command `commands/logs.rs`

**Components to Add:**

```
src/components/logs/
├── LogViewer.tsx          # Main container (virtualized list)
├── LogFilterBar.tsx       # Filter controls (level, rule name, date range)
├── LogEntry.tsx           # Single log line (memoized)
└── LogStats.tsx           # Summary stats (total entries, time range)
```

**New Zustand Store:**

```typescript
// src/stores/logStore.ts (NEW)
import { create } from "zustand";

export interface LogEntry {
  timestamp: string;
  level: "debug" | "info" | "warn" | "error";
  event_type: string;
  rule_name?: string;
  outcome: "Allow" | "Block" | "Inject";
  reason?: string;
  evaluation_time_ms: number;
}

interface LogState {
  entries: LogEntry[];
  filter: {
    level: string[];
    outcome: string[];
    ruleNames: string[];
    dateRange: { start?: Date; end?: Date };
  };
  isLoading: boolean;
  isStreaming: boolean;
  totalEntries: number;
}

interface LogActions {
  loadLogs: () => Promise<void>;
  startStreaming: () => void;
  stopStreaming: () => void;
  setFilter: (filter: Partial<LogState["filter"]>) => void;
  clearLogs: () => void;
}

export const useLogStore = create<LogState & LogActions>((set, get) => ({
  // State
  entries: [],
  filter: {
    level: [],
    outcome: [],
    ruleNames: [],
    dateRange: {},
  },
  isLoading: false,
  isStreaming: false,
  totalEntries: 0,

  // Actions
  loadLogs: async () => {
    set({ isLoading: true });
    try {
      const logs = await readLogs({ limit: 1000 }); // From tauri.ts
      set({ entries: logs, totalEntries: logs.length, isLoading: false });
    } catch (error) {
      console.error("Failed to load logs:", error);
      set({ isLoading: false });
    }
  },

  startStreaming: () => {
    // Use Tauri event listener for file changes
    const unlisten = watchLogFile((newEntry: LogEntry) => {
      set((state) => ({
        entries: [...state.entries, newEntry],
        totalEntries: state.totalEntries + 1,
      }));
    });
    set({ isStreaming: true });
  },

  stopStreaming: () => {
    // Cleanup listener
    set({ isStreaming: false });
  },

  setFilter: (filter) =>
    set((state) => ({
      filter: { ...state.filter, ...filter },
    })),

  clearLogs: () => set({ entries: [], totalEntries: 0 }),
}));
```

**New Tauri Commands:**

```rust
// rulez-ui/src-tauri/src/commands/logs.rs (NEW)
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;

#[derive(Debug, Serialize, Deserialize)]
pub struct LogEntry {
    pub timestamp: String,
    pub level: String,
    pub event_type: String,
    pub rule_name: Option<String>,
    pub outcome: String,
    pub reason: Option<String>,
    pub evaluation_time_ms: f64,
}

#[derive(Debug, Deserialize)]
pub struct ReadLogsParams {
    pub limit: Option<usize>,
    pub offset: Option<usize>,
}

/// Get log file path (~/.claude/logs/rulez.log)
fn get_log_path() -> Option<PathBuf> {
    dirs::home_dir().map(|home| home.join(".claude").join("logs").join("rulez.log"))
}

/// Read logs from JSONL file with optional pagination
#[tauri::command]
pub async fn read_logs(params: ReadLogsParams) -> Result<Vec<LogEntry>, String> {
    let log_path = get_log_path()
        .ok_or_else(|| "Could not determine log path".to_string())?;

    if !log_path.exists() {
        return Ok(vec![]);
    }

    let file = File::open(&log_path)
        .map_err(|e| format!("Failed to open log file: {}", e))?;

    let reader = BufReader::new(file);
    let offset = params.offset.unwrap_or(0);
    let limit = params.limit.unwrap_or(1000);

    let entries: Result<Vec<LogEntry>, String> = reader
        .lines()
        .skip(offset)
        .take(limit)
        .map(|line| {
            let line = line.map_err(|e| format!("Failed to read line: {}", e))?;
            serde_json::from_str(&line)
                .map_err(|e| format!("Failed to parse log entry: {}", e))
        })
        .collect();

    entries
}

/// Count total log entries
#[tauri::command]
pub async fn count_logs() -> Result<usize, String> {
    let log_path = get_log_path()
        .ok_or_else(|| "Could not determine log path".to_string())?;

    if !log_path.exists() {
        return Ok(0);
    }

    let file = File::open(&log_path)
        .map_err(|e| format!("Failed to open log file: {}", e))?;

    let reader = BufReader::new(file);
    Ok(reader.lines().count())
}

/// Clear log file
#[tauri::command]
pub async fn clear_logs() -> Result<(), String> {
    let log_path = get_log_path()
        .ok_or_else(|| "Could not determine log path".to_string())?;

    if log_path.exists() {
        std::fs::write(&log_path, "")
            .map_err(|e| format!("Failed to clear log file: {}", e))?;
    }

    Ok(())
}
```

**Frontend Wrapper (Dual-Mode):**

```typescript
// src/lib/tauri.ts (ADD)
import type { LogEntry, ReadLogsParams } from "@/types";

export async function readLogs(params: ReadLogsParams): Promise<LogEntry[]> {
  if (isTauri()) {
    const { invoke } = await import("@tauri-apps/api/core");
    return invoke<LogEntry[]>("read_logs", params);
  }
  return mockReadLogs(params);
}

export async function countLogs(): Promise<number> {
  if (isTauri()) {
    const { invoke } = await import("@tauri-apps/api/core");
    return invoke<number>("count_logs");
  }
  return mockCountLogs();
}

export async function clearLogs(): Promise<void> {
  if (isTauri()) {
    const { invoke } = await import("@tauri-apps/api/core");
    return invoke<void>("clear_logs");
  }
  return mockClearLogs();
}

// Mock implementations for web mode
async function mockReadLogs(params: ReadLogsParams): Promise<LogEntry[]> {
  await delay(100);
  // Return sample JSONL data
  return [
    {
      timestamp: new Date().toISOString(),
      level: "info",
      event_type: "PreToolUse",
      rule_name: "block-force-push",
      outcome: "Block",
      reason: "Force push prohibited",
      evaluation_time_ms: 1.2,
    },
    // ... more mock entries
  ].slice(params.offset || 0, (params.offset || 0) + (params.limit || 1000));
}
```

**Virtual Scrolling with react-window:**

```typescript
// src/components/logs/LogViewer.tsx (NEW)
import { FixedSizeList as List } from "react-window";
import { useLogStore } from "@/stores/logStore";
import { LogEntry } from "./LogEntry";

export function LogViewer() {
  const { entries, loadLogs, isLoading } = useLogStore();

  useEffect(() => {
    loadLogs();
  }, []);

  if (isLoading) {
    return <div>Loading logs...</div>;
  }

  return (
    <div className="h-full">
      <List
        height={600}
        itemCount={entries.length}
        itemSize={40}
        width="100%"
      >
        {({ index, style }) => (
          <LogEntry entry={entries[index]!} style={style} />
        )}
      </List>
    </div>
  );
}
```

**Integration with Existing Layout:**

```typescript
// src/stores/uiStore.ts (MODIFY)
export type RightPanelTab = "simulator" | "tree" | "logs";  // ← Add "logs"

// src/components/layout/RightPanel.tsx (MODIFY)
export function RightPanel() {
  const { rightPanelTab } = useUIStore();

  return (
    <div className="w-96 border-l border-gray-200 dark:border-gray-800">
      <div className="flex border-b">
        <TabButton tab="simulator" label="Simulator" />
        <TabButton tab="tree" label="Tree" />
        <TabButton tab="logs" label="Logs" />  {/* ← NEW */}
      </div>

      <div className="p-4">
        {rightPanelTab === "simulator" && <DebugSimulator />}
        {rightPanelTab === "tree" && <RuleTreeView />}
        {rightPanelTab === "logs" && <LogViewer />}  {/* ← NEW */}
      </div>
    </div>
  );
}
```

**Modified Files:**

- `rulez-ui/src/stores/logStore.ts`: NEW (log state management)
- `rulez-ui/src/components/logs/*.tsx`: NEW (4 components)
- `rulez-ui/src-tauri/src/commands/logs.rs`: NEW (JSONL reader)
- `rulez-ui/src-tauri/src/commands/mod.rs`: Add `pub mod logs;`
- `rulez-ui/src/lib/tauri.ts`: Add log functions with dual-mode support
- `rulez-ui/src/types/index.ts`: Add LogEntry, ReadLogsParams types
- `rulez-ui/package.json`: Add `react-window` dependency
- `rulez-ui/src-tauri/Cargo.toml`: No new deps (uses std::fs)

**Performance:**

```
Virtual scrolling:    Renders only ~20 visible items (not 100K)
JSONL parsing:        ~1-2ms per 1000 lines
Initial load (1K):    <100ms
Memory footprint:     ~100 bytes per entry in state
```

---

### Integration Point 2: Settings Persistence with Tauri Store Plugin

**Where:** New component `components/settings/` + Tauri Store plugin integration

**Settings Structure:**

```typescript
// src/types/index.ts (ADD)
export interface UserSettings {
  theme: "light" | "dark" | "system";
  editorFontSize: number;
  autoSave: boolean;
  autoSaveDelay: number; // ms
  defaultConfigScope: "global" | "project";
  rulez_binary_path?: string; // Custom binary location
  logLevel: "debug" | "info" | "warn" | "error";
}
```

**New Tauri Commands:**

```rust
// rulez-ui/src-tauri/src/commands/settings.rs (NEW)
use serde::{Deserialize, Serialize};
use tauri::State;
use tauri_plugin_store::StoreExt;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UserSettings {
    pub theme: String,
    pub editor_font_size: u32,
    pub auto_save: bool,
    pub auto_save_delay: u32,
    pub default_config_scope: String,
    pub rulez_binary_path: Option<String>,
    pub log_level: String,
}

impl Default for UserSettings {
    fn default() -> Self {
        Self {
            theme: "system".to_string(),
            editor_font_size: 14,
            auto_save: true,
            auto_save_delay: 2000,
            default_config_scope: "project".to_string(),
            rulez_binary_path: None,
            log_level: "info".to_string(),
        }
    }
}

/// Load settings from Tauri Store
#[tauri::command]
pub async fn load_settings(app: tauri::AppHandle) -> Result<UserSettings, String> {
    let store = app.store("settings.json")
        .map_err(|e| format!("Failed to load store: {}", e))?;

    let settings = store.get("user_settings")
        .and_then(|v| serde_json::from_value(v).ok())
        .unwrap_or_default();

    Ok(settings)
}

/// Save settings to Tauri Store
#[tauri::command]
pub async fn save_settings(
    app: tauri::AppHandle,
    settings: UserSettings,
) -> Result<(), String> {
    let store = app.store("settings.json")
        .map_err(|e| format!("Failed to load store: {}", e))?;

    store.set("user_settings", serde_json::to_value(&settings).unwrap());
    store.save()
        .map_err(|e| format!("Failed to save settings: {}", e))?;

    Ok(())
}
```

**Tauri Store Plugin Setup:**

```rust
// rulez-ui/src-tauri/src/main.rs (MODIFY)
use tauri_plugin_store::StoreBuilder;

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_store::Builder::default().build())  // ← NEW
        .invoke_handler(tauri::generate_handler![
            // ... existing commands
            commands::settings::load_settings,
            commands::settings::save_settings,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
```

**Integration with uiStore:**

```typescript
// src/stores/uiStore.ts (MODIFY)
import { loadSettings, saveSettings } from "@/lib/tauri";

export const useUIStore = create<UIState & UIActions>((set, get) => ({
  // ... existing state

  // NEW: Initialize from Tauri Store
  initSettings: async () => {
    try {
      const settings = await loadSettings();
      set({
        theme: settings.theme,
        // ... other settings
      });
    } catch (error) {
      console.error("Failed to load settings:", error);
    }
  },

  // MODIFY: Persist theme changes
  setTheme: async (theme) => {
    set({ theme });
    try {
      const settings = { ...get(), theme };
      await saveSettings(settings);
    } catch (error) {
      console.error("Failed to save theme:", error);
    }
  },
}));
```

**Settings UI Component:**

```typescript
// src/components/settings/SettingsPanel.tsx (NEW)
export function SettingsPanel() {
  const { theme, setTheme } = useUIStore();
  const [binaryPath, setBinaryPath] = useState("");

  async function handleBinaryPathChange() {
    // Use Tauri dialog to select binary
    const selected = await openFileDialog({
      filters: [{ name: "Executables", extensions: ["exe", ""] }],
    });
    if (selected) {
      setBinaryPath(selected);
      await saveSettings({ rulez_binary_path: selected });
    }
  }

  return (
    <div className="space-y-6">
      <Section title="Appearance">
        <ThemeToggle />
        <FontSizeSlider />
      </Section>

      <Section title="Editor">
        <Toggle label="Auto-save" />
        <NumberInput label="Auto-save delay (ms)" />
      </Section>

      <Section title="RuleZ Binary">
        <FilePathInput
          label="Custom binary path"
          value={binaryPath}
          onChange={handleBinaryPathChange}
        />
      </Section>
    </div>
  );
}
```

**Modified Files:**

- `rulez-ui/src-tauri/src/commands/settings.rs`: NEW (settings CRUD)
- `rulez-ui/src-tauri/src/main.rs`: Register store plugin
- `rulez-ui/src-tauri/Cargo.toml`: Add `tauri-plugin-store = "2.0"`
- `rulez-ui/src/stores/uiStore.ts`: Add settings persistence
- `rulez-ui/src/components/settings/*.tsx`: NEW (settings UI)
- `rulez-ui/package.json`: Add `@tauri-apps/plugin-store`

**Storage Location:**

```
macOS:     ~/Library/Application Support/com.rulez.ui/settings.json
Linux:     ~/.config/rulez-ui/settings.json
Windows:   C:\Users\<user>\AppData\Roaming\com.rulez.ui\settings.json
```

---

### Integration Point 3: File Watching for Live Config Reload

**Where:** Enhance existing `configStore` + add Tauri fs-watch plugin

**New Tauri Commands:**

```rust
// rulez-ui/src-tauri/src/commands/config.rs (MODIFY)
use tauri_plugin_fs::FsExt;

/// Watch config file for changes
#[tauri::command]
pub async fn watch_config_file(
    app: tauri::AppHandle,
    path: String,
) -> Result<(), String> {
    let app_clone = app.clone();
    let path_clone = path.clone();

    // Use Tauri fs plugin for watching
    app.fs_scope()
        .watch(
            vec![path.clone()],
            move |event| {
                // Emit event to frontend when file changes
                app_clone.emit("config-file-changed", &path_clone)
                    .expect("Failed to emit config change event");
            },
            Default::default(),
        )
        .map_err(|e| format!("Failed to watch file: {}", e))?;

    Ok(())
}

/// Stop watching config file
#[tauri::command]
pub async fn unwatch_config_file(
    app: tauri::AppHandle,
    path: String,
) -> Result<(), String> {
    // Cleanup watcher (implementation depends on plugin API)
    Ok(())
}
```

**Frontend Integration:**

```typescript
// src/lib/tauri.ts (ADD)
export async function watchConfigFile(
  path: string,
  callback: () => void
): Promise<() => void> {
  if (isTauri()) {
    const { invoke, listen } = await import("@tauri-apps/api/core");

    // Start watching
    await invoke("watch_config_file", { path });

    // Listen for change events
    const unlisten = await listen("config-file-changed", (event) => {
      if (event.payload === path) {
        callback();
      }
    });

    return async () => {
      unlisten();
      await invoke("unwatch_config_file", { path });
    };
  }

  // Mock mode: no watching
  return () => {};
}
```

**ConfigStore Integration:**

```typescript
// src/stores/configStore.ts (MODIFY)
import { watchConfigFile, readConfig } from "@/lib/tauri";

export const useConfigStore = create<ConfigState & ConfigActions>((set, get) => ({
  // ... existing state

  // NEW: Start watching active file
  watchActiveFile: async () => {
    const { activeFile } = get();
    if (!activeFile) return;

    const unlisten = await watchConfigFile(activeFile, async () => {
      // Reload file when changed externally
      const content = await readConfig(activeFile);
      set((state) => {
        const newOpenFiles = new Map(state.openFiles);
        const fileState = newOpenFiles.get(activeFile);
        if (fileState) {
          newOpenFiles.set(activeFile, {
            ...fileState,
            content,
            originalContent: content,
            modified: false,
          });
        }
        return { openFiles: newOpenFiles };
      });
    });

    // Store cleanup function
    set({ unwatchActiveFile: unlisten });
  },

  // Cleanup on file close
  closeFile: (path) => {
    const { unwatchActiveFile } = get();
    if (unwatchActiveFile) {
      unwatchActiveFile();
    }
    // ... existing close logic
  },
}));
```

**User Notification:**

```typescript
// src/components/editor/YamlEditor.tsx (MODIFY)
export function YamlEditor() {
  const { activeFile, watchActiveFile } = useConfigStore();

  useEffect(() => {
    if (activeFile) {
      watchActiveFile();
    }
  }, [activeFile]);

  // Show toast notification when file reloaded
  useEffect(() => {
    const file = openFiles.get(activeFile);
    if (file && !file.modified) {
      toast.info("Config file reloaded from disk");
    }
  }, [openFiles]);

  // ...
}
```

**Modified Files:**

- `rulez-ui/src-tauri/src/commands/config.rs`: Add watch/unwatch commands
- `rulez-ui/src-tauri/Cargo.toml`: Add `tauri-plugin-fs = { version = "2.0", features = ["watch"] }`
- `rulez-ui/src-tauri/src/main.rs`: Register fs plugin
- `rulez-ui/src/stores/configStore.ts`: Add file watching logic
- `rulez-ui/src/lib/tauri.ts`: Add watchConfigFile wrapper
- `rulez-ui/package.json`: Add `@tauri-apps/plugin-fs`

**Performance:**

```
Watch latency:        <50ms (debounced by Tauri plugin)
File reload:          Same as existing readConfig (~30ms)
Memory overhead:      ~1 KB per watched file
```

---

### Integration Point 4: Config Diffing (Global vs Project)

**Where:** New component group `components/diff/` + new UI tab

**Components to Add:**

```
src/components/diff/
├── ConfigDiff.tsx         # Main container
├── DiffView.tsx           # Side-by-side diff display
└── DiffStats.tsx          # Summary (additions, deletions)
```

**New Tauri Command:**

```rust
// rulez-ui/src-tauri/src/commands/config.rs (ADD)
use similar::{ChangeTag, TextDiff};

#[derive(Debug, Serialize)]
pub struct DiffHunk {
    pub old_line: Option<usize>,
    pub new_line: Option<usize>,
    pub change_type: String,  // "add", "delete", "equal"
    pub content: String,
}

/// Compare two config files and return diff hunks
#[tauri::command]
pub async fn diff_configs(
    path1: String,
    path2: String,
) -> Result<Vec<DiffHunk>, String> {
    let content1 = read_config(path1).await?;
    let content2 = read_config(path2).await?;

    let diff = TextDiff::from_lines(&content1, &content2);
    let mut hunks = Vec::new();

    for (idx, change) in diff.iter_all_changes().enumerate() {
        let change_type = match change.tag() {
            ChangeTag::Delete => "delete",
            ChangeTag::Insert => "add",
            ChangeTag::Equal => "equal",
        };

        hunks.push(DiffHunk {
            old_line: change.old_index().map(|i| i + 1),
            new_line: change.new_index().map(|i| i + 1),
            change_type: change_type.to_string(),
            content: change.to_string(),
        });
    }

    Ok(hunks)
}
```

**Frontend Component (Using react-diff-viewer):**

```typescript
// src/components/diff/ConfigDiff.tsx (NEW)
import ReactDiffViewer from "react-diff-viewer";
import { useConfigStore } from "@/stores/configStore";

export function ConfigDiff() {
  const { globalConfig, projectConfig } = useConfigStore();
  const [globalContent, setGlobalContent] = useState("");
  const [projectContent, setProjectContent] = useState("");

  useEffect(() => {
    async function loadConfigs() {
      if (globalConfig?.path) {
        const content = await readConfig(globalConfig.path);
        setGlobalContent(content);
      }
      if (projectConfig?.path) {
        const content = await readConfig(projectConfig.path);
        setProjectContent(content);
      }
    }
    loadConfigs();
  }, [globalConfig, projectConfig]);

  return (
    <div className="h-full overflow-auto">
      <div className="p-4 border-b">
        <h2 className="text-lg font-semibold">Config Comparison</h2>
        <p className="text-sm text-gray-600">
          Global vs Project Configuration
        </p>
      </div>

      <ReactDiffViewer
        oldValue={globalContent}
        newValue={projectContent}
        splitView={true}
        leftTitle="Global (~/.claude/hooks.yaml)"
        rightTitle="Project (.claude/hooks.yaml)"
        styles={{
          variables: {
            dark: {
              diffViewerBackground: "#1A1A1A",
              addedBackground: "#044B53",
              removedBackground: "#5E1A1A",
            },
          },
        }}
      />
    </div>
  );
}
```

**Integration with Sidebar:**

```typescript
// src/components/layout/Sidebar.tsx (MODIFY)
export function Sidebar() {
  const { globalConfig, projectConfig, setActiveFile } = useConfigStore();
  const { setRightPanelTab } = useUIStore();

  function showDiff() {
    setRightPanelTab("diff");  // Switch to diff view
  }

  return (
    <div className="w-64 border-r">
      <div className="p-4">
        <h3 className="font-semibold mb-2">Config Files</h3>

        <FileItem
          label="Global"
          file={globalConfig}
          onSelect={() => setActiveFile(globalConfig?.path)}
        />

        <FileItem
          label="Project"
          file={projectConfig}
          onSelect={() => setActiveFile(projectConfig?.path)}
        />

        {/* NEW: Show diff button */}
        {globalConfig?.exists && projectConfig?.exists && (
          <button
            onClick={showDiff}
            className="mt-2 w-full px-3 py-2 text-sm border rounded hover:bg-gray-100"
          >
            Compare Configs
          </button>
        )}
      </div>
    </div>
  );
}
```

**Modified Files:**

- `rulez-ui/src/components/diff/*.tsx`: NEW (3 components)
- `rulez-ui/src-tauri/src/commands/config.rs`: Add diff_configs command
- `rulez-ui/src-tauri/Cargo.toml`: Add `similar = "2.4"` (diff algorithm)
- `rulez-ui/package.json`: Add `react-diff-viewer`
- `rulez-ui/src/stores/uiStore.ts`: Add "diff" to RightPanelTab
- `rulez-ui/src/components/layout/Sidebar.tsx`: Add compare button

**Performance:**

```
Diff computation:     ~5ms for typical config (50-100 lines)
Rendering:            Virtual scrolling handles large diffs
Memory:               ~200 bytes per diff hunk
```

---

## v1.5 Component Architecture

### Updated System Overview

```
┌─────────────────────────────────────────────────────────────────────────┐
│                       RuleZ UI v1.5 Architecture                         │
├─────────────────────────────────────────────────────────────────────────┤
│  Frontend (React 18 + TypeScript + Tailwind)                            │
│                                                                           │
│  ┌───────────────────────┐  ┌───────────────────────┐                   │
│  │   Component Groups    │  │   State Management    │                   │
│  ├───────────────────────┤  ├───────────────────────┤                   │
│  │ layout/    (6)        │  │ configStore           │                   │
│  │ editor/    (4)        │  │ editorStore           │                   │
│  │ simulator/ (3)        │  │ uiStore               │                   │
│  │ files/     (1)        │  │ logStore       ← NEW  │                   │
│  │ ui/        (2)        │  └───────────────────────┘                   │
│  │ logs/      (4) ← NEW  │                                               │
│  │ settings/  (3) ← NEW  │  ┌───────────────────────┐                   │
│  │ diff/      (3) ← NEW  │  │   Tauri Abstraction   │                   │
│  └───────────────────────┘  ├───────────────────────┤                   │
│                              │ isTauri() branching   │                   │
│  Total: 26 components        │ Mock data fallbacks   │                   │
│  (18 from M1 + 8 new)        └───────────────────────┘                   │
├─────────────────────────────────────────────────────────────────────────┤
│  Backend (Tauri 2.0 Rust Commands)                                       │
│                                                                           │
│  ┌─────────────────────┐  ┌─────────────────────┐  ┌─────────────────┐ │
│  │  config.rs          │  │  debug.rs           │  │  logs.rs ← NEW  │ │
│  ├─────────────────────┤  ├─────────────────────┤  ├─────────────────┤ │
│  │ list_config_files   │  │ run_debug           │  │ read_logs       │ │
│  │ read_config         │  │ validate_config     │  │ count_logs      │ │
│  │ write_config        │  └─────────────────────┘  │ clear_logs      │ │
│  │ watch_config ← NEW  │                            └─────────────────┘ │
│  │ unwatch_config ← NEW│  ┌─────────────────────┐                       │
│  │ diff_configs ← NEW  │  │  settings.rs ← NEW  │                       │
│  └─────────────────────┘  ├─────────────────────┤                       │
│                            │ load_settings       │                       │
│                            │ save_settings       │                       │
│                            └─────────────────────┘                       │
├─────────────────────────────────────────────────────────────────────────┤
│  Tauri Plugins                                                            │
│                                                                           │
│  • tauri-plugin-fs (file watching)                                        │
│  • tauri-plugin-store (settings persistence)                              │
└─────────────────────────────────────────────────────────────────────────┘
```

### Data Flow: Log Viewer

```
User opens Logs tab
    ↓
LogViewer component mounts
    ↓
useLogStore.loadLogs()
    ↓
readLogs({ limit: 1000 }) [tauri.ts]
    ├─> isTauri() === true?
    │   ├─> YES: invoke("read_logs", { limit: 1000 })
    │   │       ↓
    │   │   Tauri Backend (commands/logs.rs)
    │   │       ↓
    │   │   Open ~/.claude/logs/rulez.log
    │   │       ↓
    │   │   BufReader::lines().take(1000)
    │   │       ↓
    │   │   Parse JSONL → Vec<LogEntry>
    │   │       ↓
    │   │   Return to frontend
    │   │
    │   └─> NO: mockReadLogs({ limit: 1000 })
    │           ↓
    │       Return sample data
    ↓
Update logStore.entries
    ↓
react-window virtualizes rendering (only ~20 visible rows)
    ↓
User sees log list (instant scroll for 100K+ entries)
```

### Data Flow: Settings Persistence

```
App initialization
    ↓
useUIStore.initSettings()
    ↓
loadSettings() [tauri.ts]
    ↓
invoke("load_settings")
    ↓
Tauri Backend (commands/settings.rs)
    ↓
Open Tauri Store (settings.json)
    ↓
store.get("user_settings") → UserSettings
    ↓
Return to frontend
    ↓
Update uiStore state
    ↓
UI renders with persisted theme/font/etc.

---

User changes theme
    ↓
useUIStore.setTheme("dark")
    ↓
Update local state
    ↓
saveSettings({ theme: "dark", ... }) [tauri.ts]
    ↓
invoke("save_settings", { settings })
    ↓
Tauri Backend (commands/settings.rs)
    ↓
store.set("user_settings", settings)
store.save()  // Persisted to disk
```

### Data Flow: File Watching

```
User opens global config
    ↓
configStore.watchActiveFile()
    ↓
watchConfigFile(path, callback) [tauri.ts]
    ↓
invoke("watch_config_file", { path })
    ↓
Tauri Backend (commands/config.rs)
    ↓
app.fs_scope().watch(path, |event| {
    app.emit("config-file-changed", path);
})
    ↓
Frontend listens for "config-file-changed" event
    ↓

--- External change occurs (e.g., user edits in VS Code) ---

Tauri fs watcher detects change
    ↓
Emit "config-file-changed" event
    ↓
Frontend callback fires
    ↓
readConfig(path) to reload content
    ↓
Update configStore.openFiles with new content
    ↓
Monaco editor re-renders with updated content
    ↓
Show toast: "Config file reloaded from disk"
```

---

## Architectural Patterns (v1.5)

### Pattern 1: Virtual Scrolling for Large Lists

**What:** Render only visible items in a scrollable list, not the entire dataset.

**When to use:** Lists with 1000+ items (log entries, rule lists, search results).

**Trade-offs:**
- **Pro:** Handles 100K+ items without performance degradation
- **Pro:** Constant memory usage regardless of list size
- **Con:** Requires fixed-height items (or dynamic measurement)
- **Con:** Slightly more complex than naive `.map()` rendering

**Example:**

```typescript
import { FixedSizeList as List } from "react-window";

function LogViewer() {
  const entries = useLogStore((state) => state.entries); // 50K entries

  return (
    <List
      height={600}
      itemCount={entries.length}
      itemSize={40}
      width="100%"
    >
      {({ index, style }) => (
        <div style={style}>
          {entries[index].message}
        </div>
      )}
    </List>
  );
}
```

---

### Pattern 2: Zustand Store Composition (Slices)

**What:** Split large stores into focused slices, compose into single store or keep separate.

**When to use:** When state domains are independent (logs, config, UI preferences).

**Trade-offs:**
- **Pro:** Clear separation of concerns
- **Pro:** Easier to test individual slices
- **Con:** Multiple stores = multiple subscriptions (minimal overhead)
- **Con:** Cross-slice dependencies require careful design

**Example:**

```typescript
// APPROACH 1: Separate stores (v1.5 choice)
const useConfigStore = create<ConfigState & ConfigActions>(...);
const useLogStore = create<LogState & LogActions>(...);
const useUIStore = create<UIState & UIActions>(...);

// Usage: Each component subscribes only to needed stores
function LogViewer() {
  const { entries } = useLogStore();  // Only re-renders on log changes
  const { theme } = useUIStore();     // Only re-renders on theme changes
}

// APPROACH 2: Single store with slices (alternative)
const useAppStore = create<AppState>((set, get) => ({
  ...createConfigSlice(set, get),
  ...createLogSlice(set, get),
  ...createUISlice(set, get),
}));
```

**Recommendation for v1.5:** Separate stores (logStore, configStore, uiStore) because:
- Log state is completely independent of config/UI state
- Prevents unnecessary re-renders (log updates don't trigger config components)
- Matches M1 pattern (already has 3 separate stores)

---

### Pattern 3: Tauri Event Listeners for File Watching

**What:** Use Tauri event system to notify frontend of backend changes.

**When to use:** File watching, long-running tasks, background processes.

**Trade-offs:**
- **Pro:** Decouples backend from frontend (event-driven)
- **Pro:** Supports multiple listeners for same event
- **Con:** Requires cleanup (`unlisten()` on component unmount)
- **Con:** Events are fire-and-forget (no return value)

**Example:**

```typescript
useEffect(() => {
  let unlisten: (() => void) | null = null;

  async function setupWatcher() {
    const { listen } = await import("@tauri-apps/api/event");

    unlisten = await listen("config-file-changed", (event) => {
      console.log("Config changed:", event.payload);
      reloadConfig(event.payload);
    });
  }

  setupWatcher();

  // Cleanup on unmount
  return () => {
    if (unlisten) unlisten();
  };
}, []);
```

---

### Pattern 4: Dual-Mode IPC with Mock Fallbacks (M1 Pattern)

**What:** All Tauri commands have web-mode fallbacks for E2E testing.

**When to use:** Always, to support Playwright tests without building Tauri app.

**Trade-offs:**
- **Pro:** E2E tests run in CI without full Tauri build (2 min vs 10 min)
- **Pro:** Frontend developers can work without Rust toolchain
- **Con:** Mock data must be kept in sync with real Tauri API
- **Con:** Adds branching logic to every IPC call

**Example:**

```typescript
// src/lib/tauri.ts
export async function readLogs(params: ReadLogsParams): Promise<LogEntry[]> {
  if (isTauri()) {
    const { invoke } = await import("@tauri-apps/api/core");
    return invoke<LogEntry[]>("read_logs", params);
  }
  return mockReadLogs(params);  // For Playwright tests
}

function mockReadLogs(params: ReadLogsParams): LogEntry[] {
  // Return realistic sample data
  return [
    { timestamp: "2026-02-10T12:00:00Z", level: "info", ... },
    // ...
  ].slice(params.offset || 0, params.limit || 1000);
}
```

---

### Pattern 5: Error Boundaries for Graceful Degradation

**What:** Wrap components in ErrorBoundary to catch rendering errors and show fallback UI.

**When to use:** Around major feature areas (editor, logs, simulator).

**Trade-offs:**
- **Pro:** Prevents entire app crash from single component error
- **Pro:** Better UX (show error, allow retry)
- **Con:** Doesn't catch async errors (use try/catch in async code)
- **Con:** Requires class component or `react-error-boundary` library

**Example:**

```typescript
// Use react-error-boundary library (functional components)
import { ErrorBoundary } from "react-error-boundary";

function ErrorFallback({ error, resetErrorBoundary }) {
  return (
    <div className="p-4 border border-red-500 rounded">
      <h2 className="text-lg font-semibold text-red-600">
        Log Viewer Error
      </h2>
      <pre className="mt-2 text-sm">{error.message}</pre>
      <button onClick={resetErrorBoundary} className="mt-4">
        Try Again
      </button>
    </div>
  );
}

export function LogViewerWithErrorBoundary() {
  return (
    <ErrorBoundary FallbackComponent={ErrorFallback}>
      <LogViewer />
    </ErrorBoundary>
  );
}
```

**Where to add in v1.5:**

- Wrap `<LogViewer />` (JSONL parsing can fail)
- Wrap `<ConfigDiff />` (diff algorithm can throw)
- Wrap `<YamlEditor />` (Monaco can crash on malformed YAML)

---

## Anti-Patterns (v1.5)

### Anti-Pattern 1: Reading Entire Log File into Memory

**What people do:** Load all 100K log entries into state at once.

**Why it's wrong:** Crashes browser with large log files (>10 MB).

**Do this instead:**

```typescript
// WRONG: Load everything
const allLogs = await readLogs({ limit: 999999 });  // ❌ 100K entries

// RIGHT: Pagination + virtual scrolling
const logs = await readLogs({ limit: 1000, offset: 0 });  // ✓ 1K entries
// + use react-window to render only visible rows
```

---

### Anti-Pattern 2: Polling for File Changes

**What people do:** Use `setInterval()` to check if config file changed.

**Why it's wrong:** Wastes CPU, battery, and has 1-5 second latency.

**Do this instead:**

```typescript
// WRONG: Poll every second
setInterval(async () => {
  const newContent = await readConfig(path);
  if (newContent !== oldContent) {
    updateContent(newContent);
  }
}, 1000);  // ❌ Polls every second

// RIGHT: Use Tauri file watcher
watchConfigFile(path, () => {
  reloadConfig(path);  // ✓ Triggered immediately on change (<50ms)
});
```

---

### Anti-Pattern 3: Storing Large Objects in Tauri Store

**What people do:** Store entire config file content in settings.

**Why it's wrong:** Tauri Store is for preferences (small data), not content (large data).

**Do this instead:**

```typescript
// WRONG: Store file content
await saveSettings({
  theme: "dark",
  lastConfigContent: "...(5000 lines of YAML)...",  // ❌ Too large
});

// RIGHT: Store only file path reference
await saveSettings({
  theme: "dark",
  lastOpenedConfigPath: "~/.claude/hooks.yaml",  // ✓ Small metadata
});
```

---

### Anti-Pattern 4: Not Cleaning Up Event Listeners

**What people do:** Register Tauri event listeners without `unlisten()` on unmount.

**Why it's wrong:** Memory leaks, duplicate event handlers.

**Do this instead:**

```typescript
// WRONG: No cleanup
useEffect(() => {
  listen("config-file-changed", handleChange);  // ❌ Leaks on unmount
}, []);

// RIGHT: Return cleanup function
useEffect(() => {
  let unlisten: (() => void) | null = null;

  async function setup() {
    unlisten = await listen("config-file-changed", handleChange);
  }
  setup();

  return () => {
    if (unlisten) unlisten();  // ✓ Cleanup
  };
}, []);
```

---

## Build Order (Recommended)

### Dependencies

```
Phase 1 (Log Viewer)  ─────────┐
                                ├─> (no dependencies, pure additive)
Phase 2 (Settings)     ─────────┤
                                │
Phase 3 (File Watching) ────────┤ (depends on configStore from M1)
                                │
Phase 4 (Config Diffing) ───────┘ (depends on readConfig from M1)
```

### Recommended Order

**Phase 1: Log Viewer** (3-4 days)
- Add `logStore.ts` with loadLogs/clearLogs actions
- Create `commands/logs.rs` with read_logs/count_logs/clear_logs
- Build `LogViewer`, `LogEntry`, `LogFilterBar` components
- Add `react-window` for virtual scrolling
- Add "logs" tab to RightPanel
- Test with large log files (10K+ entries)

**Phase 2: Settings Persistence** (2-3 days)
- Add `tauri-plugin-store` to Cargo.toml
- Create `commands/settings.rs` with load/save functions
- Integrate with existing `uiStore` for theme/preferences
- Build `SettingsPanel` component
- Add settings menu to Header
- Test persistence across app restarts

**Phase 3: File Watching** (2-3 days)
- Add `tauri-plugin-fs` with watch feature
- Add `watch_config_file` command to `config.rs`
- Integrate with `configStore.watchActiveFile()`
- Add toast notification on file reload
- Test external file edits (VS Code, terminal)
- Handle edge cases (file deleted, permission errors)

**Phase 4: Config Diffing** (2 days)
- Add `similar` crate for diff algorithm
- Add `diff_configs` command to `config.rs`
- Create `ConfigDiff`, `DiffView` components
- Add `react-diff-viewer` for rendering
- Add "Compare" button to Sidebar
- Test with real global/project configs

**Total Estimated Time:** 9-12 days (with parallelization: 7-9 days)

**Parallelization:** Phases 1 and 2 are fully independent. Phases 3 and 4 depend on M1 scaffold but not on each other.

---

## Success Criteria

### Phase 1: Log Viewer
- [ ] Loads 1000 log entries in <100ms
- [ ] Virtual scrolling handles 50K+ entries without lag
- [ ] Filter controls work (level, outcome, rule name)
- [ ] Clear logs command empties rulez.log file
- [ ] Dual-mode: Web fallback shows mock data
- [ ] Logs tab appears in RightPanel

### Phase 2: Settings Persistence
- [ ] Settings survive app restart (theme, font, etc.)
- [ ] Tauri Store creates settings.json in app data dir
- [ ] SettingsPanel UI allows customization
- [ ] rulez_binary_path setting allows custom binary location
- [ ] Auto-save settings work with debouncing

### Phase 3: File Watching
- [ ] External config edits trigger reload (<1 second latency)
- [ ] Toast notification shown on file reload
- [ ] No memory leaks (unlisten() called on unmount)
- [ ] Handles edge cases (file deleted, permission denied)
- [ ] Watch command cleanup on file close

### Phase 4: Config Diffing
- [ ] Side-by-side diff renders correctly
- [ ] Diff stats show additions/deletions
- [ ] Syntax highlighting in diff view
- [ ] Compare button only shown when both configs exist
- [ ] Handles configs with different line endings

### Overall v1.5
- [ ] No regressions in M1 features (editor, simulator)
- [ ] All new features work in dual-mode (desktop + web)
- [ ] E2E tests pass for new components
- [ ] Bundle size increase <500 KB
- [ ] Binary size increase <1 MB

---

## Modified/New Files Summary

### New Rust Files
- `rulez-ui/src-tauri/src/commands/logs.rs` (JSONL reader)
- `rulez-ui/src-tauri/src/commands/settings.rs` (Tauri Store wrapper)

### Modified Rust Files
- `rulez-ui/src-tauri/src/commands/config.rs` (add watch/unwatch/diff commands)
- `rulez-ui/src-tauri/src/commands/mod.rs` (register new modules)
- `rulez-ui/src-tauri/src/main.rs` (register plugins and commands)
- `rulez-ui/src-tauri/Cargo.toml` (add dependencies)

### New Frontend Files
- `rulez-ui/src/stores/logStore.ts`
- `rulez-ui/src/components/logs/LogViewer.tsx`
- `rulez-ui/src/components/logs/LogEntry.tsx`
- `rulez-ui/src/components/logs/LogFilterBar.tsx`
- `rulez-ui/src/components/logs/LogStats.tsx`
- `rulez-ui/src/components/settings/SettingsPanel.tsx`
- `rulez-ui/src/components/settings/Section.tsx`
- `rulez-ui/src/components/settings/FilePathInput.tsx`
- `rulez-ui/src/components/diff/ConfigDiff.tsx`
- `rulez-ui/src/components/diff/DiffView.tsx`
- `rulez-ui/src/components/diff/DiffStats.tsx`

### Modified Frontend Files
- `rulez-ui/src/lib/tauri.ts` (add log/settings/watch/diff functions)
- `rulez-ui/src/stores/configStore.ts` (add file watching logic)
- `rulez-ui/src/stores/uiStore.ts` (add settings persistence)
- `rulez-ui/src/types/index.ts` (add LogEntry, UserSettings, DiffHunk types)
- `rulez-ui/src/components/layout/RightPanel.tsx` (add logs tab)
- `rulez-ui/src/components/layout/Sidebar.tsx` (add compare button)
- `rulez-ui/package.json` (add dependencies)

### Dependencies Added

**Rust:**
- `tauri-plugin-fs = { version = "2.0", features = ["watch"] }`
- `tauri-plugin-store = "2.0"`
- `similar = "2.4"` (text diffing)

**JavaScript:**
- `react-window` (virtual scrolling)
- `react-diff-viewer` (diff UI)
- `@tauri-apps/plugin-fs`
- `@tauri-apps/plugin-store`

---

## Sources

**Tauri 2.0 File System (HIGH confidence):**
- [File System Plugin](https://v2.tauri.app/plugin/file-system/) — watch() and watchImmediate() API
- [Tauri Events](https://v2.tauri.app/develop/calling-rust/#events) — Event system for file watching
- [Tauri Store Plugin](https://v2.tauri.app/plugin/store/) — Settings persistence

**React Virtual Scrolling (HIGH confidence):**
- [react-window](https://react-window.vercel.app/) — Virtual scrolling library
- [Virtual Scrolling in React](https://medium.com/@swatikpl44/virtual-scrolling-in-react-6028f700da6b) — Best practices

**Zustand Patterns (HIGH confidence):**
- [Zustand Slices Pattern](https://zustand.docs.pmnd.rs/guides/slices-pattern) — Store composition
- [Multiple Stores Discussion](https://github.com/pmndrs/zustand/discussions/2496) — When to split stores

**React Error Boundaries (MEDIUM confidence):**
- [react-error-boundary](https://github.com/bvaughn/react-error-boundary) — Library for error handling
- [Error Boundaries Guide](https://refine.dev/blog/react-error-boundaries/) — Best practices (2026-01-15)

**Diff Libraries (MEDIUM confidence):**
- [react-diff-viewer](https://github.com/praneshr/react-diff-viewer) — GitHub-style diff component
- [similar crate](https://docs.rs/similar) — Rust text diffing

**Existing Codebase (HIGH confidence):**
- `rulez-ui/src/stores/configStore.ts`, `rulez-ui/src/lib/tauri.ts` — M1 scaffold patterns
- `rulez-ui/src-tauri/src/commands/config.rs` — Existing Tauri command structure
- `rulez-ui/CLAUDE.md` — Dual-mode architecture documentation

---

**Researched:** 2026-02-10
**Next Review:** After Phase 1 implementation (validate virtual scrolling performance with real log files)
