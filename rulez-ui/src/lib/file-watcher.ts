import { isTauri } from "./tauri";

interface FileWatcherOptions {
  paths: string[];
  onFileChanged: (path: string) => void;
  debounceMs?: number;
}

interface FileWatcher {
  start: () => Promise<void>;
  stop: () => void;
  updatePaths: (paths: string[]) => Promise<void>;
}

/**
 * Creates a file watcher that monitors config files for external changes.
 * Uses Tauri's watchImmediate() in desktop mode; no-ops in browser mode.
 * Events are debounced to handle editors that do save-to-temp-then-rename.
 */
export function createFileWatcher(options: FileWatcherOptions): FileWatcher {
  const debounceMs = options.debounceMs ?? 500;
  let stopFns: Array<() => void> = [];
  let debounceTimers = new Map<string, ReturnType<typeof setTimeout>>();
  let currentPaths = [...options.paths];

  function debouncedNotify(path: string) {
    const existing = debounceTimers.get(path);
    if (existing) clearTimeout(existing);
    debounceTimers.set(
      path,
      setTimeout(() => {
        debounceTimers.delete(path);
        options.onFileChanged(path);
      }, debounceMs),
    );
  }

  async function startWatching(paths: string[]) {
    if (!isTauri()) return;

    try {
      const { watchImmediate } = await import("@tauri-apps/plugin-fs");

      for (const filePath of paths) {
        try {
          const unwatch = await watchImmediate(filePath, (event) => {
            // watchImmediate returns DebouncedEvent which contains event types
            const eventData = event as { type?: Record<string, unknown> | string };
            const eventType = eventData.type;

            // Check for modify events
            if (eventType && typeof eventType === "object" && "modify" in eventType) {
              debouncedNotify(filePath);
            } else if (eventType && typeof eventType === "string" && eventType === "modify") {
              debouncedNotify(filePath);
            }
          });
          stopFns.push(unwatch);
        } catch {
          // File might not exist — skip gracefully
        }
      }
    } catch {
      // watchImmediate not available — skip gracefully
    }
  }

  function stopWatching() {
    for (const fn of stopFns) {
      fn();
    }
    stopFns = [];
    for (const timer of debounceTimers.values()) {
      clearTimeout(timer);
    }
    debounceTimers = new Map();
  }

  return {
    start: async () => {
      await startWatching(currentPaths);
    },

    stop: () => {
      stopWatching();
    },

    updatePaths: async (paths: string[]) => {
      stopWatching();
      currentPaths = [...paths];
      await startWatching(currentPaths);
    },
  };
}
