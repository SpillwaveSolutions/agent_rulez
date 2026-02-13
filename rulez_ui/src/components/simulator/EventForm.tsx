import type { DebugParams, EventType } from "@/types";
import { forwardRef, useImperativeHandle, useState } from "react";

const EVENT_TYPES: EventType[] = [
  "PreToolUse",
  "PostToolUse",
  "PermissionRequest",
  "UserPromptSubmit",
  "SessionStart",
  "SessionEnd",
  "PreCompact",
];

interface EventFormProps {
  onSubmit: (params: DebugParams) => void;
  isLoading: boolean;
}

export interface EventFormHandle {
  setParams: (params: DebugParams) => void;
}

export const EventForm = forwardRef<EventFormHandle, EventFormProps>(function EventForm(
  { onSubmit, isLoading },
  ref,
) {
  const [eventType, setEventType] = useState<EventType | "">("");
  const [tool, setTool] = useState("");
  const [command, setCommand] = useState("");
  const [path, setPath] = useState("");

  useImperativeHandle(ref, () => ({
    setParams(params: DebugParams) {
      setEventType(params.eventType);
      setTool(params.tool ?? "");
      setCommand(params.command ?? "");
      setPath(params.path ?? "");
    },
  }));

  function handleSubmit(e: React.FormEvent) {
    e.preventDefault();
    if (!eventType) return;

    const params: DebugParams = { eventType };
    if (tool.trim()) params.tool = tool.trim();
    if (command.trim()) params.command = command.trim();
    if (path.trim()) params.path = path.trim();

    onSubmit(params);
  }

  const inputClassName =
    "w-full px-3 py-2 text-sm rounded border border-gray-300 dark:border-gray-600 bg-white dark:bg-[#1A1A1A] text-gray-900 dark:text-gray-100 placeholder-gray-400 focus:outline-none focus:ring-2 focus:ring-accent";

  return (
    <form onSubmit={handleSubmit} className="space-y-3">
      <div>
        <label
          htmlFor="event-type"
          className="block text-xs font-medium text-gray-600 dark:text-gray-400 mb-1"
        >
          Event Type
        </label>
        <select
          id="event-type"
          value={eventType}
          onChange={(e) => setEventType(e.target.value as EventType | "")}
          className={inputClassName}
        >
          <option value="">Select event type...</option>
          {EVENT_TYPES.map((type) => (
            <option key={type} value={type}>
              {type}
            </option>
          ))}
        </select>
      </div>

      <div>
        <label
          htmlFor="tool"
          className="block text-xs font-medium text-gray-600 dark:text-gray-400 mb-1"
        >
          Tool
        </label>
        <input
          id="tool"
          type="text"
          value={tool}
          onChange={(e) => setTool(e.target.value)}
          placeholder="e.g., Bash"
          className={inputClassName}
        />
      </div>

      <div>
        <label
          htmlFor="command"
          className="block text-xs font-medium text-gray-600 dark:text-gray-400 mb-1"
        >
          Command
        </label>
        <input
          id="command"
          type="text"
          value={command}
          onChange={(e) => setCommand(e.target.value)}
          placeholder="e.g., git push --force"
          className={inputClassName}
        />
      </div>

      <div>
        <label
          htmlFor="path"
          className="block text-xs font-medium text-gray-600 dark:text-gray-400 mb-1"
        >
          Path
        </label>
        <input
          id="path"
          type="text"
          value={path}
          onChange={(e) => setPath(e.target.value)}
          placeholder="e.g., /src/main.ts"
          className={inputClassName}
        />
      </div>

      <button
        type="submit"
        disabled={!eventType || isLoading}
        className="w-full px-4 py-2 text-sm font-medium text-white bg-accent hover:bg-accent/90 rounded transition-colors disabled:opacity-50 disabled:cursor-not-allowed"
      >
        {isLoading ? "Simulating..." : "Simulate"}
      </button>
    </form>
  );
});
