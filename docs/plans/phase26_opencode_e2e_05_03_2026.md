# Phase 26: OpenCode CLI E2E Testing — Deep Research + Implementation Plan

## Context

Phase 26 adds OpenCode CLI adapter + E2E scenarios to the existing test harness (established Phase 23, pattern proven by Phases 24-25). The roadmap entry says "Add OpenCode CLI adapter + scenarios to the existing E2E harness." The user also wants **deep research docs** on OpenCode plugins written to `.planning/phases/26-opencode-cli-e2e-testing/`.

**Key insight:** Unlike Claude Code (shell hooks) and Copilot (JSON hook config), OpenCode uses a **plugin system** with TypeScript/JavaScript lifecycle callbacks loaded via Bun runtime. RuleZ already has a Rust-side OpenCode adapter (`rulez/src/adapters/opencode.rs`) that accepts JSON on stdin and emits JSON on stdout — the same `rulez opencode hook` command used by the existing install command. The E2E tests need to verify this integration works end-to-end.

**What already exists in the codebase:**
- `rulez/src/adapters/opencode.rs` — parses OpenCode events, maps to RuleZ EventTypes, canonicalizes tool names
- `rulez/src/opencode/dispatcher.rs` — dispatches events through RuleZ policy engine, audit logs
- `rulez/src/opencode/config.rs` — plugin config from `~/.config/opencode/plugins/rulez-plugin/settings.json`
- `rulez/src/cli/opencode_hook.rs` — reads stdin JSON, calls adapter, emits response JSON (exit 2 on deny)
- `rulez/src/cli/opencode_install.rs` — writes hook entries into `.opencode/settings.json`
- Event mapping: `tool.execute.before` -> PreToolUse, `tool.execute.after` -> PostToolUse, `session.created` -> SessionStart, etc.

---

## Deliverables

### Deliverable 1: Deep Research Document
**File:** `.planning/phases/26-opencode-cli-e2e-testing/26-RESEARCH.md`

Comprehensive reference covering OpenCode plugin architecture, API, and how RuleZ integrates. Content synthesized from Perplexity research + codebase analysis.

### Deliverable 2: OpenCode E2E Adapter + Fixtures
**File:** `e2e/lib/opencode_adapter.sh`

### Deliverable 3: E2E Scenario Scripts
**Files:** `e2e/scenarios/opencode/01-install.sh` through `04-inject.sh`

### Deliverable 4: Fixture YAML Files
**Files:** `e2e/fixtures/opencode/hooks-hookfire.yaml`, `hooks-deny.yaml`, `hooks-inject.yaml.template`

### Deliverable 5: Updated run.sh
**File:** `e2e/run.sh` — add OpenCode adapter source + availability check block

### Deliverable 6: Fix Stale `cch` References in opencode_install.rs
**File:** `rulez/src/cli/opencode_install.rs`

Fix `resolve_binary_path()` to search for `rulez` (not `cch`) and `is_cch_hook()` to detect both `cch` and `rulez` command strings.

### Deliverable 7: TypeScript Plugin for OpenCode Users
**Files:** `opencode-plugin/rulez-plugin/index.ts`, `opencode-plugin/rulez-plugin/package.json`, `opencode-plugin/rulez-plugin/tsconfig.json`, `opencode-plugin/README.md`

A working TypeScript plugin users can copy to `.opencode/plugins/rulez-plugin/`. It intercepts `tool.execute.before` and `tool.execute.after`, calls `rulez opencode hook` via Bun subprocess, and enforces allow/deny/inject decisions.

---

## Plan 26-01: Research Doc + Adapter + Fixtures

### Task 1: Write 26-RESEARCH.md (deep research doc)

**File:** `.planning/phases/26-opencode-cli-e2e-testing/26-RESEARCH.md`

Content to include:

#### OpenCode Plugin Architecture
- Plugins are JS/TS modules loaded by Bun runtime embedded in OpenCode (Go binary)
- Local plugins: `.opencode/plugins/<name>/index.ts` (auto-discovered)
- Global plugins: `~/.config/opencode/plugins/<name>/`
- NPM plugins: listed in `opencode.json` `"plugins"` section
- Load order: global config -> project config -> global plugins -> project plugins
- Hooks execute **sequentially** (not parallel)

#### Plugin File Structure
```
.opencode/plugins/rulez-plugin/
├── index.ts          # Main plugin (default export)
├── package.json      # { "type": "module", "main": "./index.ts" }
└── tsconfig.json     # Optional
```

#### Plugin Function Signature
```typescript
export default async function rulezPlugin({ project, directory, worktree, client, $ }) {
  return {
    async 'tool.execute.before'(ctx) { ... },
    async 'tool.execute.after'(ctx) { ... },
    async 'permission.asked'(ctx) { ... }
  };
}
```

#### Available Lifecycle Events
| Event | RuleZ Mapping | Description |
|-------|--------------|-------------|
| `tool.execute.before` | PreToolUse | Before tool invocation |
| `tool.execute.after` | PostToolUse | After tool execution |
| `session.created` | SessionStart | New session |
| `session.deleted` | SessionEnd | Session ended |
| `session.updated` | UserPromptSubmit | Session changed |
| `session.compacted` | PreCompact | Context compaction |
| `file.edited` | (mapped in adapter) | File changed |
| `permission.asked` | (custom) | Permission gate |

#### How RuleZ Integrates (Command-Based Hook)
- `rulez opencode install` writes hook entries into `.opencode/settings.json`
- Each hook entry: `{ "type": "command", "command": "rulez opencode hook", "timeout": 5 }`
- At runtime: OpenCode pipes JSON to `rulez opencode hook` stdin
- RuleZ parses, evaluates rules, outputs JSON response on stdout
- Exit code 2 = deny (blocks the tool call)

#### OpenCode Headless/Non-Interactive Mode
- `opencode --prompt "..." --format json` — headless with JSON output
- `opencode --session <id>` — attach to session
- `opencode --model provider/model` — select model
- No equivalent of `--dangerously-skip-permissions` found — use `--format json`

#### OpenCode Auth/Provider Config
- Provider configured via `opencode.json` or env vars
- No single API key env var — provider-specific
- `OPENCODE_EXPERIMENTAL_*` env vars for feature flags

#### Tool Name Mapping (from `rulez/src/adapters/opencode.rs`)
| OpenCode Name | Canonical (RuleZ) Name |
|--------------|----------------------|
| `bash` | `Bash` |
| `write` | `Write` |
| `edit` | `Edit` |
| `read` | `Read` |
| `glob` | `Glob` |
| `grep` | `Grep` |
| `task` | `Task` |
| `webfetch`/`fetch` | `WebFetch` |

### Task 2: Create OpenCode E2E Adapter

**File:** `e2e/lib/opencode_adapter.sh`

Following the exact pattern of `copilot_adapter.sh`:

```bash
# Exports: OPENCODE_CLI_NAME="opencode"
# Functions:
#   opencode_adapter_check()                               — verify opencode is in PATH
#   require_opencode_cli()                                 — return 77 if unavailable
#   setup_opencode_hooks(workspace, rulez_binary)          — write .opencode/settings.json with hook entries
#   invoke_opencode_headless(workspace, prompt, timeout)   — run opencode --prompt headlessly
```

Key differences from copilot adapter:
- **Check:** `command -v opencode` (no OAuth — provider config is separate)
- **Hook config path:** `$workspace/.opencode/settings.json` (not `.github/hooks/`)
- **Hook format:** JSON with `"hooks"` -> event name -> array of `{ "type": "command", "command": "rulez opencode hook", "timeout": 5 }`
- **Headless invocation:** `opencode --prompt "$prompt" --format json` (not `-p`)
- **RuleZ config:** Still at `.claude/hooks.yaml` (RuleZ's own config path — same across all CLIs)

### Task 3: Create Fixture YAML Files

**Files:** `e2e/fixtures/opencode/`

Three fixtures (identical content to copilot fixtures — RuleZ rules are CLI-agnostic):

1. `hooks-hookfire.yaml` — allows Bash, logs `e2e-hookfire-log`
2. `hooks-deny.yaml` — blocks force push, logs `e2e-deny-force-push`
3. `hooks-inject.yaml.template` — inject marker file at `__WORKSPACE__/e2e-inject-fired.marker`

---

## Plan 26-02: Four E2E Scenario Scripts

### Task 1: `e2e/scenarios/opencode/01-install.sh`

**Function:** `scenario_install(workspace, rulez_binary)`

- Run `rulez opencode install --scope project --binary $rulez_binary` from workspace
- Assert exit code 0
- Assert `.opencode/settings.json` exists
- Assert settings.json contains `"tool.execute.before"` hook entry
- Assert settings.json contains `"opencode hook"` command string

**Does NOT require live OpenCode CLI.**

### Task 2: `e2e/scenarios/opencode/02-hook-fire.sh`

**Function:** `scenario_hook_fire(workspace, rulez_binary)`

- `require_opencode_cli` or skip (exit 77)
- `setup_opencode_hooks` -> write `.opencode/settings.json`
- Copy `hooks-hookfire.yaml` -> `$workspace/.claude/hooks.yaml`
- `invoke_opencode_headless` with bash echo prompt
- Assert audit log contains `e2e-hookfire-log`

### Task 3: `e2e/scenarios/opencode/03-deny.sh`

**Function:** `scenario_deny(workspace, rulez_binary)`

- `require_opencode_cli` or skip
- `setup_opencode_hooks` + copy deny fixture
- Invoke with force push prompt
- Assert audit log contains `e2e-deny-force-push` and `block`

### Task 4: `e2e/scenarios/opencode/04-inject.sh`

**Function:** `scenario_inject(workspace, rulez_binary)`

- `require_opencode_cli` or skip
- `setup_opencode_hooks` + generate inject fixture from template
- Invoke with echo prompt
- Assert marker file `e2e-inject-fired.marker` exists
- Assert audit log contains `e2e-inject-marker`

---

## Plan 26-03: Fix Stale `cch` References in opencode_install.rs

### Task 1: Fix `resolve_binary_path()`

**File:** `rulez/src/cli/opencode_install.rs` (lines 196-215)

Change:
- `which "cch"` -> `which "rulez"` (line 196)
- `"./target/release/cch"` -> `"./target/release/rulez"` (line 205)
- `"./target/debug/cch"` -> `"./target/debug/rulez"` (line 210)
- Error message: `"Could not find CCH binary"` -> `"Could not find rulez binary"` (line 215)

### Task 2: Fix `is_cch_hook()` to detect both old and new names

**File:** `rulez/src/cli/opencode_install.rs` (lines 124-135)

Rename to `is_rulez_hook()` and match both `"cch"` and `"rulez"` in command string:
```rust
fn is_rulez_hook(hook: &OpenCodeHookEntry) -> bool {
    if let Some(hook_type) = hook.hook_type.as_deref() {
        if hook_type != "command" { return false; }
    }
    hook.command.as_deref()
        .map(|cmd| cmd.contains("rulez") || cmd.contains("cch"))
        .unwrap_or(false)
}
```

Update caller `remove_cch_hooks` -> `remove_rulez_hooks`.

### Task 3: Fix `print` message (line 93)

Change `"cch opencode install --print"` -> `"rulez opencode install --print"`.

### Task 4: Verify with existing tests

```bash
cargo test --tests --all-features --workspace -- opencode
```

---

## Plan 26-04: TypeScript Plugin for OpenCode Users

### Task 1: Create plugin directory structure

**Directory:** `opencode-plugin/rulez-plugin/`

### Task 2: Write `index.ts`

Main plugin file that:
- Exports default async plugin function receiving `{ project, directory, worktree, client, $ }`
- Returns hooks object with `tool.execute.before` and `tool.execute.after` handlers
- In `tool.execute.before`: builds JSON payload from event context, spawns `rulez opencode hook` via `Bun.spawn`, pipes JSON on stdin, reads JSON response from stdout
- If `response.continue === false`: throws Error with reason (blocks tool)
- If `response.context`: injects into event data
- In `tool.execute.after`: similar flow for post-tool audit logging
- Includes error handling: on subprocess failure, logs warning and allows (fail-open, matching RuleZ philosophy)

### Task 3: Write `package.json`

```json
{
  "name": "rulez-plugin",
  "version": "1.0.0",
  "type": "module",
  "main": "./index.ts",
  "description": "RuleZ policy engine plugin for OpenCode CLI",
  "dependencies": {},
  "devDependencies": {
    "@types/bun": "latest"
  }
}
```

### Task 4: Write `tsconfig.json`

Standard ESNext + bundler module resolution config.

### Task 5: Write `opencode-plugin/README.md`

Installation instructions:
1. Copy `rulez-plugin/` to `.opencode/plugins/` (project) or `~/.config/opencode/plugins/` (global)
2. Ensure `rulez` binary is in PATH (or set `RULEZ_BINARY_PATH` env var)
3. Create `.claude/hooks.yaml` with your rules
4. Restart OpenCode — plugin auto-loads

Alternatively: `rulez opencode install` for the command-based hook approach (no TS plugin needed).

---

## Plan 26-05: Update run.sh + Harness Integration

### Task 1: Update `e2e/run.sh`

Add after the copilot block (~line 35):
```bash
source "${E2E_ROOT}/lib/opencode_adapter.sh"
```

Add OpenCode availability check block in the CLI loop (~line 133):
```bash
if [[ "${cli_name}" == "opencode" ]]; then
  if opencode_adapter_check > /dev/null 2>&1; then
    OPENCODE_CLI_AVAILABLE=1
  else
    OPENCODE_CLI_AVAILABLE=0
    echo "  NOTE: opencode CLI not available — scenarios requiring it will be skipped" >&2
  fi
  export OPENCODE_CLI_AVAILABLE
fi
```

---

## Critical Files

| File | Action | Purpose |
|------|--------|---------|
| `.planning/phases/26-opencode-cli-e2e-testing/26-RESEARCH.md` | Create | Deep research doc |
| `rulez/src/cli/opencode_install.rs` | Modify | Fix stale cch->rulez refs |
| `e2e/lib/opencode_adapter.sh` | Create | Adapter library |
| `e2e/fixtures/opencode/hooks-hookfire.yaml` | Create | Hookfire fixture |
| `e2e/fixtures/opencode/hooks-deny.yaml` | Create | Deny fixture |
| `e2e/fixtures/opencode/hooks-inject.yaml.template` | Create | Inject fixture |
| `e2e/scenarios/opencode/01-install.sh` | Create | Install scenario |
| `e2e/scenarios/opencode/02-hook-fire.sh` | Create | Hook fire scenario |
| `e2e/scenarios/opencode/03-deny.sh` | Create | Deny scenario |
| `e2e/scenarios/opencode/04-inject.sh` | Create | Inject scenario |
| `e2e/run.sh` | Modify | Add opencode adapter source + check |
| `opencode-plugin/rulez-plugin/index.ts` | Create | TS plugin for OpenCode users |
| `opencode-plugin/rulez-plugin/package.json` | Create | Plugin package manifest |
| `opencode-plugin/rulez-plugin/tsconfig.json` | Create | TS config |
| `opencode-plugin/README.md` | Create | Plugin installation guide |

## Reuse from Existing Code

- `e2e/lib/harness.sh` — all assertion helpers (`assert_file_exists`, `assert_file_contains`, `assert_log_contains`, `assert_exit_code`), `setup_workspace`, `run_scenario`, `timer_*`
- `e2e/lib/reporting.sh` — `record_result`, report generation
- `e2e/lib/copilot_adapter.sh` — structural template for the OpenCode adapter
- `e2e/fixtures/copilot/` — content template for fixture YAML files
- `rulez/src/cli/opencode_install.rs` — install writes to `.opencode/settings.json`, uses `OPENCODE_HOOK_EVENTS` array, same JSON format we assert against

## Known Issues in Existing OpenCode Code

1. **`opencode_install.rs` line 196-215:** Still references `cch` binary name (stale after rename). The `resolve_binary_path` function searches for `cch` not `rulez`. This is a pre-existing bug.
2. **`opencode_install.rs` line 134:** `is_cch_hook` checks for `"cch"` in command string — should also check for `"rulez"`.

These bugs will be **fixed in Plan 26-03** as part of this phase (user confirmed).

---

## Verification

1. **Install scenario (no live CLI needed):**
   ```bash
   ./e2e/run.sh --cli opencode  # install scenario runs, others skip if no opencode CLI
   ```

2. **Full E2E (requires opencode CLI installed):**
   ```bash
   ./e2e/run.sh --cli opencode
   ```

3. **All CLIs together:**
   ```bash
   ./e2e/run.sh
   ```

4. Check reports: `.runs/<run-id>/junit.xml` and `.runs/<run-id>/summary.md` include OpenCode row.

---

## Execution Order

1. Write `26-RESEARCH.md` (deep research doc)
2. Fix stale `cch` refs in `opencode_install.rs` (Plan 26-03)
3. Run `cargo test -- opencode` to verify no regressions
4. Create fixture files (3 files)
5. Create adapter library (`opencode_adapter.sh`)
6. Create scenario scripts (4 files)
7. Update `run.sh`
8. Create TypeScript plugin (`opencode-plugin/rulez-plugin/`)
9. Write plugin README
10. Run full CI pipeline: `cargo fmt --all --check && cargo clippy ... && cargo test ... && cargo llvm-cov ...`
11. Test E2E: `./e2e/run.sh --cli opencode`
