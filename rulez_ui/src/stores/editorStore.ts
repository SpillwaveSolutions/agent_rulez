import type { CursorPosition, SelectionRange, ValidationError, ValidationWarning } from "@/types";
import type { editor } from "monaco-editor";
import { create } from "zustand";

interface EditorState {
  cursorPosition: CursorPosition;
  selection: SelectionRange | null;
  errors: ValidationError[];
  warnings: ValidationWarning[];
  isValidating: boolean;
  editorRef: editor.IStandaloneCodeEditor | null;
}

interface EditorActions {
  setCursorPosition: (position: CursorPosition) => void;
  setSelection: (selection: SelectionRange | null) => void;
  setErrors: (errors: ValidationError[]) => void;
  setWarnings: (warnings: ValidationWarning[]) => void;
  setValidationResults: (errors: ValidationError[], warnings: ValidationWarning[]) => void;
  clearValidation: () => void;
  setIsValidating: (isValidating: boolean) => void;
  setEditorRef: (ref: editor.IStandaloneCodeEditor | null) => void;
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
  editorRef: null,

  // Actions
  setCursorPosition: (cursorPosition) => set({ cursorPosition }),

  setSelection: (selection) => set({ selection }),

  setErrors: (errors) => set({ errors }),

  setWarnings: (warnings) => set({ warnings }),

  setValidationResults: (errors, warnings) => set({ errors, warnings, isValidating: false }),

  clearValidation: () => set({ errors: [], warnings: [], isValidating: false }),

  setIsValidating: (isValidating) => set({ isValidating }),

  setEditorRef: (editorRef) => set({ editorRef }),

  getErrorCount: () => get().errors.length,

  getWarningCount: () => get().warnings.length,
}));
