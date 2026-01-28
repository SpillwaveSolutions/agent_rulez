import type { HooksConfig, Rule, RuleAction, RuleMatcher } from "@/types";
import { parseDocument } from "yaml";

/**
 * Parse YAML content and return the parsed HooksConfig along with
 * a map of rule name to 1-indexed line number.
 */
export interface ParsedYamlResult {
  config: HooksConfig;
  linePositions: Map<string, number>;
}

/**
 * Parse YAML content into a structured HooksConfig object.
 * Returns null if the content is empty or not valid YAML.
 */
export function parseHooksConfig(content: string): HooksConfig | null {
  if (!content.trim()) return null;

  try {
    const doc = parseDocument(content);
    if (doc.errors.length > 0) return null;

    const data = doc.toJS() as Record<string, unknown>;
    if (!data || typeof data !== "object") return null;

    const config: HooksConfig = {
      version: typeof data.version === "string" ? data.version : "1",
    };

    if (data.settings && typeof data.settings === "object") {
      const s = data.settings as Record<string, unknown>;
      config.settings = {
        log_level: isLogLevel(s.log_level) ? s.log_level : undefined,
        fail_open: typeof s.fail_open === "boolean" ? s.fail_open : undefined,
        max_context_size: typeof s.max_context_size === "string" ? s.max_context_size : undefined,
      };
    }

    const rulesArray = Array.isArray(data.rules)
      ? data.rules
      : Array.isArray(data.hooks)
        ? data.hooks
        : undefined;

    if (rulesArray) {
      config.rules = rulesArray
        .filter((r): r is Record<string, unknown> => r !== null && typeof r === "object")
        .map((r) => parseRule(r));
    }

    return config;
  } catch {
    return null;
  }
}

function isLogLevel(v: unknown): v is "debug" | "info" | "warn" | "error" {
  return typeof v === "string" && ["debug", "info", "warn", "error"].includes(v);
}

function parseRule(r: Record<string, unknown>): Rule {
  const matchers: RuleMatcher = {};
  const actions: RuleAction = {};

  if (r.matchers && typeof r.matchers === "object") {
    const m = r.matchers as Record<string, unknown>;
    if (Array.isArray(m.tools)) matchers.tools = m.tools.map(String);
    if (Array.isArray(m.extensions)) matchers.extensions = m.extensions.map(String);
    if (Array.isArray(m.directories)) matchers.directories = m.directories.map(String);
    if (typeof m.command_match === "string") matchers.command_match = m.command_match;
    if (typeof m.path_match === "string") matchers.path_match = m.path_match;
  }

  if (r.actions && typeof r.actions === "object") {
    const a = r.actions as Record<string, unknown>;
    if (typeof a.block === "boolean") actions.block = a.block;
    if (typeof a.inject === "string") actions.inject = a.inject;
    if (Array.isArray(a.inject)) actions.inject = a.inject.map(String);
    if (typeof a.run === "string") actions.run = a.run;
    if (typeof a.block_if_match === "string") actions.block_if_match = a.block_if_match;
  }

  return {
    name: typeof r.name === "string" ? r.name : "unnamed",
    description: typeof r.description === "string" ? r.description : undefined,
    enabled: typeof r.enabled === "boolean" ? r.enabled : undefined,
    matchers,
    actions,
  };
}

/**
 * Get a map of rule name to 1-indexed line number by inspecting the YAML
 * document's AST node ranges.
 */
export function getRuleLinePositions(content: string): Map<string, number> {
  const positions = new Map<string, number>();
  if (!content.trim()) return positions;

  try {
    const doc = parseDocument(content);
    if (doc.errors.length > 0) return positions;

    const data = doc.toJS() as Record<string, unknown>;
    const rulesKey = Array.isArray(data?.rules)
      ? "rules"
      : Array.isArray(data?.hooks)
        ? "hooks"
        : null;

    if (!rulesKey) return positions;

    // Access the document's contents (a YAMLMap) to find the rules sequence node
    const docContents = doc.contents;
    if (!docContents || !("items" in docContents)) return positions;

    const items = (docContents as { items: Array<{ key: unknown; value: unknown }> }).items;
    const rulesEntry = items.find((item) => {
      const key = item.key;
      return (
        key &&
        typeof key === "object" &&
        "value" in key &&
        (key as { value: unknown }).value === rulesKey
      );
    });

    if (
      !rulesEntry?.value ||
      typeof rulesEntry.value !== "object" ||
      !("items" in (rulesEntry.value as object))
    )
      return positions;

    const seqItems = (rulesEntry.value as { items: unknown[] }).items;

    for (const seqItem of seqItems) {
      if (!seqItem || typeof seqItem !== "object") continue;

      // Each sequence item is a YAMLMap; find the "name" key's value
      const mapNode = seqItem as {
        items?: Array<{ key: unknown; value: unknown }>;
        range?: [number, number, number];
      };

      let ruleName: string | undefined;
      if (mapNode.items) {
        for (const pair of mapNode.items) {
          const pairKey = pair.key;
          if (
            pairKey &&
            typeof pairKey === "object" &&
            "value" in pairKey &&
            (pairKey as { value: unknown }).value === "name"
          ) {
            const pairValue = pair.value;
            if (pairValue && typeof pairValue === "object" && "value" in pairValue) {
              ruleName = String((pairValue as { value: unknown }).value);
            }
            break;
          }
        }
      }

      if (ruleName && mapNode.range) {
        // range[0] is the character offset; convert to line number
        const lineNumber = offsetToLine(content, mapNode.range[0]);
        positions.set(ruleName, lineNumber);
      }
    }
  } catch {
    // Return whatever we have so far
  }

  return positions;
}

/**
 * Convert a character offset to a 1-indexed line number.
 */
function offsetToLine(content: string, offset: number): number {
  let line = 1;
  for (let i = 0; i < offset && i < content.length; i++) {
    if (content[i] === "\n") line++;
  }
  return line;
}

/**
 * Parse YAML content and return both the config and line positions.
 */
export function parseYamlWithPositions(content: string): ParsedYamlResult | null {
  const config = parseHooksConfig(content);
  if (!config) return null;

  const linePositions = getRuleLinePositions(content);
  return { config, linePositions };
}
