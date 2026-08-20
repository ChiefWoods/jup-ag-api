# Jupiter API Client

## Table of Contents

- [Installation](#installation)
- [Usage](#usage)
- [Developing](#developing)

## Installation

### TypeScript

```bash
bun add @jup-ag/api
```

### Rust

```bash
cargo add jup-ag-api
```

TLS defaults to `native-tls`. For rustls:

```bash
cargo add jup-ag-api --no-default-features --features rustls
```

In Rust source the crate is imported as `jup_ag_api` (hyphens are not valid in identifiers).

## Usage

### TypeScript

```ts
import { JupiterApi } from "@jup-ag/api";

const jupiter = new JupiterApi({
  apiKey: "your-jupiter-api-key",
});

const order = await jupiter.swap.v2.getOrder({
  inputMint: "So11111111111111111111111111111111111111112",
  outputMint: "EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v",
  amount: "1000000",
  slippageBps: 100,
});
```

### Rust

```rust
use jup_ag_api::{JupiterClient, JupiterClientOptions};

let jupiter = JupiterClient::new(JupiterClientOptions {
    api_key: Some("your-jupiter-api-key".to_owned()),
    ..JupiterClientOptions::default()
});

let order = jupiter
    .swap
    .v2
    .get_order(
        "So11111111111111111111111111111111111111112",
        "EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v",
        "1000000",
        None,
        None,
        None,
        Some(100),
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
    )
    .await?;
```

## Developing

This repository uses Bun. Install its dependencies with:

```bash
bun install
```

### OpenAPI generation

The version-controlled source specifications live under `openapi/`, preserving
their Jupiter product and version directory structure. `openapi/jupiter.yaml` is
the derived composite input consumed by the TypeScript and Rust generators.
Upstream specification updates are copied into this directory and reviewed here.

To add a new OpenAPI specification to the client:

1. Add the source YAML under `openapi/`, using its natural product/version path.
2. Run `bun run prepare-openapi`. It discovers every source YAML, derives its
   normalized tag from the path, and refreshes `openapi/jupiter.yaml`.
3. Add the corresponding service key and default URL to
   `clients/ts/config.ts` and `clients/rust/src/config.rs`, then expose
   its generated API from `clients/ts/JupiterApi.ts` and
   `clients/rust/src/client.rs` in the same nested structure as `openapi/`.
4. Run `bun run generate` to rebuild the composite specification and the
   TypeScript and Rust clients under `generated/ts` and `generated/rust`.
   Rust method wrappers on `JupiterClient` are regenerated from
   `generated/rust` by `scripts/generate-rust-client.ts`.
5. Run the validation commands below before committing the change.

Use these commands during development:

```bash
# Refresh only the composite root specification.
bun run prepare-openapi

# Generate a single client from the existing composite root.
bun run openapi-gen:ts
bun run openapi-gen:rust

# Compile the Rust wrapper against generated bindings.
cargo check -p jup-ag-api

# Clean generated output, prepare the root, and generate all clients in parallel.
bun run generate

# Apply lint fixes and verify formatting.
bun run lint
bun run format:check

# Regenerate and compile the published ESM, CJS, and declaration output.
bun run build
```

### Releases

Bump the version in `package.json`, then tag that version (`v6.0.49`, etc.) to
trigger trusted publication.
