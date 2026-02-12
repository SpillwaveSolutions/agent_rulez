import { RuleTreeView } from "@/components/editor/RuleTreeView";
import { SettingsPanel } from "@/components/settings/SettingsPanel";
import { DebugSimulator } from "@/components/simulator/DebugSimulator";
import { useUIStore } from "@/stores/uiStore";

export function RightPanel() {
  const { rightPanelTab, setRightPanelTab } = useUIStore();

  return (
    <aside className="w-80 flex-shrink-0 border-l border-gray-200 dark:border-gray-700 bg-surface dark:bg-surface-dark flex flex-col">
      {/* Panel tabs */}
      <div className="flex border-b border-gray-200 dark:border-gray-700">
        <button
          type="button"
          onClick={() => setRightPanelTab("simulator")}
          className={`flex-1 px-4 py-2 text-sm font-medium transition-colors ${
            rightPanelTab === "simulator"
              ? "text-accent dark:text-accent-dark border-b-2 border-accent dark:border-accent-dark"
              : "text-gray-600 dark:text-gray-400 hover:text-gray-900 dark:hover:text-gray-200"
          }`}
        >
          Simulator
        </button>
        <button
          type="button"
          onClick={() => setRightPanelTab("tree")}
          className={`flex-1 px-4 py-2 text-sm font-medium transition-colors ${
            rightPanelTab === "tree"
              ? "text-accent dark:text-accent-dark border-b-2 border-accent dark:border-accent-dark"
              : "text-gray-600 dark:text-gray-400 hover:text-gray-900 dark:hover:text-gray-200"
          }`}
        >
          Rules
        </button>
        <button
          type="button"
          onClick={() => setRightPanelTab("settings")}
          className={`flex-1 px-4 py-2 text-sm font-medium transition-colors ${
            rightPanelTab === "settings"
              ? "text-accent dark:text-accent-dark border-b-2 border-accent dark:border-accent-dark"
              : "text-gray-600 dark:text-gray-400 hover:text-gray-900 dark:hover:text-gray-200"
          }`}
        >
          Settings
        </button>
      </div>

      {/* Panel content */}
      <div className="flex-1 overflow-y-auto p-4">
        {rightPanelTab === "simulator" && <DebugSimulator />}
        {rightPanelTab === "tree" && <RuleTreeView />}
        {rightPanelTab === "settings" && <SettingsPanel />}
      </div>
    </aside>
  );
}
