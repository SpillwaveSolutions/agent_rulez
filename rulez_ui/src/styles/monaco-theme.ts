import type { editor } from "monaco-editor";

export const LIGHT_THEME_NAME = "rulez-light";
export const DARK_THEME_NAME = "rulez-dark";

export const lightTheme: editor.IStandaloneThemeData = {
  base: "vs",
  inherit: true,
  rules: [
    { token: "comment", foreground: "6b7280", fontStyle: "italic" },
    { token: "string", foreground: "059669" },
    { token: "number", foreground: "d97706" },
    { token: "keyword", foreground: "7c3aed" },
    { token: "type", foreground: "2563eb" },
  ],
  colors: {
    "editor.background": "#ffffff",
    "editor.foreground": "#1a1a1a",
    "editor.lineHighlightBackground": "#f5f5f5",
    "editor.selectionBackground": "#bfdbfe",
    "editorLineNumber.foreground": "#9ca3af",
    "editorLineNumber.activeForeground": "#374151",
    "editor.inactiveSelectionBackground": "#e5e7eb",
    "editorGutter.background": "#ffffff",
  },
};

export const darkTheme: editor.IStandaloneThemeData = {
  base: "vs-dark",
  inherit: true,
  rules: [
    { token: "comment", foreground: "6b7280", fontStyle: "italic" },
    { token: "string", foreground: "34d399" },
    { token: "number", foreground: "fbbf24" },
    { token: "keyword", foreground: "a78bfa" },
    { token: "type", foreground: "60a5fa" },
  ],
  colors: {
    "editor.background": "#1a1a1a",
    "editor.foreground": "#f5f5f5",
    "editor.lineHighlightBackground": "#252525",
    "editor.selectionBackground": "#1e3a5f",
    "editorLineNumber.foreground": "#6b7280",
    "editorLineNumber.activeForeground": "#9ca3af",
    "editor.inactiveSelectionBackground": "#374151",
    "editorGutter.background": "#1a1a1a",
  },
};
