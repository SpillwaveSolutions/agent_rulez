#!/bin/bash
#
# read-version.sh
# Extract version from workspace Cargo.toml
#
# Usage: ./read-version.sh
#
# Returns the version string (e.g., "1.0.0") from [workspace.package] section
#

set -e

# Find repo root (where Cargo.toml with [workspace] lives)
# Path: .claude/skills/release-cch/scripts/ -> need to go up 4 levels
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd "$SCRIPT_DIR/../../../.." && pwd)"

CARGO_TOML="$REPO_ROOT/Cargo.toml"

if [ ! -f "$CARGO_TOML" ]; then
    echo "ERROR: Cargo.toml not found at $CARGO_TOML" >&2
    exit 1
fi

# Extract version from [workspace.package] section
VERSION=$(grep '^version = "' "$CARGO_TOML" | head -1 | sed 's/version = "\(.*\)"/\1/')

if [ -z "$VERSION" ]; then
    echo "ERROR: Could not read version from Cargo.toml" >&2
    echo "Expected format: version = \"X.Y.Z\"" >&2
    exit 1
fi

echo "$VERSION"
