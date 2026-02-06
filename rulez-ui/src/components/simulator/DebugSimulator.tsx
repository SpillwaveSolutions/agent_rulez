import { EvaluationTrace } from "@/components/simulator/EvaluationTrace";
import { EventForm } from "@/components/simulator/EventForm";
import { ResultView } from "@/components/simulator/ResultView";
import { runDebug } from "@/lib/tauri";
import type { DebugParams, DebugResult } from "@/types";
import { useState } from "react";

export function DebugSimulator() {
  const [result, setResult] = useState<DebugResult | null>(null);
  const [isLoading, setIsLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);

  async function handleSubmit(params: DebugParams) {
    setIsLoading(true);
    setError(null);
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

  return (
    <div className="space-y-4">
      <div>
        <h3 className="text-sm font-semibold text-gray-700 dark:text-gray-300">Debug Simulator</h3>
        <p className="text-xs text-gray-500 dark:text-gray-400 mt-1">
          Test your CCH rules by simulating events.
        </p>
      </div>

      <EventForm onSubmit={handleSubmit} isLoading={isLoading} />

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
