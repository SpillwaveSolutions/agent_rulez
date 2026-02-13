# Technology Stack

**Analysis Date:** 2026-02-06

## Languages

**Primary:**
- Rust 2021 edition - CLI binary (`cch_cli`) and Tauri backend for desktop app
- TypeScript 5.7+ - React frontend for RuleZ UI with strict type checking

**Secondary:**
- YAML - Configuration file format for CCH hooks (.claude/hooks.yaml)

## Runtime

**Environment:**
- Node.js via Bun 1.x - Package manager and test runner for rulez_ui TypeScript/React
- Rust 1.70+ - Native runtime for CLI and Tauri backend
- Tauri 2.0 - Desktop runtime with native OS integration (Chromium on Windows, WebKit on macOS/Linux)

**Package Manager:**
- Bun (primary for rulez_ui TypeScript operations)
- Cargo (Rust package manager, version 1.x)
- Lockfiles: Cargo.lock (Rust), bun.lockb (Bun)

## Frameworks

**Core:**
- Tauri 2.0 - Desktop application framework (Rust + web frontend bridge)
- React 18.3.1 - UI component framework
- Vite 6.1.0 - Frontend build tool and dev server
- Tokio 1.0 - Async runtime for Rust (features: process, time, fs, io-std, io-util, rt, macros)

**Editor/UI Libraries:**
- Monaco Editor 4.7.0 (@monaco-editor/react) - Code editor component
- monaco-yaml 5.3.1 - YAML language support for Monaco Editor
- Tailwind CSS 4.0.6 - Utility-first CSS framework
- Zustand 5.0.3 - Lightweight state management

**Testing:**
- Bun test - Unit testing framework (built-in to Bun)
- Playwright 1.50.1 (@playwright/test) - E2E testing framework
- Jest-compatible test syntax via Bun

**Build/Dev Tools:**
- Biome 1.9.4 (@biomejs/biome) - Unified linter and formatter (replaces ESLint + Prettier)
- TypeScript 5.7.3 - Type checking via tsc --noEmit
- Tauri CLI 2.3.0 - Desktop app build and development

**Data Serialization:**
- Serde 1.0 - Serialization framework (Rust)
- serde_json 1.0 - JSON serialization (Rust)
- serde_yaml 0.9 - YAML serialization (Rust)
- yaml 2.8.2 - YAML parsing (TypeScript)

## Key Dependencies

**Critical:**
- @tauri-apps/api 2.5.0 - IPC bridge for frontend-backend communication
- @tauri-apps/plugin-shell 2.2.1 (Rust 2.0) - Shell command execution from Tauri backend
- clap 4.0 - CLI argument parsing (Rust)
- regex 1.10 - Pattern matching engine (Rust)
- anyhow 1.0 - Error handling (Rust)
- thiserror 1.0 - Error type derivation (Rust)
- Zod - Schema validation library (TypeScript, in dependencies)

**Infrastructure:**
- tracing 0.1 - Structured logging (Rust)
- tracing-subscriber 0.3 - Logging infrastructure (Rust)
- chrono 0.4 - Date/time handling with serde support (Rust)
- dirs 5.0 - Cross-platform home directory access (Rust)
- TanStack Query 5.64.0 - Async data management (React, future use)

**Testing Dependencies:**
- tempfile 3.24 - Temporary file creation (Rust tests)
- assert_cmd 2.0 - CLI testing utilities (Rust)
- predicates 3.1 - Assertion combinators (Rust)
- @types/react 18.3.18 - React type definitions
- @types/react-dom 18.3.5 - React DOM type definitions
- @types/bun 1.2.4 - Bun runtime type definitions

## Configuration

**Environment:**
- No .env files detected - configuration is file-based (YAML)
- CCH system uses YAML files: `~/.claude/hooks.yaml` (global) and `.claude/hooks.yaml` (project)
- Vite uses `VITE_` and `TAURI_` prefixed environment variables

**Build:**
- `Cargo.toml` (workspace root) - Rust workspace configuration
- `Cargo.toml` (cch_cli/) - CLI binary package configuration
- `Cargo.toml` (rulez_ui/src-tauri/) - Tauri backend package configuration
- `vite.config.ts` - Frontend build configuration
- `tauri.conf.json` - Tauri application configuration
- `tsconfig.json` - TypeScript configuration (strict mode)
- `biome.json` - Linting/formatting configuration
- `.clippy.toml` - Clippy linter configuration (MSRV 1.85.0, cognitive complexity threshold 25)
- `rustfmt.toml` - Rust code formatting configuration

## Platform Requirements

**Development:**
- macOS, Linux, or Windows with Rust toolchain
- Bun 1.x or compatible Node.js runtime
- Tauri development prerequisites (Xcode on macOS, MSVC on Windows, build-essentials on Linux)

**Production:**
- macOS 10.15+ (arm64/x86_64)
- Windows 10+ (x86_64)
- Linux distributions with GTK 3+ or newer
- Target-specific binaries built with Tauri (native apps)

## Release Profile

**Optimization:**
```
opt-level = 3
lto = true (link-time optimization)
codegen-units = 1 (single unit for better optimization)
panic = "abort"
strip = true (strip symbols from binaries)
```

All workspace members use these settings for release builds to minimize binary size and maximize performance.

---

*Stack analysis: 2026-02-06*
