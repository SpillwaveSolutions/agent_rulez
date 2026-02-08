#![allow(clippy::regex_creation_in_loops)]
#![allow(clippy::unnecessary_map_or)]

use anyhow::{Context, Result};
use evalexpr::{build_operator_tree, DefaultNumericTypes};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

use crate::models::Rule;

/// Global CCH settings
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Settings {
    /// Logging verbosity level
    #[serde(default = "default_log_level")]
    pub log_level: String,

    /// Maximum size of injected context in bytes
    #[serde(default = "default_max_context_size")]
    pub max_context_size: usize,

    /// Default script execution timeout in seconds
    #[serde(default = "default_script_timeout")]
    pub script_timeout: u32,

    /// Whether to continue operations on errors
    #[serde(default = "default_fail_open")]
    pub fail_open: bool,

    /// Enable debug logging with full event and rule details
    #[serde(default = "default_debug_logs")]
    pub debug_logs: bool,
}

fn default_log_level() -> String {
    "info".to_string()
}

fn default_max_context_size() -> usize {
    1024 * 1024 // 1MB
}

fn default_script_timeout() -> u32 {
    5
}

fn default_fail_open() -> bool {
    true
}

fn default_debug_logs() -> bool {
    false
}

/// Complete CCH configuration
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Config {
    /// Configuration format version
    pub version: String,

    /// Array of policy rules to enforce
    pub rules: Vec<Rule>,

    /// Global CCH settings
    #[serde(default)]
    pub settings: Settings,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            log_level: default_log_level(),
            max_context_size: default_max_context_size(),
            script_timeout: default_script_timeout(),
            fail_open: default_fail_open(),
            debug_logs: default_debug_logs(),
        }
    }
}

impl Config {
    /// Load configuration from YAML file
    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Self> {
        let content = fs::read_to_string(&path)
            .with_context(|| format!("Failed to read config file: {}", path.as_ref().display()))?;

        let config: Config = serde_yaml::from_str(&content)
            .with_context(|| format!("Failed to parse config file: {}", path.as_ref().display()))?;

        config.validate()?;
        Ok(config)
    }

    /// Load configuration with fallback hierarchy
    pub fn load(project_root: Option<&Path>) -> Result<Self> {
        // Try project-specific config first
        let effective_root = project_root
            .map(|p| p.to_path_buf())
            .or_else(|| std::env::current_dir().ok());

        if let Some(root) = effective_root {
            let project_config = root.join(".claude").join("hooks.yaml");
            if project_config.exists() {
                return Self::from_file(&project_config);
            }
        }

        // Fall back to user-global config
        let home_config = dirs::home_dir()
            .ok_or_else(|| anyhow::anyhow!("Could not determine home directory"))?
            .join(".claude")
            .join("hooks.yaml");

        if home_config.exists() {
            return Self::from_file(&home_config);
        }

        // Return empty config if no files found
        Ok(Self::default())
    }

    /// Validate configuration integrity
    pub fn validate(&self) -> Result<()> {
        // Validate version format
        if !regex::Regex::new(r"^\d+\.\d+$")?.is_match(&self.version) {
            return Err(anyhow::anyhow!("Invalid version format: {}", self.version));
        }

        // Validate rule names are unique
        let mut seen_names = std::collections::HashSet::new();
        for rule in &self.rules {
            if !seen_names.insert(&rule.name) {
                return Err(anyhow::anyhow!("Duplicate rule name: {}", rule.name));
            }

            // Validate rule name format
            if !regex::Regex::new(r"^[a-zA-Z0-9_-]+$")?.is_match(&rule.name) {
                return Err(anyhow::anyhow!("Invalid rule name format: {}", rule.name));
            }

            // Validate enabled_when expression syntax
            if let Some(ref expr) = rule.enabled_when {
                build_operator_tree::<DefaultNumericTypes>(expr).with_context(|| {
                    format!(
                        "Invalid enabled_when expression '{}' in rule '{}': syntax error",
                        expr, rule.name
                    )
                })?;
            }
        }

        Ok(())
    }

    /// Get enabled rules sorted by priority (highest first)
    pub fn enabled_rules(&self) -> Vec<&Rule> {
        let mut rules: Vec<&Rule> = self.rules.iter().filter(|r| r.is_enabled()).collect();

        // Sort by effective priority (higher first)
        // Uses new Phase 2 priority field with fallback to legacy metadata.priority
        rules.sort_by(|a, b| {
            let a_priority = a.effective_priority();
            let b_priority = b.effective_priority();
            b_priority.cmp(&a_priority) // Higher priority first
        });

        rules
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            version: "1.0".to_string(),
            rules: Vec::new(),
            settings: Settings::default(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::RuleMetadata;
    #[allow(unused_imports)]
    use std::io::Write;
    #[allow(unused_imports)]
    use tempfile::NamedTempFile;

    #[test]
    fn test_config_validation() {
        let config = Config {
            version: "1.0".to_string(),
            rules: vec![Rule {
                name: "test-rule".to_string(),
                description: Some("Test rule".to_string()),
                enabled_when: None,
                matchers: crate::models::Matchers {
                    tools: Some(vec!["Bash".to_string()]),
                    extensions: None,
                    directories: None,
                    operations: None,
                    command_match: None,
                },
                actions: crate::models::Actions {
                    inject: None,
                    inject_inline: None,
                    inject_command: None,
                    run: None,
                    block: Some(true),
                    block_if_match: None,
                },
                mode: None,
                priority: None,
                governance: None,
                metadata: Some(RuleMetadata {
                    priority: 0,
                    timeout: 5,
                    enabled: true,
                }),
            }],
            settings: Settings::default(),
        };

        assert!(config.validate().is_ok());
    }

    #[test]
    fn test_duplicate_rule_names() {
        let config = Config {
            version: "1.0".to_string(),
            rules: vec![
                Rule {
                    name: "duplicate".to_string(),
                    description: None,
                    enabled_when: None,
                    matchers: crate::models::Matchers {
                        tools: Some(vec!["Bash".to_string()]),
                        extensions: None,
                        directories: None,
                        operations: None,
                        command_match: None,
                    },
                    actions: crate::models::Actions {
                        inject: None,
                        inject_inline: None,
                        inject_command: None,
                        run: None,
                        block: Some(true),
                        block_if_match: None,
                    },
                    mode: None,
                    priority: None,
                    governance: None,
                    metadata: None,
                },
                Rule {
                    name: "duplicate".to_string(),
                    description: None,
                    enabled_when: None,
                    matchers: crate::models::Matchers {
                        tools: Some(vec!["Edit".to_string()]),
                        extensions: None,
                        directories: None,
                        operations: None,
                        command_match: None,
                    },
                    actions: crate::models::Actions {
                        inject: None,
                        inject_inline: None,
                        inject_command: None,
                        run: None,
                        block: Some(false),
                        block_if_match: None,
                    },
                    mode: None,
                    priority: None,
                    governance: None,
                    metadata: None,
                },
            ],
            settings: Settings::default(),
        };

        assert!(config.validate().is_err());
    }

    #[test]
    fn test_rule_priority_sorting() {
        let config = Config {
            version: "1.0".to_string(),
            rules: vec![
                Rule {
                    name: "low-priority".to_string(),
                    description: None,
                    enabled_when: None,
                    matchers: crate::models::Matchers {
                        tools: Some(vec!["Bash".to_string()]),
                        extensions: None,
                        directories: None,
                        operations: None,
                        command_match: None,
                    },
                    actions: crate::models::Actions {
                        inject: None,
                        inject_inline: None,
                        inject_command: None,
                        run: None,
                        block: Some(true),
                        block_if_match: None,
                    },
                    mode: None,
                    priority: None,
                    governance: None,
                    metadata: Some(RuleMetadata {
                        priority: 0,
                        timeout: 5,
                        enabled: true,
                    }),
                },
                Rule {
                    name: "high-priority".to_string(),
                    description: None,
                    enabled_when: None,
                    matchers: crate::models::Matchers {
                        tools: Some(vec!["Edit".to_string()]),
                        extensions: None,
                        directories: None,
                        operations: None,
                        command_match: None,
                    },
                    actions: crate::models::Actions {
                        inject: None,
                        inject_inline: None,
                        inject_command: None,
                        run: None,
                        block: Some(false),
                        block_if_match: None,
                    },
                    mode: None,
                    priority: None,
                    governance: None,
                    metadata: Some(RuleMetadata {
                        priority: 10,
                        timeout: 5,
                        enabled: true,
                    }),
                },
            ],
            settings: Settings::default(),
        };

        let enabled_rules = config.enabled_rules();
        assert_eq!(enabled_rules[0].name, "high-priority");
        assert_eq!(enabled_rules[1].name, "low-priority");
    }

    // =========================================================================
    // Phase 3: enabled_when Expression Validation Tests
    // =========================================================================

    #[test]
    fn test_enabled_when_valid_expression() {
        // Test that valid enabled_when expressions pass validation
        let config = Config {
            version: "1.0".to_string(),
            rules: vec![Rule {
                name: "valid-expr".to_string(),
                description: None,
                enabled_when: Some(r#"env_CI == "true""#.to_string()),
                matchers: crate::models::Matchers {
                    tools: Some(vec!["Bash".to_string()]),
                    extensions: None,
                    directories: None,
                    operations: None,
                    command_match: None,
                },
                actions: crate::models::Actions {
                    inject: None,
                    inject_inline: None,
                    inject_command: None,
                    run: None,
                    block: Some(true),
                    block_if_match: None,
                },
                mode: None,
                priority: None,
                governance: None,
                metadata: None,
            }],
            settings: Settings::default(),
        };

        assert!(config.validate().is_ok());
    }

    #[test]
    fn test_enabled_when_invalid_expression() {
        // Test that invalid enabled_when expressions fail validation with clear error message
        let config = Config {
            version: "1.0".to_string(),
            rules: vec![Rule {
                name: "invalid-expr".to_string(),
                description: None,
                enabled_when: Some(r#"env_CI == ("true""#.to_string()), // Invalid: unclosed parenthesis
                matchers: crate::models::Matchers {
                    tools: Some(vec!["Bash".to_string()]),
                    extensions: None,
                    directories: None,
                    operations: None,
                    command_match: None,
                },
                actions: crate::models::Actions {
                    inject: None,
                    inject_inline: None,
                    inject_command: None,
                    run: None,
                    block: Some(true),
                    block_if_match: None,
                },
                mode: None,
                priority: None,
                governance: None,
                metadata: None,
            }],
            settings: Settings::default(),
        };

        let result = config.validate();
        assert!(result.is_err());
        let err_msg = result.unwrap_err().to_string();
        // Error should include the rule name and expression
        assert!(
            err_msg.contains("invalid-expr"),
            "Error should contain rule name: {}",
            err_msg
        );
        assert!(
            err_msg.contains("env_CI =="),
            "Error should contain expression: {}",
            err_msg
        );
    }

    #[test]
    fn test_enabled_when_complex_valid_expression() {
        // Test that complex expressions with logical operators validate correctly
        let config = Config {
            version: "1.0".to_string(),
            rules: vec![Rule {
                name: "complex-expr".to_string(),
                description: None,
                enabled_when: Some(r#"env_CI == "true" && tool_name == "Bash""#.to_string()),
                matchers: crate::models::Matchers {
                    tools: Some(vec!["Bash".to_string()]),
                    extensions: None,
                    directories: None,
                    operations: None,
                    command_match: None,
                },
                actions: crate::models::Actions {
                    inject: None,
                    inject_inline: None,
                    inject_command: None,
                    run: None,
                    block: Some(true),
                    block_if_match: None,
                },
                mode: None,
                priority: None,
                governance: None,
                metadata: None,
            }],
            settings: Settings::default(),
        };

        assert!(config.validate().is_ok());
    }
}
