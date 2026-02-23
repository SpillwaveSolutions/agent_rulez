#!/usr/bin/env bash
# harness.sh — Core E2E test harness functions for RuleZ CLI testing
#
# Usage: source this file, then call harness_init() before any other functions.
#
# Exported globals (set by harness_init):
#   E2E_ROOT          — absolute path to the e2e/ directory
#   RUN_ID            — date-based run identifier (YYYYMMDD-HHMMSS)
#   RUN_DIR           — absolute path to .runs/<RUN_ID>/
#   RULEZ_BINARY      — absolute path to rulez binary under test
#   TOTAL_PASS        — running count of passed assertions
#   TOTAL_FAIL        — running count of failed assertions
#   TOTAL_SKIP        — running count of skipped scenarios
#
# Per-scenario globals (set by setup_workspace):
#   WORKSPACE_LOG_SNAPSHOT — line count of rulez.log before scenario started

set -euo pipefail

# ---------------------------------------------------------------------------
# Workspace Management
# ---------------------------------------------------------------------------

# harness_init — initialize global state, detect rulez binary, create run dir
harness_init() {
  # E2E_ROOT is the e2e/ directory (parent of lib/)
  E2E_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
  export E2E_ROOT

  RUN_ID="$(date +%Y%m%d-%H%M%S)"
  export RUN_ID

  RUN_DIR="${E2E_ROOT}/.runs/${RUN_ID}"
  export RUN_DIR

  # Auto-detect rulez binary: prefer release, fall back to debug
  local release_bin="${E2E_ROOT}/../target/release/rulez"
  local debug_bin="${E2E_ROOT}/../target/debug/rulez"

  if [[ -x "${release_bin}" ]]; then
    RULEZ_BINARY="$(cd "$(dirname "${release_bin}")" && pwd)/$(basename "${release_bin}")"
  elif [[ -x "${debug_bin}" ]]; then
    RULEZ_BINARY="$(cd "$(dirname "${debug_bin}")" && pwd)/$(basename "${debug_bin}")"
  else
    echo "ERROR: rulez binary not found. Build with: cargo build --release" >&2
    echo "  Checked: ${release_bin}" >&2
    echo "  Checked: ${debug_bin}" >&2
    exit 1
  fi
  export RULEZ_BINARY

  # Initialize counters
  TOTAL_PASS=0
  TOTAL_FAIL=0
  TOTAL_SKIP=0
  export TOTAL_PASS TOTAL_FAIL TOTAL_SKIP

  # Create run directory
  mkdir -p "${RUN_DIR}"

  echo "harness: RUN_ID=${RUN_ID}"
  echo "harness: RUN_DIR=${RUN_DIR}"
  echo "harness: RULEZ_BINARY=${RULEZ_BINARY}"
}

# setup_workspace cli_name scenario_name
# Creates an isolated workspace at $RUN_DIR/$cli_name/$scenario_name/
# with a .claude/ subdirectory.
# Echoes the absolute workspace path.
# Sets WORKSPACE_LOG_SNAPSHOT (global) to current rulez.log line count.
setup_workspace() {
  local cli_name="$1"
  local scenario_name="$2"

  local workspace="${RUN_DIR}/${cli_name}/${scenario_name}"
  mkdir -p "${workspace}/.claude"

  # Snapshot rulez.log line count before scenario starts
  local log_file="${HOME}/.claude/logs/rulez.log"
  if [[ -f "${log_file}" ]]; then
    WORKSPACE_LOG_SNAPSHOT="$(wc -l < "${log_file}")"
  else
    WORKSPACE_LOG_SNAPSHOT=0
  fi
  export WORKSPACE_LOG_SNAPSHOT

  echo "${workspace}"
}

# cleanup_workspace workspace status
# If status=pass and E2E_KEEP_ALL is not set, remove workspace.
# If status=fail, keep workspace for debugging and print the path.
cleanup_workspace() {
  local workspace="$1"
  local status="$2"

  if [[ "${status}" == "fail" ]]; then
    echo "  [debug] Workspace retained for debugging: ${workspace}"
  elif [[ "${E2E_KEEP_ALL:-}" == "1" ]]; then
    echo "  [debug] E2E_KEEP_ALL=1; workspace retained: ${workspace}"
  else
    rm -rf "${workspace}"
  fi
}

# ---------------------------------------------------------------------------
# Assertions
# ---------------------------------------------------------------------------

# _pass_fail status msg
# Internal helper: prints PASS or FAIL, increments counters.
_pass_fail() {
  local status="$1"
  local msg="$2"

  if [[ "${status}" == "pass" ]]; then
    TOTAL_PASS=$((TOTAL_PASS + 1))
    printf "    [PASS] %s\n" "${msg}"
    return 0
  else
    TOTAL_FAIL=$((TOTAL_FAIL + 1))
    printf "    [FAIL] %s\n" "${msg}" >&2
    return 1
  fi
}

# assert_file_exists filepath msg
# Returns 0 if file exists, 1 otherwise.
assert_file_exists() {
  local filepath="$1"
  local msg="$2"

  if [[ -f "${filepath}" ]]; then
    _pass_fail "pass" "${msg}"
  else
    _pass_fail "fail" "${msg} (file not found: ${filepath})"
  fi
}

# assert_file_contains filepath pattern msg
# Greps file for pattern. Returns 0/1.
assert_file_contains() {
  local filepath="$1"
  local pattern="$2"
  local msg="$3"

  if [[ ! -f "${filepath}" ]]; then
    _pass_fail "fail" "${msg} (file not found: ${filepath})"
    return
  fi

  if grep -qE "${pattern}" "${filepath}" 2>/dev/null; then
    _pass_fail "pass" "${msg}"
  else
    _pass_fail "fail" "${msg} (pattern '${pattern}' not found in ${filepath})"
  fi
}

# assert_exit_code actual expected msg
# Compares two integers. Returns 0/1.
assert_exit_code() {
  local actual="$1"
  local expected="$2"
  local msg="$3"

  if [[ "${actual}" -eq "${expected}" ]]; then
    _pass_fail "pass" "${msg}"
  else
    _pass_fail "fail" "${msg} (expected exit code ${expected}, got ${actual})"
  fi
}

# assert_log_contains pattern msg
# Reads new lines from ~/.claude/logs/rulez.log since $WORKSPACE_LOG_SNAPSHOT.
# Greps for pattern. Returns 0/1.
assert_log_contains() {
  local pattern="$1"
  local msg="$2"

  local log_file="${HOME}/.claude/logs/rulez.log"
  local snapshot="${WORKSPACE_LOG_SNAPSHOT:-0}"

  if [[ ! -f "${log_file}" ]]; then
    _pass_fail "fail" "${msg} (log file not found: ${log_file})"
    return
  fi

  local new_lines
  new_lines="$(tail -n +"$((snapshot + 1))" "${log_file}" 2>/dev/null || true)"

  if echo "${new_lines}" | grep -qE "${pattern}" 2>/dev/null; then
    _pass_fail "pass" "${msg}"
  else
    _pass_fail "fail" "${msg} (pattern '${pattern}' not found in new log entries)"
  fi
}

# ---------------------------------------------------------------------------
# Timing
# ---------------------------------------------------------------------------

# timer_start — stores $SECONDS in _TIMER_START
timer_start() {
  _TIMER_START="${SECONDS}"
  export _TIMER_START
}

# timer_elapsed — echoes elapsed seconds since timer_start
timer_elapsed() {
  echo $((SECONDS - _TIMER_START))
}

# ---------------------------------------------------------------------------
# Scenario Runner
# ---------------------------------------------------------------------------

# run_scenario cli_name scenario_name scenario_func
# Orchestrates full scenario lifecycle:
#   setup_workspace -> timer_start -> invoke scenario_func -> record_result -> cleanup_workspace
# scenario_func receives: workspace rulez_binary
# Prints scenario status line.
run_scenario() {
  local cli_name="$1"
  local scenario_name="$2"
  local scenario_func="$3"

  printf "\n[scenario] %s / %s\n" "${cli_name}" "${scenario_name}"

  local workspace
  workspace="$(setup_workspace "${cli_name}" "${scenario_name}")"

  timer_start

  local scenario_exit=0
  local scenario_msg=""

  if "${scenario_func}" "${workspace}" "${RULEZ_BINARY}" 2>&1; then
    scenario_exit=0
    scenario_msg="passed"
  else
    scenario_exit=1
    scenario_msg="failed"
  fi

  local elapsed
  elapsed="$(timer_elapsed)"

  local status
  if [[ "${scenario_exit}" -eq 0 ]]; then
    status="pass"
  else
    status="fail"
  fi

  # record_result is defined in reporting.sh (sourced by run.sh before harness is called)
  record_result "${cli_name}" "${scenario_name}" "${status}" "${elapsed}" "${scenario_msg}"

  cleanup_workspace "${workspace}" "${status}"

  if [[ "${status}" == "pass" ]]; then
    printf "[scenario] %s / %s => PASS (%ss)\n" "${cli_name}" "${scenario_name}" "${elapsed}"
  else
    printf "[scenario] %s / %s => FAIL (%ss)\n" "${cli_name}" "${scenario_name}" "${elapsed}" >&2
  fi
}
