#!/bin/bash
#
# generate-changelog.sh
# Generate changelog entries from conventional commits
#
# Usage: ./generate-changelog.sh [version]
#
# Parses commits since the last tag and groups them by type:
# - feat: -> Added
# - fix: -> Fixed
# - docs: -> Documentation
# - chore: -> Changed
# - feat!: -> BREAKING CHANGES
#
# Output is printed to stdout for review before adding to CHANGELOG.md
#

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

# Get version from argument or read from Cargo.toml
if [ -n "$1" ]; then
    VERSION="$1"
else
    VERSION=$("$SCRIPT_DIR/read-version.sh")
fi

DATE=$(date +%Y-%m-%d)
PREV_TAG=$(git describe --tags --abbrev=0 2>/dev/null || echo "")

echo "Generating changelog for v${VERSION}"
echo "Previous tag: ${PREV_TAG:-'(none - first release)'}"
echo "Date: ${DATE}"
echo ""
echo "=============================================="
echo ""

# Get commits since last tag (or all if no tags)
if [ -n "$PREV_TAG" ]; then
    COMMITS=$(git log --pretty=format:"%s" "$PREV_TAG..HEAD" 2>/dev/null || echo "")
else
    COMMITS=$(git log --pretty=format:"%s" 2>/dev/null || echo "")
fi

if [ -z "$COMMITS" ]; then
    echo "No commits found since ${PREV_TAG:-'beginning'}"
    exit 0
fi

# Initialize categories
BREAKING=""
FEATURES=""
FIXES=""
DOCS=""
CHORES=""
OTHER=""

# Parse commits
while IFS= read -r commit; do
    [ -z "$commit" ] && continue

    case "$commit" in
        feat!:*)
            msg="${commit#feat!: }"
            BREAKING="${BREAKING}- ${msg}\n"
            ;;
        fix!:*)
            msg="${commit#fix!: }"
            BREAKING="${BREAKING}- ${msg}\n"
            ;;
        feat:*)
            msg="${commit#feat: }"
            FEATURES="${FEATURES}- ${msg}\n"
            ;;
        fix:*)
            msg="${commit#fix: }"
            FIXES="${FIXES}- ${msg}\n"
            ;;
        docs:*)
            msg="${commit#docs: }"
            DOCS="${DOCS}- ${msg}\n"
            ;;
        chore:*)
            msg="${commit#chore: }"
            CHORES="${CHORES}- ${msg}\n"
            ;;
        refactor:*)
            msg="${commit#refactor: }"
            CHORES="${CHORES}- ${msg}\n"
            ;;
        perf:*)
            msg="${commit#perf: }"
            FEATURES="${FEATURES}- ${msg} (performance)\n"
            ;;
        test:*)
            msg="${commit#test: }"
            CHORES="${CHORES}- ${msg}\n"
            ;;
        *)
            # Non-conventional commits go to Other
            OTHER="${OTHER}- ${commit}\n"
            ;;
    esac
done <<< "$COMMITS"

# Generate markdown output
echo "## [${VERSION}] - ${DATE}"
echo ""

if [ -n "$BREAKING" ]; then
    echo "### BREAKING CHANGES"
    echo ""
    echo -e "$BREAKING"
fi

if [ -n "$FEATURES" ]; then
    echo "### Added"
    echo ""
    echo -e "$FEATURES"
fi

if [ -n "$FIXES" ]; then
    echo "### Fixed"
    echo ""
    echo -e "$FIXES"
fi

if [ -n "$DOCS" ]; then
    echo "### Documentation"
    echo ""
    echo -e "$DOCS"
fi

if [ -n "$CHORES" ]; then
    echo "### Changed"
    echo ""
    echo -e "$CHORES"
fi

if [ -n "$OTHER" ]; then
    echo "### Other"
    echo ""
    echo -e "$OTHER"
fi

echo ""
echo "=============================================="
echo ""
echo "To update CHANGELOG.md:"
echo "1. Review the above output"
echo "2. Copy relevant sections to CHANGELOG.md"
echo "3. Edit descriptions for clarity"
echo "4. Remove any duplicate or irrelevant entries"
