# Jupiter API TypeScript Client

[![npm version](https://img.shields.io/npm/v/jupiter-api-ts)](https://www.npmjs.com/package/jupiter-api-ts)

## Installation

```bash
bun add jupiter-api-ts
```

## Usage

```ts
import { JupiterApi, createJupiterApiClient } from "jupiter-api-ts";

const jupiter = new JupiterApi({
  client: createJupiterApiClient({
    auth: "your-jupiter-api-key",
  }),
});

const { data, error } = await jupiter.swap.v2.getOrder({
  query: {
    inputMint: "So11111111111111111111111111111111111111112",
    outputMint: "EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v",
    amount: "1000000",
  },
});
```

`createJupiterApiClient()` defaults to `https://api.jup.ag`. Pass `baseUrl`
only to override the base URL.

For the separate `tx.jup.ag` transaction-submission endpoint:

```ts
import { JupiterTxApi, createJupiterTxApiClient } from "jupiter-api-ts";

const jupiterTx = new JupiterTxApi({
  client: createJupiterTxApiClient({
    auth: "your-jupiter-api-key",
  }),
});

const result = await jupiterTx.sendTransaction({
  body: {
    jsonrpc: "2.0",
    id: 1,
    method: "sendTransaction",
    params: ["base64-signed-transaction", { encoding: "base64" }],
  },
});
```

`createJupiterTxApiClient()` defaults to `https://tx.jup.ag`.

For the Perps endpoint:

```ts
import { JupiterPerpsApi, createJupiterPerpsApiClient } from "jupiter-api-ts";

const perps = new JupiterPerpsApi();

const { data, error } = await perps.getMarketStats({
  query: {
    mint: "So11111111111111111111111111111111111111112",
  },
});
```

Defaults to `https://perps-api.jup.ag/v1` and does not require an API key.

For the Data API endpoint:

```ts
import { JupiterDatapi, createJupiterDatapiClient } from "jupiter-api-ts";

const datapi = new JupiterDatapi();

const { data, error } = await datapi.searchAssets({
  query: {
    query: "SOL",
  },
});
```

Defaults to `https://datapi.jup.ag` and does not require an API key.

## API layout

This SDK uses Hey API's class-based, nested SDK generation. `JupiterApi`
mirrors Jupiter's product and version hierarchy (`swap.v2`,
`tokens.v2.verification`, and so on), keeping endpoints discoverable and future
products isolated even when their OpenAPI operation names overlap.

## Development

Run these commands from the repository root:

```bash
# Refresh only the composite OpenAPI specifications.
bun run prepare-openapi

# Generate from the existing composite specifications.
bun run openapi-gen:ts

# Refresh the specifications and generate this package's clients.
bun run generate:ts

# Type-check or build this package.
bun run typecheck
bun run build

# Apply lint fixes and check formatting across the repository.
bun run lint
bun run format:check
```

The generated clients are version-controlled under `generated/jupiter/` and
`generated/tx/` in this package.

## Releases

Create a root Changeset for a consumer-visible change, then apply the version:

```bash
bun run changeset
bun run version
```

Commit the changes and push a `v*` tag. The publish workflow invokes this
package's `prepublishOnly` hook, which regenerates the clients from the shared
OpenAPI specifications and builds the npm package before publishing.
