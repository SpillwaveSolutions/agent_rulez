import { useUIStore } from "@/stores/uiStore";

export function RightPanel() {
  const { rightPanelTab, setRightPanelTab } = useUIStore();

  return (
    <aside className="w-80 flex-shrink-0 border-l border-gray-200 dark:border-gray-700 bg-surface dark:bg-surface-dark flex flex-col">
      {/* Panel tabs */}
      <div className="flex border-b border-gray-200 dark:border-gray-700">
        <button
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
          onClick={() => setRightPanelTab("tree")}
          className={`flex-1 px-4 py-2 text-sm font-medium transition-colors ${
            rightPanelTab === "tree"
              ? "text-accent dark:text-accent-dark border-b-2 border-accent dark:border-accent-dark"
              : "text-gray-600 dark:text-gray-400 hover:text-gray-900 dark:hover:text-gray-200"
          }`}
        >
          Rules
        </button>
      </div>

      {/* Panel content */}
      <div className="flex-1 overflow-y-auto p-4">
        {rightPanelTab === "simulator" ? <SimulatorPlaceholder /> : <RulesTreePlaceholder />}
      </div>
    </aside>
  );
}

function SimulatorPlaceholder() {
  return (
    <div className="space-y-4">
      <h3 className="text-sm font-semibold text-gray-700 dark:text-gray-300">Debug Simulator</h3>
      <p className="text-xs text-gray-500 dark:text-gray-400">
        Test your CCH rules by simulating events.
      </p>

      {/* Placeholder form */}
      <div className="space-y-3">
        <div>
          <label className="block text-xs font-medium text-gray-600 dark:text-gray-400 mb-1">
            Event Type
          </label>
          <select
            className="w-full px-3 py-2 text-sm rounded border border-gray-300 dark:border-gray-600 bg-white dark:bg-[#1A1A1A] text-gray-900 dark:text-gray-100 focus:outline-none focus:ring-2 focus:ring-accent"
            defaultValue="PreToolUse"
          >
            <option value="PreToolUse">PreToolUse</option>
            <option value="PostToolUse">PostToolUse</option>
            <option value="PermissionRequest">PermissionRequest</option>
            <option value="UserPromptSubmit">UserPromptSubmit</option>
            <option value="SessionStart">SessionStart</option>
            <option value="SessionEnd">SessionEnd</option>
            <option value="PreCompact">PreCompact</option>
          </select>
        </div>

        <div>
          <label className="block text-xs font-medium text-gray-600 dark:text-gray-400 mb-1">
            Tool
          </label>
          <input
            type="text"
            placeholder="e.g., Bash"
            className="w-full px-3 py-2 text-sm rounded border border-gray-300 dark:border-gray-600 bg-white dark:bg-[#1A1A1A] text-gray-900 dark:text-gray-100 placeholder-gray-400 focus:outline-none focus:ring-2 focus:ring-accent"
          />
        </div>

        <div>
          <label className="block text-xs font-medium text-gray-600 dark:text-gray-400 mb-1">
            Command
          </label>
          <input
            type="text"
            placeholder="e.g., git push --force"
            className="w-full px-3 py-2 text-sm rounded border border-gray-300 dark:border-gray-600 bg-white dark:bg-[#1A1A1A] text-gray-900 dark:text-gray-100 placeholder-gray-400 focus:outline-none focus:ring-2 focus:ring-accent"
          />
        </div>

        <button
          className="w-full px-4 py-2 text-sm font-medium text-white bg-accent hover:bg-accent/90 rounded transition-colors disabled:opacity-50"
          disabled
        >
          Simulate (Coming in M6)
        </button>
      </div>
    </div>
  );
}

function RulesTreePlaceholder() {
  return (
    <div className="space-y-4">
      <h3 className="text-sm font-semibold text-gray-700 dark:text-gray-300">Rule Tree</h3>
      <p className="text-xs text-gray-500 dark:text-gray-400">
        Visual tree of all configured rules.
      </p>

      {/* Placeholder tree */}
      <div className="space-y-2 text-sm">
        <div className="flex items-center gap-2 text-gray-500 dark:text-gray-400">
          <svg className="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M9 5l7 7-7 7" />
          </svg>
          <span>Settings</span>
        </div>
        <div className="flex items-center gap-2 text-gray-500 dark:text-gray-400">
          <svg className="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M9 5l7 7-7 7" />
          </svg>
          <span>Rules (Coming in M5)</span>
        </div>
      </div>
    </div>
  );
}
