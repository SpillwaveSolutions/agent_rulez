import { readConfig } from "@/lib/tauri";
import { useSettingsStore } from "@/stores/settingsStore";
import { DARK_THEME_NAME, LIGHT_THEME_NAME, darkTheme, lightTheme } from "@/styles/monaco-theme";
import { DiffEditor, type BeforeMount, type DiffOnMount } from "@monaco-editor/react";
import { useCallback, useEffect, useMemo, useRef, useState } from "react";

type LoadState = "loading" | "loaded" | "error";

interface ConfigState {
  content: string;
  loadState: LoadState;
  error?: string;
  exists: boolean;
}

const EMPTY_CONFIG: ConfigState = {
  content: "",
  loadState: "loading",
  exists: false,
};

const GLOBAL_PATH = "~/.claude/hooks.yaml";
const PROJECT_PATH = ".claude/hooks.yaml";

function useResolvedTheme(): "light" | "dark" {
  const theme = useSettingsStore((s) => s.settings.theme);
  return useMemo(() => {
    if (theme === "system") {
      return window.matchMedia("(prefers-color-scheme: dark)").matches ? "dark" : "light";
    }
    return theme;
  }, [theme]);
}

export function ConfigDiffView() {
  const [globalConfig, setGlobalConfig] = useState<ConfigState>(EMPTY_CONFIG);
  const [projectConfig, setProjectConfig] = useState<ConfigState>(EMPTY_CONFIG);
  const editorFontSize = useSettingsStore((s) => s.settings.editorFontSize);
  const editorTabSize = useSettingsStore((s) => s.settings.editorTabSize);
  const resolvedTheme = useResolvedTheme();
  const themesDefined = useRef(false);

  const monacoThemeName = useMemo(
    () => (resolvedTheme === "dark" ? DARK_THEME_NAME : LIGHT_THEME_NAME),
    [resolvedTheme],
  );

  // Load both configs on mount
  useEffect(() => {
    let cancelled = false;

    async function loadConfigs() {
      // Load global config
      try {
        const content = await readConfig(GLOBAL_PATH);
        if (!cancelled) {
          setGlobalConfig({ content, loadState: "loaded", exists: true });
        }
      } catch (err) {
        if (!cancelled) {
          setGlobalConfig({
            content: "# Global config not found\n# Expected at: ~/.claude/hooks.yaml\n",
            loadState: "loaded",
            exists: false,
            error: String(err),
          });
        }
      }

      // Load project config
      try {
        const content = await readConfig(PROJECT_PATH);
        if (!cancelled) {
          setProjectConfig({ content, loadState: "loaded", exists: true });
        }
      } catch (err) {
        if (!cancelled) {
          setProjectConfig({
            content: "# Project config not found\n# Expected at: .claude/hooks.yaml\n",
            loadState: "loaded",
            exists: false,
            error: String(err),
          });
        }
      }
    }

    void loadConfigs();
    return () => {
      cancelled = true;
    };
  }, []);

  const handleBeforeMount: BeforeMount = useCallback((monaco) => {
    if (!themesDefined.current) {
      monaco.editor.defineTheme(LIGHT_THEME_NAME, lightTheme);
      monaco.editor.defineTheme(DARK_THEME_NAME, darkTheme);
      themesDefined.current = true;
    }
  }, []);

  const handleDiffMount: DiffOnMount = useCallback((editor) => {
    // Focus the modified (right) editor on mount
    editor.getModifiedEditor().focus();
  }, []);

  const isLoading = globalConfig.loadState === "loading" || projectConfig.loadState === "loading";

  return (
    <div className="flex flex-col h-full">
      {/* Header bar */}
      <div className="flex items-center justify-between px-4 py-2 border-b border-gray-200 dark:border-gray-700 bg-gray-50 dark:bg-gray-900">
        <h2 className="text-sm font-semibold text-gray-900 dark:text-gray-100">
          Config Diff View
        </h2>
        <div className="flex items-center gap-4">
          <StatusBadge label="Global" exists={globalConfig.exists} />
          <StatusBadge label="Project" exists={projectConfig.exists} />
        </div>
      </div>

      {/* Column headers */}
      <div className="grid grid-cols-2 border-b border-gray-200 dark:border-gray-700">
        <div className="px-4 py-1.5 text-xs font-medium text-gray-600 dark:text-gray-400 border-r border-gray-200 dark:border-gray-700 bg-gray-50/50 dark:bg-gray-900/50">
          Global Config
          <span className="ml-2 font-mono text-gray-400 dark:text-gray-500">
            ~/.claude/hooks.yaml
          </span>
        </div>
        <div className="px-4 py-1.5 text-xs font-medium text-gray-600 dark:text-gray-400 bg-gray-50/50 dark:bg-gray-900/50">
          Project Config
          <span className="ml-2 font-mono text-gray-400 dark:text-gray-500">
            .claude/hooks.yaml
          </span>
        </div>
      </div>

      {/* Precedence legend */}
      <div className="px-4 py-1.5 text-xs text-gray-500 dark:text-gray-400 border-b border-gray-200 dark:border-gray-700 bg-gray-50/30 dark:bg-gray-900/30 flex items-center gap-4">
        <span className="font-medium">Precedence:</span>
        <span>
          Project config overrides global config for matching rule names.
          Non-overlapping rules from both files are merged.
        </span>
      </div>

      {/* Editor area */}
      <div className="flex-1 min-h-0">
        {isLoading ? (
          <div className="flex items-center justify-center h-full text-sm text-gray-500 dark:text-gray-400">
            Loading configurations...
          </div>
        ) : (
          <DiffEditor
            height="100%"
            language="yaml"
            original={globalConfig.content}
            modified={projectConfig.content}
            beforeMount={handleBeforeMount}
            onMount={handleDiffMount}
            theme={monacoThemeName}
            options={{
              readOnly: true,
              minimap: { enabled: false },
              wordWrap: "off",
              tabSize: editorTabSize,
              fontSize: editorFontSize,
              lineNumbers: "on",
              renderSideBySide: true,
              scrollBeyondLastLine: false,
              automaticLayout: true,
              padding: { top: 8, bottom: 8 },
              renderOverviewRuler: true,
              originalEditable: false,
              enableSplitViewResizing: true,
            }}
          />
        )}
      </div>
    </div>
  );
}

/** Small badge indicating whether a config file exists on disk. */
function StatusBadge({ label, exists }: { label: string; exists: boolean }) {
  return (
    <span className="inline-flex items-center gap-1 text-xs">
      <span
        className={`inline-block w-2 h-2 rounded-full ${
          exists
            ? "bg-green-500"
            : "bg-gray-400 dark:bg-gray-600"
        }`}
      />
      <span className="text-gray-600 dark:text-gray-400">{label}</span>
      <span className={`font-medium ${exists ? "text-green-700 dark:text-green-400" : "text-gray-500 dark:text-gray-500"}`}>
        {exists ? "found" : "missing"}
      </span>
    </span>
  );
}
