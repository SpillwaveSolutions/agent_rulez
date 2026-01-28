import { create } from "zustand";

export type Theme = "light" | "dark" | "system";
export type RightPanelTab = "simulator" | "tree";

interface UIState {
  theme: Theme;
  sidebarOpen: boolean;
  rightPanelTab: RightPanelTab;
  statusMessage: string | null;
}

interface UIActions {
  setTheme: (theme: Theme) => void;
  toggleSidebar: () => void;
  setSidebarOpen: (open: boolean) => void;
  setRightPanelTab: (tab: RightPanelTab) => void;
  setStatusMessage: (message: string | null) => void;
}

export const useUIStore = create<UIState & UIActions>((set) => ({
  // State
  theme: "system",
  sidebarOpen: true,
  rightPanelTab: "simulator",
  statusMessage: null,

  // Actions
  setTheme: (theme) => {
    localStorage.setItem("rulez-ui-theme", theme);
    set({ theme });
  },

  toggleSidebar: () => set((state) => ({ sidebarOpen: !state.sidebarOpen })),

  setSidebarOpen: (sidebarOpen) => set({ sidebarOpen }),

  setRightPanelTab: (rightPanelTab) => set({ rightPanelTab }),

  setStatusMessage: (statusMessage) => set({ statusMessage }),
}));
