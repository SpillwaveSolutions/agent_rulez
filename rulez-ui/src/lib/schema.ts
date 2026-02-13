import type { IDisposable } from "monaco-editor";
import { configureMonacoYaml } from "monaco-yaml";
import hooksSchema from "./hooks-schema.json";

type MonacoInstance = Parameters<typeof configureMonacoYaml>[0];

/**
 * Configure monaco-yaml with the RuleZ hooks JSON schema.
 *
 * Must be called ONCE and BEFORE the editor mounts (in the beforeMount callback).
 * The schema is inlined via Vite JSON import so it works in both dev and Tauri production.
 */
export function configureYamlSchema(monaco: MonacoInstance): IDisposable {
  return configureMonacoYaml(monaco, {
    enableSchemaRequest: false,
    schemas: [
      {
        uri: "https://spillwave.dev/schemas/hooks-config/v1.0",
        fileMatch: ["*"],
        schema: hooksSchema as Record<string, unknown>,
      },
    ],
  });
}
