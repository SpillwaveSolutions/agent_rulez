import type { IDisposable, editor, languages } from "monaco-editor";
import { parseDocument } from "yaml";

/**
 * Register a comment-preserving YAML DocumentFormattingEditProvider.
 *
 * Uses the `yaml` package's parseDocument (preserves comments) rather than
 * parse + stringify (which drops comments).
 *
 * Returns an IDisposable that can be used to unregister the provider.
 */
export function registerYamlFormatter(monaco: typeof import("monaco-editor")): IDisposable {
  return monaco.languages.registerDocumentFormattingEditProvider("yaml", {
    provideDocumentFormattingEdits(
      model: editor.ITextModel,
      options: languages.FormattingOptions,
    ): languages.TextEdit[] {
      try {
        const content = model.getValue();
        const doc = parseDocument(content);
        if (doc.errors.length > 0) return [];
        const formatted = doc.toString({
          indent: options.tabSize,
          lineWidth: 0,
        });
        if (formatted === content) return [];
        return [{ range: model.getFullModelRange(), text: formatted }];
      } catch {
        return [];
      }
    },
  });
}
