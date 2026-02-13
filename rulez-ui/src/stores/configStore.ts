import type { ConfigFile, FileState } from "@/types";
import { create } from "zustand";

export type ActiveScope = "project" | "global" | "none";

export interface ScopeInfo {
  activeScope: ActiveScope;
  globalStatus: "active" | "overridden" | "missing";
  projectStatus: "active" | "missing";
}

interface ConfigState {
  globalConfig: ConfigFile | null;
  projectConfig: ConfigFile | null;
  activeFile: string | null;
  openFiles: Map<string, FileState>;
}

interface ConfigActions {
  setGlobalConfig: (config: ConfigFile | null) => void;
  setProjectConfig: (config: ConfigFile | null) => void;
  setActiveFile: (path: string | null) => void;
  openFile: (path: string, content: string) => void;
  closeFile: (path: string) => void;
  updateContent: (path: string, content: string) => void;
  markSaved: (path: string) => void;
  reloadFile: (path: string, content: string) => void;
  getActiveContent: () => string | null;
  hasUnsavedChanges: () => boolean;
  getScopeInfo: () => ScopeInfo;
}

export const useConfigStore = create<ConfigState & ConfigActions>((set, get) => ({
  // State
  globalConfig: null,
  projectConfig: null,
  activeFile: null,
  openFiles: new Map(),

  // Actions
  setGlobalConfig: (globalConfig) => set({ globalConfig }),

  setProjectConfig: (projectConfig) => set({ projectConfig }),

  setActiveFile: (activeFile) => set({ activeFile }),

  openFile: (path, content) =>
    set((state) => {
      const newOpenFiles = new Map(state.openFiles);
      if (!newOpenFiles.has(path)) {
        newOpenFiles.set(path, {
          content,
          originalContent: content,
          modified: false,
        });
      }
      return { openFiles: newOpenFiles, activeFile: path };
    }),

  closeFile: (path) =>
    set((state) => {
      const newOpenFiles = new Map(state.openFiles);
      newOpenFiles.delete(path);

      // If closing the active file, switch to another open file
      let newActiveFile = state.activeFile;
      if (state.activeFile === path) {
        const remaining = Array.from(newOpenFiles.keys());
        newActiveFile = remaining.length > 0 ? remaining[0]! : null;
      }

      return { openFiles: newOpenFiles, activeFile: newActiveFile };
    }),

  updateContent: (path, content) =>
    set((state) => {
      const newOpenFiles = new Map(state.openFiles);
      const fileState = newOpenFiles.get(path);
      if (fileState) {
        newOpenFiles.set(path, {
          ...fileState,
          content,
          modified: content !== fileState.originalContent,
        });
      }
      return { openFiles: newOpenFiles };
    }),

  markSaved: (path) =>
    set((state) => {
      const newOpenFiles = new Map(state.openFiles);
      const fileState = newOpenFiles.get(path);
      if (fileState) {
        newOpenFiles.set(path, {
          ...fileState,
          originalContent: fileState.content,
          modified: false,
        });
      }
      return { openFiles: newOpenFiles };
    }),

  reloadFile: (path, content) =>
    set((state) => {
      const newOpenFiles = new Map(state.openFiles);
      if (newOpenFiles.has(path)) {
        newOpenFiles.set(path, {
          content,
          originalContent: content,
          modified: false,
        });
      }
      return { openFiles: newOpenFiles };
    }),

  getActiveContent: () => {
    const state = get();
    if (!state.activeFile) return null;
    return state.openFiles.get(state.activeFile)?.content ?? null;
  },

  hasUnsavedChanges: () => {
    const state = get();
    for (const fileState of state.openFiles.values()) {
      if (fileState.modified) return true;
    }
    return false;
  },

  getScopeInfo: (): ScopeInfo => {
    const { globalConfig, projectConfig } = get();
    const projectExists = projectConfig?.exists === true;
    const globalExists = globalConfig?.exists === true;

    if (projectExists) {
      return {
        activeScope: "project",
        globalStatus: globalExists ? "overridden" : "missing",
        projectStatus: "active",
      };
    }
    if (globalExists) {
      return {
        activeScope: "global",
        globalStatus: "active",
        projectStatus: "missing",
      };
    }
    return {
      activeScope: "none",
      globalStatus: "missing",
      projectStatus: "missing",
    };
  },
}));
