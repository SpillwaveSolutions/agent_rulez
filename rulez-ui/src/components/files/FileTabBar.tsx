import { ConfirmDialog } from "@/components/ui/ConfirmDialog";
import { writeConfig } from "@/lib/tauri";
import { useConfigStore } from "@/stores/configStore";
import { useState } from "react";

export function FileTabBar() {
  const { openFiles, activeFile, setActiveFile, closeFile, markSaved } = useConfigStore();
  const [pendingClosePath, setPendingClosePath] = useState<string | null>(null);

  const files = Array.from(openFiles.entries());

  if (files.length === 0) {
    return null;
  }

  const handleRequestClose = (path: string) => {
    const fileState = openFiles.get(path);
    if (fileState?.modified) {
      setPendingClosePath(path);
    } else {
      closeFile(path);
    }
  };

  const handleSave = async () => {
    if (!pendingClosePath) return;
    const fileState = openFiles.get(pendingClosePath);
    if (fileState) {
      await writeConfig(pendingClosePath, fileState.content);
      markSaved(pendingClosePath);
    }
    closeFile(pendingClosePath);
    setPendingClosePath(null);
  };

  const handleDiscard = () => {
    if (!pendingClosePath) return;
    closeFile(pendingClosePath);
    setPendingClosePath(null);
  };

  const handleCancel = () => {
    setPendingClosePath(null);
  };

  const pendingFileName = pendingClosePath?.split("/").pop() ?? "";

  return (
    <>
      <div
        data-testid="file-tab-bar"
        className="flex items-center border-b border-gray-200 dark:border-gray-700 bg-surface dark:bg-surface-dark overflow-x-auto"
      >
        {files.map(([path, state]) => (
          <FileTab
            key={path}
            path={path}
            modified={state.modified}
            isActive={path === activeFile}
            onClick={() => setActiveFile(path)}
            onClose={() => handleRequestClose(path)}
          />
        ))}
      </div>

      <ConfirmDialog
        isOpen={pendingClosePath !== null}
        title="Unsaved Changes"
        message={`"${pendingFileName}" has unsaved changes. Do you want to save before closing?`}
        onSave={handleSave}
        onDiscard={handleDiscard}
        onCancel={handleCancel}
      />
    </>
  );
}

interface FileTabProps {
  path: string;
  modified: boolean;
  isActive: boolean;
  onClick: () => void;
  onClose: () => void;
}

function FileTab({ path, modified, isActive, onClick, onClose }: FileTabProps) {
  const fileName = path.split("/").pop() || path;

  return (
    <div
      data-testid={`file-tab-${fileName}`}
      className={`group relative flex items-center border-r border-gray-200 dark:border-gray-700 transition-colors ${
        isActive
          ? "bg-white dark:bg-[#1A1A1A] text-gray-900 dark:text-gray-100"
          : "bg-surface dark:bg-surface-dark text-gray-600 dark:text-gray-400 hover:bg-gray-100 dark:hover:bg-gray-800"
      }`}
    >
      {/* Tab selection button */}
      <button
        type="button"
        onClick={onClick}
        className="flex items-center gap-2 px-3 py-2 text-sm cursor-pointer"
      >
        {/* File icon */}
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

        {/* File name */}
        <span className="whitespace-nowrap">{fileName}</span>

        {/* Modified indicator */}
        {modified && <span className="w-2 h-2 rounded-full bg-accent dark:bg-accent-dark" />}
      </button>

      {/* Close button */}
      <button
        type="button"
        data-testid={`close-tab-${fileName}`}
        onClick={onClose}
        className="p-0.5 mr-2 rounded hover:bg-gray-200 dark:hover:bg-gray-700 opacity-0 group-hover:opacity-100 transition-opacity"
        aria-label={`Close ${fileName}`}
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
            d="M6 18L18 6M6 6l12 12"
          />
        </svg>
      </button>
    </div>
  );
}
