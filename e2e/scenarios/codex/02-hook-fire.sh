#!/usr/bin/env bash
# 02-hook-fire.sh — E2E scenario: hook-fire test for Codex CLI
#
# Scenario function: scenario_hook_fire(workspace, rulez_binary)
# Unconditionally skips — Codex CLI does not support hooks.

# Source Codex adapter for shared helpers
# shellcheck source=../../lib/codex_adapter.sh
source "${E2E_ROOT}/lib/codex_adapter.sh"

# scenario_hook_fire workspace rulez_binary
# Skips unconditionally because Codex CLI does not support hooks.
# Returns 77 (skip) always.
scenario_hook_fire() {
  local workspace="$1"
  local rulez_binary="$2"

  echo "  [skip] Codex CLI does not support hooks (no PreToolUse/BeforeTool equivalent)" >&2
  echo "  [skip] Enable this scenario when Codex CLI adds hook support." >&2
  return 77
}
