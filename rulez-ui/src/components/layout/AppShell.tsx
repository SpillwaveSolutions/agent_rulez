import { ExternalChangeDialog } from "@/components/config/ExternalChangeDialog";
import { createFileWatcher } from "@/lib/file-watcher";
import { readConfig } from "@/lib/tauri";
import { useConfigStore } from "@/stores/configStore";
import { useUIStore } from "@/stores/uiStore";
import { useCallback, useEffect, useRef, useState } from "react";
import { Header } from "./Header";
import { MainContent } from "./MainContent";
import { RightPanel } from "./RightPanel";
import { Sidebar } from "./Sidebar";
import { StatusBar } from "./StatusBar";

export function AppShell() {
  const { sidebarOpen } = useUIStore();
  const { globalConfig, projectConfig } = useConfigStore();
  const [conflictPath, setConflictPath] = useState<string | null>(null);
  const watcherRef = useRef<ReturnType<typeof createFileWatcher> | null>(null);

  const handleFileChanged = useCallback(async (path: string) => {
    const state = useConfigStore.getState();
    const fileState = state.openFiles.get(path);

    if (!fileState) {
      // File not open — just refresh the file list silently
      return;
    }

    if (fileState.modified) {
      // File has unsaved changes — show conflict dialog
      setConflictPath(path);
    } else {
      // File is unmodified — auto-reload silently
      try {
        const newContent = await readConfig(path);
        useConfigStore.getState().reloadFile(path, newContent);
      } catch (err) {
        console.error("Failed to auto-reload file:", err);
      }
    }
  }, []);

  const handleReload = useCallback(async () => {
    if (!conflictPath) return;
    try {
      const newContent = await readConfig(conflictPath);
      useConfigStore.getState().reloadFile(conflictPath, newContent);
    } catch (err) {
      console.error("Failed to reload file:", err);
    }
    setConflictPath(null);
  }, [conflictPath]);

  const handleKeepMine = useCallback(() => {
    setConflictPath(null);
  }, []);

  // Set up file watcher
  useEffect(() => {
    const paths: string[] = [];
    if (globalConfig?.exists) paths.push(globalConfig.path);
    if (projectConfig?.exists) paths.push(projectConfig.path);

    if (paths.length === 0) return;

    const watcher = createFileWatcher({
      paths,
      onFileChanged: handleFileChanged,
      debounceMs: 500,
    });

    watcherRef.current = watcher;
    watcher.start();

    return () => {
      watcher.stop();
      watcherRef.current = null;
    };
  }, [
    globalConfig?.exists,
    globalConfig?.path,
    projectConfig?.exists,
    projectConfig?.path,
    handleFileChanged,
  ]);

  return (
    <div className="flex flex-col h-screen w-screen overflow-hidden bg-white dark:bg-[#1A1A1A]">
      {/* Header */}
      <Header />

      {/* Main content area */}
      <div className="flex flex-1 overflow-hidden">
        {/* Left sidebar */}
        {sidebarOpen && <Sidebar />}

        {/* Editor area */}
        <MainContent />

        {/* Right panel (Simulator/Tree) */}
        <RightPanel />
      </div>

      {/* Status bar */}
      <StatusBar />

      {/* External change conflict dialog */}
      {conflictPath && (
        <ExternalChangeDialog
          filePath={conflictPath}
          onReload={handleReload}
          onKeepMine={handleKeepMine}
        />
      )}
    </div>
  );
}
