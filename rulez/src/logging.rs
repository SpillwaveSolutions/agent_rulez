use anyhow::Result;
use chrono::{DateTime, Utc};
use std::collections::HashMap;
use std::fs::{File, OpenOptions};
use std::io::{BufWriter, Write};
use std::path::PathBuf;
use std::sync::Mutex;
use std::time::Duration;

use crate::models::LogEntry;
use serde::{Deserialize, Serialize};

// =============================================================================
// External Logging Backend Configuration
// =============================================================================

/// Configuration for external logging backends in hooks.yaml.
///
/// ```yaml
/// settings:
///   logging:
///     backends:
///       - type: otlp
///         endpoint: "http://localhost:4318/v1/logs"
///       - type: datadog
///         api_key: "${DD_API_KEY}"
///       - type: splunk
///         endpoint: "https://splunk:8088/services/collector/event"
///         token: "${SPLUNK_HEC_TOKEN}"
/// ```
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct LoggingConfig {
    /// External logging backends
    #[serde(default)]
    pub backends: Vec<BackendConfig>,
}

/// Configuration for a single logging backend.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum BackendConfig {
    /// OpenTelemetry Protocol (OTLP) HTTP exporter
    Otlp {
        endpoint: String,
        #[serde(default)]
        headers: HashMap<String, String>,
        #[serde(default = "default_timeout")]
        timeout_secs: u64,
    },
    /// Datadog Log Management via HTTP API
    Datadog {
        #[serde(default = "default_datadog_endpoint")]
        endpoint: String,
        api_key: String,
        #[serde(default = "default_timeout")]
        timeout_secs: u64,
    },
    /// Splunk HTTP Event Collector (HEC)
    Splunk {
        endpoint: String,
        token: String,
        #[serde(default = "default_splunk_sourcetype")]
        sourcetype: String,
        #[serde(default = "default_timeout")]
        timeout_secs: u64,
    },
}

fn default_timeout() -> u64 {
    5
}
fn default_datadog_endpoint() -> String {
    "https://http-intake.logs.datadoghq.com/api/v2/logs".to_string()
}
fn default_splunk_sourcetype() -> String {
    "rulez".to_string()
}

// =============================================================================
// Backend Trait and Implementations
// =============================================================================

/// Trait for external logging backends.
trait LogBackend: Send + Sync {
    fn send(&self, entry: &LogEntry) -> Result<()>;
    fn name(&self) -> &'static str;
}

/// Expand `${VAR}` references in strings.
fn expand_env_vars(s: &str) -> String {
    let mut result = s.to_string();
    while let Some(start) = result.find("${") {
        if let Some(end) = result[start..].find('}') {
            let var_name = &result[start + 2..start + end];
            let value = std::env::var(var_name).unwrap_or_default();
            result = format!(
                "{}{}{}",
                &result[..start],
                value,
                &result[start + end + 1..]
            );
        } else {
            break;
        }
    }
    result
}

fn hostname() -> String {
    std::env::var("HOSTNAME")
        .or_else(|_| std::env::var("HOST"))
        .unwrap_or_else(|_| "unknown".to_string())
}

// --- OTLP Backend ---

struct OtlpBackend {
    endpoint: String,
    headers: HashMap<String, String>,
    timeout: Duration,
}

impl LogBackend for OtlpBackend {
    fn send(&self, entry: &LogEntry) -> Result<()> {
        let body = serde_json::to_string(entry)?;
        let otlp_payload = serde_json::json!({
            "resourceLogs": [{
                "resource": {
                    "attributes": [{
                        "key": "service.name",
                        "value": { "stringValue": "rulez" }
                    }]
                },
                "scopeLogs": [{
                    "scope": { "name": "rulez.audit" },
                    "logRecords": [{
                        "timeUnixNano": entry.timestamp.timestamp_nanos_opt()
                            .unwrap_or(0).to_string(),
                        "severityNumber": 9,
                        "severityText": "INFO",
                        "body": { "stringValue": body },
                        "attributes": [
                            { "key": "rulez.event_type",
                              "value": { "stringValue": &entry.event_type } },
                            { "key": "rulez.session_id",
                              "value": { "stringValue": &entry.session_id } },
                            { "key": "rulez.outcome",
                              "value": { "stringValue": format!("{:?}", entry.outcome) } }
                        ]
                    }]
                }]
            }]
        });
        let payload = serde_json::to_vec(&otlp_payload)?;
        send_via_curl(&self.endpoint, &payload, &self.headers, self.timeout)
    }
    fn name(&self) -> &'static str {
        "otlp"
    }
}

// --- Datadog Backend ---

struct DatadogBackend {
    endpoint: String,
    api_key: String,
    timeout: Duration,
}

impl LogBackend for DatadogBackend {
    fn send(&self, entry: &LogEntry) -> Result<()> {
        let dd_payload = serde_json::json!([{
            "ddsource": "rulez",
            "ddtags": format!("event_type:{},outcome:{:?}", entry.event_type, entry.outcome),
            "hostname": hostname(),
            "message": serde_json::to_string(entry)?,
            "service": "rulez",
            "status": match entry.outcome {
                crate::models::Outcome::Block => "error",
                _ => "info",
            }
        }]);
        let payload = serde_json::to_vec(&dd_payload)?;
        let mut headers = HashMap::new();
        headers.insert("DD-API-KEY".to_string(), self.api_key.clone());
        send_via_curl(&self.endpoint, &payload, &headers, self.timeout)
    }
    fn name(&self) -> &'static str {
        "datadog"
    }
}

// --- Splunk HEC Backend ---

struct SplunkBackend {
    endpoint: String,
    token: String,
    sourcetype: String,
    timeout: Duration,
}

impl LogBackend for SplunkBackend {
    fn send(&self, entry: &LogEntry) -> Result<()> {
        let splunk_payload = serde_json::json!({
            "event": entry,
            "sourcetype": self.sourcetype,
            "source": "rulez",
            "host": hostname(),
            "time": entry.timestamp.timestamp()
        });
        let payload = serde_json::to_vec(&splunk_payload)?;
        let mut headers = HashMap::new();
        headers.insert(
            "Authorization".to_string(),
            format!("Splunk {}", self.token),
        );
        send_via_curl(&self.endpoint, &payload, &headers, self.timeout)
    }
    fn name(&self) -> &'static str {
        "splunk"
    }
}

// =============================================================================
// HTTP Transport (via curl — avoids TLS library dependency)
// =============================================================================

fn send_via_curl(
    url: &str,
    body: &[u8],
    headers: &HashMap<String, String>,
    timeout: Duration,
) -> Result<()> {
    use std::process::{Command, Stdio};

    let mut cmd = Command::new("curl");
    cmd.arg("-s")
        .arg("-S")
        .arg("-X")
        .arg("POST")
        .arg("-H")
        .arg("Content-Type: application/json")
        .arg("--max-time")
        .arg(timeout.as_secs().to_string());

    for (key, value) in headers {
        cmd.arg("-H").arg(format!("{}: {}", key, value));
    }

    cmd.arg("-d")
        .arg("@-")
        .arg(url)
        .stdin(Stdio::piped())
        .stdout(Stdio::null())
        .stderr(Stdio::null());

    let mut child = cmd
        .spawn()
        .map_err(|e| anyhow::anyhow!("Failed to spawn curl for logging backend: {}", e))?;

    if let Some(ref mut stdin) = child.stdin {
        stdin.write_all(body)?;
    }
    drop(child.stdin.take()); // Close stdin so curl proceeds

    let status = child.wait()?;
    if !status.success() {
        anyhow::bail!("curl exited with status: {}", status);
    }

    Ok(())
}

// =============================================================================
// Backend Factory
// =============================================================================

fn create_backends(config: &LoggingConfig) -> Vec<Box<dyn LogBackend>> {
    config
        .backends
        .iter()
        .map(|bc| -> Box<dyn LogBackend> {
            match bc {
                BackendConfig::Otlp {
                    endpoint,
                    headers,
                    timeout_secs,
                } => Box::new(OtlpBackend {
                    endpoint: endpoint.clone(),
                    headers: headers
                        .iter()
                        .map(|(k, v)| (k.clone(), expand_env_vars(v)))
                        .collect(),
                    timeout: Duration::from_secs(*timeout_secs),
                }),
                BackendConfig::Datadog {
                    endpoint,
                    api_key,
                    timeout_secs,
                } => Box::new(DatadogBackend {
                    endpoint: endpoint.clone(),
                    api_key: expand_env_vars(api_key),
                    timeout: Duration::from_secs(*timeout_secs),
                }),
                BackendConfig::Splunk {
                    endpoint,
                    token,
                    sourcetype,
                    timeout_secs,
                } => Box::new(SplunkBackend {
                    endpoint: endpoint.clone(),
                    token: expand_env_vars(token),
                    sourcetype: sourcetype.clone(),
                    timeout: Duration::from_secs(*timeout_secs),
                }),
            }
        })
        .collect()
}

// =============================================================================
// Core Logger
// =============================================================================

/// JSON Lines logger with optional external backends.
///
/// Always writes to the local JSON Lines file. When external backends are
/// configured, forwards entries to each backend. Backend failures are logged
/// as warnings but do not block local logging (fail-open).
pub struct Logger {
    writer: Mutex<BufWriter<File>>,
    external_backends: Vec<Box<dyn LogBackend>>,
}

impl Logger {
    /// Create a new logger with the default log file path (no external backends)
    pub fn new() -> Result<Self> {
        let log_path = Self::default_log_path();
        Self::with_path(log_path)
    }

    /// Create a new logger with external backends from configuration
    pub fn with_backends(logging_config: &LoggingConfig) -> Result<Self> {
        let path = Self::default_log_path();

        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)?;
        }

        let file = OpenOptions::new().create(true).append(true).open(&path)?;
        let writer = BufWriter::new(file);
        let external_backends = create_backends(logging_config);

        Ok(Self {
            writer: Mutex::new(writer),
            external_backends,
        })
    }

    /// Create a new logger with a custom log file path
    #[allow(dead_code)]
    pub fn with_path<P: Into<PathBuf>>(path: P) -> Result<Self> {
        let path = path.into();

        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)?;
        }

        let file = OpenOptions::new().create(true).append(true).open(&path)?;
        let writer = BufWriter::new(file);

        Ok(Self {
            writer: Mutex::new(writer),
            external_backends: Vec::new(),
        })
    }

    /// Get the default log file path (~/.claude/logs/rulez.log)
    pub fn default_log_path() -> PathBuf {
        let mut path = dirs::home_dir().expect("Could not determine home directory");
        path.push(".claude");
        path.push("logs");
        path.push("rulez.log");
        path
    }

    /// Log an entry to the JSON Lines file and all configured backends.
    pub fn log(&self, entry: LogEntry) -> Result<()> {
        // Always write to local JSON Lines file first
        let json = serde_json::to_string(&entry)?;
        let mut writer = self.writer.lock().unwrap();
        writeln!(writer, "{}", json)?;
        writer.flush()?;

        // Forward to external backends (fail-open)
        for backend in &self.external_backends {
            if let Err(e) = backend.send(&entry) {
                tracing::warn!(
                    "External logging backend '{}' failed: {}",
                    backend.name(),
                    e
                );
            }
        }

        Ok(())
    }

    /// Log an entry asynchronously
    pub async fn log_async(&self, entry: LogEntry) -> Result<()> {
        self.log(entry)
    }
}

// =============================================================================
// Log Query
// =============================================================================

/// Query logs with filtering and pagination
pub struct LogQuery {
    log_path: PathBuf,
}

impl LogQuery {
    /// Create a new log query for the default log file
    pub fn new() -> Self {
        Self {
            log_path: Logger::default_log_path(),
        }
    }

    /// Create a new log query for a custom log file
    #[allow(dead_code)]
    pub fn with_path<P: Into<PathBuf>>(path: P) -> Self {
        Self {
            log_path: path.into(),
        }
    }

    /// Query logs with optional filters
    pub fn query(&self, filters: QueryFilters) -> Result<Vec<LogEntry>> {
        if !self.log_path.exists() {
            return Ok(Vec::new());
        }

        let content = std::fs::read_to_string(&self.log_path)?;
        let mut entries = Vec::new();

        for line in content.lines() {
            if line.trim().is_empty() {
                continue;
            }

            let entry: LogEntry = serde_json::from_str(line)?;
            if self.matches_filters(&entry, &filters) {
                entries.push(entry);
            }
        }

        entries.sort_by(|a, b| b.timestamp.cmp(&a.timestamp));

        if let Some(limit) = filters.limit {
            entries.truncate(limit);
        }

        Ok(entries)
    }

    fn matches_filters(&self, entry: &LogEntry, filters: &QueryFilters) -> bool {
        if let Some(ref session_id) = filters.session_id {
            if &entry.session_id != session_id {
                return false;
            }
        }
        if let Some(ref tool_name) = filters.tool_name {
            if entry.tool_name.as_ref() != Some(tool_name) {
                return false;
            }
        }
        if let Some(ref rule_name) = filters.rule_name {
            if !entry.rules_matched.contains(rule_name) {
                return false;
            }
        }
        if let Some(ref outcome) = filters.outcome {
            if &entry.outcome != outcome {
                return false;
            }
        }
        if let Some(since) = filters.since {
            if entry.timestamp < since {
                return false;
            }
        }
        if let Some(until) = filters.until {
            if entry.timestamp > until {
                return false;
            }
        }
        if let Some(ref mode) = filters.mode {
            if entry.mode.as_ref() != Some(mode) {
                return false;
            }
        }
        if let Some(ref decision) = filters.decision {
            if entry.decision.as_ref() != Some(decision) {
                return false;
            }
        }
        true
    }
}

/// Filters for log queries
#[derive(Debug, Clone, Default)]
pub struct QueryFilters {
    pub limit: Option<usize>,
    pub session_id: Option<String>,
    pub tool_name: Option<String>,
    pub rule_name: Option<String>,
    pub outcome: Option<crate::models::Outcome>,
    pub since: Option<DateTime<Utc>>,
    pub until: Option<DateTime<Utc>>,
    pub mode: Option<crate::models::PolicyMode>,
    pub decision: Option<crate::models::Decision>,
}

// =============================================================================
// Global Logger
// =============================================================================

use std::sync::OnceLock;

static GLOBAL_LOGGER: OnceLock<Logger> = OnceLock::new();

/// Initialize the global logger (no external backends)
#[allow(dead_code)]
pub fn init_global_logger() -> Result<()> {
    let logger = Logger::new()?;
    GLOBAL_LOGGER
        .set(logger)
        .map_err(|_| anyhow::anyhow!("Logger already initialized"))?;
    Ok(())
}

/// Initialize the global logger with external backends from config.
pub fn init_global_logger_with_config(logging_config: &LoggingConfig) -> Result<()> {
    let logger = if logging_config.backends.is_empty() {
        Logger::new()?
    } else {
        Logger::with_backends(logging_config)?
    };
    GLOBAL_LOGGER
        .set(logger)
        .map_err(|_| anyhow::anyhow!("Logger already initialized"))?;
    Ok(())
}

/// Get the global logger instance
pub fn global_logger() -> Option<&'static Logger> {
    GLOBAL_LOGGER.get()
}

/// Log an entry using the global logger
pub async fn log_entry(entry: LogEntry) -> Result<()> {
    if let Some(logger) = global_logger() {
        logger.log_async(entry).await?;
    }
    Ok(())
}

// =============================================================================
// Log Rotation
// =============================================================================

#[allow(dead_code)]
pub struct LogRotator {
    max_size_bytes: u64,
    max_files: usize,
}

#[allow(dead_code)]
impl LogRotator {
    pub fn new(max_size_bytes: u64, max_files: usize) -> Self {
        Self {
            max_size_bytes,
            max_files,
        }
    }

    pub fn rotate_if_needed(&self, log_path: &PathBuf) -> Result<()> {
        if !log_path.exists() {
            return Ok(());
        }

        let metadata = std::fs::metadata(log_path)?;
        if metadata.len() < self.max_size_bytes {
            return Ok(());
        }

        for i in (1..self.max_files).rev() {
            let old_path = format!("{}.{}", log_path.display(), i);
            let new_path = format!("{}.{}", log_path.display(), i + 1);

            if PathBuf::from(&old_path).exists() {
                std::fs::rename(&old_path, &new_path)?;
            }
        }

        let backup_path = format!("{}.1", log_path.display());
        std::fs::rename(log_path, &backup_path)?;

        Ok(())
    }
}

impl Default for LogRotator {
    fn default() -> Self {
        Self {
            max_size_bytes: 10 * 1024 * 1024, // 10MB
            max_files: 5,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::{LogMetadata, LogTiming, Outcome};
    use tempfile::NamedTempFile;

    #[tokio::test]
    async fn test_logger() {
        let temp_file = NamedTempFile::new().unwrap();
        let logger = Logger::with_path(temp_file.path()).unwrap();

        let entry = LogEntry {
            timestamp: Utc::now(),
            event_type: "PreToolUse".to_string(),
            session_id: "test-session".to_string(),
            tool_name: Some("Bash".to_string()),
            rules_matched: vec!["test-rule".to_string()],
            outcome: Outcome::Block,
            timing: LogTiming {
                processing_ms: 5,
                rules_evaluated: 3,
            },
            metadata: Some(LogMetadata {
                injected_files: None,
                validator_output: Some("blocked by policy".to_string()),
            }),
            event_details: None,
            response: None,
            raw_event: None,
            rule_evaluations: None,
            mode: None,
            priority: None,
            decision: None,
            governance: None,
            trust_level: None,
        };

        logger.log_async(entry.clone()).await.unwrap();

        let query = LogQuery::with_path(temp_file.path());
        let filters = QueryFilters {
            limit: Some(10),
            ..Default::default()
        };

        let entries = query.query(filters).unwrap();
        assert_eq!(entries.len(), 1);
        assert_eq!(entries[0].session_id, "test-session");
    }

    #[test]
    fn test_log_filtering() {
        let temp_file = NamedTempFile::new().unwrap();
        let query = LogQuery::with_path(temp_file.path());

        let filters = QueryFilters::default();
        let entries = query.query(filters).unwrap();
        assert_eq!(entries.len(), 0);
    }

    #[test]
    fn test_expand_env_vars_no_vars() {
        assert_eq!(expand_env_vars("no_vars_here"), "no_vars_here");
        assert_eq!(expand_env_vars(""), "");
        assert_eq!(expand_env_vars("plain text"), "plain text");
    }

    #[test]
    fn test_expand_env_vars_with_existing_var() {
        // Use HOME which is always set on Unix
        let home = std::env::var("HOME").unwrap_or_default();
        if !home.is_empty() {
            assert_eq!(expand_env_vars("${HOME}"), home);
            assert_eq!(
                expand_env_vars("prefix_${HOME}_suffix"),
                format!("prefix_{}_suffix", home)
            );
        }
    }

    #[test]
    fn test_expand_env_vars_missing_var() {
        // Unset vars expand to empty string
        assert_eq!(expand_env_vars("${RULEZ_DEFINITELY_UNSET_VAR_12345}"), "");
    }

    #[test]
    fn test_backend_config_deserialization() {
        let yaml = r#"
backends:
  - type: otlp
    endpoint: "http://localhost:4318/v1/logs"
    headers:
      Authorization: "Bearer test"
    timeout_secs: 10
  - type: datadog
    api_key: "${DD_API_KEY}"
  - type: splunk
    endpoint: "https://splunk.example.com:8088/services/collector/event"
    token: "${SPLUNK_TOKEN}"
"#;
        let config: LoggingConfig = serde_yaml::from_str(yaml).unwrap();
        assert_eq!(config.backends.len(), 3);
    }

    #[test]
    fn test_create_backends() {
        let config = LoggingConfig {
            backends: vec![BackendConfig::Otlp {
                endpoint: "http://localhost:4318/v1/logs".to_string(),
                headers: HashMap::new(),
                timeout_secs: 5,
            }],
        };
        let backends = create_backends(&config);
        assert_eq!(backends.len(), 1);
        assert_eq!(backends[0].name(), "otlp");
    }

    #[test]
    fn test_default_logging_config() {
        let config = LoggingConfig::default();
        assert!(config.backends.is_empty());
    }

    #[test]
    fn test_logger_with_no_backends() {
        let temp_file = NamedTempFile::new().unwrap();
        let logger = Logger::with_path(temp_file.path()).unwrap();
        assert!(logger.external_backends.is_empty());
    }

    #[test]
    fn test_logger_with_backends_config() {
        let config = LoggingConfig {
            backends: vec![BackendConfig::Otlp {
                endpoint: "http://localhost:4318/v1/logs".to_string(),
                headers: HashMap::new(),
                timeout_secs: 5,
            }],
        };
        let logger = Logger::with_backends(&config).unwrap();
        assert_eq!(logger.external_backends.len(), 1);
    }
}
