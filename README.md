# Jupiter API Clients

Generated client libraries for Jupiter APIs.

## Packages

| Package | Lang | Version |
| --- | --- | --- |
| [`jupiter-api-ts`](packages/jupiter-api-ts/README.md) | TypeScript | [![npm version](https://img.shields.io/npm/v/jupiter-api-ts)](https://www.npmjs.com/package/jupiter-api-ts) |
| [`jupiter-api-rs`](crates/jupiter-api-rs/README.md) | Rust | [![crates.io version](https://img.shields.io/crates/v/jupiter-api-rs)](https://crates.io/crates/jupiter-api-rs) |

Each package README contains its installation, usage, development, and release
instructions.

## Repository development

The version-controlled source OpenAPI specifications live under `openapi/`.
`openapi/jupiter.yaml` and `openapi/transaction.yaml` are derived composite
inputs shared by both client generators.

Install repository dependencies with Bun:

```bash
bun install
```

When source specifications change, refresh the composites before generating a
client:

```bash
bun run prepare-openapi
```

The root is a Bun workspace for npm packages and a Cargo workspace for Rust
crates. Language-specific source, generated output, package metadata, and
changelogs live with their respective packages.
