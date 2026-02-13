import { create } from "zustand";

export type RightPanelTab = "simulator" | "tree" | "settings";
export type MainView = "editor" | "logs";

interface UIState {
  sidebarOpen: boolean;
  rightPanelTab: RightPanelTab;
  mainView: MainView;
  statusMessage: string | null;
}

interface UIActions {
  toggleSidebar: () => void;
  setSidebarOpen: (open: boolean) => void;
  setRightPanelTab: (tab: RightPanelTab) => void;
  setMainView: (view: MainView) => void;
  setStatusMessage: (message: string | null) => void;
}

export const useUIStore = create<UIState & UIActions>((set) => ({
  // State
  sidebarOpen: true,
  rightPanelTab: "simulator",
  mainView: "editor",
  statusMessage: null,

  // Actions
  toggleSidebar: () => set((state) => ({ sidebarOpen: !state.sidebarOpen })),

  setSidebarOpen: (sidebarOpen) => set({ sidebarOpen }),

  setRightPanelTab: (rightPanelTab) => set({ rightPanelTab }),

  setMainView: (mainView) => set({ mainView }),

  setStatusMessage: (statusMessage) => set({ statusMessage }),
}));
