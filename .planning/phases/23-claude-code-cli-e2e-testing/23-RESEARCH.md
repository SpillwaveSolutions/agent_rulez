# Phase 23: Claude Code CLI E2E Testing - Research

**Researched:** 2026-02-22
**Domain:** Bash E2E test harness + Claude Code CLI headless invocation + hook isolation
**Confidence:** HIGH (Claude CLI verified from live binary at v2.1.50; hook structure confirmed from production settings.json)

---

<user_constraints>
## User Constraints (from CONTEXT.md)

### Locked Decisions

**Product boundaries**
- RuleZ-only scope — sister products (Memory, Cron, CLOD) are out of scope for this harness
- The harness framework should be extensible enough that sister products could add scenarios later
- Codex CLI does NOT support hooks — hook-based scenarios must be skipped (not failed) for Codex

**CLI detection & headless strategy**
- All 5 CLIs are installed locally and will be required in CI (missing CLI = CI failure)
- Real API calls, not mocks — accept cost and non-determinism for true E2E validation
- Tests use manual hook config for workspace isolation (don't pollute user's real config), even though `rulez <cli> install` commands exist

**Harness language & entry point**
- Shell scripts (bash) as primary implementation; TypeScript if needed for complex logic
- Lives at `e2e/` at repo root, completely separate from cargo unit/integration tests
- These are UAT tests, not unit tests — keep them independent
- Entry point: `task e2e` via Taskfile integration
- Isolated workspaces: `e2e/.runs/<run-id>/<cli>/<test-name>/`

**Scenario scope & proof artifacts**
- **Core 4 scenarios per CLI (must-pass):**
  1. `rulez <cli> install` succeeds in clean workspace
  2. PreToolUse hook fires and is logged (audit log proof)
  3. Deny rule blocks a tool call (exit code + stderr assertion)
  4. Inject rule adds context (marker file via inject_command)
- Proof methods: ALL of — audit log parsing, inject_command marker files, exit code + stderr, and structural assertions
- Assertion strategies vary per scenario: structural assertions, regex pattern matching, constrained prompts for predictable output

**Reporting**
- Console ASCII table (CLI × scenario matrix)
- JUnit XML for GitHub Actions CI integration
- Markdown summary for PR comments / Actions summaries
- Non-zero exit code if any scenario fails

**CI integration**
- All 5 CLIs required in CI
- Unit tests run first, then E2E
- Upload `e2e/.runs/**` artifacts on failure
- Separate from existing cargo test pipeline

### Claude's Discretion
- Exact headless invocation flags per CLI (determined by research)
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

Claude Code CLI v2.1.50 is installed at `/Users/richardhightower/.local/bin/claude` and supports a rich set of headless flags. The critical flag for E2E testing is `-p`/`--print`, which runs Claude non-interactively, takes a prompt argument, and exits. Combined with `--output-format json` and `--dangerously-skip-permissions` (for CI sandboxes), this enables fully automated testing. The `--settings` flag (accepts either a file path or inline JSON string) is the key mechanism for workspace isolation — it injects additional settings without replacing the project-level `.claude/settings.json`, so tests can layer their hook config on top cleanly.

Claude Code's configuration hierarchy is: managed > CLI flags > local (`.claude/settings.local.json`) > project (`.claude/settings.json`) > user (`~/.claude/settings.json`). Running `claude` from a temp workspace directory that contains its own `.claude/settings.json` causes those hooks to take precedence over the user's global config. This is the primary isolation strategy: create an isolated workspace with a test-specific `.claude/settings.json` containing RuleZ hook registration pointing at the test binary, then run `claude` from that workspace. There is **no** `CLAUDE_CONFIG_DIR` environment variable — isolation is workspace-directory-based.

The E2E harness is a greenfield build. The existing Rust integration tests in `rulez/tests/` provide excellent reference patterns (temp dirs, fixture files, evidence JSON), but the E2E harness lives at `e2e/` as pure bash, completely separate from cargo. JUnit XML generation is straightforward in pure bash using heredoc-based XML writing. GitHub Actions step summaries are written by appending markdown to `$GITHUB_STEP_SUMMARY`. ASCII tables use printf fixed-width formatting.

**Primary recommendation:** Use `claude -p "<constrained prompt>" --dangerously-skip-permissions --output-format json --settings '{"hooks":...}' --max-turns 1 --allowedTools Bash` as the harness invocation pattern. Run from an isolated workspace dir containing `.claude/settings.json` with RuleZ hooks registered against the test binary path.

---

## Standard Stack

### Core

| Component | Version/Source | Purpose | Why Standard |
|-----------|---------------|---------|--------------|
| `claude` CLI | v2.1.50 (installed) | Headless agent invocation | Official Anthropic CLI, `-p` flag for non-interactive |
| `bash` | system (4.x+) | Harness script language | Locked decision; no Node/Python dependency |
| `rulez` binary | current build | Policy engine under test | The system under test |
| JUnit XML (pure bash) | hand-rolled heredoc | CI test reporting | No external tools needed; GitHub Actions parses it |

### Supporting

| Component | Version | Purpose | When to Use |
|-----------|---------|---------|-------------|
| `task` (Taskfile) | existing | `task e2e` entry point | Existing project tooling |
| `$GITHUB_STEP_SUMMARY` | GHA built-in | Markdown PR summary | CI runs only (env var absent locally) |
| `actions/upload-artifact@v4` | v4 | Artifact upload on failure | CI workflow only |
| `printf` | bash built-in | ASCII table rendering | Console output |

### Alternatives Considered

| Instead of | Could Use | Tradeoff |
|------------|-----------|----------|
| Pure bash JUnit XML | `jest` or `pytest` for reporting | Adds Node/Python dep; overkill for bash harness |
| `--settings` flag for isolation | `CLAUDE_CONFIG_DIR` (does not exist) | No env var override exists; workspace dir isolation is the correct approach |
| Real API calls | Mocked API responses | Locked decision: real API for true E2E |

---

## Architecture Patterns

### Recommended Project Structure

```
e2e/
├── run.sh                    # Main entry point (called by `task e2e`)
├── lib/
│   ├── harness.sh            # Core harness functions (workspace, assertions, timing)
│   ├── reporting.sh          # JUnit XML generation, ASCII table, markdown summary
│   └── claude_adapter.sh     # Claude Code CLI invocation helpers
├── scenarios/
│   └── claude-code/
│       ├── 01-install.sh     # Scenario: rulez install succeeds
│       ├── 02-hook-fire.sh   # Scenario: PreToolUse fires and is logged
│       ├── 03-deny.sh        # Scenario: Deny rule blocks tool call
│       └── 04-inject.sh      # Scenario: Inject rule creates marker file
├── fixtures/
│   └── claude-code/
│       ├── settings-deny.json    # settings.json with deny rule hooks config
│       ├── settings-inject.json  # settings.json with inject rule hooks config
│       ├── hooks-deny.yaml       # RuleZ hooks.yaml for deny scenario
│       └── hooks-inject.yaml     # RuleZ hooks.yaml for inject scenario
└── .runs/                    # Isolated run workspaces (gitignored)
    └── <run-id>/
        └── claude-code/
            └── <test-name>/  # One dir per scenario execution
                ├── .claude/
                │   ├── settings.json
                │   └── hooks.yaml
                └── logs/
                    └── rulez.log
```

### Pattern 1: Workspace Isolation via Project-Level `.claude/settings.json`

**What:** Each scenario runs from a fresh temp directory that has its own `.claude/settings.json` pointing rulez hooks at the test binary. Claude Code loads project-level settings with higher precedence than `~/.claude/settings.json`.

**When to use:** All scenarios. Never mutate the user's real `~/.claude/settings.json`.

**Example:**
```bash
# Source: Claude Code settings hierarchy (confirmed via Perplexity research + live settings.json inspection)
setup_claude_workspace() {
  local run_dir="$1"
  local scenario="$2"
  local rulez_binary="$3"
  local hooks_config="$4"  # path to hooks.yaml fixture

  local workspace="${run_dir}/claude-code/${scenario}"
  mkdir -p "${workspace}/.claude"

  # Write settings.json with hooks pointing at test binary
  cat > "${workspace}/.claude/settings.json" <<EOF
{
  "hooks": {
    "PreToolUse": [
      {
        "matcher": "*",
        "hooks": [{ "type": "command", "command": "${rulez_binary}", "timeout": 5 }]
      }
    ],
    "PostToolUse": [
      {
        "matcher": "*",
        "hooks": [{ "type": "command", "command": "${rulez_binary}", "timeout": 5 }]
      }
    ]
  }
}
EOF

  # Copy hooks.yaml (RuleZ rules config) into workspace
  cp "${hooks_config}" "${workspace}/.claude/hooks.yaml"

  echo "${workspace}"
}
```

**Key insight:** Claude Code reads `.claude/settings.json` from the **current working directory** (project root), not just `~/.claude/`. Running `claude` from `$workspace` loads `$workspace/.claude/settings.json`, which overrides the user's global hooks.

### Pattern 2: Headless Claude Invocation with Constrained Prompt

**What:** Use `-p` (print mode) with `--max-turns 1`, `--allowedTools Bash`, and a tightly constrained prompt that produces predictable, assertable output.

**When to use:** All scenarios that require triggering hook execution (scenarios 2-4).

**Verified flags from `claude --help` (v2.1.50):**
```bash
# Source: live `claude --help` output
invoke_claude_headless() {
  local workspace="$1"
  local prompt="$2"
  local timeout_secs="${3:-60}"

  # Run from workspace dir so .claude/settings.json is picked up
  cd "${workspace}" && timeout "${timeout_secs}" claude \
    -p "${prompt}" \
    --dangerously-skip-permissions \
    --output-format json \
    --max-turns 1 \
    --allowedTools "Bash" \
    --no-session-persistence
}
```

**Flag rationale (all verified from live `--help`):**
- `-p` / `--print`: Non-interactive mode; exits after response. Required for headless.
- `--dangerously-skip-permissions`: Bypasses confirmation dialogs. Safe in isolated sandbox workspace.
- `--output-format json`: Single JSON result (not streaming). Easier to parse.
- `--max-turns 1`: Limits to one agent turn; prevents runaway API spend.
- `--allowedTools Bash`: Restricts tools, preventing unintended side effects.
- `--no-session-persistence`: Disables session saving; clean test isolation.

**Prompt design for predictable output (deny scenario):**
```bash
DENY_PROMPT="Run this exact bash command and report only the exit code: git push --force origin test-branch"
```
A force-push attempt triggers the RuleZ deny rule (exit code 2), which Claude Code interprets as a blocked tool call. The agent will report the block in its output. The assertion checks the **audit log**, not the agent's output text.

### Pattern 3: Audit Log as Proof Artifact

**What:** RuleZ writes JSON Lines to `~/.claude/logs/rulez.log` (default path from `logging.rs`). E2E tests redirect the log path to the workspace's local log file to avoid cross-contamination with the user's real log.

**When to use:** Scenarios 2, 3, 4 — any scenario that verifies hook execution.

**Log path override:**
The `rulez` binary reads log path from config or defaults to `~/.claude/logs/rulez.log`. For test isolation, the hooks.yaml in the test workspace should set a local log path:

```yaml
# e2e/fixtures/claude-code/hooks-deny.yaml
version: "1.0"

settings:
  log_level: "info"
  fail_open: false
  # No log_path field in current config — log goes to ~/.claude/logs/rulez.log
  # Assertion: grep the global log for the test session_id

rules:
  - name: e2e-deny-force-push
    description: "E2E test: blocks git force push"
    matchers:
      tools: ["Bash"]
      command_match: "git push.*--force|git push.*-f"
    actions:
      block: true
```

**NOTE:** The current `logging.rs` always writes to `~/.claude/logs/rulez.log`. There is no config-level log path override. Assertions must grep the global log file filtered by session_id. Alternatively, the E2E harness can record the log file's last line count before the run and compare after.

**Audit log parse pattern:**
```bash
assert_hook_fired() {
  local log_file="${HOME}/.claude/logs/rulez.log"
  local expected_rule="$1"
  local lines_before="$2"

  # Read only new lines added since test start
  local new_entries
  new_entries=$(tail -n +"$((lines_before + 1))" "${log_file}" 2>/dev/null)

  if echo "${new_entries}" | grep -q "\"${expected_rule}\""; then
    return 0
  else
    return 1
  fi
}
```

### Pattern 4: inject_command Marker File Strategy

**What:** The inject scenario uses an `inject_command` in the RuleZ rule that writes a marker file to the workspace. After `claude` exits, the test asserts the marker file exists.

**When to use:** Scenario 4 (inject rule).

```yaml
# e2e/fixtures/claude-code/hooks-inject.yaml
version: "1.0"
rules:
  - name: e2e-inject-marker
    description: "E2E test: inject command writes marker file"
    matchers:
      tools: ["Read"]
    actions:
      inject_command: "touch ${WORKSPACE}/e2e-inject-fired.marker && echo 'INJECTED'"
```

**Issue:** The `inject_command` is executed with `sh -c` in the rulez binary. The `${WORKSPACE}` variable must be available at shell execution time. Inject the workspace path into the hooks.yaml at test setup time (sed substitution or heredoc template).

```bash
# In harness: generate hooks.yaml with workspace path substituted
generate_inject_hooks() {
  local workspace="$1"
  sed "s|WORKSPACE_PLACEHOLDER|${workspace}|g" \
    e2e/fixtures/claude-code/hooks-inject.yaml.template \
    > "${workspace}/.claude/hooks.yaml"
}
```

### Pattern 5: JUnit XML Generation in Pure Bash

**What:** Build JUnit XML by appending testcase elements to a file, then wrap in testsuite at the end.

**When to use:** Always; drives GitHub Actions test reporting.

```bash
# Source: Adapted from standard JUnit XML schema
JUNIT_FILE=""
JUNIT_TESTS=0
JUNIT_FAILURES=0

junit_init() {
  JUNIT_FILE="$1"
  JUNIT_TESTS=0
  JUNIT_FAILURES=0
  # Start collecting testcases in a temp file
  JUNIT_CASES_FILE=$(mktemp)
}

junit_testcase() {
  local classname="$1"
  local testname="$2"
  local time_secs="$3"
  local status="$4"   # "pass" or "fail"
  local message="${5:-}"

  JUNIT_TESTS=$((JUNIT_TESTS + 1))

  if [[ "$status" == "fail" ]]; then
    JUNIT_FAILURES=$((JUNIT_FAILURES + 1))
    cat >> "$JUNIT_CASES_FILE" <<EOF
    <testcase classname="${classname}" name="${testname}" time="${time_secs}">
      <failure type="E2EFailure" message="${message}"><![CDATA[${message}]]></failure>
    </testcase>
EOF
  else
    cat >> "$JUNIT_CASES_FILE" <<EOF
    <testcase classname="${classname}" name="${testname}" time="${time_secs}"/>
EOF
  fi
}

junit_write() {
  cat > "$JUNIT_FILE" <<EOF
<?xml version="1.0" encoding="UTF-8"?>
<testsuites>
  <testsuite name="RuleZ E2E" tests="${JUNIT_TESTS}" failures="${JUNIT_FAILURES}" errors="0" time="0">
$(cat "$JUNIT_CASES_FILE")
  </testsuite>
</testsuites>
EOF
  rm -f "$JUNIT_CASES_FILE"
}
```

### Pattern 6: ASCII Table with printf

**What:** Render CLI × scenario matrix using `printf` fixed-width formatting.

```bash
# Source: bash printf formatting (standard)
print_results_table() {
  local -n results_ref="$1"  # associative array: "cli:scenario" -> "PASS|FAIL|SKIP"
  local clis=("claude-code")
  local scenarios=("install" "hook-fire" "deny" "inject")

  printf "\n%-20s" "CLI"
  for s in "${scenarios[@]}"; do
    printf " %-12s" "$s"
  done
  printf "\n"

  printf "%-20s" "--------------------"
  for s in "${scenarios[@]}"; do
    printf " %-12s" "------------"
  done
  printf "\n"

  for cli in "${clis[@]}"; do
    printf "%-20s" "$cli"
    for s in "${scenarios[@]}"; do
      local result="${results_ref["${cli}:${s}"]:-????}"
      if [[ "$result" == "PASS" ]]; then
        printf " %-12s" "PASS"
      elif [[ "$result" == "FAIL" ]]; then
        printf " %-12s" "FAIL"
      else
        printf " %-12s" "SKIP"
      fi
    done
    printf "\n"
  done
  printf "\n"
}
```

### Anti-Patterns to Avoid

- **Mutating `~/.claude/settings.json` in tests:** Always use project-level workspace isolation. Never call `rulez install` in E2E tests (it modifies real config).
- **Using `--max-turns` default (unlimited):** Always pass `--max-turns 1` to cap API spend.
- **Asserting on agent text output:** Agents are non-deterministic. Assert on audit logs, marker files, exit codes, and structural outcomes instead.
- **Hardcoding `~/.claude/logs/rulez.log` in assertions:** Record line count before test, diff after. Avoids cross-contamination from concurrent rulez executions.
- **Running scenarios without timeout:** `claude -p` can hang. Always wrap with `timeout <N>`.
- **Assuming inject_command env vars are available:** The `inject_command` runs via `sh -c` with no guaranteed env. Bake absolute paths into the generated hooks.yaml at test setup time.

---

## Don't Hand-Roll

| Problem | Don't Build | Use Instead | Why |
|---------|-------------|-------------|-----|
| JUnit XML reporting | Custom XML serializer | Pure bash heredoc pattern (see above) | Sufficient for the schema GitHub Actions needs |
| Test result aggregation | Custom state machine | Bash associative array + loop | Adequate for 4 scenarios × 5 CLIs |
| API interaction | HTTP mocking layer | Real `claude -p` invocations | Locked decision: real API |
| Hook config management | Config management system | Fixture files + sed template substitution | Simple enough for bash; no abstraction needed |

**Key insight:** The harness is intentionally simple bash — resist adding TypeScript/Node layers unless ASCII-table rendering or XML generation proves inadequate in bash.

---

## Common Pitfalls

### Pitfall 1: Global Audit Log Contamination
**What goes wrong:** Multiple concurrent test runs (or other rulez processes) pollute `~/.claude/logs/rulez.log`, making assertions fail or false-positive.
**Why it happens:** `rulez` always writes to the global log path; there is no per-workspace log config in the current codebase.
**How to avoid:** Record `wc -l ~/.claude/logs/rulez.log` immediately before each scenario. After the scenario, read only the new lines (via `tail -n +<count+1>`). Also filter by `session_id` if the claude session ID is known.
**Warning signs:** Assertions that pass locally but fail in parallel CI runs.

### Pitfall 2: Non-Deterministic Agent Output
**What goes wrong:** Asserting on the text of Claude's response causes flaky tests.
**Why it happens:** LLM output is non-deterministic even with temperature=0.
**How to avoid:** NEVER assert on agent response text. Assert only on: (1) rulez audit log entries, (2) marker files written by `inject_command`, (3) rulez exit code for the deny scenario (exit code 2 means blocked), (4) the presence of `.claude/settings.json` for the install scenario.
**Warning signs:** Tests that "usually" pass but occasionally fail.

### Pitfall 3: Hook Config Not Loaded From Workspace
**What goes wrong:** Claude Code does not pick up `.claude/settings.json` from the workspace, using global config instead.
**Why it happens:** Running `claude` from the wrong directory, or a parent directory's `.claude/` taking precedence.
**How to avoid:** Always `cd` into the isolated workspace before invoking `claude`. Use `--no-session-persistence` to prevent session reuse. Verify with `claude doctor` or debug output if needed.
**Warning signs:** Hook fire scenario passes even with empty hooks.yaml.

### Pitfall 4: inject_command Baked with Wrong Paths
**What goes wrong:** `inject_command` writes marker file to the wrong location; marker file check fails.
**Why it happens:** Environment variables are not available inside `sh -c` executed by rulez, and relative paths resolve from rulez's cwd, not the test's cwd.
**How to avoid:** Use absolute paths only in `inject_command`. Generate hooks.yaml from a template at test setup time, substituting the absolute workspace path.
**Warning signs:** inject_command runs (audit log shows it) but marker file not found.

### Pitfall 5: API Rate Limiting in CI
**What goes wrong:** Running 4 scenarios × 5 CLIs in rapid succession hits API rate limits.
**Why it happens:** Each `claude -p` invocation makes real API calls.
**How to avoid:** Add `--model claude-haiku-3-5` (cheapest/fastest) for E2E tests to reduce cost and latency. Add `sleep 2` between scenarios if rate-limit errors occur. Phase 23 is Claude Code only (4 scenarios) — manageable.
**Warning signs:** `429` errors in claude output or long timeouts.

### Pitfall 6: Workspace Trust Dialog
**What goes wrong:** `claude` shows a "do you trust this workspace?" dialog in headless mode.
**Why it happens:** First time running `claude -p` in a new directory triggers trust check.
**How to avoid:** The `-p`/`--print` flag documentation states: "The workspace trust dialog is skipped when Claude is run with the -p mode." This is confirmed in the live `--help` output. No additional flag needed.
**Warning signs:** Test hangs indefinitely without output.

### Pitfall 7: `--dangerously-skip-permissions` in CI
**What goes wrong:** CI blocks use of `--dangerously-skip-permissions` due to org policy.
**Why it happens:** This flag bypasses all permission checks; org admin policies may restrict it.
**How to avoid:** Use `--permission-mode bypassPermissions` as the safer alternative (available in `--permission-mode` choices: `acceptEdits`, `bypassPermissions`, `default`, `dontAsk`, `plan`). Try `--permission-mode dontAsk` first (safer than full bypass).
**Warning signs:** Claude exits with error about permission bypass being disallowed.

---

## Code Examples

### Claude Code Headless Invocation (Verified from live `claude --help` v2.1.50)

```bash
# Source: live claude --help output, confirmed flags
# Run from workspace directory so .claude/settings.json is loaded
run_claude_scenario() {
  local workspace="$1"
  local prompt="$2"
  local timeout_secs="${3:-120}"

  cd "${workspace}" || return 1
  timeout "${timeout_secs}" claude \
    -p "${prompt}" \
    --dangerously-skip-permissions \
    --output-format json \
    --max-turns 1 \
    --allowedTools "Bash" \
    --no-session-persistence \
    --model "claude-haiku-3-5" 2>&1
}
```

### Claude Code Hook Config JSON (Verified from production `.claude/settings.json`)

```json
{
  "hooks": {
    "PreToolUse": [
      {
        "matcher": "*",
        "hooks": [
          {
            "type": "command",
            "command": "/absolute/path/to/rulez",
            "timeout": 5
          }
        ]
      }
    ],
    "PostToolUse": [
      {
        "matcher": "*",
        "hooks": [
          {
            "type": "command",
            "command": "/absolute/path/to/rulez",
            "timeout": 5
          }
        ]
      }
    ]
  }
}
```

**Source:** Confirmed from `rulez/src/cli/install.rs` (Rust code that generates this structure) and validated against the actual `~/.claude/settings.json` in this repository.

### Hook Events Supported (from production `settings.json` in this repo)

The installed `rulez` hooks into ALL of these events (from the project's `.claude/settings.json`):
`PreToolUse`, `PostToolUse`, `PostToolUseFailure`, `Notification`, `Stop`, `SubagentStart`, `SubagentStop`, `SessionStart`, `SessionEnd`, `UserPromptSubmit`, `PermissionRequest`, `Setup`, `TeammateIdle`, `TaskCompleted`, `PreCompact`

For E2E testing, only `PreToolUse` is needed for scenarios 2-4.

### Workspace Setup Script Pattern

```bash
# Source: Adapted from rulez/tests/common/mod.rs setup_test_env() pattern
setup_e2e_workspace() {
  local run_id="$1"
  local scenario="$2"
  local rulez_binary="$3"
  local hooks_yaml_template="$4"

  local workspace="e2e/.runs/${run_id}/claude-code/${scenario}"
  mkdir -p "${workspace}/.claude"

  # Generate settings.json with absolute binary path
  cat > "${workspace}/.claude/settings.json" <<EOF
{
  "hooks": {
    "PreToolUse": [
      {
        "matcher": "*",
        "hooks": [{ "type": "command", "command": "${rulez_binary}", "timeout": 5 }]
      }
    ]
  }
}
EOF

  # Generate hooks.yaml with workspace-specific paths substituted
  sed "s|__WORKSPACE__|${PWD}/${workspace}|g" \
    "${hooks_yaml_template}" > "${workspace}/.claude/hooks.yaml"

  echo "${workspace}"
}
```

### Scenario 1: Install Verification

```bash
# Source: rulez install.rs — writes to .claude/settings.json
scenario_install() {
  local workspace="$1"
  local rulez_binary="$2"

  # Run install in a clean workspace
  cd "${workspace}" && \
    "${rulez_binary}" install --binary "${rulez_binary}"

  local exit_code=$?
  local settings_file="${workspace}/.claude/settings.json"

  if [[ $exit_code -ne 0 ]]; then
    echo "FAIL: rulez install exited with code ${exit_code}"
    return 1
  fi

  # Structural assertion: settings.json must contain hooks
  if ! grep -q '"command"' "${settings_file}" 2>/dev/null; then
    echo "FAIL: settings.json does not contain hook command"
    return 1
  fi

  if ! grep -q '"PreToolUse"' "${settings_file}" 2>/dev/null; then
    echo "FAIL: settings.json does not contain PreToolUse hook"
    return 1
  fi

  echo "PASS: rulez install succeeded and settings.json contains hooks"
  return 0
}
```

### GitHub Actions CI Workflow Structure (for E2E phase)

```yaml
# .github/workflows/cli-e2e.yml
name: CLI E2E Tests

on:
  push:
    branches: [main, develop, "feature/**"]
  workflow_dispatch:

jobs:
  e2e-claude-code:
    name: Claude Code E2E
    runs-on: ubuntu-latest
    timeout-minutes: 30
    env:
      ANTHROPIC_API_KEY: ${{ secrets.ANTHROPIC_API_KEY }}

    steps:
      - uses: actions/checkout@v4
      - name: Build rulez
        run: cargo build --release
        working-directory: rulez

      - name: Install Claude Code CLI
        run: npm install -g @anthropic-ai/claude-code

      - name: Run E2E tests
        run: task e2e

      - name: Upload test results
        uses: actions/upload-artifact@v4
        if: always()
        with:
          name: e2e-results
          path: e2e/.runs/
          retention-days: 7

      - name: Publish JUnit results
        uses: dorny/test-reporter@v1
        if: always()
        with:
          name: E2E Test Results
          path: e2e/.runs/**/junit.xml
          reporter: java-junit
```

---

## State of the Art

| Old Approach | Current Approach | When Changed | Impact |
|--------------|------------------|--------------|--------|
| Interactive `claude` sessions | `-p`/`--print` flag for headless | v1.x+ | Enables CI automation |
| Global `~/.claude/settings.json` only | Project-level `.claude/settings.json` with precedence | ~v1.x | Workspace isolation now possible |
| No per-session config | `--settings <file-or-json>` flag | v2.x | Additional settings injection without file mutation |
| `--mcp-debug` flag | Deprecated; use `--debug` instead | v2.x | Simpler debug flag |
| Unlimited turns by default | `--max-turns N` cap available | Current | Cost control for CI |

**New in v2.1.50 (current):**
- `--setting-sources` flag: comma-separated list of sources to load (`user`, `project`, `local`)
- `--no-session-persistence`: disables session saving for clean test isolation
- `--permission-mode`: fine-grained permission control (`acceptEdits`, `bypassPermissions`, `default`, `dontAsk`, `plan`)
- `--include-partial-messages`: streaming support (not needed for E2E)

---

## Open Questions

1. **Log path isolation for rulez**
   - What we know: `rulez` always logs to `~/.claude/logs/rulez.log` (hardcoded in `logging.rs`)
   - What's unclear: Is there a way to override the log path via env var or config? The `settings` struct has no `log_path` field currently.
   - Recommendation: Use line-count-diff approach for log assertions. Consider adding `RULEZ_LOG_PATH` env var support as a small enhancement (not in scope for Phase 23, but worth noting).

2. **`--settings` flag behavior with hooks**
   - What we know: `--settings <file-or-json>` "loads additional settings." The word "additional" is key — does it merge or replace hooks?
   - What's unclear: Does passing `--settings '{"hooks":{...}}'` add to or replace hooks from `.claude/settings.json`?
   - Recommendation: Use project-level `.claude/settings.json` as the primary hook injection mechanism (confirmed working from Perplexity research). Use `--settings` only if project-level doesn't work in CI for some reason. Test manually before relying on it.

3. **claude model availability for E2E in CI**
   - What we know: `claude-haiku-3-5` is fastest/cheapest. `--model` accepts aliases like `haiku`.
   - What's unclear: Whether `claude-haiku-3-5` supports the exact hook fire + deny behavior under test (hook behavior is model-agnostic — it's triggered by the tool call, not the model).
   - Recommendation: Use `--model haiku` for E2E to minimize cost and latency.

4. **`--setting-sources project` for test isolation**
   - What we know: `--setting-sources` accepts `user,project,local` comma-separated.
   - What's unclear: Does `--setting-sources project` exclude user-level settings entirely? This would be the cleanest isolation if it works.
   - Recommendation: Test `--setting-sources project` in the harness to see if it prevents user's `~/.claude/settings.json` from loading during E2E tests.

---

## Sources

### Primary (HIGH confidence)

- Live `claude --help` output (v2.1.50 installed at `/Users/richardhightower/.local/bin/claude`) — all CLI flags
- `rulez/src/cli/install.rs` — exact JSON structure for hooks written to `settings.json`
- `~/.claude/settings.json` (production file) — confirmed hook JSON structure
- `/Users/richardhightower/clients/spillwave/src/rulez_plugin/.claude/settings.json` — project-level hook config with all 14+ event types
- `rulez/src/logging.rs` — confirmed log path: `~/.claude/logs/rulez.log`
- `rulez/tests/common/mod.rs` — workspace isolation pattern (`setup_test_env`)
- `.github/workflows/e2e.yml` — existing CI artifact upload pattern

### Secondary (MEDIUM confidence)

- Perplexity research on Claude Code settings hierarchy (project overrides user) — corroborated by Rust source code in `install.rs`
- Perplexity research on `-p` flag headless invocation — corroborated by live `--help` output
- Perplexity research on JUnit XML bash generation — standard XML schema, multiple sources agree
- Perplexity: confirmed `CLAUDE_CONFIG_DIR` does NOT exist; workspace-dir isolation is the correct approach

### Tertiary (LOW confidence)

- `--setting-sources project` flag behavior (isolation capability) — mentioned in `--help` but behavior not tested
- `--settings` flag merge vs. replace behavior for hooks — described as "additional settings" but merge semantics unverified
- Whether `claude-haiku-3-5` alias `haiku` works with `--model` — flag says aliases accepted, not tested

---

## Metadata

**Confidence breakdown:**
- Claude CLI flags: HIGH — verified from live `claude --help` (v2.1.50)
- Hook JSON structure: HIGH — verified from production `settings.json` and `install.rs` source
- Settings hierarchy (project > user): HIGH — confirmed via Perplexity with corroboration from Rust source
- Bash JUnit XML pattern: HIGH — standard XML schema, straightforward heredoc
- Log path isolation: MEDIUM — path confirmed in source, but isolation strategy is workaround
- `--setting-sources` isolation: LOW — flag exists, behavior not tested

**Research date:** 2026-02-22
**Valid until:** 2026-03-22 (30 days; claude CLI flags stable, but version updates may add/remove flags)
