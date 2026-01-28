import { create } from "zustand";
import type { ValidationError, ValidationWarning, CursorPosition, SelectionRange } from "@/types";

interface EditorState {
  cursorPosition: CursorPosition;
  selection: SelectionRange | null;
  errors: ValidationError[];
  warnings: ValidationWarning[];
  isValidating: boolean;
}

interface EditorActions {
  setCursorPosition: (position: CursorPosition) => void;
  setSelection: (selection: SelectionRange | null) => void;
  setErrors: (errors: ValidationError[]) => void;
  setWarnings: (warnings: ValidationWarning[]) => void;
  setValidationResults: (errors: ValidationError[], warnings: ValidationWarning[]) => void;
  clearValidation: () => void;
  setIsValidating: (isValidating: boolean) => void;
  getErrorCount: () => number;
  getWarningCount: () => number;
}

export const useEditorStore = create<EditorState & EditorActions>((set, get) => ({
  // State
  cursorPosition: { line: 1, column: 1 },
  selection: null,
  errors: [],
  warnings: [],
  isValidating: false,

  // Actions
  setCursorPosition: (cursorPosition) => set({ cursorPosition }),

  setSelection: (selection) => set({ selection }),

  setErrors: (errors) => set({ errors }),

  setWarnings: (warnings) => set({ warnings }),

  setValidationResults: (errors, warnings) => set({ errors, warnings, isValidating: false }),

  clearValidation: () => set({ errors: [], warnings: [], isValidating: false }),

  setIsValidating: (isValidating) => set({ isValidating }),

  getErrorCount: () => get().errors.length,

  getWarningCount: () => get().warnings.length,
}));
