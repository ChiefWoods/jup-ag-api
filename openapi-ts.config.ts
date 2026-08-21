import { defineConfig } from "@hey-api/openapi-ts";

const TAG_PATHS: Record<string, readonly string[]> = {
  Lend: ["lend", "lend"],
  LendBorrow: ["lend", "borrow"],
  Portfolio: ["portfolio"],
  Prediction: ["prediction"],
  PriceV2: ["price", "v2"],
  PriceV3: ["price", "v3"],
  Recurring: ["recurring"],
  Send: ["send"],
  Studio: ["studio"],
  SwapV1: ["swap", "v1"],
  SwapV2: ["swap", "v2"],
  TokensV1: ["tokens", "v1"],
  TokensV2: ["tokens", "v2"],
  TokensV2Verification: ["tokens", "v2", "verification"],
  TriggerV1: ["trigger", "v1"],
  TriggerV2: ["trigger", "v2"],
  Ultra: ["ultra"],
};

export default defineConfig([
  {
    input: "./openapi/jupiter.yaml",
    output: "generated/ts/jupiter",
    plugins: [
      {
        name: "@hey-api/client-fetch",
        baseUrl: "https://api.jup.ag",
      },
      {
        name: "@hey-api/sdk",
        operations: {
          containerName: "JupiterApi",
          nesting(operation) {
            const tag = operation.tags?.[0];
            const path = tag === undefined ? undefined : TAG_PATHS[tag];

            if (path === undefined) {
              throw new TypeError(`Unsupported Jupiter API operation tag: ${tag ?? "none"}`);
            }

            return [...path, operation.operationId ?? operation.method.toLowerCase()];
          },
          strategy: "single",
        },
      },
    ],
  },
  {
    input: "./openapi/transaction.yaml",
    output: "generated/ts/tx",
    plugins: [
      {
        name: "@hey-api/client-fetch",
        baseUrl: "https://tx.jup.ag",
      },
      {
        name: "@hey-api/sdk",
        operations: {
          containerName: "JupiterTxApi",
          nesting(operation) {
            return [operation.operationId ?? operation.method.toLowerCase()];
          },
          strategy: "single",
        },
      },
    ],
  },
]);
