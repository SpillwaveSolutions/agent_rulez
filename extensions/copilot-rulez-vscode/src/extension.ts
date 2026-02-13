import { spawn } from "child_process";
import * as vscode from "vscode";

const PARTICIPANT_ID = "rulez.participant";

type RulezResponse = {
  continue: boolean;
  context?: string;
  reason?: string;
  timing?: {
    total_ms?: number;
  };
};

type RulezEvent = {
  hook_event_name: "UserPromptSubmit";
  session_id: string;
  tool_name?: string;
  tool_input?: Record<string, unknown>;
  cwd?: string;
};

export function activate(context: vscode.ExtensionContext) {
  const participant = vscode.chat.createChatParticipant(
    PARTICIPANT_ID,
    handleChatRequest
  );
  context.subscriptions.push(participant);
}

export function deactivate() {}

async function handleChatRequest(
  request: vscode.ChatRequest,
  chatContext: vscode.ChatContext,
  stream: vscode.ChatResponseStream,
  token: vscode.CancellationToken
) {
  const command = request.command ?? "validate";
  const prompt = extractPrompt(request);
  const binaryPath = getConfiguredBinaryPath();
  const event = buildEvent(command, prompt, chatContext);

  try {
    const response = await runRulezWithFallback(binaryPath, event, token);
    const useLmSummary = getUseLmSummary();
    if (useLmSummary) {
      const summary = await summarizeWithLm(response, prompt, command, token);
      if (summary) {
        stream.markdown(summary);
        return;
      }
    }

    stream.markdown(formatResponse(response, command));
  } catch (error) {
    const message = error instanceof Error ? error.message : String(error);
    stream.markdown(
      `**RuleZ execution failed**\n\n${sanitizeMarkdown(message)}`
    );
  }
}

function extractPrompt(request: vscode.ChatRequest): string {
  const requestAny = request as { prompt?: string; message?: { text?: string } };
  return requestAny.prompt ?? requestAny.message?.text ?? "";
}

function getConfiguredBinaryPath(): string | undefined {
  const config = vscode.workspace.getConfiguration("rulez");
  const path = config.get<string>("binaryPath");
  return path && path.trim().length > 0 ? path.trim() : undefined;
}

function getUseLmSummary(): boolean {
  const config = vscode.workspace.getConfiguration("rulez");
  return Boolean(config.get<boolean>("useLmSummary"));
}

function buildEvent(
  command: string,
  prompt: string,
  chatContext: vscode.ChatContext
): RulezEvent {
  const workspaceFolder = vscode.workspace.workspaceFolders?.[0];
  const activeFile = vscode.window.activeTextEditor?.document?.uri.fsPath;
  const cwd = workspaceFolder?.uri.fsPath;
  const sessionId = `copilot-chat-${Date.now()}`;

  return {
    hook_event_name: "UserPromptSubmit",
    session_id: sessionId,
    tool_name: "copilot.chat",
    cwd,
    tool_input: {
      command,
      prompt,
      activeFile,
      workspace: workspaceFolder
        ? {
            name: workspaceFolder.name,
            uri: workspaceFolder.uri.toString(),
            path: workspaceFolder.uri.fsPath
          }
        : undefined,
      chat: {
        historyLength: chatContext.history.length
      }
    }
  };
}

async function runRulezWithFallback(
  binaryPath: string | undefined,
  event: RulezEvent,
  token: vscode.CancellationToken
): Promise<RulezResponse> {
  const binaries = binaryPath ? [binaryPath] : ["rulez", "cch"];
  let lastError: Error | undefined;

  for (const binary of binaries) {
    try {
      return await invokeRulez(binary, event, token);
    } catch (error) {
      if (isMissingBinary(error)) {
        lastError = error instanceof Error ? error : new Error(String(error));
        continue;
      }
      throw error;
    }
  }

  throw (
    lastError ??
    new Error(
      "RuleZ binary not found. Configure rulez.binaryPath or add rulez to PATH."
    )
  );
}

function invokeRulez(
  binary: string,
  event: RulezEvent,
  token: vscode.CancellationToken
): Promise<RulezResponse> {
  return new Promise((resolve, reject) => {
    const child = spawn(binary, [], {
      stdio: ["pipe", "pipe", "pipe"]
    });

    let stdout = "";
    let stderr = "";
    let resolved = false;

    child.stdout.on("data", (chunk) => {
      stdout += chunk.toString();
    });

    child.stderr.on("data", (chunk) => {
      stderr += chunk.toString();
    });

    child.on("error", (error) => {
      if (!resolved) {
        resolved = true;
        reject(error);
      }
    });

    child.on("close", (code) => {
      if (resolved) {
        return;
      }
      resolved = true;

      if (token.isCancellationRequested) {
        reject(new Error("Request cancelled."));
        return;
      }

      if (code === 2) {
        resolve({
          continue: false,
          reason: stderr.trim() || "Blocked by RuleZ policy."
        });
        return;
      }

      if (code !== 0) {
        reject(
          new Error(
            `RuleZ exited with code ${code ?? "unknown"}: ${stderr.trim()}`
          )
        );
        return;
      }

      if (!stdout.trim()) {
        reject(new Error(`RuleZ returned no output: ${stderr.trim()}`));
        return;
      }

      try {
        const parsed = JSON.parse(stdout) as RulezResponse;
        resolve(parsed);
      } catch (error) {
        reject(
          new Error(
            `RuleZ returned invalid JSON: ${stdout.trim() || stderr.trim()}`
          )
        );
      }
    });

    if (token.isCancellationRequested) {
      child.kill();
      reject(new Error("Request cancelled."));
      return;
    }

    child.stdin.write(JSON.stringify(event));
    child.stdin.end();
  });
}

function isMissingBinary(error: unknown): boolean {
  return (
    error instanceof Error &&
    "code" in error &&
    (error as NodeJS.ErrnoException).code === "ENOENT"
  );
}

function formatResponse(response: RulezResponse, command: string): string {
  const decision = response.continue ? "allow" : "deny";
  const lines = [`**RuleZ ${command}**`, `**Decision:** ${decision}`];

  if (response.reason) {
    lines.push(`**Reason:** ${sanitizeMarkdown(response.reason)}`);
  }
  if (response.context) {
    lines.push("**Context:**");
    lines.push(sanitizeMarkdown(response.context));
  }
  if (response.timing?.total_ms !== undefined) {
    lines.push(`**Latency:** ${response.timing.total_ms}ms`);
  }

  return lines.join("\n\n");
}

async function summarizeWithLm(
  response: RulezResponse,
  prompt: string,
  command: string,
  token: vscode.CancellationToken
): Promise<string | undefined> {
  if (!vscode.lm?.selectChatModels) {
    return undefined;
  }

  try {
    const models = await vscode.lm.selectChatModels({ vendor: "copilot" });
    const model = models?.[0];
    if (!model) {
      return undefined;
    }

    const messages = [
      vscode.LanguageModelChatMessage.User(
        "Summarize the RuleZ policy decision for the user in 2-4 sentences. " +
          "Call out allow/deny and the key reason."
      ),
      vscode.LanguageModelChatMessage.User(
        `Command: ${command}\nPrompt: ${prompt}\nDecision JSON: ${JSON.stringify(
          response
        )}`
      )
    ];

    const lmResponse = await model.sendRequest(messages, {}, token);
    let text = "";
    for await (const chunk of lmResponse.text) {
      text += chunk;
    }

    const summary = text.trim();
    return summary.length > 0 ? summary : undefined;
  } catch {
    return undefined;
  }
}

function sanitizeMarkdown(value: string): string {
  return value.replace(/\r\n/g, "\n");
}
