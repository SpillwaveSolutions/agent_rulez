import { useEffect } from "react";

interface ConfirmDialogProps {
  isOpen: boolean;
  title: string;
  message: string;
  onSave: () => void;
  onDiscard: () => void;
  onCancel: () => void;
}

export function ConfirmDialog({
  isOpen,
  title,
  message,
  onSave,
  onDiscard,
  onCancel,
}: ConfirmDialogProps) {
  useEffect(() => {
    if (!isOpen) return;

    const handleKeyDown = (e: KeyboardEvent) => {
      if (e.key === "Escape") {
        onCancel();
      }
    };

    document.addEventListener("keydown", handleKeyDown);
    return () => document.removeEventListener("keydown", handleKeyDown);
  }, [isOpen, onCancel]);

  if (!isOpen) return null;

  return (
    <div className="fixed inset-0 z-50 flex items-center justify-center">
      {/* Overlay */}
      <div className="absolute inset-0 bg-black/50" />

      {/* Dialog */}
      <div className="relative z-10 w-full max-w-md rounded-lg bg-white dark:bg-[#252525] shadow-xl border border-gray-200 dark:border-gray-700 p-6">
        <h2 className="text-lg font-bold text-gray-900 dark:text-gray-100">{title}</h2>
        <p className="mt-2 text-sm text-gray-600 dark:text-gray-400">{message}</p>

        <div className="mt-6 flex items-center justify-end gap-3">
          <button
            type="button"
            onClick={onCancel}
            className="px-4 py-2 text-sm font-medium rounded border border-gray-300 dark:border-gray-600 text-gray-700 dark:text-gray-300 bg-transparent hover:bg-gray-100 dark:hover:bg-gray-800 transition-colors"
          >
            Cancel
          </button>
          <button
            type="button"
            onClick={onDiscard}
            className="px-4 py-2 text-sm font-medium rounded text-white bg-red-500 hover:bg-red-600 dark:bg-red-600 dark:hover:bg-red-700 transition-colors"
          >
            Don&apos;t Save
          </button>
          <button
            type="button"
            onClick={onSave}
            className="px-4 py-2 text-sm font-medium rounded text-white bg-accent hover:bg-accent/90 dark:bg-accent-dark dark:hover:bg-accent-dark/90 transition-colors"
          >
            Save
          </button>
        </div>
      </div>
    </div>
  );
}
