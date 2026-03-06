#!/usr/bin/env bash
# opencode_adapter.sh — OpenCode CLI headless invocation helper and workspace config generator
#
# Usage: source this file before calling adapter functions.
#
# Exported globals:
#   OPENCODE_CLI_NAME — constant string "opencode" for reporting
#
# Functions:
#   opencode_adapter_check()                                — verify opencode is in PATH
#   require_opencode_cli()                                  — return 77 (skip) if unavailable
#   setup_opencode_hooks(workspace, rulez_binary)           — write .opencode/settings.json + plugin
#   invoke_opencode_headless(workspace, prompt, timeout)    — run opencode run headlessly

# Constant for use in reporting/scenario names
OPENCODE_CLI_NAME="opencode"
export OPENCODE_CLI_NAME

# ---------------------------------------------------------------------------
# opencode_adapter_check
# Verifies `opencode` is in PATH. OpenCode uses provider-specific API keys
# (not OAuth), so we only check for the binary presence.
#
# Returns 1 with error message if opencode not found.
# Prints opencode version on success.
# ---------------------------------------------------------------------------
opencode_adapter_check() {
  if ! command -v opencode > /dev/null 2>&1; then
    echo "ERROR: 'opencode' CLI not found in PATH." >&2
    echo "  Install OpenCode: https://github.com/opencode-ai/opencode" >&2
    return 1
  fi

  local version
  version="$(opencode --version 2>&1 || true)"
  echo "opencode_adapter: found opencode CLI: ${version}"
  return 0
}

# ---------------------------------------------------------------------------
# require_opencode_cli
# Returns 0 if opencode CLI is available for headless invocation, 77 (skip) otherwise.
# Scenarios that need opencode should call this at their start and return 77
# (triggering skip) if it fails.
# ---------------------------------------------------------------------------
require_opencode_cli() {
  if [[ "${OPENCODE_CLI_AVAILABLE:-0}" -eq 1 ]]; then
    return 0
  fi
  echo "  [skip] opencode CLI not available for headless invocation" >&2
  return 77
}

# ---------------------------------------------------------------------------
# setup_opencode_hooks workspace rulez_binary
# Sets up RuleZ hook integration for OpenCode in the workspace.
#
# Writes both:
# 1. .opencode/settings.json — command-based hook config (matches `rulez opencode install` output)
# 2. .opencode/plugin/rulez-e2e.ts — TypeScript plugin for direct integration
#
# OpenCode's plugin system auto-discovers plugins from .opencode/plugin/.
# The settings.json approach may require OpenCode to support the "hooks" config key.
#
# Note: RuleZ hooks.yaml is always at .claude/hooks.yaml (even for OpenCode tests).
# ---------------------------------------------------------------------------
setup_opencode_hooks() {
  local workspace="$1"
  local rulez_binary="$2"

  # Ensure workspace dirs exist
  mkdir -p "${workspace}/.opencode"
  mkdir -p "${workspace}/.opencode/plugin"

  # Resolve rulez_binary to absolute path
  local abs_rulez
  abs_rulez="$(cd "$(dirname "${rulez_binary}")" && pwd)/$(basename "${rulez_binary}")"

  # Write command-based hook config (same format as `rulez opencode install`)
  cat > "${workspace}/.opencode/settings.json" <<EOF
{
  "hooks": {
    "file.edited": [
      {
        "type": "command",
        "command": "${abs_rulez} opencode hook",
        "timeout": 5
      }
    ],
    "tool.execute.before": [
      {
        "type": "command",
        "command": "${abs_rulez} opencode hook",
        "timeout": 5
      }
    ],
    "tool.execute.after": [
      {
        "type": "command",
        "command": "${abs_rulez} opencode hook",
        "timeout": 5
      }
    ],
    "session.updated": [
      {
        "type": "command",
        "command": "${abs_rulez} opencode hook",
        "timeout": 5
      }
    ]
  }
}
EOF

  # Write TypeScript plugin for direct plugin-based integration
  cat > "${workspace}/.opencode/plugin/rulez-e2e.ts" <<PLUGIN_EOF
const RULEZ_BINARY = "${abs_rulez}";

export const RulezE2E = async ({ client }: any) => {
  return {
    tool: {
      execute: {
        before: async (input: any, output: any) => {
          const payload = JSON.stringify({
            session_id: "e2e-session",
            hook_event_name: "tool.execute.before",
            tool_name: input.tool || "unknown",
            tool_input: output.args || {},
            cwd: process.cwd(),
          });
          try {
            const proc = Bun.spawn([RULEZ_BINARY, "opencode", "hook"], {
              stdin: new Blob([payload]),
              stdout: "pipe",
              stderr: "pipe",
            });
            const exitCode = await proc.exited;
            const stdout = await new Response(proc.stdout).text();
            if (stdout.trim()) {
              const response = JSON.parse(stdout.trim());
              if (response.continue === false) {
                throw new Error(response.reason || "Blocked by RuleZ policy");
              }
            }
            if (exitCode === 2) {
              throw new Error("Denied by RuleZ policy (exit code 2)");
            }
          } catch (err: any) {
            if (err.message?.includes("RuleZ policy") || err.message?.includes("Blocked")) {
              throw err;
            }
            console.warn("[rulez-e2e] Hook error (fail-open):", err);
          }
        },
        after: async (input: any, output: any) => {
          const payload = JSON.stringify({
            session_id: "e2e-session",
            hook_event_name: "tool.execute.after",
            tool_name: input.tool || "unknown",
            tool_input: output.args || {},
            cwd: process.cwd(),
          });
          try {
            const proc = Bun.spawn([RULEZ_BINARY, "opencode", "hook"], {
              stdin: new Blob([payload]),
              stdout: "pipe",
              stderr: "pipe",
            });
            await proc.exited;
          } catch (err) {
            console.warn("[rulez-e2e] Post-hook error:", err);
          }
        },
      },
    },
  };
};
PLUGIN_EOF

  echo "opencode_adapter: wrote settings.json to ${workspace}/.opencode/settings.json"
  echo "opencode_adapter: wrote plugin to ${workspace}/.opencode/plugin/rulez-e2e.ts"
}

# ---------------------------------------------------------------------------
# invoke_opencode_headless workspace prompt timeout_secs
# Runs OpenCode CLI in headless (non-interactive) mode from the workspace dir.
# Captures stdout+stderr to $workspace/opencode-output.txt AND echoes it.
# Returns opencode's exit code.
# If exit code is 124 (timeout), returns 77 (skip) — timeout treated as skip.
# ---------------------------------------------------------------------------
invoke_opencode_headless() {
  local workspace="$1"
  local prompt="$2"
  local timeout_secs="${3:-120}"

  local output_file="${workspace}/opencode-output.txt"

  local exit_code=0

  # Run opencode from within the workspace so project-level config is discovered
  # OpenCode uses `opencode run "message" --format json` for headless invocation
  (
    cd "${workspace}" && \
    NO_COLOR=true timeout "${timeout_secs}" opencode run \
      "${prompt}" \
      --format json 2>&1
  ) | tee "${output_file}" || exit_code="${PIPESTATUS[0]}"

  if [[ "${exit_code}" -eq 124 ]]; then
    echo "opencode_adapter: opencode timed out after ${timeout_secs}s — treating as skip" >&2
    return 77
  fi

  echo "opencode_adapter: opencode exited with code ${exit_code}"
  return "${exit_code}"
}
