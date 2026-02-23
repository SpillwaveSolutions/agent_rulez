#!/usr/bin/env bash
# run.sh — Main E2E test harness entry point for RuleZ CLI testing
#
# Usage:
#   ./e2e/run.sh                   # Run all CLI scenarios
#   ./e2e/run.sh --cli claude-code # Run only claude-code scenarios
#
# Environment:
#   E2E_KEEP_ALL=1   Keep workspace dirs even on pass (default: clean on pass)
#
# Output:
#   Console: ASCII table summary
#   Files:   .runs/<run-id>/junit.xml, .runs/<run-id>/summary.md

set -euo pipefail

# ---------------------------------------------------------------------------
# Determine E2E_ROOT as the directory containing this script
# ---------------------------------------------------------------------------
E2E_ROOT="$(cd "$(dirname "$0")" && pwd)"
export E2E_ROOT

# ---------------------------------------------------------------------------
# Source libraries
# ---------------------------------------------------------------------------
# shellcheck source=lib/harness.sh
source "${E2E_ROOT}/lib/harness.sh"
# shellcheck source=lib/reporting.sh
source "${E2E_ROOT}/lib/reporting.sh"
# shellcheck source=lib/claude_adapter.sh
source "${E2E_ROOT}/lib/claude_adapter.sh"

# ---------------------------------------------------------------------------
# Initialize harness and reporting
# ---------------------------------------------------------------------------
harness_init
reporting_init

# ---------------------------------------------------------------------------
# Parse arguments
# ---------------------------------------------------------------------------
FILTER_CLI=""

while [[ $# -gt 0 ]]; do
  case "$1" in
    --cli)
      FILTER_CLI="$2"
      shift 2
      ;;
    --help|-h)
      echo "Usage: $0 [--cli <cli-name>]"
      echo ""
      echo "Options:"
      echo "  --cli <name>   Run only scenarios for the named CLI (e.g. claude-code)"
      echo "  --help         Show this help message"
      exit 0
      ;;
    *)
      echo "Unknown argument: $1" >&2
      exit 1
      ;;
  esac
done

# ---------------------------------------------------------------------------
# Discover CLI scenario directories
# ---------------------------------------------------------------------------
SCENARIOS_DIR="${E2E_ROOT}/scenarios"

if [[ ! -d "${SCENARIOS_DIR}" ]]; then
  echo "No scenarios directory found at ${SCENARIOS_DIR}"
  echo "Nothing to run."
  write_junit_xml "${RUN_DIR}/junit.xml"
  write_markdown_summary "${RUN_DIR}/summary.md"
  exit 0
fi

# Collect CLI names from subdirectories
CLI_NAMES=()
for cli_dir in "${SCENARIOS_DIR}"/*/; do
  [[ -d "${cli_dir}" ]] || continue
  local_cli_name="$(basename "${cli_dir}")"

  # Apply --cli filter if specified
  if [[ -n "${FILTER_CLI}" && "${local_cli_name}" != "${FILTER_CLI}" ]]; then
    continue
  fi

  CLI_NAMES+=("${local_cli_name}")
done

if [[ ${#CLI_NAMES[@]} -eq 0 ]]; then
  echo "No CLI scenario directories found${FILTER_CLI:+ matching --cli ${FILTER_CLI}}."
  write_junit_xml "${RUN_DIR}/junit.xml"
  write_markdown_summary "${RUN_DIR}/summary.md"
  exit 0
fi

# ---------------------------------------------------------------------------
# Run scenarios for each CLI
# ---------------------------------------------------------------------------
for cli_name in "${CLI_NAMES[@]}"; do
  cli_dir="${SCENARIOS_DIR}/${cli_name}"

  printf "\n=== Running scenarios for: %s ===\n" "${cli_name}"

  # For claude-code scenarios, verify the claude CLI is available.
  # If not found, skip all scenarios in this directory with SKIP status.
  if [[ "${cli_name}" == "claude-code" ]]; then
    if ! claude_adapter_check > /dev/null 2>&1; then
      echo "  SKIP: claude CLI not found in PATH — skipping all claude-code scenarios" >&2
      for scenario_script in $(ls -1 "${cli_dir}"/*.sh 2>/dev/null | sort); do
        [[ -f "${scenario_script}" ]] || continue
        scenario_file="$(basename "${scenario_script}")"
        scenario_name="${scenario_file#[0-9]*-}"
        scenario_name="${scenario_name%.sh}"
        record_result "${cli_name}" "${scenario_name}" "skip" "0" "claude CLI not found in PATH"
        TOTAL_SKIP=$((TOTAL_SKIP + 1))
      done
      continue
    fi
  fi

  # Source and run each scenario script in sorted order
  for scenario_script in $(ls -1 "${cli_dir}"/*.sh 2>/dev/null | sort); do
    [[ -f "${scenario_script}" ]] || continue

    # Derive scenario name from filename: 01-install.sh -> install
    scenario_file="$(basename "${scenario_script}")"
    # Strip leading digits and dash, strip .sh suffix
    scenario_name="${scenario_file#[0-9]*-}"
    scenario_name="${scenario_name%.sh}"

    # Source the scenario script to load its function(s)
    # shellcheck disable=SC1090
    source "${scenario_script}"

    # The scenario function must be named scenario_<scenario_name>
    # Replace dashes with underscores for valid bash function names
    scenario_func="scenario_${scenario_name//-/_}"

    if declare -f "${scenario_func}" > /dev/null 2>&1; then
      run_scenario "${cli_name}" "${scenario_name}" "${scenario_func}"
    else
      echo "WARNING: ${scenario_script} does not define function ${scenario_func}; skipping" >&2
      record_result "${cli_name}" "${scenario_name}" "skip" "0" "scenario function not defined"
      TOTAL_SKIP=$((TOTAL_SKIP + 1))
    fi
  done
done

# ---------------------------------------------------------------------------
# Output results
# ---------------------------------------------------------------------------
print_results_table CLI_NAMES
write_junit_xml "${RUN_DIR}/junit.xml"
write_markdown_summary "${RUN_DIR}/summary.md"

echo ""
echo "Run artifacts:"
echo "  JUnit XML: ${RUN_DIR}/junit.xml"
echo "  Markdown:  ${RUN_DIR}/summary.md"
echo "  Workspaces: ${RUN_DIR}/"

# ---------------------------------------------------------------------------
# Exit non-zero if any test failed
# ---------------------------------------------------------------------------
if [[ "${JUNIT_FAILURES}" -gt 0 ]]; then
  echo ""
  echo "FAILED: ${JUNIT_FAILURES} scenario(s) failed." >&2
  exit 1
fi

echo ""
echo "All scenarios passed."
exit 0
