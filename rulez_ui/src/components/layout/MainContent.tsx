import { writeConfig } from "@/lib/tauri";
import { useConfigStore } from "@/stores/configStore";
import { useCallback } from "react";
import { EditorToolbar } from "../editor/EditorToolbar";
import { ValidationPanel } from "../editor/ValidationPanel";
import { YamlEditor } from "../editor/YamlEditor";
import { FileTabBar } from "../files/FileTabBar";

export function MainContent() {
  const { activeFile, updateContent, markSaved, getActiveContent } = useConfigStore();
  const activeContent = getActiveContent();

  const handleSave = useCallback(async () => {
    if (!activeFile) return;
    const content = useConfigStore.getState().getActiveContent();
    if (content === null) return;
    try {
      await writeConfig(activeFile, content);
      markSaved(activeFile);
    } catch (err) {
      console.error("Failed to save file:", err);
    }
  }, [activeFile, markSaved]);

  return (
    <main className="flex-1 flex flex-col min-w-0 overflow-hidden">
      {/* Tab bar */}
      <FileTabBar />

      {/* Editor area */}
      <div className="flex-1 overflow-hidden">
        {activeFile && activeContent !== null ? (
          <div className="h-full flex flex-col bg-white dark:bg-[#1A1A1A]">
            <EditorToolbar />
            <div className="flex-1 overflow-hidden">
              <YamlEditor
                value={activeContent}
                onChange={(val) => updateContent(activeFile, val)}
                onSave={handleSave}
              />
            </div>
            <ValidationPanel />
          </div>
        ) : (
          <div className="h-full flex items-center justify-center text-gray-400 dark:text-gray-500">
            <div className="text-center">
              <svg
                className="w-12 h-12 mx-auto mb-3 opacity-50"
                fill="none"
                stroke="currentColor"
                viewBox="0 0 24 24"
                role="img"
                aria-label="No file selected"
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
