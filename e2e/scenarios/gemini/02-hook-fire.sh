#!/usr/bin/env bash
# 02-hook-fire.sh — E2E scenario: BeforeTool hook fires and RuleZ logs the event
#
# Scenario function: scenario_hook_fire(workspace, rulez_binary)
# Invokes gemini -p headlessly and verifies via audit log.

# Source Gemini adapter for invoke_gemini_headless, setup_gemini_hooks
# shellcheck source=../../lib/gemini_adapter.sh
source "${E2E_ROOT}/lib/gemini_adapter.sh"

# scenario_hook_fire workspace rulez_binary
# Sets up workspace with hookfire fixture, invokes gemini headlessly,
# asserts the audit log contains the hookfire rule name.
# Returns 0 if all assertions pass, 1 if any fail.
scenario_hook_fire() {
  local workspace="$1"
  local rulez_binary="$2"

  # This scenario requires a live gemini CLI (returns 77 = skip)
  require_gemini_cli || return $?

  local failures=0

  # Write .gemini/settings.json with BeforeTool hook pointing at rulez
  setup_gemini_hooks "${workspace}" "${rulez_binary}"

  # Copy hookfire fixture into workspace as hooks.yaml
  # NOTE: RuleZ config (hooks.yaml) always lives at .claude/hooks.yaml even for gemini tests.
  # The .gemini/settings.json tells gemini CLI to call `rulez gemini hook`,
  # and rulez reads its policy config from .claude/hooks.yaml.
  mkdir -p "${workspace}/.claude"
  cp "${E2E_ROOT}/fixtures/gemini/hooks-hookfire.yaml" "${workspace}/.claude/hooks.yaml"

  # Snapshot the log before invocation (setup_workspace already sets WORKSPACE_LOG_SNAPSHOT,
  # but we refresh here in case setup_gemini_hooks or cp wrote to log)
  local log_file="${HOME}/.claude/logs/rulez.log"
  if [[ -f "${log_file}" ]]; then
    WORKSPACE_LOG_SNAPSHOT="$(wc -l < "${log_file}")"
  else
    WORKSPACE_LOG_SNAPSHOT=0
  fi
  export WORKSPACE_LOG_SNAPSHOT

  # Invoke Gemini headlessly to trigger the hook
  invoke_gemini_headless "${workspace}" "Run this bash command: echo hello-e2e-hookfire" 120 || true

  # Assert that audit log contains the hookfire rule name
  assert_log_contains "e2e-hookfire-log" \
    "audit log contains hookfire rule name" || failures=$((failures + 1))

  if [[ "${failures}" -eq 0 ]]; then
    return 0
  else
    return 1
  fi
}
