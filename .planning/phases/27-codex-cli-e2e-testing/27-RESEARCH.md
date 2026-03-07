# Phase 27: Codex CLI E2E Testing - Research

**Researched:** 2026-03-06
**Domain:** OpenAI Codex CLI — binary detection, headless invocation, hook support status, workspace configuration
**Confidence:** MEDIUM (Codex CLI is rapidly evolving; findings verified against official docs and GitHub)

---

<user_constraints>
## User Constraints (from CONTEXT.md)

### Locked Decisions
- Codex CLI is **optional** in CI — not required on runners
- If Codex CLI is missing, all scenarios exit 77 (skip) — no CI failure
- Full adapter with CLI detection (`codex_adapter_check()` looks for binary, reports version)
- Consistent with other adapters (claude, gemini, copilot, opencode)
- `require_codex_cli()` returns 77 if unavailable
- `codex_adapter.sh` in `e2e/lib/`
- Create all 4 standard scenarios matching the harness pattern:
  1. `01-install.sh` — runs if CLI found (tests adapter detection, workspace setup)
  2. `02-hook-fire.sh` — skips with clear "no hook support" message
  3. `03-deny.sh` — skips with clear "no hook support" message
  4. `04-inject.sh` — skips with clear "no hook support" message
- Scenarios ready to enable when/if Codex adds hooks support
- Fixtures in `e2e/fixtures/codex/`
- **No Rust changes in this phase** — no `rulez codex install` or `rulez codex hook` subcommand
- Add codex availability check block in `run.sh` (matching pattern for other CLIs)
- `CODEX_CLI_AVAILABLE` environment variable exported for scenarios

### Claude's Discretion
- Exact Codex CLI binary name and detection method (research needed — `codex` vs `openai-codex` vs other)
- Headless invocation flags for Codex CLI
- Skip message wording for hook-dependent scenarios
- Fixture file contents

### Deferred Ideas (OUT OF SCOPE)
- `rulez codex install` / `rulez codex hook` CLI subcommands — separate phase when Codex adds hook support
- Enabling hook-fire, deny, inject scenarios — when Codex CLI adds hooks
- Non-hook Codex invocation testing (direct tool use without policy enforcement)
</user_constraints>

---

## Summary

Phase 27 adds Codex CLI adapter + scenarios to the existing multi-CLI E2E harness. The pattern is identical to Phases 23-26 (Claude Code, Gemini, Copilot, OpenCode) — new adapter file, four scenario scripts, and a fixture directory. The critical difference: Codex CLI has no hooks system (as of March 2026), so scenarios 02-04 skip unconditionally with a clear message.

The binary is named `codex`, installed via `npm install -g @openai/codex`. Version detection uses `codex --version`. Headless invocation uses `codex exec "<prompt>" --ask-for-approval never --json`. The CLI requires `OPENAI_API_KEY` (or `CODEX_API_KEY` for exec) for live invocation. Since hooks are absent, the install scenario (01) validates only adapter detection and binary presence — no `rulez codex install` command exists yet (deferred).

The install scenario must not call a non-existent `rulez codex install` subcommand. Instead, it validates that the adapter finds the binary and that a placeholder `.codex/config.toml` workspace file can be written if desired. The most defensible approach: the install scenario does a structural check only (adapter check passes, binary detected, version string captured).

**Primary recommendation:** Follow the opencode_adapter.sh template exactly, except `setup_codex_hooks()` is a stub that prints a warning (no hook config to write), and scenarios 02-04 return 77 immediately after sourcing the adapter with a clear skip message.

---

## Standard Stack

### Core — Already in Project
| Component | Version | Purpose | Status |
|-----------|---------|---------|--------|
| `e2e/lib/harness.sh` | — | Workspace management, assertions, `run_scenario` | Reuse as-is |
| `e2e/lib/reporting.sh` | — | JUnit XML, markdown, ASCII table | Reuse as-is |
| `e2e/run.sh` | — | Auto-discovers scenarios, CLI availability blocks | Add codex block |

### New Files Required
| File | Purpose |
|------|---------|
| `e2e/lib/codex_adapter.sh` | Adapter: check, require, setup_hooks stub, invoke_headless |
| `e2e/scenarios/codex/01-install.sh` | Adapter detection + binary presence check |
| `e2e/scenarios/codex/02-hook-fire.sh` | Skip unconditionally — no hook support |
| `e2e/scenarios/codex/03-deny.sh` | Skip unconditionally — no hook support |
| `e2e/scenarios/codex/04-inject.sh` | Skip unconditionally — no hook support |
| `e2e/fixtures/codex/` | Directory with placeholder YAML files (for future use) |

---

## Architecture Patterns

### Recommended Project Structure (new files only)
```
e2e/
├── lib/
│   └── codex_adapter.sh          # New adapter
├── scenarios/
│   └── codex/
│       ├── 01-install.sh         # Structural check — always runs if binary found
│       ├── 02-hook-fire.sh       # Skip: no hook support
│       ├── 03-deny.sh            # Skip: no hook support
│       └── 04-inject.sh          # Skip: no hook support
└── fixtures/
    └── codex/
        ├── hooks-hookfire.yaml        # For future use (same format as opencode fixtures)
        ├── hooks-deny.yaml            # For future use
        └── hooks-inject.yaml.template # For future use
```

### Pattern 1: Codex Adapter Structure
Follows `opencode_adapter.sh` verbatim except:
- `setup_codex_hooks()` is a no-op stub (no hooks system)
- `invoke_codex_headless()` uses `codex exec` instead of `opencode run`

```bash
# Source: e2e/lib/opencode_adapter.sh (template)
CODEX_CLI_NAME="codex"
export CODEX_CLI_NAME

codex_adapter_check() {
  if ! command -v codex > /dev/null 2>&1; then
    echo "ERROR: 'codex' CLI not found in PATH." >&2
    echo "  Install Codex: npm install -g @openai/codex" >&2
    echo "  https://github.com/openai/codex" >&2
    return 1
  fi

  local version
  version="$(codex --version 2>&1 || true)"
  echo "codex_adapter: found codex CLI: ${version}"
  return 0
}

require_codex_cli() {
  if [[ "${CODEX_CLI_AVAILABLE:-0}" -eq 1 ]]; then
    return 0
  fi
  echo "  [skip] codex CLI not available for headless invocation" >&2
  return 77
}
```

### Pattern 2: Headless Invocation
**Verified command (MEDIUM confidence — from official docs):**

```bash
# Source: https://developers.openai.com/codex/noninteractive/
invoke_codex_headless() {
  local workspace="$1"
  local prompt="$2"
  local timeout_secs="${3:-120}"

  local output_file="${workspace}/codex-output.txt"
  local exit_code=0

  (
    cd "${workspace}" && \
    NO_COLOR=true timeout "${timeout_secs}" codex exec \
      "${prompt}" \
      --ask-for-approval never \
      --json 2>&1
  ) | tee "${output_file}" || exit_code="${PIPESTATUS[0]}"

  if [[ "${exit_code}" -eq 124 ]]; then
    echo "codex_adapter: codex timed out after ${timeout_secs}s — treating as skip" >&2
    return 77
  fi

  echo "codex_adapter: codex exited with code ${exit_code}"
  return "${exit_code}"
}
```

**Key flags (MEDIUM confidence):**
- `codex exec "<prompt>"` — non-interactive subcommand (documented official)
- `--ask-for-approval never` — skips all approval prompts (equivalent to --full-auto but explicit)
- `--json` — outputs newline-delimited JSON events for machine-readable output
- `--ephemeral` — prevents persisting session files to disk (good for CI isolation)

**Alternative flags to consider:**
- `--full-auto` sets `--ask-for-approval on-request` + `--sandbox workspace-write` — may be insufficient for CI
- `--dangerously-bypass-approvals-and-sandbox` (aka `--yolo`) — full bypass, use only in isolated runners

### Pattern 3: No-Hook Skip Scenario
Scenarios 02-04 are stubs that exit 77 immediately:

```bash
# Source: Project pattern — established in Phase 27 design
scenario_hook_fire() {
  local workspace="$1"
  local rulez_binary="$2"

  echo "  [skip] Codex CLI does not support hooks — scenario skipped" >&2
  echo "  [skip] Enable this scenario when Codex CLI adds hook support." >&2
  return 77
}
```

### Pattern 4: Install Scenario Without rulez subcommand
Since there is no `rulez codex install` command (deferred), the install scenario validates:
1. Adapter detection passes (binary found, version reported)
2. Workspace `.codex/` directory can be created
3. A minimal `config.toml` can be written to the workspace

This is a lighter structural check than other CLIs' install scenarios. No `rulez` subcommand is invoked.

```bash
scenario_install() {
  local workspace="$1"
  local rulez_binary="$2"

  local failures=0

  # Validate adapter check passes (binary detection)
  if ! codex_adapter_check > /dev/null 2>&1; then
    echo "  [skip] codex CLI not available" >&2
    return 77
  fi

  # Create a minimal .codex workspace directory (proves workspace setup works)
  mkdir -p "${workspace}/.codex"
  cat > "${workspace}/.codex/config.toml" <<EOF
# Codex CLI workspace config — created by RuleZ E2E test harness
model = "o4-mini"
approval_policy = "never"
EOF

  # Assert the config file was created
  assert_file_exists "${workspace}/.codex/config.toml" \
    ".codex/config.toml created" || failures=$((failures + 1))

  # Assert config contains expected content
  assert_file_contains "${workspace}/.codex/config.toml" "approval_policy" \
    ".codex/config.toml contains approval_policy" || failures=$((failures + 1))

  if [[ "${failures}" -eq 0 ]]; then
    return 0
  else
    return 1
  fi
}
```

### Pattern 5: run.sh Availability Block
Add after the opencode block, following exact pattern:

```bash
if [[ "${cli_name}" == "codex" ]]; then
  if codex_adapter_check > /dev/null 2>&1; then
    CODEX_CLI_AVAILABLE=1
  else
    CODEX_CLI_AVAILABLE=0
    echo "  NOTE: codex CLI not available — scenarios requiring it will be skipped" >&2
  fi
  export CODEX_CLI_AVAILABLE
fi
```

And source the adapter at the top of run.sh:
```bash
# shellcheck source=lib/codex_adapter.sh
source "${E2E_ROOT}/lib/codex_adapter.sh"
```

### Anti-Patterns to Avoid
- **Calling `rulez codex install`:** No such subcommand exists — Rust changes are deferred.
- **Using `codex run` instead of `codex exec`:** OpenCode uses `run`, Codex uses `exec` — these are different CLIs.
- **Assuming OPENAI_API_KEY is not needed:** Codex exec requires `OPENAI_API_KEY` or `CODEX_API_KEY`. The adapter check passes if binary is found, but live invocation (scenarios 02-04) would fail without a key — moot since those scenarios skip anyway.
- **Assuming hook scenarios should fail (not skip):** Codex has no hooks — return 77 (skip), not 1 (fail).
- **Using `codex --help` for version:** Use `codex --version` — documented as the version command.

---

## Don't Hand-Roll

| Problem | Don't Build | Use Instead | Why |
|---------|-------------|-------------|-----|
| Workspace isolation | Custom temp dir logic | `setup_workspace()` from harness.sh | Already handles cleanup, RUN_DIR structure |
| Scenario result recording | Custom pass/fail counters | `run_scenario()` + `record_result()` from harness.sh | JUnit XML, markdown, ASCII table output handled |
| Log assertion | `grep` on log file directly | `assert_log_contains()` from harness.sh | Handles WORKSPACE_LOG_SNAPSHOT scoping |
| File assertions | Raw `[[ -f ... ]]` checks | `assert_file_exists()`, `assert_file_contains()` | Consistent failure reporting |
| Exit code check | Manual `$?` comparison | `assert_exit_code()` from harness.sh | Consistent failure formatting |

**Key insight:** All assertion and workspace primitives already exist in harness.sh. The adapter is thin glue between harness primitives and the Codex CLI binary.

---

## Common Pitfalls

### Pitfall 1: Binary Name Confusion
**What goes wrong:** Searching for `openai-codex` or `codex-cli` in PATH instead of `codex`.
**Why it happens:** The npm package is `@openai/codex` but the installed binary is `codex`.
**How to avoid:** Use `command -v codex` in `codex_adapter_check()`.
**Warning signs:** Adapter check always returns 1 even when Codex is installed.

### Pitfall 2: Using `codex run` Instead of `codex exec`
**What goes wrong:** `codex run` is OpenCode's headless command, not Codex's. Codex uses `codex exec`.
**Why it happens:** OpenCode and Codex both have a non-interactive mode with different subcommand names.
**How to avoid:** Always use `codex exec "<prompt>"` for Codex.
**Warning signs:** `command not found: run` or unexpected interactive TUI launching.

### Pitfall 3: Missing OPENAI_API_KEY in Adapter Check
**What goes wrong:** `codex_adapter_check()` passes (binary found), but live invocation fails immediately due to missing API key.
**Why it happens:** Binary presence check and API key check are separate concerns. For Codex, no login flow is baked into the binary check like Copilot's GitHub OAuth.
**How to avoid:** For Phase 27, this is moot because scenarios 02-04 (which invoke Codex headlessly) all skip unconditionally. The install scenario (01) does not invoke Codex headlessly. If future phases enable live invocation, add `OPENAI_API_KEY` check to `codex_adapter_check()`.
**Warning signs:** Binary found but scenario fails with "API key not set" immediately.

### Pitfall 4: Hooks Status Confusion
**What goes wrong:** Assuming Codex has a hooks system because other CLIs do.
**Why it happens:** Feature parity assumption; Codex has a `notify` config key that triggers on `agent-turn-complete` but this is NOT a PreToolUse/BeforeTool hooks system.
**How to avoid:** Scenarios 02-04 skip unconditionally with an explicit message. `setup_codex_hooks()` is a stub.
**Warning signs:** Trying to write `.codex/settings.json` or similar hook config — no such file/format exists.

### Pitfall 5: Codex Configuration Directory
**What goes wrong:** Writing Codex config to wrong path (`.codex/` is project-level, `~/.codex/` is user-level).
**Why it happens:** Different CLIs use different config locations — OpenCode uses `.opencode/`, Copilot uses `.github/hooks/`, Codex uses `.codex/`.
**How to avoid:** Project-scoped Codex config lives at `.codex/config.toml`. The install scenario creates `${workspace}/.codex/config.toml`.
**Warning signs:** Config file created but Codex CLI ignores it (wrong directory).

### Pitfall 6: Codex Requires Git Repo by Default
**What goes wrong:** `codex exec` may fail in a non-git workspace with "not a git repository" error.
**Why it happens:** Codex CLI uses git as a safety mechanism for workspace context.
**How to avoid:** Add `--skip-git-repo-check` flag to `invoke_codex_headless()` OR initialize a git repo in `setup_workspace()` for Codex scenarios. Flag approach is simpler.
**Warning signs:** `codex exec` exits non-zero with git-related error in fresh workspace.

---

## Code Examples

### codex_adapter.sh — Full Template

```bash
#!/usr/bin/env bash
# codex_adapter.sh — Codex CLI detection helper and workspace config writer
#
# Codex CLI (openai/codex) does NOT support a hooks system as of March 2026.
# Scenarios 02-04 skip unconditionally. This adapter handles:
#   - Binary detection (codex_adapter_check)
#   - Skip guard (require_codex_cli)
#   - Workspace config stub (setup_codex_hooks — no-op, for future use)
#   - Headless invocation (invoke_codex_headless — for future use when hooks land)

CODEX_CLI_NAME="codex"
export CODEX_CLI_NAME

codex_adapter_check() {
  if ! command -v codex > /dev/null 2>&1; then
    echo "ERROR: 'codex' CLI not found in PATH." >&2
    echo "  Install Codex: npm install -g @openai/codex" >&2
    echo "  https://github.com/openai/codex" >&2
    return 1
  fi

  local version
  version="$(codex --version 2>&1 || true)"
  echo "codex_adapter: found codex CLI: ${version}"
  return 0
}

require_codex_cli() {
  if [[ "${CODEX_CLI_AVAILABLE:-0}" -eq 1 ]]; then
    return 0
  fi
  echo "  [skip] codex CLI not available for headless invocation" >&2
  return 77
}

# setup_codex_hooks workspace rulez_binary
# Stub — Codex CLI has no hooks system as of March 2026.
# Writes a minimal .codex/config.toml for workspace isolation only.
setup_codex_hooks() {
  local workspace="$1"
  local rulez_binary="$2"   # unused until Codex adds hooks

  mkdir -p "${workspace}/.codex"

  cat > "${workspace}/.codex/config.toml" <<EOF
# Codex CLI workspace config — created by RuleZ E2E harness
# NOTE: Codex CLI does not support hooks as of March 2026.
# This file exists for workspace isolation only.
model = "o4-mini"
approval_policy = "never"
EOF

  echo "codex_adapter: wrote minimal .codex/config.toml (no hook support)" >&2
}

# invoke_codex_headless workspace prompt timeout_secs
# Runs Codex CLI non-interactively (codex exec).
# For future use when Codex adds hooks support.
invoke_codex_headless() {
  local workspace="$1"
  local prompt="$2"
  local timeout_secs="${3:-120}"

  local output_file="${workspace}/codex-output.txt"
  local exit_code=0

  (
    cd "${workspace}" && \
    NO_COLOR=true timeout "${timeout_secs}" codex exec \
      "${prompt}" \
      --ask-for-approval never \
      --skip-git-repo-check \
      --json 2>&1
  ) | tee "${output_file}" || exit_code="${PIPESTATUS[0]}"

  if [[ "${exit_code}" -eq 124 ]]; then
    echo "codex_adapter: codex timed out after ${timeout_secs}s — treating as skip" >&2
    return 77
  fi

  echo "codex_adapter: codex exited with code ${exit_code}"
  return "${exit_code}"
}
```

### 01-install.sh — Structural Check Only

```bash
#!/usr/bin/env bash
# 01-install.sh — E2E scenario: codex adapter detects binary and sets up workspace
#
# Scenario function: scenario_install(workspace, rulez_binary)
# Does NOT invoke Codex CLI headlessly — validates adapter detection and
# workspace directory setup only. No `rulez codex install` command exists yet.

source "${E2E_ROOT}/lib/codex_adapter.sh"

scenario_install() {
  local workspace="$1"
  local rulez_binary="$2"

  local failures=0

  # Validate adapter check passes (binary detection + version)
  if ! codex_adapter_check > /dev/null 2>&1; then
    echo "  [skip] codex CLI not available" >&2
    return 77
  fi

  # Set up minimal workspace config
  setup_codex_hooks "${workspace}" "${rulez_binary}"

  # Assert .codex/config.toml was created
  assert_file_exists "${workspace}/.codex/config.toml" \
    ".codex/config.toml created" || failures=$((failures + 1))

  # Assert config.toml contains expected key
  assert_file_contains "${workspace}/.codex/config.toml" "approval_policy" \
    ".codex/config.toml contains approval_policy" || failures=$((failures + 1))

  if [[ "${failures}" -eq 0 ]]; then
    return 0
  else
    return 1
  fi
}
```

### 02-hook-fire.sh, 03-deny.sh, 04-inject.sh — Skip Stubs

```bash
#!/usr/bin/env bash
# 02-hook-fire.sh — E2E scenario: [SKIP] Codex CLI has no hooks system
#
# Scenario function: scenario_hook_fire(workspace, rulez_binary)
# Skips unconditionally — Codex CLI does not support PreToolUse/BeforeTool hooks.
# Enable this scenario when Codex CLI adds a hooks system.

source "${E2E_ROOT}/lib/codex_adapter.sh"

scenario_hook_fire() {
  local workspace="$1"
  local rulez_binary="$2"

  echo "  [skip] Codex CLI does not support hooks (no PreToolUse/BeforeTool equivalent)" >&2
  echo "  [skip] Enable this scenario when Codex CLI adds hook support." >&2
  return 77
}
```

(03-deny.sh and 04-inject.sh follow the same pattern with `scenario_deny` and `scenario_inject` function names.)

### run.sh — Codex Availability Block to Add

```bash
# Add after the opencode block, before scenario execution loop
source "${E2E_ROOT}/lib/codex_adapter.sh"   # Add to top-of-file sources section

# In the cli_name loop:
if [[ "${cli_name}" == "codex" ]]; then
  if codex_adapter_check > /dev/null 2>&1; then
    CODEX_CLI_AVAILABLE=1
  else
    CODEX_CLI_AVAILABLE=0
    echo "  NOTE: codex CLI not available — scenarios requiring it will be skipped" >&2
  fi
  export CODEX_CLI_AVAILABLE
fi
```

### Fixture Files (for future use)

The fixture files in `e2e/fixtures/codex/` should be identical to the opencode fixtures — same YAML format, same rule names. This lets them be enabled immediately if Codex adds hooks:

```yaml
# hooks-hookfire.yaml — same as opencode fixture
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

---

## State of the Art

| CLI | Hook System | Headless Command | Approval Flag |
|-----|-------------|-----------------|---------------|
| Claude Code | PreToolUse hooks in `.claude/settings.json` | `claude -p` | `--dangerously-skip-permissions` |
| Gemini CLI | BeforeTool hooks in `.gemini/settings.json` | `gemini --yolo` | `--yolo` |
| Copilot | preToolUse hooks in `.github/hooks/*.json` | `copilot -p` | `--allow-all-tools` |
| OpenCode | tool.execute.before in `.opencode/settings.json` | `opencode run` | `--format json` |
| **Codex** | **None (hooks in development, Nov 2025)** | **`codex exec`** | **`--ask-for-approval never`** |

**Deprecated/outdated:**
- Codex CLI `notify` config key: triggers only on `agent-turn-complete`, NOT a PreToolUse-equivalent hooks system. Do not confuse with a hooks system.

---

## Open Questions

1. **`--skip-git-repo-check` flag existence**
   - What we know: Codex exec may require a git repo by default; flag was documented as existing
   - What's unclear: Whether `--skip-git-repo-check` is the exact flag name in the current version
   - Recommendation: Test with `codex exec --help` if Codex is available. If flag doesn't exist, initialize a git repo in the workspace instead: `git init "${workspace}"` before invoking.

2. **`codex --version` output format**
   - What we know: `codex --version` is the documented version command; there is a known bug where it may report stale version strings
   - What's unclear: Whether the output is a bare version string or has a prefix (affects version display in adapter check output)
   - Recommendation: Use `version="$(codex --version 2>&1 || true)"` and print as-is — same pattern as other adapters.

3. **API key for install scenario**
   - What we know: The install scenario (01) does not invoke Codex headlessly — only writes workspace files
   - What's unclear: Whether `codex_adapter_check()` itself requires `OPENAI_API_KEY` to be set
   - Recommendation: Do NOT add API key check to `codex_adapter_check()` — binary presence is sufficient for the install scenario. Future phases that enable live invocation can add the key check then.

4. **Hooks development status**
   - What we know: A maintainer confirmed hooks were in development as of November 2025; no release date given
   - What's unclear: Whether hooks shipped between November 2025 and March 2026
   - Recommendation: Check `codex --help` or the official changelog if Codex is available. If hooks landed, Phase 27 scope may expand — but locked CONTEXT.md decisions say to skip, so proceed with skip pattern.

---

## Sources

### Primary (HIGH confidence)
- Official Codex non-interactive docs: https://developers.openai.com/codex/noninteractive/
- Official Codex CLI reference: https://developers.openai.com/codex/cli/reference/
- Official Codex config reference: https://developers.openai.com/codex/config-reference/
- DeepWiki Codex installation: https://deepwiki.com/openai/codex/1.1-installation-and-setup
- Existing adapters in `e2e/lib/` (opencode_adapter.sh, claude_adapter.sh, copilot_adapter.sh)

### Secondary (MEDIUM confidence)
- GitHub discussion on hooks: https://github.com/openai/codex/discussions/2150 — confirms no hooks system, hooks in development as of November 2025
- npm package page: https://www.npmjs.com/package/@openai/codex — confirms binary name `codex`, install via `npm install -g @openai/codex`
- WebSearch for binary name: Multiple sources confirm `codex` is the installed binary name

### Tertiary (LOW confidence)
- `--skip-git-repo-check` flag existence — referenced in docs but exact flag name not independently verified in current release

---

## Metadata

**Confidence breakdown:**
- Binary name (`codex`): HIGH — confirmed by official docs and DeepWiki
- Headless command (`codex exec`): HIGH — confirmed by official non-interactive docs
- `--ask-for-approval never` flag: MEDIUM — confirmed in CLI reference but version currency unknown
- `--json` flag: MEDIUM — confirmed in CLI reference
- `--skip-git-repo-check` flag: LOW — referenced but exact name unverified
- No hooks system: HIGH — confirmed by GitHub discussion with maintainer response
- Config at `.codex/config.toml`: HIGH — confirmed by official config reference

**Research date:** 2026-03-06
**Valid until:** 2026-04-06 (Codex is fast-moving; verify headless flags before implementing)
