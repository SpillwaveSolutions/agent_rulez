import { create } from "zustand";

import {
  SETTINGS_DEFAULTS,
  type Settings,
  loadSettings,
  updateSettings as persistSettings,
} from "@/lib/settings";

interface SettingsState {
  settings: Settings;
  isLoaded: boolean;
}

interface SettingsActions {
  loadSettings: () => Promise<void>;
  updateSettings: (partial: Partial<Settings>) => Promise<void>;
  setTheme: (theme: Settings["theme"]) => Promise<void>;
  setEditorFontSize: (size: number) => Promise<void>;
  setEditorTabSize: (size: number) => Promise<void>;
  setRulezBinaryPath: (path: string | null) => Promise<void>;
  setOnboardingComplete: (complete: boolean) => Promise<void>;
}

export const useSettingsStore = create<SettingsState & SettingsActions>((set) => ({
  settings: SETTINGS_DEFAULTS,
  isLoaded: false,

  loadSettings: async () => {
    const settings = await loadSettings();
    set({ settings, isLoaded: true });
  },

  updateSettings: async (partial) => {
    const settings = await persistSettings(partial);
    set({ settings, isLoaded: true });
  },

  setTheme: async (theme) => {
    const settings = await persistSettings({ theme });
    set({ settings, isLoaded: true });
  },

  setEditorFontSize: async (editorFontSize) => {
    const settings = await persistSettings({ editorFontSize });
    set({ settings, isLoaded: true });
  },

  setEditorTabSize: async (editorTabSize) => {
    const settings = await persistSettings({ editorTabSize });
    set({ settings, isLoaded: true });
  },

  setRulezBinaryPath: async (rulezBinaryPath) => {
    const settings = await persistSettings({ rulezBinaryPath });
    set({ settings, isLoaded: true });
  },

  setOnboardingComplete: async (onboardingComplete) => {
    const settings = await persistSettings({ onboardingComplete });
    set({ settings, isLoaded: true });
  },
}));
