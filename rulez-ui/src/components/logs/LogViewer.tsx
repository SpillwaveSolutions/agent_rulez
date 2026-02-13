import { isTauri } from "@/lib/tauri";
import { useLogStore } from "@/stores/logStore";
import type { LogEntryDto } from "@/types";
import { useVirtualizer } from "@tanstack/react-virtual";
import { useCallback, useEffect, useRef } from "react";
import { LogEntryRow } from "./LogEntryRow";
import { LogFilterBar } from "./LogFilterBar";

const ROW_HEIGHT = 36;
const OVERSCAN = 20;

export function LogViewer() {
  const { filteredEntries, isLoading, error, loadLogs } = useLogStore();
  const parentRef = useRef<HTMLDivElement>(null);

  useEffect(() => {
    loadLogs();
  }, [loadLogs]);

  const rowVirtualizer = useVirtualizer({
    count: filteredEntries.length,
    getScrollElement: () => parentRef.current,
    estimateSize: () => ROW_HEIGHT,
    overscan: OVERSCAN,
  });

  const handleCopy = useCallback(async (entry: LogEntryDto) => {
    const text = JSON.stringify(entry, null, 2);
    if (isTauri()) {
      try {
        const { writeText } = await import("@tauri-apps/plugin-clipboard-manager");
        await writeText(text);
      } catch {
        // Fallback to navigator
        await navigator.clipboard.writeText(text);
      }
    } else {
      await navigator.clipboard.writeText(text);
    }
  }, []);

  return (
    <div className="h-full flex flex-col bg-white dark:bg-[#1A1A1A]">
      {/* Filter bar */}
      <LogFilterBar />

      {/* Column headers */}
      <div className="flex items-center gap-2 px-3 py-1 text-xs font-medium text-gray-500 dark:text-gray-400 border-b border-gray-200 dark:border-gray-700 bg-gray-100 dark:bg-gray-800">
        <span className="w-24 shrink-0">Time</span>
        <span className="w-14 shrink-0 text-center">Severity</span>
        <span className="w-28 shrink-0">Event</span>
        <span className="w-16 shrink-0">Tool</span>
        <span className="w-14 shrink-0">Outcome</span>
        <span className="flex-1">Details</span>
        <span className="w-14 shrink-0 text-center">Actions</span>
      </div>

      {/* Scrollable log list */}
      <div ref={parentRef} className="flex-1 overflow-y-auto">
        {isLoading && (
          <div className="flex items-center justify-center py-12 text-gray-400 dark:text-gray-500">
            <div className="text-center">
              <div className="animate-spin w-8 h-8 border-2 border-gray-300 border-t-accent rounded-full mx-auto mb-2" />
              <p>Loading logs...</p>
            </div>
          </div>
        )}

        {error && (
          <div className="flex items-center justify-center py-12 text-red-500">
            <div className="text-center">
              <p className="font-medium">Error loading logs</p>
              <p className="text-sm mt-1">{error}</p>
            </div>
          </div>
        )}

        {!isLoading && !error && filteredEntries.length === 0 && (
          <div className="flex items-center justify-center py-12 text-gray-400 dark:text-gray-500">
            <div className="text-center">
              <svg
                className="w-10 h-10 mx-auto mb-2 opacity-50"
                fill="none"
                stroke="currentColor"
                viewBox="0 0 24 24"
                aria-hidden="true"
              >
                <path
                  strokeLinecap="round"
                  strokeLinejoin="round"
                  strokeWidth={1.5}
                  d="M9 12h6m-6 4h6m2 5H7a2 2 0 01-2-2V5a2 2 0 012-2h5.586a1 1 0 01.707.293l5.414 5.414a1 1 0 01.293.707V19a2 2 0 01-2 2z"
                />
              </svg>
              <p>No log entries found</p>
            </div>
          </div>
        )}

        {!isLoading && !error && filteredEntries.length > 0 && (
          <div
            style={{
              height: `${rowVirtualizer.getTotalSize()}px`,
              position: "relative",
              width: "100%",
            }}
          >
            {rowVirtualizer.getVirtualItems().map((virtualItem) => {
              const entry = filteredEntries[virtualItem.index];
              if (!entry) return null;
              return (
                <LogEntryRow
                  key={virtualItem.key}
                  entry={entry}
                  onCopy={handleCopy}
                  style={{
                    position: "absolute",
                    top: 0,
                    left: 0,
                    width: "100%",
                    height: `${virtualItem.size}px`,
                    transform: `translateY(${virtualItem.start}px)`,
                  }}
                />
              );
            })}
          </div>
        )}
      </div>
    </div>
  );
}
