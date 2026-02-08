# Codebase Structure

**Analysis Date:** 2026-02-06

## Directory Layout

```
rulez_plugin/
├── cch_cli/                    # Rust CLI policy engine (main binary)
│   ├── src/
│   │   ├── main.rs            # Entry point, subcommand routing
│   │   ├── lib.rs             # Public API exports
│   │   ├── cli.rs             # CLI command module index
│   │   ├── cli/               # Subcommand implementations
│   │   │   ├── debug.rs       # Debug/simulate event evaluation
│   │   │   ├── explain.rs     # Explain rules and past events
│   │   │   ├── init.rs        # Initialize project config
│   │   │   ├── install.rs     # Register hook with Claude Code
│   │   │   ├── logs.rs        # Query audit logs
│   │   │   └── validate.rs    # Validate config file
│   │   ├── hooks.rs           # Core rule evaluation engine
│   │   ├── config.rs          # Config loading and parsing
│   │   ├── models.rs          # Type definitions (Event, Rule, Action, etc.)
│   │   ├── logging.rs         # JSON Lines audit logger
│   │   └── Cargo.toml         # Dependencies
│   └── tests/                 # Integration tests (IQ, OQ, PQ)
│       ├── common/            # Shared test utilities
│       ├── fixtures/          # Test data (example configs, events)
│       ├── e2e_git_push_block.rs
│       ├── iq_installation.rs # Installation tests
│       ├── iq_new_commands.rs
│       ├── oq_us1_blocking.rs # Operational acceptance tests
│       ├── oq_us2_injection.rs
│       ├── oq_us3_validators.rs
│       ├── oq_us4_permissions.rs
│       ├── oq_us5_logging.rs
│       ├── pq_memory.rs       # Performance tests
│       └── pq_performance.rs
│
├── rulez_ui/                   # React desktop app (Tauri 2.0)
│   ├── src/
│   │   ├── main.tsx           # React entry point
│   │   ├── App.tsx            # App root component
│   │   ├── components/        # React components
│   │   │   ├── editor/        # YAML editor components (7 files)
│   │   │   │   ├── YamlEditor.tsx       # Monaco editor wrapper
│   │   │   │   ├── RuleCard.tsx         # Rule display card
│   │   │   │   ├── RuleTreeView.tsx     # Hierarchical rule browser
│   │   │   │   ├── ValidationPanel.tsx  # Error/warning display
│   │   │   │   └── EditorToolbar.tsx    # Save/validate buttons
│   │   │   ├── simulator/     # Debug simulator (3 files)
│   │   │   │   ├── DebugSimulator.tsx   # Simulator container
│   │   │   │   ├── EventForm.tsx        # Form to create test events
│   │   │   │   └── ResultView.tsx       # Display evaluation results
│   │   │   ├── layout/        # App layout (5 files)
│   │   │   │   ├── AppShell.tsx         # Main app container
│   │   │   │   ├── Header.tsx           # Top bar
│   │   │   │   ├── Sidebar.tsx          # File list and nav
│   │   │   │   ├── MainContent.tsx      # Editor area
│   │   │   │   ├── RightPanel.tsx       # Simulator/debug area
│   │   │   │   └── StatusBar.tsx        # Bottom status indicators
│   │   │   ├── files/         # File management (1 file)
│   │   │   │   └── FileTabBar.tsx       # Open file tabs
│   │   │   └── ui/            # Reusable UI primitives (1 file)
│   │   │       └── ConfirmDialog.tsx    # Confirmation modal
│   │   ├── stores/            # Zustand state management
│   │   │   ├── configStore.ts # Config files and editor state
│   │   │   ├── editorStore.ts # Cursor, selection, validation state
│   │   │   └── uiStore.ts     # Theme preference
│   │   ├── types/             # TypeScript type definitions
│   │   │   └── index.ts
│   │   ├── lib/               # Utilities
│   │   │   ├── tauri.ts       # Tauri IPC wrapper (with web fallback)
│   │   │   ├── yaml-utils.ts  # YAML parsing/formatting
│   │   │   ├── schema.ts      # JSON schema utilities
│   │   │   └── tauri.test.ts  # Tauri detection tests
│   │   ├── styles/            # Styling
│   │   │   ├── globals.css    # Tailwind setup, CSS variables
│   │   │   └── monaco-theme.ts # Editor theme configuration
│   │   └── vite-env.d.ts      # Vite type declarations
│   ├── src-tauri/             # Tauri Rust backend (not explored in detail)
│   ├── public/
│   │   ├── schema/            # JSON schema files (for Monaco validation)
│   │   └── rulez-icon.svg
│   ├── index.html             # HTML entry point
│   ├── package.json           # Dependencies (React, Tauri, Monaco, Zustand, etc.)
│   ├── playwright.config.ts   # E2E test configuration
│   ├── biome.json             # Linter/formatter config
│   ├── bunfig.toml            # Bun runtime config
│   ├── vite.config.ts         # Vite build config
│   ├── tsconfig.json          # TypeScript config
│   └── CLAUDE.md              # Project-specific guidance
│
├── mastering-hooks/           # CLI skill package for Mastering Hooks course
│   ├── assets/
│   │   ├── check-secrets.sh   # Example validator script
│   │   ├── hooks-template.yaml
│   │   └── python-standards.md
│   ├── references/            # Documentation
│   │   ├── cli-commands.md
│   │   ├── hooks-yaml-schema.md
│   │   ├── quick-reference.md
│   │   ├── rule-patterns.md
│   │   └── troubleshooting-guide.md
│   └── SKILL.md
│
├── docs/                      # Project documentation
│   ├── README.md              # Main documentation index
│   ├── USER_GUIDE_CLI.md      # CCH CLI user guide
│   ├── USER_GUIDE_SKILL.md    # Skill usage guide
│   ├── prds/                  # Product requirements
│   │   ├── cch_cli_prd.md
│   │   ├── rulez_ui_prd.md
│   │   └── phase2_prd.md
│   ├── plans/                 # Implementation plans
│   ├── devops/                # DevOps documentation
│   │   ├── BRANCHING.md
│   │   ├── CI_TIERS.md
│   │   └── RELEASE_PROCESS.md
│   └── validation/            # Test documentation (IQ/OQ/PQ)
│       ├── iq/
│       ├── oq/
│       ├── pq/
│       └── README.md
│
├── .claude/                   # Claude Code customizations
│   ├── commands/              # Custom CLI commands
│   ├── context/               # Context files
│   └── skills/                # Custom skills
│
├── .speckit/                  # SDD methodology artifacts
│   ├── memory/                # Project decisions
│   ├── features/              # Feature specifications
│   └── templates/             # SDD templates
│
├── .planning/                 # GSD documentation (generated)
│   └── codebase/              # This directory
│
├── test/                      # Top-level test data
│   └── integration/
│       ├── use-cases/         # Integration test scenarios
│       └── results/           # Test result output
│
├── Cargo.toml                 # Workspace root
├── Cargo.lock                 # Dependency lock
├── Taskfile.yml               # Task runner config
├── CLAUDE.md                  # Project guidance
├── CHANGELOG.md               # Release notes
└── .gitignore
```

## Directory Purposes

**cch_cli/src:**
- Purpose: Core policy engine implementation
- Contains: CLI logic, config parsing, rule evaluation, logging infrastructure
- Key files: `hooks.rs` (evaluation engine), `models.rs` (type definitions), `main.rs` (entry point)

**cch_cli/tests:**
- Purpose: Comprehensive integration testing (IQ/OQ/PQ layers)
- Contains: Installation tests, operational acceptance tests, performance tests
- Key files: Tests organized by phase (iq_*, oq_*, pq_*)

**rulez_ui/src/components:**
- Purpose: All React UI components organized by function
- Contains: 18 TypeScript/React files across 5 subdirectories
- Key files: `AppShell.tsx` (layout root), `YamlEditor.tsx` (editor), `DebugSimulator.tsx` (testing)

**rulez_ui/src/stores:**
- Purpose: Global state management using Zustand
- Contains: Three stores for distinct concerns
- Key files: All three stores read/write file state and UI preferences

**rulez_ui/src/lib:**
- Purpose: Utilities and helpers
- Contains: Tauri IPC wrappers, YAML parsing, schema utilities
- Key files: `tauri.ts` (handles both Tauri and web modes)

**docs/:**
- Purpose: User and developer documentation
- Contains: User guides, PRDs, implementation plans, test documentation
- Key files: `README.md` (main index), `USER_GUIDE_CLI.md` (CLI reference)

**mastering-hooks/:**
- Purpose: Educational skill package for Mastering Hooks course
- Contains: Example configs, validator scripts, references
- Key files: `references/hooks-yaml-schema.md`, `assets/check-secrets.sh`

**.speckit/:**
- Purpose: SDD methodology artifacts and decisions
- Contains: Project constitution, feature specs, templates
- Key files: `memory/` for architectural decisions

## Key File Locations

**Entry Points:**
- `cch_cli/src/main.rs`: CLI binary entry point
- `rulez_ui/src/main.tsx`: React app entry point
- `rulez_ui/src/App.tsx`: App root component

**Configuration:**
- `Cargo.toml`: Workspace manifest (Rust dependencies)
- `rulez_ui/package.json`: Node dependencies (React, Tauri, etc.)
- `rulez_ui/vite.config.ts`: Build configuration
- `rulez_ui/playwright.config.ts`: E2E test configuration
- `rulez_ui/biome.json`: Linting/formatting rules
- `Taskfile.yml`: Development task definitions

**Core Logic:**
- `cch_cli/src/hooks.rs`: Rule evaluation engine (1200+ lines)
- `cch_cli/src/models.rs`: All type definitions (1500+ lines)
- `cch_cli/src/config.rs`: Config loading and validation
- `rulez_ui/src/stores/configStore.ts`: File and editor state management

**Testing:**
- `cch_cli/tests/*.rs`: Integration tests (IQ/OQ/PQ)
- `cch_cli/tests/fixtures/`: Test data (configs, events)
- `cch_cli/tests/common/`: Test utilities
- `rulez_ui/playwright.config.ts`: E2E test setup
- E2E tests in `rulez_ui/src/` with `.test.ts` extension

## Naming Conventions

**Files:**
- Rust: `snake_case.rs` (e.g., `config.rs`, `models.rs`, `cli.rs`)
- TypeScript: `PascalCase.tsx` for components (e.g., `YamlEditor.tsx`), `camelCase.ts` for utilities (e.g., `tauri.ts`)
- Tests: `suffix.rs` for Rust (e.g., `iq_installation.rs`), `.test.ts` for TypeScript (e.g., `tauri.test.ts`)

**Directories:**
- Rust modules: `snake_case/` (e.g., `cch_cli/src/cli/`)
- Component groups: `kebab-case/` (e.g., `components/layout/`, `components/editor/`)
- Utility directories: `camelCase/` (e.g., `stores/`, `types/`)

**Modules:**
- Rust: `mod.rs` pattern (index files in subdirectories, e.g., `cch_cli/src/cli.rs`)
- TypeScript: Named exports, barrel index files (e.g., `types/index.ts`)

**Functions:**
- Rust: `snake_case` (e.g., `process_event`, `evaluate_rules`)
- TypeScript: `camelCase` (e.g., `useConfigStore`, `isTauri`)

**Types/Classes:**
- Rust: `PascalCase` (e.g., `Event`, `Rule`, `Config`)
- TypeScript: `PascalCase` for components and types (e.g., `AppShell`, `ConfigFile`)

**Constants:**
- Rust: `SCREAMING_SNAKE_CASE` (e.g., `DEFAULT_TIMEOUT`)
- TypeScript: `SCREAMING_SNAKE_CASE` or `camelCase` depending on usage (e.g., `DEFAULT_THEME`)

## Where to Add New Code

**New Feature - Rust CLI:**
- Primary code: `cch_cli/src/` (add to appropriate module: `hooks.rs`, `models.rs`, `cli/*.rs`)
- Tests: `cch_cli/tests/` (add integration test for the phase level)
- Models: Extend types in `cch_cli/src/models.rs`
- Examples: Update `mastering-hooks/assets/hooks-template.yaml` and docs

**New Feature - React UI:**
- Components: Create `.tsx` file in `rulez_ui/src/components/{category}/`
- State: Add store methods to `rulez_ui/src/stores/{appropriate-store}.ts`
- Styles: Use Tailwind classes; add CSS variables to `rulez_ui/src/styles/globals.css` if needed
- Types: Extend `rulez_ui/src/types/index.ts`
- Tests: Add `.test.ts` file co-located with component or in test suite

**New Command (CLI):**
1. Create `cch_cli/src/cli/{command_name}.rs` with `pub async fn run() -> Result<()>`
2. Add `pub mod {command_name};` to `cch_cli/src/cli.rs`
3. Add variant to `Commands` enum in `cch_cli/src/main.rs`
4. Add match arm in main.rs to call the command
5. Add integration test in `cch_cli/tests/`

**New Matcher/Action Type:**
1. Add variant to enum in `cch_cli/src/models.rs` (e.g., `MatcherKind::NewType`)
2. Implement evaluation logic in `cch_cli/src/hooks.rs`
3. Update YAML schema in `mastering-hooks/references/hooks-yaml-schema.md`
4. Add test case in `cch_cli/tests/`

**New Component Category:**
1. Create directory: `rulez_ui/src/components/{new_category}/`
2. Create index component: `rulez_ui/src/components/{new_category}/Index.tsx`
3. Create sub-components as needed
4. Export from `components/` barrel if needed
5. Add E2E tests for new interactions

**Utilities & Helpers:**
- Shared helpers: `cch_cli/src/` (module in appropriate file)
- UI utilities: `rulez_ui/src/lib/{function}.ts`
- Shared types: `rulez_ui/src/types/index.ts` (TypeScript) or `cch_cli/src/models.rs` (Rust)

## Special Directories

**cch_cli/tests/fixtures/:**
- Purpose: Test data (example configs, mock events, expected outputs)
- Generated: No (committed to repo)
- Committed: Yes
- Usage: Tests load from this directory to test config parsing and evaluation

**rulez_ui/src-tauri/:**
- Purpose: Tauri Rust backend (Tauri 2.0 commands)
- Generated: No (but excluded from workspace in Cargo.toml)
- Committed: Yes, but excluded from Cargo workspace
- Usage: Defines IPC commands for file I/O and CCH integration

**rulez_ui/playwright-report/:**
- Purpose: E2E test output and reports
- Generated: Yes (created by Playwright after test runs)
- Committed: No (.gitignore)
- Usage: View test results with `npx playwright show-report`

**.planning/codebase/:**
- Purpose: GSD methodology documentation (ARCHITECTURE.md, STRUCTURE.md, etc.)
- Generated: Yes (created by mapping commands)
- Committed: Yes
- Usage: Referenced by execution commands for implementation guidance

**docs/validation/:**
- Purpose: Test documentation for IQ/OQ/PQ phases
- Generated: No (template files committed)
- Committed: Yes, with test results added
- Usage: Track acceptance test completion and sign-off

---

*Structure analysis: 2026-02-06*
