#![allow(clippy::regex_creation_in_loops)]
#![allow(clippy::unnecessary_map_or)]

use anyhow::{Context, Result};
use evalexpr::{build_operator_tree, DefaultNumericTypes};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

use crate::models::{PromptMatch, Rule};

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

            // Validate prompt_match patterns
            if let Some(ref prompt_match) = rule.matchers.prompt_match {
                let patterns = prompt_match.patterns();

                // Reject empty patterns array
                if patterns.is_empty() {
                    return Err(anyhow::anyhow!(
                        "Empty patterns array in prompt_match for rule '{}'",
                        rule.name
                    ));
                }

                // Validate each pattern is a valid regex
                for pattern in patterns {
                    // Extract actual pattern (handle negation and shorthands)
                    let effective_pattern = if let Some(inner) = pattern.strip_prefix("not:") {
                        inner.trim().to_string()
                    } else {
                        pattern.clone()
                    };

                    // Expand shorthands before validation
                    let expanded = PromptMatch::expand_pattern(&effective_pattern);

                    // Apply anchor for full pattern validation
                    let anchored = PromptMatch::apply_anchor(&expanded, prompt_match.anchor());

                    // Validate regex compiles
                    if let Err(e) = regex::Regex::new(&anchored) {
                        return Err(anyhow::anyhow!(
                            "Invalid regex pattern '{}' (expanded to '{}') in prompt_match for rule '{}': {}",
                            pattern, anchored, rule.name, e
                        ));
                    }
                }
            }

            // Validate require_fields paths
            if let Some(ref require_fields) = rule.matchers.require_fields {
                // Reject empty arrays
                if require_fields.is_empty() {
                    return Err(anyhow::anyhow!(
                        "Empty require_fields array for rule '{}'",
                        rule.name
                    ));
                }

                for field_path in require_fields {
                    Self::validate_field_path(field_path, &rule.name, "require_fields")?;
                }
            }

            // Validate field_types paths and type specifiers
            if let Some(ref field_types) = rule.matchers.field_types {
                let valid_types = ["string", "number", "boolean", "array", "object", "any"];

                for (field_path, type_specifier) in field_types {
                    // Validate field path
                    Self::validate_field_path(field_path, &rule.name, "field_types")?;

                    // Validate type specifier
                    if !valid_types.contains(&type_specifier.as_str()) {
                        return Err(anyhow::anyhow!(
                            "Invalid type '{}' for field '{}' in field_types for rule '{}': must be one of string, number, boolean, array, object, any",
                            type_specifier, field_path, rule.name
                        ));
                    }
                }
            }
        }

        Ok(())
    }

    /// Validate field path syntax
    fn validate_field_path(field_path: &str, rule_name: &str, field_name: &str) -> Result<()> {
        // Reject empty strings
        if field_path.is_empty() {
            return Err(anyhow::anyhow!(
                "Invalid field path '' in {} for rule '{}': cannot be empty",
                field_name, rule_name
            ));
        }

        // Reject paths starting with '.'
        if field_path.starts_with('.') {
            return Err(anyhow::anyhow!(
                "Invalid field path '{}' in {} for rule '{}': cannot start with '.'",
                field_path, field_name, rule_name
            ));
        }

        // Reject paths ending with '.'
        if field_path.ends_with('.') {
            return Err(anyhow::anyhow!(
                "Invalid field path '{}' in {} for rule '{}': cannot end with '.'",
                field_path, field_name, rule_name
            ));
        }

        // Reject paths with consecutive dots
        if field_path.contains("..") {
            return Err(anyhow::anyhow!(
                "Invalid field path '{}' in {} for rule '{}': cannot contain consecutive dots",
                field_path, field_name, rule_name
            ));
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
                    prompt_match: None,
                    require_fields: None,
                    field_types: None,
                },
                actions: crate::models::Actions {
                    inject: None,
                    inject_inline: None,
                    inject_command: None,
                    run: None,
                    block: Some(true),
                    block_if_match: None,
                    validate_expr: None,
                    inline_script: None,
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
                        prompt_match: None,
                        require_fields: None,
                        field_types: None,
                    },
                    actions: crate::models::Actions {
                        inject: None,
                        inject_inline: None,
                        inject_command: None,
                        run: None,
                        block: Some(true),
                        block_if_match: None,
                        validate_expr: None,
                        inline_script: None,
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
                        prompt_match: None,
                        require_fields: None,
                        field_types: None,
                    },
                    actions: crate::models::Actions {
                        inject: None,
                        inject_inline: None,
                        inject_command: None,
                        run: None,
                        block: Some(false),
                        block_if_match: None,
                        validate_expr: None,
                        inline_script: None,
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
                        prompt_match: None,
                        require_fields: None,
                        field_types: None,
                    },
                    actions: crate::models::Actions {
                        inject: None,
                        inject_inline: None,
                        inject_command: None,
                        run: None,
                        block: Some(true),
                        block_if_match: None,
                        validate_expr: None,
                        inline_script: None,
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
                        prompt_match: None,
                        require_fields: None,
                        field_types: None,
                    },
                    actions: crate::models::Actions {
                        inject: None,
                        inject_inline: None,
                        inject_command: None,
                        run: None,
                        block: Some(false),
                        block_if_match: None,
                        validate_expr: None,
                        inline_script: None,
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
                    prompt_match: None,
                    require_fields: None,
                    field_types: None,
                },
                actions: crate::models::Actions {
                    inject: None,
                    inject_inline: None,
                    inject_command: None,
                    run: None,
                    block: Some(true),
                    block_if_match: None,
                    validate_expr: None,
                    inline_script: None,
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
                    prompt_match: None,
                    require_fields: None,
                    field_types: None,
                },
                actions: crate::models::Actions {
                    inject: None,
                    inject_inline: None,
                    inject_command: None,
                    run: None,
                    block: Some(true),
                    block_if_match: None,
                    validate_expr: None,
                    inline_script: None,
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
                    prompt_match: None,
                    require_fields: None,
                    field_types: None,
                },
                actions: crate::models::Actions {
                    inject: None,
                    inject_inline: None,
                    inject_command: None,
                    run: None,
                    block: Some(true),
                    block_if_match: None,
                    validate_expr: None,
                    inline_script: None,
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

    // =========================================================================
    // Phase 4: prompt_match Validation Tests
    // =========================================================================

    #[test]
    fn test_prompt_match_valid_simple_array() {
        let config = Config {
            version: "1.0".to_string(),
            rules: vec![Rule {
                name: "valid-prompt".to_string(),
                description: None,
                enabled_when: None,
                matchers: crate::models::Matchers {
                    tools: None,
                    extensions: None,
                    directories: None,
                    operations: Some(vec!["UserPromptSubmit".to_string()]),
                    command_match: None,
                    prompt_match: Some(crate::models::PromptMatch::Simple(vec![
                        "delete".to_string(),
                        "drop database".to_string(),
                    ])),
                    require_fields: None,
                    field_types: None,
                },
                actions: crate::models::Actions {
                    inject: None,
                    inject_inline: None,
                    inject_command: None,
                    run: None,
                    block: Some(true),
                    block_if_match: None,
                    validate_expr: None,
                    inline_script: None,
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
    fn test_prompt_match_valid_complex_object() {
        let config = Config {
            version: "1.0".to_string(),
            rules: vec![Rule {
                name: "valid-prompt-complex".to_string(),
                description: None,
                enabled_when: None,
                matchers: crate::models::Matchers {
                    tools: None,
                    extensions: None,
                    directories: None,
                    operations: Some(vec!["UserPromptSubmit".to_string()]),
                    command_match: None,
                    prompt_match: Some(crate::models::PromptMatch::Complex {
                        patterns: vec!["test".to_string(), "staging".to_string()],
                        mode: crate::models::MatchMode::All,
                        case_insensitive: true,
                        anchor: Some(crate::models::Anchor::Contains),
                    }),
                    require_fields: None,
                    field_types: None,
                },
                actions: crate::models::Actions {
                    inject: None,
                    inject_inline: None,
                    inject_command: None,
                    run: None,
                    block: Some(true),
                    block_if_match: None,
                    validate_expr: None,
                    inline_script: None,
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
    fn test_prompt_match_empty_patterns_rejected() {
        let config = Config {
            version: "1.0".to_string(),
            rules: vec![Rule {
                name: "empty-patterns".to_string(),
                description: None,
                enabled_when: None,
                matchers: crate::models::Matchers {
                    tools: None,
                    extensions: None,
                    directories: None,
                    operations: None,
                    command_match: None,
                    prompt_match: Some(crate::models::PromptMatch::Simple(vec![])),
                    require_fields: None,
                    field_types: None,
                },
                actions: crate::models::Actions {
                    inject: None,
                    inject_inline: None,
                    inject_command: None,
                    run: None,
                    block: Some(true),
                    block_if_match: None,
                    validate_expr: None,
                    inline_script: None,
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
        assert!(err_msg.contains("Empty patterns"));
        assert!(err_msg.contains("empty-patterns"));
    }

    #[test]
    fn test_prompt_match_invalid_regex_rejected() {
        let config = Config {
            version: "1.0".to_string(),
            rules: vec![Rule {
                name: "invalid-regex".to_string(),
                description: None,
                enabled_when: None,
                matchers: crate::models::Matchers {
                    tools: None,
                    extensions: None,
                    directories: None,
                    operations: None,
                    command_match: None,
                    prompt_match: Some(crate::models::PromptMatch::Simple(vec![
                        "[invalid(regex".to_string(), // Unclosed brackets
                    ])),
                    require_fields: None,
                    field_types: None,
                },
                actions: crate::models::Actions {
                    inject: None,
                    inject_inline: None,
                    inject_command: None,
                    run: None,
                    block: Some(true),
                    block_if_match: None,
                    validate_expr: None,
                    inline_script: None,
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
        assert!(err_msg.contains("Invalid regex pattern"));
        assert!(err_msg.contains("invalid-regex"));
    }

    #[test]
    fn test_prompt_match_shorthand_valid() {
        let config = Config {
            version: "1.0".to_string(),
            rules: vec![Rule {
                name: "shorthand-valid".to_string(),
                description: None,
                enabled_when: None,
                matchers: crate::models::Matchers {
                    tools: None,
                    extensions: None,
                    directories: None,
                    operations: None,
                    command_match: None,
                    prompt_match: Some(crate::models::PromptMatch::Simple(vec![
                        "contains_word:delete".to_string(),
                        "not:review".to_string(),
                    ])),
                    require_fields: None,
                    field_types: None,
                },
                actions: crate::models::Actions {
                    inject: None,
                    inject_inline: None,
                    inject_command: None,
                    run: None,
                    block: Some(true),
                    block_if_match: None,
                    validate_expr: None,
                    inline_script: None,
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

    // =========================================================================
    // Phase 5: Field Validation Tests
    // =========================================================================

    #[test]
    fn test_require_fields_valid_simple() {
        let config = Config {
            version: "1.0".to_string(),
            rules: vec![Rule {
                name: "test-require-simple".to_string(),
                description: None,
                enabled_when: None,
                matchers: crate::models::Matchers {
                    tools: None,
                    extensions: None,
                    directories: None,
                    operations: None,
                    command_match: None,
                    prompt_match: None,
                    require_fields: Some(vec!["file_path".to_string(), "content".to_string()]),
                    field_types: None,
                },
                actions: crate::models::Actions {
                    inject: None,
                    inject_inline: None,
                    inject_command: None,
                    run: None,
                    block: None,
                    block_if_match: None,
                    validate_expr: None,
                    inline_script: None,
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
    fn test_require_fields_valid_nested() {
        let config = Config {
            version: "1.0".to_string(),
            rules: vec![Rule {
                name: "test-require-nested".to_string(),
                description: None,
                enabled_when: None,
                matchers: crate::models::Matchers {
                    tools: None,
                    extensions: None,
                    directories: None,
                    operations: None,
                    command_match: None,
                    prompt_match: None,
                    require_fields: Some(vec![
                        "user.name".to_string(),
                        "input.data.value".to_string(),
                    ]),
                    field_types: None,
                },
                actions: crate::models::Actions {
                    inject: None,
                    inject_inline: None,
                    inject_command: None,
                    run: None,
                    block: None,
                    block_if_match: None,
                    validate_expr: None,
                    inline_script: None,
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
    fn test_require_fields_empty_array_rejected() {
        let config = Config {
            version: "1.0".to_string(),
            rules: vec![Rule {
                name: "test-empty-array".to_string(),
                description: None,
                enabled_when: None,
                matchers: crate::models::Matchers {
                    tools: None,
                    extensions: None,
                    directories: None,
                    operations: None,
                    command_match: None,
                    prompt_match: None,
                    require_fields: Some(vec![]),
                    field_types: None,
                },
                actions: crate::models::Actions {
                    inject: None,
                    inject_inline: None,
                    inject_command: None,
                    run: None,
                    block: None,
                    block_if_match: None,
                    validate_expr: None,
                    inline_script: None,
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
        assert!(result.unwrap_err().to_string().contains("Empty require_fields"));
    }

    #[test]
    fn test_require_fields_empty_string_rejected() {
        let config = Config {
            version: "1.0".to_string(),
            rules: vec![Rule {
                name: "test-empty-string".to_string(),
                description: None,
                enabled_when: None,
                matchers: crate::models::Matchers {
                    tools: None,
                    extensions: None,
                    directories: None,
                    operations: None,
                    command_match: None,
                    prompt_match: None,
                    require_fields: Some(vec!["".to_string()]),
                    field_types: None,
                },
                actions: crate::models::Actions {
                    inject: None,
                    inject_inline: None,
                    inject_command: None,
                    run: None,
                    block: None,
                    block_if_match: None,
                    validate_expr: None,
                    inline_script: None,
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
        assert!(result.unwrap_err().to_string().contains("cannot be empty"));
    }

    #[test]
    fn test_require_fields_leading_dot_rejected() {
        let config = Config {
            version: "1.0".to_string(),
            rules: vec![Rule {
                name: "test-leading-dot".to_string(),
                description: None,
                enabled_when: None,
                matchers: crate::models::Matchers {
                    tools: None,
                    extensions: None,
                    directories: None,
                    operations: None,
                    command_match: None,
                    prompt_match: None,
                    require_fields: Some(vec![".name".to_string()]),
                    field_types: None,
                },
                actions: crate::models::Actions {
                    inject: None,
                    inject_inline: None,
                    inject_command: None,
                    run: None,
                    block: None,
                    block_if_match: None,
                    validate_expr: None,
                    inline_script: None,
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
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("cannot start with '.'"));
    }

    #[test]
    fn test_require_fields_trailing_dot_rejected() {
        let config = Config {
            version: "1.0".to_string(),
            rules: vec![Rule {
                name: "test-trailing-dot".to_string(),
                description: None,
                enabled_when: None,
                matchers: crate::models::Matchers {
                    tools: None,
                    extensions: None,
                    directories: None,
                    operations: None,
                    command_match: None,
                    prompt_match: None,
                    require_fields: Some(vec!["name.".to_string()]),
                    field_types: None,
                },
                actions: crate::models::Actions {
                    inject: None,
                    inject_inline: None,
                    inject_command: None,
                    run: None,
                    block: None,
                    block_if_match: None,
                    validate_expr: None,
                    inline_script: None,
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
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("cannot end with '.'"));
    }

    #[test]
    fn test_require_fields_consecutive_dots_rejected() {
        let config = Config {
            version: "1.0".to_string(),
            rules: vec![Rule {
                name: "test-consecutive-dots".to_string(),
                description: None,
                enabled_when: None,
                matchers: crate::models::Matchers {
                    tools: None,
                    extensions: None,
                    directories: None,
                    operations: None,
                    command_match: None,
                    prompt_match: None,
                    require_fields: Some(vec!["name..field".to_string()]),
                    field_types: None,
                },
                actions: crate::models::Actions {
                    inject: None,
                    inject_inline: None,
                    inject_command: None,
                    run: None,
                    block: None,
                    block_if_match: None,
                    validate_expr: None,
                    inline_script: None,
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
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("cannot contain consecutive dots"));
    }

    #[test]
    fn test_field_types_valid() {
        let config = Config {
            version: "1.0".to_string(),
            rules: vec![Rule {
                name: "test-field-types-valid".to_string(),
                description: None,
                enabled_when: None,
                matchers: crate::models::Matchers {
                    tools: None,
                    extensions: None,
                    directories: None,
                    operations: None,
                    command_match: None,
                    prompt_match: None,
                    require_fields: None,
                    field_types: Some({
                        let mut map = std::collections::HashMap::new();
                        map.insert("file_path".to_string(), "string".to_string());
                        map.insert("count".to_string(), "number".to_string());
                        map
                    }),
                },
                actions: crate::models::Actions {
                    inject: None,
                    inject_inline: None,
                    inject_command: None,
                    run: None,
                    block: None,
                    block_if_match: None,
                    validate_expr: None,
                    inline_script: None,
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
    fn test_field_types_invalid_type_rejected() {
        let config = Config {
            version: "1.0".to_string(),
            rules: vec![Rule {
                name: "test-invalid-type".to_string(),
                description: None,
                enabled_when: None,
                matchers: crate::models::Matchers {
                    tools: None,
                    extensions: None,
                    directories: None,
                    operations: None,
                    command_match: None,
                    prompt_match: None,
                    require_fields: None,
                    field_types: Some({
                        let mut map = std::collections::HashMap::new();
                        map.insert("count".to_string(), "integer".to_string());
                        map
                    }),
                },
                actions: crate::models::Actions {
                    inject: None,
                    inject_inline: None,
                    inject_command: None,
                    run: None,
                    block: None,
                    block_if_match: None,
                    validate_expr: None,
                    inline_script: None,
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
        assert!(result.unwrap_err().to_string().contains("Invalid type 'integer'"));
    }

    #[test]
    fn test_field_types_invalid_path_rejected() {
        let config = Config {
            version: "1.0".to_string(),
            rules: vec![Rule {
                name: "test-invalid-path".to_string(),
                description: None,
                enabled_when: None,
                matchers: crate::models::Matchers {
                    tools: None,
                    extensions: None,
                    directories: None,
                    operations: None,
                    command_match: None,
                    prompt_match: None,
                    require_fields: None,
                    field_types: Some({
                        let mut map = std::collections::HashMap::new();
                        map.insert(".name".to_string(), "string".to_string());
                        map
                    }),
                },
                actions: crate::models::Actions {
                    inject: None,
                    inject_inline: None,
                    inject_command: None,
                    run: None,
                    block: None,
                    block_if_match: None,
                    validate_expr: None,
                    inline_script: None,
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
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("cannot start with '.'"));
    }

    #[test]
    fn test_field_types_any_type_accepted() {
        let config = Config {
            version: "1.0".to_string(),
            rules: vec![Rule {
                name: "test-any-type".to_string(),
                description: None,
                enabled_when: None,
                matchers: crate::models::Matchers {
                    tools: None,
                    extensions: None,
                    directories: None,
                    operations: None,
                    command_match: None,
                    prompt_match: None,
                    require_fields: None,
                    field_types: Some({
                        let mut map = std::collections::HashMap::new();
                        map.insert("data".to_string(), "any".to_string());
                        map
                    }),
                },
                actions: crate::models::Actions {
                    inject: None,
                    inject_inline: None,
                    inject_command: None,
                    run: None,
                    block: None,
                    block_if_match: None,
                    validate_expr: None,
                    inline_script: None,
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
