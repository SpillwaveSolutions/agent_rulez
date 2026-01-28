import type { Rule } from "@/types";

interface RuleCardProps {
  rule: Rule;
  lineNumber?: number;
  onNavigate?: (line: number) => void;
}

export function RuleCard({ rule, lineNumber, onNavigate }: RuleCardProps) {
  const isDisabled = rule.enabled === false;

  const handleClick = () => {
    if (lineNumber !== undefined && onNavigate) {
      onNavigate(lineNumber);
    }
  };

  return (
    <button
      type="button"
      onClick={handleClick}
      className={`w-full text-left p-3 rounded-lg border transition-colors cursor-pointer ${
        isDisabled
          ? "border-gray-200 dark:border-gray-700 bg-gray-50 dark:bg-gray-800/50 opacity-60"
          : "border-gray-200 dark:border-gray-700 bg-white dark:bg-[#1A1A1A] hover:border-accent/50 dark:hover:border-accent-dark/50 hover:bg-gray-50 dark:hover:bg-gray-800"
      }`}
    >
      {/* Header: name + action badge */}
      <div className="flex items-center justify-between gap-2 mb-1">
        <span
          className={`text-sm font-semibold truncate ${
            isDisabled ? "text-gray-400 dark:text-gray-500" : "text-gray-900 dark:text-gray-100"
          }`}
        >
          {rule.name}
        </span>
        <ActionBadge actions={rule.actions} />
      </div>

      {/* Description */}
      {rule.description && (
        <p className="text-xs text-gray-500 dark:text-gray-400 mb-2 line-clamp-2">
          {rule.description}
        </p>
      )}

      {/* Tools badges */}
      {rule.matchers.tools && rule.matchers.tools.length > 0 && (
        <div className="flex flex-wrap gap-1">
          {rule.matchers.tools.map((tool) => (
            <span
              key={tool}
              className="inline-block px-1.5 py-0.5 text-[10px] font-medium rounded bg-gray-100 dark:bg-gray-700 text-gray-600 dark:text-gray-300"
            >
              {tool}
            </span>
          ))}
        </div>
      )}

      {/* Disabled indicator */}
      {isDisabled && (
        <span className="inline-block mt-1.5 text-[10px] font-medium text-gray-400 dark:text-gray-500 uppercase tracking-wider">
          Disabled
        </span>
      )}
    </button>
  );
}

function ActionBadge({ actions }: { actions: Rule["actions"] }) {
  if (actions.block) {
    return (
      <span className="inline-block px-1.5 py-0.5 text-[10px] font-semibold rounded bg-red-100 dark:bg-red-900/30 text-red-700 dark:text-red-400">
        Block
      </span>
    );
  }

  if (actions.inject) {
    return (
      <span className="inline-block px-1.5 py-0.5 text-[10px] font-semibold rounded bg-blue-100 dark:bg-blue-900/30 text-blue-700 dark:text-blue-400">
        Inject
      </span>
    );
  }

  if (actions.run) {
    return (
      <span className="inline-block px-1.5 py-0.5 text-[10px] font-semibold rounded bg-green-100 dark:bg-green-900/30 text-green-700 dark:text-green-400">
        Run
      </span>
    );
  }

  if (actions.block_if_match) {
    return (
      <span className="inline-block px-1.5 py-0.5 text-[10px] font-semibold rounded bg-orange-100 dark:bg-orange-900/30 text-orange-700 dark:text-orange-400">
        Block If
      </span>
    );
  }

  return null;
}
