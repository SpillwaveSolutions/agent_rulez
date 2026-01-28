import { configureMonacoYaml } from "monaco-yaml";

type MonacoInstance = Parameters<typeof configureMonacoYaml>[0];

/**
 * Configure monaco-yaml with the CCH hooks JSON schema.
 *
 * Must be called ONCE and BEFORE the editor mounts (in the beforeMount callback).
 * The schema file lives at public/schema/hooks-schema.json, which Vite serves
 * at /schema/hooks-schema.json.
 */
export function configureYamlSchema(monaco: MonacoInstance): void {
  configureMonacoYaml(monaco, {
    enableSchemaRequest: true,
    schemas: [
      {
        uri: "/schema/hooks-schema.json",
        fileMatch: ["*"],
      },
    ],
  });
}
