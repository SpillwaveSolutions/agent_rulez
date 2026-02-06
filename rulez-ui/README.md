# RuleZ UI

Desktop application for visual CCH (Claude Context Hooks) configuration editing.

## Features

- **Visual YAML Editor** - Monaco Editor with syntax highlighting and schema validation
- **Real-time Validation** - Inline error markers as you type
- **Debug Simulator** - Test rules without running Claude Code
- **Multi-file Support** - Edit global and project configurations
- **Rule Tree View** - Visual representation of configured rules
- **Dark/Light Themes** - System preference detection

## Technology Stack

- **Runtime**: Bun (all TypeScript/React operations)
- **Frontend**: React 18 + TypeScript + Tailwind CSS 4
- **Editor**: Monaco Editor + monaco-yaml
- **Desktop**: Tauri 2.0 (Rust backend)
- **State**: Zustand + TanStack Query
- **Linting**: Biome
- **Testing**: Bun test (unit) + Playwright (E2E)

## Development

### Prerequisites

- [Bun](https://bun.sh/) (latest)
- [Rust](https://rustup.rs/) (1.70+)
- For Linux: `libwebkit2gtk-4.1-dev libappindicator3-dev librsvg2-dev`

### Installation

```bash
cd rulez_ui
bun install
```

### Commands

```bash
# Start dev server (browser mode)
bun run dev

# Start dev server (Tauri desktop mode)
bun run dev:tauri

# Run linter
bun run lint

# Run type checker
bun run typecheck

# Run unit tests
bun test

# Run E2E tests
bun run test:e2e

# Build desktop app
bun run build:tauri
```

## Architecture

### Dual-Mode Architecture

RuleZ UI supports two modes:

1. **Desktop Mode** (Primary) - Full Tauri integration with native file access and CCH binary execution
2. **Web Mode** (Testing) - Browser-based with mock data for Playwright E2E testing

The `src/lib/tauri.ts` module provides the abstraction layer that detects the runtime environment and uses the appropriate implementation.

### Directory Structure

```
rulez_ui/
├── src/                      # React frontend
│   ├── components/           # UI components
│   │   ├── editor/          # YamlEditor, ValidationPanel
│   │   ├── files/           # FileSidebar, FileTabBar
│   │   ├── layout/          # AppShell, Header, Sidebar
│   │   ├── simulator/       # DebugSimulator, EventForm
│   │   └── ui/              # Button, ThemeToggle, etc.
│   ├── hooks/               # Custom React hooks
│   ├── lib/                 # Utilities (tauri.ts, mock-data.ts)
│   ├── stores/              # Zustand stores
│   ├── styles/              # CSS and theme files
│   └── types/               # TypeScript type definitions
├── src-tauri/               # Rust backend
│   └── src/commands/        # Tauri IPC commands
├── tests/                   # Playwright E2E tests
└── public/                  # Static assets
```

## Phase 1 Implementation Status

- [x] M1: Project Setup (Tauri + React + Bun scaffold)
- [ ] M2: Monaco Editor (YAML syntax highlighting)
- [ ] M3: Schema Validation (JSON Schema, inline errors)
- [ ] M4: File Operations (read/write, global + project)
- [ ] M5: Rule Tree View (visual tree, navigation)
- [ ] M6: Debug Simulator (event form, CCH integration)
- [ ] M7: Theming (dark/light, system preference)
- [ ] M8: Playwright Tests (E2E suite, CI)

## License

MIT
