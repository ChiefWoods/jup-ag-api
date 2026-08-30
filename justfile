# Format all Cargo workspace members.
fmt:
    cargo fmt --all

# Type-check all Cargo workspace members.
check:
    cargo check --workspace

# Compile all Cargo workspace members.
build:
    cargo build --workspace

# Add an interactive Rust changeset for a publishable crate change.
changeset:
    cargo changeset add

# Show pending Rust changesets and their projected versions.
changeset-status:
    cargo changeset status

# Apply pending Rust changesets, updating crate versions and changelogs.
release:
    cargo changeset release

# Refresh the normalized composite OpenAPI documents used by both SDKs.
prepare-openapi:
    bun run prepare-openapi

# Generate all openapi-to-rust clients from their canonical specs.
openapi-gen:
    openapi-to-rust generate --config openapi-to-rust.jupiter.toml
    openapi-to-rust generate --config openapi-to-rust.tx.toml
    openapi-to-rust generate --config openapi-to-rust.perps.toml
    cargo fmt --all

# Refresh the OpenAPI inputs and regenerate both clients.
generate:
    just prepare-openapi
    just openapi-gen
