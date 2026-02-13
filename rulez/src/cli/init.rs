//! RuleZ Init Command - Initialize hooks configuration
//!
//! Creates the default hooks.yaml configuration file and supporting directories.

use anyhow::{Context, Result};
use std::fs;
use std::path::Path;

/// Default hooks.yaml template with commented examples
const DEFAULT_HOOKS_YAML: &str = r#"# RuleZ Configuration
# Location: .claude/hooks.yaml
# Documentation: https://github.com/SpillwaveSolutions/code_agent_context_hooks

version: "1.0"

# Global settings
settings:
  debug_logs: false
  log_level: info
  fail_open: true
  script_timeout: 5

# Policy rules
rules:
  # ============================================================
  # SECURITY RULES - Protect against dangerous operations
  # ============================================================
  
  # Block force push to protected branches
  - name: block-force-push
    description: Prevent force push to main/master
    matchers:
      tools: [Bash]
      command_match: "git push.*(--force|-f).*(main|master)"
    actions:
      block: true
    metadata:
      priority: 100
      enabled: true

  # Block hard reset on protected branches  
  - name: block-hard-reset
    description: Prevent destructive git reset operations
    matchers:
      tools: [Bash]
      command_match: "git reset --hard"
    actions:
      block: true
    metadata:
      priority: 90
      enabled: true

  # ============================================================
  # CODE QUALITY RULES - Inject coding standards
  # ============================================================
  
  # Inject Python coding standards when editing .py files
  # - name: python-standards
  #   description: Inject Python coding standards for .py files
  #   matchers:
  #     tools: [Write, Edit]
  #     extensions: [.py]
  #   actions:
  #     inject: .claude/context/python-standards.md
  #   metadata:
  #     priority: 50
  #     enabled: true

  # ============================================================
  # VALIDATION RULES - Run custom validators
  # ============================================================
  
  # Run secret scanner before commits
  # - name: pre-commit-secrets
  #   description: Check for secrets before git commit
  #   matchers:
  #     tools: [Bash]
  #     command_match: "git commit"
  #   actions:
  #     run: .claude/validators/check-secrets.sh
  #   metadata:
  #     priority: 80
  #     timeout: 30
  #     enabled: true
"#;

/// Example Python standards context file
const PYTHON_STANDARDS_EXAMPLE: &str = r"# Python Coding Standards

## Style Guide
- Follow PEP 8 style conventions
- Use type hints for all function signatures
- Prefer dataclasses or Pydantic for data models

## Imports
- Group imports: stdlib, third-party, local
- Use absolute imports
- Avoid wildcard imports

## Testing
- Write tests for new functionality
- Use pytest fixtures for setup/teardown
- Aim for 80%+ coverage
";

/// Example secret checker script
const SECRET_CHECKER_EXAMPLE: &str = r#"#!/bin/bash
# Check for common secret patterns in staged files
# Returns non-zero if secrets are detected

set -e

# Patterns that indicate potential secrets
PATTERNS=(
    "AKIA[0-9A-Z]{16}"           # AWS Access Key
    "sk-[a-zA-Z0-9]{48}"          # OpenAI API Key
    "ghp_[a-zA-Z0-9]{36}"         # GitHub Personal Access Token
    "password\s*=\s*['\"][^'\"]+['\"]"  # Hardcoded passwords
)

# Check staged files
for pattern in "${PATTERNS[@]}"; do
    if git diff --cached --name-only | xargs grep -lE "$pattern" 2>/dev/null; then
        echo "ERROR: Potential secret detected matching pattern: $pattern"
        exit 1
    fi
done

echo "No secrets detected"
exit 0
"#;

/// Run the init command
pub async fn run(force: bool, with_examples: bool) -> Result<()> {
    let hooks_dir = Path::new(".claude");
    let hooks_file = hooks_dir.join("hooks.yaml");

    // Check if already initialized
    if hooks_file.exists() && !force {
        println!("Configuration already exists at: {}", hooks_file.display());
        println!("Use --force to overwrite existing configuration");
        return Ok(());
    }

    println!("Initializing RuleZ configuration...\n");

    // Create .claude directory
    if !hooks_dir.exists() {
        fs::create_dir_all(hooks_dir).context("Failed to create .claude directory")?;
        println!("✓ Created directory: .claude/");
    }

    // Write hooks.yaml
    fs::write(&hooks_file, DEFAULT_HOOKS_YAML).context("Failed to write hooks.yaml")?;
    println!("✓ Created configuration: .claude/hooks.yaml");

    // Create example files if requested
    if with_examples {
        create_example_files(hooks_dir)?;
    }

    println!("\n{}", "=".repeat(60));
    println!("RuleZ initialized successfully!");
    println!("{}", "=".repeat(60));
    println!("\nNext steps:");
    println!("  1. Review and customize .claude/hooks.yaml");
    println!("  2. Run 'rulez validate' to check configuration");
    println!("  3. Run 'rulez install' to register with Claude Code");
    println!("\nDocumentation:");
    println!("  https://github.com/SpillwaveSolutions/code_agent_context_hooks");

    Ok(())
}

/// Create example context and validator files
fn create_example_files(hooks_dir: &Path) -> Result<()> {
    // Create context directory
    let context_dir = hooks_dir.join("context");
    if !context_dir.exists() {
        fs::create_dir_all(&context_dir).context("Failed to create context directory")?;
        println!("✓ Created directory: .claude/context/");
    }

    // Write Python standards example
    let python_standards = context_dir.join("python-standards.md");
    fs::write(&python_standards, PYTHON_STANDARDS_EXAMPLE)
        .context("Failed to write python-standards.md")?;
    println!("✓ Created example: .claude/context/python-standards.md");

    // Create validators directory
    let validators_dir = hooks_dir.join("validators");
    if !validators_dir.exists() {
        fs::create_dir_all(&validators_dir).context("Failed to create validators directory")?;
        println!("✓ Created directory: .claude/validators/");
    }

    // Write secret checker example
    let secret_checker = validators_dir.join("check-secrets.sh");
    fs::write(&secret_checker, SECRET_CHECKER_EXAMPLE)
        .context("Failed to write check-secrets.sh")?;

    // Make it executable on Unix
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let mut perms = fs::metadata(&secret_checker)?.permissions();
        perms.set_mode(0o755);
        fs::set_permissions(&secret_checker, perms)?;
    }
    println!("✓ Created example: .claude/validators/check-secrets.sh");

    Ok(())
}
