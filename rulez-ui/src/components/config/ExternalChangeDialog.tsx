interface ExternalChangeDialogProps {
  filePath: string;
  onReload: () => void;
  onKeepMine: () => void;
}

/**
 * Dialog shown when a file with unsaved editor changes is modified externally.
 * User can choose to reload from disk or keep their editor content.
 */
export function ExternalChangeDialog({
  filePath,
  onReload,
  onKeepMine,
}: ExternalChangeDialogProps) {
  const fileName = filePath.split("/").pop() ?? filePath;

  return (
    <div className="fixed inset-0 z-50 flex items-center justify-center bg-black/40">
      <div className="bg-white dark:bg-gray-800 rounded-lg shadow-xl w-96">
        {/* Header */}
        <div className="px-4 py-3 border-b border-gray-200 dark:border-gray-700">
          <h3 className="text-sm font-semibold text-gray-900 dark:text-gray-100">
            File Changed on Disk
          </h3>
        </div>

        {/* Message */}
        <div className="px-4 py-4">
          <p className="text-sm text-gray-700 dark:text-gray-300">
            The file <span className="font-mono font-medium">{fileName}</span> has been modified
            outside RuleZ UI. You have unsaved changes.
          </p>
        </div>

        {/* Actions */}
        <div className="px-4 py-3 border-t border-gray-200 dark:border-gray-700 flex items-center justify-end gap-2">
          <button
            type="button"
            onClick={onKeepMine}
            className="px-3 py-1.5 text-sm rounded border border-gray-300 dark:border-gray-600 text-gray-700 dark:text-gray-300 hover:bg-gray-100 dark:hover:bg-gray-700 transition-colors"
          >
            Keep Mine
          </button>
          <button
            type="button"
            onClick={onReload}
            className="px-3 py-1.5 text-sm rounded bg-blue-600 text-white hover:bg-blue-700 transition-colors"
          >
            Reload
          </button>
        </div>
      </div>
    </div>
  );
}
