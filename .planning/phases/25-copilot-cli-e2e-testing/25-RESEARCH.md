# Phase 25: Copilot CLI E2E Testing - Research

**Researched:** 2026-02-23
**Domain:** Shell-based E2E test harness extension — GitHub Copilot CLI adapter and scenarios
**Confidence:** HIGH (codebase is primary source; external Copilot docs verified via WebFetch/Perplexity)

---

<user_constraints>
## User Constraints (from CONTEXT.md)

### Locked Decisions
- Same 4 scenario pattern: install, hook-fire, deny, inject
- Copilot adapter follows same structure as claude_adapter.sh and gemini_adapter.sh
- RuleZ config always at .claude/hooks.yaml even for Copilot tests
- Copilot hook config at workspace level per Copilot's conventions
- Exit code 77 = skip when Copilot CLI unavailable

### Claude's Discretion
- Exact headless invocation flags for Copilot CLI
- Shell script structure and helper functions
- Fixture YAML contents
- Workspace cleanup strategy
- Timeout values per scenario

### Deferred Ideas (OUT OF SCOPE)
- Sister product E2E scenarios (Agent Memory, Agent Cron, Agent CLOD)
- Performance/load testing
- Cross-CLI rule compatibility matrix testing
- Automated golden file update workflow
- Phase 22.1: Expose tool_input fields in enabled_when eval context
</user_constraints>

---

## Summary

Phase 25 adds the third CLI to the E2E harness: GitHub Copilot CLI. The harness framework (Phase 23) and the pattern for adding new CLIs (Phase 24 / Gemini) already exist. This phase requires creating three deliverables: `e2e/lib/copilot_adapter.sh`, four scenario scripts under `e2e/scenarios/copilot/`, and YAML fixture files under `e2e/fixtures/copilot/`. The harness entry point `e2e/run.sh` also needs a Copilot availability check block mirroring the Claude and Gemini blocks.

The most important architectural difference from Gemini is that Copilot's hook configuration file lives at **`.github/hooks/rulez.json`** in the workspace root (not `.gemini/settings.json`). The `rulez copilot install` command targets this path. The install scenario must therefore assert the existence and structure of `.github/hooks/rulez.json` rather than a settings.json file. Scenarios 02-04 require a live `copilot` binary; the availability check returns exit code 77 when the binary is absent, matching the harness skip convention.

The Copilot hook runner (`rulez copilot hook`) reads a custom JSON envelope from stdin (the RuleZ internal format, which uses `session_id`, `hook_event_name`, `tool_name`, `tool_input`) and writes `{"permissionDecision":"allow"}` or `{"permissionDecision":"deny","permissionDecisionReason":"..."}` to stdout. The E2E scenarios for hook-fire, deny, and inject must invoke `copilot` headlessly to trigger the `.github/hooks/rulez.json` hook chain, then assert against the RuleZ audit log and marker files exactly as Gemini scenarios do.

**Primary recommendation:** Model the entire Copilot adapter and scenarios directly on `gemini_adapter.sh` and `e2e/scenarios/gemini/`. The only structural differences are the hook config file path (`.github/hooks/rulez.json` vs `.gemini/settings.json`), the hook JSON format, the CLI binary name (`copilot`), and the headless flags (`copilot -p "..." --allow-all-tools`).

---

## Standard Stack

### Core
| Component | Version/Location | Purpose | Notes |
|-----------|-----------------|---------|-------|
| Bash | 3.2+ (macOS compat) | All adapter and scenario scripts | Existing harness requires bash 3.2+ compat |
| `e2e/lib/harness.sh` | Existing | Workspace setup, assertions, run_scenario | Source unchanged |
| `e2e/lib/reporting.sh` | Existing | JUnit XML, ASCII table, Markdown summary | Already has copilot row slot via dynamic CLI list |
| `rulez copilot install` | Current binary | Writes `.github/hooks/rulez.json` | Tested in existing Rust unit tests |
| `rulez copilot hook` | Current binary | stdin->stdout hook runner | Tested in `copilot_hook_runner.rs` |
| `copilot` CLI | External — requires install | Headless agent invocation | Must be in PATH; GITHUB_TOKEN or prior login required |

### Supporting
| Component | Purpose | When to Use |
|-----------|---------|-------------|
| `e2e/fixtures/copilot/hooks-hookfire.yaml` | RuleZ policy for scenario 02 | Copied to `.claude/hooks.yaml` in workspace |
| `e2e/fixtures/copilot/hooks-deny.yaml` | RuleZ policy for scenario 03 | Force-push deny rule |
| `e2e/fixtures/copilot/hooks-inject.yaml.template` | RuleZ policy for scenario 04 | inject_command with `__WORKSPACE__` placeholder |

---

## Architecture Patterns

### File Layout to Create

```
e2e/
├── lib/
│   └── copilot_adapter.sh          # NEW — mirrors gemini_adapter.sh structure
├── scenarios/
│   └── copilot/
│       ├── 01-install.sh           # NEW
│       ├── 02-hook-fire.sh         # NEW
│       ├── 03-deny.sh              # NEW
│       └── 04-inject.sh            # NEW
└── fixtures/
    └── copilot/
        ├── hooks-hookfire.yaml     # NEW — same content as gemini version
        ├── hooks-deny.yaml         # NEW — same content as gemini version
        └── hooks-inject.yaml.template  # NEW — same content as gemini version
```

`e2e/run.sh` needs one new block (3 lines) to set `COPILOT_CLI_AVAILABLE`.

### Pattern 1: copilot_adapter.sh structure

Mirrors `gemini_adapter.sh` exactly. Functions:

```bash
COPILOT_CLI_NAME="copilot"
export COPILOT_CLI_NAME

copilot_adapter_check()          # verify `copilot` in PATH; check login/token
require_copilot_cli()            # return 77 if COPILOT_CLI_AVAILABLE != 1
setup_copilot_hooks(workspace, rulez_binary)   # write .github/hooks/rulez.json
invoke_copilot_headless(workspace, prompt, timeout_secs)  # run copilot -p ... --allow-all-tools
```

**Source:**
```bash
#!/usr/bin/env bash
# copilot_adapter.sh — GitHub Copilot CLI headless invocation helper

COPILOT_CLI_NAME="copilot"
export COPILOT_CLI_NAME

copilot_adapter_check() {
  if ! command -v copilot > /dev/null 2>&1; then
    echo "ERROR: 'copilot' CLI not found in PATH." >&2
    echo "  Install: npm install -g @github/copilot-cli or brew install copilot" >&2
    echo "  https://docs.github.com/en/copilot/how-tos/copilot-cli" >&2
    return 1
  fi

  local version
  version="$(copilot --version 2>&1 || true)"
  echo "copilot_adapter: found copilot CLI: ${version}"
  return 0
}

require_copilot_cli() {
  if [[ "${COPILOT_CLI_AVAILABLE:-0}" -eq 1 ]]; then
    return 0
  fi
  echo "  [skip] copilot CLI not available for headless invocation" >&2
  return 77
}
```

### Pattern 2: setup_copilot_hooks — writes .github/hooks/rulez.json

The Copilot hook config file lives at `.github/hooks/rulez.json` in the workspace root.
Format confirmed by `rulez/tests/copilot_install.rs` and `docs/COPILOT_CLI_HOOKS.md`:

```bash
setup_copilot_hooks() {
  local workspace="$1"
  local rulez_binary="$2"

  mkdir -p "${workspace}/.github/hooks"

  local abs_rulez
  abs_rulez="$(cd "$(dirname "${rulez_binary}")" && pwd)/$(basename "${rulez_binary}")"

  cat > "${workspace}/.github/hooks/rulez.json" <<EOF
{
  "version": 1,
  "hooks": {
    "preToolUse": [
      {
        "type": "command",
        "bash": "${abs_rulez} copilot hook",
        "powershell": "${abs_rulez} copilot hook",
        "timeoutSec": 10
      }
    ]
  }
}
EOF

  echo "copilot_adapter: wrote .github/hooks/rulez.json to ${workspace}/.github/hooks/rulez.json"
}
```

**Source (HIGH confidence):** Verified against `rulez/src/cli/copilot_install.rs` (which uses `COPILOT_HOOK_EVENTS = ["preToolUse", "postToolUse"]`) and `rulez/tests/copilot_install.rs` (which asserts `hooks.contains_key("preToolUse")`). The field names (`version`, `hooks`, `preToolUse`, `type`, `bash`, `powershell`, `timeoutSec`) are confirmed by the Rust struct `CopilotHookEntry` and GitHub's hooks-configuration reference page.

### Pattern 3: invoke_copilot_headless

```bash
invoke_copilot_headless() {
  local workspace="$1"
  local prompt="$2"
  local timeout_secs="${3:-120}"

  local output_file="${workspace}/copilot-output.txt"
  local exit_code=0

  (
    cd "${workspace}" && \
    NO_COLOR=true timeout "${timeout_secs}" copilot \
      -p "${prompt}" \
      --allow-all-tools 2>&1
  ) | tee "${output_file}" || exit_code="${PIPESTATUS[0]}"

  if [[ "${exit_code}" -eq 124 ]]; then
    echo "copilot_adapter: copilot timed out after ${timeout_secs}s — treating as skip" >&2
    return 77
  fi

  echo "copilot_adapter: copilot exited with code ${exit_code}"
  return "${exit_code}"
}
```

**Notes on headless flags (MEDIUM confidence from Perplexity + official docs):**
- `-p "prompt"` — single non-interactive prompt (confirmed in official docs)
- `--allow-all-tools` — auto-approves all tool calls without interactive prompts (confirmed in official docs)
- No `--output-format json` equivalent exists for Copilot CLI
- No `--max-turns` equivalent; rely on prompt design to constrain turns
- `NO_COLOR=true` suppresses ANSI escape codes in captured output

### Pattern 4: Scenario 01 — install (no live CLI needed)

```bash
scenario_install() {
  local workspace="$1"
  local rulez_binary="$2"
  local failures=0

  local install_output
  install_output="$(cd "${workspace}" && "${rulez_binary}" copilot install --binary "${rulez_binary}" 2>&1)"
  local install_exit=$?

  assert_exit_code "${install_exit}" 0 "rulez copilot install exits 0" || failures=$((failures+1))
  assert_file_exists "${workspace}/.github/hooks/rulez.json" ".github/hooks/rulez.json created" || failures=$((failures+1))
  assert_file_contains "${workspace}/.github/hooks/rulez.json" '"preToolUse"' \
    "rulez.json contains preToolUse hook" || failures=$((failures+1))
  assert_file_contains "${workspace}/.github/hooks/rulez.json" '"copilot hook"' \
    "rulez.json contains copilot hook command" || failures=$((failures+1))

  [[ "${failures}" -eq 0 ]] && return 0 || return 1
}
```

**Key difference from Gemini install scenario:** Gemini uses `--scope project` flag; Copilot `install` has no `--scope` flag (it always writes to `cwd/.github/hooks/rulez.json`). The Gemini scenario passes `--scope project`; Copilot scenario omits it.

### Pattern 5: Scenarios 02-04 — hook-fire, deny, inject

These scenarios follow the Gemini pattern exactly, replacing:
- `setup_gemini_hooks` → `setup_copilot_hooks`
- `invoke_gemini_headless` → `invoke_copilot_headless`
- `require_gemini_cli` → `require_copilot_cli`
- fixture path `fixtures/gemini/` → `fixtures/copilot/`
- The `.claude/hooks.yaml` placement is UNCHANGED — RuleZ always reads from `.claude/hooks.yaml`

### Pattern 6: run.sh addition

Add after the Gemini block (lines 121-130 of current `run.sh`):

```bash
  if [[ "${cli_name}" == "copilot" ]]; then
    if copilot_adapter_check > /dev/null 2>&1; then
      COPILOT_CLI_AVAILABLE=1
    else
      COPILOT_CLI_AVAILABLE=0
      echo "  NOTE: copilot CLI not available — scenarios requiring it will be skipped" >&2
    fi
    export COPILOT_CLI_AVAILABLE
  fi
```

And add the source line near the top:
```bash
source "${E2E_ROOT}/lib/copilot_adapter.sh"
```

### Pattern 7: Fixture YAML files

The Copilot fixture YAMLs are **identical in content** to the Gemini fixtures. They live at `e2e/fixtures/copilot/` to allow future divergence but start as copies.

`hooks-hookfire.yaml`:
```yaml
version: "1.0"
settings:
  log_level: "info"
  fail_open: true
rules:
  - name: e2e-hookfire-log
    description: "E2E test: logs PreToolUse event without blocking"
    matchers:
      tools: ["Bash"]
    actions:
      block: false
```

`hooks-deny.yaml`:
```yaml
version: "1.0"
settings:
  log_level: "info"
  fail_open: false
rules:
  - name: e2e-deny-force-push
    description: "E2E test: blocks git force push"
    matchers:
      tools: ["Bash"]
      command_match: "git push.*--force|git push.*-f"
    actions:
      block: true
```

`hooks-inject.yaml.template`:
```yaml
version: "1.0"
settings:
  log_level: "info"
  fail_open: true
rules:
  - name: e2e-inject-marker
    description: "E2E test: inject command writes marker file"
    matchers:
      tools: ["Bash"]
    actions:
      inject_command: "touch __WORKSPACE__/e2e-inject-fired.marker && echo 'E2E-INJECTED'"
```

### Anti-Patterns to Avoid

- **Using `--scope` flag:** `rulez copilot install` has no `--scope` flag (unlike `rulez gemini install`). Do not pass it.
- **Writing to `.gemini/settings.json`:** Copilot reads from `.github/hooks/*.json`, not `.gemini/`.
- **Asserting on Copilot CLI exit code for deny:** Like Gemini, Copilot CLI may exit 0 even when a hook denies a tool call. Proof of denial is in the audit log, not the CLI exit code.
- **Checking for `GITHUB_TOKEN` in adapter_check:** The Copilot CLI uses persistent login state (via `/login`), not an env var. Unlike Gemini which requires `GEMINI_API_KEY`, Copilot authentication is interactive at setup time. Do not fail the check for a missing env var.
- **Using `--output-format json`:** Copilot CLI does not support this flag (unlike Claude Code which does). Parse output as plain text or regex-match against audit log.

---

## Don't Hand-Roll

| Problem | Don't Build | Use Instead |
|---------|-------------|-------------|
| Hook config structure | Custom JSON builder | Direct heredoc — format is simple and tested in Rust unit tests |
| Headless invocation | Complex process wrapper | Direct `timeout copilot -p ... --allow-all-tools` pattern — matches Gemini's `timeout gemini -p ... --yolo` |
| Result assertions | Custom diff logic | Existing `assert_log_contains`, `assert_file_exists`, `assert_file_contains` from `harness.sh` |
| Availability check | Complex probe | Simple `command -v copilot` check — no API key validation needed |
| Fixture YAML | Custom policy DSL | Reuse Gemini fixture content verbatim (start as copies) |

---

## Common Pitfalls

### Pitfall 1: No `--scope` Flag on Copilot Install
**What goes wrong:** Scenario 01 runs `rulez copilot install --scope project` (copying Gemini pattern) and fails because Copilot install has no `--scope` argument.
**Why it happens:** Gemini install requires `--scope project` to write to project-level settings; Copilot always writes to `cwd/.github/hooks/rulez.json`.
**How to avoid:** Do not pass `--scope` to `rulez copilot install`.
**Confirmed by:** `rulez/src/cli/copilot_install.rs` — `run(binary_path: Option<String>, print: bool)` — no scope parameter.

### Pitfall 2: Hook Config Path Mismatch
**What goes wrong:** `setup_copilot_hooks` writes to `.gemini/settings.json` (copy-paste error from Gemini adapter).
**Why it happens:** Copy-paste from `gemini_adapter.sh`.
**How to avoid:** The config file is `.github/hooks/rulez.json`, not `.gemini/settings.json`. The directory is `.github/hooks/`, created with `mkdir -p`.

### Pitfall 3: Input Format Discrepancy — Official Docs vs RuleZ Adapter
**What goes wrong:** The official GitHub Copilot docs describe the stdin JSON as `{timestamp, cwd, toolName, toolArgs}`, but the RuleZ Copilot adapter (`rulez/src/adapters/copilot.rs`) expects `{session_id, hook_event_name, tool_name, tool_input}`.
**Why it happens:** The RuleZ Copilot adapter uses a normalized internal format rather than the exact Copilot wire format. The `rulez copilot hook` command is the intermediary — it receives whatever Copilot sends, but the **E2E scenarios don't call `rulez copilot hook` directly**. They invoke the live `copilot` CLI which calls `rulez copilot hook` with whatever format it actually uses.
**How to avoid:** The E2E adapter never needs to construct the stdin JSON manually. The live `copilot` binary handles that. Just invoke `copilot -p "..." --allow-all-tools` and let Copilot call `rulez copilot hook` internally. Only the Rust unit tests (which mock stdin) need to worry about the exact format.
**Confidence:** MEDIUM — we cannot confirm without running a live Copilot session whether Copilot sends `session_id`/`hook_event_name` or `timestamp`/`toolName`. The RuleZ adapter uses `#[serde(flatten)] extra: Map<String, Value>` for unknown fields, providing resilience.

### Pitfall 4: Authentication — No COPILOT_API_KEY Analog
**What goes wrong:** `copilot_adapter_check()` checks for `COPILOT_API_KEY` or `GITHUB_TOKEN` and fails in CI even when Copilot is installed and authenticated.
**Why it happens:** Gemini pattern checks for `GEMINI_API_KEY`. Copilot uses OAuth login state, not an API key env var.
**How to avoid:** `copilot_adapter_check()` only checks `command -v copilot`. Authentication failures will surface when `invoke_copilot_headless` actually runs — scenarios 02-04 will fail with a non-zero exit if unauthenticated, which is the correct behavior.

### Pitfall 5: Copilot Exit Code on Tool Denial
**What goes wrong:** Scenario 03 (deny) asserts that `invoke_copilot_headless` returns non-zero when a tool is denied.
**Why it happens:** Copilot CLI may return exit code 0 even when a hook denies a tool, handling the denial internally.
**How to avoid:** Use `|| true` on `invoke_copilot_headless` invocation in scenario 03, exactly as Gemini scenario 03 does. Assert denial via `assert_log_contains "block"` from the audit log.
**Source (MEDIUM):** Gemini scenario 03 comment: "Note: Gemini's exit code may be 0 even when a hook denies, because Gemini handles hook denials internally." Same pattern applies to Copilot.

### Pitfall 6: `postToolUse` Not Registered in hook config
**What goes wrong:** `setup_copilot_hooks` writes a `.github/hooks/rulez.json` that only has `preToolUse` (to avoid redundant hook firings in tests), but `rulez copilot install` registers both `preToolUse` and `postToolUse`.
**How to avoid:** For E2E tests, register only `preToolUse` in the manually-written hook config (as Gemini adapter does with `BeforeTool` only). This reduces noise and double-triggering.

---

## Code Examples

### Complete setup_copilot_hooks implementation
```bash
# Source: rulez/src/cli/copilot_install.rs + docs/COPILOT_CLI_HOOKS.md
setup_copilot_hooks() {
  local workspace="$1"
  local rulez_binary="$2"

  mkdir -p "${workspace}/.github/hooks"

  local abs_rulez
  abs_rulez="$(cd "$(dirname "${rulez_binary}")" && pwd)/$(basename "${rulez_binary}")"

  cat > "${workspace}/.github/hooks/rulez.json" <<EOF
{
  "version": 1,
  "hooks": {
    "preToolUse": [
      {
        "type": "command",
        "bash": "${abs_rulez} copilot hook",
        "powershell": "${abs_rulez} copilot hook",
        "timeoutSec": 10
      }
    ]
  }
}
EOF
  echo "copilot_adapter: wrote .github/hooks/rulez.json"
}
```

### Complete scenario_hook_fire pattern
```bash
# Source: e2e/scenarios/gemini/02-hook-fire.sh (adapted)
scenario_hook_fire() {
  local workspace="$1"
  local rulez_binary="$2"

  require_copilot_cli || return $?

  local failures=0

  setup_copilot_hooks "${workspace}" "${rulez_binary}"

  mkdir -p "${workspace}/.claude"
  cp "${E2E_ROOT}/fixtures/copilot/hooks-hookfire.yaml" "${workspace}/.claude/hooks.yaml"

  local log_file="${HOME}/.claude/logs/rulez.log"
  if [[ -f "${log_file}" ]]; then
    WORKSPACE_LOG_SNAPSHOT="$(wc -l < "${log_file}")"
  else
    WORKSPACE_LOG_SNAPSHOT=0
  fi
  export WORKSPACE_LOG_SNAPSHOT

  invoke_copilot_headless "${workspace}" "Run this bash command: echo hello-e2e-hookfire" 120 || true

  assert_log_contains "e2e-hookfire-log" \
    "audit log contains hookfire rule name" || failures=$((failures + 1))

  [[ "${failures}" -eq 0 ]] && return 0 || return 1
}
```

### run.sh block to add (after Gemini block)
```bash
# Source: e2e/run.sh lines 121-130 (adapted)
  if [[ "${cli_name}" == "copilot" ]]; then
    if copilot_adapter_check > /dev/null 2>&1; then
      COPILOT_CLI_AVAILABLE=1
    else
      COPILOT_CLI_AVAILABLE=0
      echo "  NOTE: copilot CLI not available — scenarios requiring it will be skipped" >&2
    fi
    export COPILOT_CLI_AVAILABLE
  fi
```

---

## State of the Art

| Old Approach | Current Approach | Impact |
|--------------|-----------------|--------|
| Manual hook file per test | Adapter function `setup_copilot_hooks` writes config at test time | Workspace isolation, no real config pollution |
| Global `~/.copilot/hooks/` | Workspace-level `.github/hooks/rulez.json` | Per-test isolation |
| API key env var (Gemini pattern) | Login-state auth (Copilot pattern) | `copilot_adapter_check` does not validate an API key |

---

## Open Questions

1. **Does Copilot CLI actually fire hooks from `.github/hooks/*.json` at the workspace level?**
   - What we know: `rulez copilot install` writes to `cwd/.github/hooks/rulez.json`. The `hooks_dir()` function in `copilot_install.rs` returns `cwd.join(".github").join("hooks")`. Official docs confirm `.github/hooks/` is the hook directory.
   - What's unclear: Whether the `copilot` binary scans the current directory's `.github/hooks/` or only a global location. If it scans from cwd, the workspace-level config used in scenarios 02-04 will work. If it only reads global config, all three live-CLI scenarios will skip or fail.
   - Recommendation: Run `copilot --help` and test a minimal hook locally before committing the E2E implementation. If workspace-level hooks don't fire, fallback is to use a temp global config directory with HOME override.

2. **What is the exact stdin JSON format Copilot sends to hook commands?**
   - What we know: Official docs show `{timestamp, cwd, toolName, toolArgs}` camelCase. RuleZ adapter expects `{session_id, hook_event_name, tool_name, tool_input}`. The adapter uses `#[serde(flatten)] extra` so unknown fields pass through.
   - What's unclear: Whether the live `copilot` binary actually sends `session_id` and `hook_event_name` (which RuleZ requires as non-optional fields per the `CopilotHookInput` struct) or the documented camelCase fields.
   - Recommendation: Add a test wrapper that logs raw stdin to a file before passing to `rulez copilot hook`. If the real Copilot format differs from what the Rust adapter expects, this will surface in scenario 02 (hook-fire) via a failed assertion on the audit log.

3. **Does `copilot -p "..." --allow-all-tools` exit reliably in non-interactive environments?**
   - What we know: The `-p` flag is confirmed for non-interactive mode. `--allow-all-tools` prevents tool-approval prompts. GitHub issues report intermittent silent exits with code 0.
   - What's unclear: Whether timeout-based fallback (exit code 124 → skip/77) is the correct handling, as with Gemini.
   - Recommendation: Use `timeout 120 copilot -p "..." --allow-all-tools` with the same 124→77 skip logic as `invoke_gemini_headless`. If Copilot exits silently without producing output, the audit log will be empty and the scenario will fail with a meaningful message.

---

## Sources

### Primary (HIGH confidence)
- `e2e/lib/gemini_adapter.sh` — Pattern to replicate
- `e2e/lib/claude_adapter.sh` — Original pattern
- `e2e/scenarios/gemini/*.sh` — All four scenario scripts
- `e2e/run.sh` — Harness entry point; where copilot block must be added
- `rulez/src/cli/copilot_install.rs` — Defines `.github/hooks/rulez.json` format and `hooks_dir()` function
- `rulez/src/cli/copilot_hook.rs` — Hook runner stdin/stdout protocol
- `rulez/src/adapters/copilot.rs` — Input parsing, tool name mapping, response translation
- `rulez/tests/copilot_install.rs` — Confirms `.github/hooks/rulez.json` structure
- `rulez/tests/copilot_hook_runner.rs` — Confirms `permissionDecision` response format
- `rulez/tests/copilot_adapter.rs` — Confirms input JSON field names used in unit tests
- `docs/COPILOT_CLI_HOOKS.md` — Internal docs on hook file location and format
- WebFetch `https://docs.github.com/en/copilot/reference/hooks-configuration` — Confirmed `permissionDecision`/`permissionDecisionReason` response format

### Secondary (MEDIUM confidence)
- Perplexity: Confirmed `-p` flag for non-interactive Copilot CLI invocation
- Perplexity: Confirmed `--allow-all-tools` flag to suppress tool-approval prompts
- Perplexity: Confirmed `.github/hooks/*.json` as hook file location

### Tertiary (LOW confidence)
- Perplexity: Suggested `{timestamp, cwd, toolName, toolArgs}` as stdin format — contradicts RuleZ adapter field names; flagged as open question

---

## Metadata

**Confidence breakdown:**
- Standard stack: HIGH — all adapter code, install commands, and test patterns exist in the codebase
- Architecture: HIGH — direct parallel to Gemini; all structural differences identified
- Pitfalls: HIGH — derived from code inspection of copilot_install.rs, copilot_hook.rs, adapter.rs
- Headless invocation flags: MEDIUM — confirmed by official docs via Perplexity/WebFetch but not locally tested
- Hook stdin/stdout wire format at runtime: LOW — open question #2

**Research date:** 2026-02-23
**Valid until:** 2026-03-23 (stable pattern; Copilot CLI hook format unlikely to change in 30 days)
