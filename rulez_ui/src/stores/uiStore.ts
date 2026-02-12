import { create } from "zustand";

export type RightPanelTab = "simulator" | "tree" | "settings";

interface UIState {
  sidebarOpen: boolean;
  rightPanelTab: RightPanelTab;
  statusMessage: string | null;
}

interface UIActions {
  toggleSidebar: () => void;
  setSidebarOpen: (open: boolean) => void;
  setRightPanelTab: (tab: RightPanelTab) => void;
  setStatusMessage: (message: string | null) => void;
}

export const useUIStore = create<UIState & UIActions>((set) => ({
  // State
  sidebarOpen: true,
  rightPanelTab: "simulator",
  statusMessage: null,

  // Actions
  toggleSidebar: () => set((state) => ({ sidebarOpen: !state.sidebarOpen })),

  setSidebarOpen: (sidebarOpen) => set({ sidebarOpen }),

  setRightPanelTab: (rightPanelTab) => set({ rightPanelTab }),

  setStatusMessage: (statusMessage) => set({ statusMessage }),
}));
