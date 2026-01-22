use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};


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

    /// Additional rule information
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

    /// Path to validator script to execute
    #[serde(skip_serializing_if = "Option::is_none")]
    pub run: Option<String>,

    /// Whether to block the operation
    #[serde(skip_serializing_if = "Option::is_none")]
    pub block: Option<bool>,

    /// Regex pattern for conditional blocking
    #[serde(skip_serializing_if = "Option::is_none")]
    pub block_if_match: Option<String>,
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

fn default_timeout() -> u32 {
    5
}

fn default_enabled() -> bool {
    true
}

/// Claude Code hook event data structure
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Event {
    /// Hook event type
    pub event_type: EventType,

    /// Name of the tool being used
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool_name: Option<String>,

    /// Tool parameters and arguments
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool_input: Option<serde_json::Value>,

    /// Unique session identifier
    pub session_id: String,

    /// ISO 8601 timestamp
    pub timestamp: DateTime<Utc>,

    /// User identifier if available
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
}

/// Supported hook event types
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
        }
    }
}

/// Binary output structure for hook responses
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Response {
    /// Whether the operation should proceed
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

impl Default for RuleMetadata {
    fn default() -> Self {
        Self {
            priority: 0,
            timeout: default_timeout(),
            enabled: default_enabled(),
        }
    }
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