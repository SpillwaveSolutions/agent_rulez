/**
 * RuleZ Policy Engine Plugin for OpenCode CLI
 *
 * Intercepts tool.execute.before and tool.execute.after lifecycle events,
 * pipes JSON to `rulez opencode hook` via subprocess, and enforces
 * allow/deny/inject decisions.
 *
 * Install: copy this folder to .opencode/plugins/rulez-plugin/ (project)
 * or ~/.config/opencode/plugins/rulez-plugin/ (global).
 */

interface PluginContext {
  project: string;
  directory: string;
  worktree: string;
  client: unknown;
  $: unknown;
}

interface ToolEventContext {
  sessionId: string;
  toolName: string;
  toolInput?: Record<string, unknown>;
  cwd?: string;
  [key: string]: unknown;
}

interface RulezResponse {
  continue: boolean;
  reason?: string;
  context?: string;
}

const RULEZ_BINARY = process.env.RULEZ_BINARY_PATH || "rulez";
const HOOK_TIMEOUT_MS = 5000;

async function callRulezHook(
  hookEventName: string,
  ctx: ToolEventContext
): Promise<RulezResponse> {
  const payload = JSON.stringify({
    session_id: ctx.sessionId || "unknown",
    hook_event_name: hookEventName,
    tool_name: ctx.toolName,
    tool_input: ctx.toolInput || {},
    cwd: ctx.cwd || process.cwd(),
  });

  try {
    const proc = Bun.spawn([RULEZ_BINARY, "opencode", "hook"], {
      stdin: new Blob([payload]),
      stdout: "pipe",
      stderr: "pipe",
    });

    const timeoutId = setTimeout(() => {
      proc.kill();
    }, HOOK_TIMEOUT_MS);

    const exitCode = await proc.exited;
    clearTimeout(timeoutId);

    const stdout = await new Response(proc.stdout).text();

    if (stdout.trim()) {
      return JSON.parse(stdout.trim()) as RulezResponse;
    }

    // Exit code 2 means deny
    if (exitCode === 2) {
      return { continue: false, reason: "Denied by RuleZ policy" };
    }

    return { continue: true };
  } catch (err) {
    // Fail-open: if rulez subprocess fails, allow the tool call
    console.warn(`[rulez-plugin] Hook error (fail-open): ${err}`);
    return { continue: true, reason: "RuleZ hook error (fail-open)" };
  }
}

export default async function rulezPlugin(_ctx: PluginContext) {
  return {
    async "tool.execute.before"(eventCtx: ToolEventContext) {
      const response = await callRulezHook("tool.execute.before", eventCtx);

      if (!response.continue) {
        throw new Error(
          response.reason || "Blocked by RuleZ policy"
        );
      }

      // If RuleZ injected context, attach it to the event
      if (response.context) {
        return { context: response.context };
      }

      return undefined;
    },

    async "tool.execute.after"(eventCtx: ToolEventContext) {
      // Post-tool: audit logging only, never block
      await callRulezHook("tool.execute.after", eventCtx);
      return undefined;
    },
  };
}
