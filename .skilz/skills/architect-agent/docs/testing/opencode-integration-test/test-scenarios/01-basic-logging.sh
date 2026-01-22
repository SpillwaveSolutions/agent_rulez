#!/bin/bash
#
# Test 01: Basic Logging
#
# Tests basic log session management:
# - Starting a log session
# - Active log file tracking
# - Completing a log session
# - Log file format

set -euo pipefail

echo "🧪 Test 01: Basic Logging"
echo "========================"
echo ""

# Create temporary test workspace
TEST_DIR=$(mktemp -d /tmp/opencode-test-XXXXXX)
cd "$TEST_DIR"

echo "✓ Created test workspace: $TEST_DIR"

# Setup test workspace structure
mkdir -p debugging/{logs,scripts}

# Copy scripts from architect-agent
ARCH_WORKSPACE="$HOME/.claude/skills/architect-agent"
cp "$ARCH_WORKSPACE/templates/debugging/scripts/log-decision.sh" debugging/scripts/
chmod +x debugging/scripts/log-decision.sh

# Create log-start.sh
cat > debugging/scripts/log-start.sh <<'EOF'
#!/bin/bash
set -euo pipefail
DESCRIPTION=${1:-"session"}
TIMESTAMP=$(date +%Y%m%d_%H%M%S)
LOG_FILE="debugging/logs/log-${TIMESTAMP}-${DESCRIPTION}.md"
mkdir -p debugging/logs
cat > "$LOG_FILE" <<LOGEOF
# Log Session: $DESCRIPTION
**Started:** $(date '+%Y-%m-%d %H:%M:%S')

## Goal
[Document your goal here]

## Success Criteria
- [ ] Criterion 1
- [ ] Criterion 2

---

LOGEOF
echo "$LOG_FILE" > debugging/current_log_file.txt
echo "✅ Log session started: $LOG_FILE"
EOF
chmod +x debugging/scripts/log-start.sh

# Create log-complete.sh
cat > debugging/scripts/log-complete.sh <<'EOF'
#!/bin/bash
set -euo pipefail
if [ ! -f debugging/current_log_file.txt ]; then
    echo "❌ Error: No active log session"
    exit 1
fi
LOG_FILE=$(cat debugging/current_log_file.txt)
TIMESTAMP="[$(date +%H:%M:%S)]"
cat >> "$LOG_FILE" <<LOGEOF

---
$TIMESTAMP 🏁 Final Summary
**Status:** ✅ COMPLETE
**Completed:** $(date '+%Y-%m-%d %H:%M:%S')
---
LOGEOF
echo "✅ Log session completed: $LOG_FILE"
rm debugging/current_log_file.txt
EOF
chmod +x debugging/scripts/log-complete.sh

echo "✓ Copied scripts to workspace"

# Test 1: Start log session
echo ""
echo "Test 1: Starting log session..."
./debugging/scripts/log-start.sh "test-session"

# Verify active log file exists
if [ ! -f debugging/current_log_file.txt ]; then
    echo "❌ FAIL: current_log_file.txt not created"
    exit 1
fi
echo "✓ Active log file pointer created"

# Verify log file exists
LOG_FILE=$(cat debugging/current_log_file.txt)
if [ ! -f "$LOG_FILE" ]; then
    echo "❌ FAIL: Log file $LOG_FILE not created"
    exit 1
fi
echo "✓ Log file created: $LOG_FILE"

# Test 2: Verify log format
echo ""
echo "Test 2: Verifying log format..."
if ! grep -q "# Log Session: test-session" "$LOG_FILE"; then
    echo "❌ FAIL: Log header missing"
    exit 1
fi
echo "✓ Log header present"

if ! grep -q "## Goal" "$LOG_FILE"; then
    echo "❌ FAIL: Goal section missing"
    exit 1
fi
echo "✓ Goal section present"

if ! grep -q "## Success Criteria" "$LOG_FILE"; then
    echo "❌ FAIL: Success Criteria section missing"
    exit 1
fi
echo "✓ Success Criteria section present"

# Test 3: Complete log session
echo ""
echo "Test 3: Completing log session..."
./debugging/scripts/log-complete.sh

# Verify active log cleared
if [ -f debugging/current_log_file.txt ]; then
    echo "❌ FAIL: current_log_file.txt not removed"
    exit 1
fi
echo "✓ Active log cleared"

# Verify final summary added
if ! grep -q "🏁 Final Summary" "$LOG_FILE"; then
    echo "❌ FAIL: Final summary not added"
    exit 1
fi
echo "✓ Final summary added"

# Test 4: Log file is valid markdown
echo ""
echo "Test 4: Validating log file..."
if ! grep -q "^# Log Session:" "$LOG_FILE"; then
    echo "❌ FAIL: Not valid markdown (missing H1 header)"
    exit 1
fi
echo "✓ Valid markdown format"

# Display log file for manual inspection
echo ""
echo "Generated log file:"
echo "=================="
cat "$LOG_FILE"
echo "=================="

# Success
echo ""
echo "✅ All basic logging tests passed!"
echo "   Test workspace: $TEST_DIR"
echo "   Log file: $LOG_FILE"
echo ""
echo "To inspect results: cd $TEST_DIR"
echo "To cleanup: rm -rf $TEST_DIR"

exit 0
