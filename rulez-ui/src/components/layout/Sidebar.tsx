import { ImportConfigDialog } from "@/components/config/ImportConfigDialog";
import { exportConfigFile, importConfigFile, listConfigFiles, readConfig } from "@/lib/tauri";
import { useConfigStore } from "@/stores/configStore";
import type { ScopeInfo } from "@/stores/configStore";
import { useCallback, useEffect, useState } from "react";

export function Sidebar() {
  const {
    globalConfig,
    projectConfig,
    setGlobalConfig,
    setProjectConfig,
    openFile,
    activeFile,
    getScopeInfo,
  } = useConfigStore();
  const scopeInfo: ScopeInfo = getScopeInfo();
  const [importData, setImportData] = useState<{ content: string; sourcePath: string } | null>(
    null,
  );
  const [exportFeedback, setExportFeedback] = useState<string | null>(null);

  const handleImport = useCallback(async () => {
    try {
      const result = await importConfigFile();
      if (result) {
        setImportData({ content: result.content, sourcePath: result.path });
      }
    } catch (err) {
      console.error("Import failed:", err);
    }
  }, []);

  const handleExport = useCallback(async (path: string) => {
    try {
      const fileState = useConfigStore.getState().openFiles.get(path);
      const content = fileState?.content ?? (await readConfig(path));
      const fileName = path.split("/").pop() ?? "hooks.yaml";
      const exported = await exportConfigFile(content, fileName);
      if (exported) {
        setExportFeedback(path);
        setTimeout(() => setExportFeedback(null), 2000);
      }
    } catch (err) {
      console.error("Export failed:", err);
    }
  }, []);

  // Load config files on mount
  useEffect(() => {
    async function loadConfigs() {
      try {
        const files = await listConfigFiles();
        const global = files.find((f) => f.path.includes("~/.claude"));
        const project = files.find((f) => !f.path.includes("~/.claude"));

        if (global) setGlobalConfig(global);
        if (project) setProjectConfig(project);

        // Auto-open global config if nothing is open
        if (global?.exists && !activeFile) {
          const content = await readConfig(global.path);
          openFile(global.path, content);
        }
      } catch (error) {
        console.error("Failed to load config files:", error);
      }
    }

    loadConfigs();
  }, [setGlobalConfig, setProjectConfig, openFile, activeFile]);

  const handleFileClick = async (path: string) => {
    try {
      const content = await readConfig(path);
      openFile(path, content);
    } catch (error) {
      console.error("Failed to open file:", error);
    }
  };

  return (
    <aside data-testid="sidebar" className="w-56 flex-shrink-0 border-r border-gray-200 dark:border-gray-700 bg-surface dark:bg-surface-dark overflow-y-auto">
      <div className="p-3">
        <h2 className="text-xs font-semibold uppercase tracking-wider text-gray-500 dark:text-gray-400 mb-2">
          Files
        </h2>

        {/* Global config section */}
        <div className="mb-4">
          <div className="flex items-center gap-1 text-xs text-gray-500 dark:text-gray-400 mb-1">
            <svg
              className="w-4 h-4"
              fill="none"
              stroke="currentColor"
              viewBox="0 0 24 24"
              aria-hidden="true"
            >
              <path
                strokeLinecap="round"
                strokeLinejoin="round"
                strokeWidth={2}
                d="M3 12l2-2m0 0l7-7 7 7M5 10v10a1 1 0 001 1h3m10-11l2 2m-2-2v10a1 1 0 01-1 1h-3m-6 0a1 1 0 001-1v-4a1 1 0 011-1h2a1 1 0 011 1v4a1 1 0 001 1m-6 0h6"
              />
            </svg>
            <span>Global</span>
            <ScopeBadge status={scopeInfo.globalStatus} type="global" />
          </div>
          {globalConfig ? (
            <div className="flex items-center gap-1">
              <FileItem
                path={globalConfig.path}
                exists={globalConfig.exists}
                isActive={activeFile === globalConfig.path}
                onClick={() => handleFileClick(globalConfig.path)}
                scope="global"
              />
              {globalConfig.exists && (
                <ExportButton
                  onClick={() => handleExport(globalConfig.path)}
                  feedback={exportFeedback === globalConfig.path}
                />
              )}
            </div>
          ) : (
            <div className="text-sm text-gray-400 dark:text-gray-500 italic pl-5">Loading...</div>
          )}
        </div>

        {/* Project config section */}
        <div>
          <div className="flex items-center gap-1 text-xs text-gray-500 dark:text-gray-400 mb-1">
            <svg
              className="w-4 h-4"
              fill="none"
              stroke="currentColor"
              viewBox="0 0 24 24"
              aria-hidden="true"
            >
              <path
                strokeLinecap="round"
                strokeLinejoin="round"
                strokeWidth={2}
                d="M3 7v10a2 2 0 002 2h14a2 2 0 002-2V9a2 2 0 00-2-2h-6l-2-2H5a2 2 0 00-2 2z"
              />
            </svg>
            <span>Project</span>
            <ScopeBadge status={scopeInfo.projectStatus} type="project" />
          </div>
          {projectConfig ? (
            <div className="flex items-center gap-1">
              <FileItem
                path={projectConfig.path}
                exists={projectConfig.exists}
                isActive={activeFile === projectConfig.path}
                onClick={() => handleFileClick(projectConfig.path)}
                scope="project"
              />
              {projectConfig.exists && (
                <ExportButton
                  onClick={() => handleExport(projectConfig.path)}
                  feedback={exportFeedback === projectConfig.path}
                />
              )}
            </div>
          ) : (
            <div className="text-sm text-gray-400 dark:text-gray-500 italic pl-5">
              No project config
            </div>
          )}
        </div>

        {/* Import button */}
        <div className="mt-4 pt-3 border-t border-gray-200 dark:border-gray-700">
          <button
            type="button"
            onClick={handleImport}
            className="w-full flex items-center justify-center gap-1.5 px-2 py-1.5 text-xs rounded border border-dashed border-gray-300 dark:border-gray-600 text-gray-600 dark:text-gray-400 hover:bg-gray-100 dark:hover:bg-gray-700 hover:border-gray-400 dark:hover:border-gray-500 transition-colors"
            aria-label="Import config"
          >
            <svg
              className="w-3.5 h-3.5"
              fill="none"
              stroke="currentColor"
              viewBox="0 0 24 24"
              aria-hidden="true"
            >
              <path
                strokeLinecap="round"
                strokeLinejoin="round"
                strokeWidth={2}
                d="M4 16v1a3 3 0 003 3h10a3 3 0 003-3v-1m-4-8l-4-4m0 0L8 8m4-4v12"
              />
            </svg>
            Import Config
          </button>
        </div>
      </div>

      {/* Import dialog */}
      {importData && (
        <ImportConfigDialog
          content={importData.content}
          sourcePath={importData.sourcePath}
          onClose={() => setImportData(null)}
        />
      )}
    </aside>
  );
}

interface ExportButtonProps {
  onClick: () => void;
  feedback: boolean;
}

function ExportButton({ onClick, feedback }: ExportButtonProps) {
  return (
    <button
      type="button"
      onClick={(e) => {
        e.stopPropagation();
        onClick();
      }}
      className="p-1 rounded hover:bg-gray-200 dark:hover:bg-gray-700 text-gray-400 hover:text-gray-600 dark:hover:text-gray-300 transition-colors flex-shrink-0"
      aria-label="Export config"
      title={feedback ? "Exported!" : "Export config"}
    >
      {feedback ? (
        <svg
          className="w-3.5 h-3.5 text-green-500"
          fill="none"
          stroke="currentColor"
          viewBox="0 0 24 24"
          aria-hidden="true"
        >
          <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M5 13l4 4L19 7" />
        </svg>
      ) : (
        <svg
          className="w-3.5 h-3.5"
          fill="none"
          stroke="currentColor"
          viewBox="0 0 24 24"
          aria-hidden="true"
        >
          <path
            strokeLinecap="round"
            strokeLinejoin="round"
            strokeWidth={2}
            d="M4 16v1a3 3 0 003 3h10a3 3 0 003-3v-1m-4-4l-4 4m0 0l-4-4m4 4V4"
          />
        </svg>
      )}
    </button>
  );
}

interface ScopeBadgeProps {
  status: "active" | "overridden" | "missing";
  type: "global" | "project";
}

function ScopeBadge({ status, type }: ScopeBadgeProps) {
  if (status === "missing") return null;

  const isActive = status === "active";
  const label = isActive ? "Active" : "Overridden";
  const tooltip =
    isActive && type === "project"
      ? "This project config takes priority over the global config"
      : isActive && type === "global"
        ? "No project config found — using this global config"
        : "This global config is overridden by the project config";

  return (
    <span
      title={tooltip}
      className={`text-[10px] px-1.5 py-0.5 rounded-full font-medium leading-none ${
        isActive
          ? "bg-green-100 text-green-700 dark:bg-green-900 dark:text-green-300"
          : "bg-amber-100 text-amber-700 dark:bg-amber-900 dark:text-amber-300"
      }`}
    >
      {label}
    </span>
  );
}

interface FileItemProps {
  path: string;
  exists: boolean;
  isActive: boolean;
  onClick: () => void;
  scope: "global" | "project";
}

function FileItem({ path, exists, isActive, onClick, scope }: FileItemProps) {
  const fileName = path.split("/").pop() || path;

  return (
    <button
      type="button"
      onClick={onClick}
      data-testid={`sidebar-${scope}-file-${fileName}`}
      className={`w-full flex items-center gap-2 px-2 py-1.5 rounded text-sm text-left transition-colors ${
        isActive
          ? "bg-accent/10 text-accent dark:text-accent-dark"
          : "hover:bg-gray-200 dark:hover:bg-gray-700 text-gray-700 dark:text-gray-300"
      } ${!exists ? "opacity-50" : ""}`}
    >
      <svg
        className="w-4 h-4 flex-shrink-0"
        fill="none"
        stroke="currentColor"
        viewBox="0 0 24 24"
        aria-hidden="true"
      >
        <path
          strokeLinecap="round"
          strokeLinejoin="round"
          strokeWidth={2}
          d="M9 12h6m-6 4h6m2 5H7a2 2 0 01-2-2V5a2 2 0 012-2h5.586a1 1 0 01.707.293l5.414 5.414a1 1 0 01.293.707V19a2 2 0 01-2 2z"
        />
      </svg>
      <span className="truncate">{fileName}</span>
      {!exists && <span className="text-xs text-gray-400 dark:text-gray-500">(new)</span>}
    </button>
  );
}
