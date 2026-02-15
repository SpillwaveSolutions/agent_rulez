import { isTauri } from "@/lib/tauri";
import { useUIStore } from "@/stores/uiStore";
import { ThemeToggle } from "../ui/ThemeToggle";

export function Header() {
  const { toggleSidebar, sidebarOpen, setRightPanelTab, mainView, setMainView } = useUIStore();

  return (
    <header className="flex items-center justify-between h-12 px-4 border-b border-gray-200 dark:border-gray-700 bg-surface dark:bg-surface-dark no-select">
      {/* Left section */}
      <div className="flex items-center gap-3">
        {/* Sidebar toggle */}
        <button
          type="button"
          onClick={toggleSidebar}
          className="p-1.5 rounded hover:bg-gray-200 dark:hover:bg-gray-700 transition-colors"
          aria-label={sidebarOpen ? "Hide sidebar" : "Show sidebar"}
          title={sidebarOpen ? "Hide sidebar" : "Show sidebar"}
        >
          <svg
            className="w-5 h-5 text-gray-600 dark:text-gray-400"
            fill="none"
            stroke="currentColor"
            viewBox="0 0 24 24"
            aria-hidden="true"
          >
            <path
              strokeLinecap="round"
              strokeLinejoin="round"
              strokeWidth={2}
              d="M4 6h16M4 12h16M4 18h16"
            />
          </svg>
        </button>

        {/* Logo and title */}
        <div className="flex items-center gap-2">
          <svg
            className="w-6 h-6 text-accent dark:text-accent-dark"
            viewBox="0 0 24 24"
            fill="currentColor"
            aria-hidden="true"
          >
            <path d="M12 2L2 7l10 5 10-5-10-5zM2 17l10 5 10-5M2 12l10 5 10-5" />
          </svg>
          <span className="font-semibold text-lg text-gray-900 dark:text-gray-100">RuleZ UI</span>
        </div>

        {/* View switcher */}
        <div className="flex items-center gap-0.5 bg-gray-200 dark:bg-gray-700 rounded p-0.5">
          <button
            type="button"
            onClick={() => setMainView("editor")}
            className={`px-2 py-0.5 text-xs rounded transition-colors ${
              mainView === "editor"
                ? "bg-white dark:bg-gray-600 text-gray-900 dark:text-gray-100 shadow-sm"
                : "text-gray-600 dark:text-gray-400 hover:text-gray-900 dark:hover:text-gray-200"
            }`}
            aria-label="Editor"
          >
            Editor
          </button>
          <button
            type="button"
            onClick={() => setMainView("logs")}
            className={`px-2 py-0.5 text-xs rounded transition-colors ${
              mainView === "logs"
                ? "bg-white dark:bg-gray-600 text-gray-900 dark:text-gray-100 shadow-sm"
                : "text-gray-600 dark:text-gray-400 hover:text-gray-900 dark:hover:text-gray-200"
            }`}
            aria-label="Logs"
          >
            Logs
          </button>
        </div>

        {/* Mode indicator */}
        <span className="text-xs px-2 py-0.5 rounded bg-gray-200 dark:bg-gray-700 text-gray-600 dark:text-gray-400">
          {isTauri() ? "Desktop" : "Web (Test)"}
        </span>
      </div>

      {/* Right section */}
      <div className="flex items-center gap-2">
        {/* Settings button */}
        <button
          type="button"
          onClick={() => setRightPanelTab("settings")}
          className="p-1.5 rounded hover:bg-gray-200 dark:hover:bg-gray-700 transition-colors"
          aria-label="Open settings"
          title="Settings"
        >
          <svg
            className="w-5 h-5 text-gray-600 dark:text-gray-400"
            fill="none"
            stroke="currentColor"
            viewBox="0 0 24 24"
            aria-hidden="true"
          >
            <path
              strokeLinecap="round"
              strokeLinejoin="round"
              strokeWidth={2}
              d="M12 8c-2.21 0-4 1.79-4 4s1.79 4 4 4 4-1.79 4-4-1.79-4-4-4zm8.94 4c0-.59-.06-1.17-.17-1.73l2.07-1.62-2-3.46-2.49 1a7.952 7.952 0 00-2.99-1.73l-.38-2.65H9.02l-.38 2.65a7.952 7.952 0 00-2.99 1.73l-2.49-1-2 3.46 2.07 1.62c-.11.56-.17 1.14-.17 1.73s.06 1.17.17 1.73l-2.07 1.62 2 3.46 2.49-1c.86.72 1.88 1.28 2.99 1.73l.38 2.65h3.96l.38-2.65a7.952 7.952 0 002.99-1.73l2.49 1 2-3.46-2.07-1.62c.11-.56.17-1.14.17-1.73z"
            />
          </svg>
        </button>

        {/* Help button */}
        <button
          type="button"
          className="p-1.5 rounded hover:bg-gray-200 dark:hover:bg-gray-700 transition-colors"
          aria-label="Help"
          title="Help"
        >
          <svg
            className="w-5 h-5 text-gray-600 dark:text-gray-400"
            fill="none"
            stroke="currentColor"
            viewBox="0 0 24 24"
            aria-hidden="true"
          >
            <path
              strokeLinecap="round"
              strokeLinejoin="round"
              strokeWidth={2}
              d="M8.228 9c.549-1.165 2.03-2 3.772-2 2.21 0 4 1.343 4 3 0 1.4-1.278 2.575-3.006 2.907-.542.104-.994.54-.994 1.093m0 3h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z"
            />
          </svg>
        </button>

        {/* Theme toggle */}
        <ThemeToggle />
      </div>
    </header>
  );
}
