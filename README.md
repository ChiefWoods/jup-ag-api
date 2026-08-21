# Jupiter API Client

[![npm version](https://img.shields.io/npm/v/jupiter-api-ts)](https://www.npmjs.com/package/jupiter-api-ts)

## Table of Contents

- [Installation](#installation)
- [Usage](#usage)
- [Developing](#developing)

## Installation

Install the package:

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

Use `JupiterTxApi` only for the separate `tx.jup.ag` transaction-submission
endpoint.

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

### Why class-based SDKs?

This package intentionally uses Hey API's class-based, nested SDK generation.
`JupiterApi` mirrors Jupiter's product and version hierarchy (`swap.v2`,
`tokens.v2.verification`, and so on), making endpoints discoverable and keeping
future products isolated even when their OpenAPI operation names overlap.

## Developing

This repository uses Bun. Install its dependencies with:

```bash
bun install
```

### OpenAPI generation

The version-controlled source specifications live under `openapi/`, preserving
their Jupiter product and version directory structure. `openapi/jupiter.yaml`
is the derived composite input for `JupiterApi`; `openapi/transaction.yaml` is
the derived input for `JupiterTxApi`. Upstream specification updates are copied
into this directory and reviewed here.

To add a new OpenAPI specification to the client:

1. Add the source YAML under `openapi/`, using its natural product/version path.
2. Run `bun run prepare-openapi`. It discovers every source YAML, derives its
   normalized tag from the path, and refreshes both derived inputs.
3. Add the product tag path to `openapi-ts.config.ts` so `JupiterApi` exposes
   the new API in the same nested structure as `openapi/`. A new host requires
   a separate Hey API generation target and a separately named SDK class.
4. Run `bun run generate` to rebuild both typed clients under `generated/`.
5. Run the validation commands below before committing the change.

Use these commands during development:

```bash
# Refresh only the composite root specification.
bun run prepare-openapi

# Generate from the existing composite root.
bun run openapi-gen

# Clean generated output, prepare the root, and generate the client.
bun run generate

# Apply lint fixes and verify formatting.
bun run lint
bun run format:check

# Regenerate and compile the published ESM, CJS, and declaration output.
bun run build
```

### Releases

Create a changeset for a publishable change and apply the version locally:

```bash
bun run changeset
bun run version
```

Commit the version changes, then create and push a `v*` tag for that version.
The publish workflow builds the package and publishes it to npm using npm trusted
publishing; no local `release` command is needed.
