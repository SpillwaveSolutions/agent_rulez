import { writeConfig } from "@/lib/tauri";
import { useConfigStore } from "@/stores/configStore";
import { useEditorStore } from "@/stores/editorStore";
import { useUIStore } from "@/stores/uiStore";
import { loader } from "@monaco-editor/react";
import { useCallback, useEffect, useRef } from "react";
import { EditorToolbar } from "../editor/EditorToolbar";
import { ValidationPanel } from "../editor/ValidationPanel";
import { YamlEditor } from "../editor/YamlEditor";
import { FileTabBar } from "../files/FileTabBar";
import { LogViewer } from "../logs/LogViewer";

export function MainContent() {
  const { activeFile, openFiles, updateContent, markSaved, getActiveContent } = useConfigStore();
  const mainView = useUIStore((s) => s.mainView);
  const activeContent = getActiveContent();

  // Dispose Monaco models when files are closed
  const prevOpenFilesRef = useRef<Set<string>>(new Set());

  useEffect(() => {
    const currentPaths = new Set(openFiles.keys());
    const closedPaths: string[] = [];

    for (const prevPath of prevOpenFilesRef.current) {
      if (!currentPaths.has(prevPath)) {
        closedPaths.push(prevPath);
      }
    }

    if (closedPaths.length > 0) {
      loader.init().then((monaco) => {
        for (const closedPath of closedPaths) {
          const uri = monaco.Uri.parse(closedPath);
          const model = monaco.editor.getModel(uri);
          model?.dispose();
        }
      });
    }

    prevOpenFilesRef.current = currentPaths;
  }, [openFiles]);

  const handleSave = useCallback(async () => {
    if (!activeFile) return;

    // Format before saving (if formatting provider is registered)
    const editorRefValue = useEditorStore.getState().editorRef;
    if (editorRefValue) {
      const formatAction = editorRefValue.getAction("editor.action.formatDocument");
      if (formatAction) {
        await formatAction.run();
      }
    }

    // Re-read content after formatting (formatting updates the model -> onChange -> configStore)
    const content = useConfigStore.getState().getActiveContent();
    if (content === null) return;
    try {
      await writeConfig(activeFile, content);
      markSaved(activeFile);
    } catch (err) {
      console.error("Failed to save file:", err);
    }
  }, [activeFile, markSaved]);

  if (mainView === "logs") {
    return (
      <main className="flex-1 flex flex-col min-w-0 overflow-hidden">
        <LogViewer />
      </main>
    );
  }

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
                path={activeFile}
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
