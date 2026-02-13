use anyhow::Result;
use chrono::Utc;
use uuid::Uuid;

use crate::adapters::opencode::{OpenCodeEvent, translate_response};
use crate::hooks;
use crate::models::{DebugConfig, Response};
use crate::opencode::audit::{OpenCodeAuditEntry, OpenCodeAuditLogger};
use crate::opencode::config::OpenCodePluginConfig;

pub struct OpenCodeDispatcher {
    config: OpenCodePluginConfig,
    logger: OpenCodeAuditLogger,
}

impl OpenCodeDispatcher {
    pub fn new(config: OpenCodePluginConfig) -> Self {
        let logger = OpenCodeAuditLogger::new(config.audit_log_path.clone());
        Self { config, logger }
    }

    pub async fn dispatch(
        &self,
        opencode_event: OpenCodeEvent,
        debug_config: &DebugConfig,
    ) -> Result<serde_json::Value> {
        let start_time = std::time::Instant::now();
        let event_id = Uuid::new_v4().to_string();

        // Apply event filters
        if self
            .config
            .event_filters
            .contains(&opencode_event.hook_event_name)
        {
            return Ok(translate_response(&Response::allow(), &opencode_event));
        }

        let response = hooks::process_event(opencode_event.event.clone(), debug_config).await?;
        let latency_ms = start_time.elapsed().as_millis() as u64;

        // Audit logging
        let entry = OpenCodeAuditEntry {
            timestamp: Utc::now(),
            event_id,
            event_name: opencode_event.hook_event_name.clone(),
            decision: if response.continue_ {
                "allow".to_string()
            } else {
                "deny".to_string()
            },
            reason: response.reason.clone(),
            latency_ms,
            plugin_name: "rulez-plugin".to_string(),
            plugin_version: env!("CARGO_PKG_VERSION").to_string(),
            session_id: opencode_event.event.session_id.clone(),
        };

        self.logger.log_async(entry).await;

        Ok(translate_response(&response, &opencode_event))
    }
}
