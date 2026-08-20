import { join } from "node:path";

const MODEL_DIRECTORY =
  Bun.env.GENERATED_DIRECTORY ?? join(import.meta.dir, "..", "generated", "models");
const scalarUnionPattern = /export type (\w+) = (?:number \| string|string \| number);/g;

for await (const file of new Bun.Glob("*.ts").scan({ cwd: MODEL_DIRECTORY })) {
  const path = join(MODEL_DIRECTORY, file);
  let source = await Bun.file(path).text();
  const typeNames = [...source.matchAll(scalarUnionPattern)].map((match) => match[1]);

  for (const typeName of typeNames) {
    const fromJsonPattern = new RegExp(
      `(export function ${typeName}FromJSONTyped\\([^)]*\\): ${typeName} \\{\\n    if \\(\\(json === undefined\\) \\|\\| \\(json === null\\)\\) \\{\\n        return json;\\n    \\})[\\s\\S]*?\\n\\}`,
    );
    const toJsonPattern = new RegExp(
      `(export function ${typeName}ToJSON\\([^)]*\\): any \\{\\n    if \\(value === undefined\\) \\{\\n        return undefined;\\n    \\}\\n    if \\(value === null\\) \\{\\n        return null;\\n    \\})[\\s\\S]*?\\n\\}`,
    );

    source = source.replace(fromJsonPattern, `$1\n    return json as ${typeName};\n}`);
    source = source.replace(toJsonPattern, "$1\n\n    return value;\n}");
  }

  await Bun.write(path, source);
}
