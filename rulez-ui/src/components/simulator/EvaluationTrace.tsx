import type { RuleEvaluation } from "@/types";
import { useState } from "react";

interface EvaluationTraceProps {
  evaluations: RuleEvaluation[];
}

export function EvaluationTrace({ evaluations }: EvaluationTraceProps) {
  const defaultExpanded = evaluations.length <= 5;
  const [expanded, setExpanded] = useState(defaultExpanded);

  if (evaluations.length === 0) return null;

  return (
    <div className="rounded border border-gray-200 dark:border-gray-700 bg-white dark:bg-[#1A1A1A]">
      <button
        type="button"
        onClick={() => setExpanded(!expanded)}
        className="w-full flex items-center justify-between px-3 py-2 text-xs font-semibold text-gray-700 dark:text-gray-300 hover:bg-gray-50 dark:hover:bg-gray-800/50 transition-colors"
      >
        <span>Evaluation Trace ({evaluations.length})</span>
        <svg
          className={`w-4 h-4 transition-transform ${expanded ? "rotate-180" : ""}`}
          fill="none"
          stroke="currentColor"
          viewBox="0 0 24 24"
          aria-hidden="true"
        >
          <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M19 9l-7 7-7-7" />
        </svg>
      </button>

      {expanded && (
        <div className="border-t border-gray-200 dark:border-gray-700 divide-y divide-gray-100 dark:divide-gray-800">
          {evaluations.map((evaluation) => (
            <EvaluationItem key={evaluation.ruleName} evaluation={evaluation} />
          ))}
        </div>
      )}
    </div>
  );
}

function EvaluationItem({ evaluation }: { evaluation: RuleEvaluation }) {
  return (
    <div className="px-3 py-2 space-y-1">
      <div className="flex items-center justify-between">
        <div className="flex items-center gap-1.5">
          {evaluation.matched ? (
            <svg
              className="w-3.5 h-3.5 text-green-500 dark:text-green-400"
              fill="none"
              stroke="currentColor"
              viewBox="0 0 24 24"
              aria-hidden="true"
            >
              <path
                strokeLinecap="round"
                strokeLinejoin="round"
                strokeWidth={2}
                d="M5 13l4 4L19 7"
              />
            </svg>
          ) : (
            <svg
              className="w-3.5 h-3.5 text-gray-400 dark:text-gray-500"
              fill="none"
              stroke="currentColor"
              viewBox="0 0 24 24"
              aria-hidden="true"
            >
              <path
                strokeLinecap="round"
                strokeLinejoin="round"
                strokeWidth={2}
                d="M6 18L18 6M6 6l12 12"
              />
            </svg>
          )}
          <span className="text-sm font-medium text-gray-900 dark:text-gray-100">
            {evaluation.ruleName}
          </span>
        </div>
        <span className="text-xs text-gray-500 dark:text-gray-400">
          {evaluation.timeMs.toFixed(1)} ms
        </span>
      </div>

      {evaluation.pattern && (
        <div className="text-xs">
          <span className="text-gray-500 dark:text-gray-400">Pattern: </span>
          <code className="font-mono text-gray-700 dark:text-gray-300 bg-gray-100 dark:bg-gray-800 px-1 py-0.5 rounded">
            {evaluation.pattern}
          </code>
        </div>
      )}

      {evaluation.input && (
        <div className="text-xs">
          <span className="text-gray-500 dark:text-gray-400">Input: </span>
          <code className="font-mono text-gray-700 dark:text-gray-300 bg-gray-100 dark:bg-gray-800 px-1 py-0.5 rounded">
            {evaluation.input}
          </code>
        </div>
      )}

      {evaluation.details && (
        <p className="text-xs text-gray-500 dark:text-gray-400">{evaluation.details}</p>
      )}
    </div>
  );
}
