import { isTauri } from "@/lib/tauri";
import { useConfigStore } from "@/stores/configStore";
import { useEditorStore } from "@/stores/editorStore";

export function StatusBar() {
  const { activeFile, openFiles, globalConfig, projectConfig, setActiveFile, getScopeInfo } =
    useConfigStore();
  const { cursorPosition, errors, warnings } = useEditorStore();
  const scopeInfo = getScopeInfo();

  const activeFileState = activeFile ? openFiles.get(activeFile) : null;

  return (
    <footer className="status-bar flex items-center justify-between px-4 text-xs border-t border-gray-200 dark:border-gray-700 bg-surface dark:bg-surface-dark text-gray-600 dark:text-gray-400 no-select">
      {/* Left section */}
      <div className="flex items-center gap-4">
        {/* Config scope indicator */}
        <button
          type="button"
          onClick={() => {
            const targetPath =
              scopeInfo.activeScope === "project" ? projectConfig?.path : globalConfig?.path;
            if (targetPath) setActiveFile(targetPath);
          }}
          className="flex items-center gap-1 hover:text-gray-900 dark:hover:text-gray-200 transition-colors"
          title={
            scopeInfo.activeScope === "project"
              ? "Project config is active (click to open)"
              : scopeInfo.activeScope === "global"
                ? "Global config is active (click to open)"
                : "No config found"
          }
        >
          <span
            className={`w-2 h-2 rounded-full ${
              scopeInfo.activeScope === "project"
                ? "bg-green-500"
                : scopeInfo.activeScope === "global"
                  ? "bg-green-500"
                  : "bg-gray-400"
            }`}
          />
          Config:{" "}
          {scopeInfo.activeScope === "none"
            ? "None"
            : scopeInfo.activeScope.charAt(0).toUpperCase() + scopeInfo.activeScope.slice(1)}
        </button>

        {/* Cursor position */}
        <span>
          Ln {cursorPosition.line}, Col {cursorPosition.column}
        </span>

        {/* File type */}
        <span>YAML</span>

        {/* Encoding */}
        <span>UTF-8</span>
      </div>

      {/* Center section */}
      <div className="flex items-center gap-4">
        {/* Modified indicator */}
        {activeFileState?.modified && (
          <span className="text-warning dark:text-warning-dark">Modified</span>
        )}
      </div>

      {/* Right section */}
      <div className="flex items-center gap-4">
        {/* Error/warning counts */}
        <div className="flex items-center gap-2">
          {errors.length > 0 && (
            <span className="flex items-center gap-1 text-error dark:text-error-dark">
              <svg
                className="w-3.5 h-3.5"
                fill="currentColor"
                viewBox="0 0 20 20"
                aria-hidden="true"
              >
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
              <svg
                className="w-3.5 h-3.5"
                fill="currentColor"
                viewBox="0 0 20 20"
                aria-hidden="true"
              >
                <path
                  fillRule="evenodd"
                  d="M8.257 3.099c.765-1.36 2.722-1.36 3.486 0l5.58 9.92c.75 1.334-.213 2.98-1.742 2.98H4.42c-1.53 0-2.493-1.646-1.743-2.98l5.58-9.92zM11 13a1 1 0 11-2 0 1 1 0 012 0zm-1-8a1 1 0 00-1 1v3a1 1 0 002 0V6a1 1 0 00-1-1z"
                  clipRule="evenodd"
                />
              </svg>
              {warnings.length}
            </span>
          )}
          {errors.length === 0 && warnings.length === 0 && (
            <span className="flex items-center gap-1 text-success dark:text-success-dark">
              <svg
                className="w-3.5 h-3.5"
                fill="currentColor"
                viewBox="0 0 20 20"
                aria-hidden="true"
              >
                <path
                  fillRule="evenodd"
                  d="M10 18a8 8 0 100-16 8 8 0 000 16zm3.707-9.293a1 1 0 00-1.414-1.414L9 10.586 7.707 9.293a1 1 0 00-1.414 1.414l2 2a1 1 0 001.414 0l4-4z"
                  clipRule="evenodd"
                />
              </svg>
              No issues
            </span>
          )}
        </div>

        {/* Connection status */}
        <span
          className={`flex items-center gap-1 ${
            isTauri() ? "text-success dark:text-success-dark" : "text-gray-400"
          }`}
        >
          <span className="w-2 h-2 rounded-full bg-current" />
          {isTauri() ? "Connected" : "Web Mode"}
        </span>
      </div>
    </footer>
  );
}
