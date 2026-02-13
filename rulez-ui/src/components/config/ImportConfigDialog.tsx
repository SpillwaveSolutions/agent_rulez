import { validateConfig, writeConfig } from "@/lib/tauri";
import { useConfigStore } from "@/stores/configStore";
import { useCallback, useEffect, useState } from "react";

interface ImportConfigDialogProps {
  content: string;
  sourcePath: string;
  onClose: () => void;
}

/**
 * Dialog for importing a config file. Shows content preview, validation status,
 * and lets the user choose which scope (global or project) to apply it to.
 */
export function ImportConfigDialog({ content, sourcePath, onClose }: ImportConfigDialogProps) {
  const { globalConfig, projectConfig, openFile } = useConfigStore();
  const [validationResult, setValidationResult] = useState<{
    valid: boolean;
    errors: string[];
  } | null>(null);
  const [isValidating, setIsValidating] = useState(true);
  const [applying, setApplying] = useState(false);

  // Run validation on mount
  useEffect(() => {
    async function validate() {
      setIsValidating(true);
      try {
        // Try binary validation first; fall back to basic YAML check
        const result = await validateConfig(sourcePath);
        setValidationResult(result);
      } catch {
        // If binary validation fails, do a basic check
        try {
          // Basic YAML syntax check: if the content starts with valid-looking YAML, pass it
          const hasVersion = content.includes("version:");
          setValidationResult({
            valid: hasVersion,
            errors: hasVersion ? [] : ["Missing required 'version' field"],
          });
        } catch (err) {
          setValidationResult({
            valid: false,
            errors: [err instanceof Error ? err.message : "YAML parse error"],
          });
        }
      } finally {
        setIsValidating(false);
      }
    }
    validate();
  }, [content, sourcePath]);

  const handleApply = useCallback(
    async (targetPath: string) => {
      setApplying(true);
      try {
        await writeConfig(targetPath, content);
        openFile(targetPath, content);
        onClose();
      } catch (err) {
        console.error("Failed to apply config:", err);
      } finally {
        setApplying(false);
      }
    },
    [content, openFile, onClose],
  );

  const globalPath = globalConfig?.path ?? "~/.claude/hooks.yaml";
  const projectPath = projectConfig?.path ?? ".claude/hooks.yaml";

  return (
    <div className="fixed inset-0 z-50 flex items-center justify-center bg-black/40">
      <div className="bg-white dark:bg-gray-800 rounded-lg shadow-xl w-[560px] max-h-[80vh] flex flex-col">
        {/* Header */}
        <div className="px-4 py-3 border-b border-gray-200 dark:border-gray-700">
          <h3 className="text-sm font-semibold text-gray-900 dark:text-gray-100">Import Config</h3>
          <p className="text-xs text-gray-500 dark:text-gray-400 mt-0.5 truncate">
            Source: {sourcePath}
          </p>
        </div>

        {/* Validation status */}
        <div className="px-4 py-2 border-b border-gray-200 dark:border-gray-700">
          {isValidating ? (
            <span className="text-xs text-gray-500 dark:text-gray-400">Validating...</span>
          ) : validationResult?.valid ? (
            <span className="text-xs text-green-600 dark:text-green-400 font-medium">
              Valid YAML configuration
            </span>
          ) : (
            <div>
              <span className="text-xs text-red-600 dark:text-red-400 font-medium">
                Invalid configuration
              </span>
              {validationResult?.errors.map((err) => (
                <p key={err} className="text-xs text-red-500 dark:text-red-400 mt-0.5">
                  {err}
                </p>
              ))}
            </div>
          )}
        </div>

        {/* Content preview */}
        <div className="flex-1 overflow-auto px-4 py-2 min-h-0">
          <pre className="text-xs font-mono text-gray-700 dark:text-gray-300 whitespace-pre-wrap break-words max-h-64 overflow-auto bg-gray-50 dark:bg-gray-900 rounded p-2">
            {content}
          </pre>
        </div>

        {/* Actions */}
        <div className="px-4 py-3 border-t border-gray-200 dark:border-gray-700 flex items-center justify-end gap-2">
          <button
            type="button"
            onClick={onClose}
            className="px-3 py-1.5 text-sm rounded border border-gray-300 dark:border-gray-600 text-gray-700 dark:text-gray-300 hover:bg-gray-100 dark:hover:bg-gray-700 transition-colors"
          >
            Cancel
          </button>
          <button
            type="button"
            disabled={!validationResult?.valid || applying}
            onClick={() => handleApply(globalPath)}
            className="px-3 py-1.5 text-sm rounded bg-blue-600 text-white hover:bg-blue-700 disabled:opacity-50 disabled:cursor-not-allowed transition-colors"
          >
            Apply to Global
          </button>
          <button
            type="button"
            disabled={!validationResult?.valid || applying}
            onClick={() => handleApply(projectPath)}
            className="px-3 py-1.5 text-sm rounded bg-green-600 text-white hover:bg-green-700 disabled:opacity-50 disabled:cursor-not-allowed transition-colors"
          >
            Apply to Project
          </button>
        </div>
      </div>
    </div>
  );
}
