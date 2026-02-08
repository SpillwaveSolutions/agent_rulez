import { RuleCard } from "@/components/editor/RuleCard";
import { parseYamlWithPositions } from "@/lib/yaml-utils";
import { useConfigStore } from "@/stores/configStore";
import { useEditorStore } from "@/stores/editorStore";
import { useMemo, useState } from "react";

export function RuleTreeView() {
  const activeContent = useConfigStore((s) => s.getActiveContent());
  const [settingsExpanded, setSettingsExpanded] = useState(true);
  const [rulesExpanded, setRulesExpanded] = useState(true);

  const parsed = useMemo(() => {
    if (!activeContent) return null;
    return parseYamlWithPositions(activeContent);
  }, [activeContent]);

  const handleNavigate = (line: number) => {
    const store = useEditorStore.getState();
    const editorRef = (store as unknown as Record<string, unknown>).editorRef as
      | {
          revealLineInCenter: (line: number) => void;
          setPosition: (pos: { lineNumber: number; column: number }) => void;
        }
      | undefined;
    if (editorRef) {
      editorRef.revealLineInCenter(line);
      editorRef.setPosition({ lineNumber: line, column: 1 });
    }
  };

  if (!activeContent) {
    return (
      <div className="space-y-4">
        <h3 className="text-sm font-semibold text-gray-700 dark:text-gray-300">Rule Tree</h3>
        <p className="text-xs text-gray-500 dark:text-gray-400">
          Open a configuration file to view the rule tree.
        </p>
      </div>
    );
  }

  if (!parsed) {
    return (
      <div className="space-y-4">
        <h3 className="text-sm font-semibold text-gray-700 dark:text-gray-300">Rule Tree</h3>
        <p className="text-xs text-red-500 dark:text-red-400">
          Unable to parse YAML. Fix syntax errors to see the rule tree.
        </p>
      </div>
    );
  }

  const { config, linePositions } = parsed;
  const rules = config.rules ?? [];
  const settings = config.settings;

  return (
    <div className="space-y-3">
      <h3 className="text-sm font-semibold text-gray-700 dark:text-gray-300">Rule Tree</h3>

      {/* Settings section */}
      <CollapsibleSection
        title="Settings"
        expanded={settingsExpanded}
        onToggle={() => setSettingsExpanded((v) => !v)}
      >
        {settings ? (
          <div className="space-y-1 text-xs text-gray-600 dark:text-gray-400">
            {settings.log_level !== undefined && (
              <div className="flex justify-between">
                <span>log_level</span>
                <span className="font-mono text-gray-800 dark:text-gray-200">
                  {settings.log_level}
                </span>
              </div>
            )}
            {settings.fail_open !== undefined && (
              <div className="flex justify-between">
                <span>fail_open</span>
                <span className="font-mono text-gray-800 dark:text-gray-200">
                  {String(settings.fail_open)}
                </span>
              </div>
            )}
            {settings.max_context_size !== undefined && (
              <div className="flex justify-between">
                <span>max_context_size</span>
                <span className="font-mono text-gray-800 dark:text-gray-200">
                  {settings.max_context_size}
                </span>
              </div>
            )}
            {!settings.log_level &&
              settings.fail_open === undefined &&
              !settings.max_context_size && (
                <p className="text-gray-400 dark:text-gray-500 italic">No settings defined</p>
              )}
          </div>
        ) : (
          <p className="text-xs text-gray-400 dark:text-gray-500 italic">No settings defined</p>
        )}
      </CollapsibleSection>

      {/* Rules section */}
      <CollapsibleSection
        title={`Rules (${rules.length})`}
        expanded={rulesExpanded}
        onToggle={() => setRulesExpanded((v) => !v)}
      >
        {rules.length > 0 ? (
          <div className="space-y-2">
            {rules.map((rule, index) => (
              <RuleCard
                key={`${rule.name}-${index}`}
                rule={rule}
                lineNumber={linePositions.get(rule.name)}
                onNavigate={handleNavigate}
              />
            ))}
          </div>
        ) : (
          <p className="text-xs text-gray-400 dark:text-gray-500 italic">No rules defined</p>
        )}
      </CollapsibleSection>
    </div>
  );
}

interface CollapsibleSectionProps {
  title: string;
  expanded: boolean;
  onToggle: () => void;
  children: React.ReactNode;
}

function CollapsibleSection({ title, expanded, onToggle, children }: CollapsibleSectionProps) {
  return (
    <div>
      <button
        type="button"
        onClick={onToggle}
        className="flex items-center gap-1.5 w-full text-left text-sm font-medium text-gray-600 dark:text-gray-400 hover:text-gray-900 dark:hover:text-gray-200 transition-colors"
      >
        <svg
          className={`w-3.5 h-3.5 transition-transform ${expanded ? "rotate-90" : ""}`}
          fill="none"
          stroke="currentColor"
          viewBox="0 0 24 24"
          aria-hidden="true"
        >
          <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M9 5l7 7-7 7" />
        </svg>
        <span>{title}</span>
      </button>
      {expanded && <div className="mt-2 ml-5">{children}</div>}
    </div>
  );
}
