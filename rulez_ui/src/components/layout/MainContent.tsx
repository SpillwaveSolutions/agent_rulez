import { useConfigStore } from "@/stores/configStore";
import { FileTabBar } from "../files/FileTabBar";

export function MainContent() {
  const { activeFile, openFiles, updateContent, getActiveContent } = useConfigStore();
  const activeContent = getActiveContent();

  return (
    <main className="flex-1 flex flex-col min-w-0 overflow-hidden">
      {/* Tab bar */}
      <FileTabBar />

      {/* Editor area */}
      <div className="flex-1 overflow-hidden">
        {activeFile && activeContent !== null ? (
          <div className="h-full p-4 bg-white dark:bg-[#1A1A1A]">
            {/* Placeholder for Monaco Editor - will be implemented in M2 */}
            <div className="h-full rounded border border-gray-200 dark:border-gray-700 bg-surface dark:bg-surface-dark overflow-hidden">
              <textarea
                value={activeContent}
                onChange={(e) => updateContent(activeFile, e.target.value)}
                className="w-full h-full p-4 font-mono text-sm bg-transparent resize-none focus:outline-none text-gray-900 dark:text-gray-100"
                placeholder="YAML content will appear here..."
                spellCheck={false}
              />
            </div>
          </div>
        ) : (
          <div className="h-full flex items-center justify-center text-gray-400 dark:text-gray-500">
            <div className="text-center">
              <svg
                className="w-12 h-12 mx-auto mb-3 opacity-50"
                fill="none"
                stroke="currentColor"
                viewBox="0 0 24 24"
              >
                <path
                  strokeLinecap="round"
                  strokeLinejoin="round"
                  strokeWidth={1.5}
                  d="M9 12h6m-6 4h6m2 5H7a2 2 0 01-2-2V5a2 2 0 012-2h5.586a1 1 0 01.707.293l5.414 5.414a1 1 0 01.293.707V19a2 2 0 01-2 2z"
                />
              </svg>
              <p>Select a file from the sidebar to edit</p>
            </div>
          </div>
        )}
      </div>
    </main>
  );
}
