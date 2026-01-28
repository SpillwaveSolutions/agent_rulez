import { configureYamlSchema } from "@/lib/schema";
import { useEditorStore } from "@/stores/editorStore";
import { useUIStore } from "@/stores/uiStore";
import { DARK_THEME_NAME, LIGHT_THEME_NAME, darkTheme, lightTheme } from "@/styles/monaco-theme";
import Editor, { type BeforeMount, type OnMount } from "@monaco-editor/react";
import type { MarkerSeverity, Uri, editor } from "monaco-editor";
import { useCallback, useMemo, useRef } from "react";

function useResolvedTheme(): "light" | "dark" {
  const theme = useUIStore((s) => s.theme);
  return useMemo(() => {
    if (theme === "system") {
      return window.matchMedia("(prefers-color-scheme: dark)").matches ? "dark" : "light";
    }
    return theme;
  }, [theme]);
}

interface YamlEditorProps {
  value: string;
  onChange: (value: string) => void;
  onSave?: () => void;
}

export function YamlEditor({ value, onChange, onSave }: YamlEditorProps) {
  const editorRef = useRef<editor.IStandaloneCodeEditor | null>(null);
  const setCursorPosition = useEditorStore((s) => s.setCursorPosition);
  const setSelection = useEditorStore((s) => s.setSelection);
  const setEditorRef = useEditorStore((s) => s.setEditorRef);
  const setValidationResults = useEditorStore((s) => s.setValidationResults);
  const resolvedTheme = useResolvedTheme();

  const monacoThemeName = useMemo(
    () => (resolvedTheme === "dark" ? DARK_THEME_NAME : LIGHT_THEME_NAME),
    [resolvedTheme],
  );

  const schemaConfigured = useRef(false);

  const handleBeforeMount: BeforeMount = useCallback((monaco) => {
    // Define custom themes
    monaco.editor.defineTheme(LIGHT_THEME_NAME, lightTheme);
    monaco.editor.defineTheme(DARK_THEME_NAME, darkTheme);

    // Configure monaco-yaml schema (only once)
    if (!schemaConfigured.current) {
      configureYamlSchema(monaco);
      schemaConfigured.current = true;
    }
  }, []);

  const handleMount: OnMount = useCallback(
    (editorInstance, monaco) => {
      editorRef.current = editorInstance;
      setEditorRef(editorInstance);

      // Cmd/Ctrl+S keybinding
      editorInstance.addCommand(monaco.KeyMod.CtrlCmd | monaco.KeyCode.KeyS, () => {
        onSave?.();
      });

      // Track cursor position
      editorInstance.onDidChangeCursorPosition((e) => {
        setCursorPosition({
          line: e.position.lineNumber,
          column: e.position.column,
        });
      });

      // Track selection
      editorInstance.onDidChangeCursorSelection((e) => {
        const sel = e.selection;
        if (sel.startLineNumber === sel.endLineNumber && sel.startColumn === sel.endColumn) {
          setSelection(null);
        } else {
          setSelection({
            startLine: sel.startLineNumber,
            startColumn: sel.startColumn,
            endLine: sel.endLineNumber,
            endColumn: sel.endColumn,
          });
        }
      });

      // Subscribe to marker changes (validation errors from monaco-yaml)
      const model = editorInstance.getModel();
      if (model) {
        monaco.editor.onDidChangeMarkers((uris: readonly Uri[]) => {
          const modelUri = model.uri.toString();
          if (uris.some((uri: Uri) => uri.toString() === modelUri)) {
            const markers = monaco.editor.getModelMarkers({ resource: model.uri });
            const errors = markers
              .filter(
                (m: editor.IMarker) =>
                  m.severity === (monaco.MarkerSeverity.Error as MarkerSeverity),
              )
              .map((m: editor.IMarker) => ({
                line: m.startLineNumber,
                column: m.startColumn,
                message: m.message,
                severity: "error" as const,
              }));
            const warnings = markers
              .filter(
                (m: editor.IMarker) =>
                  m.severity === (monaco.MarkerSeverity.Warning as MarkerSeverity),
              )
              .map((m: editor.IMarker) => ({
                line: m.startLineNumber,
                column: m.startColumn,
                message: m.message,
                severity: "warning" as const,
              }));
            setValidationResults(errors, warnings);
          }
        });
      }

      // Focus editor on mount
      editorInstance.focus();
    },
    [onSave, setCursorPosition, setSelection, setEditorRef, setValidationResults],
  );

  const handleChange = useCallback(
    (val: string | undefined) => {
      onChange(val ?? "");
    },
    [onChange],
  );

  return (
    <Editor
      height="100%"
      language="yaml"
      value={value}
      onChange={handleChange}
      beforeMount={handleBeforeMount}
      onMount={handleMount}
      theme={monacoThemeName}
      options={{
        minimap: { enabled: false },
        wordWrap: "off",
        tabSize: 2,
        autoIndent: "full",
        folding: true,
        fontSize: 14,
        lineNumbers: "on",
        renderLineHighlight: "line",
        scrollBeyondLastLine: false,
        automaticLayout: true,
        padding: { top: 8, bottom: 8 },
      }}
    />
  );
}
