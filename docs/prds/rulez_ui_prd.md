# RuleZ UI

## Product Requirements Document

**Version:** 1.0  
**Last Updated:** January 23, 2026  
**Status:** Draft  
**Related Plan:** [docs/plans/rulez_ui_plan.md](../plans/rulez_ui_plan.md)

---

## Table of Contents

1. [Executive Summary](#1-executive-summary)
2. [User Stories](#2-user-stories)
3. [Functional Requirements](#3-functional-requirements)
4. [Non-Functional Requirements](#4-non-functional-requirements)
5. [Technical Architecture](#5-technical-architecture)
6. [User Interface](#6-user-interface)
7. [Component Specifications](#7-component-specifications)
8. [Tauri IPC Commands](#8-tauri-ipc-commands)
9. [Configuration Schema](#9-configuration-schema)
10. [Implementation Plan](#10-implementation-plan)
11. [Testing Strategy](#11-testing-strategy)
12. [Distribution](#12-distribution)
13. [Success Metrics](#13-success-metrics)
14. [Future Enhancements](#14-future-enhancements)
15. [Open Questions](#15-open-questions)
16. [Appendix](#16-appendix)

---

## 1. Executive Summary

### 1.1 Product Name

**RuleZ UI** (Directory: `rulez_ui`)

### 1.2 Problem Statement

Claude Context Hooks (CCH) provides powerful YAML-based configuration for controlling Claude Code behavior. However, the current workflow has several pain points:

- **No visual feedback:** Users must manually validate YAML syntax and schema compliance
- **Debugging is CLI-only:** Testing rules requires command-line invocations and reading JSON output
- **No real-time validation:** Errors are only discovered when CCH runs
- **Log analysis is tedious:** JSON Lines logs require manual parsing and filtering
- **Configuration scattered:** Global and project configs require manual navigation

### 1.3 Solution

A native desktop application built with Tauri 2.0 + React that provides:

- **Visual YAML Editor** with Monaco Editor, syntax highlighting, and schema validation
- **Real-time Validation** with inline error markers (red squiggly underlines)
- **Debug Simulator** to test events against rules without running Claude Code
- **Multi-file Support** for both global (~/.claude/hooks.yaml) and project-level configs
- **Rule Tree View** for visual representation of configured rules
- **Dark/Light Themes** with system preference detection

### 1.4 Target Users

| User Type | Use Case |
|-----------|----------|
| CCH users | Visual editing and debugging of hooks.yaml configurations |
| Developers | Testing rule matching before deployment |
| Team leads | Reviewing and auditing hook configurations |
| New users | Learning CCH through interactive exploration |

### 1.5 Key Benefits

| Benefit | Description |
|---------|-------------|
| **Visual editing** | Monaco editor with full YAML support and autocomplete |
| **Instant feedback** | Schema validation with inline error markers |
| **Safe testing** | Debug simulator tests rules without affecting Claude Code |
| **Faster iteration** | Hot reload in development, instant validation |
| **Cross-platform** | Native desktop app for macOS, Windows, Linux |

### 1.6 Technology Stack

| Layer | Technology |
|-------|------------|
| **Runtime** | Bun (all TypeScript/React operations) |
| **Frontend** | React 18 + TypeScript + Tailwind CSS 4 |
| **Editor** | Monaco Editor + monaco-yaml |
| **Desktop** | Tauri 2.0 (Rust backend) |
| **State** | Zustand + TanStack Query |
| **Testing** | Bun test (unit) + Playwright (E2E) |

---

## 2. User Stories

### 2.1 Configuration Editing

> As a CCH user, I want to edit my hooks.yaml file with syntax highlighting and autocomplete, so I can write configurations faster and with fewer errors.

**Acceptance Criteria:**
- Monaco editor loads with YAML syntax highlighting
- Autocomplete suggests valid rule fields (name, matchers, actions, etc.)
- Tab completion works for nested structures
- Undo/redo with Cmd/Ctrl+Z

### 2.2 Real-time Validation

> As a CCH user, I want to see validation errors inline as I type, so I can fix problems immediately without running external commands.

**Acceptance Criteria:**
- Invalid YAML syntax shows red squiggly underlines
- Schema violations show with descriptive error messages
- Errors panel lists all issues with line numbers
- Click error to jump to line in editor

### 2.3 Multi-file Management

> As a developer, I want to view and edit both my global and project-level hooks.yaml files, so I can manage configurations in one place.

**Acceptance Criteria:**
- File sidebar shows global (~/.claude/hooks.yaml) and project (.claude/hooks.yaml) configs
- Tab bar allows multiple files open simultaneously
- Modified files show indicator (asterisk or dot)
- Save with Cmd/Ctrl+S

### 2.4 Debug Simulation

> As a CCH user, I want to simulate events and see which rules match without running Claude Code, so I can test my configuration safely.

**Acceptance Criteria:**
- Select event type (PreToolUse, PostToolUse, etc.)
- Enter tool name, command, and path
- Click "Simulate" to run debug
- See matched rules with evaluation trace
- See final outcome (Allow/Block/Inject)

### 2.5 Rule Visualization

> As a team lead, I want to see a visual tree of all configured rules, so I can quickly audit what hooks are active.

**Acceptance Criteria:**
- Tree view shows rules grouped by category
- Each rule shows: name, tools, action type, enabled status
- Click rule to jump to its location in editor
- Toggle switch to enable/disable rules

### 2.6 Theme Support

> As a user, I want the application to respect my system's dark/light mode preference, so it's comfortable to use in any lighting condition.

**Acceptance Criteria:**
- Detects system preference on launch
- Manual toggle in header
- Monaco editor theme changes with app theme
- Preference persisted across sessions

---

## 3. Functional Requirements

### 3.1 YAML Editor

| Requirement | Description | Priority |
|-------------|-------------|----------|
| FR-ED-01 | Monaco Editor with YAML language mode | P0 |
| FR-ED-02 | JSON Schema validation via monaco-yaml | P0 |
| FR-ED-03 | Inline error markers (red squiggles) | P0 |
| FR-ED-04 | Autocomplete for CCH schema fields | P0 |
| FR-ED-05 | Syntax highlighting for YAML | P0 |
| FR-ED-06 | Line numbers with error indicators | P1 |
| FR-ED-07 | Code folding for rules/sections | P1 |
| FR-ED-08 | Find/replace (Cmd+F, Cmd+H) | P1 |
| FR-ED-09 | Multi-cursor editing | P2 |

### 3.2 File Operations

| Requirement | Description | Priority |
|-------------|-------------|----------|
| FR-FO-01 | Read hooks.yaml from filesystem | P0 |
| FR-FO-02 | Write hooks.yaml to filesystem | P0 |
| FR-FO-03 | List global and project config files | P0 |
| FR-FO-04 | Create new hooks.yaml if not exists | P1 |
| FR-FO-05 | Watch files for external changes | P1 |
| FR-FO-06 | Prompt on unsaved changes before close | P1 |

### 3.3 Debug Simulator

| Requirement | Description | Priority |
|-------------|-------------|----------|
| FR-DS-01 | Event type selection (dropdown) | P0 |
| FR-DS-02 | Tool name input | P0 |
| FR-DS-03 | Command input (for Bash) | P0 |
| FR-DS-04 | Path input (for file operations) | P0 |
| FR-DS-05 | Execute simulation via CCH binary | P0 |
| FR-DS-06 | Display outcome (Allow/Block/Inject) | P0 |
| FR-DS-07 | Display matched rules list | P0 |
| FR-DS-08 | Display evaluation trace | P0 |
| FR-DS-09 | Display execution time | P1 |

### 3.4 Rule Tree View

| Requirement | Description | Priority |
|-------------|-------------|----------|
| FR-RT-01 | Parse YAML and display as tree | P0 |
| FR-RT-02 | Show settings section | P0 |
| FR-RT-03 | Show rules with name and action type | P0 |
| FR-RT-04 | Click to navigate to line in editor | P0 |
| FR-RT-05 | Toggle switch to enable/disable rules | P1 |
| FR-RT-06 | Collapsible sections | P1 |

### 3.5 Validation Panel

| Requirement | Description | Priority |
|-------------|-------------|----------|
| FR-VP-01 | List all errors with line numbers | P0 |
| FR-VP-02 | List all warnings | P0 |
| FR-VP-03 | Click error to jump to line | P0 |
| FR-VP-04 | Show error count in status bar | P0 |
| FR-VP-05 | Auto-update on editor changes | P0 |

### 3.6 Theming

| Requirement | Description | Priority |
|-------------|-------------|----------|
| FR-TH-01 | Detect system preference | P0 |
| FR-TH-02 | Manual toggle (sun/moon icon) | P0 |
| FR-TH-03 | Dark theme for app shell | P0 |
| FR-TH-04 | Light theme for app shell | P0 |
| FR-TH-05 | Monaco theme matches app theme | P0 |
| FR-TH-06 | Persist preference | P1 |

---

## 4. Non-Functional Requirements

### 4.1 Performance

| Metric | Target |
|--------|--------|
| App launch (cold start) | < 2 seconds |
| File load (10KB YAML) | < 100ms |
| Validation response | < 200ms |
| Debug simulation | < 500ms (excl. CCH) |
| Editor input latency | < 16ms (60fps) |
| Memory usage (idle) | < 150MB |

### 4.2 Reliability

- Graceful handling of missing config files
- Graceful handling of CCH binary not found
- Auto-save drafts to prevent data loss
- Recover from malformed YAML without crash
- Valid UI state even on internal errors

### 4.3 Compatibility

| Platform | Architecture | Support |
|----------|--------------|---------|
| macOS | Intel (x86_64) | Full |
| macOS | Apple Silicon (aarch64) | Full |
| Linux | x86_64 | Full |
| Linux | ARM64 | Full |
| Windows | x86_64 | Full |

**Runtime Dependencies:**
- CCH binary in PATH (for debug simulator)
- No other runtime dependencies (Tauri bundles everything)

### 4.4 Security

- No network access (fully offline)
- Files accessed only via Tauri's secure IPC
- No arbitrary code execution
- Respects file system permissions
- No telemetry or analytics

### 4.5 Usability

- Keyboard shortcuts for common operations
- Accessible UI (ARIA labels, keyboard navigation)
- Clear error messages with actionable guidance
- Responsive layout (minimum 800x600)
- Native window controls (minimize, maximize, close)

---

## 5. Technical Architecture

### 5.1 System Overview

```
+------------------------------------------------------------------+
|                        RuleZ UI Architecture                      |
+------------------------------------------------------------------+
|                                                                    |
|   +------------------------+    +-----------------------------+   |
|   |    Web Mode (Test)     |    |    Desktop Mode (Primary)   |   |
|   +------------------------+    +-----------------------------+   |
|   |                        |    |                             |   |
|   |  Browser (localhost)   |    |  Tauri WebView Window       |   |
|   |         |              |    |         |                   |   |
|   |         v              |    |         v                   |   |
|   |  React App (Bun HMR)   |    |  React App (bundled)        |   |
|   |         |              |    |         |                   |   |
|   |         v              |    |         v                   |   |
|   |  Web Fallbacks:        |    |  Tauri IPC:                 |   |
|   |  - localStorage        |    |  - invoke('read_config')    |   |
|   |  - Mock data           |    |  - invoke('run_debug')      |   |
|   |  - Sample configs      |    |  - File watchers            |   |
|   |                        |    |         |                   |   |
|   +------------------------+    |         v                   |   |
|                                 |  Rust Backend:              |   |
|   Used ONLY for:                |  - File I/O                 |   |
|   - Playwright E2E testing      |  - CCH binary execution     |   |
|   - Automated CI testing        |  - Real-time file watching  |   |
|                                 |                             |   |
|                                 +-----------------------------+   |
|                                          |                        |
|                                 PRIMARY DELIVERABLE               |
+------------------------------------------------------------------+
```

### 5.2 Component Architecture

```
+------------------------------------------------------------------+
|                        React Component Tree                       |
+------------------------------------------------------------------+
|                                                                    |
|  App                                                               |
|   |                                                                |
|   +-- AppShell                                                     |
|        |                                                           |
|        +-- Header                                                  |
|        |    +-- Logo                                               |
|        |    +-- ThemeToggle                                        |
|        |    +-- WindowControls (Tauri)                             |
|        |                                                           |
|        +-- Sidebar                                                 |
|        |    +-- FileSidebar                                        |
|        |         +-- FileTree (Global)                             |
|        |         +-- FileTree (Project)                            |
|        |                                                           |
|        +-- MainContent                                             |
|        |    +-- FileTabBar                                         |
|        |    |    +-- FileTab (for each open file)                  |
|        |    |                                                      |
|        |    +-- EditorPanel                                        |
|        |         +-- EditorToolbar                                 |
|        |         +-- YamlEditor (Monaco)                           |
|        |         +-- ValidationPanel                               |
|        |                                                           |
|        +-- RightPanel                                              |
|        |    +-- Tabs                                               |
|        |         +-- DebugSimulator                                |
|        |         |    +-- EventForm                                |
|        |         |    +-- ResultView                               |
|        |         |    +-- EvaluationTrace                          |
|        |         |                                                 |
|        |         +-- RuleTreeView                                  |
|        |                                                           |
|        +-- StatusBar                                               |
|             +-- CursorPosition                                     |
|             +-- FileType                                           |
|             +-- ErrorCount                                         |
|             +-- ConnectionStatus                                   |
|                                                                    |
+------------------------------------------------------------------+
```

### 5.3 Data Flow

```
+------------------------------------------------------------------+
|                         Data Flow                                 |
+------------------------------------------------------------------+
|                                                                    |
|  User Action          React Component        Tauri/Backend         |
|  ───────────          ───────────────        ────────────          |
|                                                                    |
|  Edit YAML    ──────► YamlEditor     ──────► (real-time)           |
|                       │                      │                     |
|                       │ onChange             │                     |
|                       v                      v                     |
|               Monaco validates ◄──── JSON Schema                   |
|                       │                                            |
|                       │ onSave (Cmd+S)                             |
|                       v                                            |
|               configStore.save() ──────► write_config()            |
|                                          │                         |
|                                          v                         |
|                                   ~/.claude/hooks.yaml             |
|                                                                    |
|  ─────────────────────────────────────────────────────────────     |
|                                                                    |
|  Run Debug    ──────► DebugSimulator ──────► run_debug()           |
|                       │                      │                     |
|                       │                      v                     |
|                       │              cch debug PreToolUse          |
|                       │              --tool Bash                   |
|                       │              --command "git push"          |
|                       │              --json                        |
|                       │                      │                     |
|                       v                      v                     |
|               ResultView     ◄────── JSON output parsed            |
|                                                                    |
+------------------------------------------------------------------+
```

### 5.4 State Management

```
+------------------------------------------------------------------+
|                       Zustand Stores                              |
+------------------------------------------------------------------+
|                                                                    |
|  configStore                                                       |
|  ├── globalConfig: ConfigFile | null                               |
|  ├── projectConfig: ConfigFile | null                              |
|  ├── activeFile: string | null                                     |
|  ├── openFiles: Map<string, FileState>                             |
|  ├── actions:                                                      |
|  │   ├── loadConfig(path: string)                                  |
|  │   ├── saveConfig(path: string, content: string)                 |
|  │   ├── setActiveFile(path: string)                               |
|  │   └── updateContent(path: string, content: string)              |
|  └── selectors:                                                    |
|      ├── getActiveContent()                                        |
|      └── hasUnsavedChanges()                                       |
|                                                                    |
|  editorStore                                                       |
|  ├── cursorPosition: { line: number, column: number }              |
|  ├── selection: Range | null                                       |
|  ├── errors: ValidationError[]                                     |
|  ├── warnings: ValidationWarning[]                                 |
|  └── actions:                                                      |
|      ├── setCursorPosition(line, column)                           |
|      ├── setErrors(errors)                                         |
|      └── clearValidation()                                         |
|                                                                    |
|  uiStore                                                           |
|  ├── theme: 'light' | 'dark' | 'system'                            |
|  ├── sidebarOpen: boolean                                          |
|  ├── rightPanelTab: 'simulator' | 'tree'                           |
|  └── actions:                                                      |
|      ├── setTheme(theme)                                           |
|      ├── toggleSidebar()                                           |
|      └── setRightPanelTab(tab)                                     |
|                                                                    |
+------------------------------------------------------------------+
```

---

## 6. User Interface

### 6.1 Main Application Layout

```
+------------------------------------------------------------------+
|  [logo] RuleZ UI                    [?] [sun/moon] [_] [x]       |
+------------------------------------------------------------------+
|        |                                              |           |
| FILES  |  [hooks.yaml] [project/hooks.yaml]  [+]     | SIMULATOR |
|        +---------------------------------------------+           |
| v Global                                             |           |
|   hooks.yaml *                                       | Event Type|
|                                                      | [PreTool~]|
| v Project                                            |           |
|   .claude/                                           | Tool      |
|     hooks.yaml                                       | [Bash    ]|
|                                                      |           |
|        +---------------------------------------------+ Command   |
|        |  1  version: "1.0"                          | [git push]|
|        |  2                                          |           |
|        |  3  settings:                               | Path      |
|        |  4    log_level: "info"                     | [        ]|
|        |  5    fail_open: true                       |           |
|        |  6                                          | [Simulate]|
|        |  7  rules:                                  +-----------+
|        |  8    - name: block-force-push              |           |
|        |  9      description: "Block force push"     | RESULTS   |
|        | 10      matchers:                           |           |
|        | 11        tools: ["Bash"]                   | Outcome:  |
|        | 12        command_match: "git push.*-f"     | [BLOCKED] |
|        | 13      actions:                            |           |
|        | 14        block: true                       | Matched:  |
|        | 15~~~~~~~~~~~~~~~~~~~~~~~~~~~~             | > block-  |
|        |         ^                                   |   force-  |
|        |         |                                   |   push    |
|        |    [red squiggle = validation error]        |           |
|        |                                             | Eval Time:|
|        |                                             | 2.3ms     |
+--------+---------------------------------------------+-----------+
| Ln 14, Col 8 | YAML | UTF-8 | 2 errors, 1 warning   | Connected |
+------------------------------------------------------------------+
```

### 6.2 Debug Simulator Panel

```
+------------------------------------------------------------------+
| DEBUG SIMULATOR                                                   |
+------------------------------------------------------------------+
|                                                                   |
| Event Configuration                                               |
| +---------------------------------------------------------------+ |
| | Event Type    | Tool          | Command                       | |
| | [PreToolUse v]| [Bash      v] | [git push --force origin main]| |
| +---------------------------------------------------------------+ |
| | Path (optional)                                                | |
| | [                                                            ] | |
| +---------------------------------------------------------------+ |
|                                                                   |
| [ Run Simulation ]                                                |
|                                                                   |
+------------------------------------------------------------------+
| RESULTS                                                           |
+------------------------------------------------------------------+
|                                                                   |
| Outcome: BLOCKED                                        [2.3ms]   |
| Reason: "Force push to main/master is prohibited"                 |
|                                                                   |
| +---------------------------------------------------------------+ |
| | EVALUATION TRACE                                              | |
| +---------------------------------------------------------------+ |
| | #  Rule Name           Match?  Time   Details                 | |
| | -- -------------------- ------- ------ ----------------------- | |
| | 1  block-force-push    YES     0.8ms  command_match matched   | |
| |    > Pattern: git push.*(--force|-f).*(main|master)           | |
| |    > Input: git push --force origin main                      | |
| |    > Action: BLOCK                                            | |
| | -- -------------------- ------- ------ ----------------------- | |
| | 2  inject-python       NO      0.1ms  tool mismatch           | |
| |    > Expected: Write, Edit                                    | |
| |    > Got: Bash                                                | |
| +---------------------------------------------------------------+ |
|                                                                   |
+------------------------------------------------------------------+
```

### 6.3 Rule Tree View Panel

```
+------------------------------------------+
| RULES                              [code]|
+------------------------------------------+
| v Settings                               |
|   log_level: info                        |
|   fail_open: true                        |
|   max_context_size: 1MB                  |
|                                          |
| v Rules (3)                              |
|   +--------------------------------------+
|   | [!] block-force-push          [on]  |
|   |     Tools: Bash                      |
|   |     Match: git push.*--force         |
|   |     Action: BLOCK                    |
|   +--------------------------------------+
|   | [i] inject-python-context     [on]  |
|   |     Tools: Write, Edit               |
|   |     Extensions: .py                  |
|   |     Action: INJECT                   |
|   +--------------------------------------+
|   | [>] run-secret-scanner        [off] |
|   |     Tools: Bash                      |
|   |     Match: git commit                |
|   |     Action: RUN SCRIPT               |
|   +--------------------------------------+
+------------------------------------------+
```

### 6.4 Validation Panel

```
+------------------------------------------------------------------+
| PROBLEMS                                                    [x]   |
+------------------------------------------------------------------+
| Errors (2)                                                        |
| +---------------------------------------------------------------+ |
| | [!] Line 14: Invalid value for 'log_level'                    | |
| |     Expected: "debug" | "info" | "warn" | "error"             | |
| |     Got: "verbose"                                            | |
| +---------------------------------------------------------------+ |
| | [!] Line 22: Missing required field 'name' in rule            | |
| +---------------------------------------------------------------+ |
|                                                                   |
| Warnings (1)                                                      |
| +---------------------------------------------------------------+ |
| | [i] Line 8: Rule 'block-force-push' has no description        | |
| +---------------------------------------------------------------+ |
+------------------------------------------------------------------+
```

---

## 7. Component Specifications

### 7.1 YamlEditor Component

**File:** `src/components/editor/YamlEditor.tsx`

**Purpose:** Monaco-based YAML editor with schema validation

**Props:**
```typescript
interface YamlEditorProps {
  value: string;
  onChange: (value: string) => void;
  onSave?: () => void;
  schema?: object;
  readOnly?: boolean;
  height?: string | number;
}
```

**Features:**
- Monaco Editor with YAML language support
- JSON Schema validation via monaco-yaml
- Inline error markers (red squiggles)
- Autocomplete for rule fields
- Custom dark/light themes
- Keyboard shortcuts (Cmd+S to save, Cmd+Z undo)
- Line numbers with error indicators

**Implementation:**
```typescript
import { configureMonacoYaml } from 'monaco-yaml';

useEffect(() => {
  if (monaco) {
    configureMonacoYaml(monaco, {
      enableSchemaRequest: false,
      schemas: [{
        uri: 'inmemory://hooks-schema.json',
        fileMatch: ['*'],
        schema: hooksSchema,
      }],
    });
  }
}, [monaco]);
```

### 7.2 DebugSimulator Component

**File:** `src/components/simulator/DebugSimulator.tsx`

**Purpose:** Simulate events and display rule evaluation results

**State:**
```typescript
interface SimulatorState {
  eventType: EventType;
  tool: string;
  command: string;
  path: string;
  isLoading: boolean;
  result: DebugResult | null;
  error: string | null;
}

type EventType = 
  | 'PreToolUse'
  | 'PostToolUse'
  | 'PermissionRequest'
  | 'UserPromptSubmit'
  | 'SessionStart'
  | 'SessionEnd'
  | 'PreCompact';
```

**Backend Integration:**
```typescript
async function runSimulation(params: SimulatorState) {
  if (isTauri()) {
    return invoke('run_debug', {
      eventType: params.eventType,
      tool: params.tool,
      command: params.command,
      path: params.path,
    });
  } else {
    return mockDebugResult(params);
  }
}
```

### 7.3 RuleTreeView Component

**File:** `src/components/editor/RuleTreeView.tsx`

**Purpose:** Visual tree representation of configured rules

**Props:**
```typescript
interface RuleTreeViewProps {
  config: HooksConfig;
  onRuleClick: (ruleName: string, lineNumber: number) => void;
  onRuleToggle: (ruleName: string, enabled: boolean) => void;
}
```

### 7.4 FileSidebar Component

**File:** `src/components/files/FileSidebar.tsx`

**Purpose:** Display and manage configuration files

**State:**
```typescript
interface FileState {
  globalConfig: ConfigFile | null;
  projectConfig: ConfigFile | null;
  activeFile: string | null;
}

interface ConfigFile {
  path: string;
  exists: boolean;
  modified: boolean;
  hasErrors: boolean;
  content?: string;
}
```

---

## 8. Tauri IPC Commands

### 8.1 Config Commands

**File:** `src-tauri/src/commands/config.rs`

```rust
/// List available config files (global and project)
#[command]
pub async fn list_config_files(project_dir: Option<String>) -> Result<Vec<ConfigFile>, String>

/// Read config file content
#[command]
pub async fn read_config(path: String) -> Result<String, String>

/// Write config file content
#[command]
pub async fn write_config(path: String, content: String) -> Result<(), String>
```

### 8.2 Debug Commands

**File:** `src-tauri/src/commands/debug.rs`

```rust
/// Run CCH debug command and parse output
#[command]
pub async fn run_debug(
    event_type: String,
    tool: Option<String>,
    command: Option<String>,
    path: Option<String>,
) -> Result<DebugResult, String>
```

**Response:**
```rust
pub struct DebugResult {
    pub outcome: String,        // "Allow", "Block", "Inject"
    pub reason: Option<String>,
    pub matched_rules: Vec<String>,
    pub evaluation_time_ms: f64,
    pub evaluations: Vec<RuleEvaluation>,
}
```

### 8.3 Validate Commands

**File:** `src-tauri/src/commands/validate.rs`

```rust
/// Validate config file using CCH
#[command]
pub async fn validate_config(path: String) -> Result<ValidationResult, String>
```

### 8.4 Frontend API Layer

**File:** `src/lib/tauri.ts`

```typescript
export const isTauri = (): boolean => {
  return typeof window !== 'undefined' && !!window.__TAURI__;
};

export async function listConfigFiles(projectDir?: string): Promise<ConfigFile[]>
export async function readConfig(path: string): Promise<string>
export async function writeConfig(path: string, content: string): Promise<void>
export async function runDebug(params: DebugParams): Promise<DebugResult>
```

---

## 9. Configuration Schema

### 9.1 JSON Schema for hooks.yaml

**File:** `public/schema/hooks-schema.json`

The schema provides validation for:
- `version` - Required, format X.Y
- `settings` - Global settings (log_level, fail_open, etc.)
- `rules` / `hooks` - Array of rule definitions
- Rule structure (name, matchers, actions, metadata)
- Matchers (tools, extensions, directories, command_match)
- Actions (block, inject, run, block_if_match)

See [Appendix A](#appendix-a-json-schema) for full schema.

---

## 10. Implementation Plan

### 10.1 Phase 1 Milestones

| Milestone | Description | Deliverables | Est. Days |
|-----------|-------------|--------------|-----------|
| **M1** | Project Setup | Tauri + React scaffold with Bun, dual-mode architecture | 1 |
| **M2** | Monaco Editor | YAML editor with syntax highlighting, basic editing | 1 |
| **M3** | Schema Validation | JSON Schema integration, inline errors, autocomplete | 2 |
| **M4** | File Operations | Read/write hooks.yaml, global + project support | 1 |
| **M5** | Rule Tree View | Visual tree component, rule navigation | 1 |
| **M6** | Debug Simulator | Event form, CCH integration, results display | 2 |
| **M7** | Theming | Dark/light mode, system preference, Monaco themes | 0.5 |
| **M8** | Playwright Tests | E2E test suite, CI setup with Bun | 1 |

**Total Estimate:** 9.5 days

### 10.2 Milestone Dependencies

```
M1 (Setup with Bun)
  |
  +---> M2 (Monaco) ---> M3 (Schema)
  |                          |
  |                          v
  +---> M4 (Files) -------->+---> M5 (Tree View)
  |                          |
  |                          v
  +---> M7 (Theming) ------>+---> M6 (Simulator)
                                    |
                                    v
                               M8 (Tests)
```

### 10.3 Future Phases

**Phase 2: Log Viewer** (5-7 days)
- Real-time log streaming with file watcher
- Virtual scrolling for large log files
- Filter by log level, rule name, timestamp
- Full-text search and export

**Phase 3: Advanced Features** (7-10 days)
- Rule templates library
- Import/export configurations
- Diff view between configs
- Regex tester for command_match

**Phase 4: Distribution** (3-5 days)
- Application installers (DMG, MSI, DEB)
- Auto-update mechanism
- Onboarding tutorial

---

## 11. Testing Strategy

### 11.1 Test Categories

| Category | Tool | Coverage Target |
|----------|------|-----------------|
| Unit Tests | Bun test | 80%+ for utilities |
| Component Tests | Bun test + Testing Library | Key components |
| E2E Tests | Playwright | Critical user flows |
| Integration Tests | Playwright | Tauri IPC |

### 11.2 E2E Test Scenarios

**Editor Tests:**
- Load with default content
- Show validation error for invalid YAML
- Show autocomplete for rule fields
- Save with keyboard shortcut

**Simulator Tests:**
- Simulate PreToolUse event
- Display matched rules
- Show evaluation trace

### 11.3 Playwright Configuration

```typescript
export default defineConfig({
  testDir: './tests',
  webServer: {
    command: 'bun run dev',
    url: 'http://localhost:1420',
  },
  projects: [
    { name: 'chromium', use: devices['Desktop Chrome'] },
    { name: 'webkit', use: devices['Desktop Safari'] },
  ],
});
```

---

## 12. Distribution

### 12.1 Build Targets

| Platform | Format | Command |
|----------|--------|---------|
| macOS | .dmg, .app | `bun run build:tauri` |
| Windows | .msi, .exe | `bun run build:tauri` |
| Linux | .deb, .AppImage | `bun run build:tauri` |

### 12.2 Release Process

1. Version bump in `package.json` and `Cargo.toml`
2. Build for all platforms via CI
3. Generate release notes
4. Publish to GitHub Releases
5. Update download links

---

## 13. Success Metrics

### 13.1 Launch Metrics

| Metric | Target |
|--------|--------|
| App launch time | < 2 seconds |
| Crash-free sessions | > 99% |
| E2E test pass rate | 100% |

### 13.2 User Satisfaction

| Metric | Target |
|--------|--------|
| Validation errors caught before save | 95%+ |
| Debug simulations match actual CCH | 100% |
| Users prefer UI over CLI for editing | 70%+ |

---

## 14. Future Enhancements

### 14.1 Phase 2+

- **Log Viewer:** Real-time streaming, filtering, search
- **Rule Templates:** Pre-built rules for common patterns
- **Diff View:** Compare global vs project configs
- **Regex Tester:** Live testing of command_match patterns
- **Context Preview:** Preview injected files
- **Settings Integration:** Edit Claude Code settings.json

### 14.2 Long-term Vision

- **Cloud Sync:** Share configurations across machines
- **Team Features:** Shared rule libraries
- **AI Assistant:** Natural language rule creation
- **Plugin System:** Custom validators and actions

---

## 15. Open Questions

| Question | Status | Decision |
|----------|--------|----------|
| Should we support multiple config file formats? | Open | YAML only for Phase 1 |
| How to handle very large log files? | Resolved | Virtual scrolling in Phase 2 |
| Should simulator mock CCH or call actual binary? | Resolved | Call actual CCH binary |
| Include rule creation wizard? | Deferred | Phase 3 |

---

## 16. Appendix

### Appendix A: JSON Schema

See `public/schema/hooks-schema.json` in the plan document for the complete JSON Schema for hooks.yaml validation.

### Appendix B: Keyboard Shortcuts

| Shortcut | Action |
|----------|--------|
| `Cmd/Ctrl + S` | Save current file |
| `Cmd/Ctrl + Z` | Undo |
| `Cmd/Ctrl + Shift + Z` | Redo |
| `Cmd/Ctrl + F` | Find in editor |
| `Cmd/Ctrl + /` | Toggle comment |
| `Cmd/Ctrl + D` | Run debug simulation |
| `Cmd/Ctrl + ,` | Open settings |
| `Cmd/Ctrl + 1` | Switch to Editor tab |
| `Cmd/Ctrl + 2` | Switch to Simulator tab |

### Appendix C: Color Palette

**Light Theme:**
| Element | Hex |
|---------|-----|
| Background | `#FFFFFF` |
| Surface | `#F5F5F5` |
| Text Primary | `#1A1A1A` |
| Accent | `#3B82F6` |
| Error | `#EF4444` |
| Success | `#10B981` |

**Dark Theme:**
| Element | Hex |
|---------|-----|
| Background | `#1A1A1A` |
| Surface | `#252525` |
| Text Primary | `#F5F5F5` |
| Accent | `#60A5FA` |
| Error | `#F87171` |
| Success | `#34D399` |

### Appendix D: CCH Binary Requirements

| Command | Purpose | Required Flags |
|---------|---------|----------------|
| `cch debug` | Simulate events | `--json` for structured output |
| `cch validate` | Validate config | `--json` for structured output |
| `cch --version` | Version check | - |

---

## Document History

| Version | Date | Author | Changes |
|---------|------|--------|---------|
| 1.0 | 2026-01-23 | Claude Code | Initial PRD based on plan |

---

*This PRD serves as the specification for RuleZ UI development and will be used to initialize the SDD (Spec-Driven Development) workflow.*
