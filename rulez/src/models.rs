use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

// =============================================================================
// Phase 2 Governance Types
// =============================================================================

/// Policy enforcement mode for rules
///
/// Controls how a rule behaves when it matches:
/// - `Enforce`: Normal behavior - blocks, injects, or runs validators
/// - `Warn`: Never blocks, injects warning context instead
/// - `Audit`: Logs only, no blocking or injection
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum PolicyMode {
    /// Normal enforcement - blocks, injects, or runs validators
    #[default]
    Enforce,
    /// Never blocks, injects warning context instead
    Warn,
    /// Logs only, no blocking or injection
    Audit,
}

impl std::fmt::Display for PolicyMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PolicyMode::Enforce => write!(f, "enforce"),
            PolicyMode::Warn => write!(f, "warn"),
            PolicyMode::Audit => write!(f, "audit"),
        }
    }
}

/// Confidence level for rule metadata
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Confidence {
    High,
    Medium,
    Low,
}

impl std::fmt::Display for Confidence {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Confidence::High => write!(f, "high"),
            Confidence::Medium => write!(f, "medium"),
            Confidence::Low => write!(f, "low"),
        }
    }
}

/// Decision outcome for logging
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Decision {
    /// Operation was allowed to proceed
    Allowed,
    /// Operation was blocked
    Blocked,
    /// Warning was issued but operation proceeded
    Warned,
    /// Rule matched but only logged (audit mode)
    Audited,
}

impl std::fmt::Display for Decision {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Decision::Allowed => write!(f, "allowed"),
            Decision::Blocked => write!(f, "blocked"),
            Decision::Warned => write!(f, "warned"),
            Decision::Audited => write!(f, "audited"),
        }
    }
}

impl std::str::FromStr for Decision {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "allowed" => Ok(Decision::Allowed),
            "blocked" => Ok(Decision::Blocked),
            "warned" => Ok(Decision::Warned),
            "audited" => Ok(Decision::Audited),
            _ => Err(format!("Invalid decision: {}", s)),
        }
    }
}

// =============================================================================
// Phase 2.4: Trust Levels
// =============================================================================

/// Trust level for validator scripts
///
/// Indicates the provenance and verification status of a validator script.
/// This is informational in v1.1 - enforcement planned for future versions.
///
/// # Trust Levels
/// - `Local`: Script exists in the local project repository
/// - `Verified`: Script has been cryptographically verified (future)
/// - `Untrusted`: Script from external/untrusted source
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum TrustLevel {
    /// Script is local to the project
    #[default]
    Local,
    /// Script has been verified (cryptographic verification - future)
    Verified,
    /// Script from external or untrusted source
    Untrusted,
}

impl std::fmt::Display for TrustLevel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TrustLevel::Local => write!(f, "local"),
            TrustLevel::Verified => write!(f, "verified"),
            TrustLevel::Untrusted => write!(f, "untrusted"),
        }
    }
}

/// Extended run action configuration supporting trust levels
///
/// Supports two YAML formats for backward compatibility:
/// ```yaml
/// # Simple format (existing)
/// actions:
///   run: .claude/validators/check.py
///
/// # Extended format (new)
/// actions:
///   run:
///     script: .claude/validators/check.py
///     trust: local
/// ```
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum RunAction {
    /// Simple string format: just the script path
    Simple(String),
    /// Extended object format with trust level
    Extended {
        /// Path to the validator script
        script: String,
        /// Trust level for the script
        #[serde(skip_serializing_if = "Option::is_none")]
        trust: Option<TrustLevel>,
    },
}

impl RunAction {
    /// Get the script path regardless of format
    pub fn script_path(&self) -> &str {
        match self {
            RunAction::Simple(path) => path,
            RunAction::Extended { script, .. } => script,
        }
    }

    /// Get the trust level (defaults to Local if not specified)
    pub fn trust_level(&self) -> TrustLevel {
        match self {
            RunAction::Simple(_) => TrustLevel::Local,
            RunAction::Extended { trust, .. } => trust.unwrap_or(TrustLevel::Local),
        }
    }
}

/// Governance metadata for rules - provenance and documentation
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
pub struct GovernanceMetadata {
    /// Who authored this rule
    #[serde(skip_serializing_if = "Option::is_none")]
    pub author: Option<String>,

    /// Source that created this rule (e.g., "react-skill@2.1.0")
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_by: Option<String>,

    /// Why this rule exists
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,

    /// Confidence level in this rule
    #[serde(skip_serializing_if = "Option::is_none")]
    pub confidence: Option<Confidence>,

    /// When this rule was last reviewed (ISO 8601 date)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_reviewed: Option<String>,

    /// Related ticket or issue reference
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ticket: Option<String>,

    /// Tags for categorization
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,
}

// =============================================================================
// Core Rule Types
// =============================================================================

/// Configuration entry defining policy enforcement logic
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Rule {
    /// Unique identifier for the rule
    pub name: String,

    /// Human-readable explanation
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// Conditions that trigger the rule
    pub matchers: Matchers,

    /// Actions to take when rule matches
    pub actions: Actions,

    // === Phase 2 Governance Fields ===
    /// Policy enforcement mode (enforce, warn, audit)
    /// Default: enforce (current behavior)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mode: Option<PolicyMode>,

    /// Rule evaluation priority (higher numbers run first)
    /// Default: 0
    #[serde(skip_serializing_if = "Option::is_none")]
    pub priority: Option<i32>,

    /// Governance metadata (provenance, documentation)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub governance: Option<GovernanceMetadata>,

    /// Legacy metadata field (for backward compatibility)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<RuleMetadata>,
}

/// Conditions that trigger a rule
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Matchers {
    /// Tool names to match (e.g., ["Bash", "Edit"])
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tools: Option<Vec<String>>,

    /// File extensions to match (e.g., [".rs", ".ts"])
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extensions: Option<Vec<String>>,

    /// Directory patterns to match (e.g., ["src/**", "tests/**"])
    #[serde(skip_serializing_if = "Option::is_none")]
    pub directories: Option<Vec<String>>,

    /// Operation types to match
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operations: Option<Vec<String>>,

    /// Regex pattern for command matching
    #[serde(skip_serializing_if = "Option::is_none")]
    pub command_match: Option<String>,
}

/// Actions to take when rule matches
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Actions {
    /// Path to context file to inject
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inject: Option<String>,

    /// Inline markdown content to inject directly (no file read)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inject_inline: Option<String>,

    /// Validator script to execute (supports string or object format)
    ///
    /// Supports two formats for backward compatibility:
    /// ```yaml
    /// # Simple format (existing)
    /// run: .claude/validators/check.py
    ///
    /// # Extended format with trust level (new)
    /// run:
    ///   script: .claude/validators/check.py
    ///   trust: local
    /// ```
    #[serde(skip_serializing_if = "Option::is_none")]
    pub run: Option<RunAction>,

    /// Whether to block the operation
    #[serde(skip_serializing_if = "Option::is_none")]
    pub block: Option<bool>,

    /// Regex pattern for conditional blocking
    #[serde(skip_serializing_if = "Option::is_none")]
    pub block_if_match: Option<String>,
}

impl Actions {
    /// Get the script path from run action (if present)
    pub fn script_path(&self) -> Option<&str> {
        self.run.as_ref().map(|r| r.script_path())
    }

    /// Get the trust level from run action (defaults to Local)
    pub fn trust_level(&self) -> Option<TrustLevel> {
        self.run.as_ref().map(|r| r.trust_level())
    }
}

/// Additional rule metadata
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct RuleMetadata {
    /// Rule evaluation order (higher numbers = higher priority)
    #[serde(default)]
    pub priority: i32,

    /// Script execution timeout in seconds
    #[serde(default = "default_timeout")]
    pub timeout: u32,

    /// Whether this rule is enabled
    #[serde(default = "default_enabled")]
    pub enabled: bool,
}

#[cfg(test)]
mod governance_tests {
    use super::*;

    // =========================================================================
    // PolicyMode Tests
    // =========================================================================

    #[test]
    fn test_policy_mode_default() {
        let mode = PolicyMode::default();
        assert_eq!(mode, PolicyMode::Enforce);
    }

    #[test]
    fn test_policy_mode_deserialize_lowercase() {
        let enforce: PolicyMode = serde_json::from_str(r#""enforce""#).unwrap();
        let warn: PolicyMode = serde_json::from_str(r#""warn""#).unwrap();
        let audit: PolicyMode = serde_json::from_str(r#""audit""#).unwrap();

        assert_eq!(enforce, PolicyMode::Enforce);
        assert_eq!(warn, PolicyMode::Warn);
        assert_eq!(audit, PolicyMode::Audit);
    }

    #[test]
    fn test_policy_mode_serialize() {
        assert_eq!(
            serde_json::to_string(&PolicyMode::Enforce).unwrap(),
            r#""enforce""#
        );
        assert_eq!(
            serde_json::to_string(&PolicyMode::Warn).unwrap(),
            r#""warn""#
        );
        assert_eq!(
            serde_json::to_string(&PolicyMode::Audit).unwrap(),
            r#""audit""#
        );
    }

    #[test]
    fn test_policy_mode_display() {
        assert_eq!(format!("{}", PolicyMode::Enforce), "enforce");
        assert_eq!(format!("{}", PolicyMode::Warn), "warn");
        assert_eq!(format!("{}", PolicyMode::Audit), "audit");
    }

    // =========================================================================
    // Confidence Tests
    // =========================================================================

    #[test]
    fn test_confidence_deserialize() {
        let high: Confidence = serde_json::from_str(r#""high""#).unwrap();
        let medium: Confidence = serde_json::from_str(r#""medium""#).unwrap();
        let low: Confidence = serde_json::from_str(r#""low""#).unwrap();

        assert_eq!(high, Confidence::High);
        assert_eq!(medium, Confidence::Medium);
        assert_eq!(low, Confidence::Low);
    }

    #[test]
    fn test_confidence_display() {
        assert_eq!(format!("{}", Confidence::High), "high");
        assert_eq!(format!("{}", Confidence::Medium), "medium");
        assert_eq!(format!("{}", Confidence::Low), "low");
    }

    // =========================================================================
    // Decision Tests
    // =========================================================================

    #[test]
    fn test_decision_serialize() {
        assert_eq!(
            serde_json::to_string(&Decision::Allowed).unwrap(),
            r#""allowed""#
        );
        assert_eq!(
            serde_json::to_string(&Decision::Blocked).unwrap(),
            r#""blocked""#
        );
        assert_eq!(
            serde_json::to_string(&Decision::Warned).unwrap(),
            r#""warned""#
        );
        assert_eq!(
            serde_json::to_string(&Decision::Audited).unwrap(),
            r#""audited""#
        );
    }

    #[test]
    fn test_decision_display() {
        assert_eq!(format!("{}", Decision::Allowed), "allowed");
        assert_eq!(format!("{}", Decision::Blocked), "blocked");
        assert_eq!(format!("{}", Decision::Warned), "warned");
        assert_eq!(format!("{}", Decision::Audited), "audited");
    }

    #[test]
    fn test_decision_from_str() {
        assert_eq!("allowed".parse::<Decision>().unwrap(), Decision::Allowed);
        assert_eq!("blocked".parse::<Decision>().unwrap(), Decision::Blocked);
        assert_eq!("warned".parse::<Decision>().unwrap(), Decision::Warned);
        assert_eq!("audited".parse::<Decision>().unwrap(), Decision::Audited);
        // Case insensitive
        assert_eq!("ALLOWED".parse::<Decision>().unwrap(), Decision::Allowed);
        assert_eq!("Blocked".parse::<Decision>().unwrap(), Decision::Blocked);
        // Invalid value
        assert!("invalid".parse::<Decision>().is_err());
    }

    // =========================================================================
    // TrustLevel Tests
    // =========================================================================

    #[test]
    fn test_trust_level_default() {
        assert_eq!(TrustLevel::default(), TrustLevel::Local);
    }

    #[test]
    fn test_trust_level_serialize() {
        assert_eq!(
            serde_json::to_string(&TrustLevel::Local).unwrap(),
            r#""local""#
        );
        assert_eq!(
            serde_json::to_string(&TrustLevel::Verified).unwrap(),
            r#""verified""#
        );
        assert_eq!(
            serde_json::to_string(&TrustLevel::Untrusted).unwrap(),
            r#""untrusted""#
        );
    }

    #[test]
    fn test_trust_level_deserialize() {
        let local: TrustLevel = serde_json::from_str(r#""local""#).unwrap();
        let verified: TrustLevel = serde_json::from_str(r#""verified""#).unwrap();
        let untrusted: TrustLevel = serde_json::from_str(r#""untrusted""#).unwrap();

        assert_eq!(local, TrustLevel::Local);
        assert_eq!(verified, TrustLevel::Verified);
        assert_eq!(untrusted, TrustLevel::Untrusted);
    }

    #[test]
    fn test_trust_level_display() {
        assert_eq!(format!("{}", TrustLevel::Local), "local");
        assert_eq!(format!("{}", TrustLevel::Verified), "verified");
        assert_eq!(format!("{}", TrustLevel::Untrusted), "untrusted");
    }

    // =========================================================================
    // RunAction Tests
    // =========================================================================

    #[test]
    fn test_run_action_simple_string() {
        let yaml = r#"".claude/validators/check.py""#;
        let action: RunAction = serde_json::from_str(yaml).unwrap();
        assert_eq!(action.script_path(), ".claude/validators/check.py");
        assert_eq!(action.trust_level(), TrustLevel::Local); // Default
    }

    #[test]
    fn test_run_action_extended_with_trust() {
        let yaml = r"
script: .claude/validators/check.py
trust: verified
";
        let action: RunAction = serde_yaml::from_str(yaml).unwrap();
        assert_eq!(action.script_path(), ".claude/validators/check.py");
        assert_eq!(action.trust_level(), TrustLevel::Verified);
    }

    #[test]
    fn test_run_action_extended_without_trust() {
        let yaml = r"
script: .claude/validators/check.py
";
        let action: RunAction = serde_yaml::from_str(yaml).unwrap();
        assert_eq!(action.script_path(), ".claude/validators/check.py");
        assert_eq!(action.trust_level(), TrustLevel::Local); // Default
    }

    #[test]
    fn test_actions_with_run_simple() {
        let yaml = r"
run: .claude/validators/test.sh
";
        let actions: Actions = serde_yaml::from_str(yaml).unwrap();
        assert_eq!(actions.script_path(), Some(".claude/validators/test.sh"));
        assert_eq!(actions.trust_level(), Some(TrustLevel::Local));
    }

    #[test]
    fn test_actions_with_run_extended() {
        let yaml = r"
run:
  script: .claude/validators/test.sh
  trust: untrusted
";
        let actions: Actions = serde_yaml::from_str(yaml).unwrap();
        assert_eq!(actions.script_path(), Some(".claude/validators/test.sh"));
        assert_eq!(actions.trust_level(), Some(TrustLevel::Untrusted));
    }

    #[test]
    fn test_actions_without_run() {
        let yaml = r"
block: true
";
        let actions: Actions = serde_yaml::from_str(yaml).unwrap();
        assert_eq!(actions.script_path(), None);
        assert_eq!(actions.trust_level(), None);
    }

    // =========================================================================
    // GovernanceMetadata Tests
    // =========================================================================

    #[test]
    fn test_governance_metadata_default() {
        let meta = GovernanceMetadata::default();
        assert!(meta.author.is_none());
        assert!(meta.created_by.is_none());
        assert!(meta.reason.is_none());
        assert!(meta.confidence.is_none());
        assert!(meta.last_reviewed.is_none());
        assert!(meta.ticket.is_none());
        assert!(meta.tags.is_none());
    }

    #[test]
    fn test_governance_metadata_deserialize_full() {
        let yaml = r"
author: security-team
created_by: aws-cdk-skill@1.2.0
reason: Enforce infrastructure coding standards
confidence: high
last_reviewed: '2025-01-21'
ticket: PLAT-3421
tags:
  - security
  - infra
  - compliance
";
        let meta: GovernanceMetadata = serde_yaml::from_str(yaml).unwrap();
        assert_eq!(meta.author, Some("security-team".to_string()));
        assert_eq!(meta.created_by, Some("aws-cdk-skill@1.2.0".to_string()));
        assert_eq!(
            meta.reason,
            Some("Enforce infrastructure coding standards".to_string())
        );
        assert_eq!(meta.confidence, Some(Confidence::High));
        assert_eq!(meta.last_reviewed, Some("2025-01-21".to_string()));
        assert_eq!(meta.ticket, Some("PLAT-3421".to_string()));
        assert_eq!(
            meta.tags,
            Some(vec![
                "security".to_string(),
                "infra".to_string(),
                "compliance".to_string()
            ])
        );
    }

    #[test]
    fn test_governance_metadata_deserialize_partial() {
        let yaml = r"
author: dev-team
reason: Code quality
";
        let meta: GovernanceMetadata = serde_yaml::from_str(yaml).unwrap();
        assert_eq!(meta.author, Some("dev-team".to_string()));
        assert_eq!(meta.reason, Some("Code quality".to_string()));
        assert!(meta.created_by.is_none());
        assert!(meta.confidence.is_none());
    }

    // =========================================================================
    // Rule Governance Field Tests
    // =========================================================================

    #[test]
    fn test_rule_effective_mode_default() {
        let rule = Rule {
            name: "test".to_string(),
            description: None,
            matchers: Matchers {
                tools: None,
                extensions: None,
                directories: None,
                operations: None,
                command_match: None,
            },
            actions: Actions {
                inject: None,
                inject_inline: None,
                run: None,
                block: None,
                block_if_match: None,
            },
            mode: None,
            priority: None,
            governance: None,
            metadata: None,
        };
        assert_eq!(rule.effective_mode(), PolicyMode::Enforce);
    }

    #[test]
    fn test_rule_effective_mode_explicit() {
        let rule = Rule {
            name: "test".to_string(),
            description: None,
            matchers: Matchers {
                tools: None,
                extensions: None,
                directories: None,
                operations: None,
                command_match: None,
            },
            actions: Actions {
                inject: None,
                inject_inline: None,
                run: None,
                block: None,
                block_if_match: None,
            },
            mode: Some(PolicyMode::Audit),
            priority: None,
            governance: None,
            metadata: None,
        };
        assert_eq!(rule.effective_mode(), PolicyMode::Audit);
    }

    #[test]
    fn test_rule_effective_priority_default() {
        let rule = Rule {
            name: "test".to_string(),
            description: None,
            matchers: Matchers {
                tools: None,
                extensions: None,
                directories: None,
                operations: None,
                command_match: None,
            },
            actions: Actions {
                inject: None,
                inject_inline: None,
                run: None,
                block: None,
                block_if_match: None,
            },
            mode: None,
            priority: None,
            governance: None,
            metadata: None,
        };
        assert_eq!(rule.effective_priority(), 0);
    }

    #[test]
    fn test_rule_effective_priority_explicit() {
        let rule = Rule {
            name: "test".to_string(),
            description: None,
            matchers: Matchers {
                tools: None,
                extensions: None,
                directories: None,
                operations: None,
                command_match: None,
            },
            actions: Actions {
                inject: None,
                inject_inline: None,
                run: None,
                block: None,
                block_if_match: None,
            },
            mode: None,
            priority: Some(100),
            governance: None,
            metadata: None,
        };
        assert_eq!(rule.effective_priority(), 100);
    }

    #[test]
    fn test_rule_effective_priority_from_legacy_metadata() {
        let rule = Rule {
            name: "test".to_string(),
            description: None,
            matchers: Matchers {
                tools: None,
                extensions: None,
                directories: None,
                operations: None,
                command_match: None,
            },
            actions: Actions {
                inject: None,
                inject_inline: None,
                run: None,
                block: None,
                block_if_match: None,
            },
            mode: None,
            priority: None,
            governance: None,
            metadata: Some(RuleMetadata {
                priority: 50,
                timeout: 5,
                enabled: true,
            }),
        };
        assert_eq!(rule.effective_priority(), 50);
    }

    #[test]
    fn test_rule_new_priority_takes_precedence() {
        let rule = Rule {
            name: "test".to_string(),
            description: None,
            matchers: Matchers {
                tools: None,
                extensions: None,
                directories: None,
                operations: None,
                command_match: None,
            },
            actions: Actions {
                inject: None,
                inject_inline: None,
                run: None,
                block: None,
                block_if_match: None,
            },
            mode: None,
            priority: Some(100), // New field takes precedence
            governance: None,
            metadata: Some(RuleMetadata {
                priority: 50, // Legacy field
                timeout: 5,
                enabled: true,
            }),
        };
        assert_eq!(rule.effective_priority(), 100);
    }

    // =========================================================================
    // Priority Sorting Tests
    // =========================================================================

    #[test]
    fn test_sort_rules_by_priority() {
        let mut rules = vec![
            create_test_rule("low", 0),
            create_test_rule("high", 100),
            create_test_rule("medium", 50),
        ];

        sort_rules_by_priority(&mut rules);

        assert_eq!(rules[0].name, "high");
        assert_eq!(rules[1].name, "medium");
        assert_eq!(rules[2].name, "low");
    }

    #[test]
    fn test_sort_rules_stable_for_same_priority() {
        let mut rules = vec![
            create_test_rule("first", 0),
            create_test_rule("second", 0),
            create_test_rule("third", 0),
        ];

        sort_rules_by_priority(&mut rules);

        // Stable sort preserves original order for same priority
        assert_eq!(rules[0].name, "first");
        assert_eq!(rules[1].name, "second");
        assert_eq!(rules[2].name, "third");
    }

    #[test]
    fn test_sort_rules_mixed_priorities() {
        let mut rules = vec![
            create_test_rule("low", 0),
            create_test_rule("very-high", 200),
            create_test_rule("medium-1", 50),
            create_test_rule("medium-2", 50),
            create_test_rule("high", 100),
        ];

        sort_rules_by_priority(&mut rules);

        assert_eq!(rules[0].name, "very-high");
        assert_eq!(rules[1].name, "high");
        // medium-1 and medium-2 preserve relative order
        assert_eq!(rules[2].name, "medium-1");
        assert_eq!(rules[3].name, "medium-2");
        assert_eq!(rules[4].name, "low");
    }

    fn create_test_rule(name: &str, priority: i32) -> Rule {
        Rule {
            name: name.to_string(),
            description: None,
            matchers: Matchers {
                tools: None,
                extensions: None,
                directories: None,
                operations: None,
                command_match: None,
            },
            actions: Actions {
                inject: None,
                inject_inline: None,
                run: None,
                block: None,
                block_if_match: None,
            },
            mode: None,
            priority: Some(priority),
            governance: None,
            metadata: None,
        }
    }

    // =========================================================================
    // YAML Parsing Integration Tests
    // =========================================================================

    #[test]
    fn test_rule_with_governance_yaml() {
        let yaml = r#"
name: block-force-push
description: Prevent force pushes to protected branches
mode: enforce
priority: 100
matchers:
  tools: [Bash]
  command_match: "git push.*--force"
actions:
  block: true
governance:
  author: security-team
  created_by: aws-cdk-skill@1.2.0
  reason: Enforce git safety standards
  confidence: high
  ticket: SEC-001
  tags: [security, git]
"#;
        let rule: Rule = serde_yaml::from_str(yaml).unwrap();

        assert_eq!(rule.name, "block-force-push");
        assert_eq!(rule.effective_mode(), PolicyMode::Enforce);
        assert_eq!(rule.effective_priority(), 100);

        let gov = rule.governance.unwrap();
        assert_eq!(gov.author, Some("security-team".to_string()));
        assert_eq!(gov.confidence, Some(Confidence::High));
        assert_eq!(
            gov.tags,
            Some(vec!["security".to_string(), "git".to_string()])
        );
    }

    #[test]
    fn test_rule_backward_compatible_yaml() {
        // This is an existing v1.0 config format - must still work
        let yaml = r"
name: inject-context
matchers:
  tools: [Edit]
actions:
  inject: .claude/context.md
metadata:
  priority: 10
  timeout: 5
  enabled: true
";
        let rule: Rule = serde_yaml::from_str(yaml).unwrap();

        assert_eq!(rule.name, "inject-context");
        assert_eq!(rule.effective_mode(), PolicyMode::Enforce); // Default
        assert_eq!(rule.effective_priority(), 10); // From legacy metadata
        assert!(rule.governance.is_none());
    }

    // =========================================================================
    // inject_inline Tests
    // =========================================================================

    #[test]
    fn test_inject_inline_literal_block() {
        // Tests YAML literal block style (|) which preserves newlines
        let yaml = r#"
inject_inline: |
  ## Production Warning
  You are editing production files.
  Be extra careful.
"#;
        let actions: Actions = serde_yaml::from_str(yaml).unwrap();

        assert!(actions.inject_inline.is_some());
        let content = actions.inject_inline.unwrap();
        assert!(content.contains("## Production Warning"));
        assert!(content.contains("\n")); // Literal block preserves newlines
        assert!(content.contains("Be extra careful"));
    }

    #[test]
    fn test_inject_inline_folded_block() {
        // Tests YAML folded block style (>) which folds newlines into spaces
        let yaml = r#"
inject_inline: >
  This is a long paragraph that
  will be folded into a single line.
"#;
        let actions: Actions = serde_yaml::from_str(yaml).unwrap();

        assert!(actions.inject_inline.is_some());
        let content = actions.inject_inline.unwrap();
        assert!(content.contains("This is a long paragraph"));
        // Folded style converts newlines within paragraph to spaces
    }

    #[test]
    fn test_inject_inline_simple_string() {
        // Tests simple quoted string parsing
        let yaml = r#"
inject_inline: "Single line warning"
"#;
        let actions: Actions = serde_yaml::from_str(yaml).unwrap();

        assert!(actions.inject_inline.is_some());
        assert_eq!(actions.inject_inline.unwrap(), "Single line warning");
    }

    #[test]
    fn test_inject_inline_precedence() {
        // Tests that both inject and inject_inline can coexist
        // Runtime precedence is handled in hooks.rs
        let yaml = r#"
inject: "/path/to/file.md"
inject_inline: "Inline takes precedence"
"#;
        let actions: Actions = serde_yaml::from_str(yaml).unwrap();

        // Both fields parse successfully
        assert!(actions.inject.is_some());
        assert!(actions.inject_inline.is_some());
        assert_eq!(actions.inject.unwrap(), "/path/to/file.md");
        assert_eq!(actions.inject_inline.unwrap(), "Inline takes precedence");
    }

    #[test]
    fn test_inject_inline_full_rule_yaml() {
        // Tests inject_inline in a complete rule definition
        let yaml = r#"
name: prod-warning
description: Warn when editing production files
matchers:
  directories: ["/prod/"]
actions:
  inject_inline: |
    ## Production Warning
    You are editing production files.
    Be extra careful with these changes.
"#;
        let rule: Rule = serde_yaml::from_str(yaml).unwrap();

        assert_eq!(rule.name, "prod-warning");
        assert!(rule.actions.inject_inline.is_some());
        let content = rule.actions.inject_inline.unwrap();
        assert!(content.contains("## Production Warning"));
        assert!(content.contains("production files"));
    }
}

#[cfg(test)]
mod event_details_tests {
    use super::*;
    use chrono::Utc;

    #[test]
    fn test_extract_bash_event() {
        let event = Event {
            hook_event_name: EventType::PreToolUse,
            tool_name: Some("Bash".to_string()),
            tool_input: Some(serde_json::json!({
                "command": "git push --force"
            })),
            session_id: "test-session".to_string(),
            timestamp: Utc::now(),
            user_id: None,
            transcript_path: None,
            cwd: None,
            permission_mode: None,
            tool_use_id: None,
        };

        let details = EventDetails::extract(&event);
        assert!(matches!(details, EventDetails::Bash { command } if command == "git push --force"));
    }

    #[test]
    fn test_extract_write_event() {
        let event = Event {
            hook_event_name: EventType::PreToolUse,
            tool_name: Some("Write".to_string()),
            tool_input: Some(serde_json::json!({
                "filePath": "/path/to/file.rs"
            })),
            session_id: "test-session".to_string(),
            timestamp: Utc::now(),
            user_id: None,
            transcript_path: None,
            cwd: None,
            permission_mode: None,
            tool_use_id: None,
        };

        let details = EventDetails::extract(&event);
        assert!(
            matches!(details, EventDetails::Write { file_path } if file_path == "/path/to/file.rs")
        );
    }

    #[test]
    fn test_extract_write_event_file_path() {
        let event = Event {
            hook_event_name: EventType::PreToolUse,
            tool_name: Some("Write".to_string()),
            tool_input: Some(serde_json::json!({
                "file_path": "/path/to/file.rs"
            })),
            session_id: "test-session".to_string(),
            timestamp: Utc::now(),
            user_id: None,
            transcript_path: None,
            cwd: None,
            permission_mode: None,
            tool_use_id: None,
        };

        let details = EventDetails::extract(&event);
        assert!(
            matches!(details, EventDetails::Write { file_path } if file_path == "/path/to/file.rs")
        );
    }

    #[test]
    fn test_extract_edit_event() {
        let event = Event {
            hook_event_name: EventType::PreToolUse,
            tool_name: Some("Edit".to_string()),
            tool_input: Some(serde_json::json!({
                "filePath": "/path/to/file.rs"
            })),
            session_id: "test-session".to_string(),
            timestamp: Utc::now(),
            user_id: None,
            transcript_path: None,
            cwd: None,
            permission_mode: None,
            tool_use_id: None,
        };

        let details = EventDetails::extract(&event);
        assert!(
            matches!(details, EventDetails::Edit { file_path } if file_path == "/path/to/file.rs")
        );
    }

    #[test]
    fn test_extract_read_event() {
        let event = Event {
            hook_event_name: EventType::PreToolUse,
            tool_name: Some("Read".to_string()),
            tool_input: Some(serde_json::json!({
                "filePath": "/path/to/file.rs"
            })),
            session_id: "test-session".to_string(),
            timestamp: Utc::now(),
            user_id: None,
            transcript_path: None,
            cwd: None,
            permission_mode: None,
            tool_use_id: None,
        };

        let details = EventDetails::extract(&event);
        assert!(
            matches!(details, EventDetails::Read { file_path } if file_path == "/path/to/file.rs")
        );
    }

    #[test]
    fn test_extract_glob_event() {
        let event = Event {
            hook_event_name: EventType::PreToolUse,
            tool_name: Some("Glob".to_string()),
            tool_input: Some(serde_json::json!({
                "pattern": "*.rs",
                "path": "src"
            })),
            session_id: "test-session".to_string(),
            timestamp: Utc::now(),
            user_id: None,
            transcript_path: None,
            cwd: None,
            permission_mode: None,
            tool_use_id: None,
        };

        let details = EventDetails::extract(&event);
        assert!(matches!(details, EventDetails::Glob { pattern, path }
            if pattern == Some("*.rs".to_string()) && path == Some("src".to_string())));
    }

    #[test]
    fn test_extract_grep_event() {
        let event = Event {
            hook_event_name: EventType::PreToolUse,
            tool_name: Some("Grep".to_string()),
            tool_input: Some(serde_json::json!({
                "pattern": "fn main",
                "path": "src"
            })),
            session_id: "test-session".to_string(),
            timestamp: Utc::now(),
            user_id: None,
            transcript_path: None,
            cwd: None,
            permission_mode: None,
            tool_use_id: None,
        };

        let details = EventDetails::extract(&event);
        assert!(matches!(details, EventDetails::Grep { pattern, path }
            if pattern == Some("fn main".to_string()) && path == Some("src".to_string())));
    }

    #[test]
    fn test_extract_session_start_event() {
        let event = Event {
            hook_event_name: EventType::SessionStart,
            tool_name: None,
            tool_input: Some(serde_json::json!({
                "source": "vscode",
                "reason": "user_initiated",
                "transcript_path": "/tmp/transcript.json",
                "cwd": "/home/user/project"
            })),
            session_id: "test-session".to_string(),
            timestamp: Utc::now(),
            user_id: None,
            transcript_path: None,
            cwd: None,
            permission_mode: None,
            tool_use_id: None,
        };

        let details = EventDetails::extract(&event);
        assert!(
            matches!(details, EventDetails::Session { source, reason, transcript_path, cwd }
            if source == Some("vscode".to_string())
            && reason == Some("user_initiated".to_string())
            && transcript_path == Some("/tmp/transcript.json".to_string())
            && cwd == Some("/home/user/project".to_string()))
        );
    }

    #[test]
    fn test_extract_unknown_tool() {
        let event = Event {
            hook_event_name: EventType::PreToolUse,
            tool_name: Some("FutureTool".to_string()),
            tool_input: None,
            session_id: "test-session".to_string(),
            timestamp: Utc::now(),
            user_id: None,
            transcript_path: None,
            cwd: None,
            permission_mode: None,
            tool_use_id: None,
        };

        let details = EventDetails::extract(&event);
        assert!(matches!(details, EventDetails::Unknown { tool_name }
            if tool_name == Some("FutureTool".to_string())));
    }

    #[test]
    fn test_response_summary_from_response() {
        let response = Response {
            continue_: true,
            context: Some("injected context".to_string()),
            reason: Some("for testing".to_string()),
            timing: None,
        };

        let summary = ResponseSummary::from_response(&response);
        assert!(summary.continue_);
        assert_eq!(summary.reason, Some("for testing".to_string()));
        assert_eq!(summary.context_length, Some(16)); // "injected context" = 16 chars
    }

    #[test]
    fn test_debug_config_new() {
        // Test CLI flag only
        let config = DebugConfig::new(true, false);
        assert!(config.enabled);

        // Test config setting only
        let config = DebugConfig::new(false, true);
        assert!(config.enabled);

        // Test both false
        let config = DebugConfig::new(false, false);
        assert!(!config.enabled);
    }
}

fn default_timeout() -> u32 {
    5
}

fn default_enabled() -> bool {
    true
}

/// Claude Code hook event data structure
///
/// Claude Code sends events with `hook_event_name` as the field name.
/// The `alias = "event_type"` preserves backward compatibility with
/// debug commands and tests that use the old field name.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Event {
    /// Hook event type (Claude Code sends as `hook_event_name`)
    #[serde(alias = "event_type")]
    pub hook_event_name: EventType,

    /// Name of the tool being used
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool_name: Option<String>,

    /// Tool parameters and arguments
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool_input: Option<serde_json::Value>,

    /// Unique session identifier
    pub session_id: String,

    /// ISO 8601 timestamp (Claude Code may not send this, so default to now)
    #[serde(default = "chrono::Utc::now")]
    pub timestamp: DateTime<Utc>,

    /// User identifier if available
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,

    /// Path to session transcript (sent by Claude Code)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transcript_path: Option<String>,

    /// Current working directory (sent by Claude Code)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cwd: Option<String>,

    /// Permission mode (sent by Claude Code)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permission_mode: Option<String>,

    /// Tool use ID (sent by Claude Code)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool_use_id: Option<String>,
}

/// Supported hook event types
///
/// Includes all event types that Claude Code can send to hooks.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub enum EventType {
    PreToolUse,
    PostToolUse,
    PermissionRequest,
    UserPromptSubmit,
    SessionStart,
    SessionEnd,
    PreCompact,
    Stop,
    PostToolUseFailure,
    SubagentStart,
    SubagentStop,
    Notification,
    Setup,
}

impl std::fmt::Display for EventType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EventType::PreToolUse => write!(f, "PreToolUse"),
            EventType::PostToolUse => write!(f, "PostToolUse"),
            EventType::PermissionRequest => write!(f, "PermissionRequest"),
            EventType::UserPromptSubmit => write!(f, "UserPromptSubmit"),
            EventType::SessionStart => write!(f, "SessionStart"),
            EventType::SessionEnd => write!(f, "SessionEnd"),
            EventType::PreCompact => write!(f, "PreCompact"),
            EventType::Stop => write!(f, "Stop"),
            EventType::PostToolUseFailure => write!(f, "PostToolUseFailure"),
            EventType::SubagentStart => write!(f, "SubagentStart"),
            EventType::SubagentStop => write!(f, "SubagentStop"),
            EventType::Notification => write!(f, "Notification"),
            EventType::Setup => write!(f, "Setup"),
        }
    }
}

/// Binary output structure for hook responses
///
/// Sent to Claude Code via stdout. The `continue` field controls whether
/// the operation proceeds or is blocked.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Response {
    /// Whether the operation should proceed
    #[serde(rename = "continue")]
    pub continue_: bool,

    /// Additional context to inject
    #[serde(skip_serializing_if = "Option::is_none")]
    pub context: Option<String>,

    /// Explanation for blocking or context injection
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,

    /// Performance metrics
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timing: Option<Timing>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Timing {
    /// Total processing time in milliseconds
    pub processing_ms: u64,

    /// Number of rules checked
    pub rules_evaluated: usize,
}

/// Structured audit log record
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct LogEntry {
    /// ISO 8601 timestamp with microsecond precision
    pub timestamp: DateTime<Utc>,

    /// Hook event type
    pub event_type: String,

    /// Session identifier
    pub session_id: String,

    /// Tool being used
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool_name: Option<String>,

    /// Names of rules that matched
    pub rules_matched: Vec<String>,

    /// Result of evaluation
    pub outcome: Outcome,

    /// Performance data
    pub timing: LogTiming,

    /// Additional context
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<LogMetadata>,

    // === Enhanced Logging Fields (CRD-001) ===
    /// Typed event details extracted from tool_input
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_details: Option<EventDetails>,

    /// Summary of response sent to Claude
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response: Option<ResponseSummary>,

    /// Full raw event JSON (debug mode only)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub raw_event: Option<serde_json::Value>,

    /// Per-rule evaluation details (debug mode only)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rule_evaluations: Option<Vec<RuleEvaluation>>,

    // === Phase 2.2 Governance Logging Fields ===
    /// Policy mode from the winning/primary matched rule
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mode: Option<PolicyMode>,

    /// Priority of the winning/primary matched rule
    #[serde(skip_serializing_if = "Option::is_none")]
    pub priority: Option<i32>,

    /// Decision outcome (Allowed, Blocked, Warned, Audited)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub decision: Option<Decision>,

    /// Governance metadata from the primary matched rule
    #[serde(skip_serializing_if = "Option::is_none")]
    pub governance: Option<GovernanceMetadata>,

    /// Trust level of validator script (if run action was executed)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trust_level: Option<TrustLevel>,
}

/// Result of rule evaluation
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum Outcome {
    Allow,
    Block,
    Inject,
}

/// Performance data for logging
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct LogTiming {
    /// Processing time in milliseconds
    pub processing_ms: u64,

    /// Rules checked
    pub rules_evaluated: usize,
}

/// Additional log context
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct LogMetadata {
    /// Files injected as context
    #[serde(skip_serializing_if = "Option::is_none")]
    pub injected_files: Option<Vec<String>>,

    /// Script execution results
    #[serde(skip_serializing_if = "Option::is_none")]
    pub validator_output: Option<String>,
}

// =============================================================================
// Enhanced Logging Types (CRD-001)
// =============================================================================

/// Typed event details for known tools
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "tool_type")]
pub enum EventDetails {
    /// Bash command execution
    Bash { command: String },
    /// File write operation
    Write { file_path: String },
    /// File edit operation
    Edit { file_path: String },
    /// File read operation
    Read { file_path: String },
    /// Glob pattern search
    Glob {
        #[serde(skip_serializing_if = "Option::is_none")]
        pattern: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        path: Option<String>,
    },
    /// Grep content search
    Grep {
        #[serde(skip_serializing_if = "Option::is_none")]
        pattern: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        path: Option<String>,
    },
    /// Session start/end events
    Session {
        #[serde(skip_serializing_if = "Option::is_none")]
        source: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        reason: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        transcript_path: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        cwd: Option<String>,
    },
    /// Permission request wrapping another tool
    Permission {
        #[serde(skip_serializing_if = "Option::is_none")]
        permission_mode: Option<String>,
        tool_details: Box<EventDetails>,
    },
    /// Unknown or unsupported tool
    Unknown {
        #[serde(skip_serializing_if = "Option::is_none")]
        tool_name: Option<String>,
    },
}

/// Summary of response sent to Claude
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ResponseSummary {
    /// Whether the operation should continue
    #[serde(rename = "continue")]
    pub continue_: bool,

    /// Explanation for blocking or context injection
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,

    /// Length of injected context (if any)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub context_length: Option<usize>,
}

/// Per-rule evaluation details (debug mode only)
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct RuleEvaluation {
    /// Name of the rule evaluated
    pub rule_name: String,

    /// Whether the rule matched
    pub matched: bool,

    /// Individual matcher results
    #[serde(skip_serializing_if = "Option::is_none")]
    pub matcher_results: Option<MatcherResults>,
}

/// Individual matcher results for debug output
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct MatcherResults {
    /// Whether tools matcher matched
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tools_matched: Option<bool>,

    /// Whether extensions matcher matched
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extensions_matched: Option<bool>,

    /// Whether directories matcher matched
    #[serde(skip_serializing_if = "Option::is_none")]
    pub directories_matched: Option<bool>,

    /// Whether command_match regex matched
    #[serde(skip_serializing_if = "Option::is_none")]
    pub command_match_matched: Option<bool>,

    /// Whether operations matcher matched
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operations_matched: Option<bool>,
}

/// Debug mode configuration
#[derive(Debug, Clone, Default)]
pub struct DebugConfig {
    /// Whether debug logging is enabled
    pub enabled: bool,
}

impl DebugConfig {
    /// Create a new DebugConfig from CLI flag and config setting
    pub fn new(cli_flag: bool, config_setting: bool) -> Self {
        let enabled = cli_flag || std::env::var("CCH_DEBUG_LOGS").is_ok() || config_setting;
        Self { enabled }
    }
}

impl EventDetails {
    /// Extract typed details from an Event
    pub fn extract(event: &Event) -> Self {
        let tool_name = event.tool_name.as_deref();
        let tool_input = event.tool_input.as_ref();

        match tool_name {
            Some("Bash") => {
                let command = tool_input
                    .and_then(|ti| ti.get("command"))
                    .and_then(|c| c.as_str())
                    .unwrap_or("")
                    .to_string();
                EventDetails::Bash { command }
            }
            Some("Write") => {
                let file_path = tool_input
                    .and_then(|ti| ti.get("file_path").or_else(|| ti.get("filePath")))
                    .and_then(|p| p.as_str())
                    .unwrap_or("")
                    .to_string();
                EventDetails::Write { file_path }
            }
            Some("Edit") => {
                let file_path = tool_input
                    .and_then(|ti| ti.get("file_path").or_else(|| ti.get("filePath")))
                    .and_then(|p| p.as_str())
                    .unwrap_or("")
                    .to_string();
                EventDetails::Edit { file_path }
            }
            Some("Read") => {
                let file_path = tool_input
                    .and_then(|ti| ti.get("file_path").or_else(|| ti.get("filePath")))
                    .and_then(|p| p.as_str())
                    .unwrap_or("")
                    .to_string();
                EventDetails::Read { file_path }
            }
            Some("Glob") => {
                let pattern = tool_input
                    .and_then(|ti| ti.get("pattern"))
                    .and_then(|p| p.as_str())
                    .map(String::from);
                let path = tool_input
                    .and_then(|ti| ti.get("path"))
                    .and_then(|p| p.as_str())
                    .map(String::from);
                EventDetails::Glob { pattern, path }
            }
            Some("Grep") => {
                let pattern = tool_input
                    .and_then(|ti| ti.get("pattern"))
                    .and_then(|p| p.as_str())
                    .map(String::from);
                let path = tool_input
                    .and_then(|ti| ti.get("path"))
                    .and_then(|p| p.as_str())
                    .map(String::from);
                EventDetails::Grep { pattern, path }
            }
            None if matches!(
                event.hook_event_name,
                EventType::SessionStart | EventType::SessionEnd
            ) =>
            {
                let source = tool_input
                    .and_then(|ti| ti.get("source"))
                    .and_then(|s| s.as_str())
                    .map(String::from);
                let reason = tool_input
                    .and_then(|ti| ti.get("reason"))
                    .and_then(|r| r.as_str())
                    .map(String::from);
                let transcript_path = tool_input
                    .and_then(|ti| ti.get("transcript_path"))
                    .and_then(|t| t.as_str())
                    .map(String::from);
                let cwd = tool_input
                    .and_then(|ti| ti.get("cwd"))
                    .and_then(|c| c.as_str())
                    .map(String::from);
                EventDetails::Session {
                    source,
                    reason,
                    transcript_path,
                    cwd,
                }
            }

            _ => EventDetails::Unknown {
                tool_name: tool_name.map(String::from),
            },
        }
    }
}

impl ResponseSummary {
    /// Create from a Response
    pub fn from_response(response: &Response) -> Self {
        Self {
            continue_: response.continue_,
            reason: response.reason.clone(),
            context_length: response.context.as_ref().map(|c| c.len()),
        }
    }
}

impl Default for RuleMetadata {
    fn default() -> Self {
        Self {
            priority: 0,
            timeout: default_timeout(),
            enabled: default_enabled(),
        }
    }
}

// =============================================================================
// Rule Helper Methods (Phase 2 Governance)
// =============================================================================

impl Rule {
    /// Get the effective policy mode (defaults to Enforce)
    #[allow(dead_code)] // Used in Phase 2.1-T05 (mode-based action execution)
    pub fn effective_mode(&self) -> PolicyMode {
        self.mode.unwrap_or_default()
    }

    /// Get the effective priority (defaults to 0)
    /// Checks both new priority field and legacy metadata.priority
    #[allow(dead_code)] // Used in Phase 2.1-T04 (priority sorting in hooks.rs)
    pub fn effective_priority(&self) -> i32 {
        self.priority
            .or_else(|| self.metadata.as_ref().map(|m| m.priority))
            .unwrap_or(0)
    }

    /// Check if the rule is enabled
    /// Uses legacy metadata.enabled field, defaults to true
    #[allow(dead_code)] // Used in Phase 2.1-T05 (mode-based action execution)
    pub fn is_enabled(&self) -> bool {
        self.metadata.as_ref().map(|m| m.enabled).unwrap_or(true)
    }
}

/// Sort rules by priority in descending order (higher numbers first)
/// Uses stable sort to preserve file order for same priority
#[allow(dead_code)] // Used in Phase 2.1-T04 (will be called from hooks.rs)
pub fn sort_rules_by_priority(rules: &mut [Rule]) {
    rules.sort_by(|a, b| {
        let priority_a = a.effective_priority();
        let priority_b = b.effective_priority();
        priority_b.cmp(&priority_a) // Descending order
    });
}

impl Response {
    /// Create a new response allowing the operation
    pub fn allow() -> Self {
        Self {
            continue_: true,
            context: None,
            reason: None,
            timing: None,
        }
    }

    /// Create a new response blocking the operation
    pub fn block(reason: impl Into<String>) -> Self {
        Self {
            continue_: false,
            context: None,
            reason: Some(reason.into()),
            timing: None,
        }
    }

    /// Create a new response with context injection
    pub fn inject(context: impl Into<String>) -> Self {
        Self {
            continue_: true,
            context: Some(context.into()),
            reason: None,
            timing: None,
        }
    }
}
