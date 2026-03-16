---
last_modified: 2026-03-16
last_validated: 2026-03-16
---

# External Logging Backends

RuleZ always writes audit logs to a local NDJSON file (`~/.claude/logs/rulez.log`). External logging backends let you forward those same log entries to centralized observability platforms -- OTLP collectors, Datadog, or Splunk -- so your team can monitor policy enforcement alongside other infrastructure telemetry.

Use external logging when you need:
- **Compliance auditing** -- immutable log trail in a central store
- **Real-time monitoring** -- dashboards and alerts on policy violations
- **Team visibility** -- multiple engineers sharing one log pipeline

## Prerequisites

- RuleZ v2.0 or later installed (`rulez --version`)
- A running log collector or backend account (OTLP collector, Datadog, or Splunk)
- Network connectivity from the machine running RuleZ to the backend endpoint

## Quick Start

Add an OTLP backend to your `hooks.yaml` in under a minute:

```yaml
version: "1"

rules:
  - name: audit-file-writes
    description: "Log all file write operations"
    mode: audit
    matchers:
      tools: ["Write"]
    actions:
      inject_inline: "Audit: file write detected"

settings:
  logging:
    backends:
      - type: otlp
        endpoint: "http://localhost:4318/v1/logs"
```

Trigger a rule to verify logs flow:

```bash
rulez debug PreToolUse --tool Write --path test.txt -v
```

Expected output:

```
Event: PreToolUse
Tool:  Write
Rules matched: audit-file-writes
Outcome: Allow (audit mode)
```

Check your OTLP collector (e.g., Grafana, Jaeger) -- a log entry should appear within seconds.

## When to Use Which Backend

| Backend | Protocol | Auth Method | Best For |
|---------|----------|-------------|----------|
| OTLP | HTTP POST to `/v1/logs` | Headers (Bearer token) | OpenTelemetry ecosystems (Grafana, Jaeger, etc.) |
| Datadog | HTTP POST | API key | Datadog customers, managed monitoring |
| Splunk | HTTP Event Collector (HEC) | HEC token | Splunk/SIEM environments, enterprise security |

All three backends use `curl` under the hood (no compiled-in TLS dependency). RuleZ ships log entries as JSON via HTTP POST, so any backend that accepts JSON over HTTP will work.

## Configuration Reference

Backends are configured under `settings.logging.backends` in `hooks.yaml`. Each entry requires a `type` field and backend-specific options.

All string fields support `${VAR}` environment variable expansion. Use this to keep secrets (API keys, tokens) out of your configuration file.

### OTLP

| Field | Type | Required | Default | Description |
|-------|------|----------|---------|-------------|
| `type` | string | Yes | -- | Must be `"otlp"`. |
| `endpoint` | string | Yes | -- | OTLP HTTP endpoint URL (e.g., `http://localhost:4318/v1/logs`). |
| `headers` | map | No | `{}` | Additional HTTP headers. Common use: Bearer tokens. |
| `timeout_secs` | integer | No | `5` | HTTP request timeout in seconds. |

### Datadog

| Field | Type | Required | Default | Description |
|-------|------|----------|---------|-------------|
| `type` | string | Yes | -- | Must be `"datadog"`. |
| `api_key` | string | Yes | -- | Datadog API key. Use `${DD_API_KEY}` for env var expansion. |
| `endpoint` | string | No | `https://http-intake.logs.datadoghq.com/api/v2/logs` | Datadog logs API endpoint. Change for EU region. |
| `timeout_secs` | integer | No | `5` | HTTP request timeout in seconds. |

### Splunk

| Field | Type | Required | Default | Description |
|-------|------|----------|---------|-------------|
| `type` | string | Yes | -- | Must be `"splunk"`. |
| `endpoint` | string | Yes | -- | Splunk HEC endpoint URL (e.g., `https://splunk.example.com:8088/services/collector/event`). |
| `token` | string | Yes | -- | Splunk HEC token. Use `${SPLUNK_HEC_TOKEN}` for env var expansion. |
| `sourcetype` | string | No | `"rulez"` | Splunk sourcetype for indexed events. |
| `timeout_secs` | integer | No | `5` | HTTP request timeout in seconds. |

## OTLP Backend

### Full configuration example

```yaml
version: "1"

rules:
  - name: deny-rm-rf
    description: "Block recursive deletion of root paths"
    matchers:
      tools: ["Bash"]
      command_match: "rm\\s+(-rf|-fr)\\s+/"
    actions:
      block: true

  - name: audit-file-writes
    description: "Log all file write operations"
    mode: audit
    matchers:
      tools: ["Write"]
    actions:
      inject_inline: "Audit: file write detected"

settings:
  logging:
    backends:
      - type: otlp
        endpoint: "http://localhost:4318/v1/logs"
        headers:
          Authorization: "Bearer ${OTEL_TOKEN}"
        timeout_secs: 10
```

### Sample JSON payload

RuleZ wraps log entries in the OTLP `resourceLogs` envelope. Your collector receives a payload like this:

```json
{
  "resourceLogs": [
    {
      "resource": {
        "attributes": [
          {
            "key": "service.name",
            "value": { "stringValue": "rulez" }
          }
        ]
      },
      "scopeLogs": [
        {
          "scope": { "name": "rulez.audit" },
          "logRecords": [
            {
              "timeUnixNano": "1710000000000000000",
              "severityNumber": 9,
              "severityText": "INFO",
              "body": {
                "stringValue": "{\"timestamp\":\"2026-03-11T14:30:00Z\",\"event_type\":\"PreToolUse\",\"session_id\":\"abc-123\",\"tool_name\":\"Write\",\"rules_matched\":[\"audit-file-writes\"],\"outcome\":\"Allow\",\"timing\":{\"processing_ms\":2,\"rules_evaluated\":3}}"
              },
              "attributes": [
                {
                  "key": "rulez.event_type",
                  "value": { "stringValue": "PreToolUse" }
                },
                {
                  "key": "rulez.session_id",
                  "value": { "stringValue": "abc-123" }
                },
                {
                  "key": "rulez.outcome",
                  "value": { "stringValue": "Allow" }
                }
              ]
            }
          ]
        }
      ]
    }
  ]
}
```

For the full event payload structure, see [Event Schema](../event-schema.md).

### Verification steps

1. Start your OTLP collector (e.g., OpenTelemetry Collector, Grafana Alloy):

   ```bash
   # Example with the OpenTelemetry Collector
   otelcol --config otel-config.yaml
   ```

2. Trigger a rule:

   ```bash
   rulez debug PreToolUse --tool Write --path test.txt -v
   ```

3. Check the collector logs or your observability platform for an entry with `service.name: rulez`.

4. Verify the local log also recorded the event:

   ```bash
   rulez logs --limit 1
   ```

   Expected output:

   ```
   2026-03-11T14:30:00Z | PreToolUse | Write | audit-file-writes | Allow | 2ms
   ```

## Datadog Backend

### Full configuration example

```yaml
version: "1"

rules:
  - name: audit-file-writes
    description: "Log all file write operations"
    mode: audit
    matchers:
      tools: ["Write"]
    actions:
      inject_inline: "Audit: file write detected"

settings:
  logging:
    backends:
      - type: datadog
        api_key: "${DD_API_KEY}"
        timeout_secs: 5
```

For Datadog EU, override the endpoint:

```yaml
      - type: datadog
        api_key: "${DD_API_KEY}"
        endpoint: "https://http-intake.logs.datadoghq.eu/api/v2/logs"
```

### Sample JSON payload

RuleZ sends a Datadog-formatted log entry array:

```json
[
  {
    "ddsource": "rulez",
    "ddtags": "event_type:PreToolUse,outcome:Allow",
    "hostname": "dev-laptop",
    "message": "{\"timestamp\":\"2026-03-11T14:30:00Z\",\"event_type\":\"PreToolUse\",\"session_id\":\"abc-123\",\"tool_name\":\"Write\",\"rules_matched\":[\"audit-file-writes\"],\"outcome\":\"Allow\",\"timing\":{\"processing_ms\":2,\"rules_evaluated\":3}}",
    "service": "rulez",
    "status": "info"
  }
]
```

The `status` field is `"error"` when the outcome is `Block`, and `"info"` otherwise. For the full event payload structure, see [Event Schema](../event-schema.md).

### Verification steps

1. Export your Datadog API key:

   ```bash
   export DD_API_KEY="your-api-key-here"
   ```

2. Trigger a rule:

   ```bash
   rulez debug PreToolUse --tool Write --path test.txt -v
   ```

3. In the Datadog console, navigate to **Logs** and filter by `service:rulez`. You should see the log entry within 30 seconds.

4. Verify the local log also recorded the event:

   ```bash
   rulez logs --limit 1
   ```

## Splunk Backend

### Full configuration example

```yaml
version: "1"

rules:
  - name: deny-rm-rf
    description: "Block recursive deletion of root paths"
    matchers:
      tools: ["Bash"]
      command_match: "rm\\s+(-rf|-fr)\\s+/"
    actions:
      block: true

settings:
  logging:
    backends:
      - type: splunk
        endpoint: "https://splunk.example.com:8088/services/collector/event"
        token: "${SPLUNK_HEC_TOKEN}"
        sourcetype: "rulez"
        timeout_secs: 5
```

### Sample JSON payload

RuleZ sends a Splunk HEC-formatted event:

```json
{
  "event": {
    "timestamp": "2026-03-11T14:30:00Z",
    "event_type": "PreToolUse",
    "session_id": "abc-123",
    "tool_name": "Bash",
    "rules_matched": ["deny-rm-rf"],
    "outcome": "Block",
    "timing": {
      "processing_ms": 1,
      "rules_evaluated": 2
    }
  },
  "sourcetype": "rulez",
  "source": "rulez",
  "host": "dev-laptop",
  "time": 1710000000
}
```

For the full event payload structure, see [Event Schema](../event-schema.md).

### Verification steps

1. Export your Splunk HEC token:

   ```bash
   export SPLUNK_HEC_TOKEN="your-hec-token-here"
   ```

2. Trigger a rule:

   ```bash
   rulez debug PreToolUse --tool Bash --command "rm -rf /tmp/test" -v
   ```

3. In Splunk, search for events with `sourcetype=rulez`:

   ```
   index=main sourcetype=rulez
   ```

4. Verify the local log also recorded the event:

   ```bash
   rulez logs --limit 1
   ```

## Combining Multiple Backends

You can configure multiple backends simultaneously. Every log entry is sent to all configured backends (fan-out). Backend failures are fail-open -- a single backend being unreachable does not block local logging or other backends.

```yaml
version: "1"

rules:
  - name: audit-file-writes
    description: "Log all file write operations"
    mode: audit
    matchers:
      tools: ["Write"]
    actions:
      inject_inline: "Audit: file write detected"

  - name: deny-rm-rf
    description: "Block recursive deletion of root paths"
    matchers:
      tools: ["Bash"]
      command_match: "rm\\s+(-rf|-fr)\\s+/"
    actions:
      block: true

settings:
  logging:
    backends:
      # Send to OTLP collector for dashboards
      - type: otlp
        endpoint: "http://localhost:4318/v1/logs"
        headers:
          Authorization: "Bearer ${OTEL_TOKEN}"

      # Also send to Datadog for alerting
      - type: datadog
        api_key: "${DD_API_KEY}"

      # Also send to Splunk for compliance
      - type: splunk
        endpoint: "https://splunk.example.com:8088/services/collector/event"
        token: "${SPLUNK_HEC_TOKEN}"
```

All three backends receive every log entry. If one backend is down, the other two still receive logs and the local `rulez.log` file is always written.

## Troubleshooting

### Backend not receiving logs

1. **Check the endpoint URL.** Ensure the URL is correct and accessible from your machine:

   ```bash
   curl -s -o /dev/null -w "%{http_code}" -X POST \
     -H "Content-Type: application/json" \
     -d '{"test": true}' \
     http://localhost:4318/v1/logs
   ```

   A `200` or `204` response means the endpoint is reachable.

2. **Check authentication.** Ensure your API key, token, or Bearer header is correct. Environment variables must be exported (not just set):

   ```bash
   # Wrong: variable not exported
   DD_API_KEY="my-key"

   # Right: variable exported
   export DD_API_KEY="my-key"
   ```

3. **Check RuleZ logs for warnings.** Backend failures produce warning messages in stderr:

   ```
   WARN: External logging backend 'otlp' failed: curl exited with status: exit status: 7
   ```

   Exit status 7 means `curl` could not connect to the host.

4. **Verify rules are matching.** Run `rulez debug` to confirm rules fire before checking the backend:

   ```bash
   rulez debug PreToolUse --tool Write --path test.txt -v
   ```

### Environment variable not expanded

If your config uses `${DD_API_KEY}` but the backend receives an empty key:

- Ensure the variable is **exported** in your shell: `export DD_API_KEY="..."`
- Check for typos in the variable name (case-sensitive)
- RuleZ expands variables at startup. If you change a variable, restart your AI coding assistant session

### Timeout errors

If logs are slow to arrive or you see timeout warnings:

- Increase `timeout_secs` (default is 5):

  ```yaml
  - type: otlp
    endpoint: "http://collector.internal:4318/v1/logs"
    timeout_secs: 15
  ```

- Check network latency between your machine and the backend
- For high-latency endpoints, consider running a local OTLP collector as a relay

### Wrong Datadog region

Datadog's default endpoint is US (`datadoghq.com`). For EU, override the endpoint:

```yaml
- type: datadog
  api_key: "${DD_API_KEY}"
  endpoint: "https://http-intake.logs.datadoghq.eu/api/v2/logs"
```

Other Datadog regions:

| Region | Endpoint |
|--------|----------|
| US1 (default) | `https://http-intake.logs.datadoghq.com/api/v2/logs` |
| EU1 | `https://http-intake.logs.datadoghq.eu/api/v2/logs` |
| US3 | `https://http-intake.logs.us3.datadoghq.com/api/v2/logs` |
| US5 | `https://http-intake.logs.us5.datadoghq.com/api/v2/logs` |
| AP1 | `https://http-intake.logs.ap1.datadoghq.com/api/v2/logs` |

## Further Reading

- [Configuration Schema](../config-schema.md) -- full `hooks.yaml` settings reference
- [Event Schema](../event-schema.md) -- event payload structure and response format
- [YAML Field Reference](../../mastering-hooks/references/hooks-yaml-schema.md) -- all rule matcher and action fields
- [CLI Commands](../../mastering-hooks/references/cli-commands.md) -- `rulez debug`, `rulez logs`, and other CLI flags
