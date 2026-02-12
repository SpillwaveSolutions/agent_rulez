import { useLogStore } from "@/stores/logStore";
import { useCallback, useEffect, useRef, useState } from "react";

export function LogFilterBar() {
  const {
    entries,
    filteredEntries,
    severityFilter,
    setSeverityFilter,
    setSinceFilter,
    setUntilFilter,
    refreshLogs,
  } = useLogStore();
  const setTextFilter = useLogStore((s) => s.setTextFilter);

  const [searchInput, setSearchInput] = useState("");
  const debounceRef = useRef<ReturnType<typeof setTimeout> | null>(null);

  // Debounced text filter
  useEffect(() => {
    if (debounceRef.current) {
      clearTimeout(debounceRef.current);
    }
    debounceRef.current = setTimeout(() => {
      setTextFilter(searchInput);
    }, 300);
    return () => {
      if (debounceRef.current) {
        clearTimeout(debounceRef.current);
      }
    };
  }, [searchInput, setTextFilter]);

  const handleSinceChange = useCallback(
    (e: React.ChangeEvent<HTMLInputElement>) => {
      const val = e.target.value;
      setSinceFilter(val ? new Date(`${val}T00:00:00Z`).toISOString() : null);
    },
    [setSinceFilter],
  );

  const handleUntilChange = useCallback(
    (e: React.ChangeEvent<HTMLInputElement>) => {
      const val = e.target.value;
      setUntilFilter(val ? new Date(`${val}T23:59:59Z`).toISOString() : null);
    },
    [setUntilFilter],
  );

  return (
    <div className="flex items-center gap-2 px-3 py-2 border-b border-gray-200 dark:border-gray-700 bg-gray-50 dark:bg-gray-800/50 flex-wrap">
      {/* Search input */}
      <input
        type="text"
        placeholder="Search logs..."
        value={searchInput}
        onChange={(e) => setSearchInput(e.target.value)}
        className="px-2 py-1 text-sm rounded border border-gray-300 dark:border-gray-600 bg-white dark:bg-gray-800 text-gray-900 dark:text-gray-100 w-48 focus:outline-none focus:ring-1 focus:ring-accent dark:focus:ring-accent-dark"
      />

      {/* Severity dropdown */}
      <select
        aria-label="Severity"
        value={severityFilter}
        onChange={(e) => setSeverityFilter(e.target.value as "all" | "error" | "warn" | "info")}
        className="px-2 py-1 text-sm rounded border border-gray-300 dark:border-gray-600 bg-white dark:bg-gray-800 text-gray-900 dark:text-gray-100 focus:outline-none focus:ring-1 focus:ring-accent dark:focus:ring-accent-dark"
      >
        <option value="all">All Severities</option>
        <option value="error">Error</option>
        <option value="warn">Warning</option>
        <option value="info">Info</option>
      </select>

      {/* Date range */}
      <label className="flex items-center gap-1 text-xs text-gray-500 dark:text-gray-400">
        From
        <input
          type="date"
          aria-label="From"
          onChange={handleSinceChange}
          className="px-1 py-1 text-sm rounded border border-gray-300 dark:border-gray-600 bg-white dark:bg-gray-800 text-gray-900 dark:text-gray-100"
        />
      </label>
      <label className="flex items-center gap-1 text-xs text-gray-500 dark:text-gray-400">
        To
        <input
          type="date"
          aria-label="To"
          onChange={handleUntilChange}
          className="px-1 py-1 text-sm rounded border border-gray-300 dark:border-gray-600 bg-white dark:bg-gray-800 text-gray-900 dark:text-gray-100"
        />
      </label>

      {/* Refresh button */}
      <button
        type="button"
        onClick={refreshLogs}
        className="px-2 py-1 text-sm rounded bg-gray-200 dark:bg-gray-700 hover:bg-gray-300 dark:hover:bg-gray-600 text-gray-700 dark:text-gray-300 transition-colors"
        aria-label="Refresh"
      >
        Refresh
      </button>

      {/* Entry count */}
      <span className="ml-auto text-xs text-gray-500 dark:text-gray-400">
        {filteredEntries.length === entries.length
          ? `${entries.length} entries`
          : `${filteredEntries.length} of ${entries.length} entries`}
      </span>
    </div>
  );
}
