import { listConfigFiles, readConfig } from "@/lib/tauri";
import { useConfigStore } from "@/stores/configStore";
import { useEffect } from "react";

export function Sidebar() {
  const { globalConfig, projectConfig, setGlobalConfig, setProjectConfig, openFile, activeFile } =
    useConfigStore();

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
    <aside
      data-testid="sidebar"
      className="w-56 flex-shrink-0 border-r border-gray-200 dark:border-gray-700 bg-surface dark:bg-surface-dark overflow-y-auto"
    >
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
          </div>
          {globalConfig ? (
            <FileItem
              path={globalConfig.path}
              exists={globalConfig.exists}
              isActive={activeFile === globalConfig.path}
              onClick={() => handleFileClick(globalConfig.path)}
              section="global"
            />
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
          </div>
          {projectConfig ? (
            <FileItem
              path={projectConfig.path}
              exists={projectConfig.exists}
              isActive={activeFile === projectConfig.path}
              onClick={() => handleFileClick(projectConfig.path)}
              section="project"
            />
          ) : (
            <div className="text-sm text-gray-400 dark:text-gray-500 italic pl-5">
              No project config
            </div>
          )}
        </div>
      </div>
    </aside>
  );
}

interface FileItemProps {
  path: string;
  exists: boolean;
  isActive: boolean;
  onClick: () => void;
  section: "global" | "project";
}

function FileItem({ path, exists, isActive, onClick, section }: FileItemProps) {
  const fileName = path.split("/").pop() || path;

  return (
    <button
      type="button"
      data-testid={`sidebar-${section}-file-${fileName}`}
      onClick={onClick}
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
