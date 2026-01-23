#!/bin/bash
# CCH Validator Script: Secret Detection
# Location: .claude/validators/check-secrets.sh
#
# Checks for potential secrets in file content before writing.
# Returns JSON response for CCH action handler.
#
# Usage: Called automatically by CCH on PreToolUse events
# Environment: CCH_TOOL_INPUT_CONTENT contains file content

set -e

# Get content from CCH environment variable
CONTENT="${CCH_TOOL_INPUT_CONTENT:-}"

# If no content provided, allow operation
if [ -z "$CONTENT" ]; then
    echo '{"continue": true, "context": "", "reason": ""}'
    exit 0
fi

# Patterns that might indicate secrets
# Using extended regex for better matching
PATTERNS=(
    # API keys and tokens
    'api[_-]?key\s*[:=]\s*["\x27]?[a-zA-Z0-9]{16,}'
    'api[_-]?secret\s*[:=]'
    'auth[_-]?token\s*[:=]'
    'bearer\s+[a-zA-Z0-9._-]+'
    
    # Passwords
    'password\s*[:=]\s*["\x27][^\s"'\'']{8,}'
    'passwd\s*[:=]'
    'pwd\s*[:=]\s*["\x27]'
    
    # AWS credentials
    'AKIA[0-9A-Z]{16}'
    'aws_access_key_id\s*[:=]'
    'aws_secret_access_key\s*[:=]'
    
    # Private keys
    '-----BEGIN (RSA |DSA |EC |OPENSSH )?PRIVATE KEY-----'
    '-----BEGIN PGP PRIVATE KEY BLOCK-----'
    
    # Database connection strings
    'mongodb(\+srv)?://[^:]+:[^@]+@'
    'postgres://[^:]+:[^@]+@'
    'mysql://[^:]+:[^@]+@'
    
    # Generic secrets
    'secret\s*[:=]\s*["\x27][^\s"'\'']{8,}'
    'private[_-]?key\s*[:=]'
)

# Check each pattern
for pattern in "${PATTERNS[@]}"; do
    if echo "$CONTENT" | grep -qiE "$pattern"; then
        # Found potential secret
        cat << 'EOF'
{"continue": false, "context": "", "reason": "Potential secret or credential detected in file content. Please use environment variables, a secrets manager, or .env files (which should be gitignored) instead of hardcoding secrets."}
EOF
        exit 0
    fi
done

# No secrets detected
echo '{"continue": true, "context": "Security scan passed: no secrets detected.", "reason": ""}'
