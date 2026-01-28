import { useConfigStore } from "@/stores/configStore";

export function FileTabBar() {
  const { openFiles, activeFile, setActiveFile, closeFile } = useConfigStore();

  const files = Array.from(openFiles.entries());

  if (files.length === 0) {
    return null;
  }

  return (
    <div className="flex items-center border-b border-gray-200 dark:border-gray-700 bg-surface dark:bg-surface-dark overflow-x-auto">
      {files.map(([path, state]) => (
        <FileTab
          key={path}
          path={path}
          modified={state.modified}
          isActive={path === activeFile}
          onClick={() => setActiveFile(path)}
          onClose={() => closeFile(path)}
        />
      ))}
    </div>
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

  const handleClose = (e: React.MouseEvent) => {
    e.stopPropagation();
    // TODO: Prompt for save if modified
    onClose();
  };

  return (
    <div
      onClick={onClick}
      className={`group flex items-center gap-2 px-3 py-2 text-sm cursor-pointer border-r border-gray-200 dark:border-gray-700 transition-colors ${
        isActive
          ? "bg-white dark:bg-[#1A1A1A] text-gray-900 dark:text-gray-100"
          : "bg-surface dark:bg-surface-dark text-gray-600 dark:text-gray-400 hover:bg-gray-100 dark:hover:bg-gray-800"
      }`}
    >
      {/* File icon */}
      <svg className="w-4 h-4 flex-shrink-0" fill="none" stroke="currentColor" viewBox="0 0 24 24">
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

      {/* Close button */}
      <button
        onClick={handleClose}
        className="p-0.5 rounded hover:bg-gray-200 dark:hover:bg-gray-700 opacity-0 group-hover:opacity-100 transition-opacity"
        aria-label={`Close ${fileName}`}
      >
        <svg className="w-3.5 h-3.5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
          <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M6 18L18 6M6 6l12 12" />
        </svg>
      </button>
    </div>
  );
}
