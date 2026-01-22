use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

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

#[cfg(test)]
mod event_details_tests {
    use super::*;
    use chrono::Utc;

    #[test]
    fn test_extract_bash_event() {
        let event = Event {
            event_type: EventType::PreToolUse,
            tool_name: Some("Bash".to_string()),
            tool_input: Some(serde_json::json!({
                "command": "git push --force"
            })),
            session_id: "test-session".to_string(),
            timestamp: Utc::now(),
            user_id: None,
        };

        let details = EventDetails::extract(&event);
        assert!(matches!(details, EventDetails::Bash { command } if command == "git push --force"));
    }

    #[test]
    fn test_extract_write_event() {
        let event = Event {
            event_type: EventType::PreToolUse,
            tool_name: Some("Write".to_string()),
            tool_input: Some(serde_json::json!({
                "filePath": "/path/to/file.rs"
            })),
            session_id: "test-session".to_string(),
            timestamp: Utc::now(),
            user_id: None,
        };

        let details = EventDetails::extract(&event);
        assert!(
            matches!(details, EventDetails::Write { file_path } if file_path == "/path/to/file.rs")
        );
    }

    #[test]
    fn test_extract_write_event_file_path() {
        let event = Event {
            event_type: EventType::PreToolUse,
            tool_name: Some("Write".to_string()),
            tool_input: Some(serde_json::json!({
                "file_path": "/path/to/file.rs"
            })),
            session_id: "test-session".to_string(),
            timestamp: Utc::now(),
            user_id: None,
        };

        let details = EventDetails::extract(&event);
        assert!(
            matches!(details, EventDetails::Write { file_path } if file_path == "/path/to/file.rs")
        );
    }

    #[test]
    fn test_extract_edit_event() {
        let event = Event {
            event_type: EventType::PreToolUse,
            tool_name: Some("Edit".to_string()),
            tool_input: Some(serde_json::json!({
                "filePath": "/path/to/file.rs"
            })),
            session_id: "test-session".to_string(),
            timestamp: Utc::now(),
            user_id: None,
        };

        let details = EventDetails::extract(&event);
        assert!(
            matches!(details, EventDetails::Edit { file_path } if file_path == "/path/to/file.rs")
        );
    }

    #[test]
    fn test_extract_read_event() {
        let event = Event {
            event_type: EventType::PreToolUse,
            tool_name: Some("Read".to_string()),
            tool_input: Some(serde_json::json!({
                "filePath": "/path/to/file.rs"
            })),
            session_id: "test-session".to_string(),
            timestamp: Utc::now(),
            user_id: None,
        };

        let details = EventDetails::extract(&event);
        assert!(
            matches!(details, EventDetails::Read { file_path } if file_path == "/path/to/file.rs")
        );
    }

    #[test]
    fn test_extract_glob_event() {
        let event = Event {
            event_type: EventType::PreToolUse,
            tool_name: Some("Glob".to_string()),
            tool_input: Some(serde_json::json!({
                "pattern": "*.rs",
                "path": "src"
            })),
            session_id: "test-session".to_string(),
            timestamp: Utc::now(),
            user_id: None,
        };

        let details = EventDetails::extract(&event);
        assert!(matches!(details, EventDetails::Glob { pattern, path }
            if pattern == Some("*.rs".to_string()) && path == Some("src".to_string())));
    }

    #[test]
    fn test_extract_grep_event() {
        let event = Event {
            event_type: EventType::PreToolUse,
            tool_name: Some("Grep".to_string()),
            tool_input: Some(serde_json::json!({
                "pattern": "fn main",
                "path": "src"
            })),
            session_id: "test-session".to_string(),
            timestamp: Utc::now(),
            user_id: None,
        };

        let details = EventDetails::extract(&event);
        assert!(matches!(details, EventDetails::Grep { pattern, path }
            if pattern == Some("fn main".to_string()) && path == Some("src".to_string())));
    }

    #[test]
    fn test_extract_session_start_event() {
        let event = Event {
            event_type: EventType::SessionStart,
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
            event_type: EventType::PreToolUse,
            tool_name: Some("FutureTool".to_string()),
            tool_input: None,
            session_id: "test-session".to_string(),
            timestamp: Utc::now(),
            user_id: None,
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
                event.event_type,
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
