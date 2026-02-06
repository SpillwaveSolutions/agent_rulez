import { useEditorStore } from "@/stores/editorStore";
import type { ValidationError, ValidationWarning } from "@/types";
import { useCallback } from "react";

interface ValidationItemProps {
  item: ValidationError | ValidationWarning;
  type: "error" | "warning";
  onClick: (line: number) => void;
}

function ValidationItem({ item, type, onClick }: ValidationItemProps) {
  const isError = type === "error";

  return (
    <button
      type="button"
      onClick={() => onClick(item.line)}
      className={`w-full text-left px-3 py-2 flex items-start gap-2 hover:bg-gray-100 dark:hover:bg-gray-800 transition-colors border-l-2 ${
        isError ? "border-error dark:border-error-dark" : "border-warning dark:border-warning-dark"
      }`}
    >
      {/* Icon */}
      <span
        className={`flex-shrink-0 mt-0.5 ${isError ? "text-error dark:text-error-dark" : "text-warning dark:text-warning-dark"}`}
      >
        {isError ? (
          <svg className="w-4 h-4" fill="currentColor" viewBox="0 0 20 20" aria-hidden="true">
            <path
              fillRule="evenodd"
              d="M18 10a8 8 0 11-16 0 8 8 0 0116 0zm-7 4a1 1 0 11-2 0 1 1 0 012 0zm-1-9a1 1 0 00-1 1v4a1 1 0 102 0V6a1 1 0 00-1-1z"
              clipRule="evenodd"
            />
          </svg>
        ) : (
          <svg className="w-4 h-4" fill="currentColor" viewBox="0 0 20 20" aria-hidden="true">
            <path
              fillRule="evenodd"
              d="M8.257 3.099c.765-1.36 2.722-1.36 3.486 0l5.58 9.92c.75 1.334-.213 2.98-1.742 2.98H4.42c-1.53 0-2.493-1.646-1.743-2.98l5.58-9.92zM11 13a1 1 0 11-2 0 1 1 0 012 0zm-1-8a1 1 0 00-1 1v3a1 1 0 002 0V6a1 1 0 00-1-1z"
              clipRule="evenodd"
            />
          </svg>
        )}
      </span>

      {/* Content */}
      <div className="flex-1 min-w-0">
        <div className="flex items-center gap-2">
          <span className="text-xs font-mono text-gray-500 dark:text-gray-400">
            Ln {item.line}:{item.column}
          </span>
        </div>
        <p className="text-sm text-gray-700 dark:text-gray-300 break-words">{item.message}</p>
      </div>
    </button>
  );
}

export function ValidationPanel() {
  const errors = useEditorStore((s) => s.errors);
  const warnings = useEditorStore((s) => s.warnings);
  const editorRef = useEditorStore((s) => s.editorRef);

  const handleNavigate = useCallback(
    (line: number) => {
      if (editorRef) {
        editorRef.revealLineInCenter(line);
        editorRef.setPosition({ lineNumber: line, column: 1 });
        editorRef.focus();
      }
    },
    [editorRef],
  );

  const hasIssues = errors.length > 0 || warnings.length > 0;

  if (!hasIssues) {
    return null;
  }

  return (
    <div className="border-t border-gray-200 dark:border-gray-700 bg-white dark:bg-[#1A1A1A] max-h-40 overflow-y-auto">
      {/* Header */}
      <div className="sticky top-0 bg-gray-50 dark:bg-gray-900 px-3 py-1.5 border-b border-gray-200 dark:border-gray-700 flex items-center gap-4 text-xs font-medium text-gray-600 dark:text-gray-400">
        <span>Problems</span>
        {errors.length > 0 && (
          <span className="flex items-center gap-1 text-error dark:text-error-dark">
            <svg className="w-3.5 h-3.5" fill="currentColor" viewBox="0 0 20 20" aria-hidden="true">
              <path
                fillRule="evenodd"
                d="M18 10a8 8 0 11-16 0 8 8 0 0116 0zm-7 4a1 1 0 11-2 0 1 1 0 012 0zm-1-9a1 1 0 00-1 1v4a1 1 0 102 0V6a1 1 0 00-1-1z"
                clipRule="evenodd"
              />
            </svg>
            {errors.length}
          </span>
        )}
        {warnings.length > 0 && (
          <span className="flex items-center gap-1 text-warning dark:text-warning-dark">
            <svg className="w-3.5 h-3.5" fill="currentColor" viewBox="0 0 20 20" aria-hidden="true">
              <path
                fillRule="evenodd"
                d="M8.257 3.099c.765-1.36 2.722-1.36 3.486 0l5.58 9.92c.75 1.334-.213 2.98-1.742 2.98H4.42c-1.53 0-2.493-1.646-1.743-2.98l5.58-9.92zM11 13a1 1 0 11-2 0 1 1 0 012 0zm-1-8a1 1 0 00-1 1v3a1 1 0 002 0V6a1 1 0 00-1-1z"
                clipRule="evenodd"
              />
            </svg>
            {warnings.length}
          </span>
        )}
      </div>

      {/* Items */}
      <div className="divide-y divide-gray-100 dark:divide-gray-800">
        {errors.map((error, idx) => (
          <ValidationItem
            key={`error-${idx}-${error.line}-${error.column}`}
            item={error}
            type="error"
            onClick={handleNavigate}
          />
        ))}
        {warnings.map((warning, idx) => (
          <ValidationItem
            key={`warning-${idx}-${warning.line}-${warning.column}`}
            item={warning}
            type="warning"
            onClick={handleNavigate}
          />
        ))}
      </div>
    </div>
  );
}
