import { useSettingsStore } from "@/stores/settingsStore";

const inputClassName =
  "w-full px-3 py-2 text-sm rounded border border-gray-300 dark:border-gray-600 bg-white dark:bg-[#1A1A1A] text-gray-900 dark:text-gray-100 placeholder-gray-400 focus:outline-none focus:ring-2 focus:ring-accent";

export function SettingsPanel() {
  const settings = useSettingsStore((s) => s.settings);
  const setTheme = useSettingsStore((s) => s.setTheme);
  const setEditorFontSize = useSettingsStore((s) => s.setEditorFontSize);
  const setEditorTabSize = useSettingsStore((s) => s.setEditorTabSize);
  const setRulezBinaryPath = useSettingsStore((s) => s.setRulezBinaryPath);

  const handleFontSizeChange = (value: string) => {
    const next = Number.parseInt(value, 10);
    if (Number.isNaN(next) || next <= 0) return;
    void setEditorFontSize(next);
  };

  const handleTabSizeChange = (value: string) => {
    const next = Number.parseInt(value, 10);
    if (Number.isNaN(next) || next <= 0) return;
    void setEditorTabSize(next);
  };

  const handleBinaryPathChange = (value: string) => {
    void setRulezBinaryPath(value.trim() ? value : null);
  };

  return (
    <div className="space-y-5">
      <div>
        <h2 className="text-sm font-semibold text-gray-900 dark:text-gray-100">Settings</h2>
        <p className="text-xs text-gray-500 dark:text-gray-400">
          Theme and editor preferences apply immediately.
        </p>
      </div>

      <div className="space-y-4">
        <div>
          <label
            htmlFor="theme"
            className="block text-xs font-medium text-gray-600 dark:text-gray-400 mb-1"
          >
            Theme
          </label>
          <select
            id="theme"
            value={settings.theme}
            onChange={(e) => void setTheme(e.target.value as typeof settings.theme)}
            className={inputClassName}
          >
            <option value="system">System</option>
            <option value="light">Light</option>
            <option value="dark">Dark</option>
          </select>
        </div>

        <div>
          <label
            htmlFor="editor-font-size"
            className="block text-xs font-medium text-gray-600 dark:text-gray-400 mb-1"
          >
            Editor font size
          </label>
          <input
            id="editor-font-size"
            type="number"
            min={10}
            max={24}
            value={settings.editorFontSize}
            onChange={(e) => handleFontSizeChange(e.target.value)}
            className={inputClassName}
          />
        </div>

        <div>
          <label
            htmlFor="editor-tab-size"
            className="block text-xs font-medium text-gray-600 dark:text-gray-400 mb-1"
          >
            Editor tab size
          </label>
          <input
            id="editor-tab-size"
            type="number"
            min={2}
            max={8}
            value={settings.editorTabSize}
            onChange={(e) => handleTabSizeChange(e.target.value)}
            className={inputClassName}
          />
        </div>

        <div>
          <label
            htmlFor="rulez-binary-path"
            className="block text-xs font-medium text-gray-600 dark:text-gray-400 mb-1"
          >
            RuleZ binary path
          </label>
          <input
            id="rulez-binary-path"
            type="text"
            value={settings.rulezBinaryPath ?? ""}
            onChange={(e) => handleBinaryPathChange(e.target.value)}
            placeholder="/usr/local/bin/rulez"
            className={inputClassName}
          />
          <p className="mt-1 text-xs text-gray-500 dark:text-gray-400">
            Leave blank to use the default PATH lookup.
          </p>
        </div>
      </div>
    </div>
  );
}
