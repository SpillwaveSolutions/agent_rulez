import type { LogEntryDto, Severity } from "@/types";

/**
 * Derive a severity level from a log entry's outcome and decision fields.
 */
export function deriveSeverity(entry: LogEntryDto): Severity {
  if (entry.outcome === "block" || entry.decision === "blocked") return "error";
  if (entry.decision === "warned" || entry.mode === "warn") return "warn";
  return "info";
}

/**
 * Convert log entries to formatted JSON string.
 */
export function entriesToJson(entries: LogEntryDto[]): string {
  return JSON.stringify(entries, null, 2);
}

/**
 * Escape a CSV field value (wrap in quotes if it contains commas, quotes, or newlines).
 */
function escapeCsvField(value: string): string {
  if (value.includes(",") || value.includes('"') || value.includes("\n")) {
    return `"${value.replace(/"/g, '""')}"`;
  }
  return value;
}

/**
 * Convert log entries to CSV string with headers.
 */
export function entriesToCsv(entries: LogEntryDto[]): string {
  const headers = [
    "timestamp",
    "event_type",
    "tool_name",
    "outcome",
    "decision",
    "mode",
    "rules_matched",
    "processing_ms",
    "session_id",
    "response_reason",
  ];

  const rows = entries.map((e) =>
    [
      escapeCsvField(e.timestamp),
      escapeCsvField(e.eventType),
      escapeCsvField(e.toolName ?? ""),
      escapeCsvField(e.outcome),
      escapeCsvField(e.decision ?? ""),
      escapeCsvField(e.mode ?? ""),
      escapeCsvField(e.rulesMatched.join("; ")),
      String(e.processingMs),
      escapeCsvField(e.sessionId),
      escapeCsvField(e.responseReason ?? ""),
    ].join(","),
  );

  return [headers.join(","), ...rows].join("\n");
}
