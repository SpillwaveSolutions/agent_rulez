import { isTauri } from "@/lib/tauri";
import { useUIStore } from "@/stores/uiStore";
import { ThemeToggle } from "../ui/ThemeToggle";

export function Header() {
  const { toggleSidebar, sidebarOpen } = useUIStore();

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

        {/* Mode indicator */}
        <span className="text-xs px-2 py-0.5 rounded bg-gray-200 dark:bg-gray-700 text-gray-600 dark:text-gray-400">
          {isTauri() ? "Desktop" : "Web (Test)"}
        </span>
      </div>

      {/* Right section */}
      <div className="flex items-center gap-2">
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
