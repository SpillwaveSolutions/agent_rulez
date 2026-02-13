import { configureYamlSchema } from "@/lib/schema";
import { registerYamlFormatter } from "@/lib/yaml-formatter";
import { useEditorStore } from "@/stores/editorStore";
import { useSettingsStore } from "@/stores/settingsStore";
import { DARK_THEME_NAME, LIGHT_THEME_NAME, darkTheme, lightTheme } from "@/styles/monaco-theme";
import Editor, { type BeforeMount, type OnMount } from "@monaco-editor/react";
import type { IDisposable, MarkerSeverity, Uri, editor } from "monaco-editor";
import { useCallback, useEffect, useMemo, useRef } from "react";

function useResolvedTheme(): "light" | "dark" {
  const theme = useSettingsStore((s) => s.settings.theme);
  return useMemo(() => {
    if (theme === "system") {
      return window.matchMedia("(prefers-color-scheme: dark)").matches ? "dark" : "light";
    }
    return theme;
  }, [theme]);
}

interface YamlEditorProps {
  value: string;
  path?: string;
  onChange: (value: string) => void;
  onSave?: () => void;
}

export function YamlEditor({ value, path, onChange, onSave }: YamlEditorProps) {
  const editorRef = useRef<editor.IStandaloneCodeEditor | null>(null);
  const setCursorPosition = useEditorStore((s) => s.setCursorPosition);
  const setSelection = useEditorStore((s) => s.setSelection);
  const setEditorRef = useEditorStore((s) => s.setEditorRef);
  const setValidationResults = useEditorStore((s) => s.setValidationResults);
  const resolvedTheme = useResolvedTheme();
  const editorFontSize = useSettingsStore((s) => s.settings.editorFontSize);
  const editorTabSize = useSettingsStore((s) => s.settings.editorTabSize);

  const monacoThemeName = useMemo(
    () => (resolvedTheme === "dark" ? DARK_THEME_NAME : LIGHT_THEME_NAME),
    [resolvedTheme],
  );

  const schemaConfigured = useRef(false);
  const yamlConfigDisposableRef = useRef<IDisposable | null>(null);
  const formatterDisposableRef = useRef<IDisposable | null>(null);
  const disposablesRef = useRef<IDisposable[]>([]);

  const handleBeforeMount: BeforeMount = useCallback((monaco) => {
    // Define custom themes
    monaco.editor.defineTheme(LIGHT_THEME_NAME, lightTheme);
    monaco.editor.defineTheme(DARK_THEME_NAME, darkTheme);

    // Configure monaco-yaml schema and formatter (only once)
    if (!schemaConfigured.current) {
      yamlConfigDisposableRef.current = configureYamlSchema(monaco);
      formatterDisposableRef.current = registerYamlFormatter(monaco);
      schemaConfigured.current = true;
    }
  }, []);

  const handleMount: OnMount = useCallback(
    (editorInstance, monaco) => {
      // Dispose any existing listeners (handles React Strict Mode double-mount)
      for (const d of disposablesRef.current) {
        d.dispose();
      }
      disposablesRef.current = [];

      editorRef.current = editorInstance;
      setEditorRef(editorInstance);

      // Cmd/Ctrl+S keybinding
      editorInstance.addCommand(monaco.KeyMod.CtrlCmd | monaco.KeyCode.KeyS, () => {
        onSave?.();
      });

      // Cmd/Ctrl+Shift+I keybinding for format
      editorInstance.addCommand(
        monaco.KeyMod.CtrlCmd | monaco.KeyMod.Shift | monaco.KeyCode.KeyI,
        () => {
          editorInstance.getAction("editor.action.formatDocument")?.run();
        },
      );

      // Track cursor position
      const cursorDisposable = editorInstance.onDidChangeCursorPosition((e) => {
        setCursorPosition({
          line: e.position.lineNumber,
          column: e.position.column,
        });
      });
      disposablesRef.current.push(cursorDisposable);

      // Track selection
      const selectionDisposable = editorInstance.onDidChangeCursorSelection((e) => {
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
      disposablesRef.current.push(selectionDisposable);

      // Subscribe to marker changes (validation errors from monaco-yaml)
      const model = editorInstance.getModel();
      if (model) {
        const markerDisposable = monaco.editor.onDidChangeMarkers((uris: readonly Uri[]) => {
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
        disposablesRef.current.push(markerDisposable);
      }

      // Focus editor on mount
      editorInstance.focus();
    },
    [onSave, setCursorPosition, setSelection, setEditorRef, setValidationResults],
  );

  // Cleanup all disposables on unmount
  useEffect(() => {
    return () => {
      for (const d of disposablesRef.current) {
        d.dispose();
      }
      disposablesRef.current = [];
      formatterDisposableRef.current?.dispose();
      formatterDisposableRef.current = null;
      setEditorRef(null);
    };
  }, [setEditorRef]);

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
      path={path}
      value={value}
      onChange={handleChange}
      beforeMount={handleBeforeMount}
      onMount={handleMount}
      theme={monacoThemeName}
      options={{
        minimap: { enabled: false },
        wordWrap: "off",
        tabSize: editorTabSize,
        autoIndent: "full",
        folding: true,
        fontSize: editorFontSize,
        lineNumbers: "on",
        renderLineHighlight: "line",
        scrollBeyondLastLine: false,
        automaticLayout: true,
        padding: { top: 8, bottom: 8 },
      }}
    />
  );
}
