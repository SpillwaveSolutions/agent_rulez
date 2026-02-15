import { entriesToCsv, entriesToJson } from "@/lib/log-utils";
import { isTauri } from "@/lib/tauri";
import { useLogStore } from "@/stores/logStore";
import { useCallback, useEffect, useRef, useState } from "react";

type ExportFormat = "json" | "csv";

/**
 * Export menu for downloading filtered log entries as JSON or CSV.
 * Uses Tauri native save dialog in desktop mode, browser download fallback in web mode.
 */
export function LogExportMenu() {
  const filteredEntries = useLogStore((s) => s.filteredEntries);
  const [open, setOpen] = useState(false);
  const [feedback, setFeedback] = useState<string | null>(null);
  const menuRef = useRef<HTMLDivElement>(null);

  // Close dropdown on outside click
  useEffect(() => {
    if (!open) return;
    function handleClick(e: MouseEvent) {
      if (menuRef.current && !menuRef.current.contains(e.target as Node)) {
        setOpen(false);
      }
    }
    document.addEventListener("mousedown", handleClick);
    return () => document.removeEventListener("mousedown", handleClick);
  }, [open]);

  const handleExport = useCallback(
    async (format: ExportFormat) => {
      setOpen(false);

      const ext = format;
      const content =
        format === "json" ? entriesToJson(filteredEntries) : entriesToCsv(filteredEntries);

      if (isTauri()) {
        try {
          const { save } = await import("@tauri-apps/plugin-dialog");
          const { writeTextFile } = await import("@tauri-apps/plugin-fs");
          const path = await save({
            filters: [{ name: format.toUpperCase(), extensions: [ext] }],
            defaultPath: `rulez-logs.${ext}`,
          });
          if (path) {
            await writeTextFile(path, content);
            setFeedback("Exported!");
          }
        } catch (err) {
          console.error("Export failed:", err);
          // Fallback to browser download
          browserDownload(
            content,
            `rulez-logs.${ext}`,
            format === "json" ? "application/json" : "text/csv",
          );
          setFeedback("Exported!");
        }
      } else {
        browserDownload(
          content,
          `rulez-logs.${ext}`,
          format === "json" ? "application/json" : "text/csv",
        );
        setFeedback("Exported!");
      }

      setTimeout(() => setFeedback(null), 2000);
    },
    [filteredEntries],
  );

  return (
    <div ref={menuRef} className="relative">
      <button
        type="button"
        onClick={() => setOpen((prev) => !prev)}
        className="px-2 py-1 text-sm rounded bg-gray-200 dark:bg-gray-700 hover:bg-gray-300 dark:hover:bg-gray-600 text-gray-700 dark:text-gray-300 transition-colors"
        aria-label="Export"
        aria-haspopup="true"
        aria-expanded={open}
      >
        {feedback ?? "Export"}
      </button>

      {open && (
        <div
          role="menu"
          className="absolute top-full left-0 mt-1 bg-white dark:bg-gray-800 border border-gray-200 dark:border-gray-700 rounded shadow-lg z-50 min-w-[140px]"
        >
          <button
            type="button"
            role="menuitem"
            onClick={() => handleExport("json")}
            className="w-full text-left px-3 py-1.5 text-sm text-gray-700 dark:text-gray-300 hover:bg-gray-100 dark:hover:bg-gray-700 transition-colors"
          >
            Export as JSON
          </button>
          <button
            type="button"
            role="menuitem"
            onClick={() => handleExport("csv")}
            className="w-full text-left px-3 py-1.5 text-sm text-gray-700 dark:text-gray-300 hover:bg-gray-100 dark:hover:bg-gray-700 transition-colors"
          >
            Export as CSV
          </button>
        </div>
      )}
    </div>
  );
}

/** Browser-based file download fallback */
function browserDownload(content: string, filename: string, mimeType: string): void {
  const blob = new Blob([content], { type: mimeType });
  const url = URL.createObjectURL(blob);
  const a = document.createElement("a");
  a.href = url;
  a.download = filename;
  document.body.appendChild(a);
  a.click();
  document.body.removeChild(a);
  URL.revokeObjectURL(url);
}
