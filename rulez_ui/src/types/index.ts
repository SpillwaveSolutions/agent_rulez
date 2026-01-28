// Configuration types
export interface ConfigFile {
  path: string;
  exists: boolean;
  modified: boolean;
  hasErrors: boolean;
  content?: string;
}

export interface FileState {
  content: string;
  originalContent: string;
  modified: boolean;
}

// Editor types
export interface CursorPosition {
  line: number;
  column: number;
}

export interface SelectionRange {
  startLine: number;
  startColumn: number;
  endLine: number;
  endColumn: number;
}

export interface ValidationError {
  line: number;
  column: number;
  endLine?: number;
  endColumn?: number;
  message: string;
  severity: "error";
  source?: string;
}

export interface ValidationWarning {
  line: number;
  column: number;
  endLine?: number;
  endColumn?: number;
  message: string;
  severity: "warning";
  source?: string;
}

// Debug simulator types
export type EventType =
  | "PreToolUse"
  | "PostToolUse"
  | "PermissionRequest"
  | "UserPromptSubmit"
  | "SessionStart"
  | "SessionEnd"
  | "PreCompact";

export interface DebugParams {
  eventType: EventType;
  tool?: string;
  command?: string;
  path?: string;
}

export interface RuleEvaluation {
  ruleName: string;
  matched: boolean;
  timeMs: number;
  details?: string;
  pattern?: string;
  input?: string;
}

export interface DebugResult {
  outcome: "Allow" | "Block" | "Inject";
  reason?: string;
  matchedRules: string[];
  evaluationTimeMs: number;
  evaluations: RuleEvaluation[];
}

// CCH Configuration types
export interface HooksSettings {
  log_level?: "debug" | "info" | "warn" | "error";
  fail_open?: boolean;
  max_context_size?: string;
}

export interface RuleMatcher {
  tools?: string[];
  extensions?: string[];
  directories?: string[];
  command_match?: string;
  path_match?: string;
}

export interface RuleAction {
  block?: boolean;
  inject?: string | string[];
  run?: string;
  block_if_match?: string;
}

export interface Rule {
  name: string;
  description?: string;
  enabled?: boolean;
  matchers: RuleMatcher;
  actions: RuleAction;
}

export interface HooksConfig {
  version: string;
  settings?: HooksSettings;
  rules?: Rule[];
  hooks?: Rule[]; // Alias for rules
}
