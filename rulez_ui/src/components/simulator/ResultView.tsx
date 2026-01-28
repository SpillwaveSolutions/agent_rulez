import type { DebugResult } from "@/types";

interface ResultViewProps {
  result: DebugResult | null;
}

const OUTCOME_STYLES: Record<string, string> = {
  Allow: "bg-green-100 text-green-800 dark:bg-green-900/30 dark:text-green-300",
  Block: "bg-red-100 text-red-800 dark:bg-red-900/30 dark:text-red-300",
  Inject: "bg-blue-100 text-blue-800 dark:bg-blue-900/30 dark:text-blue-300",
};

export function ResultView({ result }: ResultViewProps) {
  if (!result) return null;

  return (
    <div className="rounded border border-gray-200 dark:border-gray-700 bg-white dark:bg-[#1A1A1A] p-3 space-y-2">
      <div className="flex items-center justify-between">
        <span
          className={`inline-block px-2 py-0.5 rounded text-xs font-semibold ${OUTCOME_STYLES[result.outcome] ?? ""}`}
        >
          {result.outcome}
        </span>
        <span className="text-xs text-gray-500 dark:text-gray-400">
          {result.evaluationTimeMs.toFixed(1)} ms
        </span>
      </div>

      {result.reason && <p className="text-sm text-gray-700 dark:text-gray-300">{result.reason}</p>}

      <p className="text-xs text-gray-500 dark:text-gray-400">
        {result.matchedRules.length} rule{result.matchedRules.length !== 1 ? "s" : ""} matched
      </p>
    </div>
  );
}
