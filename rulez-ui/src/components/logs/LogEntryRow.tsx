import { deriveSeverity } from "@/lib/log-utils";
import type { LogEntryDto } from "@/types";
import { useCallback, useState } from "react";

const SEVERITY_STYLES = {
  error: "bg-red-100 text-red-800 dark:bg-red-900 dark:text-red-200",
  warn: "bg-yellow-100 text-yellow-800 dark:bg-yellow-900 dark:text-yellow-200",
  info: "bg-blue-100 text-blue-800 dark:bg-blue-900 dark:text-blue-200",
  debug: "bg-gray-100 text-gray-800 dark:bg-gray-900 dark:text-gray-200",
} as const;

interface LogEntryRowProps {
  entry: LogEntryDto;
  style: React.CSSProperties;
  onCopy?: (entry: LogEntryDto) => void;
}

export function LogEntryRow({ entry, style, onCopy }: LogEntryRowProps) {
  const [copied, setCopied] = useState(false);
  const severity = deriveSeverity(entry);

  const handleCopy = useCallback(() => {
    onCopy?.(entry);
    setCopied(true);
    setTimeout(() => setCopied(false), 1500);
  }, [entry, onCopy]);

  const time = entry.timestamp
    ? new Date(entry.timestamp).toLocaleTimeString("en-US", {
        hour12: false,
        hour: "2-digit",
        minute: "2-digit",
        second: "2-digit",
        fractionalSecondDigits: 3,
      })
    : "--:--:--";

  const summary =
    entry.rulesMatched.length > 0
      ? `${entry.rulesMatched.length} rule(s): ${entry.rulesMatched.join(", ")}`
      : (entry.responseReason ?? "");

  return (
    <div
      data-testid="log-entry"
      style={style}
      className="flex items-center gap-2 px-3 text-xs border-b border-gray-100 dark:border-gray-800 hover:bg-gray-50 dark:hover:bg-gray-800/50"
    >
      {/* Timestamp */}
      <span className="w-24 shrink-0 font-mono text-gray-500 dark:text-gray-400">{time}</span>

      {/* Severity badge */}
      <span
        className={`w-14 shrink-0 text-center rounded px-1 py-0.5 font-medium ${SEVERITY_STYLES[severity]}`}
      >
        {severity}
      </span>

      {/* Event type */}
      <span className="w-28 shrink-0 text-gray-700 dark:text-gray-300 truncate">
        {entry.eventType}
      </span>

      {/* Tool name */}
      <span className="w-16 shrink-0 text-gray-600 dark:text-gray-400 truncate">
        {entry.toolName ?? "-"}
      </span>

      {/* Outcome */}
      <span className="w-14 shrink-0 text-gray-600 dark:text-gray-400">{entry.outcome}</span>

      {/* Summary / matched rules */}
      <span className="flex-1 text-gray-500 dark:text-gray-400 truncate">{summary}</span>

      {/* Copy button */}
      <button
        type="button"
        onClick={handleCopy}
        className="w-14 shrink-0 text-center px-1 py-0.5 rounded text-gray-500 hover:bg-gray-200 dark:hover:bg-gray-700 dark:text-gray-400 transition-colors"
        aria-label="Copy"
      >
        {copied ? "Copied!" : "Copy"}
      </button>
    </div>
  );
}
