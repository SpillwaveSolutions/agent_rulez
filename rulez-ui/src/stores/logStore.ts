import { deriveSeverity } from "@/lib/log-utils";
import { readLogs } from "@/lib/tauri";
import type { LogEntryDto, Severity } from "@/types";
import { create } from "zustand";

interface LogState {
  entries: LogEntryDto[];
  filteredEntries: LogEntryDto[];
  totalCount: number;
  isLoading: boolean;
  error: string | null;
  textFilter: string;
  severityFilter: Severity | "all";
  sinceFilter: string | null;
  untilFilter: string | null;
}

interface LogActions {
  loadLogs: () => Promise<void>;
  refreshLogs: () => Promise<void>;
  setTextFilter: (text: string) => void;
  setSeverityFilter: (severity: Severity | "all") => void;
  setSinceFilter: (since: string | null) => void;
  setUntilFilter: (until: string | null) => void;
}

function applyClientFilters(
  entries: LogEntryDto[],
  textFilter: string,
  severityFilter: Severity | "all",
): LogEntryDto[] {
  let result = entries;

  // Apply severity filter
  if (severityFilter !== "all") {
    result = result.filter((e) => deriveSeverity(e) === severityFilter);
  }

  // Apply text filter
  if (textFilter.trim()) {
    const lower = textFilter.toLowerCase();
    result = result.filter(
      (e) =>
        e.eventType.toLowerCase().includes(lower) ||
        e.sessionId.toLowerCase().includes(lower) ||
        e.outcome.toLowerCase().includes(lower) ||
        (e.toolName?.toLowerCase().includes(lower) ?? false) ||
        e.rulesMatched.some((r) => r.toLowerCase().includes(lower)) ||
        (e.responseReason?.toLowerCase().includes(lower) ?? false) ||
        (e.eventDetailCommand?.toLowerCase().includes(lower) ?? false) ||
        (e.eventDetailFilePath?.toLowerCase().includes(lower) ?? false) ||
        (e.decision?.toLowerCase().includes(lower) ?? false),
    );
  }

  return result;
}

export const useLogStore = create<LogState & LogActions>((set, get) => ({
  // State
  entries: [],
  filteredEntries: [],
  totalCount: 0,
  isLoading: false,
  error: null,
  textFilter: "",
  severityFilter: "all",
  sinceFilter: null,
  untilFilter: null,

  // Actions
  loadLogs: async () => {
    const { sinceFilter, untilFilter, textFilter, severityFilter } = get();
    set({ isLoading: true, error: null });
    try {
      const entries = await readLogs({
        since: sinceFilter ?? undefined,
        until: untilFilter ?? undefined,
      });
      const filteredEntries = applyClientFilters(entries, textFilter, severityFilter);
      set({ entries, filteredEntries, totalCount: entries.length, isLoading: false });
    } catch (err) {
      set({
        error: err instanceof Error ? err.message : String(err),
        isLoading: false,
      });
    }
  },

  refreshLogs: async () => {
    await get().loadLogs();
  },

  setTextFilter: (textFilter) => {
    const { entries, severityFilter } = get();
    const filteredEntries = applyClientFilters(entries, textFilter, severityFilter);
    set({ textFilter, filteredEntries });
  },

  setSeverityFilter: (severityFilter) => {
    const { entries, textFilter } = get();
    const filteredEntries = applyClientFilters(entries, textFilter, severityFilter);
    set({ severityFilter, filteredEntries });
  },

  setSinceFilter: (sinceFilter) => {
    set({ sinceFilter });
    get().loadLogs();
  },

  setUntilFilter: (untilFilter) => {
    set({ untilFilter });
    get().loadLogs();
  },
}));
