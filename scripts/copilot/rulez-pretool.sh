#!/usr/bin/env bash
# RuleZ Copilot hook wrapper — forwards stdin to cch copilot hook
set -euo pipefail
exec cch copilot hook
