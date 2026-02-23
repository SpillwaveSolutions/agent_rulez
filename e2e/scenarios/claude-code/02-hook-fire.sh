#!/usr/bin/env bash
# 02-hook-fire.sh — E2E scenario: PreToolUse hook fires and RuleZ logs the event
#
# Scenario function: scenario_hook_fire(workspace, rulez_binary)
# Invokes claude -p headlessly and verifies via audit log.

# Source Claude adapter for invoke_claude_headless, setup_claude_hooks
# shellcheck source=../../lib/claude_adapter.sh
source "${E2E_ROOT}/lib/claude_adapter.sh"

# scenario_hook_fire workspace rulez_binary
# Sets up workspace with hookfire fixture, invokes claude headlessly,
# asserts the audit log contains the hookfire rule name.
# Returns 0 if all assertions pass, 1 if any fail.
scenario_hook_fire() {
  local workspace="$1"
  local rulez_binary="$2"

  local failures=0

  # Write .claude/settings.json with PreToolUse hook pointing at rulez
  setup_claude_hooks "${workspace}" "${rulez_binary}"

  # Copy hookfire fixture into workspace as hooks.yaml
  cp "${E2E_ROOT}/fixtures/claude-code/hooks-hookfire.yaml" "${workspace}/.claude/hooks.yaml"

  # Snapshot the log before invocation (setup_workspace already sets WORKSPACE_LOG_SNAPSHOT,
  # but we refresh here in case setup_claude_hooks or cp wrote to log)
  local log_file="${HOME}/.claude/logs/rulez.log"
  if [[ -f "${log_file}" ]]; then
    WORKSPACE_LOG_SNAPSHOT="$(wc -l < "${log_file}")"
  else
    WORKSPACE_LOG_SNAPSHOT=0
  fi
  export WORKSPACE_LOG_SNAPSHOT

  # Invoke Claude headlessly to trigger the hook
  invoke_claude_headless "${workspace}" "Run this bash command: echo hello-e2e-hookfire" 120 || true

  # Assert that audit log contains the hookfire rule name
  assert_log_contains "e2e-hookfire-log" \
    "audit log contains hookfire rule name" || failures=$((failures + 1))

  if [[ "${failures}" -eq 0 ]]; then
    return 0
  else
    return 1
  fi
}
