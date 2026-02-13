import { create } from "zustand";

import type { DebugParams, DebugResult, TestCase } from "@/types";

const STORAGE_KEY = "rulez-test-cases";

function loadTestCases(): TestCase[] {
  try {
    const raw = localStorage.getItem(STORAGE_KEY);
    if (raw) return JSON.parse(raw);
  } catch {
    // Ignore parse errors
  }
  return [];
}

function persistTestCases(testCases: TestCase[]): void {
  try {
    localStorage.setItem(STORAGE_KEY, JSON.stringify(testCases));
  } catch {
    // Ignore storage errors
  }
}

function generateId(): string {
  return `tc-${Date.now()}-${Math.random().toString(36).slice(2, 8)}`;
}

function generateName(params: DebugParams): string {
  const parts: string[] = [params.eventType];
  if (params.tool) parts.push(params.tool);
  if (params.command) parts.push(params.command.slice(0, 30));
  return parts.join(" - ");
}

interface TestCaseState {
  testCases: TestCase[];
  selectedTestCaseId: string | null;
}

interface TestCaseActions {
  saveTestCase: (params: DebugParams, result?: DebugResult) => TestCase;
  loadTestCase: (id: string) => TestCase | undefined;
  deleteTestCase: (id: string) => void;
  clearAll: () => void;
  setSelected: (id: string | null) => void;
}

export const useTestCaseStore = create<TestCaseState & TestCaseActions>((set, get) => ({
  testCases: loadTestCases(),
  selectedTestCaseId: null,

  saveTestCase: (params, result) => {
    const testCase: TestCase = {
      id: generateId(),
      name: generateName(params),
      createdAt: new Date().toISOString(),
      params,
      lastResult: result,
    };
    const updated = [testCase, ...get().testCases];
    persistTestCases(updated);
    set({ testCases: updated });
    return testCase;
  },

  loadTestCase: (id) => {
    return get().testCases.find((tc) => tc.id === id);
  },

  deleteTestCase: (id) => {
    const updated = get().testCases.filter((tc) => tc.id !== id);
    persistTestCases(updated);
    set({
      testCases: updated,
      selectedTestCaseId: get().selectedTestCaseId === id ? null : get().selectedTestCaseId,
    });
  },

  clearAll: () => {
    persistTestCases([]);
    set({ testCases: [], selectedTestCaseId: null });
  },

  setSelected: (id) => {
    set({ selectedTestCaseId: id });
  },
}));
