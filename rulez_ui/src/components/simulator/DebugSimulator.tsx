import { EvaluationTrace } from "@/components/simulator/EvaluationTrace";
import { EventForm } from "@/components/simulator/EventForm";
import { ResultView } from "@/components/simulator/ResultView";
import { TestCaseList } from "@/components/simulator/TestCaseList";
import { runDebug } from "@/lib/tauri";
import { useTestCaseStore } from "@/stores/testCaseStore";
import type { DebugParams, DebugResult } from "@/types";
import { useCallback, useRef, useState } from "react";

export function DebugSimulator() {
  const [result, setResult] = useState<DebugResult | null>(null);
  const [lastParams, setLastParams] = useState<DebugParams | null>(null);
  const [isLoading, setIsLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);
  const [saveMessage, setSaveMessage] = useState<string | null>(null);
  const [showTestCases, setShowTestCases] = useState(false);

  const formRef = useRef<{ setParams: (params: DebugParams) => void }>(null);

  const saveTestCase = useTestCaseStore((s) => s.saveTestCase);
  const testCases = useTestCaseStore((s) => s.testCases);

  async function handleSubmit(params: DebugParams) {
    setIsLoading(true);
    setError(null);
    setSaveMessage(null);
    setLastParams(params);
    try {
      const debugResult = await runDebug(params);
      setResult(debugResult);
    } catch (err) {
      setError(err instanceof Error ? err.message : "Simulation failed");
      setResult(null);
    } finally {
      setIsLoading(false);
    }
  }

  function handleSave() {
    if (!lastParams) return;
    saveTestCase(lastParams, result ?? undefined);
    setSaveMessage("Test case saved!");
    setTimeout(() => setSaveMessage(null), 2000);
  }

  const handleLoadTestCase = useCallback((params: DebugParams) => {
    formRef.current?.setParams(params);
    setShowTestCases(false);
  }, []);

  return (
    <div className="space-y-4">
      <div>
        <h3 className="text-sm font-semibold text-gray-700 dark:text-gray-300">Debug Simulator</h3>
        <p className="text-xs text-gray-500 dark:text-gray-400 mt-1">
          Test your RuleZ rules by simulating events.
        </p>
      </div>

      <EventForm ref={formRef} onSubmit={handleSubmit} isLoading={isLoading} />

      {/* Save / Load buttons */}
      <div className="flex gap-2">
        {lastParams && (
          <button
            type="button"
            onClick={handleSave}
            className="flex-1 px-3 py-1.5 text-xs font-medium text-gray-700 dark:text-gray-300 bg-gray-100 dark:bg-[#1A1A1A] border border-gray-300 dark:border-gray-600 rounded hover:bg-gray-200 dark:hover:bg-gray-700 transition-colors"
          >
            Save Test Case
          </button>
        )}
        {testCases.length > 0 && (
          <button
            type="button"
            onClick={() => setShowTestCases(!showTestCases)}
            className="flex-1 px-3 py-1.5 text-xs font-medium text-gray-700 dark:text-gray-300 bg-gray-100 dark:bg-[#1A1A1A] border border-gray-300 dark:border-gray-600 rounded hover:bg-gray-200 dark:hover:bg-gray-700 transition-colors"
          >
            {showTestCases ? "Hide" : "Load Test Case"} ({testCases.length})
          </button>
        )}
      </div>

      {saveMessage && (
        <p className="text-xs text-green-600 dark:text-green-400 text-center">{saveMessage}</p>
      )}

      {showTestCases && <TestCaseList onLoad={handleLoadTestCase} />}

      {error && (
        <div className="rounded border border-red-200 dark:border-red-800 bg-red-50 dark:bg-red-900/20 p-3">
          <p className="text-sm text-red-700 dark:text-red-300">{error}</p>
        </div>
      )}

      {result ? (
        <>
          <ResultView result={result} />
          <EvaluationTrace evaluations={result.evaluations} />
        </>
      ) : (
        !error && (
          <p className="text-xs text-gray-400 dark:text-gray-500 text-center py-4">
            Run a simulation to see results
          </p>
        )
      )}
    </div>
  );
}
