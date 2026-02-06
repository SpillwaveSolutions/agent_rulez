# External Integrations

**Analysis Date:** 2026-02-06

## APIs & External Services

**None detected** - This is a self-contained policy engine. The codebase does not integrate with external APIs or third-party services.

**Intra-application IPC:**
- Tauri IPC bridge between React frontend and Rust backend via `@tauri-apps/api`
- Shell plugin for executing the `cch` CLI binary from Tauri desktop app

## Data Storage

**Databases:**
- Not used - This project uses file-based configuration only

**File Storage:**
- Local filesystem only
  - Global config: `~/.claude/hooks.yaml` (home directory)
  - Project config: `.claude/hooks.yaml` (project root)
  - Accessed via: `tokio::fs` (async file I/O) and `dirs` crate for home directory resolution
  - Tauri backend commands: `read_config`, `write_config`, `list_config_files` in `rulez_ui/src-tauri/src/commands/config.rs`
  - File operations are async and use tokio for non-blocking I/O

**Caching:**
- None - All operations read/write fresh from filesystem

## Authentication & Identity

**Auth Provider:**
- None - This is an internal policy engine with no user authentication
- All authentication/authorization is managed through Claude IDE hooks (external to this codebase)

## Monitoring & Observability

**Error Tracking:**
- None detected - No external error tracking service integration

**Logs:**
- Tracing-based structured logging (Rust)
  - Configured via `tracing-subscriber` with `env-filter` support
  - Controlled via CLI flag: `--debug-logs` for full event and rule details
  - Log format: stdout/stderr, no file persistence configured
  - Location: `cch_cli/src/logging.rs` defines logging setup
  - Timestamp via `chrono` for structured log entries

**Log Levels:**
- Configurable via YAML: `settings.log_level` in hooks.yaml (default: "info")
- Debug mode available via CLI flag for detailed tracing

## CI/CD & Deployment

**Hosting:**
- Not applicable - This is a desktop/CLI application
- No cloud deployment. Binaries are distributed locally or via package managers.

**CI Pipeline:**
- GitHub Actions workflows detected (`.github/workflows/`)
- Tauri build system includes platform-specific building via `tauri build` command

**Build Process:**
- Frontend: `bun run build` -> TypeScript/Vite compilation to `dist/` directory
- Tauri: `bun run build:tauri` -> Generates native binaries for current platform
- CLI: `cargo build --release` -> Compiles Rust binary

## Environment Configuration

**Required env vars:**
- None critical - The system is file-based
- Optional env vars for development:
  - `TAURI_DEBUG` - Enable debug build with sourcemaps
  - `TAURI_PLATFORM` - Override platform detection (windows, macos, linux)
  - `VITE_*` - Custom Vite build variables (passed through)
  - `TAURI_*` - Custom Tauri build variables (passed through)

**Secrets location:**
- Not applicable - No secrets or API keys required
- Configuration files (hooks.yaml) contain rule definitions only, not sensitive data

## File System Integration

**Local File Access:**
- Config file reading/writing via Tauri commands:
  - `list_config_files(projectDir?: string)` - Lists global and project config files
  - `read_config(path: string)` - Reads YAML config from filesystem
  - `write_config(path: string, content: string)` - Writes YAML config to filesystem
  - Implementation: `rulez_ui/src-tauri/src/commands/config.rs`

**Path Expansion:**
- `~` (tilde) expansion to home directory via `dirs::home_dir()`
- Automatic parent directory creation when writing config files

**File Permissions:**
- Standard filesystem permissions apply
- No special permission handling beyond OS defaults

## CLI Integration (CCH Binary)

**Shell Plugin:**
- Tauri shell plugin configured to execute `cch` binary
- Scope limited to `cch` command with arguments allowed
- Configuration: `tauri.conf.json` plugins section
- Enables running `cch debug`, `cch validate`, etc. from Tauri backend

**Commands Available:**
- `run_debug` - Invokes `cch debug` with event type and parameters
- `validate_config` - Invokes `cch` validation for config files
- Implementation: `rulez_ui/src-tauri/src/commands/debug.rs`

## Webhooks & Callbacks

**Incoming:**
- None - CCH is triggered via Claude IDE hooks, not webhooks

**Outgoing:**
- None - CCH does not send data to external systems

## Frontend-Backend Communication

**IPC Channel:**
- Tauri IPC bridge via `@tauri-apps/api/core::invoke`
- Frontend wrappers in `rulez_ui/src/lib/tauri.ts`
- Mock fallbacks for browser testing mode (Playwright E2E)

**Commands (Frontend → Backend):**
1. `list_config_files(projectDir?: string): Promise<ConfigFile[]>`
2. `read_config(path: string): Promise<string>`
3. `write_config(path: string, content: string): Promise<void>`
4. `run_debug(params: DebugParams): Promise<DebugResult>`
5. `validate_config(path: string): Promise<{ valid: boolean; errors: string[] }>`

**Return Types:**
- All commands return serialized JSON via serde
- DebugResult includes timing data, matched rules, evaluation details
- Async/await pattern for all IPC calls

---

*Integration audit: 2026-02-06*
