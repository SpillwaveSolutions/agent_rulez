import { isTauri } from "./tauri";

export type Theme = "light" | "dark" | "system";

export interface Settings {
  theme: Theme;
  editorFontSize: number;
  editorTabSize: number;
  rulezBinaryPath: string | null;
  onboardingComplete: boolean;
}

export const SETTINGS_DEFAULTS: Settings = {
  theme: "system",
  editorFontSize: 14,
  editorTabSize: 2,
  rulezBinaryPath: null,
  onboardingComplete: false,
};

const STORE_FILE = "settings.json";
const STORE_KEY = "settings";
const LOCAL_STORAGE_KEY = "rulez-ui-settings";

let storePromise: Promise<import("@tauri-apps/plugin-store").Store> | null = null;

function mergeSettings(settings?: Partial<Settings> | null): Settings {
  return { ...SETTINGS_DEFAULTS, ...(settings ?? {}) };
}

async function getStore() {
  const { Store } = await import("@tauri-apps/plugin-store");
  if (!storePromise) {
    storePromise = Store.load(STORE_FILE, {
      autoSave: true,
      defaults: {
        [STORE_KEY]: SETTINGS_DEFAULTS,
      },
    });
  }
  return storePromise;
}

function loadLocalSettings(): Settings {
  if (typeof window === "undefined") {
    return SETTINGS_DEFAULTS;
  }

  const raw = window.localStorage.getItem(LOCAL_STORAGE_KEY);
  if (!raw) {
    return SETTINGS_DEFAULTS;
  }

  try {
    const parsed = JSON.parse(raw) as Partial<Settings>;
    return mergeSettings(parsed);
  } catch {
    return SETTINGS_DEFAULTS;
  }
}

function saveLocalSettings(settings: Settings): void {
  if (typeof window === "undefined") {
    return;
  }

  window.localStorage.setItem(LOCAL_STORAGE_KEY, JSON.stringify(settings));
}

export async function loadSettings(): Promise<Settings> {
  if (isTauri()) {
    const store = await getStore();
    const stored = await store.get<Settings>(STORE_KEY);
    return mergeSettings(stored ?? undefined);
  }

  return loadLocalSettings();
}

export async function updateSettings(partial: Partial<Settings>): Promise<Settings> {
  const current = await loadSettings();
  const next = { ...current, ...partial };

  if (isTauri()) {
    const store = await getStore();
    await store.set(STORE_KEY, next);
    await store.save();
    return next;
  }

  saveLocalSettings(next);
  return next;
}
