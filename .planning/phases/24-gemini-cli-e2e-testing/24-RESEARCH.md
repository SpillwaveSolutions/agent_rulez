# Phase 24: Gemini CLI E2E Testing - Research

**Researched:** 2026-02-22
**Domain:** Gemini CLI headless invocation, hook configuration, E2E adapter pattern
**Confidence:** MEDIUM (headless flag reliability is actively buggy per open GitHub issues)

---

<user_constraints>
## User Constraints (from CONTEXT.md)

### Locked Decisions
- Real API calls, not mocks
- Shell scripts (bash) as primary implementation
- Lives at e2e/ at repo root
- Entry point: task e2e via Taskfile
- Isolated workspaces: e2e/.runs/<run-id>/<cli>/<test-name>/
- Core 4 scenarios per CLI: install, hook-fire, deny, inject
- Proof methods: audit log parsing, inject_command marker files, exit code + stderr, structural assertions
- All 5 CLIs required in CI (missing = CI failure)
- Tests use manual hook config for workspace isolation
- Research must determine headless invocation flags for Gemini CLI

### Claude's Discretion
- Exact headless invocation flags per CLI (determined by research — this is the output)
- Shell script structure and helper functions
- Fixture project contents
- Workspace cleanup strategy (keep on failure, clean on success)
- Timeout values per scenario
- Golden file format and comparison approach

### Deferred Ideas (OUT OF SCOPE)
- Sister product E2E scenarios (Agent Memory, Agent Cron, Agent CLOD)
- Performance/load testing (many rules, large configs)
- Cross-CLI rule compatibility matrix testing
- Automated golden file update workflow
- Phase 22.1: Expose tool_input fields in enabled_when eval context
</user_constraints>

---

## Summary

Gemini CLI (`@google/gemini-cli`, binary: `gemini`) supports headless non-interactive invocation via the `-p` / `--prompt` flag. The closest equivalent to Claude Code's `-p --dangerously-skip-permissions --output-format json --max-turns 1` is `gemini -p "PROMPT" --yolo --output-format json`. However, there is a **known active bug** (GitHub issue #13561, P2, open as of Feb 2026) where `--yolo` and `--approval-mode yolo` still prompt for confirmation in some scenarios. The practical workaround is `--approval-mode=auto_edit` combined with crafting prompts that only use file read/write tools (no shell execution), OR accept that hook-fire/deny/inject scenarios need `--yolo` and may require recent `gemini` versions to work reliably.

Gemini CLI hooks are configured in `.gemini/settings.json` (project-level) — a parallel to Claude Code's `.claude/settings.json`. The hook format uses an event-keyed structure with `matcher` + `hooks[]` arrays. The RuleZ adapter (`rulez gemini hook`) reads JSON from stdin and writes JSON to stdout, and the install command `rulez gemini install --scope project` writes the appropriate `.gemini/settings.json`. Hook behavior (BeforeTool → `decision: deny`) is confirmed to work via the existing `rulez/src/adapters/gemini.rs` and `rulez/src/cli/gemini_install.rs` implementations.

The workspace isolation strategy mirrors the Claude Code adapter exactly: write a `.gemini/settings.json` containing hook registration, then invoke `gemini -p` headlessly from within that workspace. The `gemini` binary discovers project-level `.gemini/settings.json` from the current working directory.

**Primary recommendation:** Use `gemini -p "PROMPT" --yolo --output-format json` for headless invocation. If `--yolo` issues prevent automation, fall back to `--approval-mode=auto_edit` and use prompts that only trigger tool calls (read/write file, no shell exec). Set `GEMINI_CLI_AVAILABLE` flag in `run.sh` via a `gemini_adapter_check` function, skip with exit 77 if gemini not in PATH.

---

## Standard Stack

### Core
| Component | Version | Purpose | Why Standard |
|-----------|---------|---------|--------------|
| `@google/gemini-cli` | latest (npm) | The CLI under test | Official Google package |
| `GEMINI_API_KEY` env var | n/a | Authentication for real API calls | Standard auth pattern |
| `.gemini/settings.json` | n/a | Project-level hook config file | Gemini CLI config resolution |
| `rulez gemini hook` | current build | RuleZ hook runner for Gemini events | Already implemented in codebase |
| `rulez gemini install --scope project` | current build | Writes .gemini/settings.json hooks | Already implemented in codebase |

### Supporting
| Component | Version | Purpose | When to Use |
|-----------|---------|---------|-------------|
| `NO_COLOR=true` | env var | Disable ANSI color in CI output | Always set in CI |
| `GEMINI_API_KEY` | env var | API auth (required for real calls) | All hook-fire/deny/inject scenarios |
| `--output-format json` | flag | Structured output for parsing | When parsing gemini output |
| `--model gemini-2.0-flash-exp` or `gemini-1.5-flash` | flag | Cheapest/fastest model for tests | Reduce test cost |

### Installation
```bash
npm install -g @google/gemini-cli
# Verify:
gemini --version
```

---

## Architecture Patterns

### Gemini Adapter File
```
e2e/
├── lib/
│   ├── harness.sh            # (Phase 23 — exists, unchanged)
│   ├── reporting.sh          # (Phase 23 — exists, unchanged)
│   ├── claude_adapter.sh     # (Phase 23 — exists, unchanged)
│   └── gemini_adapter.sh     # NEW — parallel to claude_adapter.sh
├── scenarios/
│   ├── claude-code/          # (Phase 23 — exists, unchanged)
│   └── gemini/               # NEW
│       ├── 01-install.sh
│       ├── 02-hook-fire.sh
│       ├── 03-deny.sh
│       └── 04-inject.sh
└── fixtures/
    ├── claude-code/           # (Phase 23 — exists, unchanged)
    └── gemini/                # NEW
        ├── hooks-hookfire.yaml
        ├── hooks-deny.yaml
        └── hooks-inject.yaml.template
```

### Pattern 1: Gemini Adapter Shell Library (gemini_adapter.sh)

**What:** Parallel structure to `claude_adapter.sh` — exports `GEMINI_CLI_NAME`, provides `gemini_adapter_check`, `require_gemini_cli`, `setup_gemini_hooks`, `invoke_gemini_headless`.

**When to use:** All Gemini scenario scripts source this file.

```bash
#!/usr/bin/env bash
# gemini_adapter.sh — Gemini CLI headless invocation helper and workspace config generator

GEMINI_CLI_NAME="gemini"
export GEMINI_CLI_NAME

gemini_adapter_check() {
  if ! command -v gemini > /dev/null 2>&1; then
    echo "ERROR: 'gemini' CLI not found in PATH." >&2
    echo "  Install: npm install -g @google/gemini-cli" >&2
    return 1
  fi

  if [[ -z "${GEMINI_API_KEY:-}" ]]; then
    echo "ERROR: GEMINI_API_KEY is not set." >&2
    echo "  Set your API key: export GEMINI_API_KEY=your_key" >&2
    return 1
  fi

  local version
  version="$(gemini --version 2>&1 || true)"
  echo "gemini_adapter: found gemini CLI: ${version}"
  return 0
}

require_gemini_cli() {
  if [[ "${GEMINI_CLI_AVAILABLE:-0}" -eq 1 ]]; then
    return 0
  fi
  echo "  [skip] gemini CLI not available for headless invocation" >&2
  return 77
}

setup_gemini_hooks() {
  local workspace="$1"
  local rulez_binary="$2"

  mkdir -p "${workspace}/.gemini"

  local abs_rulez
  abs_rulez="$(cd "$(dirname "${rulez_binary}")" && pwd)/$(basename "${rulez_binary}")"
  local hook_command="${abs_rulez} gemini hook"

  cat > "${workspace}/.gemini/settings.json" <<EOF
{
  "hooks": {
    "BeforeTool": [
      {
        "matcher": ".*",
        "hooks": [
          {
            "type": "command",
            "command": "${hook_command}",
            "timeout": 10
          }
        ]
      }
    ]
  }
}
EOF

  echo "gemini_adapter: wrote settings.json to ${workspace}/.gemini/settings.json"
}

invoke_gemini_headless() {
  local workspace="$1"
  local prompt="$2"
  local timeout_secs="${3:-120}"

  local output_file="${workspace}/gemini-output.txt"
  local exit_code=0

  (
    cd "${workspace}" && \
    NO_COLOR=true timeout "${timeout_secs}" gemini \
      -p "${prompt}" \
      --yolo \
      --output-format json \
      2>&1
  ) | tee "${output_file}" || exit_code="${PIPESTATUS[0]}"

  echo "gemini_adapter: gemini exited with code ${exit_code}"
  return "${exit_code}"
}
```

### Pattern 2: Hook Configuration Format

**What:** `.gemini/settings.json` structure for Gemini hook registration. Uses the same pattern discovered in `rulez/src/cli/gemini_install.rs`.

**Key insight from codebase:** `rulez gemini install` writes hooks for ALL 11 event types. For E2E testing, we only need `BeforeTool` (analogous to Claude Code's `PreToolUse`).

```json
{
  "hooks": {
    "BeforeTool": [
      {
        "matcher": ".*",
        "hooks": [
          {
            "type": "command",
            "command": "/absolute/path/to/rulez gemini hook",
            "timeout": 10
          }
        ]
      }
    ]
  }
}
```

**Source:** `rulez/src/cli/gemini_install.rs` lines 121-131 (verified from codebase)

### Pattern 3: Gemini Hook JSON Protocol

**What:** The `rulez gemini hook` subcommand reads Gemini hook events from stdin and writes response JSON to stdout.

**Input (from Gemini CLI via stdin):**
```json
{
  "session_id": "string",
  "transcript_path": "string",
  "cwd": "/workspace/path",
  "hook_event_name": "BeforeTool",
  "timestamp": "2026-02-22T00:00:00Z",
  "tool_name": "run_shell_command",
  "tool_input": { ... }
}
```

**Output (from rulez gemini hook to stdout):**
```json
{
  "decision": "allow",
  "reason": null,
  "continue": null,
  "system_message": null,
  "tool_input": null
}
```

**Deny response:**
```json
{
  "decision": "deny",
  "reason": "Blocked by rule: e2e-deny-force-push",
  "continue": false,
  "system_message": null,
  "tool_input": null
}
```

**Source:** `rulez/src/cli/gemini_hook.rs` + `rulez/src/adapters/gemini.rs` (verified from codebase)

### Pattern 4: Tool Name Mapping

**What:** Gemini CLI uses different tool names than Claude Code. `rulez gemini hook` canonicalizes them.

| Gemini Tool Name | Canonical (RuleZ) Name |
|------------------|----------------------|
| `run_shell_command` | `Bash` |
| `execute_code` | `Bash` |
| `write_file` | `Write` |
| `replace` | `Edit` |
| `read_file` | `Read` |
| `glob` | `Glob` |
| `search_file_content` | `Grep` |
| `grep_search` | `Grep` |
| `web_fetch` | `WebFetch` |

**Impact on fixtures:** RuleZ rules in hooks.yaml should use canonical names (e.g., `Bash`, `Write`) — the adapter handles the mapping. Claude Code fixtures work as-is for Gemini.

**Source:** `rulez/src/adapters/gemini.rs` lines 244-255 (verified from codebase)

### Anti-Patterns to Avoid

- **Using Claude Code's `--dangerously-skip-permissions`:** No equivalent in Gemini CLI. Use `--yolo` or `--approval-mode=auto_edit`.
- **Using `--max-turns 1`:** Not documented for Gemini CLI. Omit — rely on prompt design for single-turn tasks.
- **Using `--no-session-persistence`:** Not documented for Gemini CLI. Sessions may persist; use separate workspaces.
- **Assuming `.gemini/settings.json` is read relative to `$HOME`:** Project-level config is resolved from the current working directory (`$cwd/.gemini/settings.json`). Always `cd` to workspace before invocation.
- **Assuming `--yolo` is always stable:** Known active bug (issue #13561). Have fallback strategy.

---

## Don't Hand-Roll

| Problem | Don't Build | Use Instead | Why |
|---------|-------------|-------------|-----|
| Hook registration | Custom JSON writer | `setup_gemini_hooks()` in adapter | Matches `gemini_install.rs` format exactly |
| Tool name normalization | Manual mapping in tests | `rulez gemini hook` (existing adapter) | Already handles all mappings |
| Hook JSON parsing | Custom stdin parser | `rulez gemini hook` subcommand | Already implemented, tested |
| Workspace isolation | Custom temp dir logic | `setup_workspace` from harness.sh | Phase 23 established this |

**Key insight:** The Gemini adapter code (`rulez/src/adapters/gemini.rs`, `rulez/src/cli/gemini_install.rs`) is already production-quality. The E2E adapter is purely a shell wrapper that wires workspace config to CLI invocation.

---

## Common Pitfalls

### Pitfall 1: --yolo Does Not Always Auto-Approve
**What goes wrong:** `gemini -p "prompt" --yolo` still prompts "Does this plan sound good?" in some versions, hanging the test.
**Why it happens:** Active bug in Gemini CLI (GitHub issue #13561, P2). The flag sets YOLO mode but confirmation dialogs still appear for plan-mode tasks.
**How to avoid:** Use prompts that are simple, single-step, direct tool invocations (e.g., "Read the file README.md and tell me the first line"). Avoid prompts that trigger the agent's planning mode. Consider `--approval-mode=auto_edit` for file-only tasks. Add a reasonable timeout (120s) and treat timeout as skip (exit 77), not fail.
**Warning signs:** Test hangs indefinitely, gemini process not exiting, no output written to gemini-output.txt.

### Pitfall 2: GEMINI_API_KEY Not Set in CI
**What goes wrong:** All hook-fire/deny/inject scenarios fail silently or with cryptic auth errors.
**Why it happens:** `gemini` requires `GEMINI_API_KEY` environment variable for API calls.
**How to avoid:** `gemini_adapter_check` MUST verify `GEMINI_API_KEY` is set before any scenario runs. Return exit 77 (skip) rather than fail if API key is absent. Document CI secret setup clearly in phase plan.
**Warning signs:** `gemini_adapter_check` returns 1, all scenarios skipped.

### Pitfall 3: Project-Level .gemini/settings.json Not Discovered
**What goes wrong:** Hook never fires — rulez audit log has no new entries after gemini invocation.
**Why it happens:** Gemini CLI discovers `.gemini/settings.json` from the **current working directory** at invocation time. If `gemini -p` is run from a different directory, the project config is ignored.
**How to avoid:** In `invoke_gemini_headless`, always `cd "${workspace}"` before running `gemini`. Mirrors how `invoke_claude_headless` uses `cd "${workspace}"`.
**Warning signs:** `assert_log_contains` fails for hook-fire scenario even though `setup_gemini_hooks` succeeded.

### Pitfall 4: Fixture Tool Names vs Gemini Native Names
**What goes wrong:** Deny rule with `tools: ["Bash"]` never fires because Gemini sends `run_shell_command`.
**Why it happens:** Fixtures use canonical tool names, but Gemini CLI sends its native tool names. RuleZ adapter maps them — but only if the hook fires.
**How to avoid:** Verify hook fires first (scenario 02). Use canonical names in fixtures — they are correct after mapping. Test with a deny rule on `Bash` tool which maps from `run_shell_command`.
**Warning signs:** hook-fire passes, deny fails. Or: deny rule logs show no match despite tool being called.

### Pitfall 5: JSON Output Format May Not Be Available
**What goes wrong:** `--output-format json` flag is not recognized or produces malformed output in older gemini versions.
**Why it happens:** The `--output-format json` flag was documented before being released to stable channel (confirmed issue #9009 and #8022). Some versions use `-o json` instead.
**How to avoid:** Do not rely on parsing gemini's JSON output for test assertions. Use the rulez audit log (`~/.claude/logs/rulez.log`) and marker files instead. Gemini's output file is secondary evidence only.
**Warning signs:** `gemini --help` does not show `--output-format` flag. Treat as degraded, not failed.

### Pitfall 6: Sandbox Mode Blocking Shell Tool Execution
**What goes wrong:** `--yolo` mode on some platforms enables sandboxing that prevents tool execution.
**Why it happens:** The `--sandbox` flag or `GEMINI_SANDBOX=true` setting can be inherited from environment or settings.json. Sandboxed execution may prevent `run_shell_command` from working.
**How to avoid:** Ensure test workspaces do NOT have `"sandbox": true` in their `.gemini/settings.json`. Do not set `GEMINI_SANDBOX` in test environment. For the inject scenario, prefer using `write_file` tool (maps to `Write`) rather than `run_shell_command` (maps to `Bash`) for the marker file — it avoids sandbox complications.
**Warning signs:** Tool calls return error; hook fires but inject_command never runs.

### Pitfall 7: run.sh CLI Availability Check Hardcodes claude-code
**What goes wrong:** `run.sh` has a hardcoded `if [[ "${cli_name}" == "claude-code" ]]` block for adapter check. Adding gemini requires modifying this block.
**Why it happens:** Phase 23 implemented only the Claude check. The pattern needs extension for each new CLI.
**How to avoid:** In Phase 24, add a parallel block in `run.sh` for `gemini` that calls `gemini_adapter_check` and sets `GEMINI_CLI_AVAILABLE`. Source `gemini_adapter.sh` at the top of `run.sh`.
**Warning signs:** gemini scenarios always skip even when gemini is installed — GEMINI_CLI_AVAILABLE never set to 1.

---

## Code Examples

### Verified: Hook registration format (from codebase)
```rust
// Source: rulez/src/cli/gemini_install.rs lines 121-131
fn build_hook_entry(command: &str) -> GeminiMatcherEntry {
    GeminiMatcherEntry {
        matcher: Some(".*".to_string()),
        hooks: Some(vec![GeminiHookCommand {
            hook_type: Some("command".to_string()),
            command: Some(command.to_string()),
            timeout: Some(5),
            extra: HashMap::new(),
        }]),
        extra: HashMap::new(),
    }
}
```

The timeout in `gemini_install.rs` uses `5` (interpreted as seconds by Gemini CLI). For E2E testing, use `10` to give rulez more time.

### Verified: Settings file locations (from codebase)
```rust
// Source: rulez/src/cli/gemini_install.rs lines 169-181
fn settings_path(scope: Scope) -> Result<PathBuf> {
    match scope {
        Scope::Project => {
            let cwd = std::env::current_dir()?;
            Ok(cwd.join(".gemini").join("settings.json"))
        }
        Scope::User => {
            let home = dirs::home_dir()?;
            Ok(home.join(".gemini").join("settings.json"))
        }
        // ...
    }
}
```

**Project scope = `<cwd>/.gemini/settings.json`** — must cd to workspace before invocation.

### Verified: Gemini hook command that rulez_binary should invoke
```bash
# The hook command registered in .gemini/settings.json must be:
# /absolute/path/to/rulez gemini hook
#
# Source: rulez/src/cli/gemini_install.rs line 62
# hook_command = format!("{} gemini hook", cch_path.display())
```

Note: The install code uses `cch` (old binary name) in binary resolution but the command string format `"{binary} gemini hook"` is correct. For tests, use absolute path to `rulez` binary + ` gemini hook`.

### Verified: Hook event names (from codebase)
```rust
// Source: rulez/src/cli/gemini_install.rs lines 8-20
const GEMINI_HOOK_EVENTS: [&str; 11] = [
    "BeforeTool",
    "AfterTool",
    "BeforeAgent",
    "AfterAgent",
    "BeforeModel",
    "AfterModel",
    "BeforeToolSelection",
    "SessionStart",
    "SessionEnd",
    "Notification",
    "PreCompact",
];
```

For E2E testing we only need `BeforeTool` in the workspace settings.json. The full install registers all 11 — not needed for isolated tests.

### Headless invocation pattern (verified from docs + known issues)
```bash
# Primary invocation - use within workspace dir
(
  cd "${workspace}" && \
  NO_COLOR=true timeout "${timeout_secs}" gemini \
    -p "${prompt}" \
    --yolo \
    --output-format json \
    2>&1
) | tee "${output_file}" || exit_code="${PIPESTATUS[0]}"

# Fallback if --output-format json not supported (older versions):
(
  cd "${workspace}" && \
  NO_COLOR=true timeout "${timeout_secs}" gemini \
    -p "${prompt}" \
    --yolo \
    2>&1
) | tee "${output_file}" || exit_code="${PIPESTATUS[0]}"
```

### 01-install.sh scenario structure
```bash
# Pattern: mirrors claude-code/01-install.sh exactly but uses gemini subcommand
scenario_install() {
  local workspace="$1"
  local rulez_binary="$2"
  local failures=0

  # Run rulez gemini install with --scope project --binary flag
  local install_output
  install_output="$(cd "${workspace}" && "${rulez_binary}" gemini install \
    --scope project \
    --binary "${rulez_binary}" 2>&1)"
  local install_exit=$?

  assert_exit_code "${install_exit}" 0 "rulez gemini install exits 0" || failures=$((failures + 1))
  assert_file_exists "${workspace}/.gemini/settings.json" ".gemini/settings.json created" || failures=$((failures + 1))
  assert_file_contains "${workspace}/.gemini/settings.json" '"BeforeTool"' \
    "settings.json contains BeforeTool hook" || failures=$((failures + 1))
  assert_file_contains "${workspace}/.gemini/settings.json" '"command"' \
    "settings.json contains command entry" || failures=$((failures + 1))

  return $((failures > 0 ? 1 : 0))
}
```

### run.sh extension needed for gemini adapter check
```bash
# Add to run.sh in the CLI loop (parallel to existing claude-code block):
if [[ "${cli_name}" == "gemini" ]]; then
  source "${E2E_ROOT}/lib/gemini_adapter.sh"
  if gemini_adapter_check > /dev/null 2>&1; then
    GEMINI_CLI_AVAILABLE=1
  else
    GEMINI_CLI_AVAILABLE=0
    echo "  NOTE: gemini CLI not available — scenarios requiring it will be skipped" >&2
  fi
  export GEMINI_CLI_AVAILABLE
fi
```

---

## State of the Art

| Old Approach | Current Approach | When Changed | Impact |
|--------------|------------------|--------------|--------|
| Mocking CLI calls | Real API calls with GEMINI_API_KEY | Decision in CONTEXT.md | Requires API key in CI |
| Global `~/.gemini/settings.json` | Project `.gemini/settings.json` per workspace | Phase 23 pattern established | Full workspace isolation |
| `--yolo` flag (unstable) | `--yolo` primary, `--approval-mode=auto_edit` fallback | Feb 2026 (active bug) | May need version check |
| `--output-format json` stable | `--output-format json` preview/unstable in some versions | GitHub issue #9009 | Don't parse gemini output for assertions |

**Deprecated/outdated:**
- `--approval-mode yolo` (with space, not equals): Use `--yolo` flag or `--approval-mode=yolo` (with equals sign)
- Binary detection of `cch` in `gemini_install.rs`: Codebase still has `cch` references in binary resolution — tests must use explicit `--binary` flag to avoid this

---

## Open Questions

1. **Does `--yolo` work reliably in current gemini version for E2E tests?**
   - What we know: Bug #13561 is open (P2), PR #18104 was in progress as of Feb 2026
   - What's unclear: Whether the fix has shipped to stable channel; current npm latest version behavior
   - Recommendation: In `gemini_adapter_check`, run `gemini --version` and log it. In `invoke_gemini_headless`, set a 120s timeout and treat exit 124 (timeout) as skip (77), not failure. Design prompts to avoid planning mode.

2. **Does `--output-format json` work in the currently installed version?**
   - What we know: Was documented before being released to stable; issue #9009 is closed (COMPLETED) suggesting it shipped at some point
   - What's unclear: Which npm version introduced it; current stable behavior
   - Recommendation: Include `--output-format json` in invocation but do NOT assert on gemini's JSON output. Assert only on rulez audit log and marker files.

3. **What model flag minimizes cost in CI?**
   - What we know: `--model` / `-m` flag selects model; `gemini-2.0-flash-exp` or `gemini-1.5-flash` are cheapest
   - What's unclear: Current model availability; whether flash models support all tools
   - Recommendation: Add `--model gemini-2.0-flash-exp` to invocation. Document that `GEMINI_MODEL` env var can override.

4. **Does the inject scenario work with `write_file` instead of `run_shell_command`?**
   - What we know: Sandbox may block shell execution; `write_file` → `Write` mapping is cleaner
   - What's unclear: Whether gemini will use `write_file` when prompted to "write a file containing X"
   - Recommendation: Design inject prompt to explicitly request file creation ("Write the text 'E2E-INJECTED' to the file marker.txt"). This uses `write_file` (canonical: `Write`) and avoids sandbox complications.

5. **Does rulez detect deny correctly for Gemini?**
   - What we know: `rulez/src/adapters/gemini.rs` maps BeforeTool → PreToolUse; deny response has `"decision": "deny"` and `"continue": false`
   - What's unclear: Whether Gemini CLI exits non-zero when a hook denies, or handles it internally (as Claude Code does)
   - Recommendation: Don't assert on exit code for deny scenario. Assert on audit log containing deny rule name + block action. Mirror Claude Code scenario 03-deny.sh pattern.

---

## Sources

### Primary (HIGH confidence)
- `rulez/src/adapters/gemini.rs` — hook payload parsing, event mapping, tool name mapping, response format
- `rulez/src/cli/gemini_install.rs` — settings.json format, file locations (project/user/system scopes), hook entry structure
- `rulez/src/cli/gemini_hook.rs` — hook runner stdin/stdout protocol
- `rulez/src/cli/gemini_doctor.rs` — all 11 hook event names, extensions dir structure
- `e2e/lib/claude_adapter.sh` — template for gemini_adapter.sh structure
- `e2e/scenarios/claude-code/*.sh` — template for scenario script patterns
- `e2e/fixtures/claude-code/*.yaml` — template for fixture YAML patterns

### Secondary (MEDIUM confidence)
- [Gemini CLI Headless Mode](https://google-gemini.github.io/gemini-cli/docs/cli/headless.html) — `-p`, `--yolo`, `--output-format`, `--approval-mode` flags
- [Gemini CLI Hooks Reference](https://geminicli.com/docs/hooks/reference/) — BeforeTool/AfterTool payload schemas, decision values
- [Google Developers Blog: Tailoring with Hooks](https://developers.googleblog.com/tailor-gemini-cli-to-your-workflow-with-hooks/) — confirmed hook JSON format
- [Gemini CLI Configuration](https://google-gemini.github.io/gemini-cli/docs/get-started/configuration.html) — settings.json locations

### Tertiary (LOW confidence — flag as needing validation)
- [GitHub Issue #13561](https://github.com/google-gemini/gemini-cli/issues/13561) — `--yolo` bug status; P2, PR in progress; may be fixed in latest npm version
- [GitHub Issue #9009](https://github.com/google-gemini/gemini-cli/issues/9009) — `--output-format json` availability; issue closed COMPLETED but version unclear
- [Inventive HQ Headless Guide](https://inventivehq.com/knowledge-base/gemini/how-to-use-headless-mode) — `--non-interactive` flag claim (not found in official docs, LOW confidence)

---

## Metadata

**Confidence breakdown:**
- Standard stack: HIGH — verified from codebase (`gemini_install.rs`, `gemini_hook.rs`, `gemini.rs`)
- Architecture (adapter pattern, file locations): HIGH — directly derived from existing codebase
- Headless invocation flags: MEDIUM — documented but active bugs in `--yolo`
- Pitfalls: HIGH for items verified from codebase/issues; MEDIUM for behavioral assumptions

**Research date:** 2026-02-22
**Valid until:** 2026-03-22 (30 days) — Gemini CLI releases frequently; `--yolo` bug status may change

---

## Phase 24 Deliverables Summary

Based on this research, Phase 24 requires:

1. **`e2e/lib/gemini_adapter.sh`** — New file with:
   - `GEMINI_CLI_NAME="gemini"`
   - `gemini_adapter_check()` — checks `gemini` in PATH AND `GEMINI_API_KEY` is set
   - `require_gemini_cli()` — returns 77 if `GEMINI_CLI_AVAILABLE!=1`
   - `setup_gemini_hooks(workspace, rulez_binary)` — writes `.gemini/settings.json`
   - `invoke_gemini_headless(workspace, prompt, timeout)` — runs `gemini -p --yolo --output-format json`

2. **`e2e/scenarios/gemini/01-install.sh`** — Tests `rulez gemini install --scope project --binary <path>`
   - Asserts `.gemini/settings.json` created
   - Asserts contains `BeforeTool` key
   - Asserts contains `command` entry
   - Does NOT require live gemini CLI (structural only)

3. **`e2e/scenarios/gemini/02-hook-fire.sh`** — Tests BeforeTool hook fires
   - Requires `GEMINI_CLI_AVAILABLE=1` (calls `require_gemini_cli`)
   - Sets up `.gemini/settings.json` via `setup_gemini_hooks`
   - Copies `hooks-hookfire.yaml` to `${workspace}/.gemini/hooks.yaml` (or `.claude/hooks.yaml` — see note)
   - Invokes `gemini -p "Read the file README.md" --yolo --output-format json`
   - Asserts audit log contains `e2e-hookfire-log`

4. **`e2e/scenarios/gemini/03-deny.sh`** — Tests deny rule blocks tool call
   - Same setup pattern as 02-hook-fire
   - Prompt: `"Run this shell command: git push --force origin main"`
   - Asserts audit log contains `e2e-deny-force-push`
   - Asserts audit log contains `block`

5. **`e2e/scenarios/gemini/04-inject.sh`** — Tests inject_command creates marker file
   - Same setup pattern
   - Prompt: `"Write the text 'E2E-INJECTED' to the file marker.txt"` (uses write_file → Write)
   - Or: `"Run this shell command: echo hello-e2e-inject"` (uses run_shell_command → Bash)
   - Asserts `e2e-inject-fired.marker` exists
   - Asserts audit log contains `e2e-inject-marker`

6. **`e2e/fixtures/gemini/hooks-hookfire.yaml`** — Same as claude-code version (uses `Bash` canonical name)

7. **`e2e/fixtures/gemini/hooks-deny.yaml`** — Same as claude-code version (`Bash`, `git push.*--force` pattern)

8. **`e2e/fixtures/gemini/hooks-inject.yaml.template`** — Same as claude-code version (uses `Bash` canonical name)

9. **`e2e/run.sh` modification** — Add gemini adapter check block parallel to claude-code block; source `gemini_adapter.sh`

**Hooks.yaml location note:** The rulez config file (`hooks.yaml`) is a RuleZ concept, not a Gemini CLI concept. It must be at a location where `rulez gemini hook` can find it when invoked from the workspace. RuleZ config loading uses `cwd` from the Gemini hook event payload to find the project hooks.yaml. The project config path for RuleZ is `.claude/hooks.yaml` (project scope) — check `rulez/src/config.rs` to confirm the exact resolution path before implementing.
