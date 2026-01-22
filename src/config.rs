use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::path::Path;
use std::fs;

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
        }

        Ok(())
    }

    /// Get enabled rules sorted by priority (highest first)
    pub fn enabled_rules(&self) -> Vec<&Rule> {
        let mut rules: Vec<&Rule> = self.rules.iter()
            .filter(|r| r.metadata.as_ref().map_or(true, |m| m.enabled))
            .collect();

        rules.sort_by(|a, b| {
            let a_priority = a.metadata.as_ref().map_or(0, |m| m.priority);
            let b_priority = b.metadata.as_ref().map_or(0, |m| m.priority);
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
            rules: vec![
                Rule {
                    name: "test-rule".to_string(),
                    description: Some("Test rule".to_string()),
                    matchers: crate::models::Matchers {
                        tools: Some(vec!["Bash".to_string()]),
                        extensions: None,
                        directories: None,
                        operations: None,
                        command_match: None,
                    },
                    actions: crate::models::Actions {
                        inject: None,
                        run: None,
                        block: Some(true),
                        block_if_match: None,
                    },
                    metadata: Some(RuleMetadata {
                        priority: 0,
                        timeout: 5,
                        enabled: true,
                    }),
                }
            ],
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
                    matchers: crate::models::Matchers {
                        tools: Some(vec!["Bash".to_string()]),
                        extensions: None,
                        directories: None,
                        operations: None,
                        command_match: None,
                    },
                    actions: crate::models::Actions {
                        inject: None,
                        run: None,
                        block: Some(true),
                        block_if_match: None,
                    },
                    metadata: None,
                },
                Rule {
                    name: "duplicate".to_string(),
                    description: None,
                    matchers: crate::models::Matchers {
                        tools: Some(vec!["Edit".to_string()]),
                        extensions: None,
                        directories: None,
                        operations: None,
                        command_match: None,
                    },
                    actions: crate::models::Actions {
                        inject: None,
                        run: None,
                        block: Some(false),
                        block_if_match: None,
                    },
                    metadata: None,
                }
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
                    matchers: crate::models::Matchers {
                        tools: Some(vec!["Bash".to_string()]),
                        extensions: None,
                        directories: None,
                        operations: None,
                        command_match: None,
                    },
                    actions: crate::models::Actions {
                        inject: None,
                        run: None,
                        block: Some(true),
                        block_if_match: None,
                    },
                    metadata: Some(RuleMetadata {
                        priority: 0,
                        timeout: 5,
                        enabled: true,
                    }),
                },
                Rule {
                    name: "high-priority".to_string(),
                    description: None,
                    matchers: crate::models::Matchers {
                        tools: Some(vec!["Edit".to_string()]),
                        extensions: None,
                        directories: None,
                        operations: None,
                        command_match: None,
                    },
                    actions: crate::models::Actions {
                        inject: None,
                        run: None,
                        block: Some(false),
                        block_if_match: None,
                    },
                    metadata: Some(RuleMetadata {
                        priority: 10,
                        timeout: 5,
                        enabled: true,
                    }),
                }
            ],
            settings: Settings::default(),
        };

        let enabled_rules = config.enabled_rules();
        assert_eq!(enabled_rules[0].name, "high-priority");
        assert_eq!(enabled_rules[1].name, "low-priority");
    }
}