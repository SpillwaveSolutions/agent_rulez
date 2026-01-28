# RuleZ UI Implementation Plan

**Feature ID:** rulez-ui
**Status:** Ready for Implementation
**Created:** 2026-01-24
**Total Estimated:** 9.5 days (Phase 1 MVP)

---

## Executive Summary

RuleZ UI is a native desktop application for CCH configuration management. This plan outlines the implementation approach for Phase 1 MVP, which delivers core editing, validation, and debug simulation capabilities.

### Key Deliverables
1. **Visual YAML Editor** - Monaco-based with syntax highlighting and autocomplete
2. **Real-time Validation** - JSON Schema validation with inline error markers
3. **Multi-file Support** - Global and project configuration management
4. **Debug Simulator** - Test rules without running Claude Code
5. **Rule Tree View** - Visual representation of configuration
6. **Theme Support** - Dark/light modes with system preference detection

---

## Architecture Overview

```
┌─────────────────────────────────────────────────────────────────┐
│                         RuleZ UI                                 │
├─────────────────────────────────────────────────────────────────┤
│                                                                  │
│  ┌─────────────────────────────────────────────────────────────┐│
│  │                    React Frontend (Bun)                      ││
│  │  ┌───────────┐ ┌───────────┐ ┌───────────┐ ┌─────────────┐  ││
│  │  │  Monaco   │ │  Zustand  │ │ TanStack  │ │  Tailwind   │  ││
│  │  │  Editor   │ │  Stores   │ │   Query   │ │    CSS 4    │  ││
│  │  └───────────┘ └───────────┘ └───────────┘ └─────────────┘  ││
│  └─────────────────────────────────────────────────────────────┘│
│                              │                                   │
│                         Tauri IPC                                │
│                              │                                   │
│  ┌─────────────────────────────────────────────────────────────┐│
│  │                    Rust Backend (Tauri 2.0)                  ││
│  │  ┌───────────┐ ┌───────────┐ ┌───────────┐ ┌─────────────┐  ││
│  │  │  Config   │ │   Debug   │ │ Validate  │ │    File     │  ││
│  │  │ Commands  │ │ Commands  │ │ Commands  │ │   Watcher   │  ││
│  │  └───────────┘ └───────────┘ └───────────┘ └─────────────┘  ││
│  └─────────────────────────────────────────────────────────────┘│
│                              │                                   │
│                      System Integration                          │
│                              │                                   │
│  ┌───────────────────┐ ┌───────────────────┐                    │
│  │   File System     │ │    CCH Binary     │                    │
│  │ ~/.claude/        │ │   cch debug       │                    │
│  │ .claude/          │ │   cch validate    │                    │
│  └───────────────────┘ └───────────────────┘                    │
└─────────────────────────────────────────────────────────────────┘
```

---

## Technology Decisions

### Frontend Stack

| Choice | Technology | Rationale |
|--------|------------|-----------|
| Runtime | **Bun** | Faster installs, native TS, unified tooling |
| Framework | **React 18** | Mature ecosystem, team familiarity |
| Language | **TypeScript** (strict) | Type safety, better DX |
| Styling | **Tailwind CSS 4** | Utility-first, small bundle |
| Editor | **Monaco + monaco-yaml** | VS Code quality, schema support |
| State | **Zustand** | Simple, no boilerplate |
| Async | **TanStack Query** | Caching, loading states |
| Linting | **Biome** | Faster than ESLint+Prettier |

### Backend Stack

| Choice | Technology | Rationale |
|--------|------------|-----------|
| Desktop | **Tauri 2.0** | Rust security, small binary |
| IPC | **Tauri Commands** | Type-safe, async |
| File I/O | **tokio + serde** | Async, JSON/YAML support |
| Watching | **notify** | Cross-platform file events |

---

## Milestone Dependency Graph

```
M1 (Project Setup)
 │
 ├──────────────────────────────────────────────────────┐
 │                                                      │
 ▼                                                      ▼
M2 (Monaco Editor) ───────────► M3 (Schema Validation)  M7 (Theming)
 │                                    │                  │
 │                                    ▼                  │
 │                              M4 (File Ops)            │
 │                                    │                  │
 │                                    ▼                  │
 ├──────────────────────────────► M5 (Tree View) ◄──────┤
 │                                    │                  │
 │                                    ▼                  │
 └──────────────────────────────► M6 (Simulator) ◄──────┘
                                      │
                                      ▼
                                 M8 (Tests)
```

---

## Milestone 1: Project Setup (1 day)

### Objective
Create the foundational project structure with Tauri 2.0, React, and Bun.

### Tasks

| ID | Task | Hours |
|----|------|-------|
| M1-T01 | Initialize Tauri + React project with Bun | 3 |
| M1-T02 | Configure dual-mode architecture | 2 |
| M1-T03 | Set up CI workflow | 3 |

### Implementation Steps

#### M1-T01: Project Initialization
```bash
# Create project directory
mkdir rulez_ui && cd rulez_ui

# Initialize with Bun
bun init

# Add Tauri
bunx create-tauri-app --yes

# Add React dependencies
bun add react react-dom
bun add -d typescript @types/react @types/react-dom

# Add Tailwind CSS 4
bun add -d tailwindcss @tailwindcss/vite

# Add Biome
bun add -d @biomejs/biome
```

#### M1-T02: Dual-Mode Architecture
Create `src/lib/tauri.ts`:
```typescript
declare global {
  interface Window {
    __TAURI__?: {
      invoke: <T>(cmd: string, args?: Record<string, unknown>) => Promise<T>;
    };
  }
}

export const isTauri = (): boolean => {
  return typeof window !== 'undefined' && !!window.__TAURI__;
};

export async function invoke<T>(cmd: string, args?: Record<string, unknown>): Promise<T> {
  if (isTauri() && window.__TAURI__) {
    return window.__TAURI__.invoke<T>(cmd, args);
  }
  throw new Error(`Tauri not available: ${cmd}`);
}
```

#### M1-T03: CI Workflow
Create `.github/workflows/rulez-ui.yml`:
```yaml
name: RuleZ UI CI
on:
  push:
    paths: ['rulez_ui/**']
  pull_request:
    paths: ['rulez_ui/**']

jobs:
  check:
    runs-on: ubuntu-latest
    defaults:
      run:
        working-directory: rulez_ui
    steps:
      - uses: actions/checkout@v4
      - uses: oven-sh/setup-bun@v1
      - run: bun install
      - run: bun run lint
      - run: bun run typecheck
      - run: bun run test
```

### Definition of Done
- [ ] `bun run dev` starts development server
- [ ] `bun run dev:tauri` opens Tauri window
- [ ] Directory structure matches spec
- [ ] CI workflow runs on push

---

## Milestone 2: Monaco Editor (1 day)

### Objective
Integrate Monaco Editor with YAML support and essential editing features.

### Tasks

| ID | Task | Hours |
|----|------|-------|
| M2-T01 | Integrate Monaco Editor | 3 |
| M2-T02 | Add editor features | 2 |
| M2-T03 | Create editor toolbar | 3 |

### Implementation Steps

#### M2-T01: Monaco Integration
```bash
bun add @monaco-editor/react monaco-editor
```

Create `src/components/editor/YamlEditor.tsx`:
```typescript
import Editor from '@monaco-editor/react';

interface YamlEditorProps {
  value: string;
  onChange: (value: string) => void;
  onSave?: () => void;
}

export function YamlEditor({ value, onChange, onSave }: YamlEditorProps) {
  return (
    <Editor
      height="100%"
      language="yaml"
      value={value}
      onChange={(v) => onChange(v ?? '')}
      options={{
        minimap: { enabled: false },
        fontSize: 14,
        lineNumbers: 'on',
        folding: true,
        wordWrap: 'on',
      }}
      onMount={(editor) => {
        // Cmd/Ctrl+S to save
        editor.addCommand(monaco.KeyMod.CtrlCmd | monaco.KeyCode.KeyS, () => {
          onSave?.();
        });
      }}
    />
  );
}
```

### Definition of Done
- [ ] Editor displays YAML with syntax highlighting
- [ ] Cmd/Ctrl+S triggers save callback
- [ ] Undo/redo works
- [ ] Code folding works

---

## Milestone 3: Schema Validation (2 days)

### Objective
Enable real-time JSON Schema validation with inline error markers.

### Tasks

| ID | Task | Hours |
|----|------|-------|
| M3-T01 | Create JSON Schema | 4 |
| M3-T02 | Integrate monaco-yaml | 4 |
| M3-T03 | Implement autocomplete | 4 |
| M3-T04 | Create ValidationPanel | 4 |

### Implementation Steps

#### M3-T01: JSON Schema
Create `public/schema/hooks-schema.json` covering:
- `version` (required, pattern X.Y)
- `settings` (log_level, fail_open, etc.)
- `rules` array with matchers and actions
- All CCH field types with descriptions

#### M3-T02: monaco-yaml Integration
```bash
bun add monaco-yaml
```

```typescript
import { configureMonacoYaml } from 'monaco-yaml';

// In editor setup
configureMonacoYaml(monaco, {
  enableSchemaRequest: false,
  schemas: [{
    uri: 'inmemory://hooks-schema.json',
    fileMatch: ['*'],
    schema: hooksSchema,
  }],
});
```

#### M3-T04: ValidationPanel
Create `src/components/editor/ValidationPanel.tsx`:
- List errors with line numbers
- Click to jump to line
- Show error count badge

### Definition of Done
- [ ] Invalid YAML shows red squiggles
- [ ] Schema violations show errors
- [ ] Autocomplete suggests field names
- [ ] ValidationPanel shows all errors

---

## Milestone 4: File Operations (1 day)

### Objective
Implement file reading/writing with multi-file support.

### Tasks

| ID | Task | Hours |
|----|------|-------|
| M4-T01 | Implement Tauri file commands | 3 |
| M4-T02 | Create config store | 2 |
| M4-T03 | Create FileSidebar | 2 |
| M4-T04 | Create FileTabBar | 1 |

### Implementation Steps

#### M4-T01: Rust Commands
Create `src-tauri/src/commands/config.rs`:
```rust
#[tauri::command]
pub async fn list_config_files(project_dir: Option<String>) -> Result<Vec<ConfigFile>, String> {
    let mut files = Vec::new();
    
    // Global config
    if let Some(home) = dirs::home_dir() {
        let path = home.join(".claude").join("hooks.yaml");
        files.push(ConfigFile {
            path: path.to_string_lossy().to_string(),
            exists: path.exists(),
        });
    }
    
    // Project config
    if let Some(dir) = project_dir {
        let path = PathBuf::from(dir).join(".claude").join("hooks.yaml");
        files.push(ConfigFile {
            path: path.to_string_lossy().to_string(),
            exists: path.exists(),
        });
    }
    
    Ok(files)
}

#[tauri::command]
pub async fn read_config(path: String) -> Result<String, String> {
    std::fs::read_to_string(&path)
        .map_err(|e| format!("Failed to read: {}", e))
}

#[tauri::command]
pub async fn write_config(path: String, content: String) -> Result<(), String> {
    if let Some(parent) = Path::new(&path).parent() {
        std::fs::create_dir_all(parent)
            .map_err(|e| format!("Failed to create dir: {}", e))?;
    }
    std::fs::write(&path, content)
        .map_err(|e| format!("Failed to write: {}", e))
}
```

#### M4-T02: Zustand Store
Create `src/stores/configStore.ts`:
```typescript
import { create } from 'zustand';

interface ConfigStore {
  activeFile: string | null;
  openFiles: Map<string, { content: string; modified: boolean }>;
  setActiveFile: (path: string) => void;
  updateContent: (path: string, content: string) => void;
  markSaved: (path: string) => void;
}

export const useConfigStore = create<ConfigStore>((set) => ({
  activeFile: null,
  openFiles: new Map(),
  setActiveFile: (path) => set({ activeFile: path }),
  updateContent: (path, content) => set((state) => {
    const files = new Map(state.openFiles);
    files.set(path, { content, modified: true });
    return { openFiles: files };
  }),
  markSaved: (path) => set((state) => {
    const files = new Map(state.openFiles);
    const file = files.get(path);
    if (file) {
      files.set(path, { ...file, modified: false });
    }
    return { openFiles: files };
  }),
}));
```

### Definition of Done
- [ ] File sidebar shows global and project configs
- [ ] Tab bar supports multiple open files
- [ ] Modified indicator shows on unsaved files
- [ ] Cmd/Ctrl+S saves to file system

---

## Milestone 5: Rule Tree View (1 day)

### Objective
Create a visual tree representation of the configuration.

### Tasks

| ID | Task | Hours |
|----|------|-------|
| M5-T01 | Create RuleTreeView component | 3 |
| M5-T02 | Implement rule cards | 2 |
| M5-T03 | Implement navigation | 3 |

### Implementation Steps

#### M5-T01: Tree View
Parse YAML and render as collapsible tree:
- Settings section
- Rules section with individual rule cards

#### M5-T02: Rule Cards
```typescript
interface RuleCardProps {
  rule: Rule;
  onNavigate: (line: number) => void;
  onToggle: (enabled: boolean) => void;
}

function RuleCard({ rule, onNavigate, onToggle }: RuleCardProps) {
  return (
    <div className="border rounded p-2">
      <div className="flex justify-between">
        <span className="font-bold">{rule.name}</span>
        <Switch checked={rule.enabled} onChange={onToggle} />
      </div>
      <div className="text-sm text-gray-500">
        Tools: {rule.matchers.tools?.join(', ')}
      </div>
      <Badge>{getActionType(rule)}</Badge>
    </div>
  );
}
```

### Definition of Done
- [ ] Tree shows settings and rules
- [ ] Rule cards show name, tools, action type
- [ ] Click rule jumps to editor line
- [ ] Toggle updates YAML

---

## Milestone 6: Debug Simulator (2 days)

### Objective
Enable testing rules by simulating events through CCH binary.

### Tasks

| ID | Task | Hours |
|----|------|-------|
| M6-T01 | Implement CCH debug command | 4 |
| M6-T02 | Create EventForm | 3 |
| M6-T03 | Create ResultView | 2 |
| M6-T04 | Create EvaluationTrace | 3 |
| M6-T05 | Integrate simulator | 4 |

### Implementation Steps

#### M6-T01: Rust Debug Command
```rust
#[tauri::command]
pub async fn run_debug(
    hook_event_name: String,
    tool: Option<String>,
    command: Option<String>,
    path: Option<String>,
) -> Result<DebugResult, String> {
    let mut cmd = Command::new("cch");
    cmd.arg("debug").arg(&hook_event_name);
    
    if let Some(t) = tool {
        cmd.arg("--tool").arg(t);
    }
    if let Some(c) = command {
        cmd.arg("--command").arg(c);
    }
    if let Some(p) = path {
        cmd.arg("--path").arg(p);
    }
    cmd.arg("--json");
    
    let output = cmd.output()
        .map_err(|e| format!("Failed to run cch: {}", e))?;
    
    serde_json::from_slice(&output.stdout)
        .map_err(|e| format!("Failed to parse: {}", e))
}
```

#### M6-T05: Simulator Container
```typescript
function DebugSimulator() {
  const [params, setParams] = useState<SimulatorParams>({
    eventType: 'PreToolUse',
    tool: '',
    command: '',
    path: '',
  });
  const [result, setResult] = useState<DebugResult | null>(null);
  const [loading, setLoading] = useState(false);

  const runSimulation = async () => {
    setLoading(true);
    try {
      if (isTauri()) {
        const res = await invoke<DebugResult>('run_debug', params);
        setResult(res);
      } else {
        setResult(getMockResult(params));
      }
    } finally {
      setLoading(false);
    }
  };

  return (
    <div>
      <EventForm params={params} onChange={setParams} />
      <Button onClick={runSimulation} loading={loading}>Simulate</Button>
      {result && <ResultView result={result} />}
      {result && <EvaluationTrace evaluations={result.evaluations} />}
    </div>
  );
}
```

### Definition of Done
- [ ] Event form with all 7 event types
- [ ] Simulate button calls CCH debug
- [ ] Results show Allow/Block/Inject
- [ ] Evaluation trace shows per-rule details

---

## Milestone 7: Theming (0.5 day)

### Objective
Implement dark/light theme support with system preference detection.

### Tasks

| ID | Task | Hours |
|----|------|-------|
| M7-T01 | Implement theme system | 1 |
| M7-T02 | Create theme toggle | 1 |
| M7-T03 | Configure Monaco themes | 1 |
| M7-T04 | Style Tailwind for themes | 1 |

### Implementation Steps

#### M7-T01: Theme Store
```typescript
import { create } from 'zustand';
import { persist } from 'zustand/middleware';

type Theme = 'light' | 'dark' | 'system';

interface UIStore {
  theme: Theme;
  setTheme: (theme: Theme) => void;
  resolvedTheme: () => 'light' | 'dark';
}

export const useUIStore = create<UIStore>()(
  persist(
    (set, get) => ({
      theme: 'system',
      setTheme: (theme) => set({ theme }),
      resolvedTheme: () => {
        const { theme } = get();
        if (theme === 'system') {
          return window.matchMedia('(prefers-color-scheme: dark)').matches
            ? 'dark'
            : 'light';
        }
        return theme;
      },
    }),
    { name: 'rulez-ui-theme' }
  )
);
```

### Definition of Done
- [ ] System preference detected on launch
- [ ] Manual toggle works
- [ ] Monaco theme matches app theme
- [ ] Preference persists across sessions

---

## Milestone 8: Playwright Tests (1 day)

### Objective
Create E2E test suite for critical user flows.

### Tasks

| ID | Task | Hours |
|----|------|-------|
| M8-T01 | Set up Playwright | 2 |
| M8-T02 | Write editor tests | 2 |
| M8-T03 | Write simulator tests | 2 |
| M8-T04 | Write file operation tests | 1 |
| M8-T05 | Configure CI | 1 |

### Test Coverage

| Area | Tests |
|------|-------|
| Editor | Load, validation errors, autocomplete, save |
| Simulator | Event form, simulation, results |
| Files | Sidebar, tabs, save prompt |
| Theme | Toggle, persistence |

### Definition of Done
- [ ] All E2E tests pass locally
- [ ] Tests run in CI
- [ ] Test reports generated
- [ ] Coverage > 80% for critical paths

---

## Risk Assessment

| Risk | Likelihood | Impact | Mitigation |
|------|------------|--------|------------|
| Monaco bundle size too large | Medium | Medium | Use worker, code split |
| Tauri IPC latency | Low | Medium | Async commands, caching |
| CCH binary not in PATH | High | High | Clear error message, help |
| Cross-platform path issues | Medium | Medium | Use path normalization |

---

## Success Criteria

| Criterion | Target | Measurement |
|-----------|--------|-------------|
| App launches | < 2s | Lighthouse |
| Validation response | < 200ms | Performance API |
| E2E test pass rate | 100% | CI reports |
| Memory usage | < 150MB | Tauri DevTools |

---

## Post-MVP Roadmap

### Phase 2: Log Viewer (5-7 days)
- Real-time log streaming via file watcher
- Virtual scrolling for large files
- Filter by level, rule, timestamp

### Phase 3: Advanced Features (7-10 days)
- Rule templates library
- Import/export configurations
- Regex pattern tester
- Context file preview

### Phase 4: Distribution (3-5 days)
- Platform installers (DMG, MSI, DEB)
- Auto-update with Tauri updater
- Onboarding tutorial
