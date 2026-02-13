import { checkBinary, generateSampleConfig, runDebug } from "@/lib/tauri";
import { useSettingsStore } from "@/stores/settingsStore";
import type { DebugResult } from "@/types";
import { useCallback, useEffect, useState } from "react";

type Step = "welcome" | "binary" | "config" | "simulation" | "complete";

const STEPS: Step[] = ["welcome", "binary", "config", "simulation", "complete"];

export function OnboardingWizard() {
  const [step, setStep] = useState<Step>("welcome");
  const [binaryFound, setBinaryFound] = useState<boolean | null>(null);
  const [binaryPath, setBinaryPath] = useState<string | null>(null);
  const [configGenerated, setConfigGenerated] = useState(false);
  const [simResult, setSimResult] = useState<DebugResult | null>(null);
  const [simRunning, setSimRunning] = useState(false);
  const [error, setError] = useState<string | null>(null);

  const setOnboardingComplete = useSettingsStore((s) => s.setOnboardingComplete);

  // Auto-check binary when entering binary step
  useEffect(() => {
    if (step === "binary" && binaryFound === null) {
      void checkBinary().then((result) => {
        setBinaryFound(result.found);
        setBinaryPath(result.path);
      });
    }
  }, [step, binaryFound]);

  const nextStep = useCallback(() => {
    const idx = STEPS.indexOf(step);
    if (idx < STEPS.length - 1) {
      setStep(STEPS[idx + 1] as Step);
    }
  }, [step]);

  const handleSkip = useCallback(() => {
    void setOnboardingComplete(true);
  }, [setOnboardingComplete]);

  const handleComplete = useCallback(() => {
    void setOnboardingComplete(true);
  }, [setOnboardingComplete]);

  const handleGenerateConfig = useCallback(async () => {
    setError(null);
    try {
      await generateSampleConfig();
      setConfigGenerated(true);
    } catch (err) {
      setError(err instanceof Error ? err.message : "Failed to generate config");
    }
  }, []);

  const handleRunSimulation = useCallback(async () => {
    setSimRunning(true);
    setError(null);
    try {
      const result = await runDebug({
        eventType: "PreToolUse",
        tool: "Bash",
        command: "git push --force origin main",
      });
      setSimResult(result);
    } catch (err) {
      setError(err instanceof Error ? err.message : "Simulation failed");
    } finally {
      setSimRunning(false);
    }
  }, []);

  const stepIndex = STEPS.indexOf(step);
  const progress = ((stepIndex + 1) / STEPS.length) * 100;

  return (
    <div className="fixed inset-0 z-50 flex items-center justify-center bg-black/50">
      <dialog
        open
        aria-label="Welcome to RuleZ"
        className="relative w-full max-w-lg mx-4 bg-white dark:bg-[#1E1E1E] rounded-lg shadow-xl overflow-hidden p-0"
      >
        {/* Progress bar */}
        <div className="h-1 bg-gray-200 dark:bg-gray-700">
          <div
            className="h-full bg-accent transition-all duration-300"
            style={{ width: `${progress}%` }}
          />
        </div>

        {/* Content */}
        <div className="p-6 space-y-4">
          {step === "welcome" && (
            <>
              <h2 className="text-lg font-semibold text-gray-900 dark:text-gray-100">
                Welcome to RuleZ
              </h2>
              <p className="text-sm text-gray-600 dark:text-gray-400">
                RuleZ enforces policy rules on AI coding agents. This wizard will help you get set
                up in a few quick steps.
              </p>
              <div className="text-sm text-gray-500 dark:text-gray-400 space-y-1">
                <p>We will:</p>
                <ul className="list-disc list-inside space-y-1 ml-2">
                  <li>Check that the RuleZ binary is installed</li>
                  <li>Generate a sample configuration with example rules</li>
                  <li>Run a test simulation to verify everything works</li>
                </ul>
              </div>
            </>
          )}

          {step === "binary" && (
            <>
              <h2 className="text-lg font-semibold text-gray-900 dark:text-gray-100">
                Binary Detection
              </h2>
              {binaryFound === null && (
                <p className="text-sm text-gray-500 dark:text-gray-400">
                  Checking for RuleZ binary...
                </p>
              )}
              {binaryFound === true && (
                <div className="rounded border border-green-200 dark:border-green-800 bg-green-50 dark:bg-green-900/20 p-3">
                  <p className="text-sm text-green-700 dark:text-green-300">
                    Binary found at: <code className="font-mono text-xs">{binaryPath}</code>
                  </p>
                </div>
              )}
              {binaryFound === false && (
                <div className="rounded border border-amber-200 dark:border-amber-800 bg-amber-50 dark:bg-amber-900/20 p-3 space-y-2">
                  <p className="text-sm text-amber-700 dark:text-amber-300">
                    Binary not found. You can still continue and configure the path later in
                    Settings.
                  </p>
                  <p className="text-xs text-amber-600 dark:text-amber-400">
                    Download from:{" "}
                    <a
                      href="https://github.com/SpillwaveSolutions/code_agent_context_hooks/releases"
                      target="_blank"
                      rel="noopener noreferrer"
                      className="underline"
                    >
                      GitHub Releases
                    </a>
                  </p>
                </div>
              )}
            </>
          )}

          {step === "config" && (
            <>
              <h2 className="text-lg font-semibold text-gray-900 dark:text-gray-100">
                Sample Configuration
              </h2>
              <p className="text-sm text-gray-600 dark:text-gray-400">
                Generate a sample <code className="font-mono text-xs">hooks.yaml</code> with example
                rules to get started.
              </p>
              {configGenerated ? (
                <div className="rounded border border-green-200 dark:border-green-800 bg-green-50 dark:bg-green-900/20 p-3">
                  <p className="text-sm text-green-700 dark:text-green-300">
                    Sample config generated with example rules at{" "}
                    <code className="font-mono text-xs">~/.claude/hooks.yaml</code>
                  </p>
                </div>
              ) : (
                <button
                  type="button"
                  onClick={() => void handleGenerateConfig()}
                  className="px-4 py-2 text-sm font-medium text-white bg-accent hover:bg-accent/90 rounded transition-colors"
                >
                  Generate Sample Config
                </button>
              )}
            </>
          )}

          {step === "simulation" && (
            <>
              <h2 className="text-lg font-semibold text-gray-900 dark:text-gray-100">
                Test Simulation
              </h2>
              <p className="text-sm text-gray-600 dark:text-gray-400">
                Run a test simulation to verify your setup works. This simulates a{" "}
                <code className="font-mono text-xs">git push --force</code> command.
              </p>
              {simResult ? (
                <div className="rounded border border-green-200 dark:border-green-800 bg-green-50 dark:bg-green-900/20 p-3 space-y-1">
                  <p className="text-sm text-green-700 dark:text-green-300">
                    Simulation complete! Outcome:{" "}
                    <span className="font-semibold">{simResult.outcome}</span>
                  </p>
                  {simResult.reason && (
                    <p className="text-xs text-green-600 dark:text-green-400">
                      Reason: {simResult.reason}
                    </p>
                  )}
                  <p className="text-xs text-green-600 dark:text-green-400">
                    {simResult.evaluations.length} rules evaluated in{" "}
                    {simResult.evaluationTimeMs.toFixed(1)}ms
                  </p>
                </div>
              ) : (
                <button
                  type="button"
                  onClick={() => void handleRunSimulation()}
                  disabled={simRunning}
                  className="px-4 py-2 text-sm font-medium text-white bg-accent hover:bg-accent/90 rounded transition-colors disabled:opacity-50"
                >
                  {simRunning ? "Running..." : "Run Test Simulation"}
                </button>
              )}
            </>
          )}

          {step === "complete" && (
            <>
              <h2 className="text-lg font-semibold text-gray-900 dark:text-gray-100">All Set!</h2>
              <p className="text-sm text-gray-600 dark:text-gray-400">
                You are ready to use RuleZ. Edit your rules in the editor, view logs, and run
                simulations from the debug panel.
              </p>
              <p className="text-xs text-gray-500 dark:text-gray-400">
                You can re-run this wizard anytime from the Settings panel.
              </p>
            </>
          )}

          {error && (
            <div className="rounded border border-red-200 dark:border-red-800 bg-red-50 dark:bg-red-900/20 p-3">
              <p className="text-sm text-red-700 dark:text-red-300">{error}</p>
            </div>
          )}
        </div>

        {/* Footer with navigation */}
        <div className="flex items-center justify-between px-6 py-4 border-t border-gray-200 dark:border-gray-700">
          <button
            type="button"
            onClick={handleSkip}
            className="px-3 py-1.5 text-xs text-gray-500 dark:text-gray-400 hover:text-gray-700 dark:hover:text-gray-200 transition-colors"
          >
            Skip
          </button>

          {step === "complete" ? (
            <button
              type="button"
              onClick={handleComplete}
              className="px-4 py-2 text-sm font-medium text-white bg-accent hover:bg-accent/90 rounded transition-colors"
            >
              Done
            </button>
          ) : (
            <button
              type="button"
              onClick={nextStep}
              className="px-4 py-2 text-sm font-medium text-white bg-accent hover:bg-accent/90 rounded transition-colors"
            >
              Next
            </button>
          )}
        </div>
      </dialog>
    </div>
  );
}
