import { useEditorStore } from "@/stores/editorStore";
import { useState } from "react";

export function EditorToolbar() {
  const editorRef = useEditorStore((s) => s.editorRef);
  const [wordWrap, setWordWrap] = useState(false);
  const [minimapEnabled, setMinimapEnabled] = useState(false);

  const handleUndo = () => {
    editorRef?.trigger("toolbar", "undo", null);
  };

  const handleRedo = () => {
    editorRef?.trigger("toolbar", "redo", null);
  };

  const handleFormat = () => {
    editorRef?.trigger("toolbar", "editor.action.formatDocument", null);
  };

  const handleToggleWordWrap = () => {
    const next = !wordWrap;
    setWordWrap(next);
    editorRef?.updateOptions({ wordWrap: next ? "on" : "off" });
  };

  const handleToggleMinimap = () => {
    const next = !minimapEnabled;
    setMinimapEnabled(next);
    editorRef?.updateOptions({ minimap: { enabled: next } });
  };

  return (
    <div
      data-testid="editor-toolbar"
      className="flex items-center gap-1 px-2 py-1 border-b border-gray-200 dark:border-gray-700 bg-gray-50 dark:bg-[#252525]"
    >
      <ToolbarButton onClick={handleUndo} title="Undo (Ctrl+Z)">
        <UndoIcon />
      </ToolbarButton>
      <ToolbarButton onClick={handleRedo} title="Redo (Ctrl+Shift+Z)">
        <RedoIcon />
      </ToolbarButton>

      <div className="w-px h-5 mx-1 bg-gray-300 dark:bg-gray-600" />

      <ToolbarButton onClick={handleFormat} title="Format Document">
        <FormatIcon />
      </ToolbarButton>

      <div className="w-px h-5 mx-1 bg-gray-300 dark:bg-gray-600" />

      <ToolbarButton onClick={handleToggleWordWrap} title="Toggle Word Wrap" active={wordWrap}>
        <WordWrapIcon />
      </ToolbarButton>
      <ToolbarButton onClick={handleToggleMinimap} title="Toggle Minimap" active={minimapEnabled}>
        <MinimapIcon />
      </ToolbarButton>
    </div>
  );
}

function ToolbarButton({
  onClick,
  title,
  active,
  children,
}: {
  onClick: () => void;
  title: string;
  active?: boolean;
  children: React.ReactNode;
}) {
  return (
    <button
      type="button"
      onClick={onClick}
      title={title}
      className={`p-1.5 rounded transition-colors ${
        active
          ? "bg-blue-100 dark:bg-blue-900/40 text-blue-600 dark:text-blue-400"
          : "text-gray-500 dark:text-gray-400 hover:bg-gray-200 dark:hover:bg-gray-700 hover:text-gray-700 dark:hover:text-gray-200"
      }`}
    >
      {children}
    </button>
  );
}

function UndoIcon() {
  return (
    <svg
      width="16"
      height="16"
      viewBox="0 0 24 24"
      fill="none"
      stroke="currentColor"
      strokeWidth="2"
      strokeLinecap="round"
      strokeLinejoin="round"
      role="img"
      aria-label="Undo"
    >
      <path d="M3 7v6h6" />
      <path d="M21 17a9 9 0 0 0-9-9 9 9 0 0 0-6 2.3L3 13" />
    </svg>
  );
}

function RedoIcon() {
  return (
    <svg
      width="16"
      height="16"
      viewBox="0 0 24 24"
      fill="none"
      stroke="currentColor"
      strokeWidth="2"
      strokeLinecap="round"
      strokeLinejoin="round"
      role="img"
      aria-label="Redo"
    >
      <path d="M21 7v6h-6" />
      <path d="M3 17a9 9 0 0 1 9-9 9 9 0 0 1 6 2.3L21 13" />
    </svg>
  );
}

function FormatIcon() {
  return (
    <svg
      width="16"
      height="16"
      viewBox="0 0 24 24"
      fill="none"
      stroke="currentColor"
      strokeWidth="2"
      strokeLinecap="round"
      strokeLinejoin="round"
      role="img"
      aria-label="Format"
    >
      <path d="M4 7h16" />
      <path d="M4 12h10" />
      <path d="M4 17h16" />
    </svg>
  );
}

function WordWrapIcon() {
  return (
    <svg
      width="16"
      height="16"
      viewBox="0 0 24 24"
      fill="none"
      stroke="currentColor"
      strokeWidth="2"
      strokeLinecap="round"
      strokeLinejoin="round"
      role="img"
      aria-label="Word wrap"
    >
      <path d="M3 6h18" />
      <path d="M3 12h15a3 3 0 1 1 0 6h-4" />
      <path d="m16 16-2 2 2 2" />
      <path d="M3 18h7" />
    </svg>
  );
}

function MinimapIcon() {
  return (
    <svg
      width="16"
      height="16"
      viewBox="0 0 24 24"
      fill="none"
      stroke="currentColor"
      strokeWidth="2"
      strokeLinecap="round"
      strokeLinejoin="round"
      role="img"
      aria-label="Minimap"
    >
      <rect x="2" y="3" width="20" height="18" rx="2" />
      <rect x="15" y="5" width="5" height="14" rx="1" opacity="0.5" />
    </svg>
  );
}
