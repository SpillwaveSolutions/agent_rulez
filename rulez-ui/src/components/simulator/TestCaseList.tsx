import { useTestCaseStore } from "@/stores/testCaseStore";
import type { DebugParams } from "@/types";

interface TestCaseListProps {
  onLoad: (params: DebugParams) => void;
}

export function TestCaseList({ onLoad }: TestCaseListProps) {
  const testCases = useTestCaseStore((s) => s.testCases);
  const deleteTestCase = useTestCaseStore((s) => s.deleteTestCase);

  if (testCases.length === 0) {
    return (
      <p className="text-xs text-gray-400 dark:text-gray-500 text-center py-2">
        No saved test cases
      </p>
    );
  }

  return (
    <div className="space-y-1 max-h-40 overflow-y-auto">
      {testCases.map((tc) => (
        <div
          key={tc.id}
          className="flex items-center justify-between gap-1 px-2 py-1.5 rounded text-xs bg-gray-50 dark:bg-[#1A1A1A] border border-gray-200 dark:border-gray-700"
        >
          <button
            type="button"
            className="flex-1 text-left truncate text-gray-700 dark:text-gray-300 hover:text-accent transition-colors"
            onClick={() => onLoad(tc.params)}
            title={`Load: ${tc.name}`}
          >
            <span className="font-medium">{tc.params.eventType}</span>
            {tc.params.tool && (
              <span className="text-gray-500 dark:text-gray-400"> - {tc.params.tool}</span>
            )}
          </button>
          <button
            type="button"
            className="flex-shrink-0 text-gray-400 hover:text-red-500 transition-colors px-1"
            onClick={() => deleteTestCase(tc.id)}
            title="Delete test case"
          >
            x
          </button>
        </div>
      ))}
    </div>
  );
}
